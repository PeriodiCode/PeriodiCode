use num_bigint::BigInt;
use num_bigint::BigUint;
use num_bigint::ParseBigIntError;
use num_rational::BigRational;
use num_traits::identities::One;
use num_traits::pow::Pow;
use num_traits::Num;
use num_traits::Zero;
use numerical_util::floor_as_bigint;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::numerical_util::power;

mod numerical_util;

struct Interpreter {
    previous_value: Option<BigRational>,
    radix_context: u32,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            previous_value: None,
            radix_context: 10,
        }
    }

    fn execute_expression(&mut self, input: &str) {
        if input.trim_start().starts_with('#') || input.trim().is_empty() {
            /* line-comment; ignore */
            println!("PeriodiCode:DEC{:<2}$ {}", self.radix_context, input);
        } else if input.starts_with("@assert_eq($_, ") {
            let second_arg = input
                .strip_prefix("@assert_eq($_, ")
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            static RE_MAYBE_FRACTION: Lazy<Regex> = Lazy::new(|| {
                Regex::new(r"(?<numerator>[0-9a-oA-O]*)(?<denominator>(/[0-9a-oA-O]*)?)").unwrap()
            });
            let caps = RE_MAYBE_FRACTION.captures(second_arg).unwrap();
            let numerator = BigInt::from_str_radix(
                caps.name("numerator").unwrap().as_str(),
                self.radix_context,
            )
            .unwrap();
            let denominator = match caps.name("denominator").unwrap().as_str().strip_prefix('/') {
                None => BigInt::one(),
                Some(u) => BigInt::from_str_radix(u, self.radix_context).unwrap(),
            };

            let ratio = BigRational::new(numerator, denominator);

            println!("PeriodiCode:DEC{:<2}$ {}", self.radix_context, input);

            if Some(&ratio) != self.previous_value.as_ref() {
                match &self.previous_value {
                    None => panic!("ASSERTION FAILED: \nleft: (null)\nright: {}", ratio),
                    Some(previous_value) => panic!(
                        "ASSERTION FAILED: \nleft: {}\nright: {}",
                        previous_value, ratio
                    ),
                }
            }
            println!("ok\n");
        } else {
            let ans = parse_numeric_literal_with_radix_context(input, self.radix_context);
            self.previous_value = Some(ans);
        }
    }

    fn execute_lines(&mut self, input: &str) {
        for line in input.lines() {
            self.execute_expression(line);
        }
    }
}

fn main() {
    let mut ctx = Interpreter::new();
    ctx.execute_lines(include_str!("../example.periodicode"));
}

#[cfg(test)]
mod test;

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

fn parse_numeric_literal_with_radix_context(input: &str, radix_context: u32) -> BigRational {
    let original_input = input;
    let (input, literal_own_radix) = strip_radix_prefix(input);

    println!("PeriodiCode:DEC{:<2}$ {}", radix_context, original_input);
    parse_numeric_literal_with_both_contexts(input, radix_context, literal_own_radix).unwrap()
}

fn parse_numeric_literal_with_both_contexts(
    input: &str,
    external_radix_context: u32,
    literal_own_radix: Option<u32>,
) -> Result<BigRational, &'static str> {
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
    print_rational_summary(&ans, external_radix_context);

    Ok(ans)
}

fn print_rational_summary(ans: &BigRational, external_radix_context: u32) {
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

    print_continued_fraction_radix(ans, external_radix_context);

    if external_radix_context != 10 {
        print!(" (DEC");
        print_continued_fraction_radix(ans, 10);
        print!(")")
    }

    println!();

    print!("digt: ");

    print_digit_expansion_radix(ans, external_radix_context);

    println!("\n");
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

fn print_digit_expansion_radix(ans: &BigRational, external_radix_context: u32) {
    if ans < &BigRational::zero() {
        print!("-");
        print_digit_expansion_radix(&-ans, external_radix_context);
        return;
    }

    print!(
        "{}",
        floor_as_bigint(ans).to_str_radix(external_radix_context)
    );

    let mut f = ans - ans.floor();
    if f.is_zero() {
        return;
    }

    print!(".");

    let mut f_list = vec![];
    let mut digits = vec![];

    loop {
        f_list.push(f.clone());

        f *= BigInt::from(external_radix_context);

        let digit = floor_as_bigint(&f).to_str_radix(external_radix_context);
        digits.push(digit.clone());

        f = f.clone() - f.floor();
        if f.is_zero() {
            print!("{}", digits.join(""));
            return;
        }

        if f_list.contains(&f) {
            let pos = f_list.iter().position(|k| k == &f).unwrap();
            print!("{}r{}", digits[0..pos].join(""), digits[pos..].join(""));
            return;
        }
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
                let n = floor_as_bigint(r);
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
