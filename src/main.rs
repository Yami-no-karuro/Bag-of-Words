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
        "I enjoy programming in Rust.",
        "I just finished reading an interesting book.",
        "What do you think about the new movie?",
        "Shall we go for a coffee together?",
        "Technology is changing the world.",
        "Tomorrow I have an important appointment.",
        "Have you eaten yet?",
        "The cat is sleeping on the couch.",
        "I'd like to travel more this year.",
        "This morning I woke up late.",
        "Music relaxes me a lot.",
        "I am learning to use neural networks.",
        "This project is very stimulating.",
        "Do you want to go out tonight?",
        "I bought a new mechanical keyboard.",
        "Every day is a new opportunity."
    ]);

    let bow: BoW = BoW::train(&sentences);
    bow.save();
}
