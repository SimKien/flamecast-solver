use clap::Parser;

use crate::solver_testing::init_test;

#[derive(Parser)]
pub struct TestInitArgs {
    /// Name of the directory which is tested
    pub dir_name: String,
}

pub fn process_init_test_command(args: TestInitArgs) {
    init_test(&args.dir_name);
}
