use crate::{
    graph_embedding::embed_directed_graph, graph_generation::generate_random_flamecast_graph,
    plotting::plot_embedded_graph, Options,
};

use super::{FlamecastOptimization, GraphEmbedding, VertexEmbeddings};

pub const FLAMECAST_BASE_FILE_PATH: &str = "./solutions/";

pub struct FlamecastInstance {
    pub alpha: f64,
    pub num_layers: usize,
    pub capacities: Vec<usize>,
    pub sources_drains_embeddings: VertexEmbeddings,
    pub current_solution: GraphEmbedding,
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
            Options::default(),
        );

        let initial_solution = GraphEmbedding::new(initial_topology, initial_embedding);

        Self {
            alpha,
            num_layers,
            capacities,
            sources_drains_embeddings,
            current_solution: initial_solution,
        }
    }

    pub fn get_number_of_sources(&self) -> usize {
        self.sources_drains_embeddings.embeddings[0].len()
    }

    pub fn get_number_of_drains(&self) -> usize {
        self.sources_drains_embeddings.embeddings[self.num_layers - 1].len()
    }

    pub fn plot_current_solution(&self, file_name: &str, show_layers: bool) {
        plot_embedded_graph(
            format!("{}.png", file_name).as_str(),
            &self.current_solution,
            show_layers,
        );
    }

    pub fn get_objective_function_value(&self) -> f64 {
        self.current_solution.calculate_costs(self.alpha)
    }

    pub fn embed_current_solution(&mut self, options: Options) {
        let current_embedding = embed_directed_graph(
            &self.current_solution.base_graph,
            &self.sources_drains_embeddings,
            self.alpha,
            options,
        );
        self.current_solution.vertices_embeddings = current_embedding;
    }

    pub fn calculate_objective_function_value(&mut self, options: Options) -> f64 {
        self.embed_current_solution(options);
        self.get_objective_function_value()
    }

    pub fn solve(&mut self) {
        let optimization_instance = FlamecastOptimization::from_flamecast_instance(self);
        // TODO: optimize
    }
}
