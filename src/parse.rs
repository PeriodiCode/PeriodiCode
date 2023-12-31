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

    pub fn set_radix_context(&mut self, new: u32) {
        if new > 25 {
            panic!("radix greater than 25 is not supported")
        }
        self.radix_context = new;
    }

    pub fn set_previous_value(&mut self, previous_value: Option<Value>) {
        self.previous_value = previous_value;
    }

    pub fn parse_expression(&mut self) -> Result<Value, &'static str> {
        self.buf = self.buf.trim_start();
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> Result<Value, &'static str> {
        self.buf = self.buf.trim_start();
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
        self.buf = self.buf.trim_start();
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

    fn parse_funccall_expression(&mut self) -> Result<Value, &'static str> {
        self.buf = self.buf.trim_start();
        if let Some(buf_) = self.buf.strip_prefix('@') {
            self.buf = buf_.trim_start();
            let ident = self.parse_identifier()?;
            if ident.0 == "assert_eq" {
                self.buf = self.buf.trim_start();
                if let Some(buf_) = self.buf.strip_prefix('(') {
                    self.buf = buf_.trim_start();
                    let first_arg = self.parse_expression()?;
                    self.buf = self.buf.trim_start();

                    if let Some(buf_) = self.buf.strip_prefix(',') {
                        self.buf = buf_.trim_start();

                        let second_arg = self.parse_expression()?;
                        self.buf = self.buf.trim_start();
                        if let Some(buf_) = self.buf.strip_prefix(')') {
                            self.buf = buf_.trim_start();
                            if first_arg == second_arg {
                                Ok(first_arg) // @assert_eq(7*6, 42) returns 42
                            } else {
                                panic!(
                                    "ASSERTION FAILED: \nleft: {}\nright: {}",
                                    first_arg, second_arg
                                )
                            }
                        } else {
                            Err("The built-in function `assert_eq` expects exactly two arguments")
                        }
                    } else {
                        Err("The built-in function `assert_eq` expects exactly two arguments")
                    }
                } else {
                    Err("No parenthesis after the built-in function `assert_eq`")
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
            self.buf = self.buf.trim_start();
            if let Some(buf) = self.buf.strip_prefix(')') {
                self.buf = buf;
                Ok(value)
            } else {
                Err("Mismatched parenthesis")
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
