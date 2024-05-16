use std::{collections::HashMap, fs::File, io::Write};

use crate::BlueprintPublic;

pub fn save_blueprint_file(blueprint: BlueprintPublic, output_folder: &str) {
    // Get the total number of items in the HashMap
    let total_items = blueprint.map.len();
    
    // Calculate the number of items per chunk
    let items_per_chunk = (total_items + 47) / 48; // Ensure ceiling division
    
    // Split the HashMap into chunks
    let mut chunks: Vec<HashMap<Vec<u8>, Vec<u16>>> = Vec::with_capacity(48);
    let mut current_chunk = HashMap::with_capacity(items_per_chunk);
    let mut count = 0;

    for (key, value) in blueprint.map.into_iter() {
        current_chunk.insert(key, value);
        count += 1;
        if count == items_per_chunk {
            chunks.push(current_chunk);
            current_chunk = HashMap::with_capacity(items_per_chunk);
            count = 0;
        }
    }

    // Add the last chunk if it contains any items
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    // Ensure we have exactly 48 chunks, even if some are empty
    while chunks.len() < 48 {
        chunks.push(HashMap::with_capacity(items_per_chunk));
    }

    // Serialize and save each chunk
    for (i, chunk) in chunks.into_iter().enumerate() {
        let chunk_blueprint = BlueprintPublic { map: chunk };
        let encoded: Vec<u8> = bincode::serialize(&chunk_blueprint).unwrap();
        let mut file = File::create(format!("{}/averaged_blueprint_chunk_{}.bin", output_folder, i)).expect("Error creating output file");
        file.write_all(&encoded).expect("Error writing to output file");
    }
}