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
      self.dyadic()
    }

    fn dyadic(&mut self) -> ParseResult {
       let mut e = self.primary()?;

       // this assumes every primitive can be monadic or dyadic, is that true? probably not
       while self.match_t(vec![TokenType::Minus,TokenType::Plus]) {
         let op = self.previous();
         let right = self.primary();

         // this is not good....
         match right {
            Ok(r)  => { e = Expr::Dyadic(Rc::new(r),op,Rc::new(e)); },
            Err(_) => { e = Expr::Monadic(op,Rc::new(e))          ; },
         }
       }
       Ok(e)
    }

    fn primary(&mut self) -> ParseResult {
        let mut v: Vec<crate::expr::Expr> = Vec::new();

        loop {
           if self.match_t(vec![TokenType::Number,TokenType::String,TokenType::Identifier]) {
              match self.previous().token {
                 TokenType::Number | TokenType::String => { v.push(Expr::Literal(self.previous().literal.unwrap())); },             
                 TokenType::Identifier                 => { v.push(Expr::Variable(self.previous())); },
                 _ => panic!("Only Number, String, Identifier should be reachable here...got {:?}", self.previous().token),
              }
           } else if self.match_t(vec![TokenType::RightParenthesis]) {
               let e = self.expression()?;
               self.consume(TokenType::LeftParenthesis,"Expected '(' after expression".to_string())?;
               v.push(Expr::Grouping(Rc::new(e)))
           } else {
             break;
           };
        }

        if v.len() == 0 {
          Err(AplError::new("Expected expression".to_string(),self.peek().line))
        } else if v.len() == 1 {
          return Ok(v.clone().into_iter().nth(0).unwrap());
        } else {
          return Ok(crate::expr::Expr::Array(v.into_iter().rev().collect::<Vec<Expr>>()));
        }
           
    }

}
