use std::collections::HashMap;

use crate::types::{DirectedGraph, Vertex, VertexEmbeddings};
use clarabel::solver::{
    DefaultSettings, DefaultSolver, IPSolver,
    SupportedConeT::{self, ZeroConeT},
};

use super::{calculate_a_matrix, calculate_b_vector, calculate_p_matrix, calculate_q_vector};

pub fn embed_graph(
    graph: &DirectedGraph,
    alpha: f32,
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
    sources_embeddings: &VertexEmbeddings,
    drains_embeddings: &VertexEmbeddings,
) -> VertexEmbeddings {
    // embed the graph using clarabel
    // assertions: fully connected graph, layered graph, only one outgoing edge from each vertex
    let number_of_vertices = graph.vertices.len();
    let number_of_sources = sources.len();
    let number_of_drains = drains.len();
    let number_of_endpoints = number_of_sources + number_of_drains;
    let subject_to_dimension = 2 * number_of_endpoints;

    // transformation from vertex to index in solution vector
    let mut vertex_indexes = HashMap::new();
    graph.vertices.iter().enumerate().for_each(|(i, vertex)| {
        vertex_indexes.insert(*vertex, i);
    });

    // calculate P-Matrix for clarabel
    let p = calculate_p_matrix(graph, &vertex_indexes, alpha);

    // calculate q-vector for clarabel
    let q = calculate_q_vector(number_of_vertices);

    // calculate A-Matrix for clarabel
    let a = calculate_a_matrix(graph, sources, drains);

    // calculate b-vector for clarabel
    let b = calculate_b_vector(sources, drains, sources_embeddings, drains_embeddings);

    // set cones for clarabel
    let cones: [SupportedConeT<f32>; 1] = [ZeroConeT(subject_to_dimension)];

    // create settings for clarabel
    let settings: DefaultSettings<f32> = DefaultSettings::default();

    let mut solver = DefaultSolver::new(&p, &q, &a, &b, &cones, settings);
    solver.solve();

    let solution = solver.solution.x; // x is of the form [x1, x2, ..., xn, y1, y2, ..., yn]

    // save the solution in the VertexEmbeddings format
    let mut result = VertexEmbeddings::new();
    graph.vertices.iter().enumerate().for_each(|(i, vertex)| {
        let x = solution[i];
        let y = solution[i + number_of_vertices];
        result.insert(*vertex, (x, y));
    });

    return result;
}
