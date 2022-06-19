use crate::apl_type::AplType;

// TODO: handle error for two array match case
// TODO: support higher dimensional arrays
// TODO: support interger and floating types
// TODO: test for errors on size mismatches

pub fn add(left: AplType, right: AplType) -> Result<AplType, &'static str> {
    fn helper(left: AplType, right: AplType) -> Result<AplType, &'static str> {
        match (left, right) {
            (AplType::Number(r), AplType::Array(l)) => {
                let mut res: Vec<AplType> = Vec::new();

                for a in l {
                    let r = add(AplType::Number(r), a);
                    match r {
                        Ok(val) => res.push(val),
                        Err(err) => return Err(err),
                    }
                }
                Ok(AplType::Array(res))
            }
            _ => panic!("Should only be called on scalar + vector mix"),
        }
    }

    match (left, right) {
        (AplType::Number(l), AplType::Number(r)) => Ok(AplType::Number(l + r)),
        (AplType::Number(r), AplType::Array(l)) => helper(AplType::Number(r), AplType::Array(l)),
        (AplType::Array(l), AplType::Number(r)) => helper(AplType::Number(r), AplType::Array(l)),
        (AplType::Array(l), AplType::Array(r)) => {
            let z: Vec<AplType> = l
                .iter()
                .zip(r)
                .map(|(a, b)| add(a.clone(), b).unwrap())
                .collect();
            Ok(AplType::Array(z))
        }
        _ => Err("+ (dyadic) can only take numeric arguments"),
    }
}
