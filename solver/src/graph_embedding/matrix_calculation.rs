use std::{collections::HashMap, vec};

use clarabel::algebra::CscMatrix;

use crate::{DirectedEdge, Vertex};

pub fn calculate_p_matrix(
    number_of_regarded_vertices: usize,
    number_of_edges: usize,
) -> CscMatrix<f64> {
    // calculate P-Matrix for clarabel
    let solution_dimension = 2 * number_of_regarded_vertices + number_of_edges;

    return CscMatrix::new(
        solution_dimension,
        solution_dimension,
        vec![0; solution_dimension + 1],
        Vec::new(),
        Vec::new(),
    );
}

//TODO: Optimize this according to the formula on tablet
pub fn calculate_a_matrix(
    edges: &Vec<DirectedEdge>,
    regarded_vertices: &Vec<Vertex>,
) -> CscMatrix<f64> {
    // calculate A-Matrix for clarabel
    let number_of_regarded_vertices = regarded_vertices.len();
    let number_of_edges = edges.len();

    // create map which maps vertices to their corresponding index
    let mut vertices_indexes = HashMap::new();
    regarded_vertices
        .iter()
        .enumerate()
        .for_each(|(index, vertex)| {
            vertices_indexes.insert(*vertex, index);
        });

    // save for each vertex-index the connexted edge-index and whether its the source or the drain
    let mut connections = vec![Vec::new(); number_of_regarded_vertices];
    for (index, edge) in edges.iter().enumerate() {
        let source_index = vertices_indexes.get(&edge.0);
        let drain_index = vertices_indexes.get(&edge.1);

        if source_index.is_some() {
            let source_index = source_index.unwrap();
            connections[*source_index].push((index, -1.0));
        }
        if drain_index.is_some() {
            let drain_index = drain_index.unwrap();
            connections[*drain_index].push((index, 1.0));
        }
    }

    let mut col_ptr = vec![0; 2 * number_of_regarded_vertices + number_of_edges + 1];
    let mut row_val = Vec::new();
    let mut values = Vec::new();

    for vertex_index in 0..number_of_regarded_vertices {
        let vertex_connections = &mut connections[vertex_index];
        vertex_connections.sort_by(|(a, _), (b, _)| a.cmp(b));

        col_ptr[vertex_index + 1] = col_ptr[vertex_index] + vertex_connections.len();
        values.append(
            vertex_connections
                .iter()
                .map(|(_, value)| *value)
                .collect::<Vec<f64>>()
                .as_mut(),
        );
        row_val.append(
            vertex_connections
                .iter()
                .map(|(index, _)| 3 * index + 1)
                .collect::<Vec<usize>>()
                .as_mut(),
        );
    }

    let total_connections = col_ptr[number_of_regarded_vertices];
    for vertex_index in 0..number_of_regarded_vertices {
        col_ptr[number_of_regarded_vertices + vertex_index + 1] =
            total_connections + col_ptr[vertex_index + 1];
    }

    // also consider the y values constraints by adding the same constraints for the y values as for the x values above
    let mut row_extension = row_val.iter().map(|&x| x + 1).collect::<Vec<usize>>();
    row_val.append(&mut row_extension);
    let mut values_extension = values.clone();
    values.append(&mut values_extension);

    // add constraints for edge variables
    for i in 0..number_of_edges {
        col_ptr[2 * number_of_regarded_vertices + i + 1] =
            col_ptr[2 * number_of_regarded_vertices + i] + 1;
        row_val.push(3 * i);
        values.push(-1.0);
    }

    CscMatrix::new(
        3 * number_of_edges,
        2 * number_of_regarded_vertices + number_of_edges,
        col_ptr,
        row_val,
        values,
    )
}
