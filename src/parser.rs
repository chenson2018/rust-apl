use crate::err::AplError;
use crate::expr::Expr;
use crate::token::{Token, TokenType};

use std::rc::Rc;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { current: 0, tokens }
    }

    // Helper methods
    fn match_t(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, t: TokenType, msg: String) -> Result<Token, AplError> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(AplError::new(msg, self.peek().line))
        }
    }

    fn check(&mut self, t: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().token == t
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn at_end(&mut self) -> bool {
        self.peek().token == TokenType::Newline
    }

    pub fn parse(&mut self) -> Result<Expr, AplError> {
        self.expression()
    }

    pub fn expression(&mut self) -> Result<Expr, AplError> {
        self.dyadic()
    }

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
            let right = self.primary();

            // TODO: this is not good....
            match right {
                Ok(r) => {
                    e = Expr::Dyadic(Rc::new(r), op, Rc::new(e));
                }
                Err(_) => {
                    e = Expr::Monadic(op, Rc::new(e));
                }
            }
        }
        Ok(e)
    }

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

        //println!("What it looks like before we maybe enclose");
        //println!("{:#?}", v);

        let all_lit = v.iter().all(|s| matches!(s, Expr::Literal(_)));

        if v.is_empty() {
            Err(AplError::new(
                "Expected expression".to_string(),
                self.peek().line,
            ))
        } else if v.len() == 1 {
            Ok(v.clone().into_iter().next().unwrap())
        } else if all_lit {
            Ok(Expr::Array(v.into_iter().rev().collect::<Vec<Expr>>()))
        } else {
            Ok(Expr::Enclose(v.into_iter().rev().collect::<Vec<Expr>>()))
        }
    }
}
