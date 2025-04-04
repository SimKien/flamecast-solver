use clap::Args;
use solver::{
    generate_flamecast_instance, generate_random_flamecast_test_instance, InitialSolutionFunction,
    Stopwatch,
};

use crate::solver_testing::INSTANCES;

#[derive(Args)]
pub struct SizeArgs {
    #[arg(short, long, value_parser = instance_in_range)]
    pub instance: Option<usize>,

    #[arg(short, long, default_value_t = false)]
    pub time: bool,
}

pub fn process_size(args: SizeArgs) {
    match args.instance {
        Some(instance) => {
            println!(
                "Starting neighborhood size calculation for instance {}",
                instance
            );

            let instance = INSTANCES[instance].clone();
            let flamecast_test_instance = generate_random_flamecast_test_instance(
                instance[0],
                instance[1],
                instance[2],
                true,
            );
            let instance = generate_flamecast_instance(
                flamecast_test_instance.alpha,
                flamecast_test_instance.num_layers,
                flamecast_test_instance.capacities,
                flamecast_test_instance.sources_drains_embeddings,
                InitialSolutionFunction::Random,
            );

            let stop_watch = if args.time {
                Some(Stopwatch::new())
            } else {
                None
            };

            println!(
                "Neighborhood size: {}",
                instance.get_all_possible_neighbors().len()
            );

            if let Some(stop_watch) = stop_watch {
                println!(
                    "Time needed for neighborhood calculation: {}",
                    stop_watch.elapsed_pretty()
                );
            }
        }
        None => {
            println!("Currently only predefined instances are supported");
        }
    }
}

fn instance_in_range(s: &str) -> Result<usize, String> {
    let instance = s
        .parse::<usize>()
        .map_err(|_| format!("`{}` isn't a valid number", s))?;
    if instance >= INSTANCES.len() {
        Err(format!(
            "Instance number must be less than the number of instances ({})",
            INSTANCES.len()
        ))
    } else {
        Ok(instance)
    }
}
