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

fn main() {
    println!("{}", parse_pointed_literal("0.1r6"));
    println!("{}", parse_pointed_literal("12."));
    println!("{}", parse_pointed_literal("12.1"));
    println!("{}", parse_pointed_literal("12.1r6"));
    println!("{}", parse_pointed_literal(".1r6"));
    println!("{}", parse_pointed_literal(".r3"));
    println!("{}", parse_pointed_literal(".r142857"));
    println!("{}", parse_pointed_literal_radix(".r0313452421", 6));
}

fn bigint_from_possibly_empty_str_radix(str: &str, radix: u32) -> Result<BigInt, ParseBigIntError> {
    BigInt::from_str_radix(if str.is_empty() { "0" } else { str }, radix)
}

fn parse_pointed_literal(input: &str) -> BigRational {
    parse_pointed_literal_radix(input, 10)
}
fn parse_pointed_literal_radix(input: &str, radix: u32) -> BigRational {
    println!("\nparsing `{}` (radix: {} in decimal)", input, radix);

    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?<integral>[0-9]*)\.(?<before_rep>[0-9]*)(?<rep_digits>(r[0-9]*)?)").unwrap()
    });

    let caps = RE.captures(input).unwrap();

    // let whole = caps.get(0).unwrap().as_str();
    let integral = caps.name("integral").unwrap().as_str();
    let before_rep = caps.name("before_rep").unwrap().as_str();
    let repeating_digits = caps.name("rep_digits").unwrap().as_str();

    // println!("whole: {:?}", whole);

    let integral_part: BigRational = BigRational::new(
        bigint_from_possibly_empty_str_radix(integral, radix).unwrap(),
        BigInt::one(),
    );
    // println!("integral_part: {:?}", integral_part);

    let scaling = BigInt::from(radix).pow(BigUint::from(before_rep.len()));

    let before_rep_part: BigRational = BigRational::new(
        bigint_from_possibly_empty_str_radix(before_rep, radix).unwrap(),
        scaling.clone(),
    );
    // println!("before_rep_part: {:?}", before_rep_part);

    let repeating_digits_part: BigRational = if repeating_digits.starts_with('r') {
        BigRational::new(
            bigint_from_possibly_empty_str_radix(
                repeating_digits.strip_prefix('r').unwrap(),
                radix,
            )
            .unwrap(),
            scaling
                * (BigInt::from(radix).pow(BigUint::from(repeating_digits.len() - 1))
                    - BigInt::one()),
        )
    } else {
        BigRational::zero()
    };
    // println!("repeating_digits_part: {:?}", repeating_digits_part);

    integral_part + before_rep_part + repeating_digits_part
}

#[test]
fn test() {}
