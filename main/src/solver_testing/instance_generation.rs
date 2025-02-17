use std::fs;

use solver::generate_random_flamecast_test_instance;

const INSTANCE_0: [usize; 3] = [3, 10, 2];
const INSTANCE_1: [usize; 3] = [8, 20, 2];
const INSTANCE_2: [usize; 3] = [5, 80, 5];
const INSTANCE_3: [usize; 3] = [4, 500, 5];
const INSTANCE_4: [usize; 3] = [8, 400, 10];

pub const INSTANCES: [[usize; 3]; 5] = [INSTANCE_0, INSTANCE_1, INSTANCE_2, INSTANCE_3, INSTANCE_4];

pub const PREDEFINED_BASE_PATH: &str = "./solver_test/predefined/";

pub fn generate_testing_flamecast_instance(
    num_layers: usize,
    num_sources: usize,
    num_drains: usize,
    index: usize,
) {
    let test_instance =
        generate_random_flamecast_test_instance(num_layers, num_sources, num_drains, false);

    test_instance.plot_instance(
        format!(
            "{}instance{}/instance_visualized.png",
            PREDEFINED_BASE_PATH, index
        )
        .as_str(),
    );

    fs::write(
        format!("{}instance{}/instance.json", PREDEFINED_BASE_PATH, index),
        serde_json::to_string_pretty(&test_instance).unwrap(),
    )
    .unwrap();
}

pub fn create_testing_instances() {
    for i in 0..INSTANCES.len() {
        println!("Creating testing instance {}...", i);

        let instance = INSTANCES[i];

        generate_testing_flamecast_instance(instance[0], instance[1], instance[2], i);
    }

    println!("Testing instances created successfully!");
}
