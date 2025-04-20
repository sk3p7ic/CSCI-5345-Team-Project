use std::{collections::HashMap, io::Write};

use serde::{Deserialize, Serialize};

/// Contains information about a `Paper` authored by a `Professor`.
#[derive(Deserialize, Serialize, Clone)]
pub struct Paper {
    pub id: u32,
    pub title: String,
}

/// Contains information about a `Professor`.
#[derive(Deserialize, Serialize, Clone)]
pub struct Professor {
    pub id: u32,
    pub name: String,
    pub dept: String,
    pub desc: String,
    pub papers: Vec<Paper>,
}

/// Stores `Professor`s, indexed by each of their IDs.
pub struct Dataset<'data>(pub HashMap<u32, Professor>, &'data str);

/// Create a `Drop` implementation to handle saving the current data (list of
/// professors) to the filesystem upon exiting the program.
impl<'data> Drop for Dataset<'data> {
    fn drop(&mut self) {
        let professors = self.0.values().collect::<Vec<_>>();
        let data = match serde_json::to_string_pretty(&professors) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Cannot serialize data: {e}");
                return;
            }
        };
        let mut file = match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.1)
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Cannot open dataset file to write: {e}");
                return;
            }
        };
        if let Err(e) = file.write_all(data.as_bytes()) {
            eprintln!("Cannot write to dataset file: {e}");
            return;
        }
        println!("Saved current data to '{path}'", path = self.1);
    }
}

/// Loads a given JSON dataset found under `path` into a HashMap.
///
/// # Panics
/// Will panic if the dataset cannot be found under the given `path` or the
/// data cannot be parsed to a `Dataset`.
pub fn load_dataset(path: &'static str) -> Dataset<'static> {
    let data = std::fs::read_to_string(path).expect("Dataset file must exist");
    let data_map = serde_json::from_str::<Vec<Professor>>(data.as_str())
        .expect("Input data must be parsable as a list of Professors")
        .into_iter()
        .map(|prof| (prof.id, prof))
        .collect();
    Dataset(data_map, path)
}
