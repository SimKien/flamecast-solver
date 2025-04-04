#[cfg(test)]
use rand::Rng;

#[cfg(test)]
use crate::{graph_generation::generate_random_flamecast_graph, tests::vertices_equal, VertexID};

#[cfg(test)]
fn process_recable_operation(
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

    let mut rng = rand::thread_rng();
    let random_start_layer = rng.gen_range(0..num_layers - 1);
    let random_start_node = rng.gen_range(0..graph.layers[random_start_layer].vertices.len());
    let start_node_id = VertexID::new(random_start_layer, random_start_node);

    let old_end_node_id = graph.get_parent(&start_node_id).unwrap();
    let old_end_node = old_end_node_id.index;

    let mut random_end_node = rng.gen_range(0..graph.layers[random_start_layer + 1].vertices.len());
    if random_end_node == old_end_node {
        if random_end_node == 0 {
            random_end_node += 1;
        } else {
            random_end_node -= 1;
        }
    }
    let end_node_id = VertexID::new(random_start_layer + 1, random_end_node);

    graph.recable(&start_node_id, &end_node_id);

    assert!(graph.get_parent(&start_node_id).unwrap() == end_node_id);
    assert!(graph
        .get_children(&end_node_id)
        .unwrap()
        .iter()
        .any(|child| *child == start_node_id));

    let old_end_children = graph.get_children(&old_end_node_id);
    if let Some(children) = old_end_children {
        println!("{:?}", start_node_id);
        println!("{:?}", children);

        assert!(children.iter().all(|child| *child != start_node_id));
    }

    graph.undo_recable(&start_node_id, &old_end_node_id);

    assert!(vertices_equal(
        graph.get_vertex(&start_node_id),
        graph_copy.get_vertex(&start_node_id)
    ));

    assert!(vertices_equal(
        graph.get_vertex(&old_end_node_id),
        graph_copy.get_vertex(&old_end_node_id)
    ));

    assert!(vertices_equal(
        graph.get_vertex(&end_node_id),
        graph_copy.get_vertex(&end_node_id)
    ));
}

#[test]
fn test_recable_operation1() {
    process_recable_operation(3, &vec![2, 2, 2], 20, 20);
}

#[test]
fn test_recable_operation2() {
    process_recable_operation(3, &vec![2, 2, 2], 4, 2);
}

#[test]
fn test_recable_operation3() {
    process_recable_operation(5, &vec![1, 4, 8, 10, 20], 400, 20);
}

#[test]
fn test_recable_operation4() {
    process_recable_operation(4, &vec![1, 8, 10, 20], 200, 20);
}
