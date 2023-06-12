use std::{collections::HashMap, vec, hash::Hash};

type Sigma = Vec<char>;

type M = usize;
type Monoid = Vec<M>;

type Q = usize;
type F = Q;
type States = Vec<Q>;

pub struct Transducer {
    input_alphabet: Sigma,
    output_alphabet: Monoid,
    states: States,
    init_state: Q,
    states_finality: Vec<bool>,
    delta: HashMap<(Q, char), Q>,
    lambda: HashMap<(Q, char), M>,
    psi: HashMap<F, M>,
}

impl Transducer {
    pub fn from_dictionary(dictionary: Vec<String>) -> Self {
        for s in dictionary {
            // TODO: construct transducer components
            print!("{}", s);
        }

        // TODO: construct with result components
        Self {
            input_alphabet: vec![],
            output_alphabet: vec![],
            states: vec![],
            init_state: 0,
            states_finality: vec![],
            delta: HashMap::new(),
            lambda: HashMap::new(),
            psi: HashMap::new(),
        }
    }
}
