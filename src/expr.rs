use crate::token::{Token,AplType};
use std::rc::Rc;
use std::fmt;

#[derive(Clone,Debug)]
pub enum Expr {
  Grouping(Rc<Expr>),
  Dyadic(Rc<Expr>,Token,Rc<Expr>),
  Monadic(Token,Rc<Expr>),
  Literal(AplType),
  Variable(Token),
  Array(Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::Dyadic(ref left,ref token,ref right) => {
                write!(f,"({} {} {})",&token.lexeme,left,right)
            },
            &Expr::Grouping(ref e) => {
                write!(f,"(group {})",e)
            },
            &Expr::Literal(ref l) => {
                write!(f,"{}",l)
            },
            &Expr::Monadic(ref token,ref e) => {
                write!(f,"({} {})",&token.lexeme,e)
            },
            &Expr::Variable(ref token) => {
                write!(f,"var({})",&token.lexeme)
            },
           &Expr::Array(ref vec) => {
              for v in vec {
                write!(f, "\t{}", v)?;
              }
              Ok(())
            },
        }
    }
}
