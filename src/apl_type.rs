use ndarray::Array;
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::IxDynImpl;
use ndarray::OwnedRepr;

use std::fmt;

#[derive(Debug, Clone)]
pub enum AplType {
    Scalar(Scalar),
    Name(String),
    Array(AplArray),
    Enclose(AplEnclose),
    Null
}

#[derive(Debug, Clone)]
pub enum Scalar {
    Number(f64),
    String(String),
}

// TODO: this same thing needs to be done for Enclose!!!
// TODO: handle getting AplArray.shape as a valid AplType (usize is the problem)

#[derive(Debug, Clone)]
pub struct AplArray {
    pub values: Vec<Scalar>,
    pub shape: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct AplEnclose {
    pub values: Vec<AplType>,
    pub shape: Vec<usize>,
}

// these are some helpers for converting back/forth from interpreted types to ndarrays

pub fn extract_scalar(apl: AplType) -> Scalar {
    match apl {
        AplType::Scalar(x) => x,
        _ => panic!("extract_scalar received a non-scalar"),
    }
}

pub fn extract_f64(apl: Vec<Scalar>) -> Vec<f64> {
    apl.iter()
        .map(|x| match *x {
            Scalar::Number(x) => x,
            _ => panic!("extract_scalar received a non-scalar"),
        })
        .collect::<Vec<f64>>()
}

impl From<AplArray> for ArrayBase<OwnedRepr<f64>, Dim<IxDynImpl>> {
    fn from(v: AplArray) -> Self {
        Array::from_shape_vec(v.shape, extract_f64(v.values)).unwrap()
    }
}

impl From<ArrayBase<OwnedRepr<f64>, Dim<IxDynImpl>>> for AplArray {
    fn from(v: ArrayBase<OwnedRepr<f64>, Dim<IxDynImpl>>) -> Self {
        let shape = v.shape().to_vec();
        let values = v.into_iter().map(Scalar::Number).collect::<Vec<Scalar>>();
        AplArray { values, shape }
    }
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
            (&Scalar::Number(_), &Scalar::String(_)) | (&Scalar::String(_), &Scalar::Number(_)) => {
                false
            }
        }
    }
}

impl PartialEq for AplType {
    fn eq(&self, other: &AplType) -> bool {
        match (self, other) {
            (&AplType::Scalar(ref s), &AplType::Scalar(ref o)) => (s == o),
            (&AplType::Enclose(ref s), &AplType::Enclose(ref o)) => (s.values == o.values),
            (&AplType::Array(ref s), &AplType::Array(ref o)) => (s.values == o.values),
            _ => false, // Name is left out here... something feels odd about this...
        }
    }
}

impl fmt::Display for AplType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AplType::Null => Ok(()),
            AplType::Scalar(ref s) => write!(f, "{}", s),
            AplType::Name(ref b) => write!(f, "{}", b),
            AplType::Enclose(ref vec) => {
                write!(f, "<<")?;
                for v in &vec.values {
                    write!(f, " {} ", v)?;
                }
                write!(f, ">>")?;
                Ok(())
            }
            AplType::Array(ref vec) => {
                write!(f, "[")?;
                for v in &vec.values {
                    write!(f, " {} ", v)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}
