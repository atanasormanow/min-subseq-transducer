use std::{
    cmp::{max, min},
    collections::{BTreeSet, HashMap, HashSet},
    panic,
};

mod tests;
mod utils;
use utils::{add_to_or_insert, longest_common_prefix, remove_from_or_delete};

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
    states_by_signature: HashMap<(Option<usize>, BTreeSet<(char, usize, usize)>), usize>,
}

impl Transducer {
    pub fn get_states(&self) -> &BTreeSet<usize> {
        return &self.states;
    }

    pub fn get_finality(&self) -> &BTreeSet<usize> {
        return &self.finality;
    }

    pub fn get_initial_output(&self) -> usize {
        return self.iota;
    }

    /** Adds a new entry to the transducer,
     * that is lexicographically greater than the last added entry*/
    pub fn add_entry_in_order(&mut self, word: &str, output: usize) {
        let word: Vec<char> = word.chars().collect();
        let n = word.len();
        let k = longest_common_prefix(&self.min_except, &word).len();

        self.reduce_except_by_k(self.min_except.len() - k);

        let tk = *self.state_sequence(&self.min_except).last().unwrap_or(&0);

        self.update_alphabet_with_word(&word[k..n]);

        let tkn = self.add_new_states(n - k);

        // Add the transitions for the missing suffix
        for i in 1..(n - k) {
            self.add_delta_transition(tkn[i - 1], word[k + i], tkn[i]);
        }

        // Make the last state of word final
        self.finality.insert(*tkn.last().unwrap_or(&tk));

        // Add a transition from the existing prefix
        if n - k > 0 {
            self.add_delta_transition(tk, word[k], tkn[0]);
        }

        let word_states = self.state_sequence(&word);

        // Update final outputs
        for i in 1..=k {
            if self.finality.contains(&word_states[i]) {
                let final_output = self.output(&word[..i].to_vec()) - self.lambda_i(i, output);
                self.psi.insert(word_states[i], final_output);
            }
        }
        word_states.last().and_then(|tm| self.psi.insert(*tm, 0));

        // Update output transitions
        //
        // NOTE: the first and last updates of lambda both depend on the old lambda.
        // This means that the updates have to be done simultaneously.
        let mut postponed_lambda_updates: Vec<(usize, char, usize)> = Vec::new();
        for i in 1..=k {
            let curr_output = self.lambda_i(i, output);
            let prev_output = self.lambda_i(i - 1, output);
            postponed_lambda_updates.push((
                word_states[i - 1],
                self.min_except[i - 1],
                curr_output - prev_output,
            ));
        }

        if n - k > 0 {
            let lambda_k = self.lambda_i(k, output);
            add_to_or_insert(&mut self.lambda, word_states[k], word[k], output - lambda_k);
        }

        for i in 1..(n - k) {
            add_to_or_insert(&mut self.lambda, tkn[i - 1], word[k + i], 0);
        }

        for i in 0..=k {
            for ch in self.alphabet.iter() {
                let is_lambda_defined = self
                    .lambda
                    .get(&word_states[i])
                    .and_then(|trans| trans.get(ch))
                    .is_some();

                if i < word.len() && *ch != word[i] && is_lambda_defined {
                    let mut prefix_with_ch = word[0..i].to_vec();
                    prefix_with_ch.push(*ch);

                    let output =
                        self.iota + self.lambda_star(&prefix_with_ch) - self.lambda_i(i, output);

                    postponed_lambda_updates.push((word_states[i], *ch, output));
                }
            }
        }

        for (q, a, o) in postponed_lambda_updates {
            add_to_or_insert(&mut self.lambda, q, a, o);
        }

        if n - k == 0 {
            let tn_output = max(output, self.output(&word)) - self.output(&word);
            word_states
                .last()
                .and_then(|tm| self.psi.insert(*tm, tn_output));
        }

        // Update iota last, as lambda and psi use the old value
        self.iota = min(self.iota, output);

        // The resulting Transducer is minimal except in the new_entry
        self.min_except = word;
    }

