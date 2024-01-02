# PeriodiCode
A programming language where you can use repeating decimals and periodic continued fractions seamlessly (WIP)

## Trying it out

1. [Install Rust](https://www.rust-lang.org/learn/get-started)
2. Clone this repository
3. `cargo run`

## Features

See [long_tutorial.periodicode](./long_tutorial.periodicode) for the full tutorial.

[summary.periodicode](./summary.periodicode) gives a more concise summary.

[literal.periodicode](./literal.periodicode) showcases the capability of numeric literals.

### Repeating decimals

Supports repeating decimals and automatically turns it into a fraction.

For instance, if you want to represent `0.16666....`, write `0.1r6` ("zero point one; repeated sixes)

### Continued fraction

Natively supports the standard syntax `[3; 7, 15, 1]`

### Base freedom
You can choose what base/radix you use to represent numbers

- You set the "radix context" with which everything is to be done
<!-- Supports up to base 25 (so that I can use the letters `pqrstuvwxyz` to serve special purposes) -->
- Numeric literals support a wide range of radix-specifying prefixes (i.e. `0x` for hexadecimal), to bring a literal with an out-of-context radix
  - `0v`: vigesimal (base 20)
  - `0x`: hexadecimal (base 16)
  - `0z`: dozenal (base 12)
  - `0d`: decimal (base 10)
  - `0o`: octal (base 8)
  - `0s`: senary (base 6)
  - `0qn`: quinary (base 5)
  - `0qt`: quaternary (base 4)
  - `0t`: trinary (base 3)
  - `0b`: binary (base 2)

### Scaling by the exponent

Intentionally designed so that C++-style `1e10`, `12e-5`, `0x1ffp10` are incorporated.

- `e` or `xp`: multiplies the number by the power of the literal's own radix. 
  - `e` can only be used if the base is less than fifteen
 
- `p`: multiplies the number by powers of two.

The digits following `e`, `xp` or `p` is interpreted with the power of radix (taken from the external context).

Hence, in decimal context, 0x1.0p10 == 1024 and 0x11p-10 == 17/1024
