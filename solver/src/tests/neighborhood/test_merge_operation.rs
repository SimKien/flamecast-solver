#[cfg(test)]
use rand::seq::SliceRandom;

#[cfg(test)]
use crate::{
    graph_generation::generate_random_flamecast_graph, tests::vertices_equal, LayeredGraph,
    VertexID,
};

#[cfg(test)]
fn process_merge_operation(
    num_layers: usize,
    capacities: &Vec<usize>,
    num_sources: usize,
    num_drains: usize,
) {
    use crate::tests::create_random_source_embeddings;

    let sources_embeddings = create_random_source_embeddings(num_sources);

    let mut graph = generate_random_flamecast_graph(
        num_layers,
        capacities,
        num_sources,
        num_drains,
        &sources_embeddings,
    );
    let graph_copy = graph.clone();

    let merge_pair = calculate_random_merge_pair(&graph, num_layers);

    let layer_index = merge_pair.0.layer;
    let grand_parent = graph.get_parent(&merge_pair.0).unwrap();

    let old_children_node1 = graph.get_children(&merge_pair.0).unwrap();
    let old_children_node2 = graph.get_children(&merge_pair.1).unwrap();

    let mut all_children = old_children_node1.clone();
    all_children.extend(old_children_node2.clone());

    graph.merge(&merge_pair.0, &merge_pair.1);

    let new_children = graph.get_children(&merge_pair.0).unwrap();
    all_children.iter().for_each(|child| {
        assert!(graph.get_parent(child).unwrap() == merge_pair.0);
        assert!(new_children.contains(child));
    });

    assert!(
        graph.layers[layer_index].vertices.len()
            == graph_copy.layers[layer_index].vertices.len() - 1
    );

    assert!(graph.get_parent(&merge_pair.0).unwrap() == grand_parent);

    assert!(graph
        .get_children(&grand_parent)
        .unwrap()
        .contains(&merge_pair.0));

    graph.undo_merge(&old_children_node2, &merge_pair.1);

    assert!(
        graph.layers[layer_index].vertices.len() == graph_copy.layers[layer_index].vertices.len()
    );

    old_children_node1.iter().for_each(|child| {
        assert!(vertices_equal(
            graph.get_vertex(child),
            graph_copy.get_vertex(child)
        ));
        assert!(graph.get_children(&merge_pair.0).unwrap().contains(child));
    });

    old_children_node2.iter().for_each(|child| {
        assert!(graph.get_parent(child).unwrap().index == merge_pair.1.index);
        assert!(graph.get_children(&merge_pair.1).unwrap().contains(child));
    });

    assert!(graph.get_parent(&merge_pair.0).unwrap() == grand_parent);
    assert!(graph.get_parent(&merge_pair.1).unwrap() == grand_parent);

    assert!(graph
        .get_children(&grand_parent)
        .unwrap()
        .contains(&merge_pair.0));
    assert!(graph
        .get_children(&grand_parent)
        .unwrap()
        .contains(&merge_pair.1));
}

#[cfg(test)]
fn calculate_random_merge_pair(graph: &LayeredGraph, num_layers: usize) -> (VertexID, VertexID) {
    let mut candidate_layers = (2..num_layers).collect::<Vec<usize>>();

    // Choose random nodes to merge
    let mut rng = rand::thread_rng();
    let mut merge_pair = (VertexID::new(0, 0), VertexID::new(0, 0));
    while !candidate_layers.is_empty() {
        let candidate_layer = *candidate_layers.choose(&mut rng).unwrap();
        let position = candidate_layers
            .iter()
            .position(|&x| x == candidate_layer)
            .unwrap();
        candidate_layers.remove(position);

        let candidate_node = graph.layers[candidate_layer]
            .vertices
            .iter()
            .find(|vertex| vertex.children_indices.as_ref().unwrap().len() > 1);

        if let Some(candidate_node) = candidate_node {
            let mut children = candidate_node.children_indices.as_ref().unwrap().clone();
            children.sort();
            merge_pair = (
                VertexID::new(candidate_layer - 1, children[0]),
                VertexID::new(candidate_layer - 1, children[1]),
            );
            break;
        }
    }

    return merge_pair;
}

#[test]
fn test_merge_operation1() {
    process_merge_operation(3, &vec![1, 2, 4], 4, 1);
}

#[test]
fn test_merge_operation2() {
    process_merge_operation(3, &vec![1, 2, 10], 10, 1);
}

#[test]
fn test_merge_operation3() {
    process_merge_operation(5, &vec![1, 4, 8, 10, 400], 400, 10);
}

#[test]
fn test_merge_operation4() {
    process_merge_operation(4, &vec![1, 8, 10, 200], 200, 10);
}
