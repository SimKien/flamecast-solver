use solver::CoolingSchedule;

const END_PROBABILITY: f64 = 0.01;

pub fn get_num_iterations(num_vertices: usize) -> usize {
    if num_vertices >= 200 {
        return (num_vertices as f64 * (1.0 + 1.001 * 0.9998_f64.powf(num_vertices as f64)))
            as usize;
    } else if num_vertices >= 20 {
        return (num_vertices as f64 * (6.0 - 0.02_f64 * num_vertices as f64)) as usize;
    } else {
        return 100;
    }
}

pub fn get_cooling_configuration(
    init_objective_value: f64,
    num_iterations: usize,
) -> (CoolingSchedule, f64) {
    let init_temp = 500.0 * init_objective_value;
    let alpha = (-0.000001 / (END_PROBABILITY.ln())).powf(1.0 / (0.8 * (num_iterations as f64)));
    (CoolingSchedule::Exponential(alpha), init_temp)
}
