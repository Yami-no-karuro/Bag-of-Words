#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod bow;
mod tokenizer;

use bow::BoW;
use tokenizer::{
    Token, 
    Tokenizer
};

fn main() {
    let titles: Vec<&str> = Vec::from([
        "Hello, how are you?",
        "I'm good, thank you very much!",
        "How's it going today?",
        "Today the weather is really nice.",
        "I enjoy programming in Rust."
    ]);

    for title in titles {
        let input: String = String::from(title);
        let bow: BoW = BoW::build(input);
        dbg!(bow);
    }
}
