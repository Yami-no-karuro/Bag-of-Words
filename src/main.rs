#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tokenizer;

use tokenizer::{Token, Tokenizer};
use std::collections::HashMap;

fn main() {
    let input: String = "Hello! My Name is Carlo, i'm 26 years old and i'm a software developer!".to_string();

    let mut tokenizer: Tokenizer = Tokenizer::new(input);
    let mut bag: HashMap<Token, usize> = HashMap::new();

    let tokens: Vec<Token> = tokenizer.tokenize();
    for token in tokens {
        if bag.contains_key(&token) {
            let count: &mut usize = bag.get_mut(&token).unwrap();
            *count += 1;
        } else {
            bag.insert(token, 1);
        }
    }

    for (token, count) in bag {
        println!("{:?}: {}", token, count);
    }
}
