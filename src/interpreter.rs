use crate::apl_type::AplType;
use crate::err::AplError;
use crate::expr::Expr;
use crate::token::TokenType;

//use crate::apl_type::extract_scalar;
use crate::apl_type::AplArray;
//use crate::apl_type::Scalar;
use crate::environment::Environment;

use crate::primitives::dyadic::*;
use crate::primitives::monadic::*;

use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Clone)]
pub struct Interpreter {
    pub env: Box<Environment>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: properly report line of errors

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Box::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, exprs: &Vec<Rc<Expr>>) -> Result<(), AplError> {
        for e in exprs {
            let res = self.evaluate(e);

            match res {
                Ok(val) => {
                    println!("{}", val);
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }

        Ok(())
    }

    // for unit testing
    pub fn eval(&mut self, exprs: &Vec<Rc<Expr>>) -> Result<Vec<AplType>, AplError> {
        let mut v: Vec<AplType> = Vec::new();

        for e in exprs {
            let res = self.evaluate(e);

            match res {
                Ok(val) => {
                    v.push(val);
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }

        Ok(v)
    }

    pub fn evaluate(&mut self, e: &Expr) -> Result<AplType, AplError> {
        match e {
            Expr::Null => Ok(AplType::Null),
            Expr::Array(ref t) => {
                let shape = vec![t.len()];

                let values = t
                    .iter()
                    .map(|x| self.evaluate(x).unwrap())
                    .collect::<Vec<AplType>>();

                Ok(AplType::Array(AplArray { values, shape }))
            }
            Expr::Literal(ref t) => Ok(t.clone()),
            Expr::Grouping(ref expr) => self.evaluate(expr),
            Expr::Dyadic(ref left, ref op, ref right) => {
                let right = self.evaluate(right)?;

                let res = match op.token {
                    // TODO: support variable modification
                    TokenType::LeftArrow => match left.borrow() {
                        Expr::Variable(t) => {
                            self.env.define(&t.lexeme, right);
                            Ok(AplType::Null)
                        }
                        _ => Err("Attempt to modify constant."),
                    },
                    _ => {
                        let left = self.evaluate(left)?;
                        match op.token {
                            TokenType::Plus => plus(left, right),
                            TokenType::Minus => minus(left, right),
                            TokenType::Rho => reshape(left, right),
                            _ => todo!("Dyadic operator {:#?}", op.token),
                        }
                    }
                };

                match res {
                    Ok(value) => Ok(value),
                    Err(err) => Err(AplError::new(err.to_string(), 0)),
                }
            }
            Expr::Monadic(ref op, ref right) => {
                let right = self.evaluate(right)?;

                let res = match op.token {
                    TokenType::Minus => negate(right),
                    TokenType::Rho => shape(right),
                    TokenType::LeftShoe => enclose(right),
                    _ => todo!("Monadic operator {:#?}", op.token),
                };

                match res {
                    Ok(value) => Ok(value),
                    Err(err) => Err(AplError::new(err.to_string(), 0)),
                }
            }
            Expr::Variable(t) => match self.env.get(&t.lexeme) {
                Some(r) => Ok(r),
                None => Err(AplError::new("Variable not found".to_string(), t.line)),
            },
        }
    }
}
