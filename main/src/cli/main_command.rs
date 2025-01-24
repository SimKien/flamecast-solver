use clap::{Parser, Subcommand};

use super::{GenerateInstancesArgs, RunTestsArgs};

#[derive(Parser)]
#[command(
    version = "1.0.0",
    about = "Commands to use the flamecast solver library.",
    long_about = "This program provides commands to use the flamecast solver library."
)]
pub struct MainArgs {
    #[command(subcommand)]
    pub subcommand: FirstLevelCommand,
}

#[derive(Subcommand)]
pub enum FirstLevelCommand {
    /// Generate testing instances for the flamecast solver.
    GenerateInstances(GenerateInstancesArgs),
    /// Run tests for the flamecast solver with predefined or provided instances.
    RunTests(RunTestsArgs),
    /// Get the number of CPUs available.
    NumCpus,
}
