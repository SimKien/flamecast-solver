use clap::Parser;

use crate::solver_testing::evaluate_alpha;

#[derive(Parser)]
pub struct EvaluateAlphaArgs {
    /// The directory which is evaluated
    pub dir: String,
}

pub fn process_evaluate_alpha_command(args: EvaluateAlphaArgs) {
    evaluate_alpha(&args.dir);
}
