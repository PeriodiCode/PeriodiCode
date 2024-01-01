use num_rational::BigRational;
use once_cell::sync::Lazy;
use regex::Regex;

pub mod numeric_literal;

type Value = BigRational;

pub struct Parser<'a> {
    radix_context: u32,
    previous_value: Option<Value>,
    buf: &'a str,
}

struct Identifier(String);

impl<'b> Parser<'b> {
    pub fn get_buf(&self) -> &str {
        self.buf
    }
    pub fn new(buf: &'b str) -> Self {
        Self {
            radix_context: 10,
            previous_value: None,
            buf,
        }
    }

    pub fn get_radix_context(&mut self) -> u32 {
        self.radix_context
    }

    pub fn set_radix_context(&mut self, new: u32) {
        assert!(new <= 25, "radix greater than 25 is not supported");
        self.radix_context = new;
    }

    pub fn set_previous_value(&mut self, previous_value: Option<Value>) {
        self.previous_value = previous_value;
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
            self.parse_funccall_expression()
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

    fn parse_funccall_expression(&mut self) -> Result<Value, &'static str> {
        self.trim_start();
        if let Some(buf_) = self.buf.strip_prefix('@') {
            self.buf = buf_.trim_start();
            let ident = self.parse_identifier()?;
            if ident.0 == "assert_eq" {
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
            } else {
                panic!("UNSUPPORTED FUNCTION: `@{}`", ident.0)
            }
        } else {
            self.parse_primary_expression()
        }
    }

    fn parse_primary_expression(&mut self) -> Result<Value, &'static str> {
        let buf = self.buf.trim_start();
        if let Some(buf) = buf.strip_prefix("$_") {
            self.buf = buf;
            Ok(self.previous_value.clone().unwrap())
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

                let final_result = values.into_iter().rev().reduce(|acc, e| acc.recip() + e).unwrap();
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
