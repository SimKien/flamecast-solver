use std::fs;

use solver::{
    EmbeddingOptions, FlamecastInstance, FlamecastTestInstance, InitialSolutionFunction,
    NeighborSearchOption, OptimizationOptions, SearchDepth,
};

use super::{get_cooling_configuration, get_num_iterations};

pub fn run_test(
    instance: FlamecastTestInstance,
    path_dir: &String,
    name: &String,
    init_function: InitialSolutionFunction,
) {
    let mut instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
        init_function,
    );

    let init_objective_value = instance.get_objective_function_value();

    let num_vertices = instance
        .solution_state
        .current_solution
        .base_graph
        .get_number_of_vertices();
    let num_iterations = get_num_iterations(num_vertices);
    let cooling_config = get_cooling_configuration(init_objective_value, num_iterations);
    let neighbor_test_options = EmbeddingOptions::from_depth(SearchDepth::Shallow);
    let neighbor_cost_options = EmbeddingOptions::from_depth(SearchDepth::Middle);
    let final_cost_options = EmbeddingOptions::from_depth(SearchDepth::Deep);

    let options = OptimizationOptions::new(
        cooling_config.0,
        cooling_config.1,
        NeighborSearchOption::CompleteHeuristical,
        num_iterations,
        false,
        neighbor_test_options,
        neighbor_cost_options,
        final_cost_options,
    );

    instance.solve(options);

    fs::write(
        format!("{}/{}.json", path_dir, name),
        serde_json::to_string_pretty(&instance.logger).unwrap(),
    )
    .unwrap();
}
