use kmeans::{EuclideanDistance, KMeans, KMeansConfig};

use crate::{
    neighborhood::{Neighbor, NeighborCost},
    NeighborLoader, EmbeddingOptions, VertexEmbeddings, VertexID,
};

use super::FlamecastInstance;

impl FlamecastInstance {
    pub fn get_neighbor_cost(
        &mut self,
        neighbor: &Neighbor,
        neighbor_cost_options: &EmbeddingOptions,
    ) -> f64 {
        // calculates the objective function value of the neighbor
        let mut neighbor_loader = NeighborLoader::new();

        neighbor_loader.load_neighbor(&mut self.current_solution.base_graph, neighbor);
        let new_objective_value = self.calculate_objective_function_value(neighbor_cost_options);
        neighbor_loader.unload_neighbor(&mut self.current_solution.base_graph, neighbor);
        return new_objective_value;
    }

    pub fn get_candidate_neighbors_cost(
        &mut self,
        neighbor_test_options: &EmbeddingOptions,
    ) -> Vec<NeighborCost> {
        let possible_neighbors = self.get_possible_neighbors();
        let mut result = Vec::new();

        let mut neighbor_loader = NeighborLoader::new();
        possible_neighbors.iter().for_each(|neighbor| {
            neighbor_loader.load_neighbor(&mut self.current_solution.base_graph, neighbor);
            let cost = self.calculate_objective_function_value(neighbor_test_options);
            neighbor_loader.unload_neighbor(&mut self.current_solution.base_graph, neighbor);
            result.push(NeighborCost::new(neighbor.clone(), cost));
        });

        return result;
    }

    pub fn get_possible_neighbors(&self) -> Vec<Neighbor> {
        let current_vertex_flows = self.current_solution.base_graph.calculate_vertex_flows();

        let mut result = self.get_possible_recablings(&current_vertex_flows);
        result.append(&mut self.get_possible_swaps(&current_vertex_flows));
        result.append(&mut self.get_possible_merges(&current_vertex_flows));
        result.append(&mut self.get_possible_splits());
        return result;
    }

    pub fn get_possible_recablings(&self, current_vertex_flows: &Vec<Vec<usize>>) -> Vec<Neighbor> {
        let mut possible_recablings = Vec::new();

        let num_layers = self.current_solution.base_graph.layers.len();
        for layer in 0..num_layers - 1 {
            for node in 0..self.current_solution.base_graph.layers[layer]
                .vertices
                .len()
            {
                let node_id = VertexID::new(layer, node);

                for target_node in 0..self.current_solution.base_graph.layers[layer + 1]
                    .vertices
                    .len()
                {
                    let target_node_id = VertexID::new(layer + 1, target_node);

                    if self.current_solution.base_graph.check_recable_possible(
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

    pub fn get_possible_swaps(&self, current_vertex_flows: &Vec<Vec<usize>>) -> Vec<Neighbor> {
        let mut possible_swaps = Vec::new();

        let num_layers = self.current_solution.base_graph.layers.len();
        for layer in 0..num_layers - 1 {
            for node1_index in 0..self.current_solution.base_graph.layers[layer]
                .vertices
                .len()
            {
                let node1_id = VertexID::new(layer, node1_index);

                for node2_index in (node1_index + 1)
                    ..self.current_solution.base_graph.layers[layer]
                        .vertices
                        .len()
                {
                    let node2_id = VertexID::new(layer, node2_index);

                    if self.current_solution.base_graph.check_swap_possible(
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

    pub fn get_possible_merges(&self, current_vertex_flows: &Vec<Vec<usize>>) -> Vec<Neighbor> {
        let mut possible_merges = Vec::new();

        let num_layers = self.current_solution.base_graph.layers.len();
        for layer in 1..num_layers - 1 {
            for node1_index in 0..self.current_solution.base_graph.layers[layer]
                .vertices
                .len()
            {
                let node1_id = VertexID::new(layer, node1_index);

                for node2_index in (node1_index + 1)
                    ..self.current_solution.base_graph.layers[layer]
                        .vertices
                        .len()
                {
                    let node2_id = VertexID::new(layer, node2_index);

                    if self.current_solution.base_graph.check_merge_possible(
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

    pub fn get_possible_splits(&self) -> Vec<Neighbor> {
        let mut possible_splits = Vec::new();

        let num_layers = self.current_solution.base_graph.layers.len();
        for layer in 1..num_layers - 1 {
            for parent in 0..self.current_solution.base_graph.layers[layer]
                .vertices
                .len()
            {
                let parent_id = VertexID::new(layer, parent);

                let children = self
                    .current_solution
                    .base_graph
                    .get_children(&parent_id)
                    .unwrap();
                if children.len() > 1 {
                    let (_, cluster2) =
                        cluster_children(&children, &self.current_solution.vertices_embeddings);
                    if self
                        .current_solution
                        .base_graph
                        .check_split_possible(&cluster2)
                    {
                        possible_splits.push(Neighbor::Split(cluster2));
                    }
                }
            }
        }

        return possible_splits;
    }
}

fn cluster_children(
    children: &Vec<VertexID>,
    embeddings: &VertexEmbeddings,
) -> (Vec<VertexID>, Vec<VertexID>) {
    // Use kmeans to cluster children into two clusters
    let sample_dimension = 2;
    let max_iter = 40;
    let mut samples = vec![0.0; sample_dimension * children.len()];

    children.iter().enumerate().for_each(|(i, child)| {
        samples[2 * i] = embeddings.embeddings[child.layer][child.index].0;
        samples[2 * i + 1] = embeddings.embeddings[child.layer][child.index].1;
    });

    let kmeans: KMeans<f64, 1, EuclideanDistance> =
        KMeans::new(samples, children.len(), sample_dimension, EuclideanDistance);
    let result = kmeans.kmeans_lloyd(
        2,
        max_iter,
        KMeans::init_kmeanplusplus,
        &KMeansConfig::default(),
    );
    let mut cluster1 = Vec::new();
    let mut cluster2 = Vec::new();
    for (i, child) in children.iter().enumerate() {
        if result.assignments[i] == 0 {
            cluster1.push(child.clone());
        } else {
            cluster2.push(child.clone());
        }
    }

    if cluster1.len() == 0 {
        cluster1.push(cluster2.pop().unwrap());
    } else if cluster2.len() == 0 {
        cluster2.push(cluster1.pop().unwrap());
    }

    return (cluster1, cluster2);
}
