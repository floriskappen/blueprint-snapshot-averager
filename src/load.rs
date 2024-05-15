use std::{collections::HashMap, fs::File, io::Read};
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub map: HashMap<Vec<u8>, Vec<u16>>,
}

pub fn load_blueprint_file(filepath: &str) -> HashMap<Vec<u8>, Vec<u16>> {
    let file = File::open(filepath).expect("error opening file");
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let decoded: Data = bincode::deserialize(&mmap[..]).unwrap();

    return decoded.map
}
