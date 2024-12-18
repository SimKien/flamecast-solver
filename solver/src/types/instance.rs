use crate::{
    graph_embedding::embed_directed_graph, graph_generation::generate_random_directed_graph,
    plotting::plot_embedded_graph, Options,
};

use super::{GraphEmbedding, VertexEmbeddings};

pub const BASE_FILE_PATH: &str = "./solutions/";

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
        let num_nodes = (0.5
            * sources_drains_embeddings.embeddings[0].len() as f64
            * num_layers as f64) as usize;
        let initial_topology = generate_random_directed_graph(num_nodes, num_layers);
        let initial_embedding = embed_directed_graph(
            &initial_topology,
            &sources_drains_embeddings,
            alpha,
            Options::default(),
        );

        //TODO: Use random_flamecast_generator to generate the initial solution

        let initial_solution = GraphEmbedding::new(initial_topology, initial_embedding);

        Self {
            alpha,
            num_layers,
            capacities,
            sources_drains_embeddings,
            current_solution: initial_solution,
        }
    }

    pub fn plot_current_solution(&self, file_name: &str, show_layers: bool) {
        plot_embedded_graph(
            format!("{}{}.png", BASE_FILE_PATH, file_name).as_str(),
            &self.current_solution,
            show_layers,
        );
    }

    pub fn solve(&mut self) {
        //TODO: Implement the solver
    }
}
