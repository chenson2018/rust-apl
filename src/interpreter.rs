use crate::apl_type::AplType;
use crate::err::AplError;
use crate::err::ErrCtx;
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

    // for unit testing
    pub fn interpret(&mut self, exprs: &Vec<Rc<Expr>>) -> Result<Vec<AplType>, AplError> {
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
            Expr::Literal(ref t) => match &t.literal {
                Some(x) => Ok(x.clone()),
                None => panic!("received a literal with no value"),
            },
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
                        _ => Err(ErrCtx {
                            err: "Attempt to modify constant".to_string(),
                            message: "interpreter".to_string(),
                            label: "invalid assignment".to_string(),
                        }),
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
                    Err(ctx) => Err(AplError::with_pos(
                        ctx.err,
                        op.line,
                        op.start,
                        op.end,
                        ctx.label,
                        ctx.message,
                    )),
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
                    Err(ctx) => Err(AplError::with_pos(
                        ctx.err,
                        op.line,
                        op.start,
                        op.end,
                        ctx.label,
                        ctx.message,
                    )),
                }
            }
            Expr::Variable(t) => match self.env.get(&t.lexeme) {
                Some(r) => Ok(r),
                None => Err(AplError::with_pos(
                    "Variable not found".to_string(),
                    t.line,
                    t.start,
                    t.end,
                    "this variable is not defined".to_string(),
                    "interpreter".to_string(),
                )),
            },
        }
    }
}
