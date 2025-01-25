use clap::Args;

use crate::solver_test::run_solver_tests;

#[derive(Args)]
pub struct RunTestsArgs {
    /// Number of threads to use for running tests.
    #[arg(short, long, value_parser = threads_in_range)]
    pub num_threads: Option<u16>,
}

pub fn process_run_tests(args: RunTestsArgs) {
    let num_threads = args.num_threads.unwrap_or(num_cpus::get() as u16);
    println!("Running tests with {} threads...", num_threads);
    run_solver_tests(num_threads as usize);
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
