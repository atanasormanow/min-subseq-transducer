use serde::Deserialize;
use std::{error::Error, vec};

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
            transducer.print();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
