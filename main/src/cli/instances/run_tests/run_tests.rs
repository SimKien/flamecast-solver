use clap::{Parser, Subcommand};
use rayon::ThreadPoolBuilder;

use crate::cli::{
    process_run_custom_instances, process_run_imported_instances, process_run_predefined_instances,
};

use super::{CustomArgs, ImportedArgs, PredefinedArgs};

#[derive(Parser)]
pub struct RunTestsArgs {
    /// Number of threads to use for running tests.
    #[arg(short, long, value_parser = threads_in_range)]
    pub num_threads: Option<u16>,

    #[command(subcommand)]
    pub subcommand: RunTestsSubcommand,
}

#[derive(Subcommand)]
pub enum RunTestsSubcommand {
    /// Run tests with predefined test instances
    Predefined(PredefinedArgs),
    /// Run tests with user defined test instances
    Custom(CustomArgs),
    /// Run tests with imported test instances
    Imported(ImportedArgs),
}

pub fn process_run_tests(args: RunTestsArgs) {
    let num_threads = args.num_threads.unwrap_or(num_cpus::get() as u16);
    ThreadPoolBuilder::new()
        .num_threads(num_threads as usize)
        .build_global()
        .unwrap();
    println!("Running tests with {} threads...", num_threads);

    match args.subcommand {
        RunTestsSubcommand::Predefined(predefined_args) => {
            process_run_predefined_instances(predefined_args);
        }
        RunTestsSubcommand::Custom(custom_args) => {
            process_run_custom_instances(custom_args);
        }
        RunTestsSubcommand::Imported(imported_args) => {
            process_run_imported_instances(imported_args);
        }
    }
}

fn threads_in_range(s: &str) -> Result<u16, String> {
    let num_cpus = num_cpus::get() as u16;
    let num_threads = s
        .parse::<u16>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if num_threads == 0 {
        Err("Number of threads must be greater than 0".to_string())
    } else if num_threads > num_cpus {
        Err(format!(
            "Number of threads must be less than or equal to the number of CPUs ({})",
            num_cpus
        ))
    } else {
        Ok(num_threads)
    }
}
