use clap::{Parser, Subcommand};

use super::{
    process_init_evaluate_command, process_init_test_command, EvaluateInitArgs, TestInitArgs,
};

#[derive(Parser)]
pub struct InitArgs {
    #[command(subcommand)]
    pub subcommand: InitSubcommand,
}

#[derive(Subcommand)]
pub enum InitSubcommand {
    /// Test the instances
    Test(TestInitArgs),
    /// Evaluate the solutions
    Evaluate(EvaluateInitArgs),
}

pub fn process_init_command(args: InitArgs) {
    match args.subcommand {
        InitSubcommand::Test(sub_args) => {
            process_init_test_command(sub_args);
        }
        InitSubcommand::Evaluate(sub_args) => {
            process_init_evaluate_command(sub_args);
        }
    }
}
