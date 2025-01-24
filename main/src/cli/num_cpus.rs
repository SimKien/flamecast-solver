pub fn process_num_cpus() {
    let num = num_cpus::get();
    println!("Number of available threads: {}", num);
}
