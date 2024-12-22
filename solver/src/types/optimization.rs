use super::FlamecastInstance;

pub struct FlamecastOptimization<'a> {
    pub flamecast_instance: &'a mut FlamecastInstance,
    pub current_vertex_flows: Vec<Vec<usize>>,
    pub current_objective_value: f64,
}

impl<'a> FlamecastOptimization<'a> {
    pub fn from_flamecast_instance(flamecast_instance: &'a mut FlamecastInstance) -> Self {
        let current_vertex_flows = flamecast_instance
            .current_solution
            .base_graph
            .calculate_vertex_flows();
        let current_objective_value = flamecast_instance
            .current_solution
            .calculate_costs(flamecast_instance.alpha);
        Self {
            flamecast_instance,
            current_vertex_flows,
            current_objective_value,
        }
    }
}
