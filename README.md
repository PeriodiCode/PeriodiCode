# PeriodiCode
A programming language where you can use repeating decimals and periodic continued fractions seamlessly (WIP)

## Current progress

```rs
fn main() {
    println!("{}", parse_pointed_literal("0.1r6"));
    println!("{}", parse_pointed_literal("12."));
    println!("{}", parse_pointed_literal("12.1"));
    println!("{}", parse_pointed_literal("12.1r6"));
    println!("{}", parse_pointed_literal(".1r6"));
    println!("{}", parse_pointed_literal(".r3"));
    println!("{}", parse_pointed_literal(".r142857"));
    println!("{}", parse_pointed_literal_radix(".r0313452421", 6));
}
```

↓

```
parsing `0.1r6` (radix: 10 in decimal)
1/6

parsing `12.` (radix: 10 in decimal)
12

parsing `12.1` (radix: 10 in decimal)
121/10

parsing `12.1r6` (radix: 10 in decimal)
73/6

parsing `.1r6` (radix: 10 in decimal)
1/6

parsing `.r3` (radix: 10 in decimal)
1/3

parsing `.r142857` (radix: 10 in decimal)
1/7

parsing `.r0313452421` (radix: 6 in decimal)
1/11
```
