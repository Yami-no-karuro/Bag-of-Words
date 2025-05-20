#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod bow;
mod tokenizer;
mod utils;

use std::env;
use std::process;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path;
use std::path::PathBuf;

use utils::cosine_similarity;

use bow::{BoW, DF};
use tokenizer::{
    Token, 
    Tokenizer
};

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 2 {
        eprintln!("Invalid arguments.");
        eprintln!("Usage: {} <directory> <query>", args[0]);
        process::exit(1);
    }

    let source: &str = &args[1];
    let query: &str = &args[2];

    let mut s_bags: Vec<BoW> = Vec::new();
    if let Ok(paths) = fs::read_dir(source) {
        for path in paths {
            let source: path::PathBuf = path.unwrap()
                .path();

            let mut content: String = String::new();
            let mut f: File = File::open(source)
                .unwrap();

            f.read_to_string(&mut content).unwrap();
            let bag: BoW = BoW::build(content);
            s_bags.push(bag);
        }
    }

    let s_df: DF = DF::build(&s_bags);
    let s_idf: HashMap<String, f64> = s_df.idf();

    let q_bag: BoW = BoW::build(query.to_string());
    let q_tfidf = q_bag.tfidf(&s_idf);

    for (i, doc_bag) in s_bags.iter().enumerate() {
        let doc_tfidf: HashMap<String, f64> = doc_bag.tfidf(&s_idf);
        let sim: f64 = cosine_similarity(&q_tfidf, &doc_tfidf);

        println!("{} - Score = {:.4}", i, sim);
    }
}

