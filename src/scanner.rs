use crate::token::Token;
use crate::token_type::TokenType;
use crate::err::AplError;
use crate::apl_type::AplType;
use std::io;

#[derive(Debug,Clone)]
pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    current: i32,
    line: i32,
    start: i32
}

impl Scanner {
    pub fn new(s: String) -> Scanner {
        Scanner {
            source: s.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
            current: 0,
            line: 0,
            start: 0
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[(self.current - 1) as usize]
    }

    fn add(&mut self,token: TokenType) {
        let lexeme = (&self.source[self.start as usize..self.current as usize]).iter().collect::<String>();
        self.tokens.push(Token { token, lexeme, line: self.line, literal: None });
    }

    fn add_token(&mut self,token: TokenType,l: AplType) {
        let lexeme = (&self.source[self.start as usize..self.current as usize]).iter().collect::<String>();
        self.tokens.push(Token { token, lexeme, line: self.line, literal: Some(l) });
    }

    fn is_end(&self) -> bool {
        self.current >= (self.source.len() as i32)
    }

    fn scan_token(&mut self) -> Result<(),Vec<AplError>> {
      let mut errs: Vec<AplError> = Vec::new();
      let mut failed = false;
      let c = self.advance();

      match c {
       '←'  => { self.add(TokenType::LeftArrow); },
       '+'  => { self.add(TokenType::Plus); },
       '-'  => { self.add(TokenType::Minus); },
       '÷'  => { self.add(TokenType::Divide); },
       '*'  => { self.add(TokenType::Star); },
       '⍟'  => { self.add(TokenType::Log); },
       '⌹'  => { self.add(TokenType::Domino); },
       '○'  => { self.add(TokenType::Circle); },
       '!'  => { self.add(TokenType::ExclamationMark); },
       '?'  => { self.add(TokenType::QuestionMark); },
       '|'  => { self.add(TokenType::Stile); },
       '⌈'  => { self.add(TokenType::Upstile); },
       '⌊'  => { self.add(TokenType::Downstile); },
       '⊥'  => { self.add(TokenType::UpTack); },
       '⊤'  => { self.add(TokenType::DownTack); },
       '⊣'  => { self.add(TokenType::LeftTack); },
       '⊢'  => { self.add(TokenType::RightTack); },
       '='  => { self.add(TokenType::Equal); },
       '≠'  => { self.add(TokenType::NotEqual); },
       '≤'  => { self.add(TokenType::LessThanEqual); },
       '<'  => { self.add(TokenType::LessThan); },
       '>'  => { self.add(TokenType::GreaterThan); },
       '≥'  => { self.add(TokenType::GreaterThanEqual); },
       '≡'  => { self.add(TokenType::EqualUnderbar); },
       '≢'  => { self.add(TokenType::EqualUnderbarSlash); },
       '∨'  => { self.add(TokenType::Or); },
       '∧'  => { self.add(TokenType::And); },
       '⍱'  => { self.add(TokenType::Nor); },
       '⍲'  => { self.add(TokenType::Nand); },
       '↑'  => { self.add(TokenType::UpArrow); },  
       '↓'  => { self.add(TokenType::DownArrow); },  
       '⊂'  => { self.add(TokenType::LeftShoe); },
       '⊃'  => { self.add(TokenType::RightShoe); },
       '⊆'  => { self.add(TokenType::LeftShoeUnderbar); },
       '⌷'  => { self.add(TokenType::Squad); },
       '⍋'  => { self.add(TokenType::GradeUp); },
       '⍒'  => { self.add(TokenType::GradeDown); },
       '⍳'  => { self.add(TokenType::Iota); },
       '⍸'  => { self.add(TokenType::IotaUnderbar); },
       '∊'  => { self.add(TokenType::Epsilon); },
       '⍷'  => { self.add(TokenType::EpsilonUnderbar); },
       '∪'  => { self.add(TokenType::DownShoe); },
       '∩'  => { self.add(TokenType::UpShoe); },
       '~'  => { self.add(TokenType::Tilde); },
       '/'  => { self.add(TokenType::Slash); },
       '\\' => { self.add(TokenType::Backslash); },
       '⌿'  => { self.add(TokenType::SlashBar); },
       '⍀'  => { self.add(TokenType::BackslashBar); },
       ','  => { self.add(TokenType::Comma); },
       '⍪'  => { self.add(TokenType::CommaBar); },      
       '⍴' => { self.add(TokenType::Rho); },
       '⌽' => { self.add(TokenType::CircleStile); },
       '⊖' => { self.add(TokenType::CircleBar); },
       '⍉' => { self.add(TokenType::Transpose); },
       '¨' => { self.add(TokenType::Diaeresis); },
       '⍨' => { self.add(TokenType::TildeDiaeresis); },
       '⍣' => { self.add(TokenType::StarDiaeresis); },
       '.' => { self.add(TokenType::Dot); },
       '∘' => { self.add(TokenType::Jot); },
       '⍤' => { self.add(TokenType::JotDiaeresis); },
       '⍥' => { self.add(TokenType::CircleDiaeresis); },
       '@' => { self.add(TokenType::At); },
       '⍞' => { self.add(TokenType::QuoteQuad); },
       '⎕' => { self.add(TokenType::Quad); },
       '⍠' => { self.add(TokenType::QuadColon); },
       '⌸' => { self.add(TokenType::QuadEqual); },
       '⌺' => { self.add(TokenType::QuadDiamond); },
       '⌶' => { self.add(TokenType::Ibeam); },
       '⍎' => { self.add(TokenType::Hydrant); },
       '⍕' => { self.add(TokenType::Thorn); },
       '⋄' => { self.add(TokenType::Diamond); },
       '⍝' => {

                    while self.peek() != '\n' && !(self.is_end()) {
                        self.advance();
                    } },
       '→' => { self.add(TokenType::RightArrow); },
       '⍵' => { self.add(TokenType::Omega); },
       '⍺' => { self.add(TokenType::Alpha); },
       '∇' => { self.add(TokenType::Del); },
       '&' => { self.add(TokenType::Ampersand); },
       '¯' => { self.add(TokenType::HighMinus); },
       '⍬' => { self.add(TokenType::Zilde); },

// non glyphs
       '(' => { self.add(TokenType::LeftParenthesis); },
       ')' => { self.add(TokenType::RightParenthesis); },
       '{' => { self.add(TokenType::LeftBrace); },
       '}' => { self.add(TokenType::RightBrace); },
       '\'' => {
           if let Err(e) = self.string() {
               errs.push(e);
           }
       },
       '\n' => { self.add(TokenType::Newline); self.line += 1; },
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
               errs.push(AplError::new("Unexpected character".to_string(),self.line));
           }
       }
      };

