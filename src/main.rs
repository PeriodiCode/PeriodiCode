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
    assert_eq!(parse_numeric_literal("0.1r6").to_string(), "1/6");
    assert_eq!(parse_numeric_literal("12.").to_string(), "12");
    assert_eq!(parse_numeric_literal("12").to_string(), "12");
    assert_eq!(parse_numeric_literal("12.1").to_string(), "121/10");
    assert_eq!(parse_numeric_literal("12.1r6").to_string(), "73/6");
    assert_eq!(parse_numeric_literal(".1r6").to_string(), "1/6");
    assert_eq!(parse_numeric_literal(".r3").to_string(), "1/3");
    assert_eq!(parse_numeric_literal(".r142857").to_string(), "1/7");
    assert_eq!(
        parse_numeric_literal_with_radix_context(".r0313452421", 6).to_string(),
        "1/11"
    );

    assert_eq!(parse_numeric_literal("0v100").to_string(), "400");
    assert_eq!(parse_numeric_literal("0x100").to_string(), "256");
    assert_eq!(parse_numeric_literal("0z100").to_string(), "144");
    assert_eq!(parse_numeric_literal("0d100").to_string(), "100");
    assert_eq!(parse_numeric_literal("0o100").to_string(), "64");
    assert_eq!(parse_numeric_literal("0s100").to_string(), "36");
    assert_eq!(parse_numeric_literal("0quin100").to_string(), "25");
    assert_eq!(parse_numeric_literal("0quat100").to_string(), "16");
    assert_eq!(parse_numeric_literal("0t100").to_string(), "9");
    assert_eq!(parse_numeric_literal("0b100").to_string(), "4");

    assert_eq!(parse_numeric_literal("0v100.").to_string(), "400");
    assert_eq!(parse_numeric_literal("0x100.").to_string(), "256");
    assert_eq!(parse_numeric_literal("0z100.").to_string(), "144");
    assert_eq!(parse_numeric_literal("0d100.").to_string(), "100");
    assert_eq!(parse_numeric_literal("0o100.").to_string(), "64");
    assert_eq!(parse_numeric_literal("0s100.").to_string(), "36");
    assert_eq!(parse_numeric_literal("0quin100.").to_string(), "25");
    assert_eq!(parse_numeric_literal("0quat100.").to_string(), "16");
    assert_eq!(parse_numeric_literal("0t100.").to_string(), "9");
    assert_eq!(parse_numeric_literal("0b100.").to_string(), "4");

    assert_eq!(parse_numeric_literal("0x1.p10").to_string(), "1024");
    assert_eq!(parse_numeric_literal("0x1.p-10").to_string(), "1/1024");
    assert_eq!(parse_numeric_literal("0b11.p-10").to_string(), "3/1024");
    assert_eq!(parse_numeric_literal("0x11.p-10").to_string(), "17/1024");
    assert_eq!(parse_numeric_literal("0d11.p-10").to_string(), "11/1024");
    assert_eq!(
        parse_numeric_literal_with_radix_context("0x1.p10", 6).to_string(),
        "64"
    );

    assert_eq!(parse_numeric_literal("0s.r0313452421").to_string(), "1/11");

    assert_eq!(parse_numeric_literal("0.1r6e1").to_string(), "5/3");
    assert_eq!(parse_numeric_literal("0.1r6xp1").to_string(), "5/3");
    assert_eq!(
        parse_numeric_literal_with_radix_context("1.0p10", 10).to_string(),
        "1024"
    );
}

fn bigint_from_possibly_empty_str_radix(str: &str, radix: u32) -> Result<BigInt, ParseBigIntError> {
    BigInt::from_str_radix(if str.is_empty() { "0" } else { str }, radix)
}

fn parse_numeric_literal(input: &str) -> BigRational {
    parse_numeric_literal_with_radix_context(input, 10)
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
    } else if input.starts_with("0quin") {
        (input.strip_prefix("0quin").unwrap(), Some(5))
    } else if input.starts_with("0quat") {
        (input.strip_prefix("0quat").unwrap(), Some(4))
    } else if input.starts_with("0t") {
        (input.strip_prefix("0t").unwrap(), Some(3))
    } else if input.starts_with("0b") {
        (input.strip_prefix("0b").unwrap(), Some(2))
    } else {
        (input, None)
    }
}

