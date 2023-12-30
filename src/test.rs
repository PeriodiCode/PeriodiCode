use crate::parse::numeric_literal::parse_numeric_literal_with_radix_context;
use num_rational::BigRational;

fn numeric_literal(input: &str) -> BigRational {
    let (ans, remaining) = parse_numeric_literal_with_radix_context(input, 10).unwrap();
    assert!(remaining.is_empty());
    ans
}

#[test]
fn parser_test() {
    let (ans, remaining) = parse_numeric_literal_with_radix_context("12.;", 10).unwrap();
    assert_eq!(ans.to_string(), "12");
    assert_eq!(remaining, ";");
}

#[test]
fn test() {
    assert_eq!(numeric_literal(".6r142857").to_string(), "43/70");
    assert_eq!(numeric_literal(".r142857").to_string(), "1/7");
    assert_eq!(numeric_literal("0.1r6").to_string(), "1/6");
    assert_eq!(numeric_literal("12.").to_string(), "12");
    assert_eq!(numeric_literal("12").to_string(), "12");
    assert_eq!(numeric_literal("12.1").to_string(), "121/10");
    assert_eq!(numeric_literal("12.1r6").to_string(), "73/6");
    assert_eq!(numeric_literal(".1r6").to_string(), "1/6");
    assert_eq!(numeric_literal(".r3").to_string(), "1/3");
    assert_eq!(
        parse_numeric_literal_with_radix_context(".r0313452421", 6)
            .unwrap()
            .0
            .to_string(),
        "1/11"
    );

    assert_eq!(numeric_literal("0v100").to_string(), "400");
    assert_eq!(numeric_literal("0x100").to_string(), "256");
    assert_eq!(numeric_literal("0z100").to_string(), "144");
    assert_eq!(numeric_literal("0d100").to_string(), "100");
    assert_eq!(numeric_literal("0o100").to_string(), "64");
    assert_eq!(numeric_literal("0s100").to_string(), "36");
    assert_eq!(numeric_literal("0qn100").to_string(), "25");
    assert_eq!(numeric_literal("0qt100").to_string(), "16");
    assert_eq!(numeric_literal("0t100").to_string(), "9");
    assert_eq!(numeric_literal("0b100").to_string(), "4");

    assert_eq!(numeric_literal("0v100.").to_string(), "400");
    assert_eq!(numeric_literal("0x100.").to_string(), "256");
    assert_eq!(numeric_literal("0z100.").to_string(), "144");
    assert_eq!(numeric_literal("0d100.").to_string(), "100");
    assert_eq!(numeric_literal("0o100.").to_string(), "64");
    assert_eq!(numeric_literal("0s100.").to_string(), "36");
    assert_eq!(numeric_literal("0qn100.").to_string(), "25");
    assert_eq!(numeric_literal("0qt100.").to_string(), "16");
    assert_eq!(numeric_literal("0t100.").to_string(), "9");
    assert_eq!(numeric_literal("0b100.").to_string(), "4");

    assert_eq!(numeric_literal("0x1.p10").to_string(), "1024");
    assert_eq!(numeric_literal("0x1.p-10").to_string(), "1/1024");
    assert_eq!(numeric_literal("0b11.p-10").to_string(), "3/1024");
    assert_eq!(numeric_literal("0x11.p-10").to_string(), "17/1024");
    assert_eq!(numeric_literal("0d11.p-10").to_string(), "11/1024");
    assert_eq!(
        parse_numeric_literal_with_radix_context("0x1.p10", 6)
            .unwrap()
            .0
            .to_string(),
        "64"
    );

    assert_eq!(numeric_literal("0s.r0313452421").to_string(), "1/11");

    assert_eq!(numeric_literal("0.1r6e1").to_string(), "5/3");
    assert_eq!(numeric_literal("0.1r6xp1").to_string(), "5/3");
    assert_eq!(
        parse_numeric_literal_with_radix_context("1.0p10", 10)
            .unwrap()
            .0
            .to_string(),
        "1024"
    );
}
