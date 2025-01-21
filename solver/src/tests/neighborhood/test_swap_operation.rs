#[cfg(test)]
use rand::Rng;

#[cfg(test)]
use crate::{graph_generation::generate_random_flamecast_graph, tests::vertices_equal, VertexID};

#[cfg(test)]
fn process_swap_operation(
    num_layers: usize,
    capacities: &Vec<usize>,
    num_sources: usize,
    num_drains: usize,
) {
    let mut graph =
        generate_random_flamecast_graph(num_layers, capacities, num_sources, num_drains);
    let graph_copy = graph.clone();

    let mut rng = rand::thread_rng();
    let random_layer = rng.gen_range(0..num_layers - 1);
    let random_node1 = rng.gen_range(0..graph.layers[random_layer].vertices.len());
    let node1_id = VertexID::new(random_layer, random_node1);

    let parent1 = graph.get_parent(&node1_id).unwrap();

    let random_node2 = graph.layers[random_layer]
        .vertices
        .iter()
        .position(|x| x.parent_index.unwrap() != parent1.index)
        .unwrap();

    let node2_id = VertexID::new(random_layer, random_node2);

    let parent2 = graph.get_parent(&node2_id).unwrap();

    graph.swap(&node1_id, &node2_id);

    assert!(graph.get_parent(&node1_id).unwrap() == parent2);
    assert!(graph.get_parent(&node2_id).unwrap() == parent1);

    assert!(graph.get_children(&parent1).unwrap().contains(&node2_id));
    assert!(!graph.get_children(&parent1).unwrap().contains(&node1_id));
    assert!(graph.get_children(&parent2).unwrap().contains(&node1_id));
    assert!(!graph.get_children(&parent2).unwrap().contains(&node2_id));

    graph.undo_swap(&node1_id, &node2_id);

    assert!(vertices_equal(
        graph.get_vertex(&node1_id),
        graph_copy.get_vertex(&node1_id)
    ));

    assert!(vertices_equal(
        graph.get_vertex(&node2_id),
        graph_copy.get_vertex(&node2_id)
    ));

    assert!(vertices_equal(
        graph.get_vertex(&parent1),
        graph_copy.get_vertex(&parent1)
    ));

    assert!(vertices_equal(
        graph.get_vertex(&parent2),
        graph_copy.get_vertex(&parent2)
    ));
}

#[test]
fn test_swap_operation1() {
    process_swap_operation(3, &vec![2, 2, 2], 2, 2);
}

#[test]
fn test_swap_operation2() {
    process_swap_operation(3, &vec![2, 2, 2], 4, 2);
}

#[test]
fn test_swap_operation3() {
    process_swap_operation(5, &vec![1, 4, 8, 10, 20], 400, 20);
}

#[test]
fn test_swap_operation4() {
    process_swap_operation(4, &vec![1, 8, 10, 20], 200, 20);
}
