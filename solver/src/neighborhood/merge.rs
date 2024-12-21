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

    pub fn check_merge_possible(
        &self,
        parent1: &VertexID,
        parent2: &VertexID,
        capacities: &Vec<usize>,
    ) -> bool {
        if self.get_parent(parent1) != self.get_parent(parent2) {
            return false;
        }

        //TODO: check von parent_layer aufwärts ob kapazität erfüllt ist

        return true;
    }
}
