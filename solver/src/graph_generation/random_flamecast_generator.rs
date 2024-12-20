use std::collections::HashMap;

use rand::Rng;

use crate::{LayeredGraph, Vertex};

type VertexSourcesMapping = HashMap<usize, Vec<usize>>;

// Random because the mapping of sources to vertices and drains is random
pub fn generate_random_flamecast_graph(
    num_layers: usize,
    capacities: &Vec<usize>,
    sources_size: usize,
    drains_size: usize,
) -> LayeredGraph {
    let mut graph = LayeredGraph::new_with_size(num_layers);

    let mut drain_sources_mappings: HashMap<usize, Vec<usize>> = HashMap::new();
    let drain_capacity = capacities[num_layers - 1];
    for source in 0..drains_size {
        drain_sources_mappings.insert(source, vec![source; 1]);
    }
    let extension = assign_sources_to_vertices(
        &(drains_size..sources_size).collect(),
        &(0..drains_size).collect(),
        drain_capacity - 1,
    );
    for (key, value) in extension {
        drain_sources_mappings
            .entry(key)
            .or_insert(Vec::new())
            .extend(value);
    }

    for drain in 0..drains_size {
        let drain_id = graph.add_vertex_to_layer(num_layers - 1, Vertex::new_empty());
        let corresponding_sources = drain_sources_mappings.get(&drain).unwrap();
        k_means_recursive(
            &mut graph,
            drain_id.index,
            drain_id.layer - 1,
            corresponding_sources,
            capacities,
        );
    }

    return graph;
}

fn k_means_recursive(
    graph: &mut LayeredGraph,
    parent_index: usize,
    layer_index: usize,
    sources: &Vec<usize>,
    capacities: &Vec<usize>,
) {
    if layer_index == 0 {
        for _ in sources {
            let mut new_vertex = Vertex::new_empty();
            new_vertex.set_parent(parent_index);
            graph.add_vertex_to_layer(layer_index, new_vertex);
        }
        return;
    }

    let capacity = capacities[layer_index];
    let k = if sources.len() % capacity == 0 {
        sources.len() / capacity
    } else {
        sources.len() / capacity + 1
    };
    let mut new_vertices = (0..k).map(|_| Vertex::new_empty()).collect::<Vec<Vertex>>();
    new_vertices
        .iter_mut()
        .for_each(|vertex| vertex.set_parent(parent_index));
    let mut new_vertices_indexes = Vec::new();
    for vertex in new_vertices {
        new_vertices_indexes.push(graph.add_vertex_to_layer(layer_index, vertex).index);
    }

    let vertex_sources_mapping =
        assign_sources_to_vertices(sources, &new_vertices_indexes, capacity);

    for (vertex_index, sources) in vertex_sources_mapping {
        k_means_recursive(graph, vertex_index, layer_index - 1, &sources, capacities);
    }
    //TODO implement k-means
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
