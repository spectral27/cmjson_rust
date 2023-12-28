use std::{env, fs, process};

mod distribution;
mod file_read_write;
mod selector;
mod id_generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments provided.");
        process::exit(1);
    }

    match args[1].as_str() {
        "-s" | "--select" => {
            if args.len() == 2 {
                selector::select_all();
            } else {
                selector::select(&args[2]);
            }
        }
        "-i" | "--insert" => {
            if args.len() > 2 {
                match fs::read_to_string(&args[2]) {
                    Ok(result) => selector::insert(&result),
                    Err(e) => println!("Error reading content from file: {}", e)
                }
            } else {
                println!("Provide a path with json input file.");
            }
        }
        "-u" | "--update" => {
            if args.len() > 2 {
                match fs::read_to_string(&args[2]) {
                    Ok(result) => selector::update(&result),
                    Err(e) => println!("Error reading content from file: {}", e)
                }
            } else {
                println!("Provide a path with json input file.");
            }
        }
        "-d" | "--delete" => {
            if args.len() == 3 {
                selector::delete(&args[2]);
            } else {
                println!("Distribution to delete id needed.");
            }
        },
        _ => println!("Unrecognized option: {}", args[1])
    }
}
