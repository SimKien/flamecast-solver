use std::collections::HashMap;

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

pub fn plot_embedded_graph(embedded_graph: &GraphEmbedding, show_flow: bool, show_layers: bool) {
    let root =
        BitMapBackend::new("./plots/output.png", (ROOT_WIDTH, ROOT_HEIGHT)).into_drawing_area();

    // Set background color
    root.fill(&WHITE).unwrap();

    // Calculate vertex colors for nodes
    let vertex_colors = calculate_vertex_colors(&embedded_graph.base_graph, show_layers);

    // Draw vertices and edges for each layer
    for layer in embedded_graph.base_graph.layers.iter() {
        // Draw edges first because they should be behind the vertices
        for edge in layer.edges.iter() {
            let source = embedded_graph.vertices_embeddings.get(&edge.0).unwrap();
            let target = embedded_graph.vertices_embeddings.get(&edge.1).unwrap();

            root.draw(&create_edge(source, target)).unwrap();
        }

        // Draw vertices
        for vertex in layer.vertices.iter() {
            root.draw(&create_node(
                &embedded_graph.vertices_embeddings.get(vertex).unwrap(),
                vertex_colors.get(vertex).unwrap().clone(),
            ))
            .unwrap();
        }
    }

    root.present().unwrap();
}

fn calculate_vertex_colors(graph: &LayeredGraph, show_layers: bool) -> HashMap<usize, RGBColor> {
    let mut vertex_colors = HashMap::new();

    if show_layers {
        let layers = graph.get_vertex_layers();

        let mut rnd_order: Vec<usize> = (0..NUM_DISTINCT_COLORS).collect();
        rnd_order.shuffle(&mut thread_rng());

        let mut index = 0;
        for layer in layers.iter() {
            let color = DISTINCT_COLORS[rnd_order[index]];
            index += 1;

            for vertex in layer.iter() {
                vertex_colors.insert(*vertex, color);
            }

            if index == NUM_DISTINCT_COLORS {
                rnd_order.shuffle(&mut thread_rng());
                index = 0;
            }
        }

        return vertex_colors;
    } else {
        for layer in graph.layers.iter() {
            for vertex in layer.vertices.iter() {
                vertex_colors.insert(*vertex, BLACK);
            }
        }
    }

    return vertex_colors;
}
