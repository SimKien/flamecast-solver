use clap::Parser;

use crate::solver_testing::evaluate_init;

#[derive(Parser)]
pub struct EvaluateInitArgs {
    /// Name of the directory which is evaluated
    pub dir_name: String,
}

pub fn process_init_evaluate_command(args: EvaluateInitArgs) {
    evaluate_init(&args.dir_name);
}
