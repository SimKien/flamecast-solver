use solver::{
    CoolingSchedule, EmbeddingOptions, NeighborSearchOption, OptimizationOptions, SearchDepth,
};

const END_PROBABILITY: f64 = 0.01;

pub fn get_num_iterations(num_vertices: usize) -> usize {
    if num_vertices >= 150 {
        return (num_vertices as f64 * 2.0) as usize;
    } else {
        return 300;
    }
}

pub fn get_cooling_exp_configuration(
    init_objective_value: f64,
    num_iterations: usize,
) -> (CoolingSchedule, f64) {
    let init_temp = 3.9 * init_objective_value; // probability that first change with +0.2 relative costs = 0.95
    let alpha =
        (-(0.0001 / 3.9) / (END_PROBABILITY.ln())).powf(1.0 / (0.95 * (num_iterations as f64))); //after 95% of iterations, the probability of accepting a change with +0.0001 relative costs = END_PROBABILITY
    (CoolingSchedule::Exponential(alpha), init_temp)
}

pub fn get_alpha_options(init_objective_value: f64, num_vertices: usize) -> OptimizationOptions {
    let max_iterations = get_num_iterations(num_vertices);
    let (cooling_schedule, initial_temperature) =
        get_cooling_exp_configuration(init_objective_value, max_iterations);
    let number_random_vertices = 10;
    let options = OptimizationOptions::new(
        cooling_schedule,
        initial_temperature,
        NeighborSearchOption::CompleteHeuristical,
        max_iterations,
        number_random_vertices,
        false,
        EmbeddingOptions::from_depth(SearchDepth::Shallow),
        EmbeddingOptions::from_depth(SearchDepth::Middle),
        EmbeddingOptions::from_depth(SearchDepth::Deep),
    );

    return options;
}

pub fn get_init_options(init_objective_value: f64, num_vertices: usize) -> OptimizationOptions {
    let max_iterations = get_num_iterations(num_vertices);
    let (cooling_schedule, initial_temperature) =
        get_cooling_exp_configuration(init_objective_value, max_iterations);
    let number_random_vertices = 10;
    let options = OptimizationOptions::new(
        cooling_schedule,
        initial_temperature,
        NeighborSearchOption::CompleteHeuristical,
        max_iterations,
        number_random_vertices,
        false,
        EmbeddingOptions::from_depth(SearchDepth::Shallow),
        EmbeddingOptions::from_depth(SearchDepth::Middle),
        EmbeddingOptions::from_depth(SearchDepth::Deep),
    );

    return options;
}

pub fn get_num_vertices_options(
    init_objective_value: f64,
    num_vertices: usize,
    random_num_vertices: usize,
) -> OptimizationOptions {
    let max_iterations = get_num_iterations(num_vertices);
    let (cooling_schedule, initial_temperature) =
        get_cooling_exp_configuration(init_objective_value, max_iterations);
    let number_random_vertices = random_num_vertices;
    let options = OptimizationOptions::new(
        cooling_schedule,
        initial_temperature,
        NeighborSearchOption::CompleteHeuristical,
        max_iterations,
        number_random_vertices,
        false,
        EmbeddingOptions::from_depth(SearchDepth::Shallow),
        EmbeddingOptions::from_depth(SearchDepth::Middle),
        EmbeddingOptions::from_depth(SearchDepth::Deep),
    );

    return options;
}

pub fn get_iterations_options(
    init_objective_value: f64,
    num_vertices: usize,
    num_iterations_multiplier: f64,
) -> OptimizationOptions {
    let max_iterations = ((num_vertices as f64) * num_iterations_multiplier) as usize;
    let (cooling_schedule, initial_temperature) =
        get_cooling_exp_configuration(init_objective_value, max_iterations);
    let number_random_vertices = 10;
    let options = OptimizationOptions::new(
        cooling_schedule,
        initial_temperature,
        NeighborSearchOption::CompleteHeuristical,
        max_iterations,
        number_random_vertices,
        false,
        EmbeddingOptions::from_depth(SearchDepth::Shallow),
        EmbeddingOptions::from_depth(SearchDepth::Middle),
        EmbeddingOptions::from_depth(SearchDepth::Deep),
    );

    return options;
}
