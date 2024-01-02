#![warn(clippy::pedantic)]

use std::ops::ControlFlow;

use num_rational::BigRational;
use num_traits::Zero;
use parse::Parser;

use crate::print::rational_print_summary;

mod numerical_util;

struct Interpreter {
    previous_value: BigRational,
    radix_context: u32,
}

/// `f` is only run when the input has no trailing semicolon
pub fn handle_empty_or_semicolon<T>(remaining: &str, f: T) -> Option<ControlFlow<(), &str>>
where
    T: Fn(),
{
    if remaining.starts_with('#') || remaining.is_empty() {
        /* line-comment; ignore */
        f();
        return Some(ControlFlow::Break(()));
    } else if let Some(buf) = remaining.strip_prefix(';') {
        let remaining = buf.trim_start();
        if remaining.is_empty() || remaining.starts_with('#') {
            // The line ends with a semicolon; don't print anything
            return Some(ControlFlow::Break(()));
        }
        return Some(ControlFlow::Continue(remaining));
    }
    None
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

        match handle_empty_or_semicolon(input.trim_start(), || ()) {
            Some(ControlFlow::Break(())) => return,
            Some(ControlFlow::Continue(s)) => input = s.to_owned(),
            _ => {}
        }

        loop {
            let mut parser = Parser::new(self.radix_context, self.previous_value.clone(), &input);

            self.previous_value = parser.parse_expression().unwrap();
            self.radix_context = parser.get_radix_context();
            let remaining = parser.get_buf().trim_start();

            match handle_empty_or_semicolon(remaining, || {
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

mod parse;
