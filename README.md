# count-digits

## CountDigits

A [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) trait to count
the digits of an integer in various number bases.

Compatible with all primitive integer types and all non-zero integer types.

##### Examples
```rust
use count_digits::CountDigits;
use core::num::NonZeroIsize;

assert_eq!(16, 0b1111000000001101.count_bits());
assert_eq!(16, 0b1111000000001101.count_digits_radix(2_u32));

assert_eq!(06, 0o170015.count_octal_digits());
assert_eq!(06, 0o170015.count_digits_radix(8_u32));

assert_eq!(05, 61453.count_digits());
assert_eq!(05, 61453.count_digits_radix(10_u32));

assert_eq!(04, 0xF00D.count_hex_digits());
assert_eq!(04, 0xF00D.count_digits_radix(16_u32));

assert_eq!(
  0xF00D_isize.count_digits(),
  NonZeroIsize::new(0xF00D).unwrap().count_digits(),
);
```

###### Note

Be mindful with negative signed integers in non-decimal number bases.

Since negative decimal numbers are represented with a negative sign,
the decimal digit count of a negative number will be equal to its
positive counterpart.

```rust
assert_eq!(
    0xF00D_i32.count_digits(),
    0xF00D_i32.wrapping_neg().count_digits(),
);
````

However, a negative number using a different radix will not have the
same count of digits as its positive counterpart.

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

License: MIT
