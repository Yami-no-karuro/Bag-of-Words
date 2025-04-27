use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufRead};

use crate::tokenizer::{Token, TokenType, Tokenizer};

#[derive(Debug)]
pub struct BoW {
    bag: HashMap<String, usize>
}

impl BoW {
    pub fn train(sentences: &[&str]) -> Self {
        let mut bag: HashMap<String, usize> = HashMap::new();
        for sentence in sentences {
            let input: String = sentence.to_string(); 
            let mut tokenizer: Tokenizer = Tokenizer::new(input);

            let tokens: Vec<Token> = tokenizer.tokenize();
            for token in tokens {
                if token.token_type == TokenType::Identifier {
                    let value = token.value;
                    *bag.entry(value).or_insert(0) += 1;
                }
            }
        }

        return Self { bag };
    }

    pub fn load(name: &str) -> Self {
        let mut bag: HashMap<String, usize> = HashMap::new();
        let path: String = format!("models/{}.csv", name);
        let f: File = File::open(path).unwrap();

        let reader: BufReader<File> = BufReader::new(f);
        for line in reader.lines() {
            let str_line: String = line.unwrap();
            let parts: Vec<&str> = str_line.split("\",\"").collect();

            let token: &str = parts[0].trim_matches('"');
            let count: usize = parts[1].trim_matches('"')
                .parse::<usize>()
                .unwrap_or_default();

            if count > 0 {
                bag.insert(token.to_string(), count);
            }
        }

        return Self { bag };
    }

    pub fn save(&self, name: &str) {
        let path: String = format!("models/{}.csv", name);
        let mut f: File = File::create(path).unwrap();
        for (token, count) in self.iter() {
            writeln!(&mut f, "\"{}\",\"{}\"", token, count).unwrap();
        }
    }

    pub fn get(&self, token: &Token) -> Option<&usize> {
        return self.bag.get(&token.value);
    }

    pub fn tokens(&self) -> impl Iterator<Item = &String> {
        return self.bag.keys();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &usize)> {
        return self.bag.iter();
    }
}
