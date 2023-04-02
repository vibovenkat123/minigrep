use std::{env, error::Error, fs};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_insensitive() {
        let query = "RUST";
        let contents = "\
Rust is
A language empowering everyone
to build reliable and efficient software.
                        ";
        assert_eq!(vec!["Rust is"], search_insensitive(query, contents));
    }
    #[test]
    fn case_sensitive() {
        let query = "empow";
        let contents = "\
Rust is
A language empowering everyone
to build reliable and efficient software.
Empowering
                        ";
        assert_eq!(
            vec!["A language empowering everyone"],
            search(query, contents)
        );
    }
}

pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn init(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query argument"),
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path argument"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&conf.path)?;
    let mut containing_lines: Vec<&str> = search(&conf.query, &contents);
    if conf.ignore_case {
        containing_lines = search_insensitive(&conf.query, &contents);
    }
    for line in containing_lines {
        println!("{line}");
    }
    Ok(())
}
pub fn search<'t>(query: &str, contents: &'t str) -> Vec<&'t str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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
