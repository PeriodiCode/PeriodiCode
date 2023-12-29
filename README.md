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

## Current progress

`example.periodicode` contains the following:

```
# repeated decimals
.6r142857
@assert_eq($_, 43/70)

.r142857
@assert_eq($_, 1/7)

0.1r6
@assert_eq($_, 1/6)

12.
@assert_eq($_, 12)

12
@assert_eq($_, 12)

12.1
@assert_eq($_, 121/10)

12.1r6
@assert_eq($_, 73/6)

.1r6
@assert_eq($_, 1/6)

.r3
@assert_eq($_, 1/3)

##################################################
# you can use a wide variety of prefix
# to specify the base of a particular literal
##################################################

# vigesimal
0v100
@assert_eq($_, 400)

# hexadecimal
0x100
@assert_eq($_, 256)

# dozenal
0z100
@assert_eq($_, 144)

# decimal
0d100
@assert_eq($_, 100)

# octal
0o100
@assert_eq($_, 64)

# senary
0s100
@assert_eq($_, 36)

# quinary
0qn100
@assert_eq($_, 25)

# quaternary
0qt100
@assert_eq($_, 16)

# ternary
0t100
@assert_eq($_, 9)

# binary
0b100
@assert_eq($_, 4)

##################################################
# you can write the same thing with an extra dot 
# and it makes no difference
##################################################
0v100.
@assert_eq($_, 400)

0x100.
@assert_eq($_, 256)

0z100.
@assert_eq($_, 144)

0d100.
@assert_eq($_, 100)

0o100.
@assert_eq($_, 64)

0s100.
@assert_eq($_, 36)

0qn100.
@assert_eq($_, 25)

0qt100.
@assert_eq($_, 16)

0t100.
@assert_eq($_, 9)

0b100.
@assert_eq($_, 4)


# of course, you can use the prefix to denote repeated decimals
0s.r0313452421
@assert_eq($_, 1/11)


# `p` inside a literal denotes multiplying the whole literal by powers of two
# This is necessary to make it compatible with C++-style float literal
0x1.p10
@assert_eq($_, 1024)

0x1.p-10
@assert_eq($_, 1/1024)

0b11.p-10
@assert_eq($_, 3/1024)

0x11.p-10
@assert_eq($_, 17/1024)

0d11.p-10
@assert_eq($_, 11/1024)


# you can also use the `e` notation to multiply the literal by the power of its radix,
# but since you need `e` as a digit in hexadecimal, an equivalent alternative `xp` can be used as a fallback
0.1r6e1
@assert_eq($_, 5/3)

0.1r6xp1
@assert_eq($_, 5/3)

0x1e2
@assert_eq($_, 482)

0x1xp2
@assert_eq($_, 256)
```


This can be executed to give the following result:


