use crate::token::AplType;
use crate::expr::Expr;
use crate::err::AplError;
use crate::token_type::TokenType;

#[derive(Debug)]
pub enum InterpreterError {
    AplError(AplError),
    AplErrors(Vec<AplError>),
    Return(AplType)
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

// this is a macro to extract a value from a certain AplType, panicing (should make an error) if got the wrong type.
macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

#[derive(Clone)]
pub struct Interpreter { }

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { }
    }

    pub fn interpret(&mut self, e: &Expr) -> Result<AplType,InterpreterError> {
      self.evaluate(e)
    }

    fn evaluate(&mut self,e: &Expr) -> Result<AplType,InterpreterError> {
      match e {
        &Expr::Array(ref t) => {
          let res =  t.into_iter().map(|x| self.evaluate(x).unwrap() ).collect::<Vec<AplType>>();
          Ok(AplType::Array(res))
        },
        &Expr::Literal(ref t) => Ok(t.clone()),
        &Expr::Grouping(ref expr) => self.evaluate(expr),
        &Expr::Dyadic(ref left,ref op,ref right) => {
          // here I would eventually like a general way to define a function and then import, APL has too many to shove it all here
          let left = self.evaluate(left)?;
          let right = self.evaluate(right)?;

          match op.token {
              // this fails: 1+((4 5 6) 4 5)
              TokenType::Plus => {
                match (left,right) {
                  (AplType::Number(l),AplType::Number(r)) => Ok(AplType::Number(l+r)),
                  // this is really only working for vectors right now, need to go back and add a size dimension to AplType::Array
                  // I wonder if there is a clean way to not have to write left and right versions of this???
                  (AplType::Array(l),AplType::Number(r))  => {
                    Ok(
                      AplType::Array(
                        l.into_iter()
                         .map(|x| AplType::Number(r + as_variant!(x, AplType::Number).unwrap()))
                         .collect()))
                  },
                  (AplType::Number(r),AplType::Array(l))  => {
                    Ok(
                      AplType::Array(
                        l.into_iter()
                         .map(|x| AplType::Number(r + as_variant!(x, AplType::Number).unwrap()))
                         .collect()))
                  },
                  (AplType::Array(l),AplType::Array(r))  => {
                    Ok(
                      AplType::Array(
                        l.iter().zip(&r).map(|(a, b)| AplType::Number(as_variant!(a, AplType::Number).unwrap() + as_variant!(b, AplType::Number).unwrap()) ).collect()
                      )
                    )
                  },
                  _ => panic!("string args to plus") 
                }
              },
              _ => todo!("need more dyadic operators..."),
          }
        }
        _ => todo!("more primitive stuff..."),
      }
    }
}
