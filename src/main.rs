#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tokenizer;

use tokenizer::{Token, Tokenizer};

fn main() {
    let input: String = "Hello! My Name is Carlo, i'm 26 years old and i'm a software developer!".to_string();
    let mut tokenizer: Tokenizer = Tokenizer::new(input);
    let tokens: Vec<Token> = tokenizer.tokenize();

    dbg!(tokens);
}
