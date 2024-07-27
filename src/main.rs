pub mod load;
pub mod save;
pub mod logger;

use std::{collections::HashMap, fs, path::Path};
use load::BlueprintTuples;
use load::MAX_AVAILABLE_ACTIONS;
use rayon::prelude::*;

use load::load_blueprint_file;
use logger::init_logger;
use regex::Regex;

use crate::save::save_blueprint_file;

pub const ROUND: usize = 1;

fn main() {
    init_logger().expect("Failed to initialize logger");
    let imports_directory = Path::new("./imports");
    let mut blueprint_snapshot_folders: Vec<String> = fs::read_dir(imports_directory)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_dir() {
                    Some(path.to_str().unwrap().to_string())
                } else {
                    None
                }
            })
        })
        .collect();
    println!("blueprint_snapshot_folders: {:?}", blueprint_snapshot_folders);

    // Sort the snapshot folders numerically, this is necessary because we might want to weigh certain snapshots more than others
    let re = Regex::new(r"snapshot_(\d+)_").unwrap();
    blueprint_snapshot_folders.sort_by(|a, b| {
        let a_caps = re.captures(a).unwrap();
        let b_caps = re.captures(b).unwrap();

        let a_num: u32 = a_caps[1].parse().unwrap();
        let b_num: u32 = b_caps[1].parse().unwrap();

        a_num.cmp(&b_num)
    });

    log::info!("Loaded snapshot folders: {:?}", blueprint_snapshot_folders);

    let mut files_per_snapshot_folder: Vec<Vec<Option<String>>> = Vec::with_capacity(blueprint_snapshot_folders.len());
    for folder in blueprint_snapshot_folders {
        let mut snapshot_folder_files: Vec<String> = fs::read_dir(folder)
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

        snapshot_folder_files.sort_by(|a, b| {
            let a_caps = re.captures(a).unwrap();
            let b_caps = re.captures(b).unwrap();
    
            let a_num: u32 = a_caps[1].parse().unwrap();
            let b_num: u32 = b_caps[1].parse().unwrap();
    
            a_num.cmp(&b_num)
        });

        let mut i = 0;
        let mut j = 0;
        let mut all_files = Vec::new();
        while i < snapshot_folder_files.len() {
            if files_per_snapshot_folder.len() > 0 {
                let current_file_name = Path::new(&snapshot_folder_files[i]).file_name().unwrap().to_str().unwrap();

                if files_per_snapshot_folder[0].len() <= j {
                    all_files.push(None);
                } else {
                    let filename = files_per_snapshot_folder[0][j].clone().unwrap();
                    let ground_truth_file_name = Path::new(
                        &filename
                    )
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
    
                    if current_file_name == ground_truth_file_name {
                        all_files.push(Some(snapshot_folder_files[i].clone()))
                    } else {
                        all_files.push(None);
                        i -= 1;
                    }
                }
            } else {
                all_files.push(Some(snapshot_folder_files[i].clone()));
            }

            i += 1;
            j += 1;
        }
        files_per_snapshot_folder.push(all_files);
    }


    let max_len = files_per_snapshot_folder.iter().map(|v| v.len()).max().unwrap_or(0);
    let mut snapshots_per_hand:Vec<Vec<Option<String>>> = vec![Vec::new(); max_len];
    for files in files_per_snapshot_folder.iter() {
        for (i, value) in files.into_iter().enumerate() {
            snapshots_per_hand[i].push(value.clone())
        }
    }

    println!("snapshots_per_hand: {:?}", snapshots_per_hand);

    for (i, hand_snapshots) in snapshots_per_hand.into_iter().enumerate() {
        let mut accumulator: HashMap<Vec<u8>, Vec<u16>> = HashMap::new();
        let first_filepath = hand_snapshots[0].clone().unwrap();
        let hand_filename = Path::new(&first_filepath).file_name().unwrap().to_str().unwrap();
        for filepath_option in hand_snapshots.clone() {
            if let Some(filepath) = filepath_option {
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
                }
            }
        }

        let averaged_blueprint = BlueprintTuples {
            data: accumulator.into_par_iter().map(|(key, sums)| {
                let total = sums.iter().sum::<u16>();
                let mut value_array = [0; MAX_AVAILABLE_ACTIONS];

                let averages = sums.iter()
                    .map(|&sum| {
                        if sum == 0 {
                            return 0
                        }
                        return ((sum * 100 / total)) as u8
                    })
                    .collect::<Vec<u8>>();

                for (i, num) in averages.into_iter().enumerate() {
                    value_array[i] = num
                };
                (key, value_array)
            }).collect()
        };

        log::info!("Computed average blueprint, it has {} keys", averaged_blueprint.data.len());

        save_blueprint_file(averaged_blueprint, format!("./exports/{}", hand_filename));
        log::info!("Saved average blueprint file");
    }
}
