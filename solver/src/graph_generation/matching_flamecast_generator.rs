use std::collections::HashMap;

use ordered_float::OrderedFloat;
use pathfinding::{matrix::Matrix, prelude::kuhn_munkres_min};

use crate::{Layer, LayeredGraph, Vertex, VertexEmbeddings};

use super::k_means_recursive;

// Creates a flamecast graph with kmeans by calcultaing the mapping of sources to drains with max matching
pub fn generate_matching_flamecast_graph(
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
    graph.layers[0] = Layer::new_with_size(sources_size);

    let sources_embeddings = &sources_drains_embeddings.embeddings[0];

    for drain in 0..drains_size {
        let drain_id = graph.add_vertex_to_layer(num_layers - 1, Vertex::new_empty());

        if drain_sources_mappings.get(&drain).is_none() {
            continue;
        }

        let corresponding_sources = drain_sources_mappings.get(&drain).unwrap();
        k_means_recursive(
            &mut graph,
            drain_id.index,
            drain_id.layer - 1,
            corresponding_sources,
            capacities,
            sources_embeddings,
        );
    }

    return graph;
}

pub fn calculate_sources_drains_weights(
    sources_drains_embeddings: &VertexEmbeddings,
    sources_size: usize,
    drains_size: usize,
    num_layers: usize,
    drain_capacity: usize,
) -> Vec<Vec<OrderedFloat<f64>>> {
    let mut weights = vec![vec![OrderedFloat(0.0); drains_size * drain_capacity]; sources_size];

    for source in 0..sources_size {
        let source_embedding = &sources_drains_embeddings.embeddings[0][source];
        for drain in 0..drains_size {
            let drain_embedding = &sources_drains_embeddings.embeddings[num_layers - 1][drain];

            // Calculate the distance between the source and drain embeddings
            let distance = (source_embedding.0 - drain_embedding.0).powi(2)
                + (source_embedding.1 - drain_embedding.1).powi(2);

            let weight = OrderedFloat(distance.sqrt());

            for i in 0..drain_capacity {
                weights[source][drain * drain_capacity + i] = weight;
            }
        }
    }

    return weights;
}
