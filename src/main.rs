pub mod load;
pub mod save;
pub mod logger;

use std::{collections::HashMap, fs, path::Path};
use rayon::prelude::*;

use load::load_blueprint_file;
use logger::init_logger;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::save::save_blueprint_file;

pub const ROUND: usize = 1;

#[derive(Serialize, Deserialize)]
pub struct BlueprintPublic {
    pub map: HashMap<Vec<u8>, Vec<u8>>
}

fn main() {
    init_logger().expect("Failed to initialize logger");
    let imports_directory = Path::new("./imports");
    let mut blueprint_snapshot_folders: Vec<String> = fs::read_dir(imports_directory)
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

    // Sort the snapshots numerically, this is necessary because we might want to weigh certain snapshots more than others
    let re = Regex::new(r"snapshot_(\d+)_").unwrap();
    blueprint_snapshot_folders.sort_by(|a, b| {
        let a_caps = re.captures(a).unwrap();
        let b_caps = re.captures(b).unwrap();

        let a_num: u32 = a_caps[1].parse().unwrap();
        let b_num: u32 = b_caps[1].parse().unwrap();

        a_num.cmp(&b_num)
    });

    log::info!("Loaded snapshot folders: {:?}", blueprint_snapshot_folders);

    let mut current_hand_index = 0;
    loop {
        log::info!("Averaging hand {}", current_hand_index);
        let mut accumulator: HashMap<Vec<u8>, Vec<u16>> = HashMap::new();
        let mut hand_snapshots = 0;
        for (i, snapshot_folder) in blueprint_snapshot_folders.iter().enumerate() {
            let filepath = format!("{}/round_{}_hand_{}", snapshot_folder, ROUND, current_hand_index);
            let path = Path::new(&filepath);
            if path.exists() {
                let blueprint_tuples = load_blueprint_file(&filepath);
                log::info!("Loaded {}", filepath);
                blueprint_tuples.into_iter().for_each(|(key, value)| {
                    let entry = accumulator.entry(key).or_insert_with(|| vec![0; value.len()]);
                    entry.iter_mut()
                        .zip(value.iter())
                        .for_each(|(acc, &val)| {
                            // Snapshot >10 should weigh 5x as much
                            if i > 10 {
                                *acc += val as u16 * 5
                            }
                        });
                });
                hand_snapshots += 1;
            }
        }

        if hand_snapshots == 0 {
            break;
        }

        let averaged_blueprint = BlueprintPublic {
            map: accumulator.into_par_iter().map(|(key, sums)| {
                let total = sums.iter().sum::<u16>();
                let averages = sums.iter()
                    .map(|&sum| {
                        ((sum * 100 / total)) as u8
                    })
                    .collect();
                (key, averages)
            }).collect()
        };

        log::info!("Computed average blueprint, it has {} keys", averaged_blueprint.map.len());

        save_blueprint_file(averaged_blueprint, "./exports", current_hand_index);
        log::info!("Saved average blueprint file");

        current_hand_index += 1;
    }
}
