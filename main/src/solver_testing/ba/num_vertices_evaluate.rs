use std::fs::DirEntry;

use plotters::{chart::SeriesLabelPosition, style::RGBColor};
use solver::{SimulatedAnnealingLogger, TimeDelta};

use super::{
    build_chart_conv_time_improvement, build_chart_iteration_converges_vertices,
    build_chart_plot_rel_improvement_per_instance, build_chart_time_converges_vertices,
    build_chart_usize_per_instance, get_converged_iteration, get_time_converges, NUM_VERTICES,
    NUM_VERTICES_SOLUTIONS_DIR,
};

const NUM_VERTICES_CHARTS_DIR: &str = "./ba/charts/num_vertices";

const COLORS: [RGBColor; 5] = [
    RGBColor(66, 212, 244),
    RGBColor(245, 130, 49),
    RGBColor(0, 0, 0),
    RGBColor(60, 180, 75),
    RGBColor(145, 30, 180),
];

pub fn evaluate_num_vertices(dir: &String, alpha: f64) {
    let mut num_vertices_logs = vec![vec![]; NUM_VERTICES.len()];
    for (index, num_vertices) in NUM_VERTICES.iter().enumerate() {
        let solutions_dir = format!(
            "{}/{}/{}/{}",
            NUM_VERTICES_SOLUTIONS_DIR,
            alpha.to_string().replace('.', "_"),
            num_vertices,
            dir
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

            num_vertices_logs[index].push(logs);
        }
    }

    //evaluate_total_time_conv(&num_vertices_logs, alpha, dir);

    evaluate_conv_iteration(&num_vertices_logs, alpha, dir);

    /*
    evaluate_conv_improvement(&num_vertices_logs, alpha, dir);

    evaluate_per_instance_time_taken(&num_vertices_logs, alpha, dir);
    evaluate_per_instance_relative_improvement(&num_vertices_logs, alpha, dir);
    evaluate_per_instance_when_convergence(&num_vertices_logs, dir, alpha);
    */
}

fn evaluate_conv_iteration(logs: &Vec<Vec<SimulatedAnnealingLogger>>, alpha: f64, dir: &String) {
    let converged = &get_converged_iteration(logs);
    let mut data = vec![vec![]; NUM_VERTICES.len()];
    for i in 0..NUM_VERTICES.len() {
        let converged = &converged[i];
        data[i] = logs[i]
            .iter()
            .zip(converged.iter())
            .map(|(log, y)| (*y as f64 / log.max_iterations as f64) * 100.0)
            .collect::<Vec<f64>>();
    }

    let chart_dir = format!(
        "{}/{}",
        NUM_VERTICES_CHARTS_DIR,
        alpha.to_string().replace('.', "_")
    );
    let name = format!("{}_cmp_iteration.png", dir);

    build_chart_iteration_converges_vertices(&chart_dir, &name, &data, &NUM_VERTICES.to_vec());
}

