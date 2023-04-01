use std::{cmp::Ordering, error::Error, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "empow";
        let contents = "\
Rust is
A language empowering everyone
to build reliable and efficient software.
                        ";
        assert_eq!(vec!["A language empowering everyone"], search(query, contents));
    }
}

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
    let containing_lines = search(&conf.query, &contents);
    for line in containing_lines {
        println!("{line}");
    }
    Ok(())
}
pub fn search<'t>(query: &str, contents: &'t str) -> Vec<&'t str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            result.push(&line);
        }
    }
    result
}

pub fn search_insensitive<'t>(query: &str, contents: &'t str) -> Vec<&'t str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(&line);
        }
    }
    result
}
