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

    fn execute_line(&mut self, input: &str) {
        let mut input = input.to_owned();
        println!(
            "\x1b[1;34mPeriodiCode\x1b[00m:\x1b[1;32mDEC{:<2}\x1b[00m> {}",
            self.radix_context, input
        );

        if input.trim_start().starts_with('#') || input.trim_start().is_empty() {
            /* line-comment; ignore */
            return;
        } else if input.trim_start().starts_with(';') {
            let remaining = input.trim_start().strip_prefix(';').unwrap().trim_start();
            if remaining.is_empty() || remaining.starts_with('#') {
                // The line ends with a semicolon; don't print anything
                return;
            } else {
                input = remaining.to_owned();
            }
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

            if remaining.is_empty() || remaining.starts_with('#') {
                print_rational_summary(self.previous_value.as_ref().unwrap(), self.radix_context);
                return;
            } else if remaining.starts_with(';') {
                let remaining = remaining.strip_prefix(';').unwrap().trim_start();
                if remaining.is_empty() || remaining.starts_with('#') {
                    // The line ends with a semicolon; don't print anything
                    return;
                } else {
                    input = remaining.to_owned();
                }
            } else {
                panic!("cannot parse the remaining `{}`", remaining);
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
