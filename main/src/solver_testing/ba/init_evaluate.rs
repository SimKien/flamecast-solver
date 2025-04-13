use std::fs::DirEntry;

use plotters::style::RGBColor;
use solver::SimulatedAnnealingLogger;

use super::{build_chart_init_per_instance, INIT_FUNCTION_TYPES, INIT_SOLUTIONS_DIR};

const COLORS: [RGBColor; 2] = [RGBColor(66, 212, 244), RGBColor(245, 130, 49)];

const INIT_CHARTS_DIR: &str = "./ba/charts/init";

pub fn evaluate_init(dir_name: &String) {
    let mut init_logs = vec![vec![]; INIT_FUNCTION_TYPES.len()];
    for (index, init_function) in INIT_FUNCTION_TYPES.iter().enumerate() {
        let solutions_dir = format!(
            "{}/{}/{}",
            INIT_SOLUTIONS_DIR,
            init_function.to_string(),
            dir_name
        );

        let dir_entries = std::fs::read_dir(&solutions_dir)
            .expect("Failed to read directory")
            .map(|entry| entry.expect("Failed to read entry"))
            .collect::<Vec<DirEntry>>();
        for entry in dir_entries {
            let file_path = entry.path();
            let logs = serde_json::from_str::<SimulatedAnnealingLogger>(
                &std::fs::read_to_string(file_path.clone()).unwrap(),
            );
            if logs.is_err() {
                println!("Failed to parse JSON: {:?}", file_path);
                return;
            }
            let logs = logs.unwrap();

            init_logs[index].push(logs);
        }
    }
    build_obj_init_chart(&init_logs, dir_name);
    build_obj_final_chart(&init_logs, dir_name);
    build_time_init_chart(&init_logs, dir_name);
}

fn build_obj_init_chart(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir_name: &String) {
    let random_rel_obj = logs[0]
        .iter()
        .zip(logs[1].iter())
        .map(|(x, y)| x.initial_objective_value / y.initial_objective_value)
        .collect::<Vec<f64>>();
    let matching_rel_obj = vec![1.0; logs[0].len()];
    build_chart_init_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_init_obj.png", dir_name),
        logs[0].len(),
        &vec![random_rel_obj, matching_rel_obj],
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Relative initial objective value".to_string(),
        &"Objective value".to_string(),
    );
}

fn build_obj_final_chart(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir_name: &String) {
    let random_rel_obj = logs[0]
        .iter()
        .zip(logs[1].iter())
        .map(|(x, y)| x.final_objective_value / y.final_objective_value)
        .collect::<Vec<f64>>();
    let matching_rel_obj = vec![1.0; logs[0].len()];
    build_chart_init_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_final_obj.png", dir_name),
        logs[0].len(),
        &vec![random_rel_obj, matching_rel_obj],
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Relative final objective value".to_string(),
        &"Objective value".to_string(),
    );
}

fn build_time_init_chart(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir_name: &String) {
    let random_rel_time = logs[0]
        .iter()
        .zip(logs[1].iter())
        .map(|(x, y)| {
            let time_random = x.init_time.to_time_delta().num_milliseconds();
            let time_matching = y.init_time.to_time_delta().num_milliseconds();
            return time_random as f64 / time_matching as f64;
        })
        .collect::<Vec<f64>>();
    let matching_rel_time = vec![1.0; logs[0].len()];
    build_chart_init_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_init_time.png", dir_name),
        logs[0].len(),
        &vec![random_rel_time, matching_rel_time],
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Relative initial time taken".to_string(),
        &"Time in ms".to_string(),
    );
}
