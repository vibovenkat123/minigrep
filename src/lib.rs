//! # Minigrep
//!
//! `vibo_minigrep` allows you to search for text within a file
use std::{env, error::Error, fs};


// the tests for the search function
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
    #[test]
    fn config_init() {
        let args = ["vibo_minigrep", "query", "path"].iter().map(|s| s.to_string());
        let conf = Config::init(args);
        assert!(conf.is_ok());
        let conf = conf.unwrap();
        let conf_to_test = Config {
            query: "query".to_string(),
            path: "path".to_string(),
            ignore_case: false,
        };
        assert_eq!(&conf_to_test.query, &conf.query);
        assert_eq!(&conf_to_test.path, &conf.path);
        assert_eq!(&conf_to_test.ignore_case, &conf.ignore_case);
    }
    #[test]
    fn config_init_no_args() {
        let args = ["vibo_minigrep"].iter().map(|s| s.to_string());
        assert!(Config::init(args).is_err());
    }
    #[test]
    fn config_init_ignore_case() {
        let args = ["vibo_minigrep", "query", "path"].iter().map(|s| s.to_string());
        env::set_var("IGNORE_CASE", "1");
        let conf = Config::init(args);
        assert!(conf.is_ok());
        let conf = conf.unwrap();
        let conf_to_test = Config {
            query: "query".to_string(),
            path: "path".to_string(),
            ignore_case: true,
        };
        assert_eq!(&conf_to_test.query, &conf.query);
        assert_eq!(&conf_to_test.path, &conf.path);
        assert_eq!(&conf_to_test.ignore_case, &conf.ignore_case);
        env::remove_var("IGNORE_CASE");
    }
}

// the config when parsing the arguments
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Initializes a config based on an iterator (std::env::args())
    ///
    /// # Example
    /// ```
    /// fn main() {
    ///     let args = ["vibo_minigrep", "query", "example.txt"].iter().map(|s| s.to_string());
    ///
    ///     let config = vibo_minigrep::Config::init(args).unwrap_or_else(|err| {
    ///         eprintln!("err: {}", err);
    ///         std::process::exit(1);
    ///     });
    ///     let config_to_test = vibo_minigrep::Config {
    ///         query: "query".to_string(),
    ///         path: "example.txt".to_string(),
    ///         ignore_case: false,
    ///     };
    ///     assert_eq!(config_to_test.query, config.query);
    ///     assert_eq!(config_to_test.path, config.path);
    ///     assert_eq!(config_to_test.ignore_case, config.ignore_case);
    /// }
    /// ```
    /// # Errors
    ///
    /// This function will return an error if, but not only if:
    ///
    /// * Providing no path or query arguments
    pub fn init(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // the first argument is always the path
        args.next();
        // recurse through the arguments
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query argument"),
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path argument"),
        };
        // ignore the case if there is a env variable
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}
/// Runs all the application logic
///
/// # Example
///
/// ```
/// use vibo_minigrep::Config;
/// use std::{error::Error, fs};
/// fn main() {
///     let mut file_path= "example.txt";
///     fs::File::create(file_path);
///     let conf = Config {
///         query: "hello".to_string(),
///         ignore_case: false,
///         path: file_path.to_string(),
///     };
///     let run_res = vibo_minigrep::run(conf);
///     assert!(run_res.is_ok());
///     fs::remove_file(file_path);
/// }
/// ```
/// # Errors
///
/// This function will return an error if, but not only if:
///
/// * it cannot read the file to a string
///
/// * the file in the config is not found
pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    // get the file contents
    let contents = fs::read_to_string(&conf.path)?;
    // get the lines which match
    let mut containing_lines: Vec<&str> = search(&conf.query, &contents);
    // if its case insensitive, only search insensitively
    if conf.ignore_case {
        containing_lines = search_insensitive(&conf.query, &contents);
    }
    // print each line
    for line in containing_lines {
        println!("{line}");
    }
    Ok(())
}

/// Searches through a string to find a query
///
/// # Example
///
/// ```
/// fn main() {
///     let cool_string = "\
///hello
///world
///hi
///there
///     ";
///     let query = "there";
///     let lines = vibo_minigrep::search(&query, &cool_string);
///     assert_eq!(vec!["there"], lines);
/// }
/// ```
pub fn search<'t>(query: &str, contents: &'t str) -> Vec<&'t str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches through a string to find a query case insensitively
///
/// # Example
///
/// ```
/// fn main() {
///     let cool_string = "\
/// foo
/// bar
/// baz
///     ";
///     let query = "FOO";
///     let lines = vibo_minigrep::search_insensitive(&query, &cool_string);
///     assert_eq!(vec!["foo"], lines);
/// }
/// ```
pub fn search_insensitive<'t>(query: &str, contents: &'t str) -> Vec<&'t str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
