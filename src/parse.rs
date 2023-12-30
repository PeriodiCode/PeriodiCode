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
        if input.starts_with('(') {
            let input = &input[1..];
            let (value, input) = self.parse_expression(input)?;
            if !input.starts_with(')') {
                return Err("Mismatched parenthesis");
            }
            let input = &input[1..];
            Ok((value, input))
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
        self.parse_primary_expression(input)
    }
}
