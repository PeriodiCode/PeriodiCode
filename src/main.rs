#![warn(clippy::pedantic)]

use std::ops::ControlFlow;

use num_rational::BigRational;
use num_traits::Zero;
use parse_bare_expression::BareExpressionParser;

use crate::print::rational_print_summary;

mod numerical_util;

struct Interpreter {
    previous_value: BigRational,
    radix_context: u32,
}

/// `f` is only run when the input has no trailing semicolon
/// multiple semicolons are collapsed
/// 
/// `Some(Break(()))` is returned when the line is terminated
/// `Some(Continue(remaining))` is returned when the preceding expression is terminated by a semicolon but the buffer still contains content
/// `None` is returned when the buffer neither is empty nor begins with a semicolon
pub fn handle_empty_or_semicolons<T>(buf: &str, f: T) -> Option<ControlFlow<(), &str>>
where
    T: Fn(),
{
    if buf.starts_with('#') || buf.is_empty() {
        // No trailing semicolon
        f();
        Some(ControlFlow::Break(()))
    } else if let Some(buf) = buf.strip_prefix(';') {

        // collapse multiple semicolons
        let mut remaining = buf.trim_start();
        while let Some(buf) = remaining.strip_prefix(';') {
            remaining = buf.trim_start();
        }

        if remaining.is_empty() || remaining.starts_with('#') {
            Some(ControlFlow::Break(()))
        } else {
            Some(ControlFlow::Continue(remaining))
        }
    } else {
        None
    }
}

impl Interpreter {
    fn new() -> Self {
        Self {
            previous_value: BigRational::zero(),
            radix_context: 10,
        }
    }

    fn execute_line(&mut self, input: &str) {
        let mut input = input.to_owned();
        println!(
            "\x1b[1;34mPeriodiCode\x1b[00m:\x1b[{};32mbase-{:<2}\x1b[00m> {}",
            if self.radix_context == 10 {
                "0" /* normal */
            } else {
                "1;4" /* bold, underline */
            },
            self.radix_context,
            input
        );

        match handle_empty_or_semicolons(input.trim_start(), || ()) {
            Some(ControlFlow::Break(())) => return,
            Some(ControlFlow::Continue(s)) => input = s.to_owned(),
            _ => {}
        }

        loop {
            let mut p = BareExpressionParser::new(self.radix_context, self.previous_value.clone(), &input);

            self.previous_value = p.parse_bare_expression().unwrap();
            self.radix_context = p.get_radix_context();
            let remaining = p.get_buf().trim_start();

            match handle_empty_or_semicolons(remaining, || {
                rational_print_summary(&self.previous_value, self.radix_context);
            }) {
                Some(ControlFlow::Break(())) => return,
                Some(ControlFlow::Continue(s)) => input = s.to_owned(),
                None => panic!("cannot parse the remaining `{remaining}`"),
            }
        }
    }

    fn execute_lines(&mut self, input: &str) {
        for line in input.lines() {
            self.execute_line(line);
        }
    }
}

fn main() {
    let mut ctx = Interpreter::new();
    ctx.execute_lines(include_str!("../example.periodicode"));
}

#[cfg(test)]
mod test;

mod print;

mod parse_bare_expression;
