use std::{collections::HashMap, vec};

use clarabel::algebra::CscMatrix;

use crate::LayeredGraph;

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

pub fn calculate_a_matrix(
    graph: &LayeredGraph,
    number_of_regarded_vertices: usize,
    number_of_edges: usize,
) -> CscMatrix<f64> {
    // calculate A-Matrix for clarabel

    let mut col_ptr = vec![0; 2 * number_of_regarded_vertices + number_of_edges + 1];
    let mut row_val = Vec::new();
    let mut values = Vec::new();

    let num_values = 2 * number_of_edges
        - graph.layers[0].vertices.len()
        - graph.layers[graph.layers.len() - 2].vertices.len();

    let mut current_edge_index = 0;
    let mut current_vertex_index = 0;
    let mut connected_edges = HashMap::new();
    for (layer_index, layer) in graph.layers.iter().enumerate() {
        if layer_index == graph.layers.len() - 1 {
            break;
        }

        let layer_size = layer.vertices.len();

        let mut edge_index = 0;
        if layer_index == 0 {
            layer.vertices.iter().for_each(|vertex| {
                let parent_index = vertex.parent_index.unwrap();

                connected_edges
                    .entry(parent_index)
                    .or_insert(Vec::new())
                    .push(edge_index);

                edge_index += 1;
            });
        } else {
            layer.vertices.iter().for_each(|vertex| {
                let vertex_input_edges = connected_edges
                    .get(&(current_vertex_index + vertex.vertex_id.index))
                    .unwrap();

                values.append(&mut vec![1.0; vertex_input_edges.len()]);
                col_ptr[current_vertex_index + vertex.vertex_id.index + 1] = col_ptr
                    [current_vertex_index + vertex.vertex_id.index]
                    + vertex_input_edges.len()
                    + 1;
                col_ptr[current_vertex_index
                    + vertex.vertex_id.index
                    + 1
                    + number_of_regarded_vertices] =
                    col_ptr[current_vertex_index + vertex.vertex_id.index + 1] + num_values;
                row_val.append(
                    &mut vertex_input_edges
                        .iter()
                        .map(|x| 3 * x + 1)
                        .collect::<Vec<usize>>(),
                );

                let parent_index = vertex.parent_index.unwrap();

                values.push(-1.0);
                row_val.push(3 * (current_edge_index + edge_index) + 1);

                connected_edges
                    .entry(current_vertex_index + layer_size + parent_index)
                    .or_insert(Vec::new())
                    .push(current_edge_index + edge_index);

                edge_index += 1;
            });

            current_vertex_index += layer_size;
        }

        current_edge_index += layer_size;
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

    return CscMatrix::new(
        3 * number_of_edges,
        2 * number_of_regarded_vertices + number_of_edges,
        col_ptr,
        row_val,
        values,
    );
}
