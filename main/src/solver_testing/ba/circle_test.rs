use std::{fs::DirEntry, path::PathBuf, sync::Mutex};

use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use serde::{Deserialize, Serialize};
use solver::{FlamecastTestInstance, InitialSolutionFunction, VertexEmbeddings};

use crate::solver_testing::{get_circle_options, get_num_instances, register_job_done, run_test};

const CIRCLE_INSTANCES_DIR: &str = "./circle/instances";
pub const CIRCLE_SOLUTIONS_DIR: &str = "./circle/solutions";

pub const CIRCLE_ALPHA_VALUES: [f64; 10] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleInstance {
    pub sources: Vec<Vec<f64>>,
    pub sinks: Vec<Vec<f64>>,
    pub layers: usize,
    pub capacities: Vec<Option<usize>>,
}

pub fn circle_test() {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let dir_path: PathBuf = CIRCLE_INSTANCES_DIR.into();

    if !dir_path.exists() {
        println!("Directory does not exist: {:?}", dir_path);
        return;
    }

    for alpha in CIRCLE_ALPHA_VALUES.iter() {
        let alpha_string = alpha.to_string().replace('.', "_");
        let solution_dir = format!("{}/{}", CIRCLE_SOLUTIONS_DIR, alpha_string);
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

    let done_jobs = Mutex::new(0);
    let total_jobs = get_num_instances(&entries) * CIRCLE_ALPHA_VALUES.len();
    println!("Total jobs: {}", total_jobs);
    println!("Starting jobs...");

    entries.par_iter().for_each(|entry| {
        let file_path = entry.path();
        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "json") {
            let instance_name = file_path.file_stem().unwrap().to_str().unwrap();
            let instance: CircleInstance =
                serde_json::from_reader(std::fs::File::open(&file_path).unwrap()).unwrap();
            CIRCLE_ALPHA_VALUES.par_iter().for_each(|alpha| {
                let mut sources_drains_embeddings =
                    VertexEmbeddings::new_with_size(instance.layers);
                sources_drains_embeddings.embeddings[0] =
                    instance.sources.iter().map(|x| (x[0], x[1])).collect();
                sources_drains_embeddings.embeddings[instance.layers - 1] =
                    instance.sinks.iter().map(|x| (x[0], x[1])).collect();
                let mut stored_capacities = instance
                    .capacities
                    .iter()
                    .map(|&x| match x {
                        Some(capacity) => capacity,
                        None => instance.sources.len(),
                    })
                    .collect::<Vec<usize>>();
                let mut capacities = vec![1];
                capacities.append(&mut stored_capacities);
                let instance = FlamecastTestInstance::new(
                    *alpha,
                    instance.layers,
                    capacities,
                    sources_drains_embeddings,
                );

                let alpha_string = alpha.to_string().replace('.', "_");
                let solution_dir = format!("{}/{}", CIRCLE_SOLUTIONS_DIR, alpha_string);

                run_test(
                    instance,
                    &solution_dir,
                    &instance_name.to_string(),
                    InitialSolutionFunction::Matching,
                    get_circle_options,
                );

                register_job_done(&done_jobs, total_jobs);
            });
        }
    });

    println!("All jobs done!");
}
