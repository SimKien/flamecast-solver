use rand::Rng;

use crate::{Layer, LayeredGraph};

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

    let mut layer_sizes = Vec::new();
    let mut current_split_index = 0;
    for index in split_indexes {
        layer_sizes.push(index - current_split_index);
        current_split_index = index;
    }
    layer_sizes.push(num_nodes - current_split_index);
    layer_sizes.sort_by(|a, b| b.cmp(a));

    let mut layers = Vec::new();
    for size in layer_sizes {
        let layer = Layer::new_with_size(size);
        layers.push(layer);
    }

    // generate random edges between following layers
    for layer_index in 0..(num_layers - 1) {
        let (first, last) = layers.split_at_mut(layer_index + 1);

        let layer1 = &mut first[layer_index];
        let layer2 = &mut last[0];
        for i in 0..layer2.vertices.len() {
            layer1.vertices[i].set_parent(Some(i));
            layer2.vertices[i].add_child(i);
        }
        for i in layer2.vertices.len()..layer1.vertices.len() {
            let random_vertex = rng.gen_range(0..layer2.vertices.len());
            layer1.vertices[i].set_parent(Some(random_vertex));
            layer2.vertices[random_vertex].add_child(i);
        }
    }

    return LayeredGraph::from(layers);
}
