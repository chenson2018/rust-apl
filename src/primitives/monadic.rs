use crate::token::AplType;

pub fn shape(right: AplType) -> Option<AplType> {
  match right {
    //assumes vectors for now
    AplType::Number(_) => Some(AplType::Array(Vec::new())),
    AplType::String(_) => Some(AplType::Array(Vec::new())),
    AplType::Array(r)  => Some(AplType::Array(vec![AplType::Number(r.len() as f64)])),
    AplType::Name(_)   => panic!("Function called on unevaluated name."),
    // need to handle string as char vector better....
    // can I represent a "scalar" type?
  }
}
