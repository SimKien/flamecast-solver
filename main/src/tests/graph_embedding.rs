use solver::{embed_graph, Options};

use crate::test_graphs::{TESTGRAPHS, TESTGRAPHS_OPTIMAL_EMBEDDINGS};

fn test_predefined_graph(index: usize) -> bool {
    let graph = TESTGRAPHS[index].graph.clone();
    let sources_embeddings = TESTGRAPHS[index].source_embeddings.clone();
    let drains_embeddings = TESTGRAPHS[index].drain_embeddings.clone();
    let alpha = TESTGRAPHS[index].alpha;

    let calculated_embeddings = embed_graph(
        graph,
        alpha,
        &sources_embeddings,
        &drains_embeddings,
        Options::default(),
    )
    .vertices_embeddings;

    let expected_embeddings = TESTGRAPHS_OPTIMAL_EMBEDDINGS[index].clone();

    calculated_embeddings == expected_embeddings
}

#[test]
fn test_predefined_graph_1() {
    assert!(test_predefined_graph(0));
}

#[test]
fn test_predefined_graph_2() {
    assert!(test_predefined_graph(1));
}

#[test]
fn test_predefined_graph_3() {
    assert!(test_predefined_graph(2));
}

#[test]
fn test_predefined_graph_4() {
    assert!(test_predefined_graph(3));
}

#[test]
fn test_predefined_graph_5() {
    assert!(test_predefined_graph(4));
}

#[test]
fn test_predefined_graph_6() {
    assert!(test_predefined_graph(5));
}

#[test]
fn test_predefined_graph_7() {
    assert!(test_predefined_graph(6));
}

#[test]
fn test_predefined_graph_8() {
    assert!(test_predefined_graph(7));
}
