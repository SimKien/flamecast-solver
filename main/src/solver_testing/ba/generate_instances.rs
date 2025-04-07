use std::path::PathBuf;

use rand::Rng;
use solver::generate_random_flamecast_test_instance;

use super::FlamecastBaseInstance;

pub const INSTANCES_DIR: &str = "./ba/instances";

pub fn generate_instances(
    number: usize,
    verbose: bool,
    num_layers: usize,
    min_sources: usize,
    max_sources: usize,
    min_drains: usize,
    max_drains: usize,
    dir_name: String,
) {
    if dir_name.is_empty() {
        return;
    }
    let dir_path: PathBuf = format!("{}/{}", INSTANCES_DIR, dir_name).into();
    if dir_path.exists() {
        for entry in std::fs::read_dir(&dir_path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            if entry.path().is_file() {
                std::fs::remove_file(entry.path()).expect("Failed to remove file");
            }
        }
    } else {
        std::fs::create_dir(&dir_path).expect("Failed to create directory");
    }
    let mut rng = rand::thread_rng();
    for i in 0..number {
        let num_sources = rng.gen_range(min_sources..=max_sources);
        let num_drains = rng.gen_range(min_drains..=max_drains);
        let test_instance =
            generate_random_flamecast_test_instance(num_layers, num_sources, num_drains, false);
        let base_instance = FlamecastBaseInstance::from_test_instance(&test_instance);
        let file_path = format!("{}/instance_{}.json", dir_path.display(), i);
        base_instance.to_file(&file_path);

        if verbose {
            println!(
                "Generated instance {} with {} sources and {} drains",
                i, num_sources, num_drains
            );
        }
    }
}
