mod graph_embedding;
mod graph_generation;
mod neighborhood;
mod plotting;
mod simulated_annealing;
mod tests;
mod types;
mod variable_neighborhood_search;

use graph_embedding::embed_directed_graph;
pub use graph_embedding::{EmbeddingOptions, SearchDepth};
use graph_generation::generate_random_directed_graph;
pub use neighborhood::Neighbor;
use plotting::plot_embedded_graph;
pub use simulated_annealing::{CoolingSchedule, OptimizationOptions};
pub use tests::TestGraph;
use tests::{
    combine_test_graphs, generate_random_flamecast_instance, FlamecastTestInstance,
    FLAMECAST_TEST_INSTANCES, NEIGHBORHOOD_TEST_INSTANCES, TESTGRAPHS,
};
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

pub fn get_sources_indexes(graph: &LayeredGraph) -> Vec<VertexID> {
    return graph.get_sources_indexes();
}

pub fn get_drains_indexes(graph: &LayeredGraph) -> Vec<VertexID> {
    return graph.get_drains_indexes();
}

pub fn get_layers_with_indexes(graph: &LayeredGraph) -> Vec<Vec<VertexID>> {
    return graph.get_vertex_layers_with_indexes();
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

pub fn get_test_graph(index: usize) -> TestGraph {
    return TESTGRAPHS[index].clone();
}

pub fn get_test_graphs_len() -> usize {
    return TESTGRAPHS.len();
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

pub fn combine_testing_graphs(graph1: &mut TestGraph, graph2: &TestGraph) -> Option<TestGraph> {
    return combine_test_graphs(graph1, graph2);
}
