use crate::apl_type::AplType;
use crate::apl_type::Scalar;
use crate::apl_type::extract_scalar;

// TODO: handle error for two array match case
// TODO: support higher dimensional arrays
// TODO: support interger and floating types
// TODO: test for errors on size mismatches

pub fn add(left: AplType, right: AplType) -> Result<AplType, &'static str> {

    fn helper(left: AplType, right: AplType) -> Result<AplType, &'static str> {
        match (left, right) {
          (AplType::Scalar(Scalar::Number(r)), AplType::Array(l)) => {
            let mut res: Vec<AplType> = Vec::new();

            for a in l {
              let r = add(AplType::Scalar(Scalar::Number(r)), AplType::Scalar(a));

              match r {
                Ok(val) => res.push(val),
                Err(err) => return Err(err),
              }
            }

            let extract = res
                    .iter()
                    .map(|x| extract_scalar(x.clone()) )
                    .collect::<Vec<Scalar>>();

            Ok(AplType::Array(extract))
          },
          _ => panic!("Should only be called on scalar + vector mix"),
        }
    }

    match (left, right) {
      (AplType::Scalar(Scalar::Number(l)), AplType::Scalar(Scalar::Number(r))) => Ok(AplType::Scalar(Scalar::Number(l+r))),
      (AplType::Scalar(Scalar::Number(l)), AplType::Array(r)) => helper(AplType::Scalar(Scalar::Number(l)), AplType::Array(r)),
      (AplType::Array(r), AplType::Scalar(Scalar::Number(l))) => helper(AplType::Scalar(Scalar::Number(l)), AplType::Array(r)),
      (AplType::Array(l), AplType::Array(r)) => {
          let z: Vec<AplType> = l
              .iter()
              .zip(r)
              .map(|(a, b)| add(AplType::Scalar(a.clone()), AplType::Scalar(b)).unwrap())
              .collect();

            let extract = z
                    .iter()
                    .map(|x| extract_scalar(x.clone()) )
                    .collect::<Vec<Scalar>>();


          Ok(AplType::Array(extract))
      }
      _ => Err("+ (dyadic) can only take numeric arguments"),
      //_ => todo!(),
    }






//
//    match (left, right) {
//        (AplType::Number(l), AplType::Number(r)) => Ok(AplType::Number(l + r)),
//        (AplType::Number(r), AplType::Array(l)) => helper(AplType::Number(r), AplType::Array(l)),
//        (AplType::Array(l), AplType::Number(r)) => helper(AplType::Number(r), AplType::Array(l)),
//        (AplType::Array(l), AplType::Array(r)) => {
//            let z: Vec<AplType> = l
//                .iter()
//                .zip(r)
//                .map(|(a, b)| add(a.clone(), b).unwrap())
//                .collect();
//            Ok(AplType::Array(z))
//        }
//        _ => Err("+ (dyadic) can only take numeric arguments"),
//    }
}
