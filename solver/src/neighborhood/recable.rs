use crate::{LayeredGraph, VertexID};

impl LayeredGraph {
    pub fn recable(&mut self, node: &VertexID, target_node: &VertexID) {
        self.remove_edge(&node);
        self.add_edge(&node, &target_node);
    }
}
