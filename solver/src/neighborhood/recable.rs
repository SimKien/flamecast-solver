use crate::{LayeredGraph, Vertex};

impl LayeredGraph {
    pub fn recable(&mut self, layer_index: usize, node: Vertex, target_node: Vertex) {
        self.layers[layer_index]
            .edges
            .entry(node)
            .and_modify(|e| e.1 = target_node);
    }
}
