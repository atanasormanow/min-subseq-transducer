#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashMap, HashSet};

    use crate::transducer::{utils::longest_common_prefix, Transducer};

    #[test]
    fn transducer_from_word() {
        let transducer = Transducer::from_entry("cab", 15);
        println!("-> transducer for {{ cab -> 15 }}");
        transducer.print();
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
        assert_eq!(transducer.lambda_star(&vec!['c', 'a', 'b', 'a']), 2);
        assert_eq!(transducer.lambda_star(&vec!['c', 'a']), 0);
    }

    #[test]
    fn lambda_i() {
        let transducer = example_transducer();
        assert_eq!(transducer.lambda_i(2, transducer.iota), transducer.iota);
        assert_eq!(transducer.lambda_i(0, transducer.iota), transducer.iota);
    }

    #[test]
    fn find_equivalent_state() {
        let transducer = example_transducer();
        assert_eq!(transducer.state_eq(6, &vec![0, 1, 2, 6]), Some(5));
    }

    #[test]
    fn transducer_output_function() {
        let transducer = example_transducer();
        assert_eq!(transducer.output(vec!['c','a','b']), 15);
        assert_eq!(transducer.output(vec!['c', 'a', 'b','a', 'b']), 10);
        assert_eq!(transducer.output(vec!['c','a','d']), 8);
    }

    // Helper functions
    ///////////////////
    fn example_transducer() -> Transducer {
        let delta = HashMap::from([
            (0, HashMap::from([('c', 1)])),
            (1, HashMap::from([('a', 2)])),
            (2, HashMap::from([('b', 3), ('d', 6)])),
            (3, HashMap::from([('a', 4)])),
            (4, HashMap::from([('b', 5)])),
        ]);

        let lambda = HashMap::from([
            (0, HashMap::from([('c', 0)])),
            (1, HashMap::from([('a', 0)])),
            (2, HashMap::from([('b', 2), ('d', 0)])),
            (3, HashMap::from([('a', 0)])),
            (4, HashMap::from([('b', 0)])),
        ]);

        let trans_order_partitions = vec![
            BTreeSet::from([5, 6]),
            BTreeSet::from([0, 1, 3, 4]),
            BTreeSet::from([2]),
        ];

        return Transducer {
            alphabet: HashSet::from(['a', 'b', 'c', 'd']),
            states: (0..=6).collect(),
            finality: HashSet::from([5, 6]),
            init_state: 0,
            delta,
            lambda,
            iota: 8,
            psi: HashMap::from([(3, 5), (5, 0), (6, 0)]),
            min_except: vec!['c', 'a', 'd'],
            trans_order_partitions,
        };
    }
}
