pub fn neighborhood_change_probability(
    current_objective_value: f64,
    neighbor_objective_value: f64,
    temperature: f64,
) -> f64 {
    let delta = neighbor_objective_value - current_objective_value;
    (-delta / temperature).exp()
}