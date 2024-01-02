use crate::numerical_util::floor_as_bigint;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::identities::One;
use num_traits::Zero;

pub fn rational_print_summary(ans: &BigRational, external_radix_context: u32) {
    let numer = ans.numer();
    let denom = ans.denom();

    print!("frac: ");

    /* print fractional */
    if denom == &BigInt::one() {
        print!("{}", numer.to_str_radix(external_radix_context));
    } else {
        print!(
            "{}/{}",
            numer.to_str_radix(external_radix_context),
            denom.to_str_radix(external_radix_context)
        );
    }
    if external_radix_context != 10 {
        print!(" \x1b[2;32m# @decimal {{ {ans} }}\x1b[00m"); // faint green
    }

    println!();

    print!("cont: ");

    print_continued_fraction_radix(ans, external_radix_context);

    if external_radix_context != 10 {
        print!(" \x1b[2;32m# @decimal {{ "); // faint green
        print_continued_fraction_radix(ans, 10);
        print!(" }}\x1b[00m"); // reset
    }

    println!();

    print!("digt: ");

    print_digit_expansion_radix(ans, external_radix_context);

    if external_radix_context != 10 {
        print!(" \x1b[2;32m# @decimal {{ "); // faint green
        print_digit_expansion_radix(ans, 10);
        print!(" }}\x1b[00m"); // reset
    }

    println!();
}

fn print_continued_fraction_radix(ans: &BigRational, external_radix_context: u32) {
    let mut cont_frac_iter = FiniteContinuedFractionIter::new(ans);
    let initial = cont_frac_iter.next().expect("empty iterator");
    let remaining: Vec<BigInt> = cont_frac_iter.collect();
    if remaining.is_empty() {
        print!("[{}]", initial.to_str_radix(external_radix_context));
    } else {
        print!(
            "[{}; {}]",
            initial.to_str_radix(external_radix_context),
            remaining
                .into_iter()
                .map(|n| n.to_str_radix(external_radix_context))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

fn print_digit_expansion_radix(ans: &BigRational, external_radix_context: u32) {
    if ans < &BigRational::zero() {
        print!("-");
        print_digit_expansion_radix(&-ans, external_radix_context);
        return;
    }

    print!(
        "{}",
        floor_as_bigint(ans).to_str_radix(external_radix_context)
    );

    let mut f = ans - ans.floor();
    if f.is_zero() {
        return;
    }

    print!(".");

    let mut f_list = vec![];
    let mut digits = vec![];

    loop {
        f_list.push(f.clone());

        f *= BigInt::from(external_radix_context);

        let digit = floor_as_bigint(&f).to_str_radix(external_radix_context);
        digits.push(digit.clone());

        f = f.clone() - f.floor();
        if f.is_zero() {
            print!("{}", digits.join(""));
            return;
        }

        if f_list.contains(&f) {
            let pos = f_list.iter().position(|k| k == &f).expect("empty iterator");
            print!("{}r{}", digits[0..pos].join(""), digits[pos..].join(""));
            return;
        }
    }
}

enum FiniteContinuedFractionIter {
    Ratio(BigRational),
    Infinity,
}

impl FiniteContinuedFractionIter {
    fn new(s: &BigRational) -> Self {
        Self::Ratio(s.clone())
    }
}

impl Iterator for FiniteContinuedFractionIter {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FiniteContinuedFractionIter::Ratio(r) => {
                let n = floor_as_bigint(r);
                let f = &*r - r.floor();
                if f == BigRational::zero() {
                    *self = Self::Infinity;
                } else {
                    *self = Self::Ratio(f.recip());
                }
                Some(n)
            }
            FiniteContinuedFractionIter::Infinity => None,
        }
    }
}
