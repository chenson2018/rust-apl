use std::fmt;

#[derive(Debug, Clone)]
pub enum AplType {
    String(String),
    Number(f64),
    Name(String),
    Array(Vec<AplType>),
}

impl PartialEq for AplType {
    fn eq(&self, other: &AplType) -> bool {
        match (self, other) {
            (&AplType::String(ref s), &AplType::String(ref o)) => (s == o),
            (&AplType::Number(ref s), &AplType::Number(ref o)) => (s == o),
            (&AplType::Array(ref s), &AplType::Array(ref o)) => (s == o),
            _ => false, // Name is left out here... something feels odd about this...
        }
    }
}

impl fmt::Display for AplType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AplType::String(ref s) => write!(f, "{}", s),
            AplType::Number(ref n) => write!(f, "{}", n),
            AplType::Name(ref b) => write!(f, "{}", b),
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
