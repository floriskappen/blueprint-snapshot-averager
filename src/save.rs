use std::{fs::File, io::Write};

use crate::BlueprintPublic;

pub fn save_blueprint_file(blueprint: &BlueprintPublic, filepath: &str, ) {
    let mut file = File::create(filepath).expect("Error creating output file");
    let encoded: Vec<u8> = bincode::serialize(blueprint).unwrap();
    file.write_all(&encoded);
}
