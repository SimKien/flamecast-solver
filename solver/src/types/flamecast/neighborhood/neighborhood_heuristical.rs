use crate::{
    graph_embedding::embed_directed_graph, neighborhood::NeighborCost, EmbeddingOptions,
    FlamecastInstance, LayeredGraph, Neighbor, Vertex, VertexEmbedding, VertexEmbeddings, VertexID,
};

use super::cluster_children;

const NUM_RANDOM_VERTICES: usize = 25;
const FIXED_NUMBER_RANDOM_VERTICES_THRESHOLD: usize = 600;

impl FlamecastInstance {
    pub fn get_heuristical_candidate_neighbors_cost(
        &self,
        neighbor_test_options: &EmbeddingOptions,
    ) -> Vec<NeighborCost> {
        let base_graph = &self.solution_state.current_solution.base_graph;

        // pick sqrt(n) amount of random vertices for neighborhood calculation if under threshold
        let num_vertices = base_graph.get_number_of_vertices();
        let num_random_vertices = if num_vertices < FIXED_NUMBER_RANDOM_VERTICES_THRESHOLD {
            f64::sqrt(num_vertices as f64) as usize
        } else {
            NUM_RANDOM_VERTICES
        };

        // generate a set of random vertices which are used to search for neighbors
        let random_vertices = base_graph.get_sorted_random_vertices(num_random_vertices);

        let current_costs = self
            .solution_state
            .current_solution
            .calculate_costs(self.alpha);
        let current_vertex_flows = base_graph.calculate_vertex_flows();

        let mut result =
            self.get_heuristical_recablings(&random_vertices, current_costs, &current_vertex_flows);
        result.append(&mut self.get_heuristical_swaps(
            &random_vertices,
            current_costs,
            &current_vertex_flows,
        ));
        result.append(&mut self.get_heuristical_merges(
            &random_vertices,
            current_costs,
            &current_vertex_flows,
            neighbor_test_options,
        ));
        result.append(&mut self.get_heuristical_splits(
            &random_vertices,
            current_costs,
            &current_vertex_flows,
            neighbor_test_options,
        ));

        return result;
    }

    pub fn get_heuristical_recablings(
        &self,
        vertices: &Vec<VertexID>,
        current_costs: f64,
        current_vertex_flows: &Vec<Vec<usize>>,
    ) -> Vec<NeighborCost> {
        let base_graph = &self.solution_state.current_solution.base_graph;
        let num_layers = base_graph.layers.len();

        let embeddings = &self
            .solution_state
            .current_solution
            .vertices_embeddings
            .embeddings;

        let mut result = Vec::new();

        for vertex in vertices {
            let old_parent = base_graph.get_parent(vertex).unwrap();
            let old_parent_edge_costs = get_edge_costs(
                vertex,
                &old_parent,
                current_vertex_flows[vertex.layer][vertex.index],
                embeddings,
                self.alpha,
            );
            let costs_without_edge = current_costs - old_parent_edge_costs;

            for target_index in 0..base_graph.layers[vertex.layer + 1].vertices.len() {
                let target_node_id = VertexID::new(vertex.layer + 1, target_index);

                if base_graph.check_recable_possible(
                    vertex,
                    num_layers,
                    &target_node_id,
                    &self.capacities,
                    current_vertex_flows,
                ) {
                    let new_costs = costs_without_edge
                        + get_edge_costs(
                            vertex,
                            &target_node_id,
                            current_vertex_flows[vertex.layer][vertex.index],
                            embeddings,
                            self.alpha,
                        );
                    let delta_flow = -(current_vertex_flows[vertex.layer][vertex.index] as i32);
                    let new_costs = self.update_path_costs(
                        current_vertex_flows,
                        delta_flow,
                        &old_parent,
                        &target_node_id,
                        new_costs,
                    );

                    let neighbor = Neighbor::Recable(vertex.clone(), target_node_id);
                    result.push(NeighborCost::new(neighbor, new_costs));
                }
            }
        }

        return result;
    }

