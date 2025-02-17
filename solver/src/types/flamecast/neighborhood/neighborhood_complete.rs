use crate::{
    neighborhood::{Neighbor, NeighborCost},
    EmbeddingOptions, FlamecastInstance, NeighborLoader, VertexID,
};

use super::cluster_children;

impl FlamecastInstance {
    pub fn get_all_candidate_neighbors_cost(
        &mut self,
        neighbor_test_options: &EmbeddingOptions,
    ) -> Vec<NeighborCost> {
        let possible_neighbors = self.get_all_possible_neighbors();
        let mut result = Vec::new();

        let mut neighbor_loader = NeighborLoader::new();
        possible_neighbors.iter().for_each(|neighbor| {
            neighbor_loader.load_neighbor(
                &mut self.solution_state.current_solution.base_graph,
                neighbor,
            );
            let cost = self.calculate_objective_function_value(neighbor_test_options);
            neighbor_loader.unload_neighbor(
                &mut self.solution_state.current_solution.base_graph,
                neighbor,
            );
            result.push(NeighborCost::new(neighbor.clone(), cost));
        });

        return result;
    }

    pub fn get_all_possible_neighbors(&self) -> Vec<Neighbor> {
        let current_vertex_flows = self
            .solution_state
            .current_solution
            .base_graph
            .calculate_vertex_flows();

        let mut result = self.get_all_possible_recablings(&current_vertex_flows);
        result.append(&mut self.get_all_possible_swaps(&current_vertex_flows));
        result.append(&mut self.get_all_possible_merges(&current_vertex_flows));
        result.append(&mut self.get_all_possible_splits());
        return result;
    }

    pub fn get_all_possible_recablings(
        &self,
        current_vertex_flows: &Vec<Vec<usize>>,
    ) -> Vec<Neighbor> {
        let mut possible_recablings = Vec::new();

        let base_graph = &self.solution_state.current_solution.base_graph;

        let num_layers = base_graph.layers.len();
        for layer in 0..num_layers - 1 {
            for node in 0..base_graph.layers[layer].vertices.len() {
                let node_id = VertexID::new(layer, node);

                for target_node in 0..base_graph.layers[layer + 1].vertices.len() {
                    let target_node_id = VertexID::new(layer + 1, target_node);

                    if base_graph.check_recable_possible(
                        &node_id,
                        &target_node_id,
                        &self.capacities,
                        current_vertex_flows,
                    ) {
                        possible_recablings
                            .push(Neighbor::Recable(node_id.clone(), target_node_id));
                    }
                }
            }
        }

        return possible_recablings;
    }

    pub fn get_all_possible_swaps(&self, current_vertex_flows: &Vec<Vec<usize>>) -> Vec<Neighbor> {
        let mut possible_swaps = Vec::new();

        let base_graph = &self.solution_state.current_solution.base_graph;

        let num_layers = base_graph.layers.len();
        for layer in 0..num_layers - 1 {
            for node1_index in 0..base_graph.layers[layer].vertices.len() {
                let node1_id = VertexID::new(layer, node1_index);

                for node2_index in (node1_index + 1)..base_graph.layers[layer].vertices.len() {
                    let node2_id = VertexID::new(layer, node2_index);

                    if base_graph.check_swap_possible(
                        &node1_id,
                        &node2_id,
                        &self.capacities,
                        current_vertex_flows,
                    ) {
                        possible_swaps.push(Neighbor::Swap(node1_id.clone(), node2_id));
                    }
                }
            }
        }

        return possible_swaps;
    }

    pub fn get_all_possible_merges(&self, current_vertex_flows: &Vec<Vec<usize>>) -> Vec<Neighbor> {
        let mut possible_merges = Vec::new();

        let base_graph = &self.solution_state.current_solution.base_graph;

        let num_layers = base_graph.layers.len();
        for layer in 1..num_layers - 1 {
            for node1_index in 0..base_graph.layers[layer].vertices.len() {
                let node1_id = VertexID::new(layer, node1_index);

                for node2_index in (node1_index + 1)..base_graph.layers[layer].vertices.len() {
                    let node2_id = VertexID::new(layer, node2_index);

                    if base_graph.check_merge_possible(
                        &node1_id,
                        &node2_id,
                        &self.capacities,
                        current_vertex_flows,
                    ) {
                        possible_merges.push(Neighbor::Merge(node1_id.clone(), node2_id));
                    }
                }
            }
        }

        return possible_merges;
    }

    pub fn get_all_possible_splits(&self) -> Vec<Neighbor> {
        let mut possible_splits = Vec::new();

        let base_graph = &self.solution_state.current_solution.base_graph;

        let num_layers = base_graph.layers.len();
        for layer in 1..num_layers - 1 {
            for parent in 0..base_graph.layers[layer].vertices.len() {
                let parent_id = VertexID::new(layer, parent);

                let children = base_graph.get_children(&parent_id).unwrap();
                if children.len() > 1 {
                    let (_, cluster2) = cluster_children(
                        &children,
                        &self.solution_state.current_solution.vertices_embeddings,
                    );
                    if base_graph.check_split_possible(&cluster2) {
                        possible_splits.push(Neighbor::Split(cluster2));
                    }
                }
            }
        }

        return possible_splits;
    }
}
