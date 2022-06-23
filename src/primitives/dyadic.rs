use crate::apl_type::AplType;

// TODO: propogate errors for nested (see in the extract_f64)
// TODO: support higher dimensional arrays (I think done??)
// TODO: support interger and floating types
// TODO: test for errors on size mismatches

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
