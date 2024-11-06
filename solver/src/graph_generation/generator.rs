use rand::Rng;

use crate::DirectedGraph;

pub fn generate_random_directed_graph(num_nodes: usize, num_layers: usize) -> DirectedGraph {
    let mut rng = rand::thread_rng();

    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    // Generate random layer sizes
    let mut layer_sizes = Vec::new();
    let mut remaining_num_nodes = num_nodes;
    for i in 0..(num_layers - 1) {
        let num_nodes_in_layer = rng.gen_range(1..=(remaining_num_nodes - (num_layers - i - 1)));
        layer_sizes.push(num_nodes_in_layer);
        remaining_num_nodes -= num_nodes_in_layer;
    }
    layer_sizes.push(remaining_num_nodes);
    layer_sizes.sort();
    layer_sizes.reverse();

    // assign vertices to layers and fill vertices vector
    let mut layers = Vec::new();
    let mut current_layer = Vec::new();
    let mut current_layer_index = 0;
    for i in 0..num_nodes {
        vertices.push(i);
        current_layer.push(i);
        if current_layer.len() == layer_sizes[current_layer_index] {
            layers.push(current_layer);
            current_layer = Vec::new();
            current_layer_index += 1;
        }
    }

    // generate random edges between following layers
    for i in 0..(num_layers - 1) {
        let layer1 = &layers[i];
        let layer2 = &layers[i + 1];
        for i in 0..layer2.len() {
            edges.push((layer1[i], layer2[i]));
        }
        for i in layer2.len()..layer1.len() {
            let random_vertex = rng.gen_range(0..layer2.len());
            edges.push((layer1[i], layer2[random_vertex]));
        }
    }

    DirectedGraph::new(vertices, edges)
}
