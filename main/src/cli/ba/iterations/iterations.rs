use clap::{Parser, Subcommand};

use super::{
    process_iterations_evaluate_command, process_iterations_test_command, EvaluateIterationsArgs,
    TestIterationsArgs,
};

#[derive(Parser)]
pub struct IterationssArgs {
    #[command(subcommand)]
    pub subcommand: IterationsSubcommand,
}

#[derive(Subcommand)]
pub enum IterationsSubcommand {
    /// Test the instances
    Test(TestIterationsArgs),
    /// Evaluate the solutions
    Evaluate(EvaluateIterationsArgs),
}

pub fn process_iterations_command(args: IterationssArgs) {
    match args.subcommand {
        IterationsSubcommand::Test(sub_args) => {
            process_iterations_test_command(sub_args);
        }
        IterationsSubcommand::Evaluate(sub_args) => {
            process_iterations_evaluate_command(sub_args);
        }
    }
}
