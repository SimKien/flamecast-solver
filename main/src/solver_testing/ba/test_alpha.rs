use std::{fs::DirEntry, path::PathBuf};

use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use solver::{FlamecastTestInstance, InitialSolutionFunction, VertexEmbeddings};

use super::{get_alpha_options, run_test, FlamecastBaseInstance, INSTANCES_DIR};

pub const ALPHA_SOLUTIONS_DIR: &str = "./ba/solutions/alpha";
pub const ALPHA_FILE_NAME: &str = "./ba/options/alpha.json";

pub fn process_test_alpha(dir_name: &String) {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let dir_path: PathBuf = format!("{}/{}", INSTANCES_DIR, dir_name).into();

    if !dir_path.exists() {
        println!("Directory does not exist: {:?}", dir_path);
        return;
    }

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
        let solution_dir = format!("{}/{}/{}", ALPHA_SOLUTIONS_DIR, alpha_string, dir_name);
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

    let entries = std::fs::read_dir(&dir_path)
        .expect("Failed to read directory")
        .map(|entry| entry.expect("Failed to read entry"))
        .collect::<Vec<DirEntry>>();
    entries.par_iter().for_each(|entry| {
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "json") {
            let base_instance = FlamecastBaseInstance::from_file(&file_path.display().to_string());
            let instance_name = file_path.file_stem().unwrap().to_str().unwrap();

            alphas.par_iter().for_each(|alpha| {
                let mut sources_drains_embeddings =
                    VertexEmbeddings::new_with_size(base_instance.layers);
                sources_drains_embeddings.embeddings[0] = base_instance.sources.clone();
                sources_drains_embeddings.embeddings[base_instance.layers - 1] =
                    base_instance.drains.clone();
                let instance = FlamecastTestInstance::new(
                    *alpha,
                    base_instance.layers,
                    base_instance.capacities.clone(),
                    sources_drains_embeddings,
                );

                let alpha_string = alpha.to_string().replace('.', "_");
                let solution_dir = format!("{}/{}/{}", ALPHA_SOLUTIONS_DIR, alpha_string, dir_name);

                run_test(
                    instance,
                    &solution_dir,
                    &instance_name.to_string(),
                    InitialSolutionFunction::Random,
                    get_alpha_options,
                );
            });
        }
    });
}
