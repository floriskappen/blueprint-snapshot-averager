use std::{collections::HashMap, fs::File, io::{BufReader, Read}};

use prost::Message;

use crate::proto::Blueprint as ProtoBlueprint;

pub fn load_blueprint_file(filepath: &str) -> HashMap<Vec<u8>, Vec<u16>> {
    log::info!("Loading {}...", filepath);
    let mut buf_reader = BufReader::new(File::open(filepath).expect("Error opening labels file"));
    let mut buf = Vec::new();
    buf_reader.read_to_end(&mut buf).expect("Error reading buffer from labels file");
    let blueprint_proto = ProtoBlueprint::decode(&*buf).expect("Error decoding flop ClusteredDataLabels");
    let mut blueprint = HashMap::new();
    for pair in blueprint_proto.pairs {
        let value = pair.value.into_iter().map(|x| x as u16).collect();
        blueprint.insert(pair.key, value);
    }
    return blueprint
}
