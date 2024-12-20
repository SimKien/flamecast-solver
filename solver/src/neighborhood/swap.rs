use crate::{LayeredGraph, VertexID};

impl LayeredGraph {
    pub fn swap(&mut self, node1: &VertexID, node2: &VertexID) {
        let new_target1 = self.get_parent(&node2).unwrap();
        let new_target2 = self.get_parent(&node1).unwrap();
        self.recable(&node1, &new_target1);
        self.recable(&node2, &new_target2);
    }
}
