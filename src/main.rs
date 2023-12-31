#![warn(clippy::pedantic)]

use num_rational::BigRational;
use num_traits::Zero;
use parse::Parser;

use crate::print::rational_print_summary;

mod numerical_util;

struct Interpreter {
    previous_value: BigRational,
    radix_context: u32,
    stack_trace: Vec<String>,
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
    fn new(previous_value: BigRational, radix_context: u32, stack_trace: Vec<String>) -> Self {
        Self {
            previous_value,
            radix_context,
            stack_trace,
        }
    }

    fn execute_line(&mut self, input: &str) -> Result<(), String> {
        let stack_trace_str = self.stack_trace.iter().fold(String::new(), |mut a, b| {
            a += "\x1b[0;34m"; /* normal blue */
            a += b;
            a += "\x1b[00m:";
            a
        });

        let mut input = input.to_owned();
        println!(
            "\x1b[1;34mPeriodiCode\x1b[00m:{stack_trace_str}\x1b[{};32mbase-{:<2}\x1b[00m> {}",
            if self.radix_context == 10 {
                "0" /* normal */
            } else {
                "1;4" /* bold, underline */
            },
            self.radix_context,
            input
        );

        match judge_termination_or_semicolons(input.trim_start(), || ()) {
            Judgement::EndOfLineEncountered => return Ok(()),
            Judgement::ExpressionTerminatedWithSemicolon(s) => input = s.to_owned(),
            Judgement::NoConsumption => {}
        }

        loop {
            let mut p = Parser::new(
                self.radix_context,
                self.previous_value.clone(),
                self.stack_trace.clone(),
                &input,
            );

            self.previous_value = p.parse_expression()?;
            self.radix_context = p.get_radix_context();
            let remaining = p.get_buf().trim_start();

            match judge_termination_or_semicolons(remaining, || {
                rational_print_summary(&self.previous_value, self.radix_context);
            }) {
                Judgement::EndOfLineEncountered => return Ok(()),
                Judgement::ExpressionTerminatedWithSemicolon(s) => input = s.to_owned(),
                Judgement::NoConsumption => {
                    return Err(format!("cannot parse the remaining `{remaining}`"))
                }
            }
        }
    }

    fn execute_lines(&mut self, input: &str) -> Result<(BigRational, u32), String> {
        for line in input.lines() {
            self.execute_line(line)?;
        }
        Ok((self.previous_value.clone(), self.radix_context))
    }
}

fn main() -> Result<(), String> {
    let mut ctx = Interpreter::new(BigRational::zero(), 10, vec![]);
    ctx.execute_lines(
        r#"@load { "summary.periodicode" };
$_"#,
    )?;
    Ok(())
}

#[cfg(test)]
mod test;

mod print;

mod parse;
