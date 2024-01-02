use big_s::S;
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

fn map_parsebiginterror<T>(r: Result<T, ParseBigIntError>) -> Result<T, String> {
    match r {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}

fn strip_radix_prefix(input: &str) -> (&str, Option<u32>) {
    if let Some(buf) = input.strip_prefix("0v") {
        (buf, Some(20))
    } else if let Some(buf) = input.strip_prefix("0x") {
        (buf, Some(16))
    } else if let Some(buf) = input.strip_prefix("0z") {
        (buf, Some(12))
    } else if let Some(buf) = input.strip_prefix("0d") {
        (buf, Some(10))
    } else if let Some(buf) = input.strip_prefix("0o") {
        (buf, Some(8))
    } else if let Some(buf) = input.strip_prefix("0s") {
        (buf, Some(6))
    } else if let Some(buf) = input.strip_prefix("0qn") {
        (buf, Some(5))
    } else if let Some(buf) = input.strip_prefix("0qt") {
        (buf, Some(4))
    } else if let Some(buf) = input.strip_prefix("0t") {
        (buf, Some(3))
    } else if let Some(buf) = input.strip_prefix("0b") {
        (buf, Some(2))
    } else {
        (input, None)
    }
}

pub fn parse_numeric_literal_with_radix_context(
    input: &str,
    radix_context: u32,
) -> Result<(BigRational, &str), String> {
    let (input, literal_own_radix) = strip_radix_prefix(input);
    parse_numeric_literal_with_both_contexts(input, radix_context, literal_own_radix)
}

fn parse_numeric_literal_with_both_contexts(
    input: &str,
    external_radix_context: u32,
    literal_own_radix: Option<u32>,
) -> Result<(BigRational, &str), String> {
    /**
     * exponent:
     * `e` or `xp`: multiplies the number by the power of the literal's own radix. `e` can only be used if the base is less than fifteen
     * `p`: multiplies the number by power of two.
     *
     *  The digits following `e`, `xp` or `p` is interpreted with the power of radix (taken from the external context).
     *  - Hence, in decimal context, 0x1.0p10 == 1024.0
     */

    static RE_ALLOWING_E: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(?<integral>[0-9a-dA-D]*)(?<dot>\.(?<before_rep>[0-9a-dA-D]*)(?<rep_digits>(r[0-9a-dA-D]*)?))?(?<exponent>((e|xp|p)(\+|-)?[0-9a-dA-D]+)?)").expect("regex compilation failed")
    });

    static RE_FORBIDDING_E: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(?<integral>[0-9a-oA-O]*)(?<dot>\.(?<before_rep>[0-9a-oA-O]*)(?<rep_digits>(r[0-9a-oA-O]*)?))?(?<exponent>((xp|p)(\+|-)?[0-9a-oA-O]+)?)").expect("regex compilation failed")
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

    let whole = caps.get(0).expect("regex match").as_str();
    if whole.is_empty() {
        return Err(S("No parse as a numeric literal"));
    }

    let integral = caps.name("integral").expect("regex match").as_str();
    let (before_rep, repeating_digits) = match caps.name("dot") {
        Some(u) => {
            if u.as_str() == "." && integral.is_empty() {
                return Err(S(
                    "\"A standalone single dot `.`, optionally followed by exponent\" is forbidden",
                ));
            }

            (
                caps.name("before_rep").expect("regex match").as_str(),
                caps.name("rep_digits").expect("regex match").as_str(),
            )
        }
        None => ("", ""),
    };

    let exponent = caps.name("exponent").expect("regex match").as_str();

    let integral_part: BigRational = BigRational::new(
        map_parsebiginterror(bigint_from_possibly_empty_str_radix(
            integral,
            literal_own_radix,
        ))?,
        BigInt::one(),
    );

    let scaling = BigInt::from(literal_own_radix).pow(BigUint::from(before_rep.len()));

    let before_rep_part: BigRational = BigRational::new(
        map_parsebiginterror(bigint_from_possibly_empty_str_radix(
            before_rep,
            literal_own_radix,
        ))?,
        scaling.clone(),
    );

    let repeating_digits_part: BigRational = if let Some(true_digits) =
        repeating_digits.strip_prefix('r')
    {
        BigRational::new(
            map_parsebiginterror(bigint_from_possibly_empty_str_radix(
                true_digits,
                literal_own_radix,
            ))?,
            scaling
                * (BigInt::from(literal_own_radix).pow(BigUint::from(repeating_digits.len() - 1))
                    - BigInt::one()),
        )
    } else {
        BigRational::zero()
    };

    /* what follows the `e`, `p` or `xp` is interpreted using the external context */
    let exponent: BigRational = if let Some(true_digits) = exponent.strip_prefix('e') {
        // power of radix
        let exponent =
            map_parsebiginterror(BigInt::from_str_radix(true_digits, external_radix_context))?;
        power(literal_own_radix, exponent)
    } else if let Some(true_digits) = exponent.strip_prefix("xp") {
        let exponent =
            map_parsebiginterror(BigInt::from_str_radix(true_digits, external_radix_context))?;
        power(literal_own_radix, exponent)
    } else if let Some(true_digits) = exponent.strip_prefix('p') {
        // power of 2
        let exponent =
            map_parsebiginterror(BigInt::from_str_radix(true_digits, external_radix_context))?;
        power(2, exponent)
    } else {
        BigRational::one()
    };

    let ans = (integral_part + before_rep_part + repeating_digits_part) * exponent;
    Ok((ans, input.strip_prefix(whole).expect("regex match")))
}
