#[cfg(test)]
use crate::{tests::embedding::compare_with_generalized_weiszfeld, FlamecastInstance};

#[cfg(test)]
use super::{generate_random_flamecast_instance, FLAMECAST_TEST_INSTANCES};

#[cfg(test)]
fn validate_initial_flamecast_instance(instance: &FlamecastInstance, index: usize) {
    let current_solution = &instance.solution_state.current_solution;

    assert!(current_solution
        .base_graph
        .is_valid_flamecast_topology_check_all(
            &instance.capacities,
            instance.get_number_of_sources(),
            instance.get_number_of_drains(),
            instance.num_layers,
        ));

    compare_with_generalized_weiszfeld(
        &current_solution.vertices_embeddings,
        &current_solution.base_graph,
        instance.alpha,
    );

    instance.plot_current_solution(
        format!("./test_initial_flamecast/test{}.png", index).as_str(),
        true,
    );
}

#[test]
fn test_predefined_initial_flamecast_instance1() {
    let instance = FLAMECAST_TEST_INSTANCES[0].clone();
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 0);
}

#[test]
fn test_predefined_initial_flamecast_instance2() {
    let instance = FLAMECAST_TEST_INSTANCES[1].clone();
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 1);
}

#[test]
fn test_predefined_initial_flamecast_instance3() {
    let instance = FLAMECAST_TEST_INSTANCES[2].clone();
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 2);
}

#[test]
fn test_random_initial_flamecast_instance1() {
    let instance = generate_random_flamecast_instance(6, 100, 10, true);
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 3);
}

#[test]
fn test_random_initial_flamecast_instance2() {
    let instance = generate_random_flamecast_instance(6, 1000, 10, true);
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 4);
}

#[test]
fn test_random_initial_flamecast_instance3() {
    let instance = generate_random_flamecast_instance(6, 10000, 20, true);
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 5);
}

#[test]
fn test_random_initial_flamecast_instance4() {
    let instance = generate_random_flamecast_instance(6, 500, 100, false);
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 6);
}

#[test]
fn test_random_initial_flamecast_instance5() {
    let instance = generate_random_flamecast_instance(6, 100, 100, true);
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 7);
}

#[test]
fn test_random_initial_flamecast_instance6() {
    let instance = generate_random_flamecast_instance(5, 800, 40, true);
    let initial_instance = FlamecastInstance::new(
        instance.alpha,
        instance.num_layers,
        instance.capacities,
        instance.sources_drains_embeddings,
    );
    validate_initial_flamecast_instance(&initial_instance, 8);
}
