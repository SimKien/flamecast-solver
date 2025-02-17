#[cfg(test)]
use crate::{
    tests::generate_random_flamecast_instance, EmbeddingOptions, FlamecastInstance, NeighborLoader,
};

#[cfg(test)]
use super::calculate_all_neighbors;
#[cfg(test)]
use super::NEIGHBORHOOD_TEST_INSTANCES;

#[cfg(test)]
fn process_test_instance(instance: &mut FlamecastInstance, index: usize) {
    use crate::Neighbor;

    instance.embed_current_solution(&EmbeddingOptions::default());
    instance.plot_current_solution(
        format!("./test_neighbor/test{}.png", index).as_str(),
        true,
        false,
    );

    let neighbors = instance.get_all_possible_neighbors();

    let solution_state = &mut instance.solution_state;
    let all_neighbors = calculate_all_neighbors(&solution_state.current_solution.base_graph);

    let mut neighbor_loader = NeighborLoader::new();
    all_neighbors.iter().for_each(|neighbor| {
        neighbor_loader.load_neighbor(&mut solution_state.current_solution.base_graph, neighbor);

        let valid = solution_state
            .current_solution
            .base_graph
            .is_valid_flamecast_topology(&instance.capacities);

        if valid && !neighbor.is_split() {
            assert!(neighbors.contains(neighbor));
        } else if !neighbor.is_split() {
            assert!(!neighbors.contains(neighbor));
        }

        neighbor_loader.unload_neighbor(&mut solution_state.current_solution.base_graph, neighbor);

        if let Neighbor::Split(children2) = neighbor {
            let parent = solution_state
                .current_solution
                .base_graph
                .get_parent(&children2[0])
                .unwrap();

            if valid {
                assert!(neighbors.iter().any(|n| match n {
                    Neighbor::Split(children) => {
                        let parent2 = solution_state
                            .current_solution
                            .base_graph
                            .get_parent(&children[0])
                            .unwrap();
                        parent == parent2
                    }
                    _ => false,
                }));
            } else {
                assert!(!neighbors.iter().any(|n| match n {
                    Neighbor::Split(children) => {
                        let parent2 = solution_state
                            .current_solution
                            .base_graph
                            .get_parent(&children[0])
                            .unwrap();
                        parent == parent2
                    }
                    _ => false,
                }));
            }
        }
    });
}

#[test]
fn test_neighbors_predefined_graph1() {
    let mut instance = NEIGHBORHOOD_TEST_INSTANCES[0].clone();

    process_test_instance(&mut instance, 0);
}

#[test]
fn test_neighbors_random_graph1() {
    let test_instance = generate_random_flamecast_instance(5, 200, 4, true);
    let mut instance = FlamecastInstance::new(
        test_instance.alpha,
        test_instance.num_layers,
        test_instance.capacities,
        test_instance.sources_drains_embeddings,
    );

    process_test_instance(&mut instance, 1);
}

#[test]
fn test_neighbors_random_graph2() {
    let test_instance = generate_random_flamecast_instance(8, 200, 4, true);
    let mut instance = FlamecastInstance::new(
        test_instance.alpha,
        test_instance.num_layers,
        test_instance.capacities,
        test_instance.sources_drains_embeddings,
    );

    process_test_instance(&mut instance, 2);
}

#[test]
fn test_neighbors_random_graph3() {
    let test_instance = generate_random_flamecast_instance(6, 200, 5, true);
    let mut instance = FlamecastInstance::new(
        test_instance.alpha,
        test_instance.num_layers,
        test_instance.capacities,
        test_instance.sources_drains_embeddings,
    );

    process_test_instance(&mut instance, 3);
}

#[test]
fn test_neighbors_random_graph4() {
    let test_instance = generate_random_flamecast_instance(7, 300, 5, true);
    let mut instance = FlamecastInstance::new(
        test_instance.alpha,
        test_instance.num_layers,
        test_instance.capacities,
        test_instance.sources_drains_embeddings,
    );

    process_test_instance(&mut instance, 4);
}
