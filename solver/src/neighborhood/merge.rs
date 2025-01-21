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

    pub fn undo_merge(&mut self, old_children: &Vec<VertexID>, parent2_id: &VertexID) {
        let position = self.split(old_children);

        // So that the parent2 is at its initial position
        self.swap_vertices_position(parent2_id, &position);
    }

    pub fn check_merge_possible(
        &self,
        parent1: &VertexID,
        parent2: &VertexID,
        capacities: &Vec<usize>,
        current_vertex_flows: &Vec<Vec<usize>>,
    ) -> bool {
        if self.get_parent(parent1) != self.get_parent(parent2) {
            return false;
        }

        if parent1 == parent2 {
            return false;
        }

        if current_vertex_flows[parent1.layer][parent1.index]
            + current_vertex_flows[parent2.layer][parent2.index]
            > capacities[parent1.layer]
        {
            return false;
        }

        return true;
    }
}
