use std::fs;

use solver::{
    generate_random_flamecast_test_instance, EmbeddingOptions, NeighborSearchOption,
    OptimizationOptions,
};

use super::{solve_processing_instance, IterationCoolingOption, ProcessingInstance};

pub const USER_DEFINED_BASE_PATH: &str = "./solver_test/user_defined/";

pub fn test_custom_instance(
    num_sources: usize,
    num_drains: usize,
    num_layers: usize,
    alpha: Option<f64>,
    iteration_cooling_option: IterationCoolingOption,
    index: usize,
) {
    let test_processing_instance = get_processing_instance(
        num_sources,
        num_drains,
        num_layers,
        alpha,
        iteration_cooling_option,
        index,
    );

    let base_path = format!("{}{}", USER_DEFINED_BASE_PATH, index);

    solve_processing_instance(&test_processing_instance, &base_path);

    println!("Finished processing the instance");
}

fn get_processing_instance(
    num_sources: usize,
    num_drains: usize,
    num_layers: usize,
    alpha: Option<f64>,
    iteration_cooling_option: IterationCoolingOption,
    index: usize,
) -> ProcessingInstance {
    let mut test_instance =
        generate_random_flamecast_test_instance(num_layers, num_sources, num_drains, false);

    if let Some(alpha) = alpha {
        test_instance.alpha = alpha;
    }

    test_instance.plot_instance(
        format!("{}{}instance_visualized.png", USER_DEFINED_BASE_PATH, index).as_str(),
    );
    fs::write(
        format!("{}{}instance.json", USER_DEFINED_BASE_PATH, index),
        serde_json::to_string_pretty(&test_instance).unwrap(),
    )
    .unwrap();

    let default_options = EmbeddingOptions::default();
    let option = OptimizationOptions::new(
        iteration_cooling_option.0 .0,
        iteration_cooling_option.0 .1,
        NeighborSearchOption::CompleteHeuristical,
        iteration_cooling_option.1,
        2,
        true,
        default_options.clone(),
        default_options.clone(),
        default_options.clone(),
    );
    return ProcessingInstance::new(test_instance, option, 0, 0);
}
