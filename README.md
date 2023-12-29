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

### Scaling by the exponent

Intentionally designed so that C++-style `1e10`, `12e-5`, `0x1ffp10` are incorporated.

- `e` or `xp`: multiplies the number by the power of the literal's own radix. 
  - `e` can only be used if the base is less than fifteen
 
- `p`: multiplies the number by powers of two.

The digits following `e`, `xp` or `p` is interpreted with the power of radix (taken from the external context).

Hence, in decimal context, 0x1.0p10 == 1024 and 0x11p-10 == 17/1024

## Current progress

```rs
fn main() {
    assert_eq!(parse_numeric_literal("0.1r6").to_string(), "1/6");
    assert_eq!(parse_numeric_literal("12.").to_string(), "12");
    assert_eq!(parse_numeric_literal("12").to_string(), "12");
    assert_eq!(parse_numeric_literal("12.1").to_string(), "121/10");
    assert_eq!(parse_numeric_literal("12.1r6").to_string(), "73/6");
    assert_eq!(parse_numeric_literal(".1r6").to_string(), "1/6");
    assert_eq!(parse_numeric_literal(".r3").to_string(), "1/3");
    assert_eq!(parse_numeric_literal(".r142857").to_string(), "1/7");
    assert_eq!(
        parse_numeric_literal_with_radix_context(".r0313452421", 6).to_string(),
        "1/11"
    );

    assert_eq!(parse_numeric_literal("0v100").to_string(), "400");
    assert_eq!(parse_numeric_literal("0x100").to_string(), "256");
    assert_eq!(parse_numeric_literal("0z100").to_string(), "144");
    assert_eq!(parse_numeric_literal("0d100").to_string(), "100");
    assert_eq!(parse_numeric_literal("0o100").to_string(), "64");
    assert_eq!(parse_numeric_literal("0s100").to_string(), "36");
    assert_eq!(parse_numeric_literal("0quin100").to_string(), "25");
    assert_eq!(parse_numeric_literal("0quat100").to_string(), "16");
    assert_eq!(parse_numeric_literal("0t100").to_string(), "9");
    assert_eq!(parse_numeric_literal("0b100").to_string(), "4");

    assert_eq!(parse_numeric_literal("0v100.").to_string(), "400");
    assert_eq!(parse_numeric_literal("0x100.").to_string(), "256");
    assert_eq!(parse_numeric_literal("0z100.").to_string(), "144");
    assert_eq!(parse_numeric_literal("0d100.").to_string(), "100");
    assert_eq!(parse_numeric_literal("0o100.").to_string(), "64");
    assert_eq!(parse_numeric_literal("0s100.").to_string(), "36");
    assert_eq!(parse_numeric_literal("0quin100.").to_string(), "25");
    assert_eq!(parse_numeric_literal("0quat100.").to_string(), "16");
    assert_eq!(parse_numeric_literal("0t100.").to_string(), "9");
    assert_eq!(parse_numeric_literal("0b100.").to_string(), "4");

    assert_eq!(parse_numeric_literal("0x1.p-10").to_string(), "1/1024");
    assert_eq!(parse_numeric_literal("0b11.p-10").to_string(), "3/1024");
    assert_eq!(parse_numeric_literal("0x11.p-10").to_string(), "17/1024");
    assert_eq!(parse_numeric_literal("0d11.p-10").to_string(), "11/1024");
    assert_eq!(
        parse_numeric_literal_with_radix_context("0x1.p10", 6).to_string(),
        "64"
    );

    assert_eq!(parse_numeric_literal("0s.r0313452421").to_string(), "1/11");

    assert_eq!(parse_numeric_literal("0.1r6e1").to_string(), "5/3");
    assert_eq!(parse_numeric_literal("0.1r6xp1").to_string(), "5/3");
    assert_eq!(
        parse_numeric_literal_with_radix_context("1.0p10", 10).to_string(),
        "1024"
    );
}
```

↓

```
PeriodiCode:DEC10$ 0.1r6
frac: 1/6
cont: [0; 6]

PeriodiCode:DEC10$ 12.
frac: 12
cont: [12]

PeriodiCode:DEC10$ 12
frac: 12
cont: [12]

PeriodiCode:DEC10$ 12.1
frac: 121/10
cont: [12; 10]

PeriodiCode:DEC10$ 12.1r6
frac: 73/6
cont: [12; 6]

PeriodiCode:DEC10$ .1r6
frac: 1/6
cont: [0; 6]

PeriodiCode:DEC10$ .r3
frac: 1/3
cont: [0; 3]

PeriodiCode:DEC10$ .r142857
frac: 1/7
cont: [0; 7]

PeriodiCode:DEC6 $ .r0313452421
frac: 1/15 (DEC1/11)
cont: [0; 15] (DEC[0; 11])

PeriodiCode:DEC10$ 0v100
frac: 400
cont: [400]

PeriodiCode:DEC10$ 0x100
frac: 256
cont: [256]

PeriodiCode:DEC10$ 0z100
frac: 144
cont: [144]

PeriodiCode:DEC10$ 0d100
frac: 100
cont: [100]

PeriodiCode:DEC10$ 0o100
frac: 64
cont: [64]

PeriodiCode:DEC10$ 0s100
frac: 36
cont: [36]

PeriodiCode:DEC10$ 0quin100
frac: 25
cont: [25]

PeriodiCode:DEC10$ 0quat100
frac: 16
cont: [16]

PeriodiCode:DEC10$ 0t100
frac: 9
cont: [9]

PeriodiCode:DEC10$ 0b100
frac: 4
cont: [4]

PeriodiCode:DEC10$ 0v100.
frac: 400
cont: [400]

PeriodiCode:DEC10$ 0x100.
frac: 256
cont: [256]

PeriodiCode:DEC10$ 0z100.
frac: 144
cont: [144]

PeriodiCode:DEC10$ 0d100.
frac: 100
cont: [100]

PeriodiCode:DEC10$ 0o100.
frac: 64
cont: [64]

PeriodiCode:DEC10$ 0s100.
frac: 36
cont: [36]

PeriodiCode:DEC10$ 0quin100.
frac: 25
cont: [25]

PeriodiCode:DEC10$ 0quat100.
frac: 16
cont: [16]

PeriodiCode:DEC10$ 0t100.
frac: 9
cont: [9]

PeriodiCode:DEC10$ 0b100.
frac: 4
cont: [4]

PeriodiCode:DEC10$ 0x1.p10
frac: 1024
cont: [1024]

PeriodiCode:DEC10$ 0x1.p-10
frac: 1/1024
cont: [0; 1024]

PeriodiCode:DEC10$ 0b11.p-10
frac: 3/1024
cont: [0; 341, 3]

PeriodiCode:DEC10$ 0x11.p-10
frac: 17/1024
cont: [0; 60, 4, 4]

PeriodiCode:DEC10$ 0d11.p-10
frac: 11/1024
cont: [0; 93, 11]

PeriodiCode:DEC6 $ 0x1.p10
frac: 144 (DEC64)
cont: [144] (DEC[64])

PeriodiCode:DEC10$ 0s.r0313452421
frac: 1/11
cont: [0; 11]

PeriodiCode:DEC10$ 0.1r6e1
frac: 5/3
cont: [1; 1, 2]

PeriodiCode:DEC10$ 0.1r6xp1
frac: 5/3
cont: [1; 1, 2]

PeriodiCode:DEC10$ 1.0p10
frac: 1024
cont: [1024]
```
