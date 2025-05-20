mod bow;
mod tokenizer;
mod utils;

use std::env;
use std::process;
use std::collections::HashMap;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::path;

use utils::cosine_similarity;
use bow::{BoW, DF};

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

    println!("Sources:");

    let mut s_bags: Vec<BoW> = Vec::new();
    if let Ok(paths) = fs::read_dir(source) {
        for (idx, path) in paths.enumerate() {
            let source: path::PathBuf = path.unwrap()
                .path();

            println!("Idx: {}, Source: \"{}\"", idx, source.display());
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

    println!("\nQuery: \"{}\"", query);
    println!("\nResults:");

    for (i, doc_bag) in s_bags.iter().enumerate() {
        let doc_tfidf: HashMap<String, f64> = doc_bag.tfidf(&s_idf);
        let sim: f64 = cosine_similarity(&q_tfidf, &doc_tfidf);
        println!("{} - Score = {:.4}", i, sim);
    }
}

