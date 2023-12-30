use num_rational::BigRational;
use parse::Parser;

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
