use crate::types::{LayeredGraph, VertexEmbeddings};
use clarabel::solver::{
    DefaultInfo, DefaultSettings, DefaultSettingsBuilder, DefaultSolution, DefaultSolver, IPSolver,
};

use super::{
    calculate_a_matrix, calculate_b_vector, calculate_cones, calculate_p_matrix,
    calculate_q_vector, EmbeddingOptions,
};

pub fn embed_directed_graph(
    graph: &LayeredGraph,
    sources_drains_embeddings: &VertexEmbeddings,
    edge_flows: &Vec<Vec<usize>>,
    alpha: f64,
    options: &EmbeddingOptions,
) -> VertexEmbeddings {
    // embed the graph using clarabel
    // assertions: valid flamecast graph
    let number_of_vertices = graph.get_number_of_vertices();
    let number_of_regarded_vertices =
        number_of_vertices - graph.get_sources_indexes().len() - graph.get_drains_indexes().len();
    let number_of_edges = graph.get_number_of_edges();

    // calculate P-Matrix for clarabel
    let p = calculate_p_matrix(number_of_regarded_vertices, number_of_edges);

    // calculate q-vector for clarabel
    let q = calculate_q_vector(
        graph,
        edge_flows,
        number_of_regarded_vertices,
        number_of_edges,
        alpha,
    );

    // calculate A-Matrix for clarabel
    let a = calculate_a_matrix(graph, number_of_regarded_vertices, number_of_edges);

    // calculate b-vector for clarabel
    let b = calculate_b_vector(graph, number_of_edges, sources_drains_embeddings);

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
    let mut result = sources_drains_embeddings.clone();

    let mut current_vertex_index = 0;
    for (layer_index, layer) in graph.layers.iter().enumerate().skip(1) {
        if layer_index == graph.layers.len() - 1 {
            break;
        }

        layer
            .vertices
            .iter()
            .enumerate()
            .for_each(|(vertex_index, _)| {
                let x = solution[current_vertex_index + vertex_index];
                let y = solution[current_vertex_index + vertex_index + number_of_regarded_vertices];
                result.embeddings[layer_index].push((x, y));
            });

        current_vertex_index += layer.vertices.len();
    }

    // print information about the solution process
    print_informations(
        &result,
        graph,
        number_of_regarded_vertices,
        options,
        &solver.info,
        &solver.solution,
    );

    return result;
}

fn print_informations(
    vertex_embeddings: &VertexEmbeddings,
    graph: &LayeredGraph,
    number_of_regarded_vertices: usize,
    options: &EmbeddingOptions,
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

        let mut index = 0;
        for (layer_index, layer) in graph.layers.iter().enumerate() {
            if layer_index == graph.layers.len() - 1 {
                break;
            }

            layer
                .vertices
                .iter()
                .enumerate()
                .for_each(|(vertex_index, vertex)| {
                    let parent_index = vertex.parent_index.unwrap();

                    let source_embedding = vertex_embeddings.embeddings[layer_index][vertex_index];
                    let target_embedding =
                        vertex_embeddings.embeddings[layer_index + 1][parent_index];

                    let edge_length = ((source_embedding.0 - target_embedding.0).powi(2)
                        + (source_embedding.1 - target_embedding.1).powi(2))
                    .sqrt();

                    let dif = (edge_length - solution.x[solution_vertices_number + index]).abs();
                    if dif > max_diff {
                        max_diff = dif;
                    }

                    index += 1;
                });
        }

        println!(
            "Maximal difference between calculated and actual edge length: {}",
            max_diff
        );
    }
}
