use std::{cmp::Ordering, env, fs, process, error::Error};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing the arguments: {}", e);
        process::exit(1);
    });
    println!("Finding {}", &config.query);
    println!("In file {}", &config.path);
    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        let (query, path): (String, String) = match args.len().cmp(&3) {
            Ordering::Less => {
                return Err("not entought arguments");
            }
            Ordering::Greater => {
                return Err("Too much arguments");
            }
            Ordering::Equal => {
                let query_arg = &args[1];
                let path_arg = &args[2];
                (query_arg.to_string(), path_arg.to_string())
            }
        };
        Ok(Config { query, path })
    }
}

fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&conf.path)?;
    println!("contents:\n{contents}");
    Ok(())
}
