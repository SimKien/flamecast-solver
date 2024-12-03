use std::collections::HashMap;

use crate::{embed_graph, tests::float_equal, LayeredGraph, Options, Vertex, VertexEmbedding};

use super::{embeddings_equal, EmbeddingSample, TESTGRAPHS, TESTGRAPHS_EMBEDDING_SAMPLES};

const WEIZFELD_ITERATIONS: usize = 1000;

fn test_predefined_graph(index: usize) {
    let test_graph = TESTGRAPHS[index].clone();
    let graph = test_graph.graph.clone();
    let sources_embeddings = TESTGRAPHS[index].source_embeddings.clone();
    let drains_embeddings = TESTGRAPHS[index].drain_embeddings.clone();
    let alpha = TESTGRAPHS[index].alpha;

    let calculated_embeddings = embed_graph(
        graph.clone(),
        alpha,
        &sources_embeddings,
        &drains_embeddings,
        Options::default(),
    )
    .vertices_embeddings;

    let embedding_sample = TESTGRAPHS_EMBEDDING_SAMPLES[index].clone();

    match embedding_sample.valid {
        true => {
            compare_with_expected_embeddings(&calculated_embeddings, &embedding_sample);
        }
        false => {
            compare_with_generalized_weiszfeld(&calculated_embeddings, &graph, test_graph.alpha);
        }
    }
}

fn compare_with_expected_embeddings(
    calculated_embeddings: &HashMap<Vertex, VertexEmbedding>,
    embedding_sample: &EmbeddingSample,
) {
    assert!(embeddings_equal(
        &calculated_embeddings,
        &embedding_sample.embeddings,
    ));
}

fn compare_with_generalized_weiszfeld(
    calculated_embeddings: &HashMap<Vertex, VertexEmbedding>,
    graph: &LayeredGraph,
    alpha: f64,
) {
    let mut unregarded_nodes = graph.layers[0].vertices.clone();
    unregarded_nodes.extend(&graph.layers[graph.layers.len() - 1].vertices);

    let flows = graph.calculate_edge_flows();

    for node in graph.cumulate_vertices().iter() {
        if unregarded_nodes.contains(node) {
            continue;
        }

        let calculated_embedding = calculated_embeddings.get(node).unwrap();
        let surrounding_nodes = graph.get_neighbours(node);
        let surrounding_points = surrounding_nodes
            .iter()
            .map(|node| calculated_embeddings.get(node).unwrap().clone())
            .collect::<Vec<VertexEmbedding>>();
        let surrounding_weights = surrounding_nodes
            .iter()
            .map(|neighbour| {
                let flow = match flows.get(&(*node, *neighbour)) {
                    Some(flow) => flow,
                    None => flows.get(&(*neighbour, *node)).unwrap(),
                };
                return (*flow as f64).powf(alpha);
            })
            .collect::<Vec<f64>>();
        let calculated_value = generalized_weiszfeld_value(
            *calculated_embedding,
            &surrounding_points,
            &surrounding_weights,
        );

        let expected_value = objective_value(
            &surrounding_points
                .iter()
                .map(|(x, y)| {
                    ((x - calculated_embedding.0).powi(2) + (y - calculated_embedding.1).powi(2))
                        .sqrt()
                })
                .collect::<Vec<f64>>(),
            &surrounding_weights,
        );

        assert!(float_equal(calculated_value, expected_value));
    }
}

fn generalized_weiszfeld_value(
    start_point: VertexEmbedding,
    surrounding_points: &Vec<VertexEmbedding>,
    surrounding_weights: &Vec<f64>,
) -> f64 {
    let num_points = surrounding_points.len();
    let mut distances = surrounding_points
        .iter()
        .map(|point| ((point.0 - start_point.0).powi(2) + (point.1 - start_point.1).powi(2)).sqrt())
        .collect::<Vec<f64>>();
    for _ in 0..(WEIZFELD_ITERATIONS) {
        let mut dividend = (0.0, 0.0);
        let mut divisor = 0.0;
        for i in 0..num_points {
            dividend.0 += surrounding_weights[i] * surrounding_points[i].0 / distances[i];
            dividend.1 += surrounding_weights[i] * surrounding_points[i].1 / distances[i];
            divisor += surrounding_weights[i] / distances[i];
        }
        let current_point = (dividend.0 / divisor, dividend.1 / divisor);

        distances = surrounding_points
            .iter()
            .map(|point| {
                ((point.0 - current_point.0).powi(2) + (point.1 - current_point.1).powi(2)).sqrt()
            })
            .collect::<Vec<f64>>();
    }

    let value = objective_value(&distances, surrounding_weights);

    return value;
}

fn objective_value(distances: &Vec<f64>, weights: &Vec<f64>) -> f64 {
    let num_points = distances.len();
    let mut objective = 0.0;
    for i in 0..num_points {
        objective += weights[i] * distances[i];
    }
    return objective;
}

#[test]
fn test_predefined_graph_1() {
    test_predefined_graph(0);
}

#[test]
fn test_predefined_graph_2() {
    test_predefined_graph(1);
}

#[test]
fn test_predefined_graph_3() {
    test_predefined_graph(2);
}

#[test]
fn test_predefined_graph_4() {
    test_predefined_graph(3);
}

#[test]
fn test_predefined_graph_5() {
    test_predefined_graph(4);
}

#[test]
fn test_predefined_graph_6() {
    test_predefined_graph(5);
}

#[test]
fn test_predefined_graph_7() {
    test_predefined_graph(6);
}

#[test]
fn test_predefined_graph_8() {
    test_predefined_graph(7);
}

#[test]
fn test_predefined_graph_9() {
    test_predefined_graph(8);
}

#[test]
fn test_predefined_graph_10() {
    test_predefined_graph(9);
}
