use crate::{LayeredGraph, Vertex};

impl LayeredGraph {
    pub fn split(&mut self, layer_index: usize, children_new_parent_vertex: &Vec<Vertex>) {
        let old_parent_vertex = self.get_parent(layer_index, children_new_parent_vertex[0]);
        let grand_parent_vertex = self.get_parent(layer_index + 1, old_parent_vertex);
        let new_parent_vertex = self.new_vertex();

        self.add_vertex_to_layer(new_parent_vertex, layer_index + 1);
        self.add_edge_to_layer((new_parent_vertex, grand_parent_vertex), layer_index + 1);

        children_new_parent_vertex.iter().for_each(|&child_vertex| {
            self.recable(layer_index, child_vertex, new_parent_vertex);
        });
    }
}
