#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tokenizer;

use tokenizer::{Token, Tokenizer};
use std::collections::HashMap;

fn train(sentences: &[&str]) -> HashMap<Token, usize> {
    let mut bag: HashMap<Token, usize> = HashMap::new();
    for sentence in sentences {
        let input: String = sentence.to_string(); 

        let mut tokenizer: Tokenizer = Tokenizer::new(input);
        let tokens: Vec<Token> = tokenizer.tokenize();
        for token in tokens {
            if bag.contains_key(&token) {
                let count: &mut usize = bag.get_mut(&token).unwrap();
                *count += 1;
            } else {
                bag.insert(token, 1);
            }
        }
    }

    return bag;
}

fn main() {
    let sentences: Vec<&str> = Vec::from([
        "Hello, how are you?",
        "I'm good, thank you very much!",
        "How's it going today?",
        "Today the weather is really nice.",
        "I enjoy programming in Python.",
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

    let bag: HashMap<Token, usize> = train(&sentences);
    for (token, count) in bag {
        println!("{:?}: {}", token, count);
    }
}
