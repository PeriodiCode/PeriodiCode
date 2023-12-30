use num_rational::BigRational;

pub mod numeric_literal;

type Value = BigRational;

pub struct Parser {
    radix_context: u32,
    previous_value: Option<Value>,
}

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

    pub fn parse_expression<'a>(
        &mut self,
        buf: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
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
            let (value, remaining) = numeric_literal::parse_numeric_literal_with_radix_context(
                buf,
                self.radix_context,
            )?;
            Ok((value, remaining))
        }
    }
}
