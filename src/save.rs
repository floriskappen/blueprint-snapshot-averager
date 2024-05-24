use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

use crate::{BlueprintPublic, ROUND};

pub const MAX_AVAILABLE_ACTIONS: usize = 8;

type BlueprintTuples = Vec<(Vec<u8>, [u8; MAX_AVAILABLE_ACTIONS])>;

#[derive(Serialize, Deserialize)]
pub struct BlueprintSerde {
    pub data: BlueprintTuples,
}

pub fn save_blueprint_file(blueprint: BlueprintPublic, output_folder: &str, hand_index: usize) {
    // Serialize and save each chunk
    let tuples: BlueprintTuples = blueprint.map
        .into_iter()
        .map(|(key, value)| {
            let mut value_array = [0; MAX_AVAILABLE_ACTIONS];
            for (i, num) in value.into_iter().enumerate() {
                value_array[i] = num
            };
            return (key, value_array);
        })
        .collect();

    let blueprint_serde = BlueprintSerde { data: tuples };
    let encoded: Vec<u8> = bincode::serialize(&blueprint_serde).unwrap();
    let mut file = File::create(format!("{}/averaged_round_{}_hand_{}.bin", output_folder, ROUND, hand_index)).expect("Error creating output file");
    file.write_all(&encoded).expect("Error writing to output file");
}