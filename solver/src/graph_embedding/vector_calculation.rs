use crate::types::{Vertex, VertexEmbeddings};

pub fn calculate_q_vector(number_of_vertices: usize) -> Vec<f32> {
    // calculate q-vector for clarabel
    let q = vec![0.0 as f32; 2 * number_of_vertices];
    return q;
}

pub fn calculate_b_vector(
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
    sources_embeddings: &VertexEmbeddings,
    drains_embeddings: &VertexEmbeddings,
) -> Vec<f32> {
    // calculate b-vector for clarabel
    let number_of_sources = sources.len();
    let number_of_drains = drains.len();
    let number_of_endpoints = number_of_sources + number_of_drains;
    let subject_to_dimension = 2 * number_of_endpoints;

    let mut b = vec![0.0 as f32; subject_to_dimension];
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
