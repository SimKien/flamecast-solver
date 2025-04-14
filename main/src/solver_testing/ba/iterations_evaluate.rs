use std::fs::DirEntry;

use plotters::style::RGBColor;
use solver::SimulatedAnnealingLogger;

use super::{
    build_chart_plot_rel_improvement_per_instance, ITERATIONS_MULTIPLIERS, ITERATIONS_SOLUTIONS_DIR,
};

const ITERATIONS_CHARTS_DIR: &str = "./ba/charts/iterations";

const COLORS: [RGBColor; 5] = [
    RGBColor(66, 212, 244),
    RGBColor(245, 130, 49),
    RGBColor(0, 0, 0),
    RGBColor(60, 180, 75),
    RGBColor(145, 30, 180),
];

pub fn iterations_evaluate(dir_name: &String, alpha: f64) {
    let mut iterations_logs = vec![vec![]; ITERATIONS_MULTIPLIERS.len()];
    for (index, multiplier) in ITERATIONS_MULTIPLIERS.iter().enumerate() {
        let solutions_dir = format!(
            "{}/{}/{}/{}",
            ITERATIONS_SOLUTIONS_DIR,
            alpha.to_string().replace('.', "_"),
            multiplier.to_string().replace(".", "_"),
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

            iterations_logs[index].push(logs);
        }
    }

    evaluate_per_instance_relative_improvement(&iterations_logs, alpha, dir_name);
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

    let mut rel_improvements_2 = relative_improvements[1]
        .iter()
        .enumerate()
        .map(|(index, x)| (index, *x))
        .collect::<Vec<(usize, f64)>>();

    rel_improvements_2.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut data = vec![vec![]; ITERATIONS_MULTIPLIERS.len()];
    for i in 0..ITERATIONS_MULTIPLIERS.len() {
        let rel_imps = &relative_improvements[i];
        rel_improvements_2.iter().for_each(|(index, _)| {
            data[i].push(rel_imps[*index]);
        });
    }

    let chart_dir = format!(
        "{}/{}",
        ITERATIONS_CHARTS_DIR,
        alpha.to_string().replace('.', "_")
    );
    let name = format!("{}_cmp_iter.png", dir);

    build_chart_plot_rel_improvement_per_instance(
        &chart_dir,
        &name,
        rel_improvements_2.len(),
        &data,
        &COLORS.to_vec(),
        &ITERATIONS_MULTIPLIERS
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
}
