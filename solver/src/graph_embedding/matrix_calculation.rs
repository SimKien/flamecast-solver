use std::{collections::HashMap, vec};

use clarabel::algebra::CscMatrix;

use crate::types::{DirectedGraph, Vertex};

pub fn calculate_p_matrix(number_of_vertices: usize, number_of_edges: usize) -> CscMatrix<f64> {
    // calculate P-Matrix for clarabel
    let solution_dimension = 2 * number_of_vertices + number_of_edges;

    return CscMatrix::new(
        solution_dimension,
        solution_dimension,
        vec![0; solution_dimension + 1],
        Vec::new(),
        Vec::new(),
    );
}

pub fn calculate_a_matrix(
    graph: &DirectedGraph,
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
) -> CscMatrix<f64> {
    // calculate A-Matrix for clarabel
    let number_of_vertices = graph.vertices.len();
    let number_of_edges = graph.edges.len();
    let number_of_sources = sources.len();
    let number_of_drains = drains.len();
    let number_of_endpoints = number_of_sources + number_of_drains;
    let subject_to_dimension = 2 * number_of_endpoints + 3 * number_of_edges;

    let mut ingoing_edges = HashMap::new();
    let mut outgoing_edges = HashMap::new();

    for (index, edge) in graph.edges.iter().enumerate() {
        ingoing_edges
            .entry(edge.1)
            .or_insert_with(Vec::new)
            .push(index);
        outgoing_edges
            .entry(edge.0)
            .or_insert_with(Vec::new)
            .push(index);
    }

    let mut col_ptr = vec![0; 2 * number_of_vertices + number_of_edges + 1];
    let mut row_val = Vec::new();
    let mut values = Vec::new();

    let mut default_ingoing_vec = Vec::new();
    let mut default_outgoing_vec = Vec::new();

    graph.vertices.iter().enumerate().for_each(|(i, vertex)| {
        // Add the source or drain position constraint to the A matrix
        if let Some(position) = sources.iter().position(|&x| x == *vertex) {
            col_ptr[i + 1] = col_ptr[i] + 1;
            row_val.push(2 * position);
            values.push(1.0);
        } else if let Some(position) = drains.iter().position(|&x| x == *vertex) {
            col_ptr[i + 1] = col_ptr[i] + 1;
            row_val.push(2 * (number_of_sources + position));
            values.push(1.0);
        } else {
            col_ptr[i + 1] = col_ptr[i];
        }

        // Add the edge length constraint to the A matrix
        let ingoing_edges = ingoing_edges
            .get_mut(vertex)
            .unwrap_or(&mut default_ingoing_vec);
        let in_len = ingoing_edges.len();
        let outgoing_edges = outgoing_edges
            .get_mut(vertex)
            .unwrap_or(&mut default_outgoing_vec);
        let out_len = outgoing_edges.len();
        ingoing_edges.sort();
        outgoing_edges.sort();

        col_ptr[i + 1] += in_len + out_len;
        col_ptr[i + 1 + number_of_vertices] =
            col_ptr[i + 1] + number_of_endpoints + 2 * number_of_edges;

        let (mut row_val_extension, mut values_extension) =
            merge_vecs(ingoing_edges, outgoing_edges, number_of_endpoints);
        row_val.append(&mut row_val_extension);
        values.append(&mut values_extension);
    });

    // Also consider the y values constraints by adding the same constraints for the y values as for the x values above
    let mut row_extension = row_val.iter().map(|&x| x + 1).collect::<Vec<usize>>();
    row_val.append(&mut row_extension);
    let mut values_extension = values.clone();
    values.append(&mut values_extension);

    for i in 0..number_of_edges {
        col_ptr[2 * number_of_vertices + i + 1] = col_ptr[2 * number_of_vertices + i] + 1;
        row_val.push(2 * number_of_endpoints + 3 * i);
        values.push(-1.0);
    }

    CscMatrix::new(
        subject_to_dimension,
        2 * number_of_vertices + number_of_edges,
        col_ptr,
        row_val,
        values,
    )
}

fn merge_vecs(
    ingoing_edges: &Vec<usize>,
    outgoing_edges: &Vec<usize>,
    number_of_endpoints: usize,
) -> (Vec<usize>, Vec<f64>) {
    let mut row_val = Vec::new();
    let mut values = Vec::new();

    let in_len = ingoing_edges.len();
    let out_len = outgoing_edges.len();

    let mut in_index = 0;
    let mut out_index = 0;

    while in_index < in_len && out_index < out_len {
        let in_edge = ingoing_edges[in_index];
        let out_edge = outgoing_edges[out_index];
        if in_edge < out_edge {
            row_val.push(2 * number_of_endpoints + 3 * in_edge + 1);
            values.push(1.0);
            in_index += 1;
        } else if in_edge > out_edge {
            row_val.push(2 * number_of_endpoints + 3 * out_edge + 1);
            values.push(-1.0);
            out_index += 1;
        }
    }

    if in_index == in_len {
        for i in out_index..out_len {
            row_val.push(2 * number_of_endpoints + 3 * outgoing_edges[i] + 1);
            values.push(-1.0);
        }
    }
    if out_index == out_len {
        for i in in_index..in_len {
            row_val.push(2 * number_of_endpoints + 3 * ingoing_edges[i] + 1);
            values.push(1.0);
        }
    }

    return (row_val, values);
}
