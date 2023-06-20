use std::{
    cmp::min,
    collections::{BTreeSet, HashMap, HashSet},
    panic,
};

mod tests;
mod utils;
use utils::longest_common_prefix;

pub struct Entry {
    pub word: Vec<char>,
    pub output: usize,
}

impl Entry {
    pub fn new(word: &str, output: usize) -> Self {
        Self {
            word: word.chars().collect(),
            output,
        }
    }

    pub fn from_tuple((word, output): (&str, usize)) -> Self {
        Entry::new(word, output)
    }
}

pub struct Transducer {
    pub alphabet: HashSet<char>,
    pub states: BTreeSet<usize>,
    pub finality: HashSet<usize>,
    pub init_state: usize,
    pub delta: HashMap<usize, HashMap<char, usize>>,
    pub lambda: HashMap<usize, HashMap<char, usize>>,
    pub iota: usize,
    pub psi: HashMap<usize, usize>,
    pub min_except: Vec<char>,
    pub trans_order_partitions: Vec<BTreeSet<usize>>,
}

impl Transducer {
    pub fn from_entry(word: &str, output: usize) -> Self {
        let entry = Entry::new(word, output);
        let n = entry.word.len();

        let mut alphabet = HashSet::new();
        let mut delta = HashMap::new();
        let mut lambda = HashMap::new();

        for i in 0..n {
            alphabet.insert(entry.word[i]);

            let state_transition = HashMap::from([(entry.word[i], i + 1)]);
            delta.insert(i, state_transition);

            let state_output = HashMap::from([(entry.word[i], 0)]);
            lambda.insert(i, state_output);
        }

        return Self {
            alphabet,
            states: (0..=n).collect(),
            finality: HashSet::from([n]),
            init_state: 0,
            delta,
            lambda,
            iota: entry.output,
            psi: HashMap::from([(n, 0)]),
            min_except: entry.word,
            trans_order_partitions: vec![BTreeSet::from([n]), (0..n).collect()],
        };
    }

    pub fn add_entry_in_order(&mut self, word: &str, output: usize) {
        let new_entry = Entry::new(word, output);
        let t_w = self.state_sequence(&self.min_except);
        let k = longest_common_prefix(&self.min_except, &new_entry.word).len();
        let new_state = self.finality.len(); // NOTE: states are never removed from finality
        let new_suffix_len = self.min_except.len() - k;

        // Make the transducer min except in (last_entry ^ new_entry)
        for _ in 0..new_suffix_len {
            self.reduce_except_by_one();
        }

        // Add new states for the missing suffix
        for i in 0..new_suffix_len {
            self.states.insert(new_state + i);
        }

        self.finality.insert(new_state + new_suffix_len);

        // Add a transition from the existing state
        self.add_delta_transition(k, new_entry.word[0], new_state + k + 1);
        // Add the transitions for the missing suffix
        for i in 0..new_suffix_len {
            self.add_delta_transition(
                new_state + i,
                new_entry.word[new_entry.word.len() + i + 1],
                new_state + i + 1,
            );
        }

        self.iota = min(self.iota, new_entry.output);

        for t in &t_w {
            self.lambda.remove(t);
        }
        for i in 1..k {
            let curr_output = self.lambda_i(i, new_entry.output);
            let prev_output = self.lambda_i(i - 1, new_entry.output);
            self.add_lambda_transition(t_w[i], self.min_except[i], curr_output - prev_output);
        }
        self.add_lambda_transition(
            t_w[t_w.len() - 1],
            new_entry.word[0],
            new_entry.output - self.lambda_i(k - 1, new_entry.output),
        ); // TODO: more lambda updates

        // The resulting Transducer is minimal except in the new_entry
        self.min_except = new_entry.word;
    }

    pub fn from_dictionary(dictionary: Vec<(&str, usize)>) -> Self {
        if dictionary.is_empty() {
            panic!("Cannot construct empty transducer (yet)!");
        }

        let (w, o) = dictionary[0];
        let mut transducer = Transducer::from_entry(w, o);

        for e in &dictionary[1..] {
            let (w, o) = *e;
            transducer.add_entry_in_order(w, o);
        }

        return transducer;
    }

    pub fn print(&self) {
        println!("T alphabet: {:?}", self.alphabet);
        println!("T states: {:?}", self.states);
        println!("T finality: {:?}", self.finality);
        println!("T init_state: {:?}", self.init_state);
        println!("T delta: {:?}", self.delta);
        println!("T lambda: {:?}", self.lambda);
        println!("T iota: {:?}", self.iota);
        println!("T psi: {:?}", self.psi);
        println!("T min_except: {:?}", self.min_except);
        println!(
            "T partitions by transition order: {:?}",
            self.trans_order_partitions
        );
    }

    ///////////////////
    // Private functions:
    //////////////////////
    pub fn reduce_except_by_one(&mut self) {
        let w = &self.min_except;
        let t_w = self.state_sequence(&w);

        let an = w[w.len() - 1];
        let tn = t_w[w.len()];
        let tn_prev = t_w[w.len() - 1]; // Note: will fail if min_except = epsilon

        // TODO: should do this for every equal state
        // TODO: refactor as None arm is empty
        match self.state_eq(tn, &t_w) {
            Some(q) => {
                self.states.remove(&tn);
                self.finality.remove(&tn);

                let state_trans_order = self.delta.get(&tn).map_or(0, |trans| trans.len());
                self.trans_order_partitions[state_trans_order].remove(&tn);

                self.delta.remove(&tn);
                self.delta
                    .get_mut(&tn_prev)
                    .and_then(|tn_prev_trans| tn_prev_trans.insert(an, q));

                self.lambda.remove(&tn);
                self.psi.remove(&tn);
            }
            None => {}
        }

        self.min_except.pop();
    }

    // Check for equal states by:
    // 1) have states partitioned based on their number of transitions
    // 2) check if tn is equal to some state with the same number of transitions
    // TODO: test, optimise, refactor and test again
    pub fn state_eq(&self, state: usize, t_w: &Vec<usize>) -> Option<usize> {
        let state_is_final = self.finality.contains(&state);
        // No transitions if delta(state) is undefined
        let state_trans_order = self.delta.get(&state).map_or(0, |trans| trans.len());

        for q in &self.trans_order_partitions[state_trans_order] {
            if t_w.contains(q) {
                continue;
            }

            let cond1 = state_is_final == self.finality.contains(q);
            let cond2 = !state_is_final || self.psi[&state] == self.psi[q];
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
            Some(dq1) => {
                dq1.insert(a, q2);
            }
            None => {
                let q1_trans = HashMap::from([(a, q2)]);
                self.delta.insert(q1, q1_trans);
            }
        }
    }

    fn add_lambda_transition(&mut self, q1: usize, a: char, m: usize) {
        match self.lambda.get_mut(&q1) {
            Some(dq1) => {
                dq1.insert(a, m);
            }
            None => {
                let q1_trans = HashMap::from([(a, m)]);
                self.lambda.insert(q1, q1_trans);
            }
        }
    }

    // NOTE: make sure using min_except is enough
    fn lambda_i(&self, i: usize, beta: usize) -> usize {
        let word_prefix_i = &self.min_except[..=i].to_vec();
        return min(self.iota + self.lambda_star(word_prefix_i), beta);
    }

    // TODO: seems like this has taken a word that is not in the language
    // accumulate word output, starting from the initial state
    fn lambda_star(&self, word: &Vec<char>) -> usize {
        let mut output = 0;
        let mut state = self.init_state;

        for ch in word {
            output += self.lambda[&state][ch];
            state = self.delta[&state][ch];
        }

        return output;
    }
}
