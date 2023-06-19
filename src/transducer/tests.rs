#[cfg(test)]
mod tests {
    use crate::transducer::{Entry, Transducer};

    #[test]
    fn transducer_from_word() {
        let entry = Entry::new("cab", 15);
        let transducer = Transducer::from_entry(entry);
        println!("-> transducer for {{ cab -> 10 }}");
        transducer.print();
    }

    #[test]
    fn state_sequence_works() {
        let entry = Entry::new("cab", 15);
        let transducer = Transducer::from_entry(entry);

        assert_eq!(
            transducer.state_sequence(&vec!['c', 'a', 'b']),
            vec![0, 1, 2, 3]
        );
        assert_eq!(transducer.state_sequence(&vec!['c', 'a']), vec![0, 1, 2]);
    }

    #[test]
    fn add_delta_and_lambda_transitions() {
        let entry = Entry::new("cab", 15);
        let mut transducer = Transducer::from_entry(entry);

        transducer.add_delta_transition(3, 'a', 4);
        transducer.add_lambda_transition(3, 'a', 123);

        assert_eq!(transducer.delta[&3][&'a'], 4);
        assert_eq!(transducer.lambda[&3][&'a'], 123);

        transducer.add_delta_transition(3, 'b', 5);
        transducer.add_lambda_transition(3, 'b', 321);

        assert_eq!(transducer.delta[&3][&'b'], 5);
        assert_eq!(transducer.lambda[&3][&'b'], 321);
    }
}
