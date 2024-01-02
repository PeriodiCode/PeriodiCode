use std::io::Read;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{judge_termination_or_semicolons, Interpreter, Judgement};

pub mod numeric_literal;

type Value = BigRational;

pub struct Parser<'a> {
    radix_context: u32,
    previous_value: Value,
    stack_trace: Vec<String>,
    buf: &'a str,
}

struct Identifier(String);

impl<'b> Parser<'b> {
    pub fn get_buf(&self) -> &str {
        self.buf
    }
    pub fn new(
        radix_context: u32,
        previous_value: Value,
        stack_trace: Vec<String>,
        buf: &'b str,
    ) -> Self {
        assert!(
            radix_context <= 25,
            "radix greater than 25 is not supported"
        );

        Self {
            radix_context,
            previous_value,
            stack_trace,
            buf,
        }
    }

    pub fn get_radix_context(&mut self) -> u32 {
        self.radix_context
    }

    pub fn parse_expression(&mut self) -> Result<Value, &'static str> {
        self.trim_start();
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> Result<Value, &'static str> {
        self.trim_start();
        let mut val = self.parse_multiplicative_expression()?;
        loop {
            if let Some(stripped) = self.buf.trim_start().strip_prefix('+') {
                self.buf = stripped;
                let val2 = self.parse_multiplicative_expression()?;
                val += val2;
            } else if let Some(stripped) = self.buf.trim_start().strip_prefix('-') {
                self.buf = stripped;
                let val2 = self.parse_multiplicative_expression()?;
                val -= val2;
            } else {
                break;
            }
        }

