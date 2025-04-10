use clap::{Parser, Subcommand};

use super::{
    process_evaluate_alpha_command, process_test_alpha_command, EvaluateAlphaArgs, TestAlphaArgs,
};

#[derive(Parser)]
pub struct AlphaArgs {
    #[command(subcommand)]
    pub subcommand: AlphaSubcommand,
}

#[derive(Subcommand)]
pub enum AlphaSubcommand {
    /// Test instances
    Test(TestAlphaArgs),
    /// Evaluate solutions
    Evaluate(EvaluateAlphaArgs),
}

pub fn process_alpha_command(args: AlphaArgs) {
    match args.subcommand {
        AlphaSubcommand::Test(test_args) => process_test_alpha_command(test_args),
        AlphaSubcommand::Evaluate(eval_args) => process_evaluate_alpha_command(eval_args),
    }
}
