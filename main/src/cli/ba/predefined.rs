use clap::Parser;

use crate::solver_testing::run_predefined_tests;

#[derive(Parser)]
pub struct PredefinedArgs {
    /// Instance type which is tested
    pub instance_dir: Vec<String>,
    /// Alpha value for the test
    #[arg(short, long)]
    pub alpha: Option<f64>,
}

pub fn process_predefined_command(args: PredefinedArgs) {
    run_predefined_tests(args.instance_dir, args.alpha);
}
