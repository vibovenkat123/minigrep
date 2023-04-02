use vibo_minigrep::Config;
use std::{env, process};

fn main() {
    // get the starter config
    let config = Config::init(env::args()).unwrap_or_else(|e| {
        // usually if there is too much or too little arguments
        eprintln!("Problem parsing the arguments: {}", e);
        process::exit(1);
    });
    // run the main application
    if let Err(e) = vibo_minigrep::run(config) {
        // if there is a problem while running
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
