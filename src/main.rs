mod proto {
    include!("proto/build/_.rs");
}
pub mod load;
pub mod save;
pub mod logger;

use std::{collections::HashMap, fs, path::Path};

use load::load_blueprint_file;
use logger::init_logger;

use crate::save::save_blueprint_file;

fn main() {
    init_logger().expect("Failed to initialize logger");
    let mut accumulator: HashMap<Vec<u8>, Vec<u64>> = HashMap::new();
    let imports_directory = Path::new("./imports");
    let mut blueprint_snapshot_filepaths: Vec<String> = Vec::new();
    for entry in fs::read_dir(imports_directory).unwrap() {
        if let Ok(entry) = entry {
            if entry.path().extension().and_then(std::ffi::OsStr::to_str) == Some("bin") {
                let filepath = entry.path();
                blueprint_snapshot_filepaths.push(filepath.to_str().unwrap().to_string())
            }
        }
    }

    let blueprint_snapshot_count = blueprint_snapshot_filepaths.len() as u64;

    for blueprint_snapshot_filepath in blueprint_snapshot_filepaths {
        let blueprint = load_blueprint_file(&blueprint_snapshot_filepath);
        log::info!("Blueprint has {} keys", blueprint.len());
        for (key, values) in blueprint {
            accumulator.entry(key).or_insert_with(|| vec![0; values.len()])
                .iter_mut()
                .zip(values.iter())
                .for_each(|(acc, &val)| *acc += val as u64);
        }
    }

    log::info!("Loaded {} blueprint snapshots", blueprint_snapshot_count);

    // Compute the average
    let mut averaged_blueprint: HashMap<Vec<u8>, Vec<u16>> = HashMap::new();
    for (key, sums) in accumulator {
        let averages = sums.iter()
            .map(|&sum| (sum / blueprint_snapshot_count) as u16)
            .collect();
        averaged_blueprint.insert(key, averages);
    }

    log::info!("Computed average blueprint, it has {} keys", averaged_blueprint.len());

    save_blueprint_file(&averaged_blueprint, "./exports/averaged_blueprint.bin");

    log::info!("Saved average blueprint file")
}
