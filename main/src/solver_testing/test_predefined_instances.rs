use std::{fs, io::Write};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solver::{
    generate_flamecast_instance, EmbeddingOptions, FlamecastTestInstance, NeighborSearchOption,
    OptimizationOptions,
};

use crate::solver_testing::PREDEFINED_BASE_PATH;

use super::{INSTANCES, OPTIMIZATION_OPTIONS};

#[derive(Debug, Clone)]
pub struct ProcessingInstance {
    pub instance: FlamecastTestInstance,
    pub optimization_option: OptimizationOptions,
    pub instance_index: usize,
    pub optimization_index: usize,
}

impl ProcessingInstance {
    pub fn new(
        instance: FlamecastTestInstance,
        optimization_option: OptimizationOptions,
        instance_index: usize,
        optimization_index: usize,
    ) -> Self {
        Self {
            instance,
            optimization_option,
            instance_index,
            optimization_index,
        }
    }
}

pub fn test_predefined_instances(predefined_instances_indexes: &Vec<usize>) {
    let test_processing_instances = get_processing_instances(predefined_instances_indexes);

    test_processing_instances
        .par_iter()
        .for_each(|processing_instance| {
            let base_path = format!(
                "{}instance{}/{}",
                PREDEFINED_BASE_PATH,
                processing_instance.instance_index,
                processing_instance.optimization_index
            );
            solve_processing_instance(processing_instance, &base_path);
        });

    println!("Finished processing all instances");
    println!("All threads finished");
}

pub fn solve_processing_instance(processing_instance: &ProcessingInstance, base_path: &String) {
    let instance_index = processing_instance.instance_index;
    let processing_index = processing_instance.optimization_index;
    let current_index = instance_index * OPTIMIZATION_OPTIONS.len() + processing_index;

    let mut instance = generate_flamecast_instance(
        processing_instance.instance.alpha,
        processing_instance.instance.num_layers,
        processing_instance.instance.capacities.clone(),
        processing_instance
            .instance
            .sources_drains_embeddings
            .clone(),
    );

    instance.plot_current_solution(
        format!("{}initial_solution.png", base_path).as_str(),
        true,
        false,
    );
    fs::write(
        format!("{}initial_solution.json", base_path),
        serde_json::to_string_pretty(&instance.solution_state.current_solution).unwrap(),
    )
    .unwrap();

    println!("Start processing instance {}", current_index);

    let mut log_file = fs::File::create(format!("{}log.txt", base_path)).unwrap();

    instance.solve(
        processing_instance.optimization_option.clone(),
        Some(&mut log_file),
    );

    log_file.flush().unwrap();

    fs::write(
        format!("{}accepted_neighbors.json", base_path),
        serde_json::to_string_pretty(&instance.solution_state.accepted_neighbors).unwrap(),
    )
    .unwrap();
    fs::write(
        format!("{}best_iteration.json", base_path),
        serde_json::to_string_pretty(&instance.solution_state.best_iteration).unwrap(),
    )
    .unwrap();

    instance.plot_current_solution(
        format!("{}final_solution.png", base_path).as_str(),
        true,
        false,
    );
    fs::write(
        format!("{}final_solution.json", base_path),
        serde_json::to_string_pretty(&instance.solution_state.current_solution).unwrap(),
    )
    .unwrap();

    println!("Finished processing instance {}", current_index);
}

fn get_processing_instances(predefined_instances_indexes: &Vec<usize>) -> Vec<ProcessingInstance> {
    let optimization_options = OPTIMIZATION_OPTIONS.to_vec();

    let mut processing_instances = Vec::new();

    let predefined_instances_indexes = if predefined_instances_indexes.len() == 0 {
        &(0..INSTANCES.len()).collect::<Vec<usize>>()
    } else {
        predefined_instances_indexes
    };

    for instance_index in predefined_instances_indexes {
        let file_path = format!(
            "{}instance{}/instance.json",
            PREDEFINED_BASE_PATH, instance_index
        );
        let test_instance = fs::read_to_string(file_path)
            .ok()
            .and_then(|test_instance| {
                serde_json::from_str::<FlamecastTestInstance>(&test_instance).ok()
            })
            .unwrap();

        let default_options = EmbeddingOptions::default();
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
                    test_instance.clone(),
                    option,
                    *instance_index,
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
