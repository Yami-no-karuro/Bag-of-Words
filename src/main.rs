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
use std::collections::HashMap;

use bow::{BoW, DF};
use tokenizer::{
    Token, 
    Tokenizer
};

fn cosine_similarity(vec_a: &HashMap<String, f64>, vec_b: &HashMap<String, f64>) -> f64 {
    let mut dot_product: f64 = 0.0;
    for (token, value_a) in vec_a.iter() {
        if let Some(value_b) = vec_b.get(token) {
            dot_product += value_a * value_b;
        }
    }

    let mut sum_squares_a: f64 = 0.0;
    for value in vec_a.values() {
        sum_squares_a += value * value;
    }

    let mut sum_squares_b: f64 = 0.0;
    for value in vec_b.values() {
        sum_squares_b += value * value;
    }

    let norm_a = sum_squares_a.sqrt();
    let norm_b = sum_squares_b.sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    let similarity = dot_product / (norm_a * norm_b);
    return similarity;
}

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
        let doc_tfidf = doc_bag.tfidf(&s_idf);
        let sim = cosine_similarity(&q_tfidf, &doc_tfidf);
        println!("Doc {}: score = {:.4}", i, sim);
    }
}

