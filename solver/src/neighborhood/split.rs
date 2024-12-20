use crate::{LayeredGraph, Vertex, VertexID};

impl LayeredGraph {
    pub fn split(&mut self, children_new_parent_vertex: &Vec<VertexID>) {
        let old_parent_vertex = self.get_parent(&children_new_parent_vertex[0]).unwrap();
        let grand_parent_vertex = self.get_parent(&old_parent_vertex).unwrap();

        let mut new_parent_vertex = Vertex::new_empty();
        new_parent_vertex.set_parent(grand_parent_vertex.index);
        let new_parent_vertex_id =
            self.add_vertex_to_layer(old_parent_vertex.layer, new_parent_vertex);

        children_new_parent_vertex.iter().for_each(|child_vertex| {
            self.recable(child_vertex, &new_parent_vertex_id);
        });
    }
}
