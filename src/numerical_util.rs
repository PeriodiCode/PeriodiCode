use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::identities::One;
use num_traits::pow::Pow;
use num_traits::Zero;

pub fn power(radix: u32, exponent: BigInt) -> BigRational {
  match exponent.into_parts() {
      (num_bigint::Sign::Minus, uint) => {
          BigRational::new(BigInt::one(), BigInt::from(radix).pow(uint))
      }
      (num_bigint::Sign::NoSign, _) => BigRational::one(),
      (num_bigint::Sign::Plus, uint) => {
          BigRational::new(BigInt::from(radix).pow(uint), BigInt::one())
      }
  }
}

pub fn floor_as_bigint(s: &BigRational) -> BigInt {
  if *s < Zero::zero() {
      let one: BigInt = One::one();
      (s.numer() - s.denom().clone() + one) / s.denom().clone()
  } else {
      s.numer().clone() / s.denom().clone()
  }
}