    pub fn get_heuristical_swaps(
        &self,
        vertices: &Vec<VertexID>,
        current_costs: f64,
        current_vertex_flows: &Vec<Vec<usize>>,
    ) -> Vec<NeighborCost> {
        let base_graph = &self.solution_state.current_solution.base_graph;
        let embeddings = &self
            .solution_state
            .current_solution
            .vertices_embeddings
            .embeddings;

        let mut current_layer = 0;
        let mut processed_vertices = vec![];
        let mut result = Vec::new();

        for vertex in vertices {
            if vertex.layer != current_layer {
                current_layer = vertex.layer;
                processed_vertices.clear();
            }

            let parent1 = base_graph.get_parent(vertex).unwrap();
            let edge_costs1 = get_edge_costs(
                vertex,
                &parent1,
                current_vertex_flows[vertex.layer][vertex.index],
                embeddings,
                self.alpha,
            );
            let costs_without_edge1 = current_costs - edge_costs1;

            for node2_index in 0..base_graph.layers[vertex.layer].vertices.len() {
                let node2_id = VertexID::new(vertex.layer, node2_index);

                if processed_vertices.binary_search(&node2_index).is_ok() {
                    continue;
                }

                if base_graph.check_swap_possible(
                    vertex,
                    &node2_id,
                    &self.capacities,
                    current_vertex_flows,
                ) {
                    let parent2 = base_graph.get_parent(&node2_id).unwrap();

                    let new_costs = costs_without_edge1
                        - get_edge_costs(
                            &node2_id,
                            &parent2,
                            current_vertex_flows[node2_id.layer][node2_id.index],
                            embeddings,
                            self.alpha,
                        )
                        + get_edge_costs(
                            vertex,
                            &parent2,
                            current_vertex_flows[vertex.layer][vertex.index],
                            embeddings,
                            self.alpha,
                        )
                        + get_edge_costs(
                            &node2_id,
                            &parent1,
                            current_vertex_flows[node2_id.layer][node2_id.index],
                            embeddings,
                            self.alpha,
                        );
                    let delta_flow = current_vertex_flows[node2_id.layer][node2_id.index] as i32
                        - current_vertex_flows[vertex.layer][vertex.index] as i32;
                    let new_costs = self.update_path_costs(
                        current_vertex_flows,
                        delta_flow,
                        &parent1,
                        &parent2,
                        new_costs,
                    );

                    let neighbor = Neighbor::Swap(vertex.clone(), node2_id);
                    result.push(NeighborCost::new(neighbor, new_costs));
                }
            }

            processed_vertices.push(vertex.index);
        }

        return result;
    }