fn parse_numeric_literal_with_radix_context(input: &str, radix_context: u32) -> BigRational {
    let original_input = input;
    let (input, literal_own_radix) = strip_radix_prefix(input);

    println!(
        "PeriodiCode:DEC{:<2}$ {}",
        radix_context, original_input
    );
    parse_numeric_literal_with_both_contexts(input, radix_context, literal_own_radix)
}

fn parse_numeric_literal_with_both_contexts(
    input: &str,
    external_radix_context: u32,
    literal_own_radix: Option<u32>,
) -> BigRational {
    /**
     * exponent:
     * `e` or `xp`: multiplies the number by the power of the literal's own radix. `e` can only be used if the base is less than fifteen
     * `p`: multiplies the number by power of two.
     *
     *  The digits following `e`, `xp` or `p` is interpreted with the power of radix (taken from the external context).
     *  - Hence, in decimal context, 0x1.0p10 == 1024.0
     */

    static RE_ALLOWING_E: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?<integral>[0-9a-dA-D]*)(?<dot>\.(?<before_rep>[0-9a-dA-D]*)(?<rep_digits>(r[0-9a-dA-D]*)?))?(?<exponent>((e|xp|p)(\+|-)?[0-9a-dA-D]+)?)").unwrap()
    });

    static RE_FORBIDDING_E: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?<integral>[0-9a-oA-O]*)(?<dot>\.(?<before_rep>[0-9a-oA-O]*)(?<rep_digits>(r[0-9a-oA-O]*)?))?(?<exponent>((xp|p)(\+|-)?[0-9a-oA-O]+)?)").unwrap()
    });

    let literal_own_radix = literal_own_radix.unwrap_or(external_radix_context);

    let caps = if literal_own_radix < 15 {
        RE_ALLOWING_E.captures(input).unwrap()
    } else {
        RE_FORBIDDING_E.captures(input).unwrap()
    };

    // let whole = caps.get(0).unwrap().as_str();
    let integral = caps.name("integral").unwrap().as_str();
    let (before_rep, repeating_digits) = match caps.name("dot") {
        Some(_) => (
            caps.name("before_rep").unwrap().as_str(),
            caps.name("rep_digits").unwrap().as_str(),
        ),
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
    let numer = ans.numer();
    let denom = ans.denom();

    print!("frac: ");

    /* print fractional */
    if denom == &BigInt::one() {
        print!("{}", numer.to_str_radix(external_radix_context));
    } else {
        print!(
            "{}/{}",
            numer.to_str_radix(external_radix_context),
            denom.to_str_radix(external_radix_context)
        );
    }
    if external_radix_context != 10 {
        print!(" (DEC{})", ans);
    }

    println!();

    print!("cont: ");

    print_continued_fraction_radix(&ans, external_radix_context);

    if external_radix_context != 10 {
        print!(" (DEC");
        print_continued_fraction_radix(&ans, 10);
        print!(")")
    }

    println!("\n");

    ans
}

fn print_continued_fraction_radix(ans: &BigRational, external_radix_context: u32) {
    let mut cont_frac_iter = FiniteContinuedFractionIter::new(ans);
    let initial = cont_frac_iter.next().unwrap();
    let remaining: Vec<BigInt> = cont_frac_iter.collect();
    if remaining.is_empty() {
        print!("[{}]", initial.to_str_radix(external_radix_context))
    } else {
        print!(
            "[{}; {}]",
            initial.to_str_radix(external_radix_context),
            remaining
                .into_iter()
                .map(|n| n.to_str_radix(external_radix_context))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

fn power(radix: u32, exponent: BigInt) -> BigRational {
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

fn floor(s: &BigRational) -> BigInt {
    if *s < Zero::zero() {
        let one: BigInt = One::one();
        (s.numer() - s.denom().clone() + one) / s.denom().clone()
    } else {
        s.numer().clone() / s.denom().clone()
    }
}

enum FiniteContinuedFractionIter {
    Ratio(BigRational),
    Infinity,
}

impl FiniteContinuedFractionIter {
    fn new(s: &BigRational) -> Self {
        Self::Ratio(s.clone())
    }
}

impl Iterator for FiniteContinuedFractionIter {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FiniteContinuedFractionIter::Ratio(r) => {
                let n = floor(r);
                let f = &*r - r.floor();
                if f == BigRational::zero() {
                    *self = Self::Infinity;
                } else {
                    *self = Self::Ratio(f.recip());
                }
                Some(n)
            }
            FiniteContinuedFractionIter::Infinity => None,
        }
    }
}
