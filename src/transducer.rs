use std::{
    cmp::min,
    collections::{BTreeSet, HashMap, HashSet},
    panic,
};

mod tests;
mod utils;
use utils::{add_to_or_insert, longest_common_prefix, remove_from_or_delete};

use self::utils::insert_or_push_in_partition;

pub struct Transducer {
    alphabet: HashSet<char>,
    states: BTreeSet<usize>,
    finality: BTreeSet<usize>,
    init_state: usize,
    delta: HashMap<usize, HashMap<char, usize>>,
    delta_inv: HashMap<usize, HashSet<(char, usize)>>,
    lambda: HashMap<usize, HashMap<char, usize>>,
    iota: usize,
    psi: HashMap<usize, usize>,
    min_except: Vec<char>,
    trans_order_partitions: Vec<BTreeSet<usize>>,
}

impl Transducer {
    pub fn add_entry_in_order(&mut self, word: &str, output: usize) {
        let word = word.chars().collect();
        let k = longest_common_prefix(&self.min_except, &word).len();
        let new_suffix_len = word.len() - k;
        let max_state = *self
            .states
            .last()
            .expect("The transducer should have at least 1 state!");

        // Make the transducer min except in (last_entry ^
        for _ in 0..(self.min_except.len() - k) {
            self.reduce_except_by_one();
        }

        let tk = *self.state_sequence(&self.min_except).last().unwrap();

        // Add potentially new characters to the alphabet
        for i in k..word.len() {
            self.alphabet.insert(word[i]);
        }

        // Add new (final) states and extend alphabet for the missing suffix
        for i in 1..=new_suffix_len {
            self.states.insert(max_state + i);
        }
        self.finality.insert(max_state + new_suffix_len);

        // Add a transition from the existing prefix
        self.add_delta_transition(tk, word[k], max_state + 1);

        // Add the transitions for the missing suffix
        for i in 1..new_suffix_len {
            self.add_delta_transition(max_state + i, word[k + i], max_state + i + 1);
        }

        let states = self.state_sequence(&word);

        // Update transition order partitions
        for i in (k + 1)..(states.len() - 1) {
            self.trans_order_partitions[1].insert(states[i]);
        }
        // NOTE: this happens after updting delta
        let tk_trans_order = self.delta.get(&states[k]).map_or(0, |trans| trans.len());
        self.trans_order_partitions[tk_trans_order - 1].remove(&states[k]);
        insert_or_push_in_partition(&mut self.trans_order_partitions, states[k], tk_trans_order);
        self.trans_order_partitions[0].insert(*states.last().unwrap());

        for i in 1..=k {
            if self.finality.contains(&states[i]) {
                let final_output = self.output(&word[..i].to_vec()) - self.lambda_i(i, output);
                self.psi.insert(states[i], final_output);
            }
        }
        states.last().and_then(|tm| self.psi.insert(*tm, 0));

        // Update output transitions
        //
        // NOTE: the first and last updates of lambda both depend on the old lambda.
        // This means that the updates have to be done simultaneously.
        let mut postponed_lambda_updates: Vec<(usize, char, usize)> = Vec::new();
        for i in 1..=k {
            let curr_output = self.lambda_i(i, output);
            let prev_output = self.lambda_i(i - 1, output);
            postponed_lambda_updates.push((
                states[i - 1],
                self.min_except[i - 1],
                curr_output - prev_output,
            ));
        }

        let lambda_k = self.lambda_i(k, output);
        add_to_or_insert(&mut self.lambda, states[k], word[k], output - lambda_k);

        for i in 1..new_suffix_len {
            add_to_or_insert(&mut self.lambda, max_state + i, word[k + i], 0);
        }

        for i in 0..=k {
            for ch in self.alphabet.iter() {
                let is_lambda_defined = self
                    .lambda
                    .get(&states[i])
                    .and_then(|trans| trans.get(ch))
                    .is_some();

                if *ch != word[i] && is_lambda_defined {
                    let mut prefix_with_ch = word[0..i].to_vec();
                    prefix_with_ch.push(*ch);

                    let output =
                        self.iota + self.lambda_star(&prefix_with_ch) - self.lambda_i(i, output);

                    postponed_lambda_updates.push((states[i], *ch, output));
                }
            }
        }

        for (q, a, o) in postponed_lambda_updates {
            add_to_or_insert(&mut self.lambda, q, a, o);
        }

        // Update iota last, as lambda and psi use the old value
        self.iota = min(self.iota, output);

        // The resulting Transducer is minimal except in the new_entry
        self.min_except = word;
    }

