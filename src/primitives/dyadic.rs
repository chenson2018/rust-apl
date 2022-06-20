use crate::apl_type::AplArray;
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
            let res = ArrayBase::from(l) + ArrayBase::from(r);
            Ok(AplType::Array(AplArray::from(res)))
        }
        (AplType::Scalar(Scalar::Number(l)), AplType::Enclose(r)) => {
            let res: Vec<AplType> = r
                .iter()
                .map(|x| add(AplType::Scalar(Scalar::Number(l)), x.clone()).unwrap())
                .collect();
            Ok(AplType::Enclose(res))
        }
        (AplType::Enclose(r), AplType::Scalar(Scalar::Number(l))) => {
            let res: Vec<AplType> = r
                .iter()
                .map(|x| add(AplType::Scalar(Scalar::Number(l)), x.clone()).unwrap())
                .collect();
            Ok(AplType::Enclose(res))
        }
        (AplType::Enclose(r), AplType::Array(l)) => {
            let res: Vec<AplType> = r
                .iter()
                .zip(l.values)
                .map(|(x, y)| add(x.clone(), AplType::Scalar(y)).unwrap())
                .collect();
            Ok(AplType::Enclose(res))
        }
        (AplType::Array(l), AplType::Enclose(r)) => {
            let res: Vec<AplType> = r
                .iter()
                .zip(l.values)
                .map(|(x, y)| add(x.clone(), AplType::Scalar(y)).unwrap())
                .collect();
            Ok(AplType::Enclose(res))
        }
        (AplType::Enclose(l), AplType::Enclose(r)) => {
            let res: Vec<AplType> = r
                .iter()
                .zip(l)
                .map(|(x, y)| add(x.clone(), y).unwrap())
                .collect();
            Ok(AplType::Enclose(res))
        }
        _ => Err("+ (dyadic) can only take numeric arguments"),
    }
}
