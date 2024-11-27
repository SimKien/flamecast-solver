use crate::types::{LayeredGraph, Vertex, VertexEmbeddings};
use clarabel::solver::{
    DefaultInfo, DefaultSettings, DefaultSettingsBuilder, DefaultSolution, DefaultSolver, IPSolver,
};

use super::{
    calculate_a_matrix, calculate_b_vector, calculate_cones, calculate_p_matrix,
    calculate_q_vector, Options,
};

pub fn embed_directed_graph(
    graph: &LayeredGraph,
    alpha: f64,
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
    sources_embeddings: &VertexEmbeddings,
    drains_embeddings: &VertexEmbeddings,
    options: Options,
) -> VertexEmbeddings {
    // embed the graph using clarabel
    // assertions: layered graph, only one outgoing edge from each vertex
    let cumulated_edges = graph.cumulate_edges();
    let number_of_edges = cumulated_edges.len();

    let cumulated_vertices = graph.cumulate_vertices();
    let regarded_vertices = cumulated_vertices
        .iter()
        .filter(|vertex| !sources.contains(vertex) && !drains.contains(vertex))
        .map(|x| *x)
        .collect::<Vec<Vertex>>();
    let number_of_regarded_vertices = regarded_vertices.len();

    // calculate P-Matrix for clarabel
    let p = calculate_p_matrix(number_of_regarded_vertices, number_of_edges);

    // calculate q-vector for clarabel
    let q = calculate_q_vector(graph, &cumulated_edges, number_of_regarded_vertices, alpha);

    // calculate A-Matrix for clarabel
    let a = calculate_a_matrix(&cumulated_edges, &regarded_vertices);

    // calculate b-vector for clarabel
    let b = calculate_b_vector(
        &cumulated_edges,
        sources,
        drains,
        sources_embeddings,
        drains_embeddings,
    );

    // set cones for clarabel
    let cones = calculate_cones(number_of_edges);

    // create settings for clarabel
    let settings: DefaultSettings<f64> = DefaultSettingsBuilder::default()
        .verbose(options.verbose)
        .max_iter(options.search_depth as u32)
        .time_limit(options.time_limit)
        .build()
        .unwrap_or(DefaultSettings::default());

    let mut solver = DefaultSolver::new(&p, &q, &a, &b, &cones, settings);
    solver.solve();

    // x is of the form [xi, xi+1, ..., xj, yi, yi+1, ..., yj, d1, d2, ..., dm], without the (x, y) of sources or drains
    let solution = &solver.solution.x;

    // save the solution in the VertexEmbeddings format
    let mut result = VertexEmbeddings::new();
    result.extend(&sources_embeddings.content);
    result.extend(&drains_embeddings.content);
    for (index, vertex) in regarded_vertices.iter().enumerate() {
        let x = solution[index];
        let y = solution[index + number_of_regarded_vertices];
        result.insert(*vertex, (x, y));
    }

    // print information about the solution process
    print_informations(
        &result,
        graph,
        number_of_regarded_vertices,
        &options,
        &solver.info,
        &solver.solution,
    );

    return result;
}

fn print_informations(
    vertex_embeddings: &VertexEmbeddings,
    graph: &LayeredGraph,
    number_of_regarded_vertices: usize,
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
        let solution_vertices_number = 2 * number_of_regarded_vertices;
        let mut max_diff = 0.0;
        for (index, edge) in graph.cumulate_edges().iter().enumerate() {
            let source_embedding = vertex_embeddings.get(&edge.0).unwrap();
            let target_embedding = vertex_embeddings.get(&edge.1).unwrap();
            let edge_length = (source_embedding.0 - target_embedding.0)
                .hypot(source_embedding.1 - target_embedding.1);
            let dif = (edge_length - solution.x[solution_vertices_number + index]).abs();
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
