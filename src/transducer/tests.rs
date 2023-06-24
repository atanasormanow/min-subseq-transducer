#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeSet, HashMap, HashSet},
        vec,
    };

    use crate::transducer::{utils::longest_common_prefix, Transducer};

    #[test]
    fn transducer_from_entry() {
        let transducer = Transducer::from_entry("baba", 10);

        assert_eq!(transducer.alphabet, HashSet::from(['a', 'b']));
        assert_eq!(transducer.states, BTreeSet::from([0, 1, 2, 3, 4]));
        assert_eq!(transducer.finality, BTreeSet::from([4]));
        assert_eq!(transducer.init_state, 0);
        assert_eq!(
            transducer.delta,
            HashMap::from([
                (0, HashMap::from([('b', 1)])),
                (1, HashMap::from([('a', 2)])),
                (2, HashMap::from([('b', 3)])),
                (3, HashMap::from([('a', 4)])),
            ])
        );
        assert_eq!(
            transducer.delta_inv,
            HashMap::from([
                (4, HashSet::from([('a', 3)])),
                (3, HashSet::from([('b', 2)])),
                (2, HashSet::from([('a', 1)])),
                (1, HashSet::from([('b', 0)])),
            ])
        );
        assert_eq!(
            transducer.lambda,
            HashMap::from([
                (0, HashMap::from([('b', 0)])),
                (1, HashMap::from([('a', 0)])),
                (2, HashMap::from([('b', 0)])),
                (3, HashMap::from([('a', 0)])),
            ])
        );
        assert_eq!(transducer.iota, 10);
        assert_eq!(transducer.psi, HashMap::from([(4, 0)]));
        assert_eq!(transducer.min_except, vec!['b', 'a', 'b', 'a']);
        assert_eq!(
            transducer.trans_order_partitions,
            Vec::from([BTreeSet::from([4]), BTreeSet::from([0, 1, 2, 3]),])
        );
    }

    #[test]
    fn transducer_from_entry_and_add_once() {
        let mut transducer = Transducer::from_entry("baba", 10);
        transducer.add_entry_in_order("bc", 15);

        assert_eq!(transducer.alphabet, HashSet::from(['a', 'b', 'c']));
        assert_eq!(transducer.states, BTreeSet::from([0, 1, 2, 3, 4, 5]));
        assert_eq!(transducer.finality, BTreeSet::from([4, 5]));
        assert_eq!(transducer.init_state, 0);
        assert_eq!(
            transducer.delta,
            HashMap::from([
                (0, HashMap::from([('b', 1)])),
                (1, HashMap::from([('a', 2), ('c', 5)])),
                (2, HashMap::from([('b', 3)])),
                (3, HashMap::from([('a', 4)])),
            ])
        );
        assert_eq!(
            transducer.delta_inv,
            HashMap::from([
                (5, HashSet::from([('c', 1)])),
                (4, HashSet::from([('a', 3)])),
                (3, HashSet::from([('b', 2)])),
                (2, HashSet::from([('a', 1)])),
                (1, HashSet::from([('b', 0)])),
            ])
        );
        assert_eq!(
            transducer.lambda,
            HashMap::from([
                (0, HashMap::from([('b', 0)])),
                (1, HashMap::from([('a', 0), ('c', 5)])),
                (2, HashMap::from([('b', 0)])),
                (3, HashMap::from([('a', 0)])),
            ])
        );
        assert_eq!(transducer.iota, 10);
        assert_eq!(transducer.psi, HashMap::from([(4, 0), (5, 0)]));
        assert_eq!(transducer.min_except, vec!['b', 'c']);
        assert_eq!(
            transducer.trans_order_partitions,
            Vec::from([
                BTreeSet::from([4, 5]),
                BTreeSet::from([0, 2, 3]),
                BTreeSet::from([1])
            ])
        );
    }

    #[test]
    fn transducer_from_dictionary() {
        let dictionary = vec![("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)];
        let transducer = Transducer::from_dictionary(dictionary);

        assert_eq!(transducer.alphabet, HashSet::from(['a', 'b', 'c', 'd']));
        assert_eq!(transducer.states, BTreeSet::from([0, 1, 2, 3, 4, 5, 7]));
        assert_eq!(transducer.finality, BTreeSet::from([3, 5]));
        assert_eq!(transducer.init_state, 0);
        assert_eq!(
            transducer.delta,
            HashMap::from([
                (0, HashMap::from([('c', 1)])),
                (1, HashMap::from([('b', 7), ('a', 2)])),
                (2, HashMap::from([('b', 3), ('d', 5)])),
                (3, HashMap::from([('a', 4)])),
                (4, HashMap::from([('b', 5)])),
                (7, HashMap::from([('a', 4)])),
            ])
        );
        assert_eq!(
            transducer.delta_inv,
            HashMap::from([
                (1, HashSet::from([('c', 0)])),
                (2, HashSet::from([('a', 5)])),
                (3, HashSet::from([('b', 2)])),
                (4, HashSet::from([('a', 3), ('a', 7)])),
                (5, HashSet::from([('b', 4), ('d', 2)])),
                (7, HashSet::from([('b', 1)])),
            ])
        );
        assert_eq!(
            transducer.lambda,
            HashMap::from([
                (0, HashMap::from([('c', 0)])),
                (1, HashMap::from([('b', 0), ('a', 5)])),
                (2, HashMap::from([('b', 2), ('d', 0)])),
                (3, HashMap::from([('a', 0)])),
                (4, HashMap::from([('b', 0)])),
                (7, HashMap::from([('a', 0)])),
            ])
        );
        assert_eq!(transducer.iota, 3);
        assert_eq!(transducer.psi, HashMap::from([(3, 5), (5, 0)]));
        assert_eq!(transducer.min_except, Vec::new());
        assert_eq!(
            transducer.trans_order_partitions,
            Vec::from([
                BTreeSet::from([5]),
                BTreeSet::from([0, 3, 4, 7]),
                BTreeSet::from([1, 2]),
            ])
        );
    }

    #[test]
    fn delete_state_from_transducer() {
        let mut transducer = example_transducer();
        transducer.delete_state(&3);

        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 4, 5, 7]);
        let finality = BTreeSet::from([5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('b', 7), ('a', 2)])),
            (2, HashMap::from([('d', 5)])),
            (4, HashMap::from([('b', 5)])),
            (7, HashMap::from([('a', 4)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 5)])),
            (4, HashSet::from([('a', 7)])),
            (5, HashSet::from([('b', 4), ('d', 2)])),
            (7, HashSet::from([('b', 1)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('b', 0), ('a', 5)])),
            (2, HashMap::from([('b', 2), ('d', 0)])),
            (4, HashMap::from([('b', 0)])),
            (7, HashMap::from([('a', 0)])),
        ]);
        let iota = 3;
        let psi = HashMap::from([(5, 0)]);
        let min_except = Vec::new();
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 4, 7]),
            BTreeSet::from([1, 2]),
        ]);

        assert_eq!(transducer.alphabet, alphabet);
        assert_eq!(transducer.states, states);
        assert_eq!(transducer.finality, finality);
        assert_eq!(transducer.init_state, init_state);
        assert_eq!(transducer.delta, delta);
        assert_eq!(transducer.delta_inv, delta_inv);
        assert_eq!(transducer.lambda, lambda);
        assert_eq!(transducer.iota, iota);
        assert_eq!(transducer.psi, psi);
        assert_eq!(transducer.min_except, min_except);
        assert_eq!(transducer.trans_order_partitions, trans_order_partitions);
    }

    #[test]
    fn add_entry_in_order() {
        let mut transducer = Transducer::from_entry("cab", 15);
        transducer.add_entry_in_order("cabab", 10);

        println!("-> transducer for {{ cab -> 15 , cabab -> 10}}");
        transducer.print();
    }

    #[test]
    fn find_state_sequence_for_a_word() {
        let transducer = example_transducer();

        assert_eq!(
            transducer.state_sequence(&vec!['c', 'a', 'b']),
            vec![0, 1, 2, 3]
        );
        assert_eq!(transducer.state_sequence(&vec!['c', 'a']), vec![0, 1, 2]);
        assert_eq!(transducer.state_sequence(&vec![]), vec![0]);
    }

    #[test]
    #[should_panic]
    fn invalid_state_sequence_call() {
        let transducer = example_transducer();
        transducer.state_sequence(&vec!['c', 'a', 'c']);
    }

    #[test]
    fn reduce_except_by_one_char() {
        let mut transducer = example_transducer();
        transducer.reduce_except_by_one();
        // TODO
    }

    #[test]
    fn add_delta_and_lambda_transitions() {
        let mut transducer = Transducer::from_entry("cab", 15);

        transducer.add_delta_transition(3, 'a', 4);
        transducer.add_lambda_transition(3, 'a', 123);

        assert_eq!(transducer.delta[&3][&'a'], 4);
        assert_eq!(transducer.lambda[&3][&'a'], 123);

        transducer.add_delta_transition(3, 'b', 5);
        transducer.add_lambda_transition(3, 'b', 321);

        assert_eq!(transducer.delta[&3][&'b'], 5);
        assert_eq!(transducer.lambda[&3][&'b'], 321);
    }

    #[test]
    fn find_longest_common_prefix() {
        let result = longest_common_prefix(&vec!['c', 'a', 'b'], &vec!['c', 'a', 'd']);
        assert_eq!(result, vec!['c', 'a']);
    }

    #[test]
    fn lambda_star() {
        let transducer = example_transducer();
        assert_eq!(transducer.lambda_star(&vec!['c', 'a', 'b', 'a']), 7);
        assert_eq!(transducer.lambda_star(&vec!['c', 'a']), 5);
    }

    #[test]
    fn transducer_output_function() {
        let transducer = example_transducer();
        assert_eq!(transducer.output(&vec!['c', 'a', 'b']), 15);
        assert_eq!(transducer.output(&vec!['c', 'a', 'b', 'a', 'b']), 10);
        assert_eq!(transducer.output(&vec!['c', 'a', 'd']), 8);
    }

    // Helper functions
    ///////////////////
    fn example_transducer() -> Transducer {
        // let dictionary = vec![("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)];
        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5, 7]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('b', 7), ('a', 2)])),
            (2, HashMap::from([('b', 3), ('d', 5)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
            (7, HashMap::from([('a', 4)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 5)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3), ('a', 7)])),
            (5, HashSet::from([('b', 4), ('d', 2)])),
            (7, HashSet::from([('b', 1)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('b', 0), ('a', 5)])),
            (2, HashMap::from([('b', 2), ('d', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
            (7, HashMap::from([('a', 0)])),
        ]);
        let iota = 3;
        let psi = HashMap::from([(3, 5), (5, 0)]);
        let min_except = Vec::new();
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 3, 4, 7]),
            BTreeSet::from([1, 2]),
        ]);

        return Transducer {
            alphabet,
            states,
            finality,
            init_state,
            delta,
            delta_inv,
            lambda,
            iota,
            psi,
            min_except,
            trans_order_partitions,
        };
    }
}
