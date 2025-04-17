#[derive(
    Debug, 
    PartialEq
)]
pub enum TokenType {
    Identifier,
    Number,
    Punctuation,
    Whitespace,
    Unknown,
}

#[derive(
    Debug, 
    PartialEq
)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        return Token { 
            token_type, 
            value 
        };
    }
}
