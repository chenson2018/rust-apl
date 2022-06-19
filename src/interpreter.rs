use crate::apl_type::AplType;
use crate::err::AplError;
use crate::expr::Expr;
use crate::token_type::TokenType;

use crate::primitives::dyadic::add;
use crate::primitives::monadic::shape;

#[derive(Debug)]
pub enum InterpreterError {
    AplError(AplError),
    AplErrors(Vec<AplError>),
    Return(AplType),
}

impl From<AplError> for InterpreterError {
    fn from(v: AplError) -> InterpreterError {
        InterpreterError::AplError(v)
    }
}

impl From<Vec<AplError>> for InterpreterError {
    fn from(v: Vec<AplError>) -> InterpreterError {
        InterpreterError::AplErrors(v)
    }
}

#[derive(Clone)]
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&mut self, e: &Expr) -> Result<AplType, InterpreterError> {
        self.evaluate(e)
    }

    fn evaluate(&mut self, e: &Expr) -> Result<AplType, InterpreterError> {
        match e {
            &Expr::Array(ref t) => {
                let res = t
                    .into_iter()
                    .map(|x| self.evaluate(x).unwrap())
                    .collect::<Vec<AplType>>();
                Ok(AplType::Array(res))
            }
            &Expr::Literal(ref t) => Ok(t.clone()),
            &Expr::Grouping(ref expr) => self.evaluate(expr),
            &Expr::Dyadic(ref left, ref op, ref right) => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match op.token {
                    TokenType::Plus => Ok(add(left, right).unwrap()),
                    _ => todo!("need more dyadic operators..."),
                }
            }
            &Expr::Monadic(ref op, ref right) => {
                let right = self.evaluate(right)?;

                match op.token {
                    TokenType::Rho => Ok(shape(right).unwrap()),
                    _ => todo!("need more mondic operators..."),
                }
            }
            _ => todo!("more primitive stuff..."),
        }
    }
}
