use crate::apl_type::AplType;
//use crate::apl_type::AplArray;
//use crate::apl_type::Scalar;
//use crate::apl_type::extract_f64;
//use std::iter;

// TODO: propogate errors for nested (see in the extract_f64)
// TODO: support higher dimensional arrays (I think done??)
// TODO: support interger and floating types
// TODO: test for errors on size mismatches

pub fn reshape(_left: AplType, _right: AplType) -> Result<AplType, &'static str> {
    todo!()
    //    let vals = match right {
    //      AplType::Scalar(s) => vec![s],
    //      AplType::Array(s)  => s.values,
    ////      AplType::Enclose(s) => s.values,
    //      _ => panic!(),
    //    };
    //
    //    let shape = match left {
    //      AplType::Scalar(Scalar::Number(s)) => vec![s],
    //      AplType::Array(s) => extract_f64(s.values),
    //      _ => panic!(),
    //    };
    //
    //    let n: f64 = shape.iter().product();
    //
    //    let values: Vec<Scalar> = vals.iter().cycle().take(n as usize).collect();
    //
    //    println!("{:?}", values);
    //
    ////    Ok(AplType::Array(AplArray{ values, shape: shape.iter().map(|x| x.clone() as usize).collect() }))
    //    Ok(AplType::Scalar(Scalar::Number(12.0)))
}

pub fn plus(left: AplType, right: AplType) -> Result<AplType, &'static str> {
    fn f(left: f64, right: f64) -> f64 {
        left + right
    }
    left.scalar_dyadic(right, &f)
}

pub fn minus(left: AplType, right: AplType) -> Result<AplType, &'static str> {
    fn f(left: f64, right: f64) -> f64 {
        left - right
    }
    left.scalar_dyadic(right, &f)
}
