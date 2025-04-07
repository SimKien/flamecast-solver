use solver::{
    CoolingSchedule, EmbeddingOptions, NeighborSearchOption, OptimizationOptions, SearchDepth,
};

const END_PROBABILITY: f64 = 0.01;

pub fn get_num_iterations_stupid(num_vertices: usize) -> usize {
    if num_vertices >= 150 {
        return (num_vertices as f64 * 1.5) as usize;
    } else {
        return 200;
    }
}

pub fn get_cooling_exp_configuration(
    init_objective_value: f64,
    num_iterations: usize,
) -> (CoolingSchedule, f64) {
    let init_temp = 500.0 * init_objective_value;
    let alpha = (-0.000001 / (END_PROBABILITY.ln())).powf(1.0 / (0.75 * (num_iterations as f64)));
    (CoolingSchedule::Exponential(alpha), init_temp)
}

pub fn get_alpha_options(init_objective_value: f64, num_vertices: usize) -> OptimizationOptions {
    let max_iterations = get_num_iterations_stupid(num_vertices);
    let (cooling_schedule, initial_temperature) =
        get_cooling_exp_configuration(init_objective_value, max_iterations);
    let options = OptimizationOptions::new(
        cooling_schedule,
        initial_temperature,
        NeighborSearchOption::CompleteHeuristical,
        max_iterations,
        false,
        EmbeddingOptions::from_depth(SearchDepth::Shallow),
        EmbeddingOptions::from_depth(SearchDepth::Middle),
        EmbeddingOptions::from_depth(SearchDepth::Deep),
    );

    return options;
}
