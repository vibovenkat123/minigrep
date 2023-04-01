use std::{cmp::Ordering, env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, path): (String, String) = match args.len().cmp(&3) {
        Ordering::Less => {
            println!("Not enough arguments");
            exit(1);
        }
        Ordering::Greater => {
            println!("Too much arguments");
            exit(1);
        }
        Ordering::Equal => {
            let query_arg = &args[1];
            let path_arg = &args[2];
            (query_arg.to_string(), path_arg.to_string())
        }
    };
    println!("Finding {}", &query);
    println!("In file {}", &path);
}
