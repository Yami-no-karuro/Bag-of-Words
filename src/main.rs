mod bow;
mod tokenizer;
mod utils;

mod unknown;
mod index;
mod search;

use std::env;
use index::handle_index;
use search::handle_search;
use unknown::handle_unknown;
use utils::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: no arguments provided.");
        print_help();
        exit(1);
    }

    let command: &str = &args[1];
    match command {
        "index" => handle_index(&args[2..]),
        "search" => handle_search(&args[2..]),
        _ => handle_unknown(command)
    }
}

fn print_help() {
    println!("Usage: ");
    println!("- index <path>");
    println!("- search <query>");
}

