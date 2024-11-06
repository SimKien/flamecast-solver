use plotters::{
    prelude::{BitMapBackend, IntoDrawingArea},
    style::WHITE,
};

use crate::GraphEmbedding;

use super::{create_edge, create_node};

pub const ROOT_WIDTH: u32 = 1024;
pub const ROOT_HEIGHT: u32 = 768;

pub const NODE_RADIUS: i32 = 6;

pub fn plot_embedded_graph(embedded_graph: &GraphEmbedding, show_flow: bool, show_layers: bool) {
    let root =
        BitMapBackend::new("./plots/output.png", (ROOT_WIDTH, ROOT_HEIGHT)).into_drawing_area();

    // Set background color
    root.fill(&WHITE).unwrap();

    // Draw nodes
    for vertex in embedded_graph.base_graph.vertices.iter() {
        root.draw(&create_node(
            &embedded_graph.vertices_embeddings.get(vertex).unwrap(),
        ))
        .unwrap();
    }

    // Draw edges
    for edge in embedded_graph.base_graph.edges.iter() {
        let source = embedded_graph.vertices_embeddings.get(&edge.0).unwrap();
        let target = embedded_graph.vertices_embeddings.get(&edge.1).unwrap();

        root.draw(&create_edge(source, target)).unwrap();
    }

    root.present().unwrap();
}
