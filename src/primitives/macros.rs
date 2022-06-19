// this is a macro to extract a value from a certain AplType, panicing (should really make an error!) if got the wrong type.
macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}
