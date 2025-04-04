#[cfg(test)]
use rand::{seq::SliceRandom, Rng};

#[cfg(test)]
use crate::{graph_generation::generate_random_flamecast_graph, LayeredGraph, VertexID};

#[cfg(test)]
fn process_split_operation(
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

    let split_node = calculate_random_split_candidate(&graph, num_layers);
    let split_children2 = get_random_split_partition(&graph, &split_node);

    let allchildren = graph.get_children(&split_node).unwrap();

    let layer_index = split_node.layer;
    let grand_parent = graph.get_parent(&split_node).unwrap();

    let new_parent_node = graph.split(&split_children2);

    let children1 = graph.get_children(&split_node).unwrap();
    let children2 = graph.get_children(&new_parent_node).unwrap();

    allchildren.iter().for_each(|child| {
        if split_children2.contains(child) {
            assert!(graph.get_parent(child).unwrap().index == new_parent_node.index);
            assert!(children2.contains(child));
        } else {
            assert!(graph.get_parent(child).unwrap().index == split_node.index);
            assert!(children1.contains(child));
        }
    });

    assert!(
        graph.layers[layer_index].vertices.len()
            == graph_copy.layers[layer_index].vertices.len() + 1
    );

    assert!(graph.get_parent(&split_node).unwrap() == grand_parent);
    assert!(graph.get_parent(&new_parent_node).unwrap() == grand_parent);

    let grand_parent_children = graph.get_children(&grand_parent).unwrap();

    assert!(grand_parent_children.contains(&split_node));
    assert!(grand_parent_children.contains(&new_parent_node));

    graph.undo_split(&split_node, &new_parent_node);

    let children = graph.get_children(&split_node).unwrap();

    allchildren.iter().for_each(|child| {
        assert!(graph.get_parent(child).unwrap() == split_node);
        assert!(children.contains(child));
    });

    assert!(
        graph.layers[layer_index].vertices.len() == graph_copy.layers[layer_index].vertices.len()
    );

    assert!(graph.get_parent(&split_node).unwrap() == grand_parent);

    let grand_parent_children = graph.get_children(&grand_parent).unwrap();

    assert!(grand_parent_children.contains(&split_node));
    assert!(!grand_parent_children.contains(&new_parent_node));
}

#[cfg(test)]
fn get_random_split_partition(graph: &LayeredGraph, node: &VertexID) -> Vec<VertexID> {
    let mut rng = rand::thread_rng();
    let mut children = graph.get_children(node).unwrap();
    let mut partition = vec![vec![], vec![]];

    partition[0].push(children.remove(rng.gen_range(0..children.len())));
    partition[1].push(children.remove(rng.gen_range(0..children.len())));

    for child in children {
        let partition_index = rng.gen_range(0..=1);
        partition[partition_index].push(child);
    }

    return partition[0].clone();
}

#[cfg(test)]
fn calculate_random_split_candidate(graph: &LayeredGraph, num_layers: usize) -> VertexID {
    let mut candidate_layers = (1..num_layers - 1).collect::<Vec<usize>>();

    // Choose random nodes to merge
    let mut rng = rand::thread_rng();
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
            .position(|vertex| vertex.children_indices.as_ref().unwrap().len() > 1);

        if let Some(candidate_node) = candidate_node {
            return VertexID::new(candidate_layer, candidate_node);
        }
    }

    return VertexID::new(0, 0);
}

#[test]
fn test_split_operation1() {
    process_split_operation(3, &vec![1, 2, 4], 4, 1);
}

#[test]
fn test_split_operation2() {
    process_split_operation(3, &vec![1, 2, 10], 10, 1);
}

#[test]
fn test_split_operation3() {
    process_split_operation(5, &vec![1, 4, 8, 10, 400], 400, 10);
}

#[test]
fn test_split_operation4() {
    process_split_operation(4, &vec![1, 8, 10, 200], 200, 10);
}
