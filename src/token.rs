use crate::token_type::TokenType;

#[derive(Debug,Clone)]
pub enum AplType {
    String(String),
    Number(f64),
    Name(String),
}

#[derive(Debug,Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub literal: Option<AplType>,
    pub line: i32,
}