```
PeriodiCode:DEC10$ .6r142857
frac: 43/70
cont: [0; 1, 1, 1, 1, 2, 5]
digt: 0.6r142857

PeriodiCode:DEC10$ @assert_eq($_, 43/70)
ok

PeriodiCode:DEC10$ .r142857
frac: 1/7
cont: [0; 7]
digt: 0.r142857

PeriodiCode:DEC10$ @assert_eq($_, 1/7)
ok

PeriodiCode:DEC10$ 0.1r6
frac: 1/6
cont: [0; 6]
digt: 0.1r6

PeriodiCode:DEC10$ @assert_eq($_, 1/6)
ok

PeriodiCode:DEC10$ 12.
frac: 12
cont: [12]
digt: 12

PeriodiCode:DEC10$ @assert_eq($_, 12)
ok

PeriodiCode:DEC10$ 12
frac: 12
cont: [12]
digt: 12

PeriodiCode:DEC10$ @assert_eq($_, 12)
ok

PeriodiCode:DEC10$ 12.1
frac: 121/10
cont: [12; 10]
digt: 12.1

PeriodiCode:DEC10$ @assert_eq($_, 121/10)
ok

PeriodiCode:DEC10$ 12.1r6
frac: 73/6
cont: [12; 6]
digt: 12.1r6

PeriodiCode:DEC10$ @assert_eq($_, 73/6)
ok

PeriodiCode:DEC10$ .1r6
frac: 1/6
cont: [0; 6]
digt: 0.1r6

PeriodiCode:DEC10$ @assert_eq($_, 1/6)
ok

PeriodiCode:DEC10$ .r3
frac: 1/3
cont: [0; 3]
digt: 0.r3

PeriodiCode:DEC10$ @assert_eq($_, 1/3)
ok

PeriodiCode:DEC10$ 0v100
frac: 400
cont: [400]
digt: 400

PeriodiCode:DEC10$ @assert_eq($_, 400)
ok

PeriodiCode:DEC10$ 0x100
frac: 256
cont: [256]
digt: 256

PeriodiCode:DEC10$ @assert_eq($_, 256)
ok

PeriodiCode:DEC10$ 0z100
frac: 144
cont: [144]
digt: 144

PeriodiCode:DEC10$ @assert_eq($_, 144)
ok

PeriodiCode:DEC10$ 0d100
frac: 100
cont: [100]
digt: 100

PeriodiCode:DEC10$ @assert_eq($_, 100)
ok

PeriodiCode:DEC10$ 0o100
frac: 64
cont: [64]
digt: 64

PeriodiCode:DEC10$ @assert_eq($_, 64)
ok

PeriodiCode:DEC10$ 0s100
frac: 36
cont: [36]
digt: 36

PeriodiCode:DEC10$ @assert_eq($_, 36)
ok

PeriodiCode:DEC10$ 0qn100
frac: 25
cont: [25]
digt: 25

PeriodiCode:DEC10$ @assert_eq($_, 25)
ok

PeriodiCode:DEC10$ 0qt100
frac: 16
cont: [16]
digt: 16

PeriodiCode:DEC10$ @assert_eq($_, 16)
ok

PeriodiCode:DEC10$ 0t100
frac: 9
cont: [9]
digt: 9

PeriodiCode:DEC10$ @assert_eq($_, 9)
ok

PeriodiCode:DEC10$ 0b100
frac: 4
cont: [4]
digt: 4

PeriodiCode:DEC10$ @assert_eq($_, 4)
ok

PeriodiCode:DEC10$ 0v100.
frac: 400
cont: [400]
digt: 400

PeriodiCode:DEC10$ @assert_eq($_, 400)
ok

PeriodiCode:DEC10$ 0x100.
frac: 256
cont: [256]
digt: 256

PeriodiCode:DEC10$ @assert_eq($_, 256)
ok

PeriodiCode:DEC10$ 0z100.
frac: 144
cont: [144]
digt: 144

PeriodiCode:DEC10$ @assert_eq($_, 144)
ok

PeriodiCode:DEC10$ 0d100.
frac: 100
cont: [100]
digt: 100

PeriodiCode:DEC10$ @assert_eq($_, 100)
ok

PeriodiCode:DEC10$ 0o100.
frac: 64
cont: [64]
digt: 64

PeriodiCode:DEC10$ @assert_eq($_, 64)
ok

PeriodiCode:DEC10$ 0s100.
frac: 36
cont: [36]
digt: 36

PeriodiCode:DEC10$ @assert_eq($_, 36)
ok

PeriodiCode:DEC10$ 0qn100.
frac: 25
cont: [25]
digt: 25

PeriodiCode:DEC10$ @assert_eq($_, 25)
ok

PeriodiCode:DEC10$ 0qt100.
frac: 16
cont: [16]
digt: 16

PeriodiCode:DEC10$ @assert_eq($_, 16)
ok

PeriodiCode:DEC10$ 0t100.
frac: 9
cont: [9]
digt: 9

PeriodiCode:DEC10$ @assert_eq($_, 9)
ok

PeriodiCode:DEC10$ 0b100.
frac: 4
cont: [4]
digt: 4

PeriodiCode:DEC10$ @assert_eq($_, 4)
ok

PeriodiCode:DEC10$ 0s.r0313452421
frac: 1/11
cont: [0; 11]
digt: 0.r09

PeriodiCode:DEC10$ @assert_eq($_, 1/11)
ok

PeriodiCode:DEC10$ 0x1.p10
frac: 1024
cont: [1024]
digt: 1024

PeriodiCode:DEC10$ @assert_eq($_, 1024)
ok

PeriodiCode:DEC10$ 0x1.p-10
frac: 1/1024
cont: [0; 1024]
digt: 0.0009765625

PeriodiCode:DEC10$ @assert_eq($_, 1/1024)
ok

PeriodiCode:DEC10$ 0b11.p-10
frac: 3/1024
cont: [0; 341, 3]
digt: 0.0029296875

PeriodiCode:DEC10$ @assert_eq($_, 3/1024)
ok

PeriodiCode:DEC10$ 0x11.p-10
frac: 17/1024
cont: [0; 60, 4, 4]
digt: 0.0166015625

PeriodiCode:DEC10$ @assert_eq($_, 17/1024)
ok

PeriodiCode:DEC10$ 0d11.p-10
frac: 11/1024
cont: [0; 93, 11]
digt: 0.0107421875

PeriodiCode:DEC10$ @assert_eq($_, 11/1024)
ok

PeriodiCode:DEC10$ 0.1r6e1
frac: 5/3
cont: [1; 1, 2]
digt: 1.r6

PeriodiCode:DEC10$ @assert_eq($_, 5/3)
ok

PeriodiCode:DEC10$ 0.1r6xp1
frac: 5/3
cont: [1; 1, 2]
digt: 1.r6

PeriodiCode:DEC10$ @assert_eq($_, 5/3)
ok

PeriodiCode:DEC10$ 0x1e2
frac: 482
cont: [482]
digt: 482

PeriodiCode:DEC10$ @assert_eq($_, 482)
ok

PeriodiCode:DEC10$ 0x1xp2
frac: 256
cont: [256]
digt: 256

PeriodiCode:DEC10$ @assert_eq($_, 256)
ok
```
