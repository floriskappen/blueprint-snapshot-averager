pub mod load;
pub mod save;
pub mod logger;

use std::{collections::HashMap, fs, path::Path};
use rayon::prelude::*;

use load::load_blueprint_file;
use logger::init_logger;
use serde::Deserialize;
use serde::Serialize;

use crate::save::save_blueprint_file;

#[derive(Serialize, Deserialize)]
pub struct BlueprintPublic {
    pub map: HashMap<Vec<u8>, Vec<u16>>
}

fn main() {
    init_logger().expect("Failed to initialize logger");
    let imports_directory = Path::new("./imports");
    let blueprint_snapshot_filepaths: Vec<String> = fs::read_dir(imports_directory)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().and_then(std::ffi::OsStr::to_str) == Some("bin") {
                    Some(path.to_str().unwrap().to_string())
                } else {
                    None
                }
            })
        })
        .collect();

    let blueprint_snapshot_count = blueprint_snapshot_filepaths.len() as u64;

    let mut accumulator: HashMap<Vec<u8>, Vec<u32>> = HashMap::new();
    for filepath in blueprint_snapshot_filepaths {
        log::info!("Loading {}", &filepath);
        let blueprint: HashMap<Vec<u8>, Vec<u16>> = load_blueprint_file(&filepath);
        log::info!("Loaded {}", &filepath);
        for (key, value) in blueprint.into_iter() {
            accumulator.entry(key).or_insert_with(|| vec![0; value.len()])
                .iter_mut()
                .zip(value.iter())
                .for_each(|(acc, &val)| *acc += val as u32);
        }
        log::info!("Added {} to accumulator", &filepath);
    }

    log::info!("Loaded {} blueprint snapshots", blueprint_snapshot_count);

    // Increase parallelism for normalization
    let averaged_blueprint = BlueprintPublic {
        map: accumulator.into_par_iter().map(|(key, sums)| {
            let total = sums.iter().sum::<u32>();
            let averages = sums.iter()
                .map(|&sum| {
                    ((sum * 10_000 / total)) as u16
                })
                .collect();
            (key, averages)
        }).collect()
    };

    log::info!("Computed average blueprint, it has {} keys", averaged_blueprint.map.len());

    save_blueprint_file(&averaged_blueprint, "./exports/averaged_blueprint.bin");

    log::info!("Saved average blueprint file")
}
