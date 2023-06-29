#[cfg(test)]

// TODO: add a test for a bigger dictionary
mod tests {
    use std::{
        collections::{BTreeSet, HashMap, HashSet},
        vec,
    };

    use crate::transducer::{
        utils::{add_to_or_insert, longest_common_prefix},
        Transducer,
    };

    #[test]
    fn constructs_the_transducer_from_entry() {
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
    fn adds_a_small_entry_out_of_order() {
        let dictionary = vec![("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)];
        let mut transducer = Transducer::from_dictionary(dictionary);
        transducer.add_entry_out_of_order("ca", 9);

        let expected_transducer = example_transducer();

        assert_eq!(transducer.alphabet, expected_transducer.alphabet);
        assert_eq!(transducer.states, expected_transducer.states);
        assert_eq!(transducer.finality, BTreeSet::from([2, 3, 5]));
        assert_eq!(transducer.init_state, expected_transducer.init_state);
        assert_eq!(transducer.delta, expected_transducer.delta);
        assert_eq!(transducer.delta_inv, expected_transducer.delta_inv);
        assert_eq!(transducer.lambda, expected_transducer.lambda);
        assert_eq!(transducer.iota, expected_transducer.iota);
        assert_eq!(transducer.psi, HashMap::from([(2, 1), (3, 5), (5, 0)]));
        assert_eq!(transducer.min_except, expected_transducer.min_except);
        assert_eq!(
            transducer.trans_order_partitions,
            expected_transducer.trans_order_partitions
        );
    }

    #[test]
    fn removes_a_short_word() {
        let dictionary = vec![("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)];
        let mut transducer = Transducer::from_dictionary(dictionary);
        transducer.remove_entry_with_word("cab");

        assert_eq!(transducer.alphabet, HashSet::from(['a', 'b', 'c', 'd']));
        assert_eq!(transducer.states, BTreeSet::from([0, 1, 2, 4, 5, 6]));
        assert_eq!(transducer.finality, BTreeSet::from([5]));
        assert_eq!(transducer.init_state, 0);
        assert_eq!(
            transducer.delta,
            HashMap::from([
                (0, HashMap::from([('c', 1)])),
                (1, HashMap::from([('b', 6), ('a', 2)])),
                (2, HashMap::from([('b', 6), ('d', 5)])),
                (4, HashMap::from([('b', 5)])),
                (6, HashMap::from([('a', 4)])),
            ])
        );
        assert_eq!(
            transducer.delta_inv,
            HashMap::from([
                (1, HashSet::from([('c', 0)])),
                (2, HashSet::from([('a', 1)])),
                (4, HashSet::from([('a', 6)])),
                (5, HashSet::from([('b', 4), ('d', 2)])),
                (6, HashSet::from([('b', 1), ('b', 2)])),
            ])
        );
        assert_eq!(
            transducer.lambda,
            HashMap::from([
                (0, HashMap::from([('c', 0)])),
                (1, HashMap::from([('b', 0), ('a', 5)])),
                (2, HashMap::from([('b', 2), ('d', 0)])),
                (4, HashMap::from([('b', 0)])),
                (6, HashMap::from([('a', 0)])),
            ])
        );
        assert_eq!(transducer.iota, 3);
        assert_eq!(transducer.psi, HashMap::from([(5, 0)]));
        assert_eq!(transducer.min_except, Vec::new());
        assert_eq!(
            transducer.trans_order_partitions,
            Vec::from([
                BTreeSet::from([5]),
                BTreeSet::from([0, 4, 6]),
                BTreeSet::from([1, 2]),
            ])
        );
    }

    #[test]
    fn construct_transducer_from_entry_and_add_word_in_order() {
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
    fn constructs_the_transducer_from_a_dictionary() {
        let dictionary = vec![("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)];
        let transducer = Transducer::from_dictionary(dictionary);
        let expected_transducer = example_transducer();

        assert_eq!(transducer.alphabet, expected_transducer.alphabet);
        assert_eq!(transducer.states, expected_transducer.states);
        assert_eq!(transducer.finality, expected_transducer.finality);
        assert_eq!(transducer.init_state, expected_transducer.init_state);
        assert_eq!(transducer.delta, expected_transducer.delta);
        assert_eq!(transducer.delta_inv, expected_transducer.delta_inv);
        assert_eq!(transducer.lambda, expected_transducer.lambda);
        assert_eq!(transducer.iota, expected_transducer.iota);
        assert_eq!(transducer.psi, expected_transducer.psi);
        assert_eq!(transducer.min_except, expected_transducer.min_except);
        assert_eq!(
            transducer.trans_order_partitions,
            expected_transducer.trans_order_partitions
        );
    }

    #[test]
    fn constructs_the_transducer_from_a_dictionary2() {
        let dictionary = vec![("cab", 15), ("cabab", 10), ("cabad", 8), ("cabc", 12)];
        let transducer = Transducer::from_dictionary(dictionary);

        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('a', 2)])),
            (2, HashMap::from([('b', 3)])),
            (3, HashMap::from([('a', 4), ('c', 5)])),
            (4, HashMap::from([('b', 5), ('d', 5)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3)])),
            (5, HashSet::from([('b', 4), ('d', 4), ('c', 3)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('a', 0)])),
            (2, HashMap::from([('b', 0)])),
            (3, HashMap::from([('a', 0), ('c', 4)])),
            (4, HashMap::from([('b', 2), ('d', 0)])),
        ]);
        let iota = 8;
        let psi = HashMap::from([(3, 7), (5, 0)]);
        let min_except = Vec::new();
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 1, 2]),
            BTreeSet::from([3, 4]),
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
    fn removes_a_long_word() {
        let dictionary = vec![("cab", 15), ("cabab", 10), ("cabad", 8), ("cabc", 12)];
        let mut transducer = Transducer::from_dictionary(dictionary);
        transducer.remove_entry_with_word("cabad");

        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('a', 2)])),
            (2, HashMap::from([('b', 3)])),
            (3, HashMap::from([('a', 4), ('c', 5)])),
            (4, HashMap::from([('b', 5)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3)])),
            (5, HashSet::from([('b', 4), ('c', 3)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('a', 0)])),
            (2, HashMap::from([('b', 0)])),
            (3, HashMap::from([('a', 0), ('c', 2)])),
            (4, HashMap::from([('b', 0)])),
        ]);
        let iota = 10;
        let psi = HashMap::from([(3, 5), (5, 0)]);
        let min_except = Vec::new();
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 1, 2, 4]),
            BTreeSet::from([3]),
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
    fn canonicalises_min_except_path() {
        let example = example_transducer3();
        let mut transducer = example_transducer3();
        transducer.canonicalise_min_except();

        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('a', 0)])),
            (2, HashMap::from([('b', 0)])),
            (3, HashMap::from([('a', 0), ('c', 2)])),
            (4, HashMap::from([('b', 0)])),
        ]);
        let iota = 10;
        let psi = HashMap::from([(3, 5), (5, 0)]);

        assert_eq!(transducer.alphabet, example.alphabet);
        assert_eq!(transducer.states, example.states);
        assert_eq!(transducer.finality, example.finality);
        assert_eq!(transducer.init_state, example.init_state);
        assert_eq!(transducer.delta, example.delta);
        assert_eq!(transducer.delta_inv, example.delta_inv);
        assert_eq!(transducer.lambda, lambda);
        assert_eq!(transducer.iota, iota);
        assert_eq!(transducer.psi, psi);
        assert_eq!(transducer.min_except, example.min_except);
        assert_eq!(
            transducer.trans_order_partitions,
            example.trans_order_partitions
        );
    }

    #[test]
    fn adds_entry_out_of_order2() {
        let dictionary = vec![("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)];
        let mut transducer = Transducer::from_dictionary(dictionary);
        transducer.add_entry_out_of_order("cabada", 6);

        assert_eq!(transducer.alphabet, HashSet::from(['a', 'b', 'c', 'd']));
        assert_eq!(
            transducer.states,
            BTreeSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
        assert_eq!(transducer.finality, BTreeSet::from([3, 5]));
        assert_eq!(transducer.init_state, 0);
        assert_eq!(
            transducer.delta,
            HashMap::from([
                (0, HashMap::from([('c', 1)])),
                (1, HashMap::from([('b', 6), ('a', 2)])),
                (2, HashMap::from([('b', 3), ('d', 5)])),
                (3, HashMap::from([('a', 7)])),
                (4, HashMap::from([('b', 5)])),
                (6, HashMap::from([('a', 4)])),
                (7, HashMap::from([('b', 5), ('d', 8)])),
                (8, HashMap::from([('a', 5)])),
            ])
        );
        assert_eq!(
            transducer.delta_inv,
            HashMap::from([
                (1, HashSet::from([('c', 0)])),
                (2, HashSet::from([('a', 1)])),
                (3, HashSet::from([('b', 2)])),
                (4, HashSet::from([('a', 6)])),
                (5, HashSet::from([('b', 4), ('d', 2), ('b', 7), ('a', 8)])),
                (6, HashSet::from([('b', 1)])),
                (7, HashSet::from([('a', 3)])),
                (8, HashSet::from([('d', 7)])),
            ])
        );
        assert_eq!(
            transducer.lambda,
            HashMap::from([
                (0, HashMap::from([('c', 0)])),
                (1, HashMap::from([('b', 0), ('a', 3)])),
                (2, HashMap::from([('b', 0), ('d', 2)])),
                (3, HashMap::from([('a', 0)])),
                (4, HashMap::from([('b', 0)])),
                (6, HashMap::from([('a', 0)])),
                (7, HashMap::from([('b', 4), ('d', 0)])),
                (8, HashMap::from([('a', 0)])),
            ])
        );
        assert_eq!(transducer.iota, 3);
        assert_eq!(transducer.psi, HashMap::from([(3, 9), (5, 0)]));
        assert_eq!(transducer.min_except, Vec::new());
        assert_eq!(
            transducer.trans_order_partitions,
            Vec::from([
                BTreeSet::from([5]),
                BTreeSet::from([0, 3, 4, 6, 8]),
                BTreeSet::from([1, 2, 7]),
            ])
        );
    }

    #[test]
    fn deletes_a_state() {
        let mut transducer = example_transducer();
        transducer.delete_state(&3);

        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 4, 5, 6]);
        let finality = BTreeSet::from([5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('b', 6), ('a', 2)])),
            (2, HashMap::from([('d', 5)])),
            (4, HashMap::from([('b', 5)])),
            (6, HashMap::from([('a', 4)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (4, HashSet::from([('a', 6)])),
            (5, HashSet::from([('b', 4), ('d', 2)])),
            (6, HashSet::from([('b', 1)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('b', 0), ('a', 5)])),
            (2, HashMap::from([('d', 0)])),
            (4, HashMap::from([('b', 0)])),
            (6, HashMap::from([('a', 0)])),
        ]);
        let iota = 3;
        let psi = HashMap::from([(5, 0)]);
        let min_except = Vec::new();
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 2, 4, 6]),
            BTreeSet::from([1]),
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
    fn adds_entry_in_order() {
        let mut transducer = Transducer::from_entry("cab", 15);
        transducer.add_entry_in_order("cabab", 10);

        let alphabet = HashSet::from(['a', 'b', 'c']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('a', 2)])),
            (2, HashMap::from([('b', 3)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3)])),
            (5, HashSet::from([('b', 4)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('a', 0)])),
            (2, HashMap::from([('b', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
        ]);
        let iota = 10;
        let psi = HashMap::from([(3, 5), (5, 0)]);
        let min_except = vec!['c', 'a', 'b', 'a', 'b'];
        let trans_order_partitions =
            Vec::from([BTreeSet::from([5]), BTreeSet::from([0, 1, 2, 3, 4])]);

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
    fn increases_min_except_from_epsilon_to_word() {
        let mut transducer = Transducer::from_entry("cabab", 10);
        transducer.reduce_to_epsilon();
        transducer.increase_except_from_epsilon_to_word(&vec!['c', 'a', 'b']);

        let alphabet = HashSet::from(['a', 'b', 'c']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5]);
        let finality = BTreeSet::from([5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('a', 2)])),
            (2, HashMap::from([('b', 3)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3)])),
            (5, HashSet::from([('b', 4)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('a', 0)])),
            (2, HashMap::from([('b', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
        ]);
        let iota = 10;
        let psi = HashMap::from([(5, 0)]);
        let min_except = vec!['c', 'a', 'b'];
        let trans_order_partitions =
            Vec::from([BTreeSet::from([5]), BTreeSet::from([0, 1, 2, 3, 4])]);

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
    fn adds_entry_out_of_order() {
        let mut transducer = Transducer::from_entry("cabab", 10);
        transducer.reduce_to_epsilon();
        transducer.add_entry_out_of_order("cab", 15);

        let alphabet = HashSet::from(['a', 'b', 'c']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('a', 2)])),
            (2, HashMap::from([('b', 3)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3)])),
            (5, HashSet::from([('b', 4)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('a', 0)])),
            (2, HashMap::from([('b', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
        ]);
        let iota = 10;
        let psi = HashMap::from([(3, 5), (5, 0)]);
        let min_except = Vec::new();
        let trans_order_partitions =
            Vec::from([BTreeSet::from([5]), BTreeSet::from([0, 1, 2, 3, 4])]);

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
    fn finds_state_sequence_for_a_word() {
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
    fn state_sequence_fails() {
        let transducer = example_transducer();
        transducer.state_sequence(&vec!['c', 'a', 'c']);
    }

    #[test]
    fn reduces_min_except_by_one_char() {
        let mut transducer = example_transducer2();
        transducer.reduce_except_by_one();

        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5, 6, 7]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('b', 6), ('a', 2)])),
            (2, HashMap::from([('b', 3), ('d', 5)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
            (6, HashMap::from([('a', 7)])),
            (7, HashMap::from([('b', 5)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3)])),
            (5, HashSet::from([('b', 4), ('d', 2), ('b', 7)])),
            (6, HashSet::from([('b', 1)])),
            (7, HashSet::from([('a', 6)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('b', 0), ('a', 5)])),
            (2, HashMap::from([('b', 2), ('d', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
            (6, HashMap::from([('a', 0)])),
            (7, HashMap::from([('b', 0)])),
        ]);
        let iota = 3;
        let psi = HashMap::from([(3, 5), (5, 0)]);
        let min_except = vec!['c', 'b', 'a'];
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 3, 4, 6, 7]),
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
    fn reduces_min_except_to_epsilon() {
        let mut transducer = example_transducer2();
        transducer.reduce_to_epsilon();

        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5, 6]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('b', 6), ('a', 2)])),
            (2, HashMap::from([('b', 3), ('d', 5)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
            (6, HashMap::from([('a', 4)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3), ('a', 6)])),
            (5, HashSet::from([('b', 4), ('d', 2)])),
            (6, HashSet::from([('b', 1)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('b', 0), ('a', 5)])),
            (2, HashMap::from([('b', 2), ('d', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
            (6, HashMap::from([('a', 0)])),
        ]);
        let iota = 3;
        let psi = HashMap::from([(3, 5), (5, 0)]);
        let min_except = vec![];
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 3, 4, 6]),
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
    fn adds_delta_and_lambda_transitions() {
        let mut transducer = Transducer::from_entry("cab", 15);

        transducer.add_delta_transition(3, 'a', 4);
        add_to_or_insert(&mut transducer.lambda, 3, 'a', 123);

        assert_eq!(transducer.delta[&3][&'a'], 4);
        assert_eq!(transducer.lambda[&3][&'a'], 123);

        transducer.add_delta_transition(3, 'b', 5);
        add_to_or_insert(&mut transducer.lambda, 3, 'b', 321);

        assert_eq!(transducer.delta[&3][&'b'], 5);
        assert_eq!(transducer.lambda[&3][&'b'], 321);
    }

    #[test]
    fn finds_longest_common_prefix() {
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
    fn calculates_word_output() {
        let transducer = example_transducer();
        assert_eq!(transducer.output(&vec!['c', 'a', 'b']), 15);
        assert_eq!(transducer.output(&vec!['c', 'a', 'b', 'a', 'b']), 10);
        assert_eq!(transducer.output(&vec!['c', 'a', 'd']), 8);
    }

    #[test]
    fn finds_previous_divergent_state() {
        let transducer = example_transducer2();
        assert_eq!(transducer.find_prev_divergent_state(&8), Some(('b', 1)));
        assert_eq!(transducer.find_prev_divergent_state(&1), None);
        assert_eq!(transducer.find_prev_divergent_state(&3), Some(('b', 2)));
    }

    // Helper functions
    ///////////////////
    fn example_transducer() -> Transducer {
        // dictionary := [("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)]
        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5, 6]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('b', 6), ('a', 2)])),
            (2, HashMap::from([('b', 3), ('d', 5)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
            (6, HashMap::from([('a', 4)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3), ('a', 6)])),
            (5, HashSet::from([('b', 4), ('d', 2)])),
            (6, HashSet::from([('b', 1)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('b', 0), ('a', 5)])),
            (2, HashMap::from([('b', 2), ('d', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
            (6, HashMap::from([('a', 0)])),
        ]);
        let iota = 3;
        let psi = HashMap::from([(3, 5), (5, 0)]);
        let min_except = Vec::new();
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 3, 4, 6]),
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

    fn example_transducer2() -> Transducer {
        // dictionary := [("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)]
        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let finality = BTreeSet::from([3, 5, 8]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('b', 6), ('a', 2)])),
            (2, HashMap::from([('b', 3), ('d', 5)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
            (6, HashMap::from([('a', 7)])),
            (7, HashMap::from([('b', 8)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3)])),
            (5, HashSet::from([('b', 4), ('d', 2)])),
            (6, HashSet::from([('b', 1)])),
            (7, HashSet::from([('a', 6)])),
            (8, HashSet::from([('b', 7)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('b', 0), ('a', 5)])),
            (2, HashMap::from([('b', 2), ('d', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
            (6, HashMap::from([('a', 0)])),
            (7, HashMap::from([('b', 0)])),
        ]);
        let iota = 3;
        let psi = HashMap::from([(3, 5), (5, 0), (8, 0)]);
        let min_except = vec!['c', 'b', 'a', 'b'];
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5, 8]),
            BTreeSet::from([0, 3, 4, 6, 7]),
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

    fn example_transducer3() -> Transducer {
        // dictionary := [("cab", 15), ("cabab", 10), ("cabad", 8), ("cabc", 12)]
        // Non canonical, after removing the prefix for "cabad"
        let alphabet = HashSet::from(['a', 'b', 'c', 'd']);
        let states = BTreeSet::from([0, 1, 2, 3, 4, 5]);
        let finality = BTreeSet::from([3, 5]);
        let init_state = 0;
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('a', 2)])),
            (2, HashMap::from([('b', 3)])),
            (3, HashMap::from([('a', 4), ('c', 5)])),
            (4, HashMap::from([('b', 5)])),
        ]);
        let delta_inv = HashMap::from([
            (1, HashSet::from([('c', 0)])),
            (2, HashSet::from([('a', 1)])),
            (3, HashSet::from([('b', 2)])),
            (4, HashSet::from([('a', 3)])),
            (5, HashSet::from([('b', 4), ('c', 3)])),
        ]);
        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('a', 0)])),
            (2, HashMap::from([('b', 0)])),
            (3, HashMap::from([('c', 4), ('a', 0)])),
            (4, HashMap::from([('b', 2)])),
        ]);
        let iota = 8;
        let psi = HashMap::from([(3, 7), (5, 0)]);
        let min_except = vec!['c', 'a', 'b', 'a'];
        let trans_order_partitions = Vec::from([
            BTreeSet::from([5]),
            BTreeSet::from([0, 1, 2, 4]),
            BTreeSet::from([3]),
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
