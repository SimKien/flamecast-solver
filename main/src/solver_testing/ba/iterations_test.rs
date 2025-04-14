use std::{
    fs::{self, DirEntry},
    path::PathBuf,
    sync::Mutex,
};

use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use solver::{FlamecastInstance, FlamecastTestInstance, InitialSolutionFunction, VertexEmbeddings};

use crate::solver_testing::{
    get_iterations_options, get_num_instances, register_job_done, FlamecastBaseInstance,
    INSTANCES_DIR,
};

pub const ITERATIONS_SOLUTIONS_DIR: &str = "./ba/solutions/iterations";
const ITERATIONS_ALPHAS: [f64; 2] = [0.1, 0.8];

pub const ITERATIONS_MULTIPLIERS: [f64; 4] = [1.5, 2.0, 3.0, 4.0];

pub fn iterations_test(dir_name: &String) {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let dir_path: PathBuf = format!("{}/{}", INSTANCES_DIR, dir_name).into();

    if !dir_path.exists() {
        println!("Directory does not exist: {:?}", dir_path);
        return;
    }

    for num_iterations in ITERATIONS_MULTIPLIERS.iter() {
        for alpha in ITERATIONS_ALPHAS.iter() {
            let solution_dir = format!(
                "{}/{}/{}/{}",
                ITERATIONS_SOLUTIONS_DIR,
                alpha.to_string().replace(".", "_"),
                num_iterations.to_string().replace(".", "_"),
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
    }

    let entries = std::fs::read_dir(&dir_path)
        .expect("Failed to read directory")
        .map(|entry| entry.expect("Failed to read entry"))
        .collect::<Vec<DirEntry>>();

    let done_jobs = Mutex::new(0);
    let total_jobs =
        get_num_instances(&entries) * ITERATIONS_MULTIPLIERS.len() * ITERATIONS_ALPHAS.len();
    println!("Total jobs: {}", total_jobs);
    println!("Starting jobs...");

    entries.par_iter().for_each(|entry| {
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "json") {
            let base_instance = FlamecastBaseInstance::from_file(&file_path.display().to_string());
            let instance_name = file_path.file_stem().unwrap().to_str().unwrap();

            ITERATIONS_MULTIPLIERS
                .par_iter()
                .for_each(|num_iterations| {
                    for alpha in ITERATIONS_ALPHAS.iter() {
                        let solution_dir = format!(
                            "{}/{}/{}/{}",
                            ITERATIONS_SOLUTIONS_DIR,
                            alpha.to_string().replace(".", "_"),
                            num_iterations.to_string().replace(".", "_"),
                            dir_name
                        );

                        let mut sources_drains_embeddings =
                            VertexEmbeddings::new_with_size(base_instance.layers);
                        sources_drains_embeddings.embeddings[0] = base_instance.sources.clone();
                        sources_drains_embeddings.embeddings[base_instance.layers - 1] =
                            base_instance.drains.clone();

                        let instance = FlamecastTestInstance::new(
                            *alpha,
                            base_instance.layers,
                            base_instance.capacities.clone(),
                            sources_drains_embeddings.clone(),
                        );

                        let mut instance = FlamecastInstance::new(
                            instance.alpha,
                            instance.num_layers,
                            instance.capacities,
                            instance.sources_drains_embeddings,
                            InitialSolutionFunction::Matching,
                        );

                        let options = get_iterations_options(
                            instance.get_objective_function_value(),
                            instance
                                .solution_state
                                .current_solution
                                .base_graph
                                .get_number_of_vertices(),
                            *num_iterations,
                        );

                        instance.solve(options);

                        fs::write(
                            format!("{}/{}.json", solution_dir, instance_name),
                            serde_json::to_string_pretty(&instance.logger).unwrap(),
                        )
                        .unwrap();

                        register_job_done(&done_jobs, total_jobs);
                    }
                });
        }
    });

    println!("All jobs done!");
}
