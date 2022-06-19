use crate::apl_type::AplType;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub literal: Option<AplType>,
    pub line: usize,
}
