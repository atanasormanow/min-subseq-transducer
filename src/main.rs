use serde::Deserialize;
use std::{error::Error, time::Instant};

mod transducer;
use transducer::Transducer;

#[derive(Debug, Deserialize)]
struct Record {
    word: String,
    output: usize,
}

#[derive(Debug, Deserialize)]
struct Word {
    content: String,
}

fn main() {
    let mut transducer =
        transducer_from_csv("/home/nakk/Workspace/uni/min-subseq-transducer/resources/add_all.csv").unwrap();

    println!("--------------------------");

    read_and_delete_entries(
        &mut transducer,
        "/home/nakk/Workspace/uni/min-subseq-transducer/resources/1-1.csv",
    );

    read_and_delete_entries(
        &mut transducer,
        "/home/nakk/Workspace/uni/min-subseq-transducer/resources/1-2.csv",
    );
    // TODO!: Cannot delete the last word for now
}

fn read_and_delete_entries(transducer: &mut Transducer, file_name: &str) {
    let csv_result = read_csv2(file_name);

    match csv_result {
        Ok(records) => {
            let now = Instant::now();

            for w in records {
                transducer.remove_entry_with_word(w.content.as_str());
            }

            println!("Done deleting in {:?}", now.elapsed());

            transducer.print();
        }
        Err(e) => {
            println!("Found an error: {:?}", e);
        }
    }
}

fn read_csv2(file_name: &str) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut results: Vec<Word> = Vec::new();
    let reader = csv::Reader::from_path(file_name);

    for record in reader?.deserialize() {
        let record: Word = record?;
        results.push(record);
    }

    Ok(results)
}

fn read_csv(file_name: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut results: Vec<Record> = Vec::new();
    let reader = csv::Reader::from_path(file_name);

    for record in reader?.deserialize() {
        let record: Record = record?;
        results.push(record);
    }

    Ok(results)
}

fn transducer_from_csv(file_name: &str) -> Option<Transducer> {
    let csv_result = read_csv(file_name);

    match csv_result {
        Ok(records) => {
            let dictionary: Vec<(&str, usize)> = records
                .iter()
                .map(|r| (r.word.as_str(), r.output))
                .collect();

            let now = Instant::now();

            let transducer = Transducer::from_dictionary(dictionary);

            println!("Done building in {:?}", now.elapsed());

            transducer.print();

            return Some(transducer);
        }
        Err(e) => {
            println!("Found an error: {:?}", e);
            return None;
        }
    }
}

fn read_and_add_entries(transducer: &mut Transducer, file_name: &str) {
    let csv_result = read_csv(file_name);

    match csv_result {
        Ok(records) => {
            let now = Instant::now();

            for record in records {
                transducer.add_entry_out_of_order(record.word.as_str(), record.output);
            }

            println!("Done adding in {:?}", now.elapsed());

            transducer.print();
        }
        Err(e) => {
            println!("Found an error: {:?}", e);
        }
    }
}
