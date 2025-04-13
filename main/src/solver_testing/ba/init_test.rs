use std::{fs::DirEntry, path::PathBuf, sync::Mutex};

use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use solver::{FlamecastTestInstance, InitialSolutionFunction, VertexEmbeddings};

use crate::solver_testing::{
    get_init_options, get_num_instances, register_job_done, run_test, FlamecastBaseInstance,
    INSTANCES_DIR,
};

pub const INIT_SOLUTIONS_DIR: &str = "./ba/solutions/init";
pub const INIT_FUNCTION_TYPES: [InitialSolutionFunction; 2] = [
    InitialSolutionFunction::Random,
    InitialSolutionFunction::Matching,
];

const INIT_ALPHA: f64 = 0.1;

pub fn init_test(dir_name: &String) {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let dir_path: PathBuf = format!("{}/{}", INSTANCES_DIR, dir_name).into();

    if !dir_path.exists() {
        println!("Directory does not exist: {:?}", dir_path);
        return;
    }

    for init_solution in INIT_FUNCTION_TYPES.iter() {
        let solution_dir = format!(
            "{}/{}/{}",
            INIT_SOLUTIONS_DIR,
            init_solution.to_string(),
            dir_name
        );
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
    let total_jobs = get_num_instances(&entries) * INIT_FUNCTION_TYPES.len();
    println!("Total jobs: {}", total_jobs);
    println!("Starting jobs...");

    entries.par_iter().for_each(|entry| {
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "json") {
            let base_instance = FlamecastBaseInstance::from_file(&file_path.display().to_string());
            let instance_name = file_path.file_stem().unwrap().to_str().unwrap();

            INIT_FUNCTION_TYPES.par_iter().for_each(|init_function| {
                let solution_dir = format!(
                    "{}/{}/{}",
                    INIT_SOLUTIONS_DIR,
                    init_function.to_string(),
                    dir_name
                );

                let mut sources_drains_embeddings =
                    VertexEmbeddings::new_with_size(base_instance.layers);
                sources_drains_embeddings.embeddings[0] = base_instance.sources.clone();
                sources_drains_embeddings.embeddings[base_instance.layers - 1] =
                    base_instance.drains.clone();
                let instance = FlamecastTestInstance::new(
                    INIT_ALPHA,
                    base_instance.layers,
                    base_instance.capacities.clone(),
                    sources_drains_embeddings,
                );

                run_test(
                    instance,
                    &solution_dir,
                    &instance_name.to_string(),
                    init_function.clone(),
                    get_init_options,
                );

                register_job_done(&done_jobs, total_jobs);
            });
        }
    });

    println!("All jobs done!");
}
