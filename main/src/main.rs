use solver::{
    generate_flamecast_instance, generate_random_flamecast_test_instance, EmbeddingOptions,
    OptimizationOptions, SearchDepth,
};

fn main() {
    let embed_options =
        EmbeddingOptions::new(false, SearchDepth::Middle, f64::INFINITY, false, false);

    let flamecast_test_instance = generate_random_flamecast_test_instance(6, 10, 3, true);
    let mut flamecast_instance = generate_flamecast_instance(
        flamecast_test_instance.alpha,
        flamecast_test_instance.num_layers,
        flamecast_test_instance.capacities,
        flamecast_test_instance.sources_drains_embeddings,
    );

    let value = flamecast_instance.calculate_objective_function_value(&embed_options);
    flamecast_instance.plot_current_solution("./solutions/initial.png", true);
    println!("Initial Objective Function Value: {}", value);

    let result = flamecast_instance.solve(OptimizationOptions::default());
    flamecast_instance.plot_current_solution("./solutions/result.png", true);
    println!("Initial Objective Function Value: {}", value);
    println!("Result: {}", result);
}
