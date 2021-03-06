use crate::token::Token;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    Grouping(Rc<Expr>),
    Dyadic(Rc<Expr>, Token, Rc<Expr>),
    Monadic(Token, Rc<Expr>),
    Literal(Token),
    Variable(Token),
    Array(Vec<Expr>),
    Null,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Null => Ok(()),
            Expr::Dyadic(ref left, ref token, ref right) => {
                write!(f, "({} {} {})", &token.lexeme, left, right)
            }
            Expr::Grouping(ref e) => {
                write!(f, "(group {})", e)
            }
            Expr::Literal(ref token) => {
                write!(f, "{}", &token.lexeme)
            }
            Expr::Monadic(ref token, ref e) => {
                write!(f, "({} {})", &token.lexeme, e)
            }
            Expr::Variable(ref token) => {
                write!(f, "var({})", &token.lexeme)
            }
            Expr::Array(ref vec) => {
                write!(f, "[")?;
                for v in vec {
                    write!(f, " {}", v)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}
