use super::types::{Token, TokenType};

pub struct Tokenizer {
    chars: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        return Tokenizer {
            chars: input.chars().collect(),
            position: 0,
        };
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(token) = self.collect_next() {
            tokens.push(token);
        }

        return tokens;
    }

    fn collect_next(&mut self) -> Option<Token> {
        if self.position >= self.chars.len() {
            return None;
        }

        let current: char = self.chars[self.position];
        if current.is_alphabetic() {
            return self.collect_identifier();
        } else if current.is_numeric() {
            return self.collect_number();
        } else if current.is_whitespace() {
            return self.collect_whitespace();
        } else if ",;.:!?()[]{}\"'".contains(current) {
            return self.collect_punctuation();
        } else {
            self.position += 1;
            return Some(Token::new(TokenType::Unknown, current.to_string()));
        }
    }

    fn collect_identifier(&mut self) -> Option<Token> {
        let start: usize = self.position;
        while self.position < self.chars.len() && self.chars[self.position].is_alphabetic() {
            self.position += 1;
        }

        let identifier: String = self.chars[start..self.position]
            .iter()
            .collect::<String>()
            .to_lowercase();

        return Some(Token::new(
            TokenType::Identifier, 
            identifier
        ));
    }

    fn collect_number(&mut self) -> Option<Token> {
        let start: usize = self.position;
        while self.position < self.chars.len() && self.chars[self.position].is_numeric() {
            self.position += 1;
        }

        return Some(Token::new(
            TokenType::Number,
            self.chars[start..self.position].iter().collect(),
        ));
    }

    fn collect_whitespace(&mut self) -> Option<Token> {
        let start: usize = self.position;
        while self.position < self.chars.len() && self.chars[self.position].is_whitespace() {
            self.position += 1;
        }

        return Some(Token::new(
            TokenType::Whitespace,
            self.chars[start..self.position].iter().collect(),
        ));
    }

    fn collect_punctuation(&mut self) -> Option<Token> {
        let current: char = self.chars[self.position];
        self.position += 1;

        return Some(Token::new(
            TokenType::Punctuation, 
            current.to_string()
        ));
    }
}
