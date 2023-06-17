use std::collections::{BTreeSet, HashMap, HashSet};

mod utils;
pub use utils::{longest_common_prefix, Entry};

pub struct Transducer {
    alphabet: HashSet<char>,
    // I use this to avoid shifting when removing states
    states: BTreeSet<usize>,
    // NOTE: If i need to iterate over the final states,
    // i would have to iterate over all the states
    finality: Vec<bool>,
    init_state: usize,
    delta: HashMap<usize, HashMap<char, usize>>,
    lambda: HashMap<usize, HashMap<char, usize>>,
    iota: usize,
    psi: HashMap<usize, usize>,
    min_except: Vec<char>,
}

impl Transducer {
    pub fn from_word(entry: Entry) -> Self {
        let n = entry.word.len();
        let mut alphabet = HashSet::new();
        let mut finality: Vec<bool> = Vec::with_capacity(10000000);
        let mut delta = HashMap::new();
        let mut lambda = HashMap::new();
        let mut psi = HashMap::new();

        for i in 0..n {
            alphabet.insert(entry.word[i]);
            finality.push(false);

            let mut state_transition = HashMap::new();
            state_transition.insert(entry.word[i], i + 1);
            delta.insert(i, state_transition);

            let mut state_output = HashMap::new();
            state_output.insert(entry.word[i], 0);
            lambda.insert(i, state_output);
        }

        finality.push(true);
        psi.insert(n, 0);

        return Self {
            alphabet,
            states: (0..n + 1).collect(),
            finality,
            init_state: 0,
            delta,
            lambda,
            iota: entry.output,
            psi,
            min_except: Vec::new(),
        };
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
    }

    // Make the transducer min except in (last_word ^ new_word)
    fn reduce_except_prefix(&self, next_word: &Vec<char>) {
        let target = longest_common_prefix(&self.min_except, next_word);
        // TODO reduce until target is reached
    }

    fn reduce_except_step(&mut self) {
        let w = &self.min_except;
        let t_w = self.state_sequence(&w);

        let an = w[w.len() - 1];
        let tn = t_w[w.len()];
        let tn_prev = t_w[w.len() - 1]; // Note: will fail if min_except = epsilon

        match self.state_eq(tn, &t_w) {
            Some(q) => {
                self.states.remove(&tn);
                self.finality[q] = false;
                self.delta.remove(&tn);
                self.delta
                    .get_mut(&tn_prev)
                    .and_then(|tn_prev_trans| tn_prev_trans.insert(an, q));
                self.lambda.remove(&tn);
                self.psi.remove(&tn);
            }
            None => {
                self.min_except.pop();
            }
        }
    }

    // TODO: How to check for equal states?
    // 1) separate states in buckets based on their number of transitions
    // 2) check if tn is equal to some state with the same number of transitions
    fn state_eq(&self, state: usize, t_w: &Vec<usize>) -> Option<usize> {
        todo!();
    }

    // delta[(q,a)] will panic if delta is not defined
    // TODO: make private
    pub fn state_sequence(&self, w: &Vec<char>) -> Vec<usize> {
        let mut next = self.init_state;
        let mut states = vec![next];

        for i in 0..w.len() {
            next = self.delta[&next][&w[i]];
            states.push(next);
        }

        return states;
    }

    pub fn add_word_in_order(&self, word: Vec<char>) {
        todo!();
    }

    pub fn from_dictionary(dictionary: Vec<(Vec<char>, usize)>) -> Self {
        todo!();
    }
}