    /** Adds a new entry to the transducer,
     * that is NOT lexicographically greater than the last added entry*/
    pub fn add_entry_out_of_order(&mut self, word: &str, output: usize) {
        let word_vec = word.chars().collect();
        let word_lcp = self.longest_common_prefix(&word_vec);

        self.increase_except_from_epsilon_to_word(&word_lcp);
        self.add_entry_in_order(word, output);
        self.reduce_to_epsilon();
    }

    /** Removes the entry with the given word from the transducer */
    pub fn remove_entry_with_word(&mut self, word_raw: &str) {
        if word_raw.is_empty() {
            panic!("The transducer cannot take epsilon as input!");
        }

        let word = word_raw.chars().collect();
        self.increase_except_from_epsilon_to_word(&word);

        let mut t_w = self.state_sequence(&word);
        t_w.reverse();

        self.print_with_message("After incresing min except");
        println!("Searching for prev div state of {:?}", t_w[0]);

        // Delete only if the current word has no continuation
        if self.delta.get(&t_w[0]).is_none() {
            // TODO: This won't work if the word, that is being deleted,
            // is the last one in the dictionary
            let (_, prev_div_state) = self
                .find_prev_divergent_state(&t_w[0])
                .expect("Shouldn't be removing an entry with epsilon!");

            for i in 0..t_w.len() {
                if t_w[i] != prev_div_state {
                    self.delete_state(&t_w[i]);
                    self.min_except.pop();
                } else {
                    break;
                }
            }
        }

        self.finality.remove(&t_w[0]);
        self.psi.remove(&t_w[0]);

        self.canonicalise_min_except();
        self.reduce_to_epsilon();
    }

    /** Constructs a minimal subsequential transducer from a dictionary of entries */
    pub fn from_dictionary(dictionary: Vec<(&str, usize)>) -> Self {
        if dictionary.is_empty() {
            panic!("Cannot construct empty transducer!");
        }

        let (w, o) = dictionary[0];
        let mut transducer = Transducer::from_entry_with_capacity(w, o, dictionary.len() * 30);
        let mut n = 1;

        for e in &dictionary[1..] {
            let (w, o) = *e;
            // println!("{:?} Adding {:?}", n, w);
            transducer.add_entry_in_order(w, o);
            n += 1;
        }

        transducer.reduce_to_epsilon();
        return transducer;
    }

