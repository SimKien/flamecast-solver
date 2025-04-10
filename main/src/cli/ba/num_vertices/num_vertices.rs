use clap::{Parser, Subcommand};

use super::{process_num_vertices_test_command, TestNumVerticesArgs};

#[derive(Parser)]
pub struct NumVerticesArgs {
    #[command(subcommand)]
    pub subcommand: NumVerticesSubcommand,
}

#[derive(Subcommand)]
pub enum NumVerticesSubcommand {
    /// Test the instances
    Test(TestNumVerticesArgs),
    /// Evaluate the solutions
    Evaluate,
}

pub fn process_num_vertices_command(args: NumVerticesArgs) {
    match args.subcommand {
        NumVerticesSubcommand::Test(sub_args) => {
            process_num_vertices_test_command(sub_args);
        }
        NumVerticesSubcommand::Evaluate => {}
    }
}
