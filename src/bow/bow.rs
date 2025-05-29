use std::collections::HashMap;

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
}

