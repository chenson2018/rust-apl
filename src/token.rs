use crate::token_type::TokenType;
use crate::apl_type::AplType;

#[derive(Debug,Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub line: i32,
    pub literal: Option<AplType>
}
