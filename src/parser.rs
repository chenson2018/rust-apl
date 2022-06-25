use crate::err::AplError;
use crate::expr::Expr;
use crate::token::{Token, TokenType};

use std::rc::Rc;

// these define the few functions that only have one meaning
// TODO: handle operators
// TODO: Niladic

static DYADIC_ONLY: [TokenType; 14] = [
    TokenType::LeftArrow,
    TokenType::UpTack,
    TokenType::DownTack,
    TokenType::Equal,
    TokenType::LessThanEqual,
    TokenType::LessThan,
    TokenType::GreaterThan,
    TokenType::GreaterThanEqual,
    TokenType::Or,
    TokenType::And,
    TokenType::Nor,
    TokenType::Nand,
    TokenType::EpsilonUnderbar,
    TokenType::UpShoe,
];

//static MONADIC_ONLY: [TokenType; 0] = [];

/// A struct representing a parser. This operates much like
/// [rust_apl::scanner::Scanner](crate::scanner::Scanner), but instead of transforming raw
/// characters into tokens, this takes those tokens and builds a tree representing an expression.

pub struct Parser {
    /// index of self.tokens that the parser is examining
    current: usize,
    /// vector of tokens, as received by [a Scanner](crate::scanner::Scanner)
    tokens: Vec<Token>,
}

impl Parser {
    /// initialize a new parser
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { current: 0, tokens }
    }

    /// if the current token matches any of `types`, advance the parser
    fn match_t(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// advance the parser a `Token` with the specified `TokenType` is found
    fn consume(&mut self, t: TokenType, msg: String) -> Result<Token, AplError> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(AplError::new(msg, self.peek().line))
        }
    }

    /// check if the current `Token` has the specified `TokenType`
    fn check(&mut self, t: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().token == t
    }

    /// return the current `Token` and advance the parser one token
    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// return the current Token without advancing the scanner
    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    /// return the previous Token without advancing the scanner
    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    /// check if all tokens have been parsed
    fn at_end(&mut self) -> bool {
        self.peek().token == TokenType::Newline
    }

    /// parse `self.tokens` into an [expr::Expr](crate::expr::Expr)
    pub fn parse(&mut self) -> Result<Expr, AplError> {
        self.expression()
    }

    /// parse an expression into an [expr::Expr](crate::expr::Expr)
    ///
    /// Note that this function is called recursively, so that `self.parse`
    /// requires only a single call.
    pub fn expression(&mut self) -> Result<Expr, AplError> {
        self.dyadic()
    }

    /// parse a dyadic expression, i.e an expression with a function that takes a left and right argument
    ///
    /// OR (split this later!!!)
    ///
    /// parse a monadic expression, i.e an expression with a function that only a right argument
    ///
    fn dyadic(&mut self) -> Result<Expr, AplError> {
        let mut e = self.primary()?;

        // this assumes every primitive can be monadic or dyadic, is that true? probably not
        while self.match_t(vec![
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Rho,
            TokenType::LeftShoe,
            TokenType::LeftArrow,
        ]) {
            let op = self.previous();
            let left = self.primary();

            match left {
                Ok(Expr::Null) => {
                    if DYADIC_ONLY.contains(&op.token) {
                        return Err(AplError::new(
                            "Left argument required.".to_string(),
                            op.line,
                        ));
                    } else {
                        e = Expr::Monadic(op, Rc::new(e));
                    };
                }
                Ok(r) => {
                    e = Expr::Dyadic(Rc::new(r), op, Rc::new(e));
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }
        Ok(e)
    }

    /// loop, parsing literals (string, number, variable) or groupings (an expression bounded by
    /// parenthesis) until the steam of tokens is exhausted.
    fn primary(&mut self) -> Result<Expr, AplError> {
        let mut v: Vec<Expr> = Vec::new();

        loop {
            if self.match_t(vec![
                TokenType::Number,
                TokenType::String,
                TokenType::Identifier,
            ]) {
                match self.previous().token {
                    TokenType::Number | TokenType::String => {
                        v.push(Expr::Literal(self.previous().literal.unwrap()));
                    }
                    TokenType::Identifier => {
                        v.push(Expr::Variable(self.previous()));
                    }
                    _ => panic!(
                        "Only Number, String, Identifier should be reachable here...got {:#?}",
                        self.previous().token
                    ),
                }
            } else if self.match_t(vec![TokenType::RightParenthesis]) {
                let e = self.expression()?;
                self.consume(
                    TokenType::LeftParenthesis,
                    "Expected '(' after expression".to_string(),
                )?;
                v.push(Expr::Grouping(Rc::new(e)))
            } else {
                break;
            };
        }

        if v.is_empty() {
            Ok(Expr::Null)
        } else if v.len() == 1 {
            Ok(v.clone().into_iter().next().unwrap())
        } else {
            Ok(Expr::Array(v.into_iter().rev().collect::<Vec<Expr>>()))
        }
    }
}