        Ok(val)
    }

    fn parse_multiplicative_expression(&mut self) -> Result<Value, &'static str> {
        self.trim_start();
        let mut val = self.parse_unary_expression()?;
        loop {
            if let Some(stripped) = self.buf.trim_start().strip_prefix('*') {
                self.buf = stripped;
                let val2 = self.parse_unary_expression()?;
                val *= val2;
            } else if let Some(stripped) = self.buf.trim_start().strip_prefix('/') {
                self.buf = stripped;
                let val2 = self.parse_unary_expression()?;
                val /= val2;
            } else {
                break;
            }
        }
        Ok(val)
    }

    fn parse_unary_expression(&mut self) -> Result<Value, &'static str> {
        let buf = self.buf.trim_start();
        if let Some(buf) = buf.strip_prefix('+') {
            self.buf = buf;
            let value = self.parse_unary_expression()?;
            Ok(value)
        } else if let Some(buf) = buf.strip_prefix('-') {
            self.buf = buf;
            let value = self.parse_unary_expression()?;
            Ok(-value)
        } else {
            self.parse_funccall_or_decorated_block()
        }
    }

    fn consume_char_or_err(&mut self, c: char, msg: &'static str) -> Result<(), &'static str> {
        self.trim_start();
        if let Some(buf_) = self.buf.strip_prefix(c) {
            self.buf = buf_.trim_start();
            Ok(())
        } else {
            Err(msg)
        }
    }

    fn trim_start(&mut self) {
        self.buf = self.buf.trim_start();
    }

    fn parse_string_literal(&mut self) -> Result<String, &'static str> {
        self.trim_start();
        let mut s = String::new();
        if let Some(buf_) = self.buf.strip_prefix('"') {
            let mut char_indices = buf_.char_indices();
            let closing_quote_index = loop {
                match char_indices.next() {
                    Some((i, '"')) => {
                        break i;
                    }
                    Some((_, '\\')) => match char_indices.next() {
                        Some((_, 'n')) => {
                            s.push('\n');
                        }
                        Some((_, '"')) => {
                            s.push('\"');
                        }
                        Some((_, '\'')) => {
                            s.push('\'');
                        }
                        Some((_, '\\')) => {
                            s.push('\\');
                        }
                        None => return Err("Unterminated escape sequence inside a string literal"),
                        Some((_, _)) => {
                            return Err("Unsupported escape sequence inside a string literal")
                        }
                    },
                    Some((_, c)) => s.push(c),
                    None => return Err("Unterminated string literal"),
                }
            };

            self.buf = &buf_[(closing_quote_index + 1)..];

            Ok(s)
        } else {
            Err("Not a string literal")
        }
    }

    fn parse_string_literal_and_load_single_file_dirty(&mut self) -> Result<Value, &'static str> {
        let filename = self.parse_string_literal()?;
        println!("\x1b[2;34m##### Start of {filename}: \x1b[00m"); // faint blue

        let mut f = std::fs::File::open(filename.clone()).map_err(|_| "File not found")?;
        let mut content = String::new();
        f.read_to_string(&mut content)
            .map_err(|_| "something went wrong reading the file")?;

        // boot up the new interpreter, inheriting the environment
        let mut new_stack_trace = self.stack_trace.clone();
        new_stack_trace.push(filename.strip_suffix(".periodicode").unwrap_or(&filename).to_owned());

        let mut new_ctx = Interpreter::new(
            self.previous_value.clone(),
            self.radix_context,
            new_stack_trace,
        );
        let (value, radix_context) = new_ctx.execute_lines(&content);

        self.previous_value = value.clone();

        // write back the radix context
        self.radix_context = radix_context;

        println!("\x1b[2;34m##### End of {filename}\x1b[00m"); // faint blue

        Ok(value)
    }

    fn parse_string_literal_and_load_single_file_clean(&mut self) -> Result<Value, &'static str> {
        let filename = self.parse_string_literal()?;
        println!("\x1b[2;34m##### Entering {filename}: \x1b[00m"); // faint blue

        let mut f = std::fs::File::open(filename.clone()).map_err(|_| "File not found")?;
        let mut content = String::new();
        f.read_to_string(&mut content)
            .map_err(|_| "something went wrong reading the file")?;

        // Boot up the interpreter with the default environment
        // but keep track of the stack trace
        let mut new_stack_trace = self.stack_trace.clone();
        new_stack_trace.push(filename.strip_suffix(".periodicode").unwrap_or(&filename).to_owned());
        let mut new_ctx = Interpreter::new(BigRational::zero(), 10, new_stack_trace);

        // Do not write back the radix context
        let (value, _) = new_ctx.execute_lines(&content);

        self.previous_value = value.clone();

        println!("\x1b[2;34m##### Exiting {filename}\x1b[00m"); // faint blue

        Ok(value)
    }

    fn parse_funccall_or_decorated_block(&mut self) -> Result<Value, &'static str> {
        self.trim_start();
        if let Some(buf_) = self.buf.strip_prefix('@') {
            self.buf = buf_.trim_start();
            let ident = self.parse_identifier()?;
            if let Some(new_radix_content) = ident.to_radix() {
                let stashed_radix_content = self.radix_context;
                self.radix_context = new_radix_content;
                let val = self.parse_block_expression(Self::parse_expression)?;
                self.radix_context = stashed_radix_content;
                Ok(val)
            } else if ident.0 == "load_dirty" {
                self.parse_block_expression(Self::parse_string_literal_and_load_single_file_dirty)
            } else if ident.0 == "load" {
                self.parse_block_expression(Self::parse_string_literal_and_load_single_file_clean)
            } else if ident.0 == "assert_eq" {
                self.consume_char_or_err(
                    '(',
                    "No parenthesis after the built-in function `assert_eq`",
                )?;

                let first_arg = self.parse_expression()?;
                self.consume_char_or_err(
                    ',',
                    "The built-in function `assert_eq` expects exactly two arguments",
                )?;
                let second_arg = self.parse_expression()?;
                self.consume_char_or_err(
                    ')',
                    "The built-in function `assert_eq` expects exactly two arguments",
                )?;
                self.trim_start();
                if first_arg == second_arg {
                    Ok(first_arg) // @assert_eq(7*6, 42) returns 42
                } else {
                    panic!("ASSERTION FAILED: \nleft: {first_arg}\nright: {second_arg}",)
                }
            } else if ident.0 == "set_radix" {
                self.consume_char_or_err(
                    '(',
                    "No parenthesis after the built-in function `set_radix`",
                )?;
                self.consume_char_or_err(
                    '@',
                    "No radix argument found in the built-in function `set_radix`",
                )?;
                self.trim_start();
                let radix_ident = self.parse_identifier()?;

                let radix: u32 = radix_ident
                    .to_radix()
                    .ok_or("Unrecognizable radix name found")?;

                self.radix_context = radix;

                self.consume_char_or_err(
                    ')',
                    "The built-in function `set_radix` expects exactly one argument",
                )?;

                Ok(BigRational::new(BigInt::from(radix), BigInt::one()))
            } else {
                panic!("UNSUPPORTED FUNCTION: `@{}`", ident.0)
            }
        } else {
            self.parse_primary_expression()
        }
    }

    fn parse_block_expression<T>(&mut self, f: T) -> Result<Value, &'static str>
    where
        T: Fn(&mut Self) -> Result<Value, &'static str>,
    {
        self.trim_start();
        self.consume_char_or_err('{', "Expected the start of a block")?;

        // Inside the block, it's allowed to have as many preceding or trailing semicolons,
        // but it must contain at least one expression
        loop {
            match judge_termination_or_semicolons(self.buf, || ()) {
                Judgement::EndOfLineEncountered => {
                    panic!("Line is terminated but the block is unterminated")
                }
                Judgement::ExpressionTerminatedWithSemicolon(s) => self.buf = s,
                Judgement::NoConsumption => {}
            }

            let val = f(self)?;
            self.trim_start();
            match judge_termination_or_semicolons(self.buf, || ()) {
                Judgement::NoConsumption => {
                    self.consume_char_or_err('}', "Expected an operator or end of block")?;
                    return Ok(val);
                }
                Judgement::EndOfLineEncountered => {
                    panic!("Line is terminated but the block is unterminated")
                }
                Judgement::ExpressionTerminatedWithSemicolon(s) => self.buf = s,
            }
            self.trim_start();
            if let Some(buf_) = self.buf.strip_prefix('}') {
                self.buf = buf_.trim_start();
                return Ok(val);
            }
        }
    }

    fn parse_primary_expression(&mut self) -> Result<Value, &'static str> {
        let buf = self.buf.trim_start();
        if buf.starts_with('{') {
            self.parse_block_expression(Self::parse_expression)
        } else if let Some(buf) = buf.strip_prefix("$_") {
            self.buf = buf;
            Ok(self.previous_value.clone())
        } else if let Some(buf_) = buf.strip_prefix('(') {
            self.buf = buf_;
            let value = self.parse_expression()?;
            self.consume_char_or_err(')', "Mismatched parenthesis")?;
            Ok(value)
        } else if let Some(buf_) = buf.strip_prefix('[') {
            self.buf = buf_;
            let first_value = self.parse_expression()?;
            let buf = self.buf.trim_start();
            if let Some(buf_) = buf.strip_prefix(']') {
                self.buf = buf_;
                Ok(first_value)
            } else if let Some(buf_) = buf.strip_prefix(';') {
                self.buf = buf_;
                // Currently forbid trailing commas
                // what follows is (<value> <comma>)* <value> <]>
                let mut values = vec![first_value];
                loop {
                    let val = self.parse_expression()?;
                    values.push(val);
                    let buf = self.buf.trim_start();
                    if let Some(buf_) = buf.strip_prefix(',') {
                        self.buf = buf_;
                        continue;
                    } else if let Some(buf_) = buf.strip_prefix(']') {
                        self.buf = buf_;
                        break;
                    }
                }

                let final_result = values
                    .into_iter()
                    .rev()
                    .reduce(|acc, e| acc.recip() + e)
                    .unwrap();
                Ok(final_result)
            } else {
                Err("Expected `]` or `;` after the first slot of a continued-fraction literal")
            }
        } else {
            let (value, remaining) =
                numeric_literal::parse_numeric_literal_with_radix_context(buf, self.radix_context)?;
            self.buf = remaining;
            Ok(value)
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier, &'static str> {
        static RE_IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-zA-Z_]+").unwrap());

        match RE_IDENTIFIER.captures(self.buf) {
            None => Err("No identifier found after `@`"),
            Some(u) => {
                let whole = u.get(0).unwrap().as_str();
                self.buf = self.buf.strip_prefix(whole).unwrap();
                Ok(Identifier(String::from(whole)))
            }
        }
    }
}

impl Identifier {
    fn to_radix(&self) -> Option<u32> {
        let radix: u32 = match &self.0[..] {
            "binary" => 2,
            "trinary" | "ternary" => 3,
            "quaternary" => 4,
            "quinary" | "pental" => 5,
            "senary" | "seximal" | "heximal" => 6,
            "octal" | "oct" => 8,
            "decimal" | "denary" | "decanary" | "dec" => 10,
            "duodecimal" | "dozenal" => 12,
            "hexadecimal" | "hex" => 16,
            "vigesimal" => 20,
            _ => return None,
        };
        Some(radix)
    }
}
