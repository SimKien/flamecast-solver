mod cli;
mod solver_testing;

use clap::Parser;
use cli::{
    process_ba_command, process_instances_command, process_neighborhood_command, process_num_cpus,
    FirstLevelCommand, MainArgs,
};

fn main() {
    let args = MainArgs::parse();

    match args.subcommand {
        FirstLevelCommand::BA(sub_args) => {
            process_ba_command(sub_args);
        }
        FirstLevelCommand::Instances(sub_args) => {
            process_instances_command(sub_args);
        }
        FirstLevelCommand::Neighborhood(sub_args) => {
            process_neighborhood_command(sub_args);
        }
        FirstLevelCommand::NumCpus => {
            process_num_cpus();
        }
    }
}
