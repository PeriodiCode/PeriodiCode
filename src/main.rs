use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::identities::One;
use num_traits::Num;
use once_cell::sync::Lazy;
use parse::Parser;
use regex::Regex;

use crate::print::print_rational_summary;

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

    fn execute_expression(&mut self, parser: &mut Parser, input: &str) {
        println!(
            "\x1b[1;34mPeriodiCode\x1b[00m:\x1b[1;32mDEC{:<2}\x1b[00m> {}",
            self.radix_context, input
        );

        if input.trim_start().starts_with('#') || input.trim().is_empty() {
            /* line-comment; ignore */
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

            if Some(&ratio) != self.previous_value.as_ref() {
                match &self.previous_value {
                    None => panic!("ASSERTION FAILED: \nleft: (null)\nright: {}", ratio),
                    Some(previous_value) => panic!(
                        "ASSERTION FAILED: \nleft: {}\nright: {}",
                        previous_value, ratio
                    ),
                }
            }
        } else {
            parser.set_radix_context(self.radix_context);
            parser.set_previous_value(self.previous_value.clone());
            let (ans, remaining) = parser.parse_expression(input).unwrap();
            print_rational_summary(&ans, self.radix_context);
            self.previous_value = Some(ans);
        }
    }

    fn execute_lines(&mut self, parser: &mut Parser, input: &str) {
        for line in input.lines() {
            self.execute_expression(parser, line);
        }
    }
}

fn main() {
    let mut parser = Parser::new();
    let mut ctx = Interpreter::new();
    ctx.execute_lines(&mut parser, include_str!("../example.periodicode"));
}

#[cfg(test)]
mod test;

mod print;

mod parse;
