use crate::{LayeredGraph, VertexID};

impl LayeredGraph {
    // Merging only possible if the two nodes have the same parent
    pub fn merge(&mut self, parent1: &VertexID, parent2: &VertexID) {
        self.get_children(parent2)
            .unwrap()
            .iter()
            .for_each(|child| {
                self.recable(child, &parent1);
            });

        self.remove_edge(parent2);
        self.remove_vertex(parent2);
    }
}
