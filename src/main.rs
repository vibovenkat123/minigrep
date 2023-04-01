use std::{env, process};
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing the arguments: {}", e);
        process::exit(1);
    });
    println!("Finding {}", &config.query);
    println!("In file {}", &config.path);
    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