    pub fn get_heuristical_merges(
        &self,
        vertices: &Vec<VertexID>,
        current_costs: f64,
        current_vertex_flows: &Vec<Vec<usize>>,
        neighbor_test_options: &EmbeddingOptions,
    ) -> Vec<NeighborCost> {
        let base_graph = &self.solution_state.current_solution.base_graph;
        let embeddings = &self
            .solution_state
            .current_solution
            .vertices_embeddings
            .embeddings;

        let mut current_layer = 0;
        let mut processed_vertices = vec![];
        let mut result = Vec::new();

        for vertex in vertices {
            if vertex.layer == 0 {
                continue;
            }

            if vertex.layer != current_layer {
                current_layer = vertex.layer;
                processed_vertices.clear();
            }

            let vertex_surrounding_costs = get_surrounding_costs(
                vertex,
                current_vertex_flows,
                base_graph,
                embeddings,
                self.alpha,
            );
            let costs_without_vertex_surrounding = current_costs - vertex_surrounding_costs;

            let children1 = base_graph.get_children(vertex).unwrap();
            let children1_size = children1.len();
            let parent = base_graph.get_parent(vertex).unwrap();

            let mut merged_graph = create_merged_base_graph(children1_size);
            let mut sources_drains_embeddings =
                create_sources_drains_embeddings(&children1, &parent, embeddings);

            for node2_index in 0..base_graph.layers[vertex.layer].vertices.len() {
                let node2_id = VertexID::new(vertex.layer, node2_index);

                if processed_vertices.binary_search(&node2_index).is_ok() {
                    continue;
                }

                if base_graph.check_merge_possible(
                    vertex,
                    &node2_id,
                    &self.capacities,
                    current_vertex_flows,
                ) {
                    let children2 = base_graph.get_children(&node2_id).unwrap();
                    let children2_size = children2.len();

                    append_merged_base_graph(&mut merged_graph, children1_size, children2_size);
                    sources_drains_embeddings.append(
                        0,
                        &mut children2
                            .iter()
                            .map(|child| embeddings[child.layer][child.index])
                            .collect(),
                    );

                    let mut children_flows = get_vertex_flows(current_vertex_flows, &children1);
                    children_flows.append(&mut get_vertex_flows(current_vertex_flows, &children2));
                    let graph_vertex_flows = calculate_vertex_flows(&merged_graph, &children_flows);

                    let merged_embeddings = embed_directed_graph(
                        &merged_graph,
                        &sources_drains_embeddings,
                        &get_edge_flows(&graph_vertex_flows),
                        self.alpha,
                        neighbor_test_options,
                    );

                    let new_costs = costs_without_vertex_surrounding
                        - get_surrounding_costs(
                            &node2_id,
                            current_vertex_flows,
                            base_graph,
                            embeddings,
                            self.alpha,
                        )
                        + get_surrounding_costs(
                            &VertexID::new(1, 0),
                            &graph_vertex_flows,
                            &merged_graph,
                            &merged_embeddings.embeddings,
                            self.alpha,
                        );

                    reset_merged_base_graph(&mut merged_graph, children1_size);
                    sources_drains_embeddings.truncate(0, children1_size);

                    let neighbor = Neighbor::Merge(vertex.clone(), node2_id);
                    result.push(NeighborCost::new(neighbor, new_costs));
                }
            }

            processed_vertices.push(vertex.index);
        }

        return result;
    }

    pub fn get_heuristical_splits(
        &self,
        vertices: &Vec<VertexID>,
        current_costs: f64,
        current_vertex_flows: &Vec<Vec<usize>>,
        neighbor_test_options: &EmbeddingOptions,
    ) -> Vec<NeighborCost> {
        let base_graph = &self.solution_state.current_solution.base_graph;
        let embeddings = &self
            .solution_state
            .current_solution
            .vertices_embeddings
            .embeddings;

        let mut result = Vec::new();

        for vertex in vertices {
            if vertex.layer == 0 {
                continue;
            }

            let children = base_graph.get_children(vertex).unwrap();
            let parent = base_graph.get_parent(vertex).unwrap();

            if children.len() > 1 {
                let (cluster1, cluster2) = cluster_children(
                    &children,
                    &self.solution_state.current_solution.vertices_embeddings,
                );
                if base_graph.check_split_possible(&cluster2) {
                    let graph = create_split_base_graph(cluster1.len(), cluster2.len());
                    let mut children_sorted = cluster1;
                    children_sorted.append(&mut cluster2.clone());
                    let sources_drains_embeddings =
                        create_sources_drains_embeddings(&children_sorted, &parent, embeddings);

                    let graph_vertex_flows = calculate_vertex_flows(
                        &graph,
                        &get_vertex_flows(current_vertex_flows, &children_sorted),
                    );
                    let split_embeddings = embed_directed_graph(
                        &graph,
                        &sources_drains_embeddings,
                        &get_edge_flows(&graph_vertex_flows),
                        self.alpha,
                        neighbor_test_options,
                    );

                    let new_costs = current_costs
                        - get_surrounding_costs(
                            vertex,
                            current_vertex_flows,
                            base_graph,
                            embeddings,
                            self.alpha,
                        )
                        + get_surrounding_costs(
                            &VertexID::new(1, 0),
                            &graph_vertex_flows,
                            &graph,
                            &split_embeddings.embeddings,
                            self.alpha,
                        )
                        + get_surrounding_costs(
                            &VertexID::new(1, 1),
                            &graph_vertex_flows,
                            &graph,
                            &split_embeddings.embeddings,
                            self.alpha,
                        );

                    let neighbor = Neighbor::Split(cluster2);
                    result.push(NeighborCost::new(neighbor, new_costs));
                }
            }
        }

        return result;
    }

