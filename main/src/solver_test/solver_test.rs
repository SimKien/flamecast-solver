use std::{
    fs,
    io::Write,
    sync::{Arc, Mutex},
};

use solver::{
    generate_flamecast_instance, EmbeddingOptions, FlamecastTestInstance, OptimizationOptions,
};

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

#[derive(Debug)]
pub struct ProcessingState {
    processing_instances: Vec<ProcessingInstance>,
    current_index: usize,
}

impl ProcessingState {
    pub fn new(processing_instances: Vec<ProcessingInstance>) -> Self {
        Self {
            processing_instances,
            current_index: 0,
        }
    }
}

pub fn run_solver_tests(num_threads: usize) {
    let test_optimization_instances = get_processing_instances();

    let processing_state = Arc::new(Mutex::new(ProcessingState::new(
        test_optimization_instances,
    )));

    let mut threads = Vec::new();

    for index in 0..num_threads {
        let processing_state = Arc::clone(&processing_state);
        let thread = std::thread::spawn(move || process_thread(processing_state, index));

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Finished processing all instances");
    println!("All threads finished");
}

fn process_thread(processing_state: Arc<Mutex<ProcessingState>>, thread_index: usize) {
    loop {
        let mut state = processing_state.lock().unwrap();
        if state.current_index >= state.processing_instances.len() {
            println!("Thread {} finished", thread_index);
            break;
        }

        let current_index = state.current_index;
        state.current_index += 1;

        let processing_instance = state.processing_instances[current_index].clone();

        drop(state);

        let instance_index = processing_instance.instance_index;
        let optimization_index = processing_instance.optimization_index;
        let base_path = format!(
            "./solver_test/instance{}/{}",
            instance_index, optimization_index
        );

        let mut instance = generate_flamecast_instance(
            processing_instance.instance.alpha,
            processing_instance.instance.num_layers,
            processing_instance.instance.capacities,
            processing_instance
                .instance
                .sources_drains_embeddings
                .clone(),
        );

        instance.plot_current_solution(format!("{}initial_solution.png", base_path).as_str(), true);
        fs::write(
            format!("{}initial_solution.json", base_path),
            serde_json::to_string_pretty(&instance.current_solution).unwrap(),
        )
        .unwrap();

        println!("Start processing instance {}", current_index);

        let mut log_file = fs::File::create(format!("{}log.txt", base_path)).unwrap();

        instance.solve(processing_instance.optimization_option, &mut log_file);

        log_file.flush().unwrap();

        fs::write(
            format!("{}accepted_neighbors.json", base_path),
            serde_json::to_string_pretty(&instance.accepted_neighbors).unwrap(),
        )
        .unwrap();

        instance.plot_current_solution(format!("{}final_solution.png", base_path).as_str(), true);
        fs::write(
            format!("{}final_solution.json", base_path),
            serde_json::to_string_pretty(&instance.current_solution).unwrap(),
        )
        .unwrap();

        println!("Finished processing instance {}", current_index);
    }
}

fn get_processing_instances() -> Vec<ProcessingInstance> {
    let optimization_options = OPTIMIZATION_OPTIONS.to_vec();

    let mut processing_instances = Vec::new();

    for instance_index in 0..INSTANCES.len() {
        let file_path = format!("./solver_test/instance{}/instance.json", instance_index);
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
                    optimization_option.1,
                    true,
                    default_options.clone(),
                    default_options.clone(),
                    default_options.clone(),
                );
                processing_instances.push(ProcessingInstance::new(
                    test_instance.clone(),
                    option,
                    instance_index,
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
