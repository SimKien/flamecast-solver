use std::fs::DirEntry;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solver::SimulatedAnnealingLogger;

use crate::solver_testing::ALPHA_FILE_NAME;

use super::ALPHA_SOLUTIONS_DIR;

const ALPHA_CHARTS_DIR: &str = "./ba/charts/alpha";

pub fn evaluate_alpha(dir: &String) {
    let alpha_file = std::fs::read_to_string(ALPHA_FILE_NAME);
    if alpha_file.is_err() {
        println!("Failed to read alpha file: {:?}", ALPHA_FILE_NAME);
        return;
    }
    let alpha_file = alpha_file.unwrap();
    let alpha_value = serde_json::from_str::<Vec<f64>>(&alpha_file);
    if alpha_value.is_err() {
        println!("Failed to parse alpha JSON: {:?}", ALPHA_FILE_NAME);
        return;
    }
    let alphas = alpha_value.unwrap();

    for alpha in alphas.iter() {
        let alpha_string = alpha.to_string().replace('.', "_");
        let solution_dir = format!("{}/{}/{}", ALPHA_CHARTS_DIR, alpha_string, dir);
        if !std::path::Path::new(&solution_dir).exists() {
            std::fs::create_dir_all(&solution_dir).expect("Failed to create directory");
        } else {
            for entry in std::fs::read_dir(&solution_dir).expect("Failed to read directory") {
                let entry = entry.expect("Failed to read entry");
                if entry.path().is_file() {
                    std::fs::remove_file(entry.path()).expect("Failed to remove file");
                }
            }
        }
    }

    for alpha in alphas.iter() {
        let alpha_string = alpha.to_string().replace('.', "_");
        let solution_dir = format!("{}/{}/{}", ALPHA_SOLUTIONS_DIR, alpha_string, dir);
        if !std::path::Path::new(&solution_dir).exists() {
            println!("Directory does not exist: {:?}", solution_dir);
            continue;
        }

        let entries = std::fs::read_dir(&solution_dir)
            .expect("Failed to read directory")
            .map(|entry| entry.expect("Failed to read entry"))
            .collect::<Vec<DirEntry>>();
        entries.par_iter().for_each(|entry| {
            let file_path = entry.path();
            if file_path.is_file() && file_path.extension().unwrap() == "json" {
                let logs = serde_json::from_str::<SimulatedAnnealingLogger>(
                    &std::fs::read_to_string(file_path.clone()).unwrap(),
                );
                if logs.is_err() {
                    println!("Failed to parse JSON: {:?}", file_path);
                    return;
                }
                let logs = logs.unwrap();
                //TODO: adde anzahl knoten die gepickt werden zu den optionen
            }
        });
    }
}
