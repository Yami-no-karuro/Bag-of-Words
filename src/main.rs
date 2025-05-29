#![allow(dead_code)]

mod bow;
mod tokenizer;
mod cli;

use std::env;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::path;
use std::path::PathBuf;

use bow::BoW;
use cli::{
    exit,
    print_help
};

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 2 {
        eprintln!("Error: no arguments provided.");
        print_help();
        exit(1);
    }

    let command: &str = &args[1];
    match command {
        "index" => handle_index_command(&args[2..]),
        _ => handle_unknown_command(command)
    }
}

fn handle_index_command(args: &[String]) {
    let source: &str = &args[0];
    let source_path: PathBuf = PathBuf::from(source);
    if !source_path.is_dir() {
        eprintln!("Error: the provided source (\"{}\") is not a directory", source);
        exit(1);
    }

    let mut source_bags: Vec<BoW> = Vec::new();
    if let Ok(paths) = fs::read_dir(source) {
        for (_idx, path) in paths.enumerate() {
            let source: path::PathBuf = path.unwrap()
                .path();

            let mut content: String = String::new();
            let mut f: File = File::open(source).unwrap();

            f.read_to_string(&mut content).unwrap();
            let bag: BoW = BoW::build(content);

            source_bags.push(bag);
        }
    } else {
        eprintln!("Error: unable to read the source (\"{}\") directory.", source);
        exit(1);
    }

    dbg!(source_bags);
    exit(0);
}

fn handle_unknown_command(command: &str) {
    eprintln!("Error: unknown command \"{}\".", command);
    print_help();
    exit(1);
}

