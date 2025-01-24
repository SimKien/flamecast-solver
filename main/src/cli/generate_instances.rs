use clap::Args;

use crate::solver_test::create_testing_instances;

#[derive(Args)]
pub struct GenerateInstancesArgs {
    /// Use predefined testing instances.
    #[arg(short, long, default_value_t = false)]
    pub predefined: bool,
}

pub fn process_generate_instances(args: GenerateInstancesArgs) {
    if args.predefined {
        println!("Creating testing instances...");
        create_testing_instances();
    } else {
        println!("Not using predefined testing instances is not supported yet.");
    }
}
