use num_rational::BigRational;

pub mod numeric_literal;

type Value = BigRational;

pub struct Parser {
    radix_context: u32,
}

impl Parser {
    pub fn new() -> Self {
        Self { radix_context: 10 }
    }

    pub fn set_radix_context(&mut self, new: u32) {
        if new > 25 {
            panic!("radix greater than 25 is not supported")
        }
        self.radix_context = new;
    }

    fn parse_primary_expression<'a>(
        &mut self,
        input: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
        let input = input.trim_start();
        if let Some(stripped) = input.strip_prefix('(') {
            let (value, input) = self.parse_expression(stripped)?;
            if let Some(stripped) = input.strip_prefix(')') {
                Ok((value, stripped))
            } else {
                Err("Mismatched parenthesis")
            }
        } else {
            let (value, remaining) = numeric_literal::parse_numeric_literal_with_radix_context(
                input,
                self.radix_context,
            )?;
            Ok((value, remaining))
        }
    }

    pub fn parse_expression<'a>(
        &mut self,
        input: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
        let input = input.trim_start();
        self.parse_multiplicative_expression(input)
    }

    pub fn parse_multiplicative_expression<'a>(
        &mut self,
        input: &'a str,
    ) -> Result<(Value, &'a str), &'static str> {
        let input = input.trim_start();
        let (mut val, remaining) = self.parse_primary_expression(input)?;
        let mut input = remaining;
        loop {
            if let Some(stripped) = input.trim_start().strip_prefix('*') {
                let (val2, input2) = self.parse_primary_expression(stripped)?;
                val *= val2;
                input = input2;
            } else if let Some(stripped) = input.trim_start().strip_prefix('/') {
                let (val2, input2) = self.parse_primary_expression(stripped)?;
                val /= val2;
                input = input2;
            } else {
                break;
            }
        }

        Ok((val, input))
    }
}
