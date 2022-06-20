use crate::apl_type::AplType;
use crate::token::Token;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    Grouping(Rc<Expr>),
    Dyadic(Rc<Expr>, Token, Rc<Expr>),
    Monadic(Token, Rc<Expr>),
    Literal(AplType),
    Variable(Token),
    Enclose(Vec<Expr>),
    Array(Vec<Expr>), 
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Dyadic(ref left, ref token, ref right) => {
                write!(f, "({} {} {})", &token.lexeme, left, right)
            }
            Expr::Grouping(ref e) => {
                write!(f, "(group {})", e)
            }
            Expr::Literal(ref l) => {
                write!(f, "{}", l)
            }
            Expr::Monadic(ref token, ref e) => {
                write!(f, "({} {})", &token.lexeme, e)
            }
            Expr::Variable(ref token) => {
                write!(f, "var({})", &token.lexeme)
            }
            Expr::Enclose(ref vec) => {
                write!(f, "[")?;
                for v in vec {
                    write!(f, " {}", v)?;
                }
                write!(f, "]")?;
                Ok(())
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
