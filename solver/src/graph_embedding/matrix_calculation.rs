use std::collections::HashMap;

use clarabel::algebra::CscMatrix;

use crate::types::{DirectedGraph, Vertex};

pub fn calculate_p_matrix(
    graph: &DirectedGraph,
    vertex_indexes: &HashMap<Vertex, Vertex>,
    alpha: f32,
) -> CscMatrix<f32> {
    // calculate P-Matrix for clarabel
    let number_of_vertices = graph.vertices.len();
    let edge_flows = graph.calculate_edge_flows();
    let double_alpha = 2.0 * alpha;

    let mut vertex_connections = HashMap::new();
    for edge in graph.edges.iter() {
        let edge_flow = edge_flows.get(edge).unwrap();
        let flow_weight = 2.0 * (*edge_flow as f32).powf(double_alpha);
        vertex_connections
            .entry(edge.0)
            .or_insert_with(Vec::new)
            .push((edge.1, flow_weight));
        vertex_connections
            .entry(edge.1)
            .or_insert_with(Vec::new)
            .push((edge.0, flow_weight));
    }

    let mut col_ptr = vec![0; 2 * number_of_vertices + 1];
    let mut row_val = Vec::new();
    let mut p_values = Vec::new();
    for (index, vertex) in graph.vertices.iter().enumerate() {
        let connections = vertex_connections.get(vertex).unwrap();
        col_ptr[index + 1] = col_ptr[index] + connections.len() + 1;
        col_ptr[index + 1 + number_of_vertices] =
            col_ptr[index + 1] + 2 * graph.edges.len() + number_of_vertices;
        let main_index = p_values.len();
        p_values.push(0.0);
        row_val.push(index);
        for (connected_vertex, flow_weight) in connections {
            p_values[main_index] += flow_weight;
            p_values.push(-*flow_weight);
            row_val.push(*vertex_indexes.get(connected_vertex).unwrap());
        }
    }

    let mut row_extension = row_val
        .iter()
        .map(|&x| x + number_of_vertices)
        .collect::<Vec<usize>>();
    row_val.append(&mut row_extension);
    let mut p_values_extension = p_values.clone();
    p_values.append(&mut p_values_extension);

    CscMatrix::new(
        2 * number_of_vertices,
        2 * number_of_vertices,
        col_ptr,
        row_val,
        p_values,
    )
}

pub fn calculate_a_matrix(
    graph: &DirectedGraph,
    sources: &Vec<Vertex>,
    drains: &Vec<Vertex>,
) -> CscMatrix<f32> {
    // calculate A-Matrix for clarabel
    let number_of_vertices = graph.vertices.len();
    let number_of_sources = sources.len();
    let number_of_drains = drains.len();
    let number_of_endpoints = number_of_sources + number_of_drains;
    let subject_to_dimension = 2 * number_of_endpoints;

    let mut col_ptr = vec![0; 2 * number_of_vertices + 1];
    let mut row_val = Vec::new();

    graph.vertices.iter().enumerate().for_each(|(i, vertex)| {
        if let Some(position) = sources.iter().position(|&x| x == *vertex) {
            col_ptr[i + 1] = col_ptr[i] + 1;
            col_ptr[i + 1 + number_of_vertices] = col_ptr[i + 1] + number_of_endpoints;
            row_val.push(2 * position);
        } else if let Some(position) = drains.iter().position(|&x| x == *vertex) {
            col_ptr[i + 1] = col_ptr[i] + 1;
            col_ptr[i + 1 + number_of_vertices] = col_ptr[i + 1] + number_of_endpoints;
            row_val.push(2 * (number_of_sources + position));
        } else {
            col_ptr[i + 1] = col_ptr[i];
            col_ptr[i + 1 + number_of_vertices] = col_ptr[i + 1] + number_of_endpoints;
        }
    });

    let mut row_extension = row_val.iter().map(|&x| x + 1).collect::<Vec<usize>>();
    row_val.append(&mut row_extension);

    CscMatrix::new(
        subject_to_dimension,
        2 * number_of_vertices,
        col_ptr,
        row_val,
        vec![1.0; subject_to_dimension],
    )
}
