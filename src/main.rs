use std::vec;

mod transducer;
use transducer::Transducer;

fn main() {
    let dictionary = vec![("cab", 15), ("cabab", 10), ("cad", 8), ("cbab", 3)];
    let transducer = Transducer::from_dictionary(dictionary);
    transducer.print();
}
