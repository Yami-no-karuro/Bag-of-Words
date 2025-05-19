#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod bow;
mod tokenizer;

use std::env;
use std::process;

use std::fs;
use std::fs::File;

use std::io;
use std::io::Read;

use std::path;
use std::path::PathBuf;

use bow::BoW;
use tokenizer::{
    Token, 
    Tokenizer
};

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 2 {
        eprintln!("Invalid arguments.");
        eprintln!("Usage: {} <dir>", args[0]);
        process::exit(1);
    }

    let source_dir: &str = &args[1];
    if let Ok(paths) = fs::read_dir(source_dir) {
        for path in paths {
            let source: path::PathBuf = path.unwrap()
                .path();

            let mut content: String = String::new();
            let mut f: File = File::open(source)
                .unwrap();

            f.read_to_string(&mut content).unwrap();
            let bag: BoW = BoW::build(content);
            dbg!(bag);
        }
    }
}

