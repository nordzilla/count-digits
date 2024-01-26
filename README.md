[![Coverage Status](https://codecov.io/gh/nordzilla/count-digits/branch/main/graph/badge.svg)](https://codecov.io/gh/nordzilla/count-digits)

# count-digits

A [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) trait to count
the digits of integer types in various number bases.

Compatible with all primitive integer types and all non-zero integer types.

#### Examples

```rust
use count_digits::CountDigits;

assert_eq!(16, 0b1111000000001101.count_bits());
assert_eq!(16, 0b1111000000001101.count_digits_radix(2_u32));

assert_eq!(06, 0o170015.count_octal_digits());
assert_eq!(06, 0o170015.count_digits_radix(8_u32));

assert_eq!(05, 61453.count_digits());
assert_eq!(05, 61453.count_digits_radix(10_u32));

assert_eq!(04, 0xF00D.count_hex_digits());
assert_eq!(04, 0xF00D.count_digits_radix(16_u32));
```

---

> [!NOTE]  
> The [count_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits)
> and [count_digits_radix(10)](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits_radix)
> functions do not include the negative sign in their counts.

```rust
assert_eq!(5, 12345_i32.wrapping_neg().count_digits());
assert_eq!(5, 12345_i32.wrapping_neg().count_digits_radix(10));
````

---

> [!NOTE]  
> Negative numbers counted in base-10 are counted differently than
> negative numbers counted in other number bases.

Since negative, base-10 numbers are represented with a negative sign,
the digit count of a positive, base-10 number will be equal to the count
of its negated value.

```rust
assert_eq!(
    0xF00D_i32.count_digits(),
    0xF00D_i32.wrapping_neg().count_digits(),
);
````

However, the digit counts of negative numbers represented in other bases reflect the
[twos-complement representation](https://en.wikipedia.org/wiki/Two%27s_complement),
and the digit count of a positive number will _not_ be the same as the count
of its negated value.

```rust
assert_ne!(
    0xBAD_i32.count_bits(),
    0xBAD_i32.wrapping_neg().count_bits(),
);
assert_ne!(
    0xBAD_i32.count_octal_digits(),
    0xBAD_i32.wrapping_neg().count_octal_digits(),
);
assert_ne!(
    0xBAD_i32.count_hex_digits(),
    0xBAD_i32.wrapping_neg().count_hex_digits(),
);

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

These counts are consistent with the representations of Rust's display format.
```rust
assert_eq!(1, format!("{:b}",  1_i8).chars().count());
assert_eq!(1, format!("{:o}",  1_i8).chars().count());
assert_eq!(1, format!("{  }",  1_i8).chars().count());
assert_eq!(1, format!("{:x}",  1_i8).chars().count());

assert_eq!(8, format!("{:b}", -1_i8).chars().count());
assert_eq!(3, format!("{:o}", -1_i8).chars().count());
assert_eq!(1, format!("{  }", -1_i8).strip_prefix('-').unwrap().chars().count());
assert_eq!(2, format!("{:x}", -1_i8).chars().count());
````

License: MIT
