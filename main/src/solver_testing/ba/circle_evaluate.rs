use std::{collections::HashMap, fs::DirEntry};

use plotters::{chart::SeriesLabelPosition, style::RGBColor};
use solver::SimulatedAnnealingLogger;

use super::{
    build_chart_best_found, build_chart_node_improvement, CIRCLE_ALPHA_VALUES, CIRCLE_SOLUTIONS_DIR,
};

const CIRCLE_OPT_FILE: &str = "./circle/circles_opt.csv";

const CIRCLE_CHARTS_DIR: &str = "./ba/charts/circle";

const COLORS: [RGBColor; 5] = [
    RGBColor(66, 212, 244),
    RGBColor(245, 130, 49),
    RGBColor(0, 0, 0),
    RGBColor(60, 180, 75),
    RGBColor(145, 30, 180),
];

const ALPHA_VALUES: [f64; 5] = [0.1, 0.3, 0.5, 0.7, 0.9];

pub fn circle_evaluate() {
    let opt = load_opt();

    let mut circle_logs = vec![vec![]; CIRCLE_ALPHA_VALUES.len()];
    for (index, alpha) in CIRCLE_ALPHA_VALUES.iter().enumerate() {
        let solutions_dir = format!(
            "{}/{}",
            CIRCLE_SOLUTIONS_DIR,
            alpha.to_string().replace('.', "_")
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
            let file_stem = file_path.file_stem().unwrap().to_str().unwrap().to_string();
            let num_nodes = file_stem
                .split('_')
                .skip(1)
                .next()
                .unwrap()
                .replace("n", "");

            circle_logs[index].push((num_nodes, logs));
        }
    }

    evaluate_rel_correct(&circle_logs, &opt);
    //evaluate_correct_calculated(&circle_logs, &opt);
}

fn evaluate_rel_correct(
    logs: &Vec<Vec<(String, SimulatedAnnealingLogger)>>,
    opt: &HashMap<String, Vec<f64>>,
) {
    let mut node_amounts = opt
        .keys()
        .map(|key| key.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    node_amounts.sort();

    let mut data = vec![vec![]; CIRCLE_ALPHA_VALUES.len()];
    for (index, _) in CIRCLE_ALPHA_VALUES.iter().enumerate() {
        for (num_nodes, log) in logs[index].iter() {
            let opt_value = opt.get(num_nodes).unwrap()[index];
            let best_value = log.final_objective_value;
            data[index].push((num_nodes.parse::<usize>().unwrap(), best_value / opt_value));
        }
    }

    let data = vec![
        data[1].clone(),
        data[3].clone(),
        data[5].clone(),
        data[7].clone(),
        data[9].clone(),
    ];

    build_chart_node_improvement(
        &CIRCLE_CHARTS_DIR.to_string(),
        &"rel_found.svg".to_string(),
        &data,
        &node_amounts,
        &COLORS.to_vec(),
        &ALPHA_VALUES
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        SeriesLabelPosition::MiddleRight,
    );
}

fn evaluate_correct_calculated(
    logs: &Vec<Vec<(String, SimulatedAnnealingLogger)>>,
    opt: &HashMap<String, Vec<f64>>,
) {
    let mut node_amounts = opt
        .keys()
        .map(|key| key.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    node_amounts.sort();

    let mut data = vec![vec![false; CIRCLE_ALPHA_VALUES.len()]; logs[0].len()];
    for (index, _) in CIRCLE_ALPHA_VALUES.iter().enumerate() {
        for (num_nodes, log) in logs[index].iter() {
            let opt_value = opt.get(num_nodes).unwrap()[index];
            let best_value = log.final_objective_value;
            if float_equal(opt_value, best_value) {
                let i = node_amounts
                    .iter()
                    .position(|&x| x == num_nodes.parse::<usize>().unwrap())
                    .unwrap();
                data[i][index] = true;
            }
        }
    }

    build_chart_best_found(
        &CIRCLE_CHARTS_DIR.to_string(),
        &"best_found.png".to_string(),
        &node_amounts,
        &CIRCLE_ALPHA_VALUES.to_vec(),
        &data,
        &COLORS.to_vec(),
        &vec!["Found".to_string(), "Not Found".to_string()],
    );
}

fn float_equal(a: f64, b: f64) -> bool {
    let epsilon = 1e-6;
    (a - b).abs() < epsilon
}

fn load_opt() -> HashMap<String, Vec<f64>> {
    let mut rdr = csv::Reader::from_reader(std::fs::File::open(CIRCLE_OPT_FILE).unwrap());
    let headers = rdr
        .headers()
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut opt: HashMap<String, Vec<f64>> = HashMap::new();
    for record in rdr.records() {
        let record = record.unwrap();
        for (index, head) in headers.iter().enumerate().skip(1) {
            let value = record.get(index).unwrap().parse::<f64>().unwrap();
            if let Some(v) = opt.get_mut(head) {
                v.push(value);
            } else {
                opt.insert(head.to_string(), vec![value]);
            }
        }
    }
    return opt;
}
