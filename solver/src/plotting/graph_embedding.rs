use plotters::{
    prelude::{BitMapBackend, IntoDrawingArea},
    style::{RGBColor, BLACK, WHITE},
};
use rand::{seq::SliceRandom, thread_rng};

use crate::{GraphEmbedding, LayeredGraph};

use super::{create_edge, create_node, DISTINCT_COLORS, NUM_DISTINCT_COLORS};

pub const ROOT_WIDTH: u32 = 1000;
pub const ROOT_HEIGHT: u32 = 1000;

pub const NODE_RADIUS: i32 = 6;

pub fn plot_embedded_graph(file_path: &str, embedded_graph: &GraphEmbedding, show_layers: bool) {
    let root = BitMapBackend::new(file_path, (ROOT_WIDTH, ROOT_HEIGHT)).into_drawing_area();

    // Set background color
    root.fill(&WHITE).unwrap();

    // Calculate vertex colors for nodes
    let vertex_colors = calculate_vertex_colors(&embedded_graph.base_graph, show_layers);

    let graph = &embedded_graph.base_graph;
    let embeddings = &embedded_graph.vertices_embeddings;

    // Draw vertices and edges for each layer
    for (layer_index, layer) in graph.layers.iter().enumerate() {
        // Draw edges first because they should be behind the vertices

        // Draw edges
        if layer_index != graph.layers.len() - 1 {
            for (vertex_index, vertex) in layer.vertices.iter().enumerate() {
                let source = &embeddings.embeddings[layer_index][vertex_index];
                let target = &embeddings.embeddings[layer_index + 1][vertex.parent_index.unwrap()];

                root.draw(&create_edge(source, target)).unwrap();
            }
        }

        // Draw vertices
        for (vertex_index, _) in layer.vertices.iter().enumerate() {
            root.draw(&create_node(
                &embeddings.embeddings[layer_index][vertex_index],
                vertex_colors[layer_index],
            ))
            .unwrap();
        }
    }

    root.present().unwrap();
}

fn calculate_vertex_colors(graph: &LayeredGraph, show_layers: bool) -> Vec<RGBColor> {
    let mut vertex_colors = Vec::new();

    if show_layers {
        let mut rng = thread_rng();
        let mut color_indices: Vec<usize> = (0..NUM_DISTINCT_COLORS).collect();
        color_indices.shuffle(&mut rng);

        for i in 0..graph.layers.len() {
            let color = DISTINCT_COLORS[color_indices[i % NUM_DISTINCT_COLORS]];
            vertex_colors.push(color);

            if i == (NUM_DISTINCT_COLORS - 1) {
                color_indices.shuffle(&mut rng);
            }
        }
    } else {
        vertex_colors = vec![BLACK; graph.layers.len()];
    }

    return vertex_colors;
}
