use rand::Rng;

use crate::{
    embed_graph, generate_random_graph, plotting::plot_embedded_graph, tests::float_equal,
    LayeredGraph, Options, VertexEmbedding, VertexEmbeddings,
};

use super::{
    embeddings_equal, EmbeddingSample, TESTGRAPHS, TESTGRAPHS_EMBEDDING_SAMPLES, WEISZFELD_EPSILON,
};

fn test_random_graph(num_nodes: usize, num_layers: usize, alpha: f64, index: usize) {
    let random_graph = generate_random_graph(num_nodes, num_layers);

    let x_delta = 1.0 / (num_layers as f64);
    let side_distance = x_delta / 2.0;

    let mut rng = rand::thread_rng();
    let mut sources_drains_embeddings = VertexEmbeddings::new_with_size(num_layers);

    for _ in random_graph.get_sources_indexes().iter() {
        let y = rng.gen_range(0.0..=1.0);
        sources_drains_embeddings.embeddings[0].push((side_distance, y));
    }
    for _ in random_graph.get_drains_indexes().iter() {
        let y = rng.gen_range(0.0..=1.0);
        sources_drains_embeddings.embeddings[num_layers - 1].push((1.0 - side_distance, y));
    }

    let embedded_graph = embed_graph(
        random_graph.clone(),
        &sources_drains_embeddings,
        alpha,
        Options::default(),
    );

    plot_embedded_graph(
        format!("./test_plots/output{}.png", index).as_str(),
        &embedded_graph,
        true,
    );

    compare_with_generalized_weiszfeld(&embedded_graph.vertices_embeddings, &random_graph, alpha);
}

fn test_predefined_graph(index: usize) {
    let test_graph = TESTGRAPHS[index].clone();
    let graph = test_graph.graph.clone();
    let sources_drains_embeddings = &test_graph.sources_drains_embeddings;
    let alpha = test_graph.alpha;

    let calculated_embeddings = embed_graph(
        graph.clone(),
        sources_drains_embeddings,
        alpha,
        Options::default(),
    )
    .vertices_embeddings;

    let embedding_sample = TESTGRAPHS_EMBEDDING_SAMPLES[index].clone();

    match embedding_sample.valid {
        true => {
            compare_with_expected_embeddings(&calculated_embeddings, &embedding_sample);
        }
        false => {
            compare_with_generalized_weiszfeld(&calculated_embeddings, &graph, alpha);
        }
    }
}

fn compare_with_expected_embeddings(
    calculated_embeddings: &VertexEmbeddings,
    embedding_sample: &EmbeddingSample,
) {
    assert!(embeddings_equal(
        &calculated_embeddings,
        &embedding_sample.embeddings,
    ));
}

fn compare_with_generalized_weiszfeld(
    calculated_embeddings: &VertexEmbeddings,
    graph: &LayeredGraph,
    alpha: f64,
) {
    let flows = graph.calculate_edge_flows();

    for layer in graph.layers.iter().skip(1) {
        if layer.index == graph.layers.len() - 1 {
            break;
        }
        for vertex in layer.vertices.iter() {
            let calculated_embedding =
                calculated_embeddings.embeddings[vertex.vertex_id.layer][vertex.vertex_id.index];
            let children = graph.get_children(&vertex.vertex_id).unwrap();
            let parent = graph.get_parent(&vertex.vertex_id).unwrap();
            let mut surrounding_nodes = children.clone();
            surrounding_nodes.push(parent);
            let surrounding_points = surrounding_nodes
                .iter()
                .map(|node| calculated_embeddings.embeddings[node.layer][node.index])
                .collect::<Vec<VertexEmbedding>>();
            let mut surrounding_weights = children
                .iter()
                .map(|child| {
                    let flow = flows[child.layer][child.index] as f64;
                    return flow.powf(alpha);
                })
                .collect::<Vec<f64>>();
            surrounding_weights
                .push((flows[vertex.vertex_id.layer][vertex.vertex_id.index] as f64).powf(alpha));

            let calculated_value = generalized_weiszfeld_value(
                calculated_embedding,
                &surrounding_points,
                &surrounding_weights,
            );

            let expected_value = objective_value(
                &surrounding_points
                    .iter()
                    .map(|(x, y)| {
                        ((x - calculated_embedding.0).powi(2)
                            + (y - calculated_embedding.1).powi(2))
                        .sqrt()
                    })
                    .collect::<Vec<f64>>(),
                &surrounding_weights,
            );

            assert!(float_equal(calculated_value, expected_value));
        }
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
    let mut last_value = objective_value(&distances, surrounding_weights) + 1.0;
    let mut current_value = objective_value(&distances, surrounding_weights);
    while (last_value - current_value).abs() > WEISZFELD_EPSILON {
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

        last_value = current_value;
        current_value = objective_value(&distances, surrounding_weights);
    }

    return current_value;
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

#[test]
fn test_predefined_graph_11() {
    test_predefined_graph(10);
}

#[test]
fn test_random_big_graph_1() {
    let num_nodes = 1000;
    let num_layers = 6;
    let alpha = 0.9;

    test_random_graph(num_nodes, num_layers, alpha, 0);
}

#[test]
fn test_random_big_graph_2() {
    let num_nodes = 1000;
    let num_layers = 6;
    let alpha = 0.5;

    test_random_graph(num_nodes, num_layers, alpha, 1);
}

#[test]
fn test_random_big_graph_3() {
    let num_nodes = 1000;
    let num_layers = 6;
    let alpha = 0.0;

    test_random_graph(num_nodes, num_layers, alpha, 2);
}

#[test]
fn test_random_big_graph_4() {
    let num_nodes = 1000;
    let num_layers = 6;
    let alpha = 1.0;

    test_random_graph(num_nodes, num_layers, alpha, 3);
}

#[test]
fn test_random_small_graph_1() {
    let num_nodes = 50;
    let num_layers = 4;
    let alpha = 0.9;

    test_random_graph(num_nodes, num_layers, alpha, 4);
}

#[test]
fn test_random_small_graph_2() {
    let num_nodes = 50;
    let num_layers = 4;
    let alpha = 0.5;

    test_random_graph(num_nodes, num_layers, alpha, 5);
}

#[test]
fn test_random_small_graph_3() {
    let num_nodes = 50;
    let num_layers = 4;
    let alpha = 0.0;

    test_random_graph(num_nodes, num_layers, alpha, 6);
}

#[test]
fn test_random_small_graph_4() {
    let num_nodes = 50;
    let num_layers = 4;
    let alpha = 1.0;

    test_random_graph(num_nodes, num_layers, alpha, 7);
}

#[test]
fn test_random_middle_graph_1() {
    let num_nodes = 400;
    let num_layers = 5;
    let alpha = 0.9;

    test_random_graph(num_nodes, num_layers, alpha, 8);
}

#[test]
fn test_random_middle_graph_2() {
    let num_nodes = 400;
    let num_layers = 5;
    let alpha = 0.5;

    test_random_graph(num_nodes, num_layers, alpha, 9);
}

#[test]
fn test_random_middle_graph_3() {
    let num_nodes = 400;
    let num_layers = 5;
    let alpha = 0.0;

    test_random_graph(num_nodes, num_layers, alpha, 10);
}

#[test]
fn test_random_middle_graph_4() {
    let num_nodes = 400;
    let num_layers = 5;
    let alpha = 1.0;

    test_random_graph(num_nodes, num_layers, alpha, 11);
}
