pub mod load;
pub mod save;
pub mod logger;

use std::{collections::HashMap, fs, path::Path};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use load::load_blueprint_file;
use logger::init_logger;

use crate::load::Data;
use crate::save::save_blueprint_file;

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

    // Limit parallelism to 5 threads for file loading and accumulation
    let limited_pool = ThreadPoolBuilder::new().num_threads(3).build().unwrap();
    let accumulator: HashMap<Vec<u8>, Vec<u32>> = limited_pool.install(|| {
        blueprint_snapshot_filepaths
            .par_iter()
            .map(|filepath| {
                let blueprint: HashMap<Vec<u8>, Vec<u16>> = load_blueprint_file(filepath);
                let blueprint_converted: HashMap<Vec<u8>, Vec<u32>> = blueprint.into_iter().map(|(key, value)| (key, value.into_iter().map(|v| v as u32).collect())).collect();
                return blueprint_converted;
            })
            .reduce(
                || HashMap::new(),
                |mut acc, blueprint| {
                    for (key, values) in blueprint {
                        acc.entry(key).or_insert_with(|| vec![0; values.len()])
                            .iter_mut()
                            .zip(values.iter())
                            .for_each(|(acc, &val)| *acc += val);
                    }
                    acc
                },
            )
    });

    log::info!("Loaded {} blueprint snapshots", blueprint_snapshot_count);

    // Increase parallelism for normalization
    let max_pool = ThreadPoolBuilder::new().num_threads(num_cpus::get()).build().unwrap();
    let averaged_blueprint = Data {
        map: max_pool.install(|| {
            accumulator.into_par_iter().map(|(key, sums)| {
                let total = sums.iter().sum::<u32>();
                let averages = sums.iter()
                    .map(|&sum| {
                        ((sum * 10_000 / total)) as u16
                    })
                    .collect();
                (key, averages)
            }).collect()
        })
    };

    log::info!("Computed average blueprint, it has {} keys", averaged_blueprint.map.len());

    save_blueprint_file(&averaged_blueprint, "./exports/averaged_blueprint.bin");

    log::info!("Saved average blueprint file")
}
