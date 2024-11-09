use crate::types::{DirectedGraph, Vertex, VertexEmbeddings};
use clarabel::solver::{DefaultSettings, DefaultSettingsBuilder, DefaultSolver, IPSolver};

use super::{
    calculate_a_matrix, calculate_b_vector, calculate_cones, calculate_p_matrix,
    calculate_q_vector, SearchDepth,
};

pub fn embed_directed_graph(
    graph: &DirectedGraph,
    alpha: f64,
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
    sources_embeddings: &VertexEmbeddings,
    drains_embeddings: &VertexEmbeddings,
    search_depth: SearchDepth,
) -> VertexEmbeddings {
    // embed the graph using clarabel
    // assertions: fully connected graph, layered graph, only one outgoing edge from each vertex
    let number_of_vertices = graph.vertices.len();
    let number_of_edges = graph.edges.len();
    let number_of_sources = sources.len();
    let number_of_drains = drains.len();
    let number_of_endpoints = number_of_sources + number_of_drains;
    let subject_to_dimension = 2 * number_of_endpoints + 3 * number_of_edges;

    // calculate P-Matrix for clarabel
    let p = calculate_p_matrix(number_of_vertices, number_of_edges);

    // calculate q-vector for clarabel
    let q = calculate_q_vector(graph, alpha);

    // calculate A-Matrix for clarabel
    let a = calculate_a_matrix(graph, sources, drains);

    // calculate b-vector for clarabel
    let b = calculate_b_vector(
        subject_to_dimension,
        sources,
        drains,
        sources_embeddings,
        drains_embeddings,
    );

    // set cones for clarabel
    let cones = calculate_cones(number_of_endpoints, number_of_edges);

    // create settings for clarabel
    let settings: DefaultSettings<f64> = DefaultSettingsBuilder::default()
        .verbose(false)
        .max_iter(search_depth as u32)
        .build()
        .unwrap_or(DefaultSettings::default());

    let mut solver = DefaultSolver::new(&p, &q, &a, &b, &cones, settings);
    solver.solve();

    let solution = solver.solution.x; // x is of the form [x1, x2, ..., xn, y1, y2, ..., yn, d1, d2, ..., dm]

    // save the solution in the VertexEmbeddings format
    let mut result = VertexEmbeddings::new();
    graph.vertices.iter().enumerate().for_each(|(i, vertex)| {
        let x = solution[i];
        let y = solution[i + number_of_vertices];
        result.insert(*vertex, (x, y));
    });

    // print information about the solution process
    println!("Number of iterations: {}", solver.info.iterations);
    println!("Termination status: {:?}", solver.info.status);
    println!("Time taken: {}ms", solver.info.solve_time * 1000.0);

    return result;
}
