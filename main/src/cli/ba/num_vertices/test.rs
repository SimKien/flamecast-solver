use clap::Parser;

use crate::solver_testing::num_vertices_test;

#[derive(Parser)]
pub struct TestNumVerticesArgs {
    /// Name of the directory which is tested
    pub dir_name: String,
}

pub fn process_num_vertices_test_command(args: TestNumVerticesArgs) {
    num_vertices_test(&args.dir_name);
}
