use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let path = &args[2];
    println!("Finding {}", query);
    println!("In file {}", path);
}
