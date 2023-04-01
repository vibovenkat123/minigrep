use std::{cmp::Ordering, env, fs, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing the arguments: {}", e);
        process::exit(1);
    });
    println!("Finding {}", &config.query);
    println!("In file {}", &config.path);

    let contents_out = fs::read_to_string(&config.path);
    let contents = match contents_out {
        Ok(val) => val,
        Err(err) => {
            let err_code_out = err.raw_os_error();
            let code = match err_code_out {
                Some(val) => val,
                None => {
                    panic!("{}", err);
                }
            };
            if code == 2 {
                println!("{}: file not found", &config.path);
            } else {
                panic!("{}", err);
            }
            process::exit(1);
        }
    };
    println!("contents:\n{contents}");
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
