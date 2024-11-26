mod graph_embedding;
mod graph_generation;
mod plotting;
mod types;

use graph_embedding::embed_directed_graph;
pub use graph_embedding::{Options, SearchDepth};
use graph_generation::generate_random_directed_graph;
use plotting::plot_embedded_graph;
pub use types::*;

pub fn hello_world() {
    println!("Hello, world!");
}

pub fn generate_random_graph(num_nodes: usize, num_layers: usize) -> LayeredGraph {
    return generate_random_directed_graph(num_nodes, num_layers);
}

pub fn get_sources(graph: &LayeredGraph) -> Vec<Vertex> {
    return graph.get_sources();
}

pub fn get_drains(graph: &LayeredGraph) -> Vec<Vertex> {
    return graph.get_drains();
}

pub fn get_layers(graph: &LayeredGraph) -> Vec<Vec<Vertex>> {
    return graph.get_vertex_layers();
}

pub fn embed_graph(
    graph: LayeredGraph,
    alpha: f64,
    sources_embeddings: &VertexEmbeddings,
    drains_embeddings: &VertexEmbeddings,
    options: Options,
) -> GraphEmbedding {
    let sources = &get_sources(&graph);
    let drains = &get_drains(&graph);
    let graph_embedding = embed_directed_graph(
        &graph,
        alpha,
        sources,
        drains,
        sources_embeddings,
        drains_embeddings,
        options,
    );
    return GraphEmbedding::new(graph, graph_embedding);
}

pub fn plot_graph(embedded_graph: &GraphEmbedding, show_flow: bool, show_layers: bool) {
    plot_embedded_graph(embedded_graph, show_flow, show_layers);
}
