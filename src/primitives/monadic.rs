use crate::apl_type::AplEnclose;
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

pub fn enclose(right: AplType) -> Result<AplType, &'static str> {
    match right {
        AplType::Name(_) => panic!("Call on unevaluated name."),
        AplType::Null => panic!("Call on Null"),
        AplType::Scalar(_) => Ok(right),
        AplType::Enclose(x) => Ok(AplType::Enclose(AplEnclose {
            values: vec![AplType::Enclose(x)],
            shape: vec![],
        })),
        AplType::Array(x) => Ok(AplType::Enclose(AplEnclose {
            values: vec![AplType::Array(x)],
            shape: vec![],
        })),
    }
}
