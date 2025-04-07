use clap::Parser;

use crate::solver_testing::generate_instances;

#[derive(Parser)]
pub struct GenerateArgs {
    /// The name of the directory to store the generated instances
    pub dir_name: String,
    /// number of instances to generate
    pub number: usize,
    /// Flag to turn on verbose printing while instance generation
    #[arg(short, long)]
    pub verbose: bool,
    /// The number of layers in the generated instances
    #[arg(short, long, default_value_t = 3)]
    pub layers: usize,
    /// The minimum number of sources in the generated instances
    #[arg(long, default_value_t = 1)]
    pub min_sources: usize,
    /// The maximum number of sources in the generated instances
    #[arg(long, default_value_t = 1)]
    pub max_sources: usize,
    /// The minimum number of drains in the generated instances
    #[arg(long, default_value_t = 1)]
    pub min_drains: usize,
    /// The maximum number of drains in the generated instances
    #[arg(long, default_value_t = 1)]
    pub max_drains: usize,
}

pub fn process_generate_command(args: GenerateArgs) {
    generate_instances(
        args.number,
        args.verbose,
        args.layers,
        args.min_sources,
        args.max_sources,
        args.min_drains,
        args.max_drains,
        args.dir_name.clone(),
    );
    if args.verbose {
        println!("Generated {} instances.", args.number);
    }
}
