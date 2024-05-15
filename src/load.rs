use std::{collections::HashMap, fs::File, io::Read};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub map: HashMap<Vec<u8>, Vec<u16>>,
}

pub fn load_blueprint_file(filepath: &str) -> HashMap<Vec<u8>, Vec<u16>> {

    let mut file = File::open(filepath).expect("error opening file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("error reading ifle v2");
    let decoded: Data = bincode::deserialize(&buffer).unwrap();

    return decoded.map
}