    pub fn update_path_costs(
        &self,
        current_vertex_flows: &Vec<Vec<usize>>,
        delta_flow: i32,
        start1: &VertexID,
        start2: &VertexID,
        current_costs: f64,
    ) -> f64 {
        let graph = &self.solution_state.current_solution.base_graph;
        let embeddings = &self
            .solution_state
            .current_solution
            .vertices_embeddings
            .embeddings;

        let mut current1 = start1.clone();
        let mut current2 = start2.clone();

        let mut parent1 = graph.get_parent(&current1);
        let mut parent2 = graph.get_parent(&current2);

        let mut current_costs = current_costs;
        while (current1 != current2) && parent1.is_some() {
            let par1 = parent1.unwrap();
            let par2 = parent2.unwrap();

            current_costs -= get_edge_costs(
                &current1,
                &par1,
                current_vertex_flows[current1.layer][current1.index],
                embeddings,
                self.alpha,
            ) + get_edge_costs(
                &current2,
                &par2,
                current_vertex_flows[current2.layer][current2.index],
                embeddings,
                self.alpha,
            );

            current_costs += get_edge_costs(
                &current1,
                &par1,
                (current_vertex_flows[current1.layer][current1.index] as i32 + delta_flow) as usize,
                embeddings,
                self.alpha,
            ) + get_edge_costs(
                &current2,
                &par2,
                (current_vertex_flows[current2.layer][current2.index] as i32 - delta_flow) as usize,
                embeddings,
                self.alpha,
            );

            current1 = par1;
            current2 = par2;
            parent1 = graph.get_parent(&current1);
            parent2 = graph.get_parent(&current2);
        }

        return current_costs;
    }
}

fn get_edge_flows(current_vertex_flows: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut vertex_flows = current_vertex_flows.clone();

    vertex_flows.pop();

    return vertex_flows;
}

fn get_vertex_flows(
    current_vertex_flows: &Vec<Vec<usize>>,
    vertices: &Vec<VertexID>,
) -> Vec<usize> {
    return vertices
        .iter()
        .map(|vertex| current_vertex_flows[vertex.layer][vertex.index])
        .collect();
}

fn create_split_base_graph(children1_size: usize, children2_size: usize) -> LayeredGraph {
    let mut graph = LayeredGraph::new_with_size(3);
    (0..children1_size).into_iter().for_each(|_| {
        graph.layers[0].add_vertex(Vertex::new(Some(0), None));
    });
    (0..children2_size).into_iter().for_each(|_| {
        graph.layers[0].add_vertex(Vertex::new(Some(1), None));
    });
    graph.layers[1].add_vertex(Vertex::new(Some(0), Some((0..children1_size).collect())));
    graph.layers[1].add_vertex(Vertex::new(
        Some(0),
        Some((children1_size..children1_size + children2_size).collect()),
    ));
    graph.layers[2].add_vertex(Vertex::new(None, Some(vec![0, 1])));

    return graph;
}

