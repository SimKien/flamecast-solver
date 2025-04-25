use std::fs;

use solver::{
    FlamecastInstance, FlamecastTestInstance, InitialSolutionFunction, OptimizationOptions,
    SimulatedAnnealingLogger,
};

pub const CONVERGED_PERCENTAGE: f64 = 0.02;

pub fn run_test(
    instance: FlamecastTestInstance,
    path_dir: &String,
    name: &String,
    init_function: InitialSolutionFunction,
    options_generator: fn(f64, usize) -> OptimizationOptions,
) {
    let mut instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
        init_function,
    );

    let options = options_generator(
        instance.get_objective_function_value(),
        instance
            .solution_state
            .current_solution
            .base_graph
            .get_number_of_vertices(),
    );

    instance.solve(options);

    fs::write(
        format!("{}/{}.json", path_dir, name),
        serde_json::to_string_pretty(&instance.logger).unwrap(),
    )
    .unwrap();
}

pub fn get_converged_iteration(logs: &Vec<Vec<SimulatedAnnealingLogger>>) -> Vec<Vec<usize>> {
    let mut converged_iterations = vec![vec![]; logs.len()];
    for i in 0..logs.len() {
        for log in logs[i].iter() {
            let converged_threshold = log.final_objective_value * (1.0 + CONVERGED_PERCENTAGE);
            let mut found = false;

            if log.initial_objective_value <= converged_threshold {
                converged_iterations[i].push(0);
                continue;
            }

            for (iteration, value) in log.current_best_costs.iter().enumerate() {
                if *value <= converged_threshold {
                    converged_iterations[i].push(iteration + 1);
                    found = true;
                    break;
                }
            }
            if !found {
                converged_iterations[i].push(log.max_iterations);
            }
        }
    }
    return converged_iterations;
}
