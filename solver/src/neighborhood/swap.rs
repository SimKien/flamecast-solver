use crate::LayeredGraph;

impl LayeredGraph {
    pub fn swap(&mut self, layer_index: usize, node1: usize, node2: usize) {
        self.layers[layer_index].edges.iter_mut().for_each(|edge| {
            if edge.0 == node1 {
                edge.0 = node2;
            } else if edge.0 == node2 {
                edge.0 = node1;
            }
        });
    }
}