fn calculate_vertex_flows(graph: &LayeredGraph, children_flows: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut result = vec![vec![]; 3];

    result[0] = children_flows.clone();

    for parent in 0..graph.layers[1].vertices.len() {
        let children = graph.get_children(&VertexID::new(1, parent)).unwrap();

        let mut flow = 0;

        children.iter().for_each(|child| {
            flow += children_flows[child.index];
        });

        result[1].push(flow);
    }

    let parent_flow_sum = result[1].iter().sum();
    result[2].push(parent_flow_sum);

    return result;
}

fn create_sources_drains_embeddings(
    children: &Vec<VertexID>,
    parent: &VertexID,
    embeddings: &Vec<Vec<VertexEmbedding>>,
) -> VertexEmbeddings {
    let mut sources_drains_embeddings = VertexEmbeddings::new_with_size(3);
    children.iter().for_each(|child| {
        sources_drains_embeddings.embeddings[0].push(embeddings[child.layer][child.index]);
    });
    sources_drains_embeddings.embeddings[2].push(embeddings[parent.layer][parent.index]);

    return sources_drains_embeddings;
}

fn reset_merged_base_graph(base_graph: &mut LayeredGraph, children1_size: usize) {
    base_graph.layers[0].vertices.truncate(children1_size);
    base_graph.layers[1].vertices[0].truncate_children(children1_size);
}

fn append_merged_base_graph(
    base_graph: &mut LayeredGraph,
    children1_size: usize,
    children2_size: usize,
) {
    (0..children2_size).into_iter().for_each(|_| {
        base_graph.layers[0].add_vertex(Vertex::new(Some(0), None));
    });
    base_graph.layers[1].vertices[0]
        .append_children(&mut (children1_size..children1_size + children2_size).collect());
}

fn create_merged_base_graph(num_children: usize) -> LayeredGraph {
    let mut graph = LayeredGraph::new_with_size(3);
    (0..num_children).into_iter().for_each(|_| {
        graph.layers[0].add_vertex(Vertex::new(Some(0), None));
    });
    graph.layers[1].add_vertex(Vertex::new(Some(0), Some((0..num_children).collect())));
    graph.layers[2].add_vertex(Vertex::new(None, Some(vec![0])));

    return graph;
}

fn get_surrounding_costs(
    vertex: &VertexID,
    current_vertex_flows: &Vec<Vec<usize>>,
    graph: &LayeredGraph,
    embeddings: &Vec<Vec<VertexEmbedding>>,
    alpha: f64,
) -> f64 {
    // assumes that the vertex has a parent and children
    let mut costs = 0.0;

    let children = graph.get_children(vertex).unwrap();
    let parent = graph.get_parent(vertex).unwrap();

    let vertex_embedding = embeddings[vertex.layer][vertex.index];
    children.iter().for_each(|child| {
        let child_embedding = embeddings[child.layer][child.index];
        let len = ((vertex_embedding.0 - child_embedding.0).powi(2)
            + (vertex_embedding.1 - child_embedding.1).powi(2))
        .sqrt();
        costs += len * (current_vertex_flows[child.layer][child.index] as f64).powf(alpha);
    });
    let parent_embedding = embeddings[parent.layer][parent.index];
    let len = ((parent_embedding.0 - vertex_embedding.0).powi(2)
        + (parent_embedding.1 - vertex_embedding.1).powi(2))
    .sqrt();
    costs += len * (current_vertex_flows[vertex.layer][vertex.index] as f64).powf(alpha);

    return costs;
}

fn get_edge_costs(
    start_vertex: &VertexID,
    end_vertex: &VertexID,
    flow: usize,
    embeddings: &Vec<Vec<VertexEmbedding>>,
    alpha: f64,
) -> f64 {
    let start_position = embeddings[start_vertex.layer][start_vertex.index];
    let end_position = embeddings[end_vertex.layer][end_vertex.index];
    let len = ((end_position.0 - start_position.0).powi(2)
        + (end_position.1 - start_position.1).powi(2))
    .sqrt();
    return len * (flow as f64).powf(alpha);
}
