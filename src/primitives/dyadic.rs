use crate::apl_type::AplType;

// I should change this type to a result and then implement a cast into the approprate error reporting so as to isolate the error from the reporting of the error
// the zip allows some implicit skipping... kinda interesting
pub fn add(left: AplType, right: AplType) -> Option<AplType> {
    match (left, right) {
        (AplType::Number(l), AplType::Number(r)) => Some(AplType::Number(l + r)),
        (AplType::Number(r), AplType::Array(l)) => {
            let mut res: Vec<AplType> = Vec::new();

            for a in l {
                res.push(add(AplType::Number(r), a).unwrap());
            }

            Some(AplType::Array(res))
        }
        (AplType::Array(l), AplType::Number(r)) => {
            let mut res: Vec<AplType> = Vec::new();

            for a in l {
                res.push(add(AplType::Number(r), a).unwrap());
            }
            Some(AplType::Array(res))
        }
        (AplType::Array(l), AplType::Array(r)) => {
            let z: Vec<AplType> = l
                .iter()
                .zip(r)
                .map(|(a, b)| add(a.clone(), b.clone()).unwrap())
                .collect();
            Some(AplType::Array(z))
        }
        _ => None,
    }
}
