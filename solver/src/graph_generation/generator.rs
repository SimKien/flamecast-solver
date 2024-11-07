use rand::Rng;

use crate::DirectedGraph;

pub fn generate_random_directed_graph(num_nodes: usize, num_layers: usize) -> DirectedGraph {
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
        let layer = (current_split_index..index).collect::<Vec<usize>>();
        layers.push(layer);
        current_split_index = index;
    }
    layers.push((current_split_index..num_nodes).collect::<Vec<usize>>());
    layers.sort_by(|a, b| b.len().cmp(&a.len()));

    // fill vertices vector
    let vertices = (0..num_nodes).collect::<Vec<usize>>();

    // generate random edges between following layers
    let mut edges = Vec::new();
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
