use clap::Args;
use solver::CoolingSchedule;

use crate::solver_testing::test_custom_instance;

#[derive(Args)]
pub struct CustomArgs {
    /// Number of sources of the test instance
    #[arg(short, long, value_parser = sources_drains_valid)]
    pub sources: usize,

    /// Number of drains of the test instance
    #[arg(short, long, default_value_t = 1, value_parser = sources_drains_valid)]
    pub drains: usize,

    /// Number of layers of the test instance
    #[arg(short, long, default_value_t = 3, value_parser = layers_valid)]
    pub layers: usize,

    /// Alpha value of the test instance
    #[arg(short, long, value_parser = alpha_valid)]
    pub alpha: Option<f64>,

    /// Cooling operation/schedule of the test instance
    #[arg(short, long, default_value_t = String::from("lin"), value_parser = cooling_schedule_valid)]
    pub operation: String,

    /// Cooling alpha of the test instance
    #[arg(short, long, default_value_t = 0.002, value_parser = cooling_alpha_valid)]
    pub cooling_alpha: f64,

    /// initial temperature of the test instance
    #[arg(short, long, default_value_t = 1.0, value_parser = initial_temperature_valid)]
    pub initial_temperature: f64,

    /// max iterations of the test instance
    #[arg(short, long, default_value_t = 500, value_parser = max_iterations_valid)]
    pub max_iterations: usize,

    /// index of the test instance
    #[arg(long, default_value_t = 0, value_parser = index_valid)]
    pub index: usize,
}

pub fn process_run_custom_instances(args: CustomArgs) {
    if args.sources < args.drains {
        println!("Number of sources must be at least number of drains");
        return;
    }

    let iteration_cooling_option = (
        (
            CoolingSchedule::from_string(&args.operation, args.cooling_alpha).unwrap(),
            args.initial_temperature,
        ),
        args.max_iterations,
    );

    test_custom_instance(
        args.sources,
        args.drains,
        args.layers,
        args.alpha,
        iteration_cooling_option,
        args.index,
    );
}

fn sources_drains_valid(s: &str) -> Result<usize, String> {
    let num = s
        .parse::<usize>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if num < 1 {
        Err(format!(
            "Number of sources and drains must be at least 1 (Provided: {})",
            num
        ))
    } else {
        Ok(num)
    }
}

fn layers_valid(s: &str) -> Result<usize, String> {
    let num_layers = s
        .parse::<usize>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if num_layers < 2 {
        Err(format!(
            "Number of layers must be at least 2 (Provided: {})",
            num_layers
        ))
    } else {
        Ok(num_layers)
    }
}

fn alpha_valid(s: &str) -> Result<f64, String> {
    let alpha = s
        .parse::<f64>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if alpha < 0.0 {
        Err(format!(
            "Alpha must be greater than 0 (Provided: {})",
            alpha
        ))
    } else if alpha > 1.0 {
        Err(format!("Alpha must be less than 1 (Provided: {})", alpha))
    } else {
        Ok(alpha)
    }
}

fn cooling_schedule_valid(s: &str) -> Result<String, String> {
    match s {
        "lin" | "fas" | "exp" | "log" => Ok(s.to_string()),
        _ => Err(format!("{} isn't a valid cooling schedule", s)),
    }
}

fn cooling_alpha_valid(s: &str) -> Result<f64, String> {
    let alpha = s
        .parse::<f64>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if alpha <= 0.0 {
        Err(format!(
            "Alpha must be greater than 0 (Provided: {})",
            alpha
        ))
    } else {
        Ok(alpha)
    }
}

fn initial_temperature_valid(s: &str) -> Result<f64, String> {
    let init_temp = s
        .parse::<f64>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if init_temp <= 0.0 {
        Err(format!(
            "Initial temperature must be greater than 0 (Provided: {})",
            init_temp
        ))
    } else {
        Ok(init_temp)
    }
}

fn max_iterations_valid(s: &str) -> Result<usize, String> {
    let max_iterations = s
        .parse::<usize>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if max_iterations < 1 {
        Err(format!(
            "Max iterations must be greater than 0 (Provided: {})",
            max_iterations
        ))
    } else {
        Ok(max_iterations)
    }
}

fn index_valid(s: &str) -> Result<usize, String> {
    let index = s
        .parse::<usize>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    Ok(index)
}
