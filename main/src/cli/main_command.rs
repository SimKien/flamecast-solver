use clap::{Parser, Subcommand};

use super::{BAArgs, InstancesArgs, NeighborhoodArgs};

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
    /// Run Bachelor Thesis Tests.
    BA(BAArgs),
    /// Run operations on Flamecast instances.
    Instances(InstancesArgs),
    /// Run neighborhood operations
    Neighborhood(NeighborhoodArgs),
    /// Get the number of CPUs available.
    NumCpus,
}
