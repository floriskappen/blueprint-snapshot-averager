use std::{collections::HashMap, fs::File, io::{BufReader, Read}};
use memmap::MmapOptions;

use prost::Message;

use crate::proto::Blueprint as ProtoBlueprint;

pub fn load_blueprint_file(filepath: &str) -> HashMap<Vec<u8>, Vec<u16>> {
    log::info!("Loading {}...", filepath);

    // Use memory-mapped file for potentially faster access
    let file = File::open(filepath).expect("Error opening labels file");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("Error memory-mapping the file") };

    // Decode directly from memory-mapped file
    let blueprint_proto = ProtoBlueprint::decode(&*mmap).expect("Error decoding ProtoBlueprint");

    let mut blueprint = HashMap::new();
    for pair in blueprint_proto.pairs {
        // Map directly into the target Vec
        let value = pair.value.into_iter().map(|x| x as u16).collect();
        blueprint.insert(pair.key, value);
    }

    blueprint
}