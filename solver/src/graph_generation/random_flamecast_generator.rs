use std::collections::HashMap;

use rand::Rng;

use crate::{Layer, LayeredGraph, Vertex, VertexEmbedding};

use super::k_means_recursive;

pub type VertexSourcesMapping = HashMap<usize, Vec<usize>>;

// Random because the mapping of sources to drains is random
pub fn generate_random_flamecast_graph(
    num_layers: usize,
    capacities: &Vec<usize>,
    sources_size: usize,
    drains_size: usize,
    sources_embeddings: &Vec<VertexEmbedding>,
) -> LayeredGraph {
    let mut graph = LayeredGraph::new_with_size(num_layers);
    graph.layers[0] = Layer::new_with_size(sources_size);

    let drain_capacity = capacities[num_layers - 1];
    let drain_sources_mappings = assign_sources_to_vertices(
        &(0..sources_size).collect(),
        &(0..drains_size).collect(),
        drain_capacity,
    );

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

fn assign_sources_to_vertices(
    sources: &Vec<usize>,
    vertices: &Vec<usize>,
    capacity: usize,
) -> VertexSourcesMapping {
    let mut vertex_sources_mapping = HashMap::new();
    let mut vertices = vertices.clone();

    for source in sources {
        let vertex_index = rand::thread_rng().gen_range(0..vertices.len());
        vertex_sources_mapping
            .entry(vertices[vertex_index])
            .or_insert(Vec::new())
            .push(*source);
        if vertex_sources_mapping
            .get(&vertices[vertex_index])
            .unwrap()
            .len()
            >= capacity
        {
            vertices.swap_remove(vertex_index);
        }
    }

    return vertex_sources_mapping;
}
