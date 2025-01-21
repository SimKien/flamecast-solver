use crate::{LayeredGraph, VertexID};

impl LayeredGraph {
    pub fn swap(&mut self, node1: &VertexID, node2: &VertexID) {
        let new_target1 = self.get_parent(&node2).unwrap();
        let new_target2 = self.get_parent(&node1).unwrap();
        self.recable(&node1, &new_target1);
        self.recable(&node2, &new_target2);
    }

    pub fn undo_swap(&mut self, node1: &VertexID, node2: &VertexID) {
        self.swap(node1, node2);
    }

    pub fn check_swap_possible(
        &self,
        node1: &VertexID,
        node2: &VertexID,
        capacities: &Vec<usize>,
        current_vertex_flows: &Vec<Vec<usize>>,
    ) -> bool {
        if node1.layer != node2.layer {
            return false;
        }

        if node1.index == node2.index {
            return false;
        }

        let flow1 = current_vertex_flows[node1.layer][node1.index];
        let flow2 = current_vertex_flows[node2.layer][node2.index];

        let mut current_layer = node1.layer + 1;
        let mut current_vertex1 = self.get_vertex(node2);
        let mut current_vertex2 = self.get_vertex(node1);

        if current_vertex1.parent_index.unwrap() == current_vertex2.parent_index.unwrap() {
            return false;
        }

        while current_vertex1.parent_index.is_some() {
            let parent_index1 = current_vertex1.parent_index.unwrap();
            let parent_index2 = current_vertex2.parent_index.unwrap();
            if parent_index1 == parent_index2 {
                break;
            }

            let flow_parent1 = current_vertex_flows[current_layer][parent_index1];
            let flow_parent2 = current_vertex_flows[current_layer][parent_index2];

            if flow_parent1 - flow2 + flow1 > capacities[current_layer] {
                return false;
            }
            if flow_parent2 - flow1 + flow2 > capacities[current_layer] {
                return false;
            }

            current_vertex1 = self.get_vertex(&VertexID::new(current_layer, parent_index1));
            current_vertex2 = self.get_vertex(&VertexID::new(current_layer, parent_index2));

            current_layer += 1;
        }

        return true;
    }
}
