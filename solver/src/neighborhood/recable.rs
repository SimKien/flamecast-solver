use crate::{LayeredGraph, VertexID};

impl LayeredGraph {
    pub fn recable(&mut self, node: &VertexID, target_node: &VertexID) {
        self.remove_edge(&node);
        self.add_edge(&node, &target_node);
    }

    pub fn check_recable_possible(
        &mut self,
        node: &VertexID,
        target_node: &VertexID,
        capacities: &Vec<usize>,
    ) -> bool {
        // node and target_node must be valid in the graph and target_node must be in the following layer of the node
        let child = self.get_vertex(node);
        if child.parent_index.unwrap() == target_node.index {
            return false;
        }
        let old_parent = self.get_parent(node).unwrap();
        self.recable(node, target_node);

        //TODO: Checke nicht ganzen graphen, sondern nur ob alle vertices besucht werden und ob von neuem parent aufwärts kapazität erfüllt ist
        let valid = self.is_valid_flamecast_topology(capacities);

        self.recable(node, &old_parent);
        return valid;
    }
}
