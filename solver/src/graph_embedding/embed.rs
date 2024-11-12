use crate::types::{DirectedGraph, Vertex, VertexEmbeddings};
use clarabel::solver::{
    DefaultInfo, DefaultSettings, DefaultSettingsBuilder, DefaultSolution, DefaultSolver, IPSolver,
};

use super::{
    calculate_a_matrix, calculate_b_vector, calculate_cones, calculate_p_matrix,
    calculate_q_vector, Options,
};

pub fn embed_directed_graph(
    graph: &DirectedGraph,
    alpha: f64,
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
    sources_embeddings: &VertexEmbeddings,
    drains_embeddings: &VertexEmbeddings,
    options: Options,
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
        .verbose(options.verbose)
        .max_iter(options.search_depth as u32)
        .time_limit(options.time_limit)
        .build()
        .unwrap_or(DefaultSettings::default());

    let mut solver = DefaultSolver::new(&p, &q, &a, &b, &cones, settings);
    solver.solve();

    let solution = &solver.solution.x; // x is of the form [x1, x2, ..., xn, y1, y2, ..., yn, d1, d2, ..., dm]

    // save the solution in the VertexEmbeddings format
    let mut result = VertexEmbeddings::new();
    graph.vertices.iter().enumerate().for_each(|(i, vertex)| {
        let x = solution[i];
        let y = solution[i + number_of_vertices];
        result.insert(*vertex, (x, y));
    });

    // print information about the solution process
    print_informations(&result, graph, &options, &solver.info, &solver.solution);

    return result;
}

fn print_informations(
    vertex_embeddings: &VertexEmbeddings,
    graph: &DirectedGraph,
    options: &Options,
    solver_info: &DefaultInfo<f64>,
    solution: &DefaultSolution<f64>,
) {
    if options.print_embedding_infos {
        println!("Number of iterations: {}", solver_info.iterations);
        println!("Termination status: {:?}", solver_info.status);
        println!("Time taken: {}ms", solver_info.solve_time * 1000.0);
    }

    if options.show_calculated_actual_edge_length_diff {
        let mut max_diff = 0.0;
        for (index, edge) in graph.edges.iter().enumerate() {
            let source_embedding = vertex_embeddings.get(&edge.0).unwrap();
            let target_embedding = vertex_embeddings.get(&edge.1).unwrap();
            let edge_length = (source_embedding.0 - target_embedding.0)
                .hypot(source_embedding.1 - target_embedding.1);
            let dif = (edge_length - solution.x[2 * graph.vertices.len() + index]).abs();
            if dif > max_diff {
                max_diff = dif;
            }
        }
        println!(
            "Maximal difference between calculated and actual edge length: {}",
            max_diff
        );
    }
}
