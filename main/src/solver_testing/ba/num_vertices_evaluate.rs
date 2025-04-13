use std::fs::DirEntry;

use plotters::style::RGBColor;
use solver::SimulatedAnnealingLogger;

use super::{
    build_chart_plot_rel_improvement_per_instance, build_chart_process, NUM_VERTICES,
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

    let log = num_vertices_logs[0][0].clone();

    build_chart_process(
        &"./ba/charts".to_string(),
        &"test.png".to_string(),
        &log.current_best_costs
            .iter()
            .map(|x| vec![x.clone() as f32])
            .collect::<Vec<Vec<f32>>>(),
        None,
        "obj value",
        log.max_iterations,
    );

    //evaluate_per_instance_relative_improvement(&num_vertices_logs, alpha, dir);
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
    let name = format!("{}_cmp_num_vert.png", dir);

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
}
