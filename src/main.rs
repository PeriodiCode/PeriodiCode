#![warn(clippy::pedantic)]

use num_rational::BigRational;
use num_traits::Zero;
use parse::Parser;

use crate::print::rational_print_summary;

mod numerical_util;

struct Interpreter {
    previous_value: BigRational,
    radix_context: u32,
}

enum Judgement<T> {
    EndOfLineEncountered,
    ExpressionTerminatedWithSemicolon(T),
    NoConsumption,
}

/// `f` is only run when the input has no trailing semicolon
/// multiple semicolons are collapsed
///
/// `Judgement::EndOfLineEncountered` is returned when the line is terminated
/// `Judgement::ExpressionTerminatedWithSemicolon(new_buf)` is returned when the preceding expression is terminated by a semicolon but the buffer still contains content
/// `Judgement::NoConsumption` is returned when the buffer neither is empty nor begins with a semicolon
fn judge_termination_or_semicolons<T>(buf: &str, f: T) -> Judgement<&str>
where
    T: Fn(),
{
    if buf.starts_with('#') || buf.is_empty() {
        // No trailing semicolon
        f();
        Judgement::EndOfLineEncountered
    } else if let Some(buf) = buf.strip_prefix(';') {
        // collapse multiple semicolons
        let mut remaining = buf.trim_start();
        while let Some(buf) = remaining.strip_prefix(';') {
            remaining = buf.trim_start();
        }

        if remaining.is_empty() || remaining.starts_with('#') {
            Judgement::EndOfLineEncountered
        } else {
            Judgement::ExpressionTerminatedWithSemicolon(remaining)
        }
    } else {
        Judgement::NoConsumption
    }
}

impl Interpreter {
    fn new(previous_value: BigRational, radix_context: u32) -> Self {
        Self {
            previous_value,
            radix_context,
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

        match judge_termination_or_semicolons(input.trim_start(), || ()) {
            Judgement::EndOfLineEncountered => return,
            Judgement::ExpressionTerminatedWithSemicolon(s) => input = s.to_owned(),
            Judgement::NoConsumption => {}
        }

        loop {
            let mut p = Parser::new(self.radix_context, self.previous_value.clone(), &input);

            self.previous_value = p.parse_expression().unwrap();
            self.radix_context = p.get_radix_context();
            let remaining = p.get_buf().trim_start();

            match judge_termination_or_semicolons(remaining, || {
                rational_print_summary(&self.previous_value, self.radix_context);
            }) {
                Judgement::EndOfLineEncountered => return,
                Judgement::ExpressionTerminatedWithSemicolon(s) => input = s.to_owned(),
                Judgement::NoConsumption => panic!("cannot parse the remaining `{remaining}`"),
            }
        }
    }

    fn execute_lines(&mut self, input: &str) -> (BigRational, u32) {
        for line in input.lines() {
            self.execute_line(line);
        }
        (self.previous_value.clone(), self.radix_context)
    }
}

fn main() {
    let mut ctx = Interpreter::new(BigRational::zero(), 10);
    ctx.execute_lines(include_str!("../long_tutorial.periodicode"));
}

#[cfg(test)]
mod test;

mod print;

mod parse;
