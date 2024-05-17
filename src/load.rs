use std::fs::File;
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlueprintTuples {
    pub data: Vec<(Vec<u8>, Vec<u16>)>,
}

pub fn load_blueprint_file(filepath: &str) -> Vec<(Vec<u8>, Vec<u16>)> {
    let file = File::open(filepath).expect("error opening file");
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    log::info!("Opened & memmapped file");
    let decoded: BlueprintTuples = bincode::deserialize(&mmap[..]).unwrap();
    log::info!("Decoded file");

    return decoded.data
}
