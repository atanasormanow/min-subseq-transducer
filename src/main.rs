use serde::Deserialize;
use std::error::Error;

mod transducer;
use transducer::Transducer;

#[derive(Debug, Deserialize)]
struct Record {
    word: String,
    output: usize,
}

fn read_csv(name: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut results: Vec<Record> = Vec::new();
    let reader = csv::Reader::from_path(name);

    for record in reader?.deserialize() {
        let record: Record = record?;
        results.push(record);
    }

    Ok(results)
}

fn main() {
    let csv_result = read_csv("/home/nakk/Workspace/uni/min-subseq-transducer/resources/test.csv");

    match csv_result {
        Ok(records) => {
            let dictionary: Vec<(&str, usize)> = records
                .iter()
                .map(|r| (r.word.as_str(), r.output))
                .collect();

            let transducer = Transducer::from_dictionary(dictionary);
            // transducer.print();
            println!("Number of states: {:?}", transducer.states.len());
            println!("Number of transitions: {:?}", transducer.get_number_of_transitions());
            println!("Initial output: {:?}", transducer.iota);
            println!("Number of final states: {:?}", transducer.finality.len());
            // TODO: should be able to ask for removal or more additions then read from another
            // file
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
