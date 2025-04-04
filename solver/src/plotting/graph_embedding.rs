use plotters::{
    prelude::{BitMapBackend, IntoDrawingArea},
    style::{RGBColor, BLACK, WHITE},
};
use rand::{seq::SliceRandom, thread_rng};

use crate::{GraphEmbedding, LayeredGraph, VertexEmbeddings};

use super::{
    create_edge, create_node, DISTINCT_COLORS, NUM_DISTINCT_COLORS, ROOT_HEIGHT, ROOT_WIDTH,
};

pub fn plot_embedded_graph(
    file_path: &str,
    embedded_graph: &GraphEmbedding,
    show_layers: bool,
    show_indices: bool,
) {
    let graph = &embedded_graph.base_graph;
    let original_embeddings = &embedded_graph.vertices_embeddings;

    let x_values: Vec<f64> = original_embeddings
        .embeddings
        .iter()
        .flat_map(|layer| layer.iter().map(|vertex| vertex.0))
        .collect();
    let y_values: Vec<f64> = original_embeddings
        .embeddings
        .iter()
        .flat_map(|layer| layer.iter().map(|vertex| vertex.1))
        .collect();

    let min_x = x_values
        .iter()
        .cloned()
        .fold(f64::INFINITY, |a, b| a.min(b));
    let max_x = x_values.iter().fold(f64::NEG_INFINITY, |a, b| a.max(*b));
    let min_y = y_values
        .iter()
        .cloned()
        .fold(f64::INFINITY, |a, b| a.min(b));
    let max_y = y_values.iter().fold(f64::NEG_INFINITY, |a, b| a.max(*b));

    let embeddings = normalize_vertex_embeddings(original_embeddings, min_x, min_y, max_x, max_y);

    let root = BitMapBackend::new(file_path, (ROOT_WIDTH, ROOT_HEIGHT)).into_drawing_area();

    // Set background color
    root.fill(&WHITE).unwrap();

    // Calculate vertex colors for nodes
    let vertex_colors = calculate_vertex_colors(&embedded_graph.base_graph, show_layers);

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
            let index = if show_indices {
                Some(vertex_index)
            } else {
                None
            };
            root.draw(&create_node(
                &embeddings.embeddings[layer_index][vertex_index],
                vertex_colors[layer_index],
                index,
            ))
            .unwrap();
        }
    }
}

pub fn normalize_vertex_embeddings(
    embeddings: &VertexEmbeddings,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> VertexEmbeddings {
    let mut normalized_embeddings = VertexEmbeddings::new_with_size(embeddings.embeddings.len());

    let is_x_biggest = (max_x - min_x) > (max_y - min_y);
    let (size, padding) = if is_x_biggest {
        (
            max_x - min_x,
            (max_x - min_x - (max_y - min_y)) / (2.0 * (max_x - min_x)),
        )
    } else {
        (
            max_y - min_y,
            (max_y - min_y - (max_x - min_x)) / (2.0 * (max_y - min_y)),
        )
    };

    for (layer_index, layer) in embeddings.embeddings.iter().enumerate() {
        for vertex in layer.iter() {
            let normalized_x = if is_x_biggest {
                (vertex.0 - min_x) / size
            } else {
                (vertex.0 - min_x) / size + padding
            };
            let normalized_y = if is_x_biggest {
                (vertex.1 - min_y) / size + padding
            } else {
                (vertex.1 - min_y) / size
            };

            normalized_embeddings.embeddings[layer_index].push((normalized_x, normalized_y));
        }
    }

    return normalized_embeddings;
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
