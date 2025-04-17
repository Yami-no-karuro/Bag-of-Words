#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tokenizer;

use tokenizer::{Tokenizer};

fn main() {
    let input = "Hello! My Name is Carlo, i'm 26 years old and i'm a software developer!".to_string();
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    dbg!(tokens);
}

