use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug,Clone)]
pub enum AplType {
    String(String),
    Number(f64),
    Name(String),
}

impl fmt::Display for AplType {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AplType::String(ref s) => write!(f,"{}",s),
            &AplType::Number(ref n) => write!(f,"{}",n),
            &AplType::Name(ref b) => write!(f,"{}",b),
        }
    }
}



#[derive(Debug,Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub literal: Option<AplType>,
    pub line: i32,
}
