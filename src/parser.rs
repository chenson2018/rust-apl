use crate::token::Token;
use crate::expr::Expr;
use crate::err::AplError;
use crate::token_type::TokenType;

use std::rc::Rc;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>
}

type ParseResult = Result<Expr,AplError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser{
        Parser { current: 0, tokens: tokens }
    }

    // Helper methods
    fn match_t(&mut self,types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self,t: TokenType,msg: String) -> Result<Token,AplError> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(AplError::new(msg,self.peek().line))
        }
    }

    fn check(&mut self,t: TokenType) -> bool {
        if self.is_end() { return false; }
        self.peek().token == t
    }

    fn advance(&mut self) -> Token {
        if !self.is_end() { self.current += 1; }
        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
        // This probably shouldn't clone, but im tired AND lazy.
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_end(&mut self) -> bool {
        self.peek().token == TokenType::Eof
    }

    pub fn parse(&mut self) -> ParseResult {
      self.expression()
    }

    pub fn expression(&mut self) -> ParseResult {
      self.equality()
    }
    pub fn equality(&mut self) -> ParseResult {
      self.comparison()
    }
    pub fn comparison(&mut self) -> ParseResult {
      self.term()
    }

    pub fn term(&mut self) -> ParseResult {
        let mut e = self.factor()?;

        while self.match_t(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            e = Expr::Dyadic(Rc::new(e),op,Rc::new(right));
        }
        Ok(e)
    }

    fn factor(&mut self) -> ParseResult {
        let mut e = self.monadic()?;

        while self.match_t(vec![TokenType::Slash,TokenType::Star]) {
            let op = self.previous();
            let right = self.monadic()?;
            e = Expr::Dyadic(Rc::new(e),op,Rc::new(right));
        }
        Ok(e)
    }

    fn monadic(&mut self) -> ParseResult {
        if self.match_t(vec![TokenType::ExclamationMark,TokenType::Minus]) {
            let op = self.previous();
            let right = self.monadic()?;
            return Ok(Expr::Monadic(op,Rc::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult {
        if self.match_t(vec![TokenType::Number,TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.unwrap()))
        }

        if self.match_t(vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous()))
        }

        if self.match_t(vec![TokenType::RightParenthesis]) {
            let e = self.expression()?;
            self.consume(TokenType::LeftParenthesis,"Expected ')' after expression".to_string())?;
            return Ok(Expr::Grouping(Rc::new(e)))
        };

        Err(AplError::new("Expected expression".to_string(),self.peek().line))
    }

}
