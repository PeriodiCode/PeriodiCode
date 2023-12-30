use num_bigint::BigInt;
use num_bigint::BigUint;
use num_bigint::ParseBigIntError;
use num_rational::BigRational;
use num_traits::identities::One;
use num_traits::pow::Pow;
use num_traits::Num;
use num_traits::Zero;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::numerical_util::power;

fn bigint_from_possibly_empty_str_radix(str: &str, radix: u32) -> Result<BigInt, ParseBigIntError> {
  BigInt::from_str_radix(if str.is_empty() { "0" } else { str }, radix)
}

fn strip_radix_prefix(input: &str) -> (&str, Option<u32>) {
  if input.starts_with("0v") {
      (input.strip_prefix("0v").unwrap(), Some(20))
  } else if input.starts_with("0x") {
      (input.strip_prefix("0x").unwrap(), Some(16))
  } else if input.starts_with("0z") {
      (input.strip_prefix("0z").unwrap(), Some(12))
  } else if input.starts_with("0d") {
      (input.strip_prefix("0d").unwrap(), Some(10))
  } else if input.starts_with("0o") {
      (input.strip_prefix("0o").unwrap(), Some(8))
  } else if input.starts_with("0s") {
      (input.strip_prefix("0s").unwrap(), Some(6))
  } else if input.starts_with("0qn") {
      (input.strip_prefix("0qn").unwrap(), Some(5))
  } else if input.starts_with("0qt") {
      (input.strip_prefix("0qt").unwrap(), Some(4))
  } else if input.starts_with("0t") {
      (input.strip_prefix("0t").unwrap(), Some(3))
  } else if input.starts_with("0b") {
      (input.strip_prefix("0b").unwrap(), Some(2))
  } else {
      (input, None)
  }
}

pub fn parse_numeric_literal_with_radix_context(
  input: &str,
  radix_context: u32,
) -> Result<(BigRational, &str), &'static str> {
  let (input, literal_own_radix) = strip_radix_prefix(input);
  parse_numeric_literal_with_both_contexts(input, radix_context, literal_own_radix)
}

fn parse_numeric_literal_with_both_contexts(
  input: &str,
  external_radix_context: u32,
  literal_own_radix: Option<u32>,
) -> Result<(BigRational, &str), &'static str> {
  /**
   * exponent:
   * `e` or `xp`: multiplies the number by the power of the literal's own radix. `e` can only be used if the base is less than fifteen
   * `p`: multiplies the number by power of two.
   *
   *  The digits following `e`, `xp` or `p` is interpreted with the power of radix (taken from the external context).
   *  - Hence, in decimal context, 0x1.0p10 == 1024.0
   */

  static RE_ALLOWING_E: Lazy<Regex> = Lazy::new(|| {
      Regex::new(r"^(?<integral>[0-9a-dA-D]*)(?<dot>\.(?<before_rep>[0-9a-dA-D]*)(?<rep_digits>(r[0-9a-dA-D]*)?))?(?<exponent>((e|xp|p)(\+|-)?[0-9a-dA-D]+)?)").unwrap()
  });

  static RE_FORBIDDING_E: Lazy<Regex> = Lazy::new(|| {
      Regex::new(r"^(?<integral>[0-9a-oA-O]*)(?<dot>\.(?<before_rep>[0-9a-oA-O]*)(?<rep_digits>(r[0-9a-oA-O]*)?))?(?<exponent>((xp|p)(\+|-)?[0-9a-oA-O]+)?)").unwrap()
  });

  let literal_own_radix = literal_own_radix.unwrap_or(external_radix_context);

  let caps = if literal_own_radix < 15 {
      RE_ALLOWING_E
          .captures(input)
          .ok_or("No parse as a numeric literal")?
  } else {
      RE_FORBIDDING_E
          .captures(input)
          .ok_or("No parse as a numeric literal")?
  };

  let whole = caps.get(0).unwrap().as_str();
  let integral = caps.name("integral").unwrap().as_str();
  let (before_rep, repeating_digits) = match caps.name("dot") {
      Some(u) => {
          if u.as_str() == "." && integral.is_empty() {
              return Err("\"A single dot `.`, optionally followed by exponent\" is forbidden");
          }

          (
              caps.name("before_rep").unwrap().as_str(),
              caps.name("rep_digits").unwrap().as_str(),
          )
      }
      None => ("", ""),
  };

  let exponent = caps.name("exponent").unwrap().as_str();

  // println!("whole: {:?}", whole);

  let integral_part: BigRational = BigRational::new(
      bigint_from_possibly_empty_str_radix(integral, literal_own_radix).unwrap(),
      BigInt::one(),
  );
  // println!("integral_part: {:?}", integral_part);

  let scaling = BigInt::from(literal_own_radix).pow(BigUint::from(before_rep.len()));

  let before_rep_part: BigRational = BigRational::new(
      bigint_from_possibly_empty_str_radix(before_rep, literal_own_radix).unwrap(),
      scaling.clone(),
  );
  // println!("before_rep_part: {:?}", before_rep_part);

  let repeating_digits_part: BigRational = if repeating_digits.starts_with('r') {
      BigRational::new(
          bigint_from_possibly_empty_str_radix(
              repeating_digits.strip_prefix('r').unwrap(),
              literal_own_radix,
          )
          .unwrap(),
          scaling
              * (BigInt::from(literal_own_radix).pow(BigUint::from(repeating_digits.len() - 1))
                  - BigInt::one()),
      )
  } else {
      BigRational::zero()
  };
  // println!("repeating_digits_part: {:?}", repeating_digits_part);

  /* what follows the `e`, `p` or `xp` is interpreted using the external context */
  let exponent: BigRational = if exponent.starts_with('e') {
      // power of radix
      let exponent =
          BigInt::from_str_radix(exponent.strip_prefix('e').unwrap(), external_radix_context)
              .unwrap();
      power(literal_own_radix, exponent)
  } else if exponent.starts_with("xp") {
      let exponent =
          BigInt::from_str_radix(exponent.strip_prefix("xp").unwrap(), external_radix_context)
              .unwrap();
      power(literal_own_radix, exponent)
  } else if exponent.starts_with('p') {
      // power of 2
      let exponent =
          BigInt::from_str_radix(exponent.strip_prefix('p').unwrap(), external_radix_context)
              .unwrap();
      power(2, exponent)
  } else {
      BigRational::one()
  };

  let ans = (integral_part + before_rep_part + repeating_digits_part) * exponent;
  Ok((ans, input.strip_prefix(whole).unwrap()))
}
