use std::fs::File;
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};

pub const MAX_AVAILABLE_ACTIONS: usize = 10;

pub type BlueprintTuplesData = Vec<(Vec<u8>, [u8; MAX_AVAILABLE_ACTIONS])>;
#[derive(Serialize, Deserialize)]
pub struct BlueprintTuples {
    pub data: BlueprintTuplesData,
}

pub fn load_blueprint_file(filepath: &str) -> BlueprintTuplesData {
    let file = File::open(filepath).expect("error opening file");
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    log::info!("Opened & memmapped file");
    let decoded: BlueprintTuples = bincode::deserialize(&mmap[..]).unwrap();
    log::info!("Decoded file");

    return decoded.data
}
