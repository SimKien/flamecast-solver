mod cli;
mod solver_test;

use clap::Parser;
use cli::{
    process_generate_instances, process_num_cpus, process_run_tests, FirstLevelCommand, MainArgs,
};

fn main() {
    let args = MainArgs::parse();

    match args.subcommand {
        FirstLevelCommand::GenerateInstances(sub_args) => {
            process_generate_instances(sub_args);
        }
        FirstLevelCommand::RunTests(sub_args) => {
            process_run_tests(sub_args);
        }
        FirstLevelCommand::NumCpus => {
            process_num_cpus();
        }
    }
}
