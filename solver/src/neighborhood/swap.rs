use crate::{LayeredGraph, VertexID};

impl LayeredGraph {
    pub fn swap(&mut self, node1: &VertexID, node2: &VertexID) {
        let new_target1 = self.get_parent(&node2).unwrap();
        let new_target2 = self.get_parent(&node1).unwrap();
        self.recable(&node1, &new_target1);
        self.recable(&node2, &new_target2);
    }

    pub fn check_swap_possible(
        &mut self,
        node1: &VertexID,
        node2: &VertexID,
        capacities: &Vec<usize>,
    ) -> bool {
        self.swap(node1, node2);

        //TODO: checke nicht ganzen graphen, sondern nur von node1 und node2 aufwärts ob kapazitäten erfüllt sind
        let valid = self.is_valid_flamecast_topology_check_capacities(capacities);
        self.swap(node1, node2);
        return valid;
    }
}