    /** Returns the output of a given word from the transducer */
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
        println!("T states by signature: {:?}", self.states_by_signature);
    }

    pub fn print_with_message(&self, message: &str) {
        println!("{:?}", message);
        self.print();
    }

    pub fn get_number_of_transitions(&self) -> usize {
        let mut n = 0;
        for (_, trans) in &self.delta {
            n += trans.len();
        }
        return n;
    }

    // ////////////////
    // Private functions:
    // ///////////////////
    //
    /** Constructs the trivial minimal subsequential transducer from a single entry */
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
            states_by_signature: HashMap::new(),
        };
    }

    /** Like from_entry but initializes some HashMaps with a given capacity */
    fn from_entry_with_capacity(word: &str, output: usize, capacity: usize) -> Self {
        let word: Vec<char> = word.chars().collect();
        let n = word.len();

        let mut alphabet = HashSet::new();
        let mut delta = HashMap::with_capacity(capacity);
        let mut delta_inv = HashMap::with_capacity(capacity);
        let mut lambda = HashMap::with_capacity(capacity);

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
            states_by_signature: HashMap::with_capacity(capacity),
        };
    }

    /** Reduces the word that the transducer is minimal except by one character (from the right) */
    fn reduce_except_by_one(&mut self) {
        if self.min_except.is_empty() {
            panic!("Transduser must be minimal except in non-empty word!");
        }

        let word = &self.min_except;
        let t_w = self.state_sequence(&word);
        let n = word.len();
        let an = word[n - 1];

        if let Some(q) = self.state_eq(t_w[n]) {
            let prev_output = self.lambda[&t_w[n - 1]][&an];
            self.delete_state(&t_w[n]);
            self.add_delta_transition(t_w[n - 1], an, q);
            add_to_or_insert(&mut self.lambda, t_w[n - 1], an, prev_output);
        } else {
            self.add_signature(t_w[n]);
        }

        self.min_except.pop();
    }

    /** Reduces the word that the transducer is minimal except by k characters (from the right) */
    fn reduce_except_by_k(&mut self, k: usize) {
        for _ in 0..k {
            self.reduce_except_by_one();
        }
    }

    /** Makes the transducer minimal (except in epsilon) */
    fn reduce_to_epsilon(&mut self) {
        for _ in 0..self.min_except.len() {
            self.reduce_except_by_one();
        }
        self.add_signature(self.init_state);
    }

    /** Makes a minimal subsequential transducer minimal except in a given word */
    fn increase_except_from_epsilon_to_word(&mut self, word: &Vec<char>) {
        if !self.min_except.is_empty() {
            panic!("Transduser must be minimal except in epsilon!");
        }

        let mut current_state = self.init_state;
        let mut max_state = *self.states.last().expect("States cannot be empty!");

        self.remove_signature(current_state);

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

                // Clone the transitions of the convergent successor
                if let Some(trans) = self.delta.get(&next_state).cloned() {
                    for (ch, q) in trans {
                        self.add_delta_transition(new_state, ch, q);
                    }
                }
                if let Some(output_trans) = self.lambda.get(&next_state).cloned() {
                    self.lambda.insert(new_state, output_trans);
                }

                current_state = new_state;
            } else {
                self.remove_signature(next_state);
                current_state = next_state;
            }

            self.min_except.push(word[i]);
        }
    }

    /** Checks if a state is convergent, meaning it has more than one ingoing transitions */
    fn is_state_convergent(&self, state: usize) -> bool {
        return self
            .delta_inv
            .get(&state)
            .is_some_and(|trans| trans.len() > 1);
    }

    /** Searches for an equivalent state of `state` outside of t_w */
    fn state_eq(&self, q: usize) -> Option<usize> {
        let state_sig = self.signature(q);

        if let Some(q_eq) = self.states_by_signature.get(&state_sig) {
            if *q_eq != q {
                return Some(*q_eq);
            } else {
                println!("FYI - this state already has this signature");
            }
        }

        return None;
    }

    // NOTE: delta[(q,a)] will panic if delta is not defined
    /** Finds the state sequence, corresponding to a given word */
    fn state_sequence(&self, w: &Vec<char>) -> Vec<usize> {
        let mut next = self.init_state;
        let mut path = vec![next];

        for i in 0..w.len() {
            next = self.delta[&next][&w[i]];
            path.push(next);
        }

        return path;
    }

    /** Adds a delta transition, overwriting existing transition from the given state with the
     * given character. Updates delta_inv but does NOT update state signatures! */
    fn add_delta_transition(&mut self, q1: usize, a: char, q2: usize) {
        match self.delta.get_mut(&q1) {
            Some(dq_1) => {
                dq_1.insert(a, q2);
            }
            None => {
                self.delta.insert(q1, HashMap::from([(a, q2)]));
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

    fn lambda_i(&self, i: usize, beta: usize) -> usize {
        let word_prefix_i = &self.min_except[..i].to_vec();
        return min(self.iota + self.lambda_star(word_prefix_i), beta);
    }

    /** Returns the accumulated transition output for a given word  */
    fn lambda_star(&self, word: &Vec<char>) -> usize {
        let mut output = 0;
        let mut state = self.init_state;

        for ch in word {
            output += self.lambda[&state][ch];
            state = self.delta[&state][ch];
        }

        return output;
    }

    /** Completely deletes a given state */
    fn delete_state(&mut self, state: &usize) {
        if *state == self.init_state {
            panic!("Cannot delete init state!");
        }

        if let Some(preds) = self.delta_inv.remove(state) {
            for (ch, q) in preds {
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
        let tn = *self
            .state_sequence(&self.min_except)
            .last()
            .expect("State sequence cannot be empty!");

        let mut carry = self.extract_min_from_state(&tn);
        let mut prev_div_state = self.find_prev_divergent_state(&tn);

        // NOTE: this is a transition is from delta_inv
        while let Some((ch, q)) = prev_div_state {
            self.lambda.entry(q).and_modify(|trans| {
                trans.entry(ch).and_modify(|o| {
                    *o += carry;
                });
            });

            carry = self.extract_min_from_state(&q);
            prev_div_state = self.find_prev_divergent_state(&q);
        }

        self.iota += carry;
    }

    /** Decreases all outputs of a state with their minimum and returns the found minimum */
    fn extract_min_from_state(&mut self, state: &usize) -> usize {
        let mut outputs: Vec<usize> = self.lambda[state].iter().map(|(_, v)| *v).collect();
        if let Some(v) = self.psi.get(state) {
            outputs.push(*v);
        }
        let min_output = *outputs.iter().min().unwrap_or(&0);

        self.psi.entry(*state).and_modify(|o| {
            *o -= min_output;
        });

        if let Some(trans) = self.lambda.get_mut(state) {
            for ch in trans.clone().keys() {
                trans.entry(*ch).and_modify(|o| {
                    *o -= min_output;
                });
            }
        }

        return min_output;
    }

    /** Going backwards from a state, finds the first state (and it's char for transition),
     * that is final or has more than 1 outgoing transitions. This works only if there are no
     * convergent states along the path */
    fn find_prev_divergent_state(&self, state: &usize) -> Option<(char, usize)> {
        // NOTE: assumes that the state has exactly one predecessor
        let (mut curr_ch, mut curr_state) = self.delta_inv[state].iter().last().unwrap();

        // NOTE: This shouldn't loop infinitely
        loop {
            if self.is_state_divergent(&curr_state) {
                return Some((curr_ch, curr_state));
            }

            if curr_state == self.init_state {
                return None;
            }

            (curr_ch, curr_state) = *self.delta_inv[&curr_state].iter().last().unwrap();
        }
    }

    /** Finds the longest prefix of the words that the transducer reads */
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

    /** Add k new states to the transducer */
    fn add_new_states(&mut self, k: usize) -> Vec<usize> {
        let max_state = *self.states.last().unwrap_or(&0);
        let mut new_states = Vec::new();

        for i in 1..=k {
            self.states.insert(max_state + i);
            new_states.push(max_state + i);
        }

        return new_states;
    }

    /** Add all new characters of a word in the transducer's alphabet */
    fn update_alphabet_with_word(&mut self, word: &[char]) {
        for ch in word {
            self.alphabet.insert(*ch);
        }
    }

    /** Checks if a state is final or has more than one outgoing transitions */
    fn is_state_divergent(&self, state: &usize) -> bool {
        return self.delta.get(state).is_some_and(|trans| trans.len() > 1)
            || self.finality.contains(state);
    }

    fn signature(&self, q: usize) -> (Option<usize>, BTreeSet<(char, usize, usize)>) {
        let final_output = self.psi.get(&q).map(|o| *o);
        let mut transitions = BTreeSet::new();

        for ch in &self.alphabet {
            if let Some(q_dest) = self.delta.get(&q).and_then(|q_trans| q_trans.get(&ch)) {
                let q_out = self
                    .lambda
                    .get(&q)
                    .and_then(|q_out_trans| q_out_trans.get(&ch))
                    .expect("Lambda must be defined if delta is defined");
                transitions.insert((*ch, *q_dest, *q_out));
            }
        }

        return (final_output, transitions);
    }

    fn add_signature(&mut self, q: usize) {
        self.states_by_signature.insert(self.signature(q), q);
    }

    fn remove_signature(&mut self, q: usize) {
        self.states_by_signature.remove(&self.signature(q));
    }
}
