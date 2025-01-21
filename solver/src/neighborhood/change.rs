use crate::{LayeredGraph, VertexID};

use super::Neighbor;

impl LayeredGraph {
    pub fn apply_neighbor_change(&mut self, neighbor: &Neighbor) -> Option<VertexID> {
        match neighbor {
            Neighbor::Recable(node, target_node) => {
                self.recable(node, target_node);
                return None;
            },
            Neighbor::Swap(node1, node2) => {
                self.swap(node1, node2);
                return None;
            },
            Neighbor::Merge(parent1, parent2) => {
                self.merge(parent1, parent2);
                return None;
            },
            Neighbor::Split(children_new_parent) => {
                let new_parent_id = self.split(children_new_parent);
                return Some(new_parent_id);
            },
        }
    }
}