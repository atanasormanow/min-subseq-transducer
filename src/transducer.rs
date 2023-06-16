use std::collections::{HashMap, HashSet};

mod utils;
use utils::{Entry, longest_common_prefix};

// T :: (Sigma, Monoid, Q, Init, F, Delata, Lambda, Iota, Psi)
// Sigma :: [Char]
// Monoid :: Int
// Q :: [Int]
// Init :: Int
// F :: [Int]
// Delta :: (Q, Sigma) -> Q
// Lambda :: (Q, Sigma) -> Monoid
// Iota :: Monoid
// Psi :: F -> Monoid
pub struct Transducer {
    alphabet: HashSet<char>,
    states_with_finality: Vec<bool>,
    init_state: usize,
    delta: HashMap<(usize, char), usize>,
    lambda: HashMap<(usize, char), usize>,
    iota: usize,
    psi: HashMap<usize, usize>,
    min_except: Vec<char>,
}

impl Transducer {
    pub fn from_word(entry: Entry) -> Self {
        let mut alphabet = HashSet::new();
        let mut states_with_finality: Vec<bool> = Vec::with_capacity(10000000);
        let mut delta = HashMap::new();
        let mut lambda = HashMap::new();
        let mut psi = HashMap::new();

        for i in 0..entry.word.len() {
            alphabet.insert(entry.word[i]);
            states_with_finality.push(false);
            delta.insert((i, entry.word[i + 1]), i + 1);
            lambda.insert((i, entry.word[i + 1]), 0);
        }

        states_with_finality[entry.word.len() - 1] = true;
        psi.insert(entry.word.len() - 1, 0);

        return Self {
            alphabet,
            states_with_finality,
            init_state: 0,
            delta,
            lambda,
            iota: entry.output,
            psi,
            min_except: Vec::new(),
        };
    }

    pub fn from_dictionary(dictionary: Vec<String>) -> Self {
        todo!();
    }
}
