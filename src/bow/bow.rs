use std::collections::HashMap;
use std::fs::File;
use std::io::{
    Write, 
    BufReader, 
    BufRead
};

use crate::tokenizer::{
    Token, 
    TokenType, 
    Tokenizer
};

#[derive(Debug)]
pub struct BoW {
    bag: HashMap<String, usize>
}

impl BoW {
    pub fn build(input: String) -> Self {
        let mut bag: HashMap<String, usize> = HashMap::new();
        let mut tokenizer: Tokenizer = Tokenizer::new(input);

        let tokens: Vec<Token> = tokenizer.tokenize();
        for token in tokens {
            if token.token_type == TokenType::Identifier {
                let value: String = token.value;
                *bag.entry(value)
                    .or_insert(0) += 1;
            }
        }

        return Self { bag };
    }

    pub fn tfidf(&self, idf: &HashMap<String, f64>) -> HashMap<String, f64> {
        let mut tfidf_vec: HashMap<String, f64> = HashMap::new();
        for (token, &tf_count) in self.iter() {
            if let Some(&idf_val) = idf.get(token) {
                let tfidf: f64 = (tf_count as f64) * idf_val;
                tfidf_vec.insert(token.clone(), tfidf);
            }
        }

        return tfidf_vec;
    }

    pub fn _load(name: &str) -> Self {
        let mut bag: HashMap<String, usize> = HashMap::new();
        let path: String = format!("bags/{}.csv", name);
        let f: File = File::open(path)
            .unwrap();

        let reader: BufReader<File> = BufReader::new(f);
        for line in reader.lines() {
            let str_line: String = line.unwrap();
            let parts: Vec<&str> = str_line.split("\",\"")
                .collect();

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

    pub fn _save(&self, name: &str) {
        let path: String = format!("bags/{}.csv", name);
        let mut f: File = File::create(path)
            .unwrap();

        for (token, count) in self.iter() {
            writeln!(&mut f, "\"{}\",\"{}\"", token, count)
                .unwrap();
        }
    }

    pub fn tokens(&self) -> impl Iterator<Item = &String> {
        return self.bag.keys();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &usize)> {
        return self.bag.iter();
    }
}

