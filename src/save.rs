use std::{fs::File, io::Write};

use crate::load::BlueprintTuples;

pub fn save_blueprint_file(blueprint: BlueprintTuples, filepath: String) {
    let blueprint_serde = blueprint;
    let encoded: Vec<u8> = bincode::serialize(&blueprint_serde).unwrap();
    let mut file = File::create(filepath).expect("Error creating output file");
    file.write_all(&encoded).expect("Error writing to output file");
}