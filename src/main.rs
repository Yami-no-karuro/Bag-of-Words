#[derive(Debug, PartialEq)]
enum TokenType {
    Identifier,
    Number,
    Punctuation,
    Whitespace,
    Unknown
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    value: String
}

impl Token {
    fn new(token_type: TokenType, value: String) -> Self {
        return Token { 
            token_type, 
            value 
        };
    }
}

struct Tokenizer {
    chars: Vec<char>,
    position: usize
}

impl Tokenizer {
    fn new(input: String) -> Self {
        return Tokenizer { 
            chars: input.chars().collect(), 
            position: 0 
        };
    }

    fn tokenize(&mut self) -> Vec<Token> {
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

        return Some(Token::new(
            TokenType::Identifier,
            self.chars[start..self.position].iter().collect()
        ));
    }

    fn collect_number(&mut self) -> Option<Token> {
        let start: usize = self.position;
        while self.position < self.chars.len() && self.chars[self.position].is_numeric() {
            self.position += 1;
        }

        return Some(Token::new(
            TokenType::Number,
            self.chars[start..self.position].iter().collect()
        ));
    }

    fn collect_whitespace(&mut self) -> Option<Token> {
        let start: usize = self.position;
        while self.position < self.chars.len() && self.chars[self.position].is_whitespace() {
            self.position += 1;
        }

        return Some(Token::new(
            TokenType::Whitespace,
            self.chars[start..self.position].iter().collect()
        ));
    }

    fn collect_punctuation(&mut self) -> Option<Token> {
        let current: char = self.chars[self.position];
        self.position += 1;

        Some(Token::new(TokenType::Punctuation, current.to_string()));
        return Some(Token::new(
            TokenType::Punctuation,
            current.to_string()
        ));
    }
}

fn main() {
    let input = "Hello! My Name is Carlo, i'm 26 years old and i'm a software developer!".to_string();
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    dbg!(tokens);
}

