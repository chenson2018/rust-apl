use crate::apl_type::extract_scalar;
use crate::apl_type::AplType;
use crate::apl_type::Scalar;

// TODO: propogate errors for nested (see unwrap)
// TODO: support higher dimensional arrays
// TODO: support interger and floating types
// TODO: test for errors on size mismatches

pub fn add(left: AplType, right: AplType) -> Result<AplType, &'static str> {
    fn helper_a(left: AplType, right: AplType) -> Result<AplType, &'static str> {
        match (left, right) {
            (AplType::Scalar(Scalar::Number(r)), AplType::Array(l)) => {
                let mut res: Vec<AplType> = Vec::new();

                for a in l {
                    let r = add(AplType::Scalar(Scalar::Number(r)), AplType::Scalar(a));

                    match r {
                        Ok(val) => res.push(val),
                        Err(err) => return Err(err),
                    }
                }

                let extract = res
                    .iter()
                    .map(|x| extract_scalar(x.clone()))
                    .collect::<Vec<Scalar>>();

                Ok(AplType::Array(extract))
            }
            _ => panic!("Should only be called on scalar + vector mix"),
        }
    }

    fn helper_e(left: AplType, right: AplType) -> Result<AplType, &'static str> {
        match (left, right) {
            (AplType::Scalar(Scalar::Number(r)), AplType::Enclose(l)) => {
                let mut res: Vec<AplType> = Vec::new();

                for a in l {
                    let r = add(a, AplType::Scalar(Scalar::Number(r)));

                    match r {
                        Ok(val) => res.push(val),
                        Err(err) => return Err(err),
                    }
                }
                Ok(AplType::Enclose(res))
            }
            _ => panic!("Should only be called on scalar + enclose mix"),
        }
    }

    fn helper_ea(left: AplType, right: AplType) -> Result<AplType, &'static str> {
        match (left, right) {
            (AplType::Enclose(l), AplType::Array(r)) => {
                // I want this clean, but propogate the errors up!!!!
                let z: Vec<AplType> = l
                    .iter()
                    .zip(r)
                    .map(|(a, b)| add(a.clone(), AplType::Scalar(b)).unwrap())
                    .collect();
                Ok(AplType::Enclose(z))
            }
            _ => panic!("Should only be called on enclose + vector mix"),
        }
    }

    match (left, right) {
        (AplType::Scalar(Scalar::Number(l)), AplType::Scalar(Scalar::Number(r))) => {
            Ok(AplType::Scalar(Scalar::Number(l + r)))
        }
        (AplType::Scalar(Scalar::Number(l)), AplType::Array(r)) => {
            helper_a(AplType::Scalar(Scalar::Number(l)), AplType::Array(r))
        }
        (AplType::Array(r), AplType::Scalar(Scalar::Number(l))) => {
            helper_a(AplType::Scalar(Scalar::Number(l)), AplType::Array(r))
        }
        (AplType::Array(l), AplType::Array(r)) => {
            let z: Vec<AplType> = l
                .iter()
                .zip(r)
                .map(|(a, b)| add(AplType::Scalar(a.clone()), AplType::Scalar(b)).unwrap())
                .collect();

            let extract = z
                .iter()
                .map(|x| extract_scalar(x.clone()))
                .collect::<Vec<Scalar>>();

            Ok(AplType::Array(extract))
        }
        (AplType::Scalar(Scalar::Number(l)), AplType::Enclose(r)) => {
            helper_e(AplType::Scalar(Scalar::Number(l)), AplType::Enclose(r))
        }
        (AplType::Enclose(r), AplType::Scalar(Scalar::Number(l))) => {
            helper_e(AplType::Scalar(Scalar::Number(l)), AplType::Enclose(r))
        }
        (AplType::Enclose(l), AplType::Array(r)) => {
            helper_ea(AplType::Enclose(l), AplType::Array(r))
        }
        (AplType::Array(r), AplType::Enclose(l)) => {
            helper_ea(AplType::Enclose(l), AplType::Array(r))
        }
        (AplType::Enclose(r), AplType::Enclose(l)) => {
            let z: Vec<AplType> = l
                .iter()
                .zip(r)
                .map(|(a, b)| add(a.clone(), b).unwrap())
                .collect();
            Ok(AplType::Enclose(z))
        }
        _ => Err("+ (dyadic) can only take numeric arguments"),
    }
}