    pub fn add_entry_out_of_order(&mut self, word: &str, output: usize) {
        let word_vec = word.chars().collect();
        let word_lcp = self.longest_common_prefix(&word_vec);

        self.increase_except_from_epsilon_to_word(&word_lcp);
        // NOTE: This shouldn't reduce except as it is already minimal except in word_lcp
        self.add_entry_in_order(word, output);
        self.reduce_to_epsilon();
    }

    pub fn remove_entry_with_word(&mut self, word_raw: &str) {
        if word_raw == "" {
            panic!("The transducer is undefined for epsilon input!");
        }

        let word = word_raw.chars().collect();
        let mut curr_state = self.increase_except_from_epsilon_to_word(&word);
        let mut prev_state;

        self.finality.remove(&curr_state);
        self.psi.remove(&curr_state);

        loop {
            let has_more_transitions = self
                .delta
                .get(&curr_state)
                .is_some_and(|trans| trans.len() > 1);

            if curr_state == self.init_state
                || has_more_transitions
                || self.finality.contains(&curr_state)
            {
                self.canonicalise_min_except();
                break;
            }

            prev_state = curr_state;

            // NOTE: increase_except_from_epsilon_to_word leaves the path without any convergent
            // states, meaning that all states have only one ingoing transition
            curr_state = self.delta_inv[&curr_state].iter().last().unwrap().1;

            self.delete_state(&prev_state);
            self.min_except.pop();
        }

        self.reduce_to_epsilon();
    }

    pub fn from_dictionary(dictionary: Vec<(&str, usize)>) -> Self {
        if dictionary.is_empty() {
            panic!("Cannot construct empty transducer");
        }

        let (w, o) = dictionary[0];
        let mut transducer = Transducer::from_entry(w, o);

        for e in &dictionary[1..] {
            let (w, o) = *e;
            transducer.add_entry_in_order(w, o);
        }

        transducer.reduce_to_epsilon();
        return transducer;
    }

    pub fn output(&self, word: &Vec<char>) -> usize {
        let final_output = self
            .state_sequence(&word)
            .last()
            .and_then(|q| self.psi.get(q))
            .unwrap_or(&0);
        return self.iota + self.lambda_star(&word) + final_output;
    }

    pub fn print(&self) {
        println!("T alphabet: {:?}", self.alphabet);
        println!("T states: {:?}", self.states);
        println!("T finality: {:?}", self.finality);
        println!("T init_state: {:?}", self.init_state);
        println!("T delta: {:?}", self.delta);
        println!("T delta inversed: {:?}", self.delta_inv);
        println!("T lambda: {:?}", self.lambda);
        println!("T iota: {:?}", self.iota);
        println!("T psi: {:?}", self.psi);
        println!("T min_except: {:?}", self.min_except);
        println!(
            "T partitions by transition order: {:?}",
            self.trans_order_partitions
        );
    }

    pub fn print_with_message(&self, message: &str) {
        println!("{:?}", message);
        self.print();
    }

    ///////////////////
    // Private functions:
    //////////////////////
    fn from_entry(word: &str, output: usize) -> Self {
        let word: Vec<char> = word.chars().collect();
        let n = word.len();

        let mut alphabet = HashSet::new();
        let mut delta = HashMap::new();
        let mut delta_inv = HashMap::new();
        let mut lambda = HashMap::new();

        for i in 0..n {
            alphabet.insert(word[i]);

            let state_transition = HashMap::from([(word[i], i + 1)]);
            delta.insert(i, state_transition);

            delta_inv.insert(i + 1, HashSet::from([(word[i], i)]));

            let state_output = HashMap::from([(word[i], 0)]);
            lambda.insert(i, state_output);
        }

        return Self {
            alphabet,
            states: (0..=n).collect(),
            finality: BTreeSet::from([n]),
            init_state: 0,
            delta,
            delta_inv,
            lambda,
            iota: output,
            psi: HashMap::from([(n, 0)]),
            min_except: word,
            trans_order_partitions: vec![BTreeSet::from([n]), (0..n).collect()],
        };
    }

