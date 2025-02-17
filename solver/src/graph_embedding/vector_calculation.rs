use crate::{types::VertexEmbeddings, LayeredGraph};

pub fn calculate_q_vector(
    graph: &LayeredGraph,
    edge_flows: &Vec<Vec<usize>>,
    number_of_regarded_vertices: usize,
    number_of_edges: usize,
    alpha: f64,
) -> Vec<f64> {
    // calculate q-vector for clarabel
    let mut q = vec![0.0 as f64; 2 * number_of_regarded_vertices + number_of_edges];

    let mut edge_index = 0;
    for layer_index in 0..graph.layers.len() - 1 {
        let layer = &graph.layers[layer_index];

        for vertex_index in 0..layer.vertices.len() {
            let flow = edge_flows[layer_index][vertex_index] as f64;
            let flow_weight = flow.powf(alpha);
            q[2 * number_of_regarded_vertices + edge_index] = flow_weight;
            edge_index += 1;
        }
    }

    return q;
}

pub fn calculate_b_vector(
    graph: &LayeredGraph,
    number_of_edges: usize,
    sources_drains_embeddings: &VertexEmbeddings,
) -> Vec<f64> {
    // calculate b-vector for clarabel
    let mut b = vec![0.0; 3 * number_of_edges];

    graph.layers[0]
        .vertices
        .iter()
        .enumerate()
        .for_each(|(vertex_index, _)| {
            let embedding = sources_drains_embeddings.embeddings[0][vertex_index];
            b[3 * vertex_index + 1] = embedding.0;
            b[3 * vertex_index + 2] = embedding.1;
        });

    let edge_base_index = number_of_edges - graph.layers[graph.layers.len() - 2].vertices.len();

    graph.layers[graph.layers.len() - 2]
        .vertices
        .iter()
        .enumerate()
        .for_each(|(vertex_index, vertex)| {
            let parent_index = vertex.parent_index.unwrap();

            let embedding =
                sources_drains_embeddings.embeddings[graph.layers.len() - 1][parent_index];
            b[3 * (edge_base_index + vertex_index) + 1] = -embedding.0;
            b[3 * (edge_base_index + vertex_index) + 2] = -embedding.1;
        });

    return b;
}
