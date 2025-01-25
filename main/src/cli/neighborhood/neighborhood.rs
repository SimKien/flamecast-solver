use clap::{Parser, Subcommand};

use super::{process_size, SizeArgs};

#[derive(Parser)]
pub struct NeighborhoodArgs {
    #[command(subcommand)]
    pub subcommand: NeighborhoodSubCommand,
}

#[derive(Subcommand)]
pub enum NeighborhoodSubCommand {
    /// Get the size of the neighborhood
    Size(SizeArgs),
}

pub fn process_neighborhood_command(args: NeighborhoodArgs) {
    match args.subcommand {
        NeighborhoodSubCommand::Size(size_args) => {
            process_size(size_args);
        }
    }
}
