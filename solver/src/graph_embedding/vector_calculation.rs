use crate::{
    types::{Vertex, VertexEmbeddings},
    DirectedGraph,
};

pub fn calculate_q_vector(graph: &DirectedGraph, alpha: f64) -> Vec<f64> {
    // calculate q-vector for clarabel
    let number_of_vertices = graph.vertices.len();
    let number_of_egdes = graph.edges.len();
    let edge_flows = graph.calculate_edge_flows();

    let mut q = vec![0.0 as f64; 2 * number_of_vertices + number_of_egdes];

    graph.edges.iter().enumerate().for_each(|(index, edge)| {
        let edge_flow = edge_flows.get(edge).unwrap();
        let flow_weight = (*edge_flow as f64).powf(alpha);
        q[2 * number_of_vertices + index] = flow_weight;
    });

    return q;
}

pub fn calculate_b_vector(
    subject_to_dimension: usize,
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
    sources_embeddings: &VertexEmbeddings,
    drains_embeddings: &VertexEmbeddings,
) -> Vec<f64> {
    // calculate b-vector for clarabel
    let number_of_sources = sources.len();

    let mut b = vec![0.0 as f64; subject_to_dimension];
    sources.iter().enumerate().for_each(|(i, source)| {
        let (x, y) = sources_embeddings.get(source).unwrap();
        b[2 * i] = *x;
        b[2 * i + 1] = *y;
    });
    drains.iter().enumerate().for_each(|(i, drain)| {
        let (x, y) = drains_embeddings.get(drain).unwrap();
        b[2 * (number_of_sources + i)] = *x;
        b[2 * (number_of_sources + i) + 1] = *y;
    });

    return b;
}
