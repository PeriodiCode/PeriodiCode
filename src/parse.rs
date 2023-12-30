use num_rational::BigRational;
use once_cell::sync::Lazy;
use regex::Regex;

pub mod numeric_literal;

type Value = BigRational;

pub struct Parser {
    radix_context: u32,
    previous_value: Option<Value>,
}

struct Identifier(String);

impl Parser {
    pub fn new() -> Self {
        Self {
            radix_context: 10,
            previous_value: None,
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

    pub fn parse_expression<'a>(&mut self, buf: &'a str) -> Result<(Value, &'a str), &'static str> {
        let buf = buf.trim_start();
        self.parse_additive_expression(buf)
    }

    fn parse_additive_expression<'a>(
        &mut self,
        buf: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
        let buf = buf.trim_start();
        let (mut val, buf) = self.parse_multiplicative_expression(buf)?;
        let mut buf = buf;
        loop {
            if let Some(stripped) = buf.trim_start().strip_prefix('+') {
                let (val2, buf2) = self.parse_multiplicative_expression(stripped)?;
                val += val2;
                buf = buf2;
            } else if let Some(stripped) = buf.trim_start().strip_prefix('-') {
                let (val2, buf2) = self.parse_multiplicative_expression(stripped)?;
                val -= val2;
                buf = buf2;
            } else {
                break;
            }
        }

        Ok((val, buf))
    }

    fn parse_multiplicative_expression<'a>(
        &mut self,
        buf: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
        let buf = buf.trim_start();
        let (mut val, buf) = self.parse_unary_expression(buf)?;
        let mut buf = buf;
        loop {
            if let Some(stripped) = buf.trim_start().strip_prefix('*') {
                let (val2, buf2) = self.parse_unary_expression(stripped)?;
                val *= val2;
                buf = buf2;
            } else if let Some(stripped) = buf.trim_start().strip_prefix('/') {
                let (val2, buf2) = self.parse_unary_expression(stripped)?;
                val /= val2;
                buf = buf2;
            } else {
                break;
            }
        }

        Ok((val, buf))
    }

    fn parse_unary_expression<'a>(
        &mut self,
        buf: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
        let buf = buf.trim_start();
        if let Some(stripped) = buf.strip_prefix('+') {
            let (value, buf) = self.parse_unary_expression(stripped)?;
            Ok((value, buf))
        } else if let Some(stripped) = buf.strip_prefix('-') {
            let (value, buf) = self.parse_unary_expression(stripped)?;
            Ok((-value, buf))
        } else {
            self.parse_funccall_expression(buf)
        }
    }

    fn parse_funccall_expression<'a>(
        &mut self,
        buf: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
        let buf = buf.trim_start();
        if let Some(buf) = buf.strip_prefix('@') {
            let buf = buf.trim_start();
            let (ident, buf) = self.parse_identifier(buf)?;
            if ident.0 == "assert_eq" {
                let buf = buf.trim_start();
                if let Some(buf) = buf.strip_prefix('(') {
                    let buf = buf.trim_start();

                    let (first_arg, buf) = self.parse_expression(buf)?;
                    let buf = buf.trim_start();

                    if let Some(buf) = buf.strip_prefix(',') {
                        let buf = buf.trim_start();

                        let (second_arg, buf) = self.parse_expression(buf)?;
                        let buf = buf.trim_start();
                        if let Some(buf) = buf.strip_prefix(')') {
                            let buf = buf.trim_start();
                            if first_arg == second_arg {
                                Ok((first_arg, buf)) // @assert_eq(7*6, 42) returns 42
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
            self.parse_primary_expression(buf)
        }
    }

    fn parse_primary_expression<'a>(
        &mut self,
        buf: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
        let buf = buf.trim_start();
        if let Some(stripped) = buf.strip_prefix("$_") {
            Ok((self.previous_value.clone().unwrap(), stripped))
        } else if let Some(stripped) = buf.strip_prefix('(') {
            let (value, buf) = self.parse_expression(stripped)?;
            let buf = buf.trim_start();
            if let Some(stripped) = buf.strip_prefix(')') {
                Ok((value, stripped))
            } else {
                Err("Mismatched parenthesis")
            }
        } else {
            let (value, remaining) =
                numeric_literal::parse_numeric_literal_with_radix_context(buf, self.radix_context)?;
            Ok((value, remaining))
        }
    }

    fn parse_identifier<'a>(&self, buf: &'a str) -> Result<(Identifier, &'a str), &'static str> {
        static RE_IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-zA-Z_]+").unwrap());

        match RE_IDENTIFIER.captures(buf) {
            None => Err("No identifier found after `@`"),
            Some(u) => {
                let whole = u.get(0).unwrap().as_str();
                return Ok((
                    Identifier(String::from(whole)),
                    buf.strip_prefix(whole).unwrap(),
                ));
            }
        }
    }
}
