# count-digits

[![github]](https://github.com/nordzilla/count-digits)
[![crates-io]](https://crates.io/crates/count-digits)
[![docs-rs]](https://docs.rs/count-digits)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

[![license]](https://github.com/nordzilla/count-digits/blob/main/LICENSE)
[![build]](https://github.com/nordzilla/count-digits/commits/main/)
[![codecov]](https://app.codecov.io/gh/nordzilla/count-digits)

[license]: https://img.shields.io/github/license/nordzilla/count-digits?style=flat-square&color=009050&label=License
[build]: https://img.shields.io/github/actions/workflow/status/nordzilla/count-digits/rust.yml?style=flat-square&logo=github&color=009050&label=Build
[codecov]: https://img.shields.io/codecov/c/github/nordzilla/count-digits?style=flat-square&logo=codecov&color=009050&label=Test+Coverage

<br>

A [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) trait to count
the digits of integer types in various number bases.

Compatible with all primitive integer types and all non-zero integer types.

```rust
pub trait CountDigits: Copy + Sized {
    type Radix;

    /// Returns the count of bits in an integer.
    fn count_bits(self) -> u32;

    /// Returns the count of octal digits in an integer.
    fn count_octal_digits(self) -> u32;

    /// Returns the count of hexadecimal digits in an integer.
    fn count_hex_digits(self) -> u32;

    /// Returns the count of decimal digits in an integer.
    fn count_digits(self) -> usize;

    /// Returns the count of digits in an integer for a given radix.
    fn count_digits_radix(self, radix: Self::Radix) -> usize;
}
```

### Examples

```rust
use count_digits::CountDigits;

assert_eq!(16___u32, 0b1111000000001101.count_bits());
assert_eq!(16_usize, 0b1111000000001101.count_digits_radix(2_u32));

assert_eq!(06___u32, 0o170015.count_octal_digits());
assert_eq!(06_usize, 0o170015.count_digits_radix(8_u32));

assert_eq!(04___u32, 0xF00D.count_hex_digits());
assert_eq!(04_usize, 0xF00D.count_digits_radix(16_u32));

assert_eq!(05_usize, 61453.count_digits());
assert_eq!(05_usize, 61453.count_digits_radix(10_u32));
```

The named functions for which the radix is a power of two (
[count_bits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_bits),
[count_octal_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_octal_digits), and
[count_hex_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_hex_digits)
) return a [u32](https://doc.rust-lang.org/core/primitive.u32.html) for compatibility with Rust's bit-shifting functions
and constants, which all use [u32](https://doc.rust-lang.org/core/primitive.u32.html) for arguments and return types.

```rust
assert_eq!(0b1011___u8.count_bits(),   u8::BITS - 0b1011___u8.leading_zeros());
assert_eq!(0b1011__i32.count_bits(),  i32::BITS - 0b1011__i32.leading_zeros());
assert_eq!(0b1011_u128.count_bits(), u128::BITS - 0b1011_u128.leading_zeros());
```

The base-10 [count_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits)
function returns [usize](https://doc.rust-lang.org/core/primitive.usize.html) for compatibility with Rust's formatting macros.

```rust
let max_digits = [1, 2, 15, 105]
    .iter()
    .map(CountDigits::count_digits)
    .max()
    .unwrap();

for n in [1, 2, 15, 105] {
    assert_eq!(3, format!("{n:0>pad$}", pad = max_digits).len());
}
```

In the case of formatting binary, octal, or hex numbers, the
[count_digits_radix(2 | 8 | 16)](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits_radix)
function can be used to retrieve the desired count directly as a [usize](https://doc.rust-lang.org/core/primitive.usize.html).

```rust
let max_bits = [0b1, 0b10, 0b101, 0b1011]
    .iter()
    .map(|n| n.count_digits_radix(2u32))
    .max()
    .unwrap();

for n in [0b1, 0b10, 0b101, 0b1011] {
    assert_eq!(4, format!("{n:0>pad$}", pad = max_bits).len());
}
```

---

> [!NOTE]
> The base-10 functions 
> [count_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits)
> and [count_digits_radix(10)](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits_radix)
> do not include the negative sign in their counts.

```rust
assert_eq!(5, 12345_i32.wrapping_neg().count_digits());
assert_eq!(5, 12345_i32.wrapping_neg().count_digits_radix(10));
````

---

> [!NOTE]
> Negative numbers counted in base-10 are counted differently than
> negative numbers counted in other number bases.

Since negative numbers represented in base-10 are displayed with a negative sign,
the base-10 digit count of a positive number will be equal to the base-10 digit count
of the number's negated value, assuming no wrapping occurred.

```rust
assert_eq!(
    867_5309_i32.count_digits(),
    867_5309_i32.wrapping_neg().count_digits(),
);
````

However, the digit counts of negative numbers represented in other bases reflect the
[twos-complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation,
and the digit count of a positive number will _not_ be the same as the count
of its negated value.

```rust
for radix in 2..=16 {
    match radix {
        10 => assert_eq!(
            0xF00D_i32.count_digits_radix(radix),
            0xF00D_i32.wrapping_neg().count_digits_radix(radix),
        ),
        _ => assert_ne!(
            0xBAD_i32.count_digits_radix(radix),
            0xBAD_i32.wrapping_neg().count_digits_radix(radix),
        ),
    }
}
````

This behavior is consistent with Rust's display format.
```rust
assert_eq!(1, format!("{:b}",  1_i8).chars().count());
assert_eq!(1, format!("{:o}",  1_i8).chars().count());
assert_eq!(1, format!("{  }",  1_i8).chars().count());
assert_eq!(1, format!("{:x}",  1_i8).chars().count());

assert_eq!(8, format!("{:b}", -1_i8).chars().count());
assert_eq!(3, format!("{:o}", -1_i8).chars().count());
assert_eq!(1, format!("{  }", -1_i8).strip_prefix('-').unwrap().chars().count());
assert_eq!(2, format!("{:x}", -1_i8).chars().count());
```

### Benchmarks

* [table](https://nordzilla.github.io/count-digits)
* [count_bits()](https://nordzilla.github.io/count-digits/count_bits/report/index.html)
* [count_octal_digits()](https://nordzilla.github.io/count-digits/count_octal_digits/report/index.html)
* [count_digits()](https://nordzilla.github.io/count-digits/count_digits/report/index.html)
* [count_hex_digits()](https://nordzilla.github.io/count-digits/count_hex_digits/report/index.html)

License: MIT
