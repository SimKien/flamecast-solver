use clap::Parser;

use crate::solver_testing::process_test_alpha;

#[derive(Parser)]
pub struct TestAlphaArgs {
    /// Name of the directory which is tested
    pub dir_name: String,
}

pub fn process_test_alpha_command(args: TestAlphaArgs) {
    process_test_alpha(&args.dir_name);
}
