use std::fs::DirEntry;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solver::{Neighbor, SimulatedAnnealingLogger};

use super::{
    build_chart_compare_operations, build_chart_improve_per_operation, build_chart_no_process,
    build_chart_num_nodes, build_chart_process, ALPHA_SOLUTIONS_DIR, ALPHA_VALUES,
};

const ALPHA_CHARTS_DIR: &str = "./ba/charts/alpha";

pub fn evaluate_alpha(process: bool, dir: Option<String>) {
    let dirs = match dir {
        Some(dir) => vec![dir],
        None => {
            let mut dirs = Vec::new();
            let solution_dir = format!(
                "{}/{}",
                ALPHA_SOLUTIONS_DIR,
                ALPHA_VALUES[0].to_string().replace('.', "_")
            );
            for entry in std::fs::read_dir(&solution_dir).unwrap() {
                let entry = entry.unwrap();
                if entry.path().is_dir() {
                    dirs.push(
                        entry
                            .path()
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );
                }
            }
            dirs
        }
    };

    if dirs.is_empty() {
        println!("No directories found in {}", ALPHA_SOLUTIONS_DIR);
        return;
    }

    if process {
        process_multi_dir_process(&dirs, &ALPHA_VALUES.to_vec());
    } else {
        process_multi_dir_no_process(&dirs, &ALPHA_VALUES.to_vec());
    }
}

fn process_multi_dir_process(dirs: &Vec<String>, alphas: &Vec<f64>) {
    for dir in dirs.iter() {
        for alpha in alphas.iter() {
            let alpha_string = alpha.to_string().replace('.', "_");
            let solution_dir = format!("{}/{}/{}", ALPHA_SOLUTIONS_DIR, alpha_string, dir);

            if !std::path::Path::new(&solution_dir).exists() {
                println!("Directory does not exist: {}", solution_dir);
                return;
            }

            let entries = std::fs::read_dir(&solution_dir)
                .expect("Failed to read directory")
                .map(|entry| entry.expect("Failed to read entry"))
                .collect::<Vec<DirEntry>>();
            let mut num_iterations = Vec::new();
            for entry in entries.iter() {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().unwrap() == "json" {
                    let logs = serde_json::from_str::<SimulatedAnnealingLogger>(
                        &std::fs::read_to_string(file_path.clone()).unwrap(),
                    );
                    if logs.is_err() {
                        println!("Failed to parse JSON: {:?}", file_path);
                        return;
                    }
                    let logs = logs.unwrap();
                    num_iterations.push(logs.max_iterations);
                }
            }

            let max_iterations = num_iterations.iter().min().unwrap_or(&0).clone();
            let mut data_time = vec![vec![]; max_iterations];
            let mut data_obj_value = vec![vec![]; max_iterations];
            let mut data_secondary = vec![vec![]; max_iterations];
            let mut num_nodes = vec![vec![]; max_iterations];
            let mut operations = vec![vec![]; 4];
            let mut improvements = vec![vec![]; 4];
            for entry in entries.iter() {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().unwrap() == "json" {
                    let logs = serde_json::from_str::<SimulatedAnnealingLogger>(
                        &std::fs::read_to_string(file_path.clone()).unwrap(),
                    );
                    if logs.is_err() {
                        println!("Failed to parse JSON: {:?}", file_path);
                        return;
                    }
                    let logs = logs.unwrap();
                    for i in 0..max_iterations {
                        let time_needed = logs.times_needed[i].clone();
                        let mut time = time_needed.secs as f32;
                        if time_needed.nanos > 100_000_000 {
                            let nanos = time_needed.nanos / 100_000;
                            time += nanos as f32 / 10_000.0;
                        } else if time_needed.nanos > 1_000_000 {
                            let nanos = time_needed.nanos / 1_000;
                            time += nanos as f32 / 1_000_000.0;
                        } else if time_needed.nanos > 0 {
                            time += time_needed.nanos as f32 / 1_000_000_000.0;
                        }
                        data_time[i].push(time);
                        data_obj_value[i].push(logs.current_best_costs[i] as f32);
                        data_secondary[i].push(logs.current_costs[i] as f32);
                        num_nodes[i].push(logs.current_amount_nodes[i]);
                    }
                    let mut nums = vec![0; 4];
                    let mut improvements_nums = vec![0.0; 4];
                    for neighbor_accepted in logs.accepted_neighbors.iter() {
                        let improvement = if neighbor_accepted.iteration == 0 {
                            (0.0_f32).max(
                                (logs.initial_objective_value - neighbor_accepted.neighbor_cost)
                                    as f32,
                            )
                        } else {
                            (0.0_f32).max(
                                (logs.current_best_costs[neighbor_accepted.iteration - 1]
                                    - neighbor_accepted.neighbor_cost)
                                    as f32,
                            )
                        };
                        if neighbor_accepted.neighbor.is_recable() {
                            nums[0] += 1;
                            improvements_nums[0] += improvement;
                        } else if neighbor_accepted.neighbor.is_swap() {
                            nums[1] += 1;
                            improvements_nums[1] += improvement;
                        } else if neighbor_accepted.neighbor.is_merge() {
                            nums[2] += 1;
                            improvements_nums[2] += improvement;
                        } else if neighbor_accepted.neighbor.is_split() {
                            nums[3] += 1;
                            improvements_nums[3] += improvement;
                        }
                    }
                    for i in 0..4 {
                        operations[i].push(nums[i]);
                        improvements[i].push(improvements_nums[i]);
                    }
                }
            }
            let chart_dir = format!("{}/{}", ALPHA_CHARTS_DIR, alpha_string);
            let time_chart_name = format!("{}_time.png", dir);
            let obj_chart_name = format!("{}_obj.png", dir);
            let op_chart_name = format!("{}_op.png", dir);
            let op_improvement_chart_name = format!("{}_op_improvement.png", dir);
            let num_nodes_chart_name = format!("{}_num_nodes.png", dir);

            build_chart_process(
                &chart_dir,
                &obj_chart_name,
                &data_obj_value,
                Some(&data_secondary),
                "Objective value",
                max_iterations,
            );
            build_chart_process(
                &chart_dir,
                &time_chart_name,
                &data_time,
                None,
                "Time needed in (s)",
                max_iterations,
            );
            build_chart_compare_operations(
                &chart_dir,
                &op_chart_name,
                &operations,
                &Neighbor::get_names(),
            );
            build_chart_improve_per_operation(
                &chart_dir,
                &op_improvement_chart_name,
                &improvements,
                &Neighbor::get_names(),
            );
            build_chart_num_nodes(
                &chart_dir,
                &num_nodes_chart_name,
                &num_nodes,
                max_iterations,
            );
        }
    }
}

