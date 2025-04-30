use plotters::{
    chart::{ChartBuilder, SeriesLabelPosition},
    data::{fitting_range, Quartiles},
    prelude::{
        BitMapBackend, Boxplot, Circle, IntoDrawingArea, IntoLogRange, IntoSegmentedCoord,
        SVGBackend, SegmentValue,
    },
    style::{Color, IntoFont, RGBColor, ShapeStyle, BLACK, WHITE},
};

pub fn build_chart_time_converges_alpha(
    dir: &String,
    name: &String,
    data: &Vec<Vec<f64>>,
    labels: &Vec<f64>,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = SVGBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(140)
        .build_cartesian_2d(
            labels.into_segmented(),
            0.0..(value_range.end + padding) as f32,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("arial", 46).into_font())
        .y_label_style(("arial", 46).into_font())
        .x_desc("Alpha values")
        .y_desc("Time (s)")
        .draw()
        .unwrap();

    for (index, data) in data.iter().enumerate() {
        let boxplot = Boxplot::new_vertical(
            SegmentValue::CenterOf(&labels[index]),
            &Quartiles::new(data),
        );
        chart.draw_series(vec![boxplot]).unwrap();
    }

    root.present().unwrap();
}

pub fn build_chart_iteration_converges_vertices(
    dir: &String,
    name: &String,
    data: &Vec<Vec<f64>>,
    labels: &Vec<usize>,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(140)
        .build_cartesian_2d(
            labels.into_segmented(),
            0.0..(value_range.end + padding) as f32,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("arial", 46).into_font())
        .y_label_style(("arial", 46).into_font())
        .x_desc("Number of vertices")
        .y_desc("Iterations done (%)")
        .draw()
        .unwrap();

    for (index, data) in data.iter().enumerate() {
        let boxplot = Boxplot::new_vertical(
            SegmentValue::CenterOf(&labels[index]),
            &Quartiles::new(data),
        );
        chart.draw_series(vec![boxplot]).unwrap();
    }

    root.present().unwrap();
}

pub fn build_chart_time_converges_vertices(
    dir: &String,
    name: &String,
    data: &Vec<Vec<f64>>,
    labels: &Vec<usize>,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(140)
        .build_cartesian_2d(
            labels.into_segmented(),
            0.0..(value_range.end + padding) as f32,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("arial", 46).into_font())
        .y_label_style(("arial", 46).into_font())
        .x_desc("Number of vertices")
        .y_desc("Time (s)")
        .draw()
        .unwrap();

    for (index, data) in data.iter().enumerate() {
        let boxplot = Boxplot::new_vertical(
            SegmentValue::CenterOf(&labels[index]),
            &Quartiles::new(data),
        );
        chart.draw_series(vec![boxplot]).unwrap();
    }

    root.present().unwrap();
}

