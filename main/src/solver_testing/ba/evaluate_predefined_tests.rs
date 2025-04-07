use std::fs;

use plotters::{
    chart::ChartBuilder,
    data::{fitting_range, Quartiles},
    prelude::{BitMapBackend, Boxplot, IntoDrawingArea, IntoSegmentedCoord, SegmentValue},
    style::{IntoFont, WHITE},
};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use solver::SimulatedAnnealingLogger;

use super::{ALPHA_FILE, PREDEFINED_SOLUTIONS_BASE_DIR};

const CHARTS_DIR: &str = "./ba/predefined_charts";
const COMPARE_DIR: &str = "compare";
const HISTORY_DIR: &str = "history";

pub fn evaluate_tests(
    instances_dir: &Vec<String>,
    alpha: Option<f64>,
    init_functions: &Vec<String>,
) {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    if instances_dir.is_empty() || init_functions.is_empty() {
        println!("No instance directory or init functions provided.");
        return;
    }

    let alphas = if alpha.is_none() {
        let alpha_file = fs::read_to_string(ALPHA_FILE);
        if alpha_file.is_err() {
            println!("Failed to read alpha file: {:?}", ALPHA_FILE);
            return;
        }
        let alpha_file = alpha_file.unwrap();
        let alpha_value = serde_json::from_str::<Vec<f64>>(&alpha_file);
        if alpha_value.is_err() {
            println!("Failed to parse alpha JSON: {:?}", ALPHA_FILE);
            return;
        }
        alpha_value.unwrap()
    } else {
        vec![alpha.unwrap()]
    };

    for dir in instances_dir.iter() {
        match init_functions.len() {
            1 => {
                println!("Plot history");
            }
            _ => {
                alphas.par_iter().for_each(|alpha| {
                    let alpha_str = alpha.to_string().replace(".", "_");
                    let matching_dir = format!(
                        "{}/matching/{}/{}",
                        PREDEFINED_SOLUTIONS_BASE_DIR, alpha_str, dir
                    );
                    let random_dir = format!(
                        "{}/random/{}/{}",
                        PREDEFINED_SOLUTIONS_BASE_DIR, alpha_str, dir
                    );
                    let instance_matching_files = std::fs::read_dir(matching_dir)
                        .expect("Failed to read directory")
                        .map(|entry| entry.expect("Failed to read entry"))
                        .collect::<Vec<_>>();
                    let instance_random_files = std::fs::read_dir(random_dir)
                        .expect("Failed to read directory")
                        .map(|entry| entry.expect("Failed to read entry"))
                        .collect::<Vec<_>>();

                    let matching_results = instance_matching_files
                        .iter()
                        .map(|entry| {
                            let path = entry.path();
                            let file = fs::read_to_string(path.clone());
                            let file = file.unwrap();
                            let instance = serde_json::from_str::<SimulatedAnnealingLogger>(&file);
                            let instance = instance.unwrap();
                            return instance.final_objective_value as f32;
                        })
                        .collect::<Vec<f32>>();
                    let random_results = instance_random_files
                        .iter()
                        .map(|entry| {
                            let path = entry.path();
                            let file = fs::read_to_string(path.clone());
                            let file = file.unwrap();
                            let instance = serde_json::from_str::<SimulatedAnnealingLogger>(&file);
                            let instance = instance.unwrap();
                            return instance.final_objective_value as f32;
                        })
                        .collect::<Vec<f32>>();
                    let result_dir = format!("{}/{}/{}", CHARTS_DIR, COMPARE_DIR, dir);
                    build_chart_compare(
                        &result_dir,
                        &alpha_str,
                        &random_results,
                        &matching_results,
                        vec!["Matching".to_string(), "Random".to_string()],
                    );
                });
            }
        }
    }
}

pub fn build_chart_compare(
    dir: &String,
    instance: &String,
    data_random: &Vec<f32>,
    data_matching: &Vec<f32>,
    init_functions: Vec<String>,
) {
    let file_name = format!("{}/{}_compare.png", dir, instance);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let value_range = fitting_range(data_random.iter().chain(data_matching.iter()));
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .caption("Comparison Matching/Random", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(
            init_functions[..].into_segmented(),
            0.0..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("sans-serif", 20).into_font())
        .y_label_style(("sans-serif", 20).into_font())
        .x_desc("Initial Function")
        .y_desc("Objective Value")
        .draw()
        .unwrap();

    let quartiles_0 = Quartiles::new(&data_matching);
    let quartiles_1 = Quartiles::new(&data_random);

    chart
        .draw_series(vec![
            Boxplot::new_vertical(SegmentValue::CenterOf(&init_functions[0]), &quartiles_0),
            Boxplot::new_vertical(SegmentValue::CenterOf(&init_functions[1]), &quartiles_1),
        ])
        .unwrap();
}
