use std::fs::DirEntry;

use plotters::{chart::SeriesLabelPosition, style::RGBColor};
use solver::{SimulatedAnnealingLogger, TimeDelta};

use super::{
    build_chart_conv_time_improvement, build_chart_plot_rel_improvement_per_instance,
    build_chart_time_converges_alpha, build_chart_usize_per_instance, get_converged_iteration,
    get_time_converges, ALPHA_SOLUTIONS_DIR, ALPHA_VALUES,
};

const ALPHA_CHARTS_DIR: &str = "./ba/charts/alpha";

const COLORS: [RGBColor; 5] = [
    RGBColor(66, 212, 244),
    RGBColor(245, 130, 49),
    RGBColor(0, 0, 0),
    RGBColor(60, 180, 75),
    RGBColor(145, 30, 180),
];

pub fn evaluate_alpha(dir_name: &String) {
    let mut alpha_logs = vec![vec![]; ALPHA_VALUES.len()];
    for (index, alpha) in ALPHA_VALUES.iter().enumerate() {
        let solutions_dir = format!(
            "{}/{}/{}",
            ALPHA_SOLUTIONS_DIR,
            alpha.to_string().replace('.', "_"),
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

            alpha_logs[index].push(logs);
        }
    }

    //evaluate_convergence(&alpha_logs, dir_name);

    //evaluate_per_instance_when_converged(&alpha_logs, dir_name);
    evaluate_per_instance_relative_improvement(&alpha_logs, dir_name);
}

fn evaluate_convergence(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir: &String) {
    let converged = get_converged_iteration(logs);

    let converged = converged
        .iter()
        .enumerate()
        .map(|(func_index, x)| {
            x.iter()
                .enumerate()
                .map(|(index, y)| {
                    get_time_converges(&logs[func_index][index], *y).num_milliseconds() as f64
                        / 1000.0
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    let name = format!("{}_cmp_time.svg", dir);

    build_chart_time_converges_alpha(
        &ALPHA_CHARTS_DIR.to_string(),
        &name,
        &converged,
        &ALPHA_VALUES.to_vec(),
    );
}

fn evaluate_per_instance_when_converged(logs: &Vec<Vec<SimulatedAnnealingLogger>>, dir: &String) {
    let converged_iterations = get_converged_iteration(logs);

    let mut convergence_0_1 = converged_iterations[0]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, usize)>>();

    convergence_0_1.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut data = vec![vec![]; ALPHA_VALUES.len()];
    for i in 0..ALPHA_VALUES.len() {
        let converged_iterations = &converged_iterations[i];
        convergence_0_1.iter().for_each(|(index, _)| {
            data[i].push(converged_iterations[*index]);
        });
    }

    let name = format!("{}_cmp_conv.png", dir);

    build_chart_usize_per_instance(
        &ALPHA_CHARTS_DIR.to_string(),
        &name,
        convergence_0_1.len(),
        &data,
        &Vec::new(),
        &COLORS.to_vec(),
        &ALPHA_VALUES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        &"Convergence iteration".to_string(),
        SeriesLabelPosition::MiddleRight,
        RGBColor(255, 0, 0),
    );
}

fn evaluate_per_instance_relative_improvement(
    logs: &Vec<Vec<SimulatedAnnealingLogger>>,
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

    let mut rel_improvements_0_1 = relative_improvements[0]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, f64)>>();

    rel_improvements_0_1.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut data = vec![vec![]; ALPHA_VALUES.len()];
    for i in 0..ALPHA_VALUES.len() {
        let rel_imps = &relative_improvements[i];
        rel_improvements_0_1.iter().for_each(|(index, _)| {
            data[i].push(rel_imps[*index]);
        });
    }

    let name = format!("{}_cmp_alphas.svg", dir);

    build_chart_plot_rel_improvement_per_instance(
        &ALPHA_CHARTS_DIR.to_string(),
        &name,
        rel_improvements_0_1.len(),
        &data,
        &COLORS.to_vec(),
        &ALPHA_VALUES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
}
