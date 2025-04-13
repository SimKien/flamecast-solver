use clap::Parser;

use crate::solver_testing::iterations_test;

#[derive(Parser)]
pub struct TestIterationsArgs {
    /// Name of the directory which is tested
    pub dir_name: String,
}

pub fn process_iterations_test_command(args: TestIterationsArgs) {
    iterations_test(&args.dir_name);
}
