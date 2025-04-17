use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Contains information about a `Paper` authored by a `Professor`.
#[derive(Deserialize, Serialize)]
pub struct Paper {
    pub id: u32,
    pub title: String,
}

/// Contains information about a `Professor`.
#[derive(Deserialize, Serialize)]
pub struct Professor {
    pub id: u32,
    pub name: String,
    pub dept: String,
    pub desc: String,
    pub papers: Vec<Paper>,
}

/// Stores `Professor`s, indexed by each of their IDs.
pub type Dataset = HashMap<u32, Professor>;

/// Loads a given JSON dataset found under `path` into a HashMap.
///
/// # Panics
/// Will panic if the dataset cannot be found under the given `path` or the
/// data cannot be parsed to a `Dataset`.
pub fn load_dataset(path: &'static str) -> Dataset {
    let data = std::fs::read_to_string(path).expect("Dataset file must exist");
    serde_json::from_str::<Vec<Professor>>(data.as_str())
        .expect("Input data must be parsable as a list of Professors")
        .into_iter()
        .map(|prof| (prof.id, prof))
        .collect()
}
