use crate::apl_type::AplType;
use crate::err::AplError;
use crate::expr::Expr;
use crate::token_type::TokenType;

use crate::primitives::dyadic::add;
use crate::primitives::monadic::shape;

#[derive(Clone)]
pub struct Interpreter {}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: properly report line of errors

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&mut self, e: &Expr) -> Result<AplType, AplError> {
        self.evaluate(e)
    }

    fn evaluate(&mut self, e: &Expr) -> Result<AplType, AplError> {
        match &*e {
            Expr::Array(ref t) => {
                let res = t
                    .iter()
                    .map(|x| self.evaluate(x).unwrap())
                    .collect::<Vec<AplType>>();
                Ok(AplType::Array(res))
            }
            Expr::Literal(ref t) => Ok(t.clone()),
            Expr::Grouping(ref expr) => self.evaluate(expr),
            Expr::Dyadic(ref left, ref op, ref right) => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                let res = match op.token {
                    TokenType::Plus => add(left, right),
                    _ => todo!("Dyadic operator {:?}", op.token),
                };

                match res {
                    Ok(value) => Ok(value),
                    Err(err) => Err(AplError::new(err.to_string(), 0)),
                }
            }
            Expr::Monadic(ref op, ref right) => {
                let right = self.evaluate(right)?;

                let res =  match op.token {
                    TokenType::Rho => shape(right),
                    _ => todo!("Monadic operator {:?}", op.token),
                };

                match res {
                    Ok(value) => Ok(value),
                    Err(err) => Err(AplError::new(err.to_string(), 0)),
                }
    
            }
            Expr::Variable(t) => todo!("Primitive {:?} not implemented.", t.token),
        }
    }
}
