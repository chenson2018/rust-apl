use std::fmt;

#[derive(Debug, Clone)]
pub enum AplType {
    Scalar(Scalar),
    Name(String),
    Array(Vec<Scalar>),
    Enclose(Vec<AplType>),
}

#[derive(Debug, Clone)]
pub enum Scalar {
  Number(f64),
  String(String),
}

// do this eventually???
//#[derive(Debug, Clone)]
//pub struct Array {
//  pub values: Vec<Scalar>,
//  pub shape: Vec<usize>,
//}

pub fn extract_scalar(apl: AplType) -> Scalar {
  match apl { AplType::Scalar(x) => x, _ => panic!("extract_scalar received a non-scalar")}
}


impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Scalar::String(ref s) => write!(f, "{}", s),
            Scalar::Number(ref n) => write!(f, "{}", n),
        }
    }
}

impl PartialEq for Scalar {
  fn eq(&self, other: &Scalar) -> bool {
    match (self, other) {
      (&Scalar::Number(ref s), Scalar::Number(ref o)) => (s == o),
      (&Scalar::String(ref s), Scalar::String(ref o)) => (s == o),
      (&Scalar::Number(_), &Scalar::String(_)) | (&Scalar::String(_), &Scalar::Number(_)) => false,
    }
  }
}

impl PartialEq for AplType {
    fn eq(&self, other: &AplType) -> bool {
        match (self, other) {
            (&AplType::Scalar(ref s), &AplType::Scalar(ref o)) => (s == o),
            (&AplType::Enclose(ref s), &AplType::Enclose(ref o)) => (s == o),
            (&AplType::Array(ref s), &AplType::Array(ref o)) => (s == o),
            _ => false, // Name is left out here... something feels odd about this...
        }
    }
}

impl fmt::Display for AplType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AplType::Scalar(ref s) => write!(f, "{}", s),
            AplType::Name(ref b) => write!(f, "{}", b),
            AplType::Enclose(ref vec) => {
                write!(f, "[")?;
                for v in vec {
                    write!(f, " {} ", v)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            AplType::Array(ref vec) => {
                write!(f, "[")?;
                for v in vec {
                    write!(f, " {} ", v)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}
