use clap::Args;

use crate::solver_testing::test_imported_instances;

#[derive(Args)]
pub struct ImportedArgs {
    /// Indices of imported instances that are tested
    #[arg(short, long, value_parser = index_valid)]
    pub imported_index: Vec<usize>,
}

pub fn process_run_imported_instances(args: ImportedArgs) {
    let mut imported_indexes = args.imported_index;
    imported_indexes.sort();
    imported_indexes.dedup();

    if imported_indexes.len() > 0 {
        test_imported_instances(&imported_indexes);
    }
}

fn index_valid(s: &str) -> Result<usize, String> {
    let index = s
        .parse::<usize>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    Ok(index)
}
