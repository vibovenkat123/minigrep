use std::{cmp::Ordering, env, fs, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Finding {}", &config.query);
    println!("In file {}", &config.path);
        }
    };
    println!("Finding {}", &query);
    println!("In file {}", &path);
struct Config {
    query: String,
    path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let (query, path): (String, String) = match args.len().cmp(&3) {
            Ordering::Less => {
                println!("Not enough arguments");
                process::exit(1);
            }
            Ordering::Greater => {
                println!("Too much arguments");
                process::exit(1);
            }
            Ordering::Equal => {
                let query_arg = &args[1];
                let path_arg = &args[2];
                (query_arg.to_string(), path_arg.to_string())
            }
        };
        Config { query, path }
    }
}
