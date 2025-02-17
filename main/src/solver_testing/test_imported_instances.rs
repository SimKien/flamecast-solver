use std::fs;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solver::{EmbeddingOptions, FlamecastTestInstance, NeighborSearchOption, OptimizationOptions};

use super::{solve_processing_instance, ProcessingInstance, OPTIMIZATION_OPTIONS};

pub const IMPORTED_BASE_PATH: &str = "./solver_test/imported/";

pub fn test_imported_instances(imported_indices: &Vec<usize>) {
    let processing_instances = get_processing_instances(&imported_indices);

    if processing_instances.len() == 0 {
        return;
    }

    processing_instances
        .par_iter()
        .for_each(|processing_instance| {
            let base_path = format!(
                "{}instance{}/{}",
                IMPORTED_BASE_PATH,
                processing_instance.instance_index,
                processing_instance.optimization_index
            );
            solve_processing_instance(processing_instance, &base_path);
        });

    println!("Finished processing the imported instance");
    println!("All threads finished");
}

fn get_processing_instances(imported_indices: &Vec<usize>) -> Vec<ProcessingInstance> {
    let optimization_options = OPTIMIZATION_OPTIONS.to_vec();
    let default_options = EmbeddingOptions::default();

    let mut processing_instances = Vec::new();

    for imported_index in imported_indices {
        let file_path = format!(
            "{}instance{}/instance.json",
            IMPORTED_BASE_PATH, imported_index
        );
        let file = fs::read_to_string(file_path);
        if file.is_err() {
            println!(
                "An error occured while reading the file with index {}.",
                imported_index
            );
            continue;
        }
        let file = file.unwrap();
        let instance = serde_json::from_str::<FlamecastTestInstance>(&file);
        if instance.is_err() {
            println!(
                "File with index {} isn't a valid flamecast test instance.",
                imported_index
            );
            continue;
        }
        let instance = instance.unwrap();

        optimization_options.iter().enumerate().for_each(
            |(optimization_index, optimization_option)| {
                let option = OptimizationOptions::new(
                    optimization_option.0 .0,
                    optimization_option.0 .1,
                    NeighborSearchOption::CompleteHeuristical,
                    optimization_option.1,
                    true,
                    default_options.clone(),
                    default_options.clone(),
                    default_options.clone(),
                );
                processing_instances.push(ProcessingInstance::new(
                    instance.clone(),
                    option,
                    *imported_index,
                    optimization_index,
                ));
            },
        );
    }

    println!(
        "Number of processing instances: {}",
        processing_instances.len()
    );

    return processing_instances;
}
