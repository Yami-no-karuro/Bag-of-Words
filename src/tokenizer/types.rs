#[derive(Debug, Eq, PartialEq, Hash)]
pub enum TokenType {
    Identifier,
    Number,
    Punctuation,
    Unknown
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        return Token { 
            token_type, 
            value 
        };
    }
}
