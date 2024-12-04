use crate::LayeredGraph;

impl LayeredGraph {
    pub fn merge(&mut self, layer_index: usize, remaining_node: usize, node_to_remove: usize) {
        self.layers[layer_index - 1]
            .edges
            .iter_mut()
            .for_each(|(_, v)| {
                if *v == node_to_remove {
                    *v = remaining_node;
                }
            });
        self.remove_edge_from_layer(
            (node_to_remove, self.get_parent(layer_index, node_to_remove)),
            layer_index,
        );
        self.remove_vertex_from_layer(node_to_remove, layer_index);
    }
}
