use std::{collections::HashMap, fs::File, io::{BufWriter, Write}};

use prost::Message;

use crate::proto::{Blueprint, KeyValuePair};

pub fn save_blueprint_file(information_sets: &HashMap<Vec<u8>, Vec<u16>>, filepath: &str) {
    let mut pairs = Vec::new();
    for (key, value) in information_sets {
        let pair = KeyValuePair {
            key: key.clone(),
            value: value.iter().map(|&x| x as u32).collect(),
        };
        pairs.push(pair);
    }

    let hash_map_vec = Blueprint { pairs };
    let mut file = BufWriter::new(File::create(filepath).expect("Error creating file"));
    let bytes = hash_map_vec.encode_to_vec();
    file.write_all(&bytes).expect("Error writing to file");
}