pub fn build_chart_node_improvement(
    dir: &String,
    name: &String,
    data: &Vec<Vec<(usize, f64)>>,
    nodes: &Vec<usize>,
    colors: &Vec<RGBColor>,
    labels: &Vec<String>,
    position: SeriesLabelPosition,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = SVGBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    //root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten().map(|x| x.1).collect::<Vec<f64>>();

    let value_range = fitting_range(&all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(140)
        .build_cartesian_2d(
            (1..nodes[nodes.len() - 1]).log_scale(),
            1.0..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("arial", 41).into_font())
        .y_label_style(("arial", 41).into_font())
        .x_desc("Number of Sources")
        .y_desc("Deviation")
        .draw()
        .unwrap();

    for (index, data) in data.iter().enumerate() {
        let points = data
            .iter()
            .map(|x| (x.0, x.1))
            .collect::<Vec<(usize, f64)>>();

        let circles = points
            .iter()
            .map(|x| Circle::new(x.clone(), 4, ShapeStyle::from(colors[index]).filled()));
        let color = colors[index].to_rgba();
        chart
            .draw_series(circles)
            .unwrap()
            .label(labels[index].as_str())
            .legend(move |(x, y)| Circle::new((x, y), 8, ShapeStyle::from(color).filled()));
    }

    chart
        .configure_series_labels()
        .position(position)
        .label_font(("arial", 41).into_font())
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}

pub fn build_chart_conv_time_improvement(
    dir: &String,
    name: &String,
    data: &Vec<Vec<(f64, f64)>>,
    colors: &Vec<RGBColor>,
    labels: &Vec<String>,
    position: SeriesLabelPosition,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_x_iter = data.iter().flatten().map(|x| x.0).collect::<Vec<f64>>();

    let x_value_range = fitting_range(&all_data_x_iter);
    let x_padding = 0.1 * (x_value_range.end - x_value_range.start);

    let all_data_y_iter = data
        .iter()
        .flatten()
        .into_iter()
        .map(|x| x.1)
        .collect::<Vec<f64>>();
    let y_value_range = fitting_range(&all_data_y_iter);

    let y_padding = 0.1 * (y_value_range.end - y_value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(140)
        .build_cartesian_2d(
            0.0..x_value_range.end + x_padding,
            0.0..y_value_range.end + y_padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("arial", 50).into_font())
        .y_label_style(("arial", 50).into_font())
        .x_desc("Time (s)")
        .y_desc("Relative Improvement")
        .draw()
        .unwrap();

    for (index, data) in data.iter().enumerate() {
        let points = data.iter().map(|x| (x.0, x.1)).collect::<Vec<(f64, f64)>>();

        let circles = points
            .iter()
            .map(|x| Circle::new(x.clone(), 2, ShapeStyle::from(colors[index]).filled()));
        let color = colors[index].to_rgba();
        chart
            .draw_series(circles)
            .unwrap()
            .label(labels[index].as_str())
            .legend(move |(x, y)| Circle::new((x, y), 8, ShapeStyle::from(color).filled()));
    }

    chart
        .configure_series_labels()
        .position(position)
        .label_font(("arial", 50).into_font())
        .background_style(WHITE.to_rgba())
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}

// Funtion for circles
pub fn build_chart_best_found(
    dir: &String,
    name: &String,
    circle_nodes: &Vec<usize>,
    alphas: &Vec<f64>,
    data: &Vec<Vec<bool>>,
    colors: &Vec<RGBColor>,
    labels: &Vec<String>,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(100)
        .build_cartesian_2d(circle_nodes.into_segmented(), alphas.into_segmented())
        .unwrap();

    chart
        .configure_mesh()
        .x_label_style(("arial", 41).into_font())
        .y_label_style(("arial", 41).into_font())
        .x_desc("Source Nodes")
        .y_desc("Alpha Value")
        .draw()
        .unwrap();

    let mut circles_true = Vec::new();
    let mut circles_false = Vec::new();

    data.iter().enumerate().for_each(|(index, data)| {
        data.iter().enumerate().for_each(|(alpha_index, x)| {
            if *x {
                circles_true.push(Circle::new(
                    (
                        SegmentValue::CenterOf(&circle_nodes[index]),
                        SegmentValue::CenterOf(&alphas[alpha_index]),
                    ),
                    2,
                    ShapeStyle::from(colors[0].to_rgba()).filled(),
                ));
            } else {
                circles_false.push(Circle::new(
                    (
                        SegmentValue::CenterOf(&circle_nodes[index]),
                        SegmentValue::CenterOf(&alphas[alpha_index]),
                    ),
                    2,
                    ShapeStyle::from(colors[1].to_rgba()).filled(),
                ));
            };
        });
    });

    chart
        .draw_series(circles_true)
        .unwrap()
        .label(labels[0].as_str())
        .legend(|(x, y)| Circle::new((x, y), 8, ShapeStyle::from(colors[0].to_rgba()).filled()));
    chart
        .draw_series(circles_false)
        .unwrap()
        .label(labels[1].as_str())
        .legend(|(x, y)| Circle::new((x, y), 8, ShapeStyle::from(colors[1].to_rgba()).filled()));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .label_font(("arial", 41).into_font())
        .background_style(WHITE.to_rgba())
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}

// Function for init, num_vertices, alpha, iterations
pub fn build_chart_usize_per_instance(
    dir: &String,
    name: &String,
    num_instances: usize,
    data: &Vec<Vec<usize>>,
    max_iterations: &Vec<Vec<usize>>,
    colors: &Vec<RGBColor>,
    labels: &Vec<String>,
    data_name: &String,
    position: SeriesLabelPosition,
    max_iterations_color: RGBColor,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = (0.1 * ((value_range.end - value_range.start) as f64)).round() as usize;

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(140)
        .build_cartesian_2d(
            (0..num_instances).into_segmented(),
            0..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("arial", 50).into_font())
        .y_label_style(("arial", 50).into_font())
        .x_desc("Instance")
        .y_desc(data_name)
        .draw()
        .unwrap();

    for (index, data) in data.iter().enumerate() {
        let points = data
            .iter()
            .enumerate()
            .map(|(index, x)| (index.into(), *x))
            .collect::<Vec<(SegmentValue<usize>, usize)>>();
        let max_iterations = if max_iterations.len() > 0 {
            max_iterations[index]
                .iter()
                .enumerate()
                .map(|(index, x)| (index.into(), *x))
                .collect::<Vec<(SegmentValue<usize>, usize)>>()
        } else {
            vec![]
        };

        let circles = points
            .iter()
            .map(|x| Circle::new(x.clone(), 2, ShapeStyle::from(colors[index]).filled()));
        let circles_max_iterations = if max_iterations.len() > 0 {
            max_iterations
                .iter()
                .map(|x| {
                    Circle::new(
                        x.clone(),
                        2,
                        ShapeStyle::from(max_iterations_color).filled(),
                    )
                })
                .collect()
        } else {
            vec![]
        };
        let color = colors[index].to_rgba();
        chart
            .draw_series(circles)
            .unwrap()
            .label(labels[index].as_str())
            .legend(move |(x, y)| Circle::new((x, y), 8, ShapeStyle::from(color).filled()));
        if max_iterations.len() > 0 {
            chart
                .draw_series(circles_max_iterations)
                .unwrap()
                .label("Finished Iteration")
                .legend(move |(x, y)| {
                    Circle::new((x, y), 8, ShapeStyle::from(max_iterations_color).filled())
                });
        }
    }

    chart
        .configure_series_labels()
        .position(position)
        .label_font(("arial", 50).into_font())
        .background_style(WHITE.to_rgba())
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}

// Function for init
pub fn build_chart_init_obj_per_instance(
    dir: &String,
    name: &String,
    num_instances: usize,
    data: &Vec<Vec<f64>>,
    colors: &Vec<RGBColor>,
    labels: &Vec<String>,
    data_name: &String,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = BitMapBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(140)
        .build_cartesian_2d(
            (0..num_instances).into_segmented(),
            0.0..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("arial", 50).into_font())
        .y_label_style(("arial", 50).into_font())
        .x_desc("Instance")
        .y_desc(data_name)
        .draw()
        .unwrap();

    for (index, data) in data.iter().enumerate() {
        let points = data
            .iter()
            .enumerate()
            .map(|(index, x)| (index.into(), *x))
            .collect::<Vec<(SegmentValue<usize>, f64)>>();

        let circles = points
            .iter()
            .map(|x| Circle::new(x.clone(), 2, ShapeStyle::from(colors[index]).filled()));
        let color = colors[index].to_rgba();
        chart
            .draw_series(circles)
            .unwrap()
            .label(labels[index].as_str())
            .legend(move |(x, y)| Circle::new((x, y), 8, ShapeStyle::from(color).filled()));
    }

    chart
        .configure_series_labels()
        .label_font(("arial", 50).into_font())
        .background_style(WHITE.to_rgba())
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}

// Function for num_vertices, alpha, iterations
pub fn build_chart_plot_rel_improvement_per_instance(
    dir: &String,
    name: &String,
    num_instances: usize,
    data: &Vec<Vec<f64>>,
    colors: &Vec<RGBColor>,
    labels: &Vec<String>,
) {
    let file_name = format!("{}/{}", dir, name);
    let root = SVGBackend::new(file_name.as_str(), (1024, 768)).into_drawing_area();
    //root.fill(&WHITE).unwrap();

    let all_data_iter = data.iter().flatten();

    let value_range = fitting_range(all_data_iter);
    let padding = 0.1 * (value_range.end - value_range.start);

    let mut chart = ChartBuilder::on(&root)
        .margin_left(5)
        .margin_bottom(5)
        .margin_right(24)
        .margin_top(10)
        .x_label_area_size(80)
        .y_label_area_size(140)
        .build_cartesian_2d(
            (0..num_instances).into_segmented(),
            0.0..value_range.end + padding,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_label_style(("arial", 50).into_font())
        .y_label_style(("arial", 50).into_font())
        .x_desc("Instance")
        .y_desc("Relative Improvement")
        .draw()
        .unwrap();

    for (index, data) in data.iter().enumerate() {
        let points = data
            .iter()
            .enumerate()
            .map(|(index, x)| (index.into(), *x as f64))
            .collect::<Vec<(SegmentValue<usize>, f64)>>();

        let circles = points
            .iter()
            .map(|x| Circle::new(x.clone(), 2, ShapeStyle::from(colors[index]).filled()));
        let color = colors[index].to_rgba();
        chart
            .draw_series(circles)
            .unwrap()
            .label(labels[index].as_str())
            .legend(move |(x, y)| Circle::new((x, y), 8, ShapeStyle::from(color).filled()));
    }

    chart
        .configure_series_labels()
        .label_font(("arial", 50).into_font())
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}

/*
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
*/
