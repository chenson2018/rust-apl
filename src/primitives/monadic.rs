use crate::apl_type::AplType;

// TODO: handle shape of arrays
// TODO: handle difference between ⍴'' and ⍴0

pub fn shape(right: AplType) -> Result<AplType, &'static str> {
    match right {
        AplType::Number(_) => Ok(AplType::Array(Vec::new())),
        AplType::String(_) => Ok(AplType::Array(Vec::new())),
        AplType::Array(r) => Ok(AplType::Array(vec![AplType::Number(r.len() as f64)])),
        AplType::Name(_) => panic!("Function called on unevaluated name."),
    }
}
