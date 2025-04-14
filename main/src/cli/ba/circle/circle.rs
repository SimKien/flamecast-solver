use clap::{Parser, Subcommand};

use super::{process_evaluate_circle_command, process_test_circle_command};

#[derive(Parser)]
pub struct CircleArgs {
    #[command(subcommand)]
    pub subcommand: CircleSubcommand,
}

#[derive(Subcommand)]
pub enum CircleSubcommand {
    /// Test instances
    Test,
    /// Evaluate solutions
    Evaluate,
}

pub fn process_circle_command(args: CircleArgs) {
    match args.subcommand {
        CircleSubcommand::Test => process_test_circle_command(),
        CircleSubcommand::Evaluate => process_evaluate_circle_command(),
    }
}
