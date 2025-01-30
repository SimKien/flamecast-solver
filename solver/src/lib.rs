mod graph_embedding;
mod graph_generation;
mod neighborhood;
mod plotting;
mod simulated_annealing;
mod tests;
mod timer;
mod types;
mod variable_neighborhood_search;

use graph_embedding::embed_directed_graph;
pub use graph_embedding::{EmbeddingOptions, SearchDepth};
use graph_generation::generate_random_directed_graph;
pub use neighborhood::Neighbor;
pub use plotting::PlottingVertices;
use plotting::{plot_embedded_graph, plot_vertices_with_colors};
pub use simulated_annealing::{CoolingSchedule, NeighborSearchOption, OptimizationOptions};
use tests::{
    combine_test_graphs, generate_random_flamecast_instance, FLAMECAST_TEST_INSTANCES,
    NEIGHBORHOOD_TEST_INSTANCES, TESTGRAPHS,
};
pub use tests::{FlamecastTestInstance, TestGraph};
pub use timer::Stopwatch;
pub use types::*;

pub fn hello_world() {
    println!("Hello, world!");
}

pub fn generate_random_graph(num_nodes: usize, num_layers: usize) -> LayeredGraph {
    return generate_random_directed_graph(num_nodes, num_layers);
}

pub fn generate_random_flamecast_test_instance(
    num_layers: usize,
    num_sources: usize,
    num_drains: usize,
    clear_structure: bool,
) -> FlamecastTestInstance {
    return generate_random_flamecast_instance(
        num_layers,
        num_sources,
        num_drains,
        clear_structure,
    );
}

pub fn generate_flamecast_instance(
    alpha: f64,
    num_layers: usize,
    capacities: Vec<usize>,
    sources_drains_embeddings: VertexEmbeddings,
) -> FlamecastInstance {
    return FlamecastInstance::new(alpha, num_layers, capacities, sources_drains_embeddings);
}

pub fn embed_graph(
    graph: LayeredGraph,
    sources_drains_embeddings: &VertexEmbeddings,
    alpha: f64,
    options: &EmbeddingOptions,
) -> GraphEmbedding {
    let graph_embedding = embed_directed_graph(&graph, sources_drains_embeddings, alpha, options);
    return GraphEmbedding::new(graph, graph_embedding);
}

pub fn plot_graph(file_path: &str, embedded_graph: &GraphEmbedding, show_layers: bool) {
    plot_embedded_graph(file_path, embedded_graph, show_layers);
}

pub fn plot_vertices(file_path: &str, plotting_vertices: Vec<PlottingVertices>) {
    plot_vertices_with_colors(file_path, &plotting_vertices);
}

pub fn get_test_graph(index: usize) -> TestGraph {
    return TESTGRAPHS[index].clone();
}

pub fn get_test_graphs_len() -> usize {
    return TESTGRAPHS.len();
}

pub fn combine_testing_graphs(graph1: &mut TestGraph, graph2: &TestGraph) -> Option<TestGraph> {
    return combine_test_graphs(graph1, graph2);
}

pub fn get_test_flamecast_instance(index: usize) -> FlamecastTestInstance {
    return FLAMECAST_TEST_INSTANCES[index].clone();
}

pub fn get_test_flamecast_instances_len() -> usize {
    return FLAMECAST_TEST_INSTANCES.len();
}

pub fn get_neighborhood_test_instance(index: usize) -> FlamecastInstance {
    return NEIGHBORHOOD_TEST_INSTANCES[index].clone();
}

pub fn get_neighborhood_test_instances_len() -> usize {
    return NEIGHBORHOOD_TEST_INSTANCES.len();
}
