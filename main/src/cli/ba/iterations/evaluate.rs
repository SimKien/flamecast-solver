use clap::Parser;

#[derive(Parser)]
pub struct EvaluateIterationsArgs {
    /// Name of the directory which is evaluated
    pub dir_name: String,
}

pub fn process_iterations_evaluate_command(args: EvaluateIterationsArgs) {}
