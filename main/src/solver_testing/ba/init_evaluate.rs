use std::fs::DirEntry;

use plotters::{chart::SeriesLabelPosition, data::Quartiles, style::RGBColor};
use solver::{SimulatedAnnealingLogger, TimeDelta};

use super::{
    build_chart_usize_per_instance, get_converged_iteration, INIT_FUNCTION_TYPES,
    INIT_SOLUTIONS_DIR,
};

const COLORS: [RGBColor; 2] = [RGBColor(66, 212, 244), RGBColor(245, 130, 49)];
const COLOR_NOT_FOUND: RGBColor = RGBColor(255, 0, 0);

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

    //converged_evaluation(&init_logs);

    let converged = &get_converged_iteration(&init_logs)[1];
    let converged = init_logs[1]
        .iter()
        .zip(converged.iter())
        .map(|(x, y)| *y as f64 / x.max_iterations as f64)
        .collect::<Vec<f64>>();

    let quartile = Quartiles::new(&converged);
    println!("Quartiles matching: {:?}", quartile);

    let converged = &get_converged_iteration(&init_logs)[0];
    let converged = init_logs[0]
        .iter()
        .zip(converged.iter())
        .map(|(x, y)| *y as f64 / x.max_iterations as f64)
        .collect::<Vec<f64>>();

    let quartile = Quartiles::new(&converged);
    println!("Quartiles random: {:?}", quartile);

    /*
    build_obj_init_chart(&init_logs, dir_name);
    build_obj_final_chart(&init_logs, dir_name);
    build_time_init_chart(&init_logs, dir_name);
    build_converged_init_chart(&init_logs, dir_name);
    */
}

fn converged_evaluation(logs: &Vec<Vec<SimulatedAnnealingLogger>>) {
    let converged = get_converged_iteration(logs);
    let converged = converged
        .iter()
        .enumerate()
        .map(|(func_index, x)| {
            x.iter()
                .enumerate()
                .map(|(index, y)| get_time_converges(&logs[func_index][index], *y))
                .collect::<Vec<TimeDelta>>()
        })
        .collect::<Vec<Vec<TimeDelta>>>();

    let random_taken = Quartiles::new(
        &converged[0]
            .iter()
            .map(|x| x.num_milliseconds() as f64)
            .collect::<Vec<f64>>(),
    );
    let matching_taken = Quartiles::new(
        &converged[1]
            .iter()
            .map(|x| x.num_milliseconds() as f64)
            .collect::<Vec<f64>>(),
    );
    println!("Quartiles random: {:?}", random_taken);
    println!("Quartiles matching: {:?}", matching_taken);

    let diff_taken = Quartiles::new(
        &converged[1]
            .iter()
            .zip(converged[0].iter())
            .map(|(x, y)| x.num_milliseconds() as f64 - y.num_milliseconds() as f64)
            .collect::<Vec<f64>>(),
    );
    println!("Quartiles diff: {:?}", diff_taken);
}

fn build_converged_init_chart(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir_name: &String) {
    let converged = get_converged_iteration(logs);

    let converged = converged
        .iter()
        .enumerate()
        .map(|(func_index, x)| {
            x.iter()
                .enumerate()
                .map(|(index, y)| get_time_converges(&logs[func_index][index], *y))
                .collect::<Vec<TimeDelta>>()
        })
        .collect::<Vec<Vec<TimeDelta>>>();

    let mut convergence_matching = converged[1]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, TimeDelta)>>();

    convergence_matching.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut data = vec![vec![]; INIT_FUNCTION_TYPES.len()];
    let mut data_max_time = vec![vec![]; INIT_FUNCTION_TYPES.len()];
    for i in 0..INIT_FUNCTION_TYPES.len() {
        let converged_iterations = &converged[i];
        convergence_matching.iter().for_each(|(index, _)| {
            data[i].push(converged_iterations[*index].num_seconds() as usize);
            data_max_time[i]
                .push(logs[i][*index].total_time.to_time_delta().num_seconds() as usize);
        });
    }

    let mut convergence_random = converged[0]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, TimeDelta)>>();

    convergence_random.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut max_time = Vec::new();
    convergence_random.iter().for_each(|(index, _)| {
        max_time.push(logs[0][*index].total_time.to_time_delta().num_seconds() as usize);
    });

    build_chart_usize_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_init_random__conv.png", dir_name),
        logs[0].len(),
        &vec![convergence_random
            .iter()
            .map(|x| x.1.num_seconds() as usize)
            .collect::<Vec<usize>>()],
        &vec![max_time],
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Time until Convergence (s)".to_string(),
        SeriesLabelPosition::UpperMiddle,
        COLOR_NOT_FOUND,
    );
    build_chart_usize_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_init_matching_conv.png", dir_name),
        logs[0].len(),
        &vec![data[1].clone()],
        &data_max_time,
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Time until Convergence (s)".to_string(),
        SeriesLabelPosition::UpperMiddle,
        COLOR_NOT_FOUND,
    );
}

pub fn get_time_converges(log: &SimulatedAnnealingLogger, iterations: usize) -> TimeDelta {
    let mut time = log.init_time.to_time_delta();
    for i in 0..iterations {
        time += log.times_needed[i].to_time_delta();
    }
    time
}

/*
fn build_converged_init_chart(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir_name: &String) {
    let converged = get_converged_iteration(logs);

    let mut convergence_matching = converged[1]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, usize)>>();

    convergence_matching.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut data = vec![vec![]; INIT_FUNCTION_TYPES.len()];
    for i in 0..INIT_FUNCTION_TYPES.len() {
        let converged_iterations = &converged[i];
        convergence_matching.iter().for_each(|(index, _)| {
            data[i].push(converged_iterations[*index]);
        });
    }

    build_chart_converged_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_init_conv.png", dir_name),
        logs[0].len(),
        &data,
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Convergence iteration".to_string(),
        SeriesLabelPosition::UpperMiddle,
    );
}

fn build_obj_init_chart(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir_name: &String) {
    let random_rel_obj = logs[0]
        .iter()
        .zip(logs[1].iter())
        .map(|(x, y)| x.initial_objective_value / y.initial_objective_value)
        .collect::<Vec<f64>>();
    let matching_rel_obj = vec![1.0; logs[0].len()];
    build_chart_init_obj_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_init_obj.png", dir_name),
        logs[0].len(),
        &vec![random_rel_obj, matching_rel_obj],
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Relative objective value".to_string(),
    );
}

fn build_obj_final_chart(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir_name: &String) {
    let random_rel_obj = logs[0]
        .iter()
        .zip(logs[1].iter())
        .map(|(x, y)| x.final_objective_value / y.final_objective_value)
        .collect::<Vec<f64>>();
    let matching_rel_obj = vec![1.0; logs[0].len()];
    build_chart_init_obj_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_final_obj.png", dir_name),
        logs[0].len(),
        &vec![random_rel_obj, matching_rel_obj],
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Relative objective value".to_string(),
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
    build_chart_init_obj_per_instance(
        &INIT_CHARTS_DIR.to_string(),
        &format!("{}_init_time.png", dir_name),
        logs[0].len(),
        &vec![random_rel_time, matching_rel_time],
        &COLORS.to_vec(),
        &INIT_FUNCTION_TYPES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Relative time taken".to_string(),
    );
}
    */
