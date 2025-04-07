use clap::Parser;

use crate::solver_testing::{evaluate_alpha, process_test_alpha};

#[derive(Parser)]
pub struct TestAlphaArgs {
    /// Name of the directory which is tested
    pub dir_name: String,
    /// Flag to activate evaluation mode
    #[arg(long, short)]
    pub evaluation: bool,
}

pub fn process_test_alpha_command(args: TestAlphaArgs) {
    if args.evaluation {
        evaluate_alpha(&args.dir_name);
        return;
    }
    process_test_alpha(&args.dir_name);
}
