use crate::apl_type::AplType;
use crate::apl_type::Scalar;

// TODO: handle shape of arrays
// TODO: handle difference between ⍴'' and ⍴0

pub fn shape(right: AplType) -> Result<AplType, &'static str> {
    match right {
        AplType::Enclose(r) => Ok(
                                AplType::Array(
                                    vec![ Scalar::Number(r.len() as f64)
                                        ] )),
        AplType::Name(_) => panic!("Function called on unevaluated name."),
        AplType::Scalar(_) => todo!(),
        AplType::Array(_) => todo!(),
    }
}
