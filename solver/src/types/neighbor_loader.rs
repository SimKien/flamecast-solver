use crate::neighborhood::Neighbor;

use super::{LayeredGraph, VertexID};

#[derive(Debug)]
pub struct NeighborLoader {
    pub neighbor_loaded: bool,
    unload_informations: Vec<VertexID>,
}

impl NeighborLoader {
    pub fn new() -> Self {
        Self {
            neighbor_loaded: false,
            unload_informations: Vec::new(),
        }
    }

    pub fn load_neighbor(&mut self, base_graph: &mut LayeredGraph, neighbor: &Neighbor) {
        if self.neighbor_loaded {
            return;
        }

        match neighbor {
            Neighbor::Recable(node, _) => {
                let old_parent_node = base_graph.get_parent(node).unwrap();
                base_graph.apply_neighbor_change(neighbor);
                self.unload_informations.push(old_parent_node);
            }
            Neighbor::Swap(_, _) => {
                base_graph.apply_neighbor_change(neighbor);
            }
            Neighbor::Merge(_, parent2) => {
                let parent2_children = base_graph.get_children(parent2).unwrap();
                base_graph.apply_neighbor_change(neighbor);
                self.unload_informations = parent2_children;
            }
            Neighbor::Split(children_new_parent) => {
                let old_parent = base_graph.get_parent(&children_new_parent[0]).unwrap();
                let new_parent = base_graph.apply_neighbor_change(neighbor).unwrap();
                self.unload_informations.push(old_parent);
                self.unload_informations.push(new_parent);
            }
        }

        self.neighbor_loaded = true;
    }

    pub fn unload_neighbor(&mut self, base_graph: &mut LayeredGraph, neighbor: &Neighbor) {
        if !self.neighbor_loaded {
            return;
        }

        match neighbor {
            Neighbor::Recable(node, _) => {
                let old_parent_node = self.unload_informations.pop().unwrap();
                base_graph.undo_recable(node, &old_parent_node);
            }
            Neighbor::Swap(node1, node2) => {
                base_graph.undo_swap(node1, node2);
            }
            Neighbor::Merge(_, parent2) => {
                let parent2_children = &self.unload_informations;
                base_graph.undo_merge(parent2_children, parent2);
            }
            Neighbor::Split(_) => {
                let new_parent = self.unload_informations.pop().unwrap();
                let old_parent = self.unload_informations.pop().unwrap();
                base_graph.undo_split(&old_parent, &new_parent);
            }
        }

        self.neighbor_loaded = false;
        self.unload_informations.clear();
    }
}
