use std::fs;

use solver::{
    FlamecastInstance, FlamecastTestInstance, InitialSolutionFunction, OptimizationOptions,
};

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
