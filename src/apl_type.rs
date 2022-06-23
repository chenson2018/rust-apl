use ndarray::Array;
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::IxDynImpl;
use ndarray::OwnedRepr;
use ndarray::Zip;

use std::fmt;
use std::iter;

#[derive(Debug, Clone)]
pub enum AplType {
    Scalar(Scalar),
    Name(String),
    Array(AplArray),
    Enclose(AplEnclose),
    Null,
}

#[derive(Debug, Clone)]
pub enum Scalar {
    Number(f64),
    String(String),
}

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

impl AplType {
    pub fn scalar_monadic(self, f: &dyn Fn(f64) -> f64) -> Result<AplType, &'static str> {
        match self {
            AplType::Scalar(Scalar::Number(r)) => Ok(AplType::Scalar(Scalar::Number(f(r)))),
            AplType::Array(r) => {
                let res = ArrayBase::from(r).mapv_into(f);
                Ok(AplType::Array(AplArray::from(res)))
            }
            AplType::Enclose(r) => {
                let shape = r.shape;

                let values: Vec<AplType> = r
                    .values
                    .iter()
                    .map(|x| x.clone().scalar_monadic(f).unwrap())
                    .collect();

                Ok(AplType::Enclose(AplEnclose { values, shape }))
            }
            _ => Err("non numeric argument to scalar function"),
        }
    }

    pub fn scalar_dyadic(
        self,
        other: AplType,
        f: &dyn Fn(f64, f64) -> f64,
    ) -> Result<AplType, &'static str> {
        match (self, other) {
            (AplType::Scalar(Scalar::Number(l)), AplType::Scalar(Scalar::Number(r))) => {
                Ok(AplType::Scalar(Scalar::Number(f(l, r))))
            }
            (AplType::Array(r), AplType::Scalar(Scalar::Number(l))) => {
                let res = ArrayBase::from(r).mapv_into(|v| f(v, l));
                Ok(AplType::Array(AplArray::from(res)))
            }
            (AplType::Scalar(Scalar::Number(l)), AplType::Array(r)) => {
                let res = ArrayBase::from(r).mapv_into(|v| f(l, v));
                Ok(AplType::Array(AplArray::from(res)))
            }
            (AplType::Scalar(Scalar::Number(l)), AplType::Enclose(r)) => {
                let shape = r.shape;

                let values: Vec<AplType> = r
                    .values
                    .iter()
                    .map(|x| {
                        AplType::Scalar(Scalar::Number(l))
                            .scalar_dyadic(x.clone(), f)
                            .unwrap()
                    })
                    .collect();

                Ok(AplType::Enclose(AplEnclose { values, shape }))
            }
            (AplType::Enclose(r), AplType::Scalar(Scalar::Number(l))) => {
                let shape = r.shape;

                let values: Vec<AplType> = r
                    .values
                    .iter()
                    .map(|x| {
                        x.clone()
                            .scalar_dyadic(AplType::Scalar(Scalar::Number(l)), f)
                            .unwrap()
                    })
                    .collect();

                Ok(AplType::Enclose(AplEnclose { values, shape }))
            }
            (AplType::Enclose(r), AplType::Array(l)) => {
                if r.values.len() == 1 {
                    let shape = l.shape.clone();
                    let mut rep = iter::repeat(r.values[0].clone());

                    let values: Vec<AplType> = l
                        .values
                        .iter()
                        .map(|x| {
                            AplType::Scalar(x.clone())
                                .scalar_dyadic(rep.next().unwrap(), f)
                                .unwrap()
                        })
                        .collect();

                    Ok(AplType::Enclose(AplEnclose { values, shape }))
                } else {
                    let shape = r.shape;

                    let values: Vec<AplType> = r
                        .values
                        .iter()
                        .zip(l.values)
                        .map(|(x, y)| x.clone().scalar_dyadic(AplType::Scalar(y), f).unwrap())
                        .collect();
                    Ok(AplType::Enclose(AplEnclose { values, shape }))
                }
            }
            (AplType::Array(l), AplType::Enclose(r)) => {
                if r.values.len() == 1 {
                    let shape = l.shape.clone();
                    let mut rep = iter::repeat(r.values[0].clone());

                    let values: Vec<AplType> = l
                        .values
                        .iter()
                        .map(|x| {
                            rep.next()
                                .unwrap()
                                .scalar_dyadic(AplType::Scalar(x.clone()), f)
                                .unwrap()
                        })
                        .collect();

                    Ok(AplType::Enclose(AplEnclose { values, shape }))
                } else {
                    let shape = r.shape;

                    let values: Vec<AplType> = r
                        .values
                        .iter()
                        .zip(l.values)
                        .map(|(x, y)| AplType::Scalar(y).scalar_dyadic(x.clone(), f).unwrap())
                        .collect();
                    Ok(AplType::Enclose(AplEnclose { values, shape }))
                }
            }

            (AplType::Array(l), AplType::Array(r)) => {
                if l.shape != r.shape {
                    return Err("Incompatibile shapes");
                }
                let mut r2: ArrayBase<OwnedRepr<f64>, Dim<IxDynImpl>> =
                    ArrayBase::zeros(r.shape.clone());

                Zip::from(&mut r2)
                    .and(&ArrayBase::from(l))
                    .and(&ArrayBase::from(r))
                    .for_each(|a, &b, &c| {
                        *a = f(b, c);
                    });

                Ok(AplType::Array(AplArray::from(r2)))
            }

            (AplType::Enclose(l), AplType::Enclose(r)) => {
                let shape = r.shape;

                let values: Vec<AplType> = l
                    .values
                    .iter()
                    .zip(r.values)
                    .map(|(x, y)| x.clone().scalar_dyadic(y, f).unwrap())
                    .collect();

                Ok(AplType::Enclose(AplEnclose { values, shape }))
            }

            _ => Err("non numeric argument to scalar function"),
        }
    }
}

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
