use crate::{EmbeddingOptions, FlamecastInstance, Neighbor, NeighborLoader};

impl FlamecastInstance {
    pub fn get_neighbor_cost(
        &mut self,
        neighbor: &Neighbor,
        neighbor_cost_options: &EmbeddingOptions,
    ) -> f64 {
        // calculates the objective function value of the neighbor
        let mut neighbor_loader = NeighborLoader::new();

        let current_embedding = self
            .solution_state
            .current_solution
            .vertices_embeddings
            .clone();

        neighbor_loader.load_neighbor(
            &mut self.solution_state.current_solution.base_graph,
            neighbor,
        );
        let new_objective_value = self.calculate_objective_function_value(neighbor_cost_options);
        neighbor_loader.unload_neighbor(
            &mut self.solution_state.current_solution.base_graph,
            neighbor,
        );

        // saving of the current embedding because the actual embedding was overwritten by the calculation of the neighbors cost
        self.solution_state.current_solution.vertices_embeddings = current_embedding;

        return new_objective_value;
    }
}
