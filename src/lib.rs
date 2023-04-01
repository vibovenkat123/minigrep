use std::{cmp::Ordering, error::Error, fs};

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&conf.path)?;
    println!("contents:\n{contents}");
    Ok(())
}
