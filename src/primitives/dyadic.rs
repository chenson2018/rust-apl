use crate::apl_type::AplArray;
use crate::apl_type::AplEnclose;
use crate::apl_type::AplType;
use crate::apl_type::Scalar;

use ndarray::ArrayBase;

// TODO: propogate errors for nested (see in the extract_f64)
// TODO: support higher dimensional arrays (I think done??)
// TODO: support interger and floating types
// TODO: test for errors on size mismatches

pub fn add(left: AplType, right: AplType) -> Result<AplType, &'static str> {
    match (left, right) {
        (AplType::Scalar(Scalar::Number(l)), AplType::Scalar(Scalar::Number(r))) => {
            Ok(AplType::Scalar(Scalar::Number(l + r)))
        }
        (AplType::Scalar(Scalar::Number(l)), AplType::Array(r)) => {
            let res = ArrayBase::from(r) + l;
            Ok(AplType::Array(AplArray::from(res)))
        }
        (AplType::Array(r), AplType::Scalar(Scalar::Number(l))) => {
            let res = ArrayBase::from(r) + l;
            Ok(AplType::Array(AplArray::from(res)))
        }
        (AplType::Array(l), AplType::Array(r)) => {
            if l.shape != r.shape {
                return Err("Incompatibile shapes");
            }

            let res = ArrayBase::from(l) + ArrayBase::from(r);
            Ok(AplType::Array(AplArray::from(res)))
        }
        (AplType::Scalar(Scalar::Number(l)), AplType::Enclose(r)) => {
            let shape = r.shape;

            let values: Vec<AplType> = r
                .values
                .iter()
                .map(|x| add(AplType::Scalar(Scalar::Number(l)), x.clone()).unwrap())
                .collect();

            Ok(AplType::Enclose(AplEnclose { values, shape }))
        }
        (AplType::Enclose(r), AplType::Scalar(Scalar::Number(l))) => {
            let shape = r.shape;

            let values: Vec<AplType> = r
                .values
                .iter()
                .map(|x| add(AplType::Scalar(Scalar::Number(l)), x.clone()).unwrap())
                .collect();

            Ok(AplType::Enclose(AplEnclose { values, shape }))
        }
        (AplType::Enclose(r), AplType::Array(l)) => {
            if l.shape != r.shape {
                return Err("Incompatibile shapes");
            }

            let shape = r.shape;

            let values: Vec<AplType> = r
                .values
                .iter()
                .zip(l.values)
                .map(|(x, y)| add(x.clone(), AplType::Scalar(y)).unwrap())
                .collect();

            Ok(AplType::Enclose(AplEnclose { values, shape }))
        }
        (AplType::Array(l), AplType::Enclose(r)) => {
            if l.shape != r.shape {
                return Err("Incompatibile shapes");
            }

            let shape = r.shape;

            let values: Vec<AplType> = r
                .values
                .iter()
                .zip(l.values)
                .map(|(x, y)| add(x.clone(), AplType::Scalar(y)).unwrap())
                .collect();

            Ok(AplType::Enclose(AplEnclose { values, shape }))
        }
        (AplType::Enclose(l), AplType::Enclose(r)) => {
            if l.shape != r.shape {
                return Err("Incompatibile shapes");
            }

            let shape = r.shape;

            let values: Vec<AplType> = r
                .values
                .iter()
                .zip(l.values)
                .map(|(x, y)| add(x.clone(), y).unwrap())
                .collect();

            Ok(AplType::Enclose(AplEnclose { values, shape }))
        }
        _ => Err("+ (dyadic) can only take numeric arguments"),
    }
}
