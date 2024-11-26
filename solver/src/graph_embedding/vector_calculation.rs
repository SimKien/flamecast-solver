use crate::{
    types::{Vertex, VertexEmbeddings},
    DirectedEdge, LayeredGraph,
};

pub fn calculate_q_vector(
    graph: &LayeredGraph,
    edges: &Vec<DirectedEdge>,
    number_of_regarded_endpoints: usize,
    alpha: f64,
) -> Vec<f64> {
    // calculate q-vector for clarabel
    let number_of_egdes = edges.len();
    let edge_flows = graph.calculate_edge_flows();

    let mut q = vec![0.0 as f64; 2 * number_of_regarded_endpoints + number_of_egdes];

    edges.iter().enumerate().for_each(|(index, edge)| {
        let edge_flow = edge_flows.get(edge).unwrap();
        let flow_weight = (*edge_flow as f64).powf(alpha);
        q[2 * number_of_regarded_endpoints + index] = flow_weight;
    });

    return q;
}

pub fn calculate_b_vector(
    edges: &Vec<DirectedEdge>,
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
    sources_embeddings: &VertexEmbeddings,
    drains_embeddings: &VertexEmbeddings,
) -> Vec<f64> {
    // calculate b-vector for clarabel
    let mut b = vec![0.0; 3 * edges.len()];

    for (index, (start, end)) in edges.iter().enumerate() {
        if sources.contains(start) {
            let embedding = sources_embeddings.get(start).unwrap();
            b[3 * index + 1] += embedding.0;
            b[3 * index + 2] += embedding.1;
        }
        if drains.contains(end) {
            let embedding = drains_embeddings.get(end).unwrap();
            b[3 * index + 1] -= embedding.0;
            b[3 * index + 2] -= embedding.1;
        }
    }

    return b;
}