fn process_multi_dir_no_process(dirs: &Vec<String>, alphas: &Vec<f64>) {
    dirs.par_iter().for_each(|dir| {
        let mut data_time = vec![vec![]; alphas.len()];
        let mut data_obj_value = vec![vec![]; alphas.len()];

        for (index, alpha) in alphas.iter().enumerate() {
            let alpha_string = alpha.to_string().replace('.', "_");
            let solution_dir = format!("{}/{}/{}", ALPHA_SOLUTIONS_DIR, alpha_string, dir);

            if !std::path::Path::new(&solution_dir).exists() {
                println!("Directory does not exist: {}", solution_dir);
                return;
            }

            let entries = std::fs::read_dir(&solution_dir)
                .expect("Failed to read directory")
                .map(|entry| entry.expect("Failed to read entry"))
                .collect::<Vec<DirEntry>>();
            for entry in entries.iter() {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().unwrap() == "json" {
                    let logs = serde_json::from_str::<SimulatedAnnealingLogger>(
                        &std::fs::read_to_string(file_path.clone()).unwrap(),
                    );
                    if logs.is_err() {
                        println!("Failed to parse JSON: {:?}", file_path);
                        return;
                    }
                    let logs = logs.unwrap();
                    let time_needed = logs.total_time;
                    let mut time = time_needed.secs as f32;
                    if time_needed.nanos > 100_000_000 {
                        let nanos = time_needed.nanos / 100_000;
                        time += nanos as f32 / 10_000.0;
                    } else if time_needed.nanos > 1_000_000 {
                        let nanos = time_needed.nanos / 1_000;
                        time += nanos as f32 / 1_000_000.0;
                    } else if time_needed.nanos > 0 {
                        time += time_needed.nanos as f32 / 1_000_000_000.0;
                    }
                    data_time[index].push(time);
                    data_obj_value[index].push(logs.final_objective_value as f32);
                }
            }
        }
        let time_chart_name = format!("{}_compare_time.png", dir);
        let obj_chart_name = format!("{}_compare_obj.png", dir);

        build_chart_no_process(
            &ALPHA_CHARTS_DIR.to_string(),
            &obj_chart_name,
            &data_obj_value,
            "Objective value",
            &alphas,
        );
        build_chart_no_process(
            &ALPHA_CHARTS_DIR.to_string(),
            &time_chart_name,
            &data_time,
            "Time needed in (s)",
            &alphas,
        );
    });
}
