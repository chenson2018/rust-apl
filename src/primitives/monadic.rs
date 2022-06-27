use crate::apl_type::AplArray;
use crate::apl_type::AplType;
//use crate::apl_type::Scalar;

use crate::err::ErrCtx;

// TODO: handle shape of arrays
// TODO: handle difference between ⍴'' and ⍴0

pub fn shape(_right: AplType) -> Result<AplType, ErrCtx> {
    todo!()
    //    match right {
    //        AplType::Enclose(r) => Ok(AplType::Array(vec![Scalar::Number(r.len() as f64)])), //eventually get from shape...
    //        AplType::Array(r) => Ok(r.shape),
    //        AplType::Name(r) => Err("Undefined name {}", r),
    //        AplType::Scalar(_) => todo!(),
    //    }
}

pub fn enclose(right: AplType) -> Result<AplType, ErrCtx> {
    match right {
        AplType::Name(_) => panic!("Call on unevaluated name."),
        AplType::Null => panic!("Call on Null"),
        AplType::Scalar(_) => Ok(right),
        AplType::Array(x) => Ok(AplType::Array(AplArray {
            values: vec![AplType::Array(x)],
            shape: vec![],
        })),
    }
}

pub fn negate(right: AplType) -> Result<AplType, ErrCtx> {
    fn f(right: f64) -> f64 {
        -right
    }
    right.scalar_monadic(&f)
}
