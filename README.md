# PeriodiCode
A programming language where you can use repeating decimals and periodic continued fractions seamlessly (WIP)

## Features

### Repeating decimals

Supports repeating decimals and automatically turns it into a fraction.

For instance, if you want to represent `0.16666....`, write `0.1r6` ("zero point one; repeated sixes)

### Base freedom
You can choose what base/radix you use to represent numbers

- You set the "radix context" with which everything is to be done
- Supports up to base 25 (so that I can use the letters `pqrstuvwxyz` to serve special purposes)
- Numeric literals support a wide range of radix-specifying prefixes (i.e. `0x` for hexadecimal), to bring a literal with an out-of-context radix
  - `0v`: vigesimal (base 20)
  - `0x`: hexadecimal (base 16)
  - `0z`: dozenal (base 12)
  - `0d`: decimal (base 10)
  - `0o`: octal (base 8)
  - `0s`: senary (base 6)
  - `0quin`: quinary (base 5)
  - `0quat`: quaternary (base 4)
  - `0t`: trinary (base 3)
  - `0b`: binary (base 2)

## Current progress

```rs
fn main() {
    assert_eq!(parse_pointed_literal("0.1r6").to_string(), "1/6");
    assert_eq!(parse_pointed_literal("12.").to_string(), "12");
    assert_eq!(parse_pointed_literal("12.1").to_string(), "121/10");
    assert_eq!(parse_pointed_literal("12.1r6").to_string(), "73/6");
    assert_eq!(parse_pointed_literal(".1r6").to_string(), "1/6");
    assert_eq!(parse_pointed_literal(".r3").to_string(), "1/3");
    assert_eq!(parse_pointed_literal(".r142857").to_string(), "1/7");
    assert_eq!(
        parse_pointed_literal_with_radix_context(".r0313452421", 6).to_string(),
        "1/11"
    );

    assert_eq!(parse_pointed_literal("0v100.").to_string(), "400");
    assert_eq!(parse_pointed_literal("0x100.").to_string(), "256");
    assert_eq!(parse_pointed_literal("0z100.").to_string(), "144");
    assert_eq!(parse_pointed_literal("0d100.").to_string(), "100");
    assert_eq!(parse_pointed_literal("0o100.").to_string(), "64");
    assert_eq!(parse_pointed_literal("0s100.").to_string(), "36");
    assert_eq!(parse_pointed_literal("0quin100.").to_string(), "25");
    assert_eq!(parse_pointed_literal("0quat100.").to_string(), "16");
    assert_eq!(parse_pointed_literal("0t100.").to_string(), "9");
    assert_eq!(parse_pointed_literal("0b100.").to_string(), "4");

    assert_eq!(parse_pointed_literal("0x1.p10").to_string(), "1024");
    assert_eq!(
        parse_pointed_literal_with_radix_context("0x1.p10", 6).to_string(),
        "64"
    );

    assert_eq!(parse_pointed_literal("0s.r0313452421").to_string(), "1/11");

    assert_eq!(parse_pointed_literal("0.1r6e1").to_string(), "5/3");
    assert_eq!(parse_pointed_literal("0.1r6xp1").to_string(), "5/3");
    assert_eq!(
        parse_pointed_literal_with_radix_context("1.0p10", 10).to_string(),
        "1024"
    );
}
```

↓

```
[radix_context: 10 in decimal] 0.1r6 => 1/6
[radix_context: 10 in decimal] 12. => 12
[radix_context: 10 in decimal] 12.1 => 121/10
[radix_context: 10 in decimal] 12.1r6 => 73/6
[radix_context: 10 in decimal] .1r6 => 1/6
[radix_context: 10 in decimal] .r3 => 1/3
[radix_context: 10 in decimal] .r142857 => 1/7
[radix_context:  6 in decimal] .r0313452421 => 1/11
[radix_context: 10 in decimal] 0v100. => 400
[radix_context: 10 in decimal] 0x100. => 256
[radix_context: 10 in decimal] 0z100. => 144
[radix_context: 10 in decimal] 0d100. => 100
[radix_context: 10 in decimal] 0o100. => 64
[radix_context: 10 in decimal] 0s100. => 36
[radix_context: 10 in decimal] 0quin100. => 25
[radix_context: 10 in decimal] 0quat100. => 16
[radix_context: 10 in decimal] 0t100. => 9
[radix_context: 10 in decimal] 0b100. => 4
[radix_context: 10 in decimal] 0x1.p10 => 1024
[radix_context:  6 in decimal] 0x1.p10 => 64
[radix_context: 10 in decimal] 0s.r0313452421 => 1/11
[radix_context: 10 in decimal] 0.1r6e1 => 5/3
[radix_context: 10 in decimal] 0.1r6xp1 => 5/3
[radix_context: 10 in decimal] 1.0p10 => 1024
```
