use crate::LayeredGraph;

impl LayeredGraph {
    pub fn recable(&mut self, layer_index: usize, node: usize, target_node: usize) {
        self.layers[layer_index]
            .edges
            .iter_mut()
            .find(|(x, _)| *x == node)
            .unwrap()
            .1 = target_node;
    }
}
