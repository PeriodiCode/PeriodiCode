#![warn(clippy::pedantic)]

use std::ops::ControlFlow;

use num_rational::BigRational;
use parse::Parser;

use crate::print::rational_print_summary;

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

    // `f` is only run when the input has no trailing semicolon
    fn handle_empty_or_semicolon<T>(remaining: &str, f: T) -> Option<ControlFlow<(), String>>
    where
        T: Fn(),
    {
        if remaining.starts_with('#') || remaining.is_empty() {
            /* line-comment; ignore */
            f();
            return Some(ControlFlow::Break(()));
        } else if remaining.starts_with(';') {
            let remaining = remaining.strip_prefix(';').unwrap().trim_start();
            if remaining.is_empty() || remaining.starts_with('#') {
                // The line ends with a semicolon; don't print anything
                return Some(ControlFlow::Break(()));
            }
            return Some(ControlFlow::Continue(remaining.to_owned()));
        }
        None
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

        match Self::handle_empty_or_semicolon(input.trim_start(), || ()) {
            Some(ControlFlow::Break(())) => return,
            Some(ControlFlow::Continue(s)) => input = s,
            _ => {}
        }

        let mut ans;
        loop {
            let mut parser = Parser::new(&input);
            parser.set_radix_context(self.radix_context);
            parser.set_previous_value(self.previous_value.clone());

            ans = parser.parse_expression().unwrap();
            self.radix_context = parser.get_radix_context();
            let remaining = parser.get_buf().trim_start();

            // Set the result of the final expression to `$_`
            self.previous_value = Some(ans);

            match Self::handle_empty_or_semicolon(remaining, || {
                rational_print_summary(self.previous_value.as_ref().unwrap(), self.radix_context);
            }) {
                Some(ControlFlow::Break(())) => return,
                Some(ControlFlow::Continue(s)) => input = s,
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
