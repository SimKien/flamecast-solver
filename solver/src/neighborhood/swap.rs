use crate::{LayeredGraph, Vertex};

impl LayeredGraph {
    pub fn swap(&mut self, layer_index: usize, node1: Vertex, node2: Vertex) {
        let new_target1 = self.get_parent(layer_index, node2);
        let new_target2 = self.get_parent(layer_index, node1);
        self.recable(layer_index, node1, new_target1);
        self.recable(layer_index, node2, new_target2);
    }
}
