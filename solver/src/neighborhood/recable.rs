use crate::{LayeredGraph, VertexID};

impl LayeredGraph {
    pub fn recable(&mut self, node: &VertexID, target_node: &VertexID) {
        self.remove_edge(&node);
        self.add_edge(&node, &target_node);
    }

    pub fn undo_recable(&mut self, node: &VertexID, old_parent_node: &VertexID) {
        self.recable(node, old_parent_node);
    }

    pub fn check_recable_possible(
        &self,
        node: &VertexID,
        target_node: &VertexID,
        capacities: &Vec<usize>,
        current_vertex_flows: &Vec<Vec<usize>>,
    ) -> bool {
        // node and target_node must be valid in the graph and target_node must be in the following layer of the node
        let old_parent_id = self.get_parent(&node).unwrap();
        let old_parent = self.get_vertex(&old_parent_id);

        if &old_parent_id == target_node {
            return false;
        }

        if old_parent.children_indices.as_ref().unwrap().len() == 1 {
            return false;
        }

        let node_flow = current_vertex_flows[node.layer][node.index];

        if current_vertex_flows[target_node.layer][target_node.index] + node_flow > capacities[target_node.layer] {
            return false;
        }

        let mut current_layer = target_node.layer + 1;
        let mut current_vertex = self.get_vertex(&target_node);
        let mut current_initial_vertex = old_parent;

        while current_vertex.parent_index.is_some() {
            let parent_id = VertexID::new(current_layer, current_vertex.parent_index.unwrap());
            let initial_parent_id = VertexID::new(current_layer, current_initial_vertex.parent_index.unwrap());

            if parent_id.index == initial_parent_id.index {
                break;
            }

            if current_vertex_flows[parent_id.layer][parent_id.index] + node_flow > capacities[parent_id.layer] {
                return false;
            }

            current_vertex = self.get_vertex(&parent_id);
            current_initial_vertex = self.get_vertex(&initial_parent_id);

            current_layer += 1;
        }

        return true;
    }
}
