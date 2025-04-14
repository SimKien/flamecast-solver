use clap::Parser;

use crate::solver_testing::iterations_evaluate;

#[derive(Parser)]
pub struct EvaluateIterationsArgs {
    /// Name of the directory which is evaluated
    pub dir_name: String,
    /// Alpha value for the evaluation
    pub alpha: f64,
}

pub fn process_iterations_evaluate_command(args: EvaluateIterationsArgs) {
    iterations_evaluate(&args.dir_name, args.alpha);
}
