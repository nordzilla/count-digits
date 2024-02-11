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

[CountDigits](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html)
is a [no-std](https://docs.rust-embedded.org/book/intro/no-std.html) trait with functions
to determine the lengths of integers in various number bases.

It is [implemented](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#foreign-impls)
for all primitive integer types and all non-zero integer types.

```rust
pub trait CountDigits: Copy + Sized {
    /// The type of integer that should be used for radix arguments.
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
    /// Panics if the provided radix is invalid.
    fn count_digits_radix(self, radix: Self::Radix) -> usize;

    /// Returns the count of digits in an integer for a given radix.
    /// Returns None if the given radix is invalid.
    fn checked_count_digits_radix(self, radix: Self::Radix) -> Option<usize>;
}
```

### Examples

```rust
use count_digits::CountDigits;

// Base 2
assert_eq!(16, 0b1111000000001101.count_bits());
assert_eq!(16, 0b1111000000001101.count_digits_radix(2_u32));

// Base 8
assert_eq!(06, 0o170015.count_octal_digits());
assert_eq!(06, 0o170015.count_digits_radix(8_u32));

// Base 10
assert_eq!(05, 61453.count_digits());
assert_eq!(05, 61453.count_digits_radix(10_u32));

// Base 16
assert_eq!(04, 0xF00D.count_hex_digits());
assert_eq!(04, 0xF00D.count_digits_radix(16_u32));
```

#### Functions That Return u32

Named functions for which the radix is a power of two return
[u32](https://doc.rust-lang.org/core/primitive.u32.html) for
compatibility with Rust's bitwise functions and constants.

* [count_bits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_bits)
* [count_octal_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_octal_digits)
* [count_hex_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_hex_digits)

```rust
assert_eq!(0b1011___u8.count_bits(),   u8::BITS - 0b1011___u8.leading_zeros());
assert_eq!(0b1011__i32.count_bits(),  i32::BITS - 0b1011__i32.leading_zeros());
assert_eq!(0b1011_u128.count_bits(), u128::BITS - 0b1011_u128.leading_zeros());
```

#### Functions That Return usize

Functions that are not inherently meaningful in a bitwise context return [usize](https://doc.rust-lang.org/core/primitive.usize.html)
for compatibility with Rust's formatting functions and macros.

* [count_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits)
* [count_digits_radix()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits_radix)
* [checked_count_digits_radix()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.checked_count_digits_radix)

```rust
let numbers = [2, 3, 13, 103, 1337];
let max_digits = numbers
    .iter()
    .map(CountDigits::count_digits)
    .max()
    .unwrap();

for n in numbers {
    assert_eq!(4, format!("{n:>max_digits$}").chars().count());
}
```

When formatting binary, octal, or hexadecimal numbers, the
[count_digits_radix(2 | 8 | 16)](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits_radix)
and [checked_count_digits_radix(2 | 8 | 16)](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.checked_count_digits_radix)
functions can be used in place of [count_bits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_bits),
[count_octal_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_octal_digits), and
[count_hex_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_hex_digits)
to retrieve the desired count directly as a [usize](https://doc.rust-lang.org/core/primitive.usize.html).

```rust
let numbers = [0b1, 0b10, 0b101, 0b1011];
let max_bits = numbers
    .iter()
    .map(|n| n.count_digits_radix(2u32))
    .max()
    .unwrap();

for n in numbers {
    assert_eq!(4, format!("{n:>max_bits$}").chars().count());
}
```

#### Invalid Radix Values

Values passed to [count_digits_radix()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits_radix)
and [checked_count_digits_radix()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.checked_count_digits_radix)
must be greater than or equal to 2.

* [count_digits_radix()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits_radix)
will [panic](https://doc.rust-lang.org/stable/core/macro.panic.html) if given an invalid radix.

```rust
for n in 0..100 {
    assert!(std::panic::catch_unwind(|| n.count_digits_radix(0_u32)).is_err());
    assert!(std::panic::catch_unwind(|| n.count_digits_radix(1_u32)).is_err());
}
```

* [checked_count_digits_radix()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.checked_count_digits_radix)
will return [None](https://doc.rust-lang.org/stable/core/option/enum.Option.html#variant.None) if given an invalid radix.

```rust
for n in 0..100 {
    assert!(n.checked_count_digits_radix(0_u32).is_none());
    assert!(n.checked_count_digits_radix(1_u32).is_none());
}
```

#### Negative Numbers

Since negative numbers represented in base 10 are displayed with a negative sign,
the base-10 digit count of a positive number will be equal to the base-10 digit count
of the number's negated value, assuming no wrapping occurs.

```rust
assert_eq!(
    867_5309_i32.count_digits(),
    867_5309_i32.wrapping_neg().count_digits(),
);
```

> [!NOTE]
> The negative sign itself is not included in the count because
> the negative sign is not a digit.

The digit counts of negative numbers represented in other bases reflect the
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
```

This is consistent with Rust's display format.
```rust
// Base 2
assert_eq!(01, format!("{:b}",  1_i32).chars().count());
assert_eq!(32, format!("{:b}", -1_i32).chars().count());

// Base 8
assert_eq!(01, format!("{:o}",  1_i32).chars().count());
assert_eq!(11, format!("{:o}", -1_i32).chars().count());

// Base 10
assert_eq!(01, format!("{  }",  1_i32).chars().count());
assert_eq!(01, format!("{  }", -1_i32).strip_prefix('-').unwrap().chars().count());

// Base 16
assert_eq!(01, format!("{:x}",  1_i32).chars().count());
assert_eq!(08, format!("{:x}", -1_i32).chars().count());
```

### Benchmarks

* [table](https://nordzilla.github.io/count-digits)
* [count_bits()](https://nordzilla.github.io/count-digits/count_bits/report/index.html)
* [count_octal_digits()](https://nordzilla.github.io/count-digits/count_octal_digits/report/index.html)
* [count_digits()](https://nordzilla.github.io/count-digits/count_digits/report/index.html)
* [count_hex_digits()](https://nordzilla.github.io/count-digits/count_hex_digits/report/index.html)

License: MIT
