#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod bow;
mod tokenizer;

use bow::BoW;
use tokenizer::{Token, Tokenizer};

fn main() {
    let sentences: Vec<&str> = Vec::from([
        "Hello, how are you?",
        "I'm good, thank you very much!",
        "How's it going today?",
        "Today the weather is really nice.",
        "I enjoy programming in Rust."
    ]);

    let trained: BoW = BoW::train(&sentences);
    trained.save("example");

    let loaded: BoW = BoW::load("example");
    dbg!(loaded);
}
