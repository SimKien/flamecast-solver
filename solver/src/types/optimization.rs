use super::FlamecastInstance;

pub struct FlamecastOptimization<'a> {
    pub flamecast_instance: &'a mut FlamecastInstance,
    pub current_flow: Vec<Vec<usize>>, // TODO: make it not edge flow but the amaount of flow going through a vertex
    pub current_objective_value: f64,
}

impl<'a> FlamecastOptimization<'a> {
    pub fn from_flamecast_instance(flamecast_instance: &'a mut FlamecastInstance) -> Self {
        let current_flow = flamecast_instance
            .current_solution
            .base_graph
            .calculate_edge_flows();
        let current_objective_value = flamecast_instance
            .current_solution
            .calculate_costs(flamecast_instance.alpha);
        Self {
            flamecast_instance,
            current_flow,
            current_objective_value,
        }
    }
}
