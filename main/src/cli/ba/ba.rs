use clap::{Parser, Subcommand};

use super::{process_predefined_command, PredefinedArgs};

#[derive(Parser)]
pub struct BAArgs {
    #[command(subcommand)]
    pub subcommand: BASubcommand,
}

#[derive(Subcommand)]
pub enum BASubcommand {
    /// Run predefined tests
    Predefined(PredefinedArgs),
}

pub fn process_ba_command(args: BAArgs) {
    match args.subcommand {
        BASubcommand::Predefined(sub_args) => {
            process_predefined_command(sub_args);
        }
    }
}
