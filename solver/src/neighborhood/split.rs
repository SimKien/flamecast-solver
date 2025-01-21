use crate::{LayeredGraph, Vertex, VertexID};

impl LayeredGraph {
    pub fn split(&mut self, children_new_parent_vertex: &Vec<VertexID>) -> VertexID {
        // Returns the new parent vertex id
        let old_parent_vertex = self.get_parent(&children_new_parent_vertex[0]).unwrap();
        let grand_parent_vertex = self.get_parent(&old_parent_vertex).unwrap();

        let mut new_parent_vertex = Vertex::new_empty();
        new_parent_vertex.set_parent(Some(grand_parent_vertex.index));
        let new_parent_vertex_id =
            self.add_vertex_to_layer(old_parent_vertex.layer, new_parent_vertex);

        children_new_parent_vertex.iter().for_each(|child_vertex| {
            self.recable(child_vertex, &new_parent_vertex_id);
        });

        return new_parent_vertex_id;
    }

    pub fn undo_split(&mut self, old_parent_node: &VertexID, new_parent_node: &VertexID) {
        self.merge(old_parent_node, new_parent_node);
    }

    pub fn check_split_possible(&self, children_new_parent_vertex: &Vec<VertexID>) -> bool {
        if children_new_parent_vertex.len() == 0 {
            return false;
        }

        let parent_id = self.get_parent(&children_new_parent_vertex[0]).unwrap();
        let parent = self.get_vertex(&parent_id);
        if children_new_parent_vertex.len() == parent.children_indices.as_ref().unwrap().len() {
            return false;
        }

        return true;
    }
}