    fn reduce_except_by_one(&mut self) {
        let w = &self.min_except;
        let t_w = self.state_sequence(&w);

        let an = w[w.len() - 1];
        let tn = t_w[w.len()];
        let tn_prev = t_w[w.len() - 1]; // Note: will fail if min_except = epsilon

        // TODO: do this for every equal state?
        if let Some(q) = self.state_eq(tn, &t_w) {
            let prev_output = self.lambda[&tn_prev][&an];
            self.delete_state(&tn);
            self.add_delta_transition(tn_prev, an, q);
            add_to_or_insert(&mut self.lambda, tn_prev, an, prev_output);
        }

        self.min_except.pop();
    }

    // NOTE: returns the last and final state that reads the word
    fn increase_except_from_epsilon_to_word(&mut self, word: &Vec<char>) -> usize {
        if !self.min_except.is_empty() {
            panic!("Transduser must be minimal except in epsilon!");
        }

        let mut current_state = self.init_state;
        let mut max_state = *self.states.last().expect("States cannot be empty!");

        for i in 0..word.len() {
            let next_state = self.delta[&current_state][&word[i]];

            if self.is_state_convergent(next_state) {
                max_state += 1;
                let new_state = max_state;

                self.states.insert(new_state);

                self.add_delta_transition(current_state, word[i], new_state);

                if let Some(trans) = self.delta_inv.get_mut(&next_state) {
                    trans.remove(&(word[i], current_state));
                }

                if self.finality.contains(&next_state) {
                    self.finality.insert(new_state);
                    self.psi.insert(new_state, self.psi[&next_state]);
                }

                let trans_order = self.delta.get(&next_state).map_or(0, |trans| trans.len());
                self.trans_order_partitions[trans_order].insert(new_state);

                if trans_order > 0 {
                    for (ch, q) in self.delta[&next_state].clone() {
                        self.add_delta_transition(new_state, ch, q);
                    }
                }

                if let Some(output_trans) = self.lambda.get(&next_state).cloned() {
                    self.lambda.insert(new_state, output_trans);
                }

                current_state = new_state;
            } else {
                current_state = next_state;
            }

            self.min_except.push(word[i]);
        }

        return current_state;
    }

    fn is_state_convergent(&self, state: usize) -> bool {
        return self.delta_inv.get(&state).map_or(0, |trans| trans.len()) > 1;
    }

    // Check for equal states by:
    // 1) have states partitioned based on their number of transitions
    // 2) check if tn is equal to some state with the same number of transitions
    fn state_eq(&self, state: usize, t_w: &Vec<usize>) -> Option<usize> {
        let state_is_final = self.finality.contains(&state);
        // No transitions if delta(state) is undefined
        let state_trans_order = self.delta.get(&state).map_or(0, |trans| trans.len());

        for q in &self.trans_order_partitions[state_trans_order] {
            if t_w.contains(q) {
                continue;
            }

            let cond1 = state_is_final == self.finality.contains(q);
            let cond2 = !state_is_final || self.psi.get(&state) == self.psi.get(q);
            let mut cond3 = true;

            for a in &self.alphabet {
                let dsa = self.delta.get(&state).and_then(|trans| trans.get(a));
                let dqa = self.delta.get(&q).and_then(|trans| trans.get(a));
                let lsa = self.lambda.get(&state).and_then(|trans| trans.get(a));
                let lqa = self.lambda.get(&q).and_then(|trans| trans.get(a));

                match (dsa, dqa, lsa, lqa) {
                    (None, None, _, _) => (),
                    (Some(q1), Some(q2), Some(m1), Some(m2)) => {
                        cond3 = cond3 && q1 == q2 && m1 == m2;
                    }
                    _ => {
                        cond3 = false;
                    }
                };
            }

            // Return the first match
            if cond1 && cond2 && cond3 {
                return Some(*q);
            }
        }

        return None;
    }

    fn reduce_to_epsilon(&mut self) {
        for _ in 0..self.min_except.len() {
            self.reduce_except_by_one();
        }
    }

