use clap::Args;

use crate::solver_testing::{test_predefined_instances, INSTANCES};

#[derive(Args)]
pub struct PredefinedArgs {
    /// Indices of predefined instances that are tested
    #[arg(short, long, value_parser = index_valid)]
    pub imported_index: Vec<usize>,
}

pub fn process_run_predefined_instances(args: PredefinedArgs) {
    let mut imported_indexes = args.imported_index;
    imported_indexes.sort();
    imported_indexes.dedup();
    test_predefined_instances(&imported_indexes);
}

fn index_valid(s: &str) -> Result<usize, String> {
    let index = s
        .parse::<usize>()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if index >= INSTANCES.len() {
        Err(format!(
            "The index {} is too big for {} instances",
            index,
            INSTANCES.len()
        ))
    } else {
        Ok(index)
    }
}
