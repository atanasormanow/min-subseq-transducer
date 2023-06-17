use std::vec;

mod transducer;
use transducer::{Entry, Transducer};

fn main() {
    let entry = Entry {
        word: vec!['c', 'a', 'd'],
        output: 10,
    };

    let transducer = Transducer::from_word(entry);
    let state_seq = transducer.state_sequence(&vec!['c','a','d']);
    println!("Here is the sequence: {:?}", state_seq);
    transducer.print();

}
