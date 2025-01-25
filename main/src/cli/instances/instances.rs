use super::{process_generate_instances, process_run_tests, GenerateInstancesArgs, RunTestsArgs};
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct InstancesArgs {
    #[command(subcommand)]
    pub subcommand: InstancesSubcommand,
}

#[derive(Subcommand)]
pub enum InstancesSubcommand {
    /// Generate Flamecast instances.
    Generate(GenerateInstancesArgs),
    /// Run tests on Flamecast instances.
    RunTests(RunTestsArgs),
}

pub fn process_instances_command(args: InstancesArgs) {
    match args.subcommand {
        InstancesSubcommand::Generate(generate_args) => {
            process_generate_instances(generate_args);
        }
        InstancesSubcommand::RunTests(run_tests_args) => {
            process_run_tests(run_tests_args);
        }
    }
}
