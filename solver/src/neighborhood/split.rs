use crate::LayeredGraph;

impl LayeredGraph {
    pub fn split(&mut self, layer_index: usize, connections_new_node: &mut Vec<usize>) {
        let old_vertex = self.get_parent(layer_index, connections_new_node[0]);
        let parent_node = self.get_parent(layer_index, old_vertex);
        let new_vertex = self.new_vertex();
        self.add_vertex_to_layer(new_vertex, layer_index + 1);
        self.add_edge_to_layer((new_vertex, parent_node), layer_index);
        self.layers[layer_index]
            .edges
            .iter_mut()
            .for_each(|(x, v)| {
                if connections_new_node.contains(x) {
                    *v = new_vertex;
                    connections_new_node.remove(*x);
                }
            });
    }
}