    // NOTE: delta[(q,a)] will panic if delta is not defined
    fn state_sequence(&self, w: &Vec<char>) -> Vec<usize> {
        let mut next = self.init_state;
        let mut path = vec![next];

        for i in 0..w.len() {
            next = self.delta[&next][&w[i]];
            path.push(next);
        }

        return path;
    }

    fn add_delta_transition(&mut self, q1: usize, a: char, q2: usize) {
        match self.delta.get_mut(&q1) {
            Some(dq_1) => {
                self.trans_order_partitions[dq_1.len()].remove(&q1);
                dq_1.insert(a, q2);
                insert_or_push_in_partition(&mut self.trans_order_partitions, q1, dq_1.len());
            }
            None => {
                self.delta.insert(q1, HashMap::from([(a, q2)]));
                self.trans_order_partitions[0].remove(&q1);
                self.trans_order_partitions[1].insert(q1);
            }
        }
        match self.delta_inv.get_mut(&q2) {
            Some(di_q2) => {
                di_q2.insert((a, q1));
            }
            None => {
                self.delta_inv.insert(q2, HashSet::from([(a, q1)]));
            }
        }
    }

    // NOTE: make sure using min_except is enough
    fn lambda_i(&self, i: usize, beta: usize) -> usize {
        let word_prefix_i = &self.min_except[..i].to_vec();
        return min(self.iota + self.lambda_star(word_prefix_i), beta);
    }

    fn lambda_star(&self, word: &Vec<char>) -> usize {
        let mut output = 0;
        let mut state = self.init_state;

        for ch in word {
            output += self.lambda[&state][ch];
            state = self.delta[&state][ch];
        }

        return output;
    }

    fn delete_state(&mut self, state: &usize) {
        if *state == self.init_state {
            panic!("Cannot delete init state!");
        }

        let trans_order = self.delta.get(state).map_or(0, |trans| trans.len());

        self.trans_order_partitions[trans_order].remove(state);

        if let Some(preds) = self.delta_inv.remove(state) {
            for (ch, q) in preds {
                // NOTE: it can't be 0
                let q_trans_order = self.delta.get(&q).map_or(0, |trans| trans.len());
                self.trans_order_partitions[q_trans_order].remove(&q);
                self.trans_order_partitions[q_trans_order - 1].insert(q);
                remove_from_or_delete(&mut self.lambda, &q, &ch);
                remove_from_or_delete(&mut self.delta, &q, &ch);
            }
        }

        if let Some(succs) = self.delta.remove(state) {
            for (ch, q) in succs.iter() {
                if let Some(trans) = self.delta_inv.get_mut(q) {
                    trans.remove(&(*ch, *state));
                }
            }
        }

        self.lambda.remove(state);
        self.states.remove(state);
        self.finality.remove(state);
        self.psi.remove(state);
    }

    fn canonicalise_min_except(&mut self) {
        let t_w = self.state_sequence(&self.min_except);

        // NOTE: t_w shouldn't be empty
        let mut i = t_w.len() - 1;
        let mut curr_state = t_w[i - 1];

        // Find the index at which output can be accumulated the earliest
        loop {
            let has_more_transitions = self
                .delta
                .get(&curr_state)
                .is_some_and(|trans| trans.len() > 1);

            if curr_state == self.init_state
                || self.finality.contains(&curr_state)
                || has_more_transitions
            {
                break;
            }

            i -= 1;
            curr_state = t_w[i];
        }

        let word_output = self.output(&self.min_except);
        let output = word_output - self.lambda_i(i - 1, word_output);

        add_to_or_insert(&mut self.lambda, curr_state, self.min_except[i - 1], output);

        for j in i..t_w.len() {
            curr_state = t_w[j];

            if self.finality.contains(&curr_state) {
                self.psi.insert(curr_state, 0);
            } else {
                add_to_or_insert(&mut self.lambda, curr_state, self.min_except[j], 0);
            }
        }
        // TODO: consider iota update case
    }

    fn longest_common_prefix(&self, word: &Vec<char>) -> Vec<char> {
        let mut state = 0;
        let mut prefix = Vec::new();

        for i in 0..word.len() {
            match self.delta.get(&state).and_then(|trans| trans.get(&word[i])) {
                Some(q) => {
                    state = *q;
                    prefix.push(word[i]);
                }
                None => {
                    break;
                }
            }
        }

        return prefix;
    }
}
