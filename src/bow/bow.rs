use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub struct BoW {
    bag: HashMap<Token, usize>
}

impl BoW {
    pub fn train(sentences: &[&str]) -> Self {
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

        return Self { bag };
    }

    pub fn get(&self, token: &Token) -> Option<&usize> {
        return self.bag.get(token);
    }

    pub fn tokens(&self) -> impl Iterator<Item = &Token> {
        return self.bag.keys();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Token, &usize)> {
        return self.bag.iter();
    }
}

