use std::io;

use crate::apl_type::AplType;
use crate::apl_type::Scalar;
use crate::err::AplError;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TOKENS;

/// A struct representing a scanner. Note the difference between `self.current` and `self.start`.
/// Suppose that we are scanning something like a number, string, or identifier (variable) that
/// spans more than a single token. The index `self.start` will be the begining of the token, and
/// after the scanner reaches the end of the token, `self.current` will be further ahead. We then
/// add the token to `self.tokens` and set `self.start = self.current` before scanning the next
/// token.

#[derive(Debug, Clone)]
pub struct Scanner {
    /// raw character input of an APL program
    source: Vec<char>,
    /// resulting tokens after scanning `self.source`
    pub tokens: Vec<Token>,
    /// index of `self.source` that the scanner is examining
    current: usize,
    /// current line number the scanner is examining
    line: usize,
    /// starting index before scanner gets next token
    start: usize,
}

impl Scanner {
    /// initialize a new scanner
    pub fn new(s: String) -> Scanner {
        Scanner {
            source: s.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
            current: 0,
            line: 0,
            start: 0,
        }
    }

    /// return the current character and advance the scanner one character
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    /// add a non-literal token to `self.tokens`
    fn add(&mut self, token: TokenType) {
        let lexeme = (&self.source[self.start..self.current])
            .iter()
            .collect::<String>();

        self.tokens.push(Token {
            token,
            lexeme,
            line: self.line,
            literal: None,
        });
    }

    /// add a literal token (string, number, or identifier) to `self.tokens`
    fn add_token(&mut self, token: TokenType, l: AplType) {
        let lexeme = (&self.source[self.start..self.current])
            .iter()
            .collect::<String>();

        self.tokens.push(Token {
            token,
            lexeme,
            line: self.line,
            literal: Some(l),
        });
    }

    /// check if all characters have been scanned
    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// scan a single token
    fn scan_token(&mut self) -> Result<(), Vec<AplError>> {
        let mut errs: Vec<AplError> = Vec::new();
        let mut failed = false;
        let c = self.advance();

        // if we match 'simple' one char token, add it
        match TOKENS.get(&c) {
            Some(tt) => self.add(tt.clone()),
            None => {
                match c {
                    // High Minus is syntactical
                    // TODO: fix lexeme added here
                    '¯' => {
                        self.start = self.current;
                        if let Err(e) = self.number(-1.0) {
                            errs.push(e);
                            failed = true;
                        };
                    }

                    // move past comments
                    '⍝' => {
                        while self.peek() != '\n' && !(self.is_end()) {
                            self.advance();
                        }
                    }

                    // strings
                    '\'' => {
                        if let Err(e) = self.string() {
                            errs.push(e);
                            failed = true;
                        }
                    }

                    // end of line
                    '\n' => {
                        self.add(TokenType::Newline);
                        self.line += 1;
                    }

                    // whitespace
                    ' ' => (),
                    '\r' => (),
                    '\t' => (),

                    // numbers or variables
                    _ => {
                        if c.is_ascii_digit() {
                            if let Err(e) = self.number(1.0) {
                                errs.push(e);
                                failed = true;
                            };
                        } else if c.is_alphabetic() || c == '_' {
                            self.identifier();
                        } else {
                            errs.push(AplError::new("Unexpected character".to_string(), self.line));
                        }
                    }
                };
            }
        };

        if failed {
            Err(errs)
        } else {
            Ok(())
        }
    }

    /// return the current character without advancing the scanner
    fn peek(&mut self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    /// return the following character without advancing the scanner
    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    /// scan tokens until past all of `self.tokens`
    pub fn scan(&mut self) -> Result<(), Vec<AplError>> {
        let mut errors: Vec<AplError> = Vec::new();
        let mut failed = false;

        while !(self.is_end()) {
            self.start = self.current;
            if let Err(errs) = self.scan_token() {
                errors.extend(errs);
                failed = true;
            }
        }

        self.tokens.push(Token {
            token: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
            literal: None,
        });

        // transformation to reverse each line
        let mut line_counter = 0;

        self.tokens = self
            .tokens
            .clone()
            .into_iter()
            .fold(Vec::new(), |mut acc, x| {
                if x.line > line_counter || acc.is_empty() {
                    line_counter = x.line;
                    acc.push(Vec::new());
                }
                acc.last_mut().unwrap().push(x);
                acc
            })
            .into_iter()
            .flat_map(|x| x.into_iter().rev().collect::<Vec<Token>>())
            .collect::<Vec<Token>>();

        if failed {
            Err(errors)
        } else {
            Ok(())
        }
    }

    /// scan a string
    fn string(&mut self) -> Result<(), AplError> {
        while self.peek() != '\'' && !(self.is_end()) {
            if self.peek() == '\n' {
                self.add(TokenType::Newline);
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            return Err(AplError::new("Unterminated string".to_string(), self.line));
        }

        self.advance();

        let s = (&self.source[(self.start + 1)..(self.current - 1)])
            .iter()
            .collect::<String>();

        // for right now I'm doing this hack because the string arrays are UGLY
        // but... I kinda like it?
        // there is something philosophically interesting about changing input in the scanner
        // but it makes the parserer way more consistent...

        self.add(TokenType::LeftParenthesis);

        if s.chars().count() == 0 {
            self.add_token(
                TokenType::String,
                AplType::Scalar(Scalar::String("".to_string())),
            );
        }

        for c in s.chars() {
            self.add_token(
                TokenType::String,
                AplType::Scalar(Scalar::String(c.to_string())),
            );
        }

        self.add(TokenType::RightParenthesis);

        Ok(())
    }

    /// scan a number
    fn number(&mut self, scale: f64) -> Result<(), AplError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let s = &self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        match s.parse::<f64>() {
            Ok(n) => self.add_token(
                TokenType::Number,
                AplType::Scalar(Scalar::Number(scale * n)),
            ),
            Err(e) => {
                return Err(AplError::with_lower(
                    "Invalid number".to_string(),
                    self.line,
                    io::Error::new(io::ErrorKind::Other, e),
                ))
            }
        };

        Ok(())
    }

    /// scan an identifier
    fn identifier(&mut self) {
        // TODO: need to come back and change these rules to match APL
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let s = (&self.source[self.start..self.current])
            .iter()
            .collect::<String>();
        self.add_token(TokenType::Identifier, AplType::Name(s));
    }
}
