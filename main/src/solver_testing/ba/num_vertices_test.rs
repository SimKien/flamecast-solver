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
    get_num_instances, get_num_vertices_options, register_job_done, FlamecastBaseInstance,
    INSTANCES_DIR,
};

const NUM_VERTICES_SOLUTIONS_DIR: &str = "./ba/solutions/num_vertices";
const NUM_VERTICES: [usize; 5] = [2, 5, 10, 20, 40];

const NUM_VERTICES_ALPHAS: [f64; 2] = [0.1, 0.8];

pub fn num_vertices_test(dir_name: &String) {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let dir_path: PathBuf = format!("{}/{}", INSTANCES_DIR, dir_name).into();

    if !dir_path.exists() {
        println!("Directory does not exist: {:?}", dir_path);
        return;
    }

    for alpha in NUM_VERTICES_ALPHAS.iter() {
        let alpha_string = alpha.to_string().replace('.', "_");
        let solution_base_dir = format!("{}/{}", NUM_VERTICES_SOLUTIONS_DIR, alpha_string);
        for num_vertices in NUM_VERTICES.iter() {
            let solution_dir = format!("{}/{}/{}", solution_base_dir, num_vertices, dir_name);
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
    let total_jobs = get_num_instances(&entries) * NUM_VERTICES.len() * NUM_VERTICES_ALPHAS.len();
    println!("Total jobs: {}", total_jobs);
    println!("Starting jobs...");

    entries.par_iter().for_each(|entry| {
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "json") {
            let base_instance = FlamecastBaseInstance::from_file(&file_path.display().to_string());
            let instance_name = file_path.file_stem().unwrap().to_str().unwrap();

            NUM_VERTICES.par_iter().for_each(|num_vertices| {
                for alpha in NUM_VERTICES_ALPHAS.iter() {
                    let solution_dir = format!(
                        "{}/{}/{}/{}",
                        NUM_VERTICES_SOLUTIONS_DIR,
                        alpha.to_string().replace('.', "_"),
                        num_vertices,
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
                        sources_drains_embeddings,
                    );

                    let mut instance = FlamecastInstance::new(
                        instance.alpha,
                        instance.num_layers,
                        instance.capacities,
                        instance.sources_drains_embeddings,
                        InitialSolutionFunction::Matching,
                    );

                    let options = get_num_vertices_options(
                        instance.get_objective_function_value(),
                        instance
                            .solution_state
                            .current_solution
                            .base_graph
                            .get_number_of_vertices(),
                        *num_vertices,
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
