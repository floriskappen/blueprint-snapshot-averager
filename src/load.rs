use std::{collections::HashMap, fs::File};
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Blueprint {
    map: HashMap<Vec<u8>, Vec<u16>>
}

pub fn load_blueprint_file(filepath: &str) -> HashMap<Vec<u8>, Vec<u16>> {
    let file = File::open(filepath).expect("error opening file");
    log::info!("Opened file");
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    log::info!("Memmapped file");
    let decoded: Blueprint = bincode::deserialize(&mmap[..]).unwrap();
    log::info!("Decoded file");

    return decoded.map
}
