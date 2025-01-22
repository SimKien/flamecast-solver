use serde::{Deserialize, Serialize};

use crate::{
    graph_embedding::embed_directed_graph,
    graph_generation::generate_random_flamecast_graph,
    plotting::plot_embedded_graph,
    simulated_annealing::{OptimizationOptions, SimulatedAnnealing},
    EmbeddingOptions, GraphEmbedding, Neighbor, VertexEmbeddings,
};

pub const FLAMECAST_BASE_FILE_PATH: &str = "./solutions/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlamecastInstance {
    pub alpha: f64,
    pub num_layers: usize,
    pub capacities: Vec<usize>,
    pub sources_drains_embeddings: VertexEmbeddings,
    pub current_solution: GraphEmbedding,
    pub accepted_neighbors: Vec<Neighbor>,
}

impl FlamecastInstance {
    pub fn new(
        alpha: f64,
        num_layers: usize,
        capacities: Vec<usize>,
        sources_drains_embeddings: VertexEmbeddings,
    ) -> Self {
        let initial_topology = generate_random_flamecast_graph(
            num_layers,
            &capacities,
            sources_drains_embeddings.embeddings[0].len(),
            sources_drains_embeddings.embeddings[num_layers - 1].len(),
        );
        let initial_embedding = embed_directed_graph(
            &initial_topology,
            &sources_drains_embeddings,
            alpha,
            &EmbeddingOptions::default(),
        );

        let initial_solution = GraphEmbedding::new(initial_topology, initial_embedding);

        Self {
            alpha,
            num_layers,
            capacities,
            sources_drains_embeddings,
            current_solution: initial_solution,
            accepted_neighbors: Vec::new(),
        }
    }

    pub fn get_number_of_sources(&self) -> usize {
        self.sources_drains_embeddings.embeddings[0].len()
    }

    pub fn get_number_of_drains(&self) -> usize {
        self.sources_drains_embeddings.embeddings[self.num_layers - 1].len()
    }

    pub fn plot_current_solution(&self, file_path: &str, show_layers: bool) {
        plot_embedded_graph(file_path, &self.current_solution, show_layers);
    }

    pub fn get_objective_function_value(&self) -> f64 {
        self.current_solution.calculate_costs(self.alpha)
    }

    pub fn embed_current_solution(&mut self, options: &EmbeddingOptions) {
        let current_embedding = embed_directed_graph(
            &self.current_solution.base_graph,
            &self.sources_drains_embeddings,
            self.alpha,
            options,
        );
        self.current_solution.vertices_embeddings = current_embedding;
    }

    pub fn calculate_objective_function_value(&mut self, options: &EmbeddingOptions) -> f64 {
        self.embed_current_solution(options);
        self.get_objective_function_value()
    }

    pub fn solve<T: std::io::Write>(&mut self, options: OptimizationOptions, buf: &mut T) -> f64 {
        let mut optimization_instance = SimulatedAnnealing::from_flamecast_instance(self, options);

        return optimization_instance.solve(buf);
    }
}
