use std::collections::HashMap;

use pathfinding::{matrix::Matrix, prelude::kuhn_munkres_min};

use crate::{Layer, LayeredGraph, Vertex, VertexEmbeddings, VertexID};

use super::calculate_sources_drains_weights;

// Creates a flamecast graph with low connectivity between layers
pub fn generate_low_connectivity_flamecast_graph(
    num_layers: usize,
    capacities: &Vec<usize>,
    sources_size: usize,
    drains_size: usize,
    sources_drains_embeddings: &VertexEmbeddings,
) -> LayeredGraph {
    let drain_capacity = capacities[num_layers - 1];

    let weights = calculate_sources_drains_weights(
        sources_drains_embeddings,
        sources_size,
        drains_size,
        num_layers,
        drain_capacity,
    );

    let weights = Matrix::from_rows(weights).unwrap();

    let (_, assignments) = kuhn_munkres_min(&weights);

    let mut drain_sources_mappings = HashMap::new();

    for (source_id, index) in assignments.iter().enumerate() {
        let drain_id = *index / drain_capacity;

        drain_sources_mappings
            .entry(drain_id)
            .or_insert_with(Vec::new)
            .push(source_id);
    }

    let mut graph = LayeredGraph::new_with_size(num_layers);
    let num_sources = sources_drains_embeddings.embeddings[0].len();

    for layer in 0..num_layers - 1 {
        graph.layers[layer] = Layer::new_with_size(num_sources);
    }

    for drain in 0..drains_size {
        let drain_id = graph.add_vertex_to_layer(num_layers - 1, Vertex::new_empty());

        if drain_sources_mappings.get(&drain).is_none() {
            continue;
        }

        let corresponding_sources = drain_sources_mappings.get(&drain).unwrap();
        let mut current_parent = drain_id.index;
        for source in corresponding_sources {
            for layer in (0..num_layers - 1).rev() {
                let vertex = Vertex::new(Some(current_parent), None);
                graph.set_vertex_at_position(&VertexID::new(layer, *source), vertex);

                current_parent = *source;
            }
            current_parent = drain_id.index;
        }
    }

    return graph;
}
