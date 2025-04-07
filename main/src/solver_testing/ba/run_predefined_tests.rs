use std::{
    fs::{self, DirEntry},
    sync::Mutex,
};

use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use serde::{Deserialize, Serialize};
use solver::{FlamecastTestInstance, InitialSolutionFunction, VertexEmbeddings};

use super::run_test;

pub const PREDEFINED_INSTANCES_BASE_DIR: &str = "./ba/predefined";
pub const PREDEFINED_SOLUTIONS_BASE_DIR: &str = "./ba/predefined_solutions";
pub const ALPHA_FILE: &str = "./ba/options/alpha.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredefinedInstances {
    pub sources: Vec<Vec<f64>>,
    pub sinks: Vec<Vec<f64>>,
    pub layers: usize,
    pub capacities: Vec<usize>,
}

pub fn run_predefined_tests(instance_dir: Vec<String>, alpha: Option<f64>) {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    if instance_dir.is_empty() {
        println!("No instance directory provided.");
        return;
    }

    let finished_run_instances = Mutex::new(0);

    for dir in instance_dir.iter() {
        let instances_dir = format!("{}/{}", PREDEFINED_INSTANCES_BASE_DIR, dir);
        let instance_files = std::fs::read_dir(instances_dir)
            .expect("Failed to read directory")
            .map(|entry| entry.expect("Failed to read entry"))
            .collect::<Vec<DirEntry>>();

        let alphas = if alpha.is_none() {
            let alpha_file = fs::read_to_string(ALPHA_FILE);
            if alpha_file.is_err() {
                println!("Failed to read alpha file: {:?}", ALPHA_FILE);
                return;
            }
            let alpha_file = alpha_file.unwrap();
            let alpha_value = serde_json::from_str::<Vec<f64>>(&alpha_file);
            if alpha_value.is_err() {
                println!("Failed to parse alpha JSON: {:?}", ALPHA_FILE);
                return;
            }
            alpha_value.unwrap()
        } else {
            vec![alpha.unwrap()]
        };

        instance_files.par_iter().for_each(|entry| {
            process_predefined_entry(entry, &alphas, &dir);
            {
                let mut finished_run_instances = finished_run_instances.lock().unwrap();
                *finished_run_instances += 1;
                println!("Finished run instances: {}", finished_run_instances);
            }
        });
    }
}

fn process_predefined_entry(entry: &DirEntry, alphas: &Vec<f64>, instance_dir: &String) {
    let path = entry.path();
    if path.is_file() {
        let file = fs::read_to_string(path.clone());
        if file.is_err() {
            println!("Failed to read file: {:?}", path);
            return;
        }
        let file = file.unwrap();
        let instance = serde_json::from_str::<PredefinedInstances>(&file);
        if instance.is_err() {
            println!("Failed to parse JSON: {:?}", path);
            return;
        }
        let instance = instance.unwrap();
        let mut sources_drains_embeddings = VertexEmbeddings::new_with_size(instance.layers);
        for source in instance.sources.iter() {
            sources_drains_embeddings.embeddings[0].push((source[0], source[1]));
        }
        for sink in instance.sinks.iter() {
            sources_drains_embeddings.embeddings[instance.layers - 1].push((sink[0], sink[1]));
        }
        let mut capacities = Vec::with_capacity(instance.layers);
        capacities.push(1);
        capacities.append(&mut instance.capacities.clone());

        alphas.par_iter().for_each(|alpha| {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let name = file_name.split('.').next().unwrap().to_string();

            let instance = FlamecastTestInstance::new(
                *alpha,
                instance.layers,
                capacities.clone(),
                sources_drains_embeddings.clone(),
            );

            let alpha_dir = alpha.to_string().replace('.', "_");
            let instance_dir = format!("{}/{}", alpha_dir, instance_dir);

            for init_function in InitialSolutionFunction::iterate() {
                let path_dir = format!(
                    "{}/{}/{}",
                    PREDEFINED_SOLUTIONS_BASE_DIR,
                    init_function.to_string(),
                    instance_dir
                );

                run_test(instance.clone(), &path_dir, &name, init_function);
            }
        });
    }
}
