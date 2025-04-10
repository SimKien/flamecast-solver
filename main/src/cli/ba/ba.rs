use clap::{Parser, Subcommand};

use super::{
    process_alpha_command, process_generate_command, process_init_command,
    process_num_vertices_command, AlphaArgs, GenerateArgs, InitArgs, NumVerticesArgs,
};

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
    Alpha(AlphaArgs),
    /// Test init functions
    Init(InitArgs),
    /// Test various number of random vertices
    NumVertices(NumVerticesArgs),
}

pub fn process_ba_command(args: BAArgs) {
    match args.subcommand {
        BASubcommand::Generate(sub_args) => {
            process_generate_command(sub_args);
        }
        BASubcommand::Alpha(sub_args) => {
            process_alpha_command(sub_args);
        }
        BASubcommand::Init(sub_args) => {
            process_init_command(sub_args);
        }
        BASubcommand::NumVertices(sub_args) => {
            process_num_vertices_command(sub_args);
        }
    }
}
