use crate::apl_type::AplType;
//use crate::apl_type::Scalar;

// TODO: handle shape of arrays
// TODO: handle difference between ⍴'' and ⍴0

pub fn shape(_right: AplType) -> Result<AplType, &'static str> {
    todo!()
    //    match right {
    //        AplType::Enclose(r) => Ok(AplType::Array(vec![Scalar::Number(r.len() as f64)])), //eventually get from shape...
    //        AplType::Array(r) => Ok(r.shape),
    //        AplType::Name(r) => Err("Undefined name {}", r),
    //        AplType::Scalar(_) => todo!(),
    //    }
}
