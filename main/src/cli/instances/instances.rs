use super::{process_generate_instances, process_run_tests, RunTestsArgs};
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct InstancesArgs {
    #[command(subcommand)]
    pub subcommand: InstancesSubcommand,
}

#[derive(Subcommand)]
pub enum InstancesSubcommand {
    /// Generate Flamecast instances.
    Generate,
    /// Run tests on Flamecast instances.
    RunTests(RunTestsArgs),
}

pub fn process_instances_command(args: InstancesArgs) {
    match args.subcommand {
        InstancesSubcommand::Generate => {
            process_generate_instances();
        }
        InstancesSubcommand::RunTests(run_tests_args) => {
            process_run_tests(run_tests_args);
        }
    }
}
