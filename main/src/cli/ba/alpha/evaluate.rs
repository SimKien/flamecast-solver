use clap::Parser;

use crate::solver_testing::evaluate_alpha;

#[derive(Parser)]
pub struct EvaluateAlphaArgs {
    /// Whether the process should be regarded
    #[arg(short, long)]
    pub process: bool,
    /// The directory which is regarded
    #[arg(short, long)]
    pub dir: Option<String>,
}

pub fn process_evaluate_alpha_command(args: EvaluateAlphaArgs) {
    evaluate_alpha(args.process, args.dir);
}
