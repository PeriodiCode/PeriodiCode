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
/// multiple semicolons are collapsed
pub fn handle_empty_or_semicolons<T>(remaining: &str, f: T) -> Option<ControlFlow<(), &str>>
where
    T: Fn(),
{
    if remaining.starts_with('#') || remaining.is_empty() {
        f();
        Some(ControlFlow::Break(()))
    } else if let Some(buf) = remaining.strip_prefix(';') {
        let mut remaining = buf.trim_start();

        while let Some(buf) = remaining.strip_prefix(';') {
            remaining = buf.trim_start();
        }

        if remaining.is_empty() || remaining.starts_with('#') {
            // The line ends with a semicolon; don't print anything
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

    /// The grammar that I want to express is as follows:
    /// ```bnf
    /// <line> ::= <statement>* <expression>? <line-comment>?
    /// <statement> ::= <expression>? ";" | ("@" <radix-identifier>)? "{" <statement>* "}"
    /// <expression> ::= <bare-expression> | ("@" <radix-identifier>)? "{" <statement>* <expression> "}"
    /// ```
    ///
    /// Make it into a context-free grammar and we have
    /// ```ebnf
    /// L = L2 | L2 "#comment"
    /// L2 = Ss | Ss E
    /// Ss = ε | S Ss
    /// S = ";" | E ";" | O Ss "}"
    /// E = "bare-expr" | O Ss E "}"
    /// O = "{" | "@radix{"
    /// ```
    /// 
    /// Eliminating `S` gives
    /// ```ebnf
    /// L = L2 | L2 "#comment"
    /// L2 = Ss | Ss E
    /// Ss = ε | ";" Ss | E ";" Ss | O Ss "}" Ss
    /// E = "bare-expr" | O Ss E "}"
    /// O = "{" | "@radix{"
    /// ```
    /// 
    /// This, in turn, can be made into a deterministic grammar:
    /// ```ebnf
    /// L2 = Ss | Ss E
    /// Ss = ε | ";" Ss | "bare-expr" ";" Ss | O Ss R
    /// E = "bare-expr" | O Ss E "}"
    /// R = "bare-expr" "}" ";" 
    ///   | O Ss E "}" "}" ";" 
    ///   | "}"
    /// ```
    /// 
    /// where R's semantics is "closing bracket that follows statements and creates statements"
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
            let mut parser = Parser::new(self.radix_context, self.previous_value.clone(), &input);

            self.previous_value = parser.parse_expression().unwrap();
            self.radix_context = parser.get_radix_context();
            let remaining = parser.get_buf().trim_start();

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

mod parse;