      if failed {
          Err(errs)
      } else {
          Ok(())
      }
    }

//    fn match_c(&mut self, expected: char) -> bool {
//        if self.is_end() {
//            return false;
//        }
//        if self.source[self.current as usize] != expected {
//            return false;
//        }
//
//        self.current += 1;
//        true
//    }

    fn peek(&mut self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.source[self.current as usize]
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= (self.source.len() as i32) {
            '\0'
        } else {
            self.source[(self.current + 1) as usize]
        }
    }

    pub fn scan(&mut self) -> Result<(),Vec<AplError>> {
      let mut errors: Vec<AplError> = Vec::new();
      let mut failed = false;
        while !(self.is_end()) {
            self.start = self.current;
            if let Err(errs)  = self.scan_token() {
                errors.extend(errs);
                failed = true;
            }
        }
        self.tokens.push(Token { token: TokenType::Eof, lexeme:"".to_string(), line:self.line, literal: None });

        if failed {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn string(&mut self) -> Result<(),AplError> {
        while self.peek() != '\'' && !(self.is_end()) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            return Err(AplError::new("Unterminated string".to_string(),self.line));
        }

        self.advance();
        let s = (&self.source[(self.start + 1) as usize..(self.current - 1) as usize]).iter().collect::<String>();
        self.add_token(TokenType::String,AplType::String(s));
        Ok(())
    }

    fn number(&mut self) -> Result<(),AplError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let s = &self.source[self.start as usize..self.current as usize].iter().collect::<String>();
        match s.parse::<f64>() {
            Ok(n) => self.add_token(TokenType::Number,AplType::Number(n)),
            Err(e) => return Err(AplError::with_lower("Invalid number".to_string(),self.line,io::Error::new(io::ErrorKind::Other,e)))
        };
        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' || self.peek() == '-' {
            self.advance();
        }
        let s = (&self.source[self.start as usize..self.current as usize]).iter().collect::<String>();
        self.add_token(TokenType::Identifier, AplType::Name(s));
    }
}
