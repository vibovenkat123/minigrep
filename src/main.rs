use minigrep::Config;
use std::{env, process};
fn main() {
    let config = Config::init(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing the arguments: {}", e);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
