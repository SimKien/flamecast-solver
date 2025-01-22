use solver_test::{create_testing_instances, run_solver_tests};

mod solver_test;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let num = num_cpus::get();
    println!("Number of available threads: {}", num);

    if args.len() > 3 || args.len() < 2 {
        println!("Usage: cargo run create_testing_instances");
        println!("Usage: cargo run run_tests <num_threads>");
        return;
    }

    if args.len() == 2 && args[1] == "create_testing_instances" {
        println!("Creating testing instances...");
        create_testing_instances();
    } else if args[1] == "run_tests" {
        let num_threads = if args.len() == 3 {
            args[2].parse::<usize>().unwrap()
        } else {
            num
        };
        println!("Running tests with {} threads...", num_threads);
        run_solver_tests(num_threads);
    } else {
        println!("Usage: cargo run create_testing_instances");
        println!("Usage: cargo run run_tests <num_threads>");
        return;
    }
}