fn evaluate_total_time_conv(logs: &Vec<Vec<SimulatedAnnealingLogger>>, alpha: f64, dir: &String) {
    let converged = get_converged_iteration(logs);

    let time_taken = logs
        .iter()
        .enumerate()
        .map(|(index, log)| {
            log.iter()
                .enumerate()
                .map(|(log_index, log)| {
                    let time = get_time_converges(log, converged[index][log_index]);
                    return time.num_milliseconds() as f64 / 1000.0;
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    let chart_dir = format!(
        "{}/{}",
        NUM_VERTICES_CHARTS_DIR,
        alpha.to_string().replace('.', "_")
    );
    let name = format!("{}_cmp_time.png", dir);

    build_chart_time_converges_vertices(&chart_dir, &name, &time_taken, &NUM_VERTICES.to_vec());
}

fn evaluate_conv_improvement(logs: &Vec<Vec<SimulatedAnnealingLogger>>, alpha: f64, dir: &String) {
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

    let mut data = vec![vec![]; NUM_VERTICES.len()];
    logs.iter().enumerate().for_each(|(index, logs)| {
        logs.iter().enumerate().for_each(|(log_index, log)| {
            let improvement = (log.initial_objective_value - log.final_objective_value)
                / log.initial_objective_value;
            data[index].push((
                converged[index][log_index].clone().num_milliseconds() as f64 / 1000.0,
                improvement,
            ));
        });
    });

    let chart_dir = format!(
        "{}/{}",
        NUM_VERTICES_CHARTS_DIR,
        alpha.to_string().replace('.', "_")
    );
    let name = format!("{}_conv_impr.png", dir);

    build_chart_conv_time_improvement(
        &chart_dir,
        &name,
        &data,
        &COLORS.to_vec(),
        &NUM_VERTICES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        SeriesLabelPosition::MiddleRight,
    );
}

fn evaluate_per_instance_time_taken(
    logs: &Vec<Vec<SimulatedAnnealingLogger>>,
    alpha: f64,
    dir: &String,
) {
    let time_taken = logs
        .iter()
        .map(|log| {
            log.iter()
                .map(|log| {
                    let time = log.total_time.to_time_delta() + log.init_time.to_time_delta();
                    return time.num_seconds() as usize;
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut time_taken_10 = time_taken[2]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, usize)>>();

    time_taken_10.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut data = vec![vec![]; NUM_VERTICES.len()];
    for i in 0..NUM_VERTICES.len() {
        let time = &time_taken[i];
        time_taken_10.iter().for_each(|(index, _)| {
            data[i].push(time[*index]);
        });
    }

    let chart_dir = format!(
        "{}/{}",
        NUM_VERTICES_CHARTS_DIR,
        alpha.to_string().replace('.', "_")
    );
    let name = format!("{}_cmp_time.png", dir);

    build_chart_usize_per_instance(
        &chart_dir,
        &name,
        time_taken_10.len(),
        &data,
        &Vec::new(),
        &COLORS.to_vec(),
        &NUM_VERTICES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Time taken (s)".to_string(),
        SeriesLabelPosition::MiddleRight,
        RGBColor(255, 0, 0),
    );
}

fn evaluate_per_instance_relative_improvement(
    logs: &Vec<Vec<SimulatedAnnealingLogger>>,
    alpha: f64,
    dir: &String,
) {
    let relative_improvements = logs
        .iter()
        .map(|log| {
            log.iter()
                .map(|log| {
                    (log.initial_objective_value - log.final_objective_value)
                        / log.initial_objective_value
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    let mut rel_improvements_10 = relative_improvements[2]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, f64)>>();

    rel_improvements_10.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut data = vec![vec![]; NUM_VERTICES.len()];
    for i in 0..NUM_VERTICES.len() {
        let rel_imps = &relative_improvements[i];
        rel_improvements_10.iter().for_each(|(index, _)| {
            data[i].push(rel_imps[*index]);
        });
    }

    let chart_dir = format!(
        "{}/{}",
        NUM_VERTICES_CHARTS_DIR,
        alpha.to_string().replace('.', "_")
    );
    let name = format!("{}_cmp_obj.png", dir);

    build_chart_plot_rel_improvement_per_instance(
        &chart_dir,
        &name,
        rel_improvements_10.len(),
        &data,
        &COLORS.to_vec(),
        &NUM_VERTICES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
}

fn evaluate_per_instance_when_convergence(
    logs: &Vec<Vec<SimulatedAnnealingLogger>>,
    dir: &String,
    alpha: f64,
) {
    let converged_iterations = get_converged_iteration(logs);

    let mut convergence_10 = converged_iterations[2]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, usize)>>();

    convergence_10.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut data = vec![vec![]; NUM_VERTICES.len()];
    for i in 0..NUM_VERTICES.len() {
        let converged_iterations = &converged_iterations[i];
        convergence_10.iter().for_each(|(index, _)| {
            data[i].push(converged_iterations[*index]);
        });
    }

    let chart_dir = format!(
        "{}/{}",
        NUM_VERTICES_CHARTS_DIR,
        alpha.to_string().replace('.', "_")
    );
    let name = format!("{}_cmp_conv.png", dir);

    build_chart_usize_per_instance(
        &chart_dir,
        &name,
        convergence_10.len(),
        &data,
        &Vec::new(),
        &COLORS.to_vec(),
        &NUM_VERTICES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Convergence iteration".to_string(),
        SeriesLabelPosition::MiddleRight,
        RGBColor(255, 0, 0),
    );
}
