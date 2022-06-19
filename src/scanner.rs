use std::io;

use crate::apl_type::AplType;
use crate::err::AplError;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    current: usize,
    line: usize,
    start: usize,
}

impl Scanner {
    pub fn new(s: String) -> Scanner {
        Scanner {
            source: s.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
            current: 0,
            line: 0,
            start: 0,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    // add and add_token accomplish the same thing, but for literals vs non-literals
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

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), Vec<AplError>> {
        let mut errs: Vec<AplError> = Vec::new();
        let mut failed = false;
        let c = self.advance();

        match c {
            // glyphs
            '←' => {
                self.add(TokenType::LeftArrow);
            }
            '+' => {
                self.add(TokenType::Plus);
            }
            '-' => {
                self.add(TokenType::Minus);
            }
            '÷' => {
                self.add(TokenType::Divide);
            }
            '*' => {
                self.add(TokenType::Star);
            }
            '⍟' => {
                self.add(TokenType::Log);
            }
            '⌹' => {
                self.add(TokenType::Domino);
            }
            '○' => {
                self.add(TokenType::Circle);
            }
            '!' => {
                self.add(TokenType::ExclamationMark);
            }
            '?' => {
                self.add(TokenType::QuestionMark);
            }
            '|' => {
                self.add(TokenType::Stile);
            }
            '⌈' => {
                self.add(TokenType::Upstile);
            }
            '⌊' => {
                self.add(TokenType::Downstile);
            }
            '⊥' => {
                self.add(TokenType::UpTack);
            }
            '⊤' => {
                self.add(TokenType::DownTack);
            }
            '⊣' => {
                self.add(TokenType::LeftTack);
            }
            '⊢' => {
                self.add(TokenType::RightTack);
            }
            '=' => {
                self.add(TokenType::Equal);
            }
            '≠' => {
                self.add(TokenType::NotEqual);
            }
            '≤' => {
                self.add(TokenType::LessThanEqual);
            }
            '<' => {
                self.add(TokenType::LessThan);
            }
            '>' => {
                self.add(TokenType::GreaterThan);
            }
            '≥' => {
                self.add(TokenType::GreaterThanEqual);
            }
            '≡' => {
                self.add(TokenType::EqualUnderbar);
            }
            '≢' => {
                self.add(TokenType::EqualUnderbarSlash);
            }
            '∨' => {
                self.add(TokenType::Or);
            }
            '∧' => {
                self.add(TokenType::And);
            }
            '⍱' => {
                self.add(TokenType::Nor);
            }
            '⍲' => {
                self.add(TokenType::Nand);
            }
            '↑' => {
                self.add(TokenType::UpArrow);
            }
            '↓' => {
                self.add(TokenType::DownArrow);
            }
            '⊂' => {
                self.add(TokenType::LeftShoe);
            }
            '⊃' => {
                self.add(TokenType::RightShoe);
            }
            '⊆' => {
                self.add(TokenType::LeftShoeUnderbar);
            }
            '⌷' => {
                self.add(TokenType::Squad);
            }
            '⍋' => {
                self.add(TokenType::GradeUp);
            }
            '⍒' => {
                self.add(TokenType::GradeDown);
            }
            '⍳' => {
                self.add(TokenType::Iota);
            }
            '⍸' => {
                self.add(TokenType::IotaUnderbar);
            }
            '∊' => {
                self.add(TokenType::Epsilon);
            }
            '⍷' => {
                self.add(TokenType::EpsilonUnderbar);
            }
            '∪' => {
                self.add(TokenType::DownShoe);
            }
            '∩' => {
                self.add(TokenType::UpShoe);
            }
            '~' => {
                self.add(TokenType::Tilde);
            }
            '/' => {
                self.add(TokenType::Slash);
            }
            '\\' => {
                self.add(TokenType::Backslash);
            }
            '⌿' => {
                self.add(TokenType::SlashBar);
            }
            '⍀' => {
                self.add(TokenType::BackslashBar);
            }
            ',' => {
                self.add(TokenType::Comma);
            }
            '⍪' => {
                self.add(TokenType::CommaBar);
            }
            '⍴' => {
                self.add(TokenType::Rho);
            }
            '⌽' => {
                self.add(TokenType::CircleStile);
            }
            '⊖' => {
                self.add(TokenType::CircleBar);
            }
            '⍉' => {
                self.add(TokenType::Transpose);
            }
            '¨' => {
                self.add(TokenType::Diaeresis);
            }
            '⍨' => {
                self.add(TokenType::TildeDiaeresis);
            }
            '⍣' => {
                self.add(TokenType::StarDiaeresis);
            }
            '.' => {
                self.add(TokenType::Dot);
            }
            '∘' => {
                self.add(TokenType::Jot);
            }
            '⍤' => {
                self.add(TokenType::JotDiaeresis);
            }
            '⍥' => {
                self.add(TokenType::CircleDiaeresis);
            }
            '@' => {
                self.add(TokenType::At);
            }
            '⍞' => {
                self.add(TokenType::QuoteQuad);
            }
            '⎕' => {
                self.add(TokenType::Quad);
            }
            '⍠' => {
                self.add(TokenType::QuadColon);
            }
            '⌸' => {
                self.add(TokenType::QuadEqual);
            }
            '⌺' => {
                self.add(TokenType::QuadDiamond);
            }
            '⌶' => {
                self.add(TokenType::Ibeam);
            }
            '⍎' => {
                self.add(TokenType::Hydrant);
            }
            '⍕' => {
                self.add(TokenType::Thorn);
            }
            '⋄' => {
                self.add(TokenType::Diamond);
            }
            '→' => {
                self.add(TokenType::RightArrow);
            }
            '⍵' => {
                self.add(TokenType::Omega);
            }
            '⍺' => {
                self.add(TokenType::Alpha);
            }
            '∇' => {
                self.add(TokenType::Del);
            }
            '&' => {
                self.add(TokenType::Ampersand);
            }
            '¯' => {
                self.add(TokenType::HighMinus);
            }
            '⍬' => {
                self.add(TokenType::Zilde);
            }
            // move past comments
            '⍝' => {
                while self.peek() != '\n' && !(self.is_end()) {
                    self.advance();
                }
            }
            // non glyphs
            '(' => {
                self.add(TokenType::LeftParenthesis);
            }
            ')' => {
                self.add(TokenType::RightParenthesis);
            }
            '{' => {
                self.add(TokenType::LeftBrace);
            }
            '}' => {
                self.add(TokenType::RightBrace);
            }
            // strings (which is reall a vector of char scalars, may need to come back to fix this)
            '\'' => {
                if let Err(e) = self.string() {
                    errs.push(e);
                    failed = true;
                }
            }
            // end of line
            '\n' => {
                self.line += 1;
            }

            // whitespace
            ' ' => (),
            '\r' => (),
            '\t' => (),

            _ => {
                if c.is_ascii_digit() {
                    if let Err(e) = self.number() {
                        errs.push(e);
                        failed = true;
                    };
                } else if c.is_alphabetic() || c == '_' || c == '-' {
                    self.identifier();
                } else {
                    errs.push(AplError::new("Unexpected character".to_string(), self.line));
                }
            }
        };

        if failed {
            Err(errs)
        } else {
            Ok(())
        }
    }

    fn peek(&mut self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

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

    fn string(&mut self) -> Result<(), AplError> {
        while self.peek() != '\'' && !(self.is_end()) {
            if self.peek() == '\n' {
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
          self.add_token(TokenType::String, AplType::String("".to_string()));
        }

        for c in s.chars() {
            self.add_token(TokenType::String, AplType::String(c.to_string()));
        }

        self.add(TokenType::RightParenthesis);

        Ok(())
    }

    fn number(&mut self) -> Result<(), AplError> {
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
            Ok(n) => self.add_token(TokenType::Number, AplType::Number(n)),
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

    fn identifier(&mut self) {
        // need to come back and change these rules to match APL
        while self.peek().is_alphanumeric() || self.peek() == '_' || self.peek() == '-' {
            self.advance();
        }
        let s = (&self.source[self.start..self.current])
            .iter()
            .collect::<String>();
        self.add_token(TokenType::Identifier, AplType::Name(s));
    }
}
