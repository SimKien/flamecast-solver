use plotters::{
    chart::ChartBuilder,
    data::{fitting_range, Quartiles},
    prelude::{BitMapBackend, Boxplot, IntoDrawingArea, IntoSegmentedCoord, Polygon, SegmentValue},
    series::{Histogram, LineSeries},
    style::{Color, IntoFont, ShapeStyle, BLACK, WHITE},
};

pub fn build_chart_num_nodes(
    dir: &String,
    name: &String,
    data: &Vec<Vec<usize>>,
    max_iteration: usize,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = (0.1 * (value_range.end - value_range.start) as f64).round() as usize;

    let mut chart = ChartBuilder::on(&root)
        .caption("Progress", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(
            (0..max_iteration).into_segmented(),
            value_range.start - padding..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("sans-serif", 20).into_font())
        .y_label_style(("sans-serif", 20).into_font())
        .x_desc("Iteration")
        .y_desc("Number of Nodes")
        .draw()
        .unwrap();

    let mut lows = Vec::new();
    let mut medians = Vec::new();
    let mut ups = Vec::new();
    let mut bounds = Vec::new();
    for (index, data_set) in data.iter().enumerate() {
        let quartil = Quartiles::new(&data_set.iter().map(|&x| x as f64).collect::<Vec<f64>>());
        let [_, low, median, up, _] = quartil.values();
        lows.push((index.into(), low.round() as usize));
        medians.push((index.into(), median.round() as usize));
        ups.push((index.into(), up.round() as usize));
    }
    ups.reverse();
    bounds.append(&mut lows);
    bounds.append(&mut ups);

    chart
        .draw_series(vec![Polygon::new(
            bounds,
            ShapeStyle::from(&BLACK.mix(0.5)).filled(),
        )])
        .unwrap();
    chart.draw_series(LineSeries::new(medians, &BLACK)).unwrap();
}

pub fn build_chart_improve_per_operation(
    dir: &String,
    name: &String,
    data: &Vec<Vec<f32>>,
    operation_names: &Vec<String>,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .caption("Improvement per Operation", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(
            operation_names[..].into_segmented(),
            0.0..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .x_label_style(("sans-serif", 20).into_font())
        .y_label_style(("sans-serif", 20).into_font())
        .x_desc("Neighborhood Operations")
        .y_desc("Improvement")
        .draw()
        .unwrap();

    let mut res = Vec::new();
    for i in 0..operation_names.len() {
        let quartil = Quartiles::new(&data[i]);
        res.push(quartil.values()[2]);
    }

    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(BLACK.mix(0.8).filled())
                .data(
                    operation_names
                        .iter()
                        .enumerate()
                        .map(|(index, x)| (x, res[index])),
                ),
        )
        .unwrap();
}

pub fn build_chart_compare_operations(
    dir: &String,
    name: &String,
    data: &Vec<Vec<usize>>,
    operation_names: &Vec<String>,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = (0.1 * ((value_range.end - value_range.start) as f64)) as usize;

    let mut chart = ChartBuilder::on(&root)
        .caption("Executed Operations", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(
            operation_names[..].into_segmented(),
            0..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .x_label_style(("sans-serif", 20).into_font())
        .y_label_style(("sans-serif", 20).into_font())
        .x_desc("Neighborhood Operations")
        .y_desc("Number of Executions")
        .draw()
        .unwrap();

    let mut res = Vec::new();
    for i in 0..operation_names.len() {
        let quartil = Quartiles::new(&data[i].iter().map(|&x| x as f64).collect::<Vec<f64>>());
        res.push(quartil.values()[2] as usize);
    }

    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(BLACK.mix(0.8).filled())
                .data(
                    operation_names
                        .iter()
                        .enumerate()
                        .map(|(index, x)| (x, res[index])),
                ),
        )
        .unwrap();
}

pub fn build_chart_process(
    dir: &String,
    name: &String,
    data: &Vec<Vec<f32>>,
    data_secondary: Option<&Vec<Vec<f32>>>,
    data_name: &str,
    max_iteration: usize,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .caption("Progress", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(
            (0..max_iteration).into_segmented(),
            value_range.start - padding..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("sans-serif", 20).into_font())
        .y_label_style(("sans-serif", 20).into_font())
        .x_desc("Iteration")
        .y_desc(data_name)
        .draw()
        .unwrap();

    if let Some(secondary) = data_secondary {
        let mut lows = Vec::new();
        let mut medians = Vec::new();
        let mut ups = Vec::new();
        let mut bounds = Vec::new();
        for (index, data_set) in secondary.iter().enumerate() {
            let quartil = Quartiles::new(data_set);
            let [_, low, median, up, _] = quartil.values();
            lows.push((index.into(), low));
            medians.push((index.into(), median));
            ups.push((index.into(), up));
        }
        ups.reverse();
        bounds.append(&mut lows);
        bounds.append(&mut ups);

        chart
            .draw_series(vec![Polygon::new(
                bounds,
                ShapeStyle::from(&BLACK.mix(0.25)).filled(),
            )])
            .unwrap();
        chart
            .draw_series(LineSeries::new(
                medians,
                ShapeStyle::from(&BLACK.mix(0.8)).filled(),
            ))
            .unwrap();
    }

    let mut lows = Vec::new();
    let mut medians = Vec::new();
    let mut ups = Vec::new();
    let mut bounds = Vec::new();
    for (index, data_set) in data.iter().enumerate() {
        let quartil = Quartiles::new(data_set);
        let [_, low, median, up, _] = quartil.values();
        lows.push((index.into(), low));
        medians.push((index.into(), median));
        ups.push((index.into(), up));
    }
    ups.reverse();
    bounds.append(&mut lows);
    bounds.append(&mut ups);

    chart
        .draw_series(vec![Polygon::new(
            bounds,
            ShapeStyle::from(&BLACK.mix(0.5)).filled(),
        )])
        .unwrap();
    chart.draw_series(LineSeries::new(medians, &BLACK)).unwrap();
}

pub fn build_chart_no_process(
    dir: &String,
    name: &String,
    data: &Vec<Vec<f32>>,
    data_name: &str,
    alphas: &Vec<f64>,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .caption("Compare Alpha Values", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(alphas[..].into_segmented(), 0.0..value_range.end + padding)
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .disable_x_mesh()
        .x_label_style(("sans-serif", 20).into_font())
        .y_label_style(("sans-serif", 20).into_font())
        .x_desc("Alpha Value")
        .y_desc(data_name)
        .draw()
        .unwrap();

    let mut boxplots = Vec::new();
    for (index, data_set) in data.iter().enumerate() {
        boxplots.push(Boxplot::new_vertical(
            SegmentValue::CenterOf(&alphas[index]),
            &Quartiles::new(data_set),
        ));
    }

    chart.draw_series(boxplots).unwrap();
}

pub fn build_chart_compare_init_functions(
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
        .caption("Comparison Matching-Random", ("sans-serif", 50).into_font())
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
        .disable_x_mesh()
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
