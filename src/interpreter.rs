use crate::apl_type::AplType;
use crate::err::AplError;
use crate::expr::Expr;
use crate::token_type::TokenType;

use crate::apl_type::extract_scalar;
use crate::apl_type::AplArray;
use crate::apl_type::AplEnclose;
use crate::apl_type::Scalar;

use crate::primitives::dyadic::*;
use crate::primitives::monadic::*;

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
        match e {
            Expr::Enclose(ref t) => {
                let shape = vec![t.len()];

                let values = t
                    .iter()
                    .map(|x| self.evaluate(x).unwrap())
                    .collect::<Vec<AplType>>();

                Ok(AplType::Enclose(AplEnclose { values, shape }))
            }
            Expr::Array(ref t) => {
                let shape = vec![t.len()];

                let values = t
                    .iter()
                    .map(|x| extract_scalar(self.evaluate(x).unwrap()))
                    .collect::<Vec<Scalar>>();

                Ok(AplType::Array(AplArray { values, shape }))
            }
            Expr::Literal(ref t) => Ok(t.clone()),
            Expr::Grouping(ref expr) => self.evaluate(expr),
            Expr::Dyadic(ref left, ref op, ref right) => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                let res = match op.token {
                    TokenType::Plus => add(left, right),
                    _ => todo!("Dyadic operator {:#?}", op.token),
                };

                match res {
                    Ok(value) => Ok(value),
                    Err(err) => Err(AplError::new(err.to_string(), 0)),
                }
            }
            Expr::Monadic(ref op, ref right) => {
                let right = self.evaluate(right)?;

                let res = match op.token {
                    TokenType::Rho => shape(right),
                    TokenType::LeftShoe => enclose(right),
                    _ => todo!("Monadic operator {:#?}", op.token),
                };

                match res {
                    Ok(value) => Ok(value),
                    Err(err) => Err(AplError::new(err.to_string(), 0)),
                }
            }
            Expr::Variable(t) => todo!("Primitive {:#?} not implemented.", t.token),
        }
    }
}
