use clap::Parser;

use crate::solver_testing::evaluate_num_vertices;

#[derive(Parser)]
pub struct EvaluateNumVerticesArgs {
    /// Name of the directory which is evaluated
    pub dir_name: String,
    /// Alpha value for the evaluation
    pub alpha: f64,
}

pub fn process_num_vertices_evaluate_command(args: EvaluateNumVerticesArgs) {
    evaluate_num_vertices(&args.dir_name, args.alpha);
}
