use crate::{LayeredGraph, Vertex};

impl LayeredGraph {
    // Merging only possible if the two nodes have the same parent
    pub fn merge(&mut self, layer_index: usize, parent1: Vertex, parent2: Vertex) {
        let grand_parent = self.get_parent(layer_index + 1, parent1);

        self.get_children(layer_index, parent2)
            .iter()
            .for_each(|&child| {
                self.recable(layer_index, child, parent1);
            });

        self.remove_edge_from_layer((parent2, grand_parent), layer_index + 1);
        self.remove_vertex_from_layer(parent2, layer_index + 1);
    }
}
