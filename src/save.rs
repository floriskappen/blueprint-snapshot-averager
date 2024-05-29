use std::{fs::File, io::Write};

use crate::{load::{BlueprintTuples, BlueprintTuplesData, MAX_AVAILABLE_ACTIONS}, BlueprintPublic};

pub fn save_blueprint_file(blueprint: BlueprintPublic, filepath: String) {
    // Serialize and save each chunk
    let tuples: BlueprintTuplesData = blueprint.map
        .into_iter()
        .map(|(key, value)| {
            let mut value_array = [0; MAX_AVAILABLE_ACTIONS];
            for (i, num) in value.into_iter().enumerate() {
                value_array[i] = num
            };
            return (key, value_array);
        })
        .collect();

    let blueprint_serde = BlueprintTuples { data: tuples };
    let encoded: Vec<u8> = bincode::serialize(&blueprint_serde).unwrap();
    let mut file = File::create(filepath).expect("Error creating output file");
    file.write_all(&encoded).expect("Error writing to output file");
}