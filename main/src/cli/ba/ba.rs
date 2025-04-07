use clap::{Parser, Subcommand};

use super::{process_generate_command, process_test_alpha_command, GenerateArgs, TestAlphaArgs};

#[derive(Parser)]
pub struct BAArgs {
    #[command(subcommand)]
    pub subcommand: BASubcommand,
}

#[derive(Subcommand)]
pub enum BASubcommand {
    /// Generate random instances
    Generate(GenerateArgs),
    /// Test with alpha configuration
    TestAlpha(TestAlphaArgs),
}

pub fn process_ba_command(args: BAArgs) {
    match args.subcommand {
        BASubcommand::Generate(sub_args) => {
            process_generate_command(sub_args);
        }
        BASubcommand::TestAlpha(test_args) => {
            // Call the function to process the test command with alpha configuration
            process_test_alpha_command(test_args);
        }
    }
}
