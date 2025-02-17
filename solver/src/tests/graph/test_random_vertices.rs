#[cfg(test)]
fn process_test_random_vertices(num_nodes: usize, num_layers: usize, amount: usize) {
    use crate::graph_generation::generate_random_directed_graph;

    let graph = generate_random_directed_graph(num_nodes, num_layers);
    let rand_vertices = graph.get_sorted_random_vertices(amount);

    assert!(rand_vertices.len() == amount);

    assert!(rand_vertices
        .is_sorted_by(|a, b| a.layer < b.layer || (a.layer == b.layer && a.index < b.index)));
}

#[test]
fn test_get_random_vertices1() {
    process_test_random_vertices(200, 4, 20);
}

#[test]
fn test_get_random_vertices2() {
    process_test_random_vertices(200, 8, 20);
}

#[test]
fn test_get_random_vertices3() {
    process_test_random_vertices(200, 4, 150);
}

#[test]
fn test_get_random_vertices4() {
    process_test_random_vertices(200, 4, 100);
}

#[test]
fn test_get_random_vertices5() {
    process_test_random_vertices(20000, 10, 1000);
}
