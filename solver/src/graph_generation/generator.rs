use rand::Rng;

use crate::{Layer, LayeredGraph, Vertex};

pub fn generate_random_directed_graph(num_nodes: usize, num_layers: usize) -> LayeredGraph {
    // Assertion: num_nodes > 2 * num_layers + 2

    let mut rng = rand::thread_rng();

    // Generate random layers in descending order of size
    let mut split_indexes = Vec::new();
    let half_nodes = num_nodes / 2;
    for _ in 0..(num_layers - 1) {
        let mut split_index = rng.gen_range(1..num_nodes);
        if !split_indexes.contains(&split_index) {
            split_indexes.push(split_index);
            continue;
        }
        if split_index < half_nodes {
            while split_indexes.contains(&split_index) {
                split_index += 1;
            }
        } else {
            while split_indexes.contains(&split_index) {
                split_index -= 1;
            }
        }
        split_indexes.push(split_index);
    }
    split_indexes.sort();

    let mut layers = Vec::new();
    let mut current_split_index = 0;
    for index in split_indexes {
        let layer = (current_split_index..index).collect::<Vec<Vertex>>();
        layers.push(Layer::new(layer, Vec::new()));
        current_split_index = index;
    }
    layers.push(Layer::new(
        (current_split_index..num_nodes).collect::<Vec<usize>>(),
        Vec::new(),
    ));
    layers.sort_by(|a, b| b.vertices.len().cmp(&a.vertices.len()));

    // generate random edges between following layers
    for i in 0..(num_layers - 1) {
        let mut new_edges = Vec::new();
        let layer1 = &layers[i];
        let layer2 = &layers[i + 1];
        for i in 0..layer2.vertices.len() {
            new_edges.push((layer1.vertices[i], layer2.vertices[i]));
        }
        for i in layer2.vertices.len()..layer1.vertices.len() {
            let random_vertex = rng.gen_range(0..layer2.vertices.len());
            new_edges.push((layer1.vertices[i], layer2.vertices[random_vertex]));
        }
        layers[i].edges = new_edges;
    }

    LayeredGraph::new(layers, num_nodes, Vec::new())
}
