#![cfg_attr(not(test), no_std)]
#![allow(clippy::zero_prefixed_literal)]
//! [![github]](https://github.com/nordzilla/count-digits)
//! [![crates-io]](https://crates.io/crates/count-digits)
//! [![docs-rs]](https://docs.rs/count-digits)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! [![license]](https://github.com/nordzilla/count-digits/blob/main/LICENSE)
//! [![build]](https://github.com/nordzilla/count-digits/commits/main/)
//! [![codecov]](https://app.codecov.io/gh/nordzilla/count-digits)
//!
//! [license]: https://img.shields.io/github/license/nordzilla/count-digits?style=flat-square&color=009050&label=License
//! [build]: https://img.shields.io/github/actions/workflow/status/nordzilla/count-digits/rust.yml?style=flat-square&logo=github&color=009050&label=Build
//! [codecov]: https://img.shields.io/codecov/c/github/nordzilla/count-digits?style=flat-square&logo=codecov&color=009050&label=Test+Coverage
//!
//! <br>
//!
//! A [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) trait to count
//! the digits of integer types in various number bases.
//!
//! Compatible with all primitive integer types and all non-zero integer types.
//!
//! ```rust
//! pub trait CountDigits: Copy + Sized {
//!     type Radix;
//!
//!     /// Returns the count of bits in an integer.
//!     fn count_bits(self) -> u32;
//!
//!     /// Returns the count of octal digits in an integer.
//!     fn count_octal_digits(self) -> u32;
//!
//!     /// Returns the count of hexadecimal digits in an integer.
//!     fn count_hex_digits(self) -> u32;
//!
//!     /// Returns the count of decimal digits in an integer.
//!     fn count_digits(self) -> usize;
//!
//!     /// Returns the count of digits in an integer for a given radix.
//!     /// Panics if the provided radix is invalid.
//!     fn count_digits_radix(self, radix: Self::Radix) -> usize;
//!
//!     /// Returns the count of digits in an integer for a given radix.
//!     /// Returns None if the given radix is invalid.
//!     fn checked_count_digits_radix(self, radix: Self::Radix) -> Option<usize>;
//! }
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use count_digits::CountDigits;
//! # use core::num::NonZeroIsize;
//!
//! assert_eq!(16___u32, 0b1111000000001101.count_bits());
//! assert_eq!(16_usize, 0b1111000000001101.count_digits_radix(2_u32));
//!
//! assert_eq!(06___u32, 0o170015.count_octal_digits());
//! assert_eq!(06_usize, 0o170015.count_digits_radix(8_u32));
//!
//! assert_eq!(04___u32, 0xF00D.count_hex_digits());
//! assert_eq!(04_usize, 0xF00D.count_digits_radix(16_u32));
//!
//! assert_eq!(05_usize, 61453.count_digits());
//! assert_eq!(05_usize, 61453.count_digits_radix(10_u32));
//!
//! assert!(1.checked_count_digits_radix(0_u32).is_none());
//! assert!(1.checked_count_digits_radix(1_u32).is_none());
//! assert!(1.checked_count_digits_radix(2_u32).is_some());
//! ```
//!
//! The named functions for which the radix is a power of two (
//! [count_bits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_bits),
//! [count_octal_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_octal_digits), and
//! [count_hex_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_hex_digits)
//! ) return a [u32](https://doc.rust-lang.org/core/primitive.u32.html) for compatibility with Rust's bit-shifting functions
//! and constants, which all use [u32](https://doc.rust-lang.org/core/primitive.u32.html) for arguments and return types.
//!
//! ```rust
//! # use count_digits::CountDigits;
//! assert_eq!(0b1011___u8.count_bits(),   u8::BITS - 0b1011___u8.leading_zeros());
//! assert_eq!(0b1011__i32.count_bits(),  i32::BITS - 0b1011__i32.leading_zeros());
//! assert_eq!(0b1011_u128.count_bits(), u128::BITS - 0b1011_u128.leading_zeros());
//! ```
//!
//! The base-10 function [count_digits()](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits)
//! returns [usize](https://doc.rust-lang.org/core/primitive.usize.html) for compatibility with Rust's formatting macros.
//!
//! ```rust
//! # use count_digits::CountDigits;
//! let max_digits = [1, 2, 15, 105]
//!     .iter()
//!     .map(CountDigits::count_digits)
//!     .max()
//!     .unwrap();
//!
//! for n in [1, 2, 15, 105] {
//!     assert_eq!(3, format!("{n:0>pad$}", pad = max_digits).len());
//! }
//! ```
//!
//! In the case of formatting binary, octal, or hex numbers, the
//! [count_digits_radix(2 | 8 | 16)](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.count_digits_radix)
//! and [checked_count_digits_radix(2 | 8 | 16)](https://docs.rs/count-digits/latest/count_digits/trait.CountDigits.html#tymethod.checked_count_digits_radix)
//! functions can be used to retrieve the desired count directly as a [usize](https://doc.rust-lang.org/core/primitive.usize.html), or
//! the value can simply be cast using the [as](https://doc.rust-lang.org/std/keyword.as.html) keyword.
//!
//! ```rust
//! # use count_digits::CountDigits;
//! let max_bits = [0b1, 0b10, 0b101, 0b1011]
//!     .iter()
//!     .map(|n| n.count_digits_radix(2u32))
//!     .max()
//!     .unwrap();
//!
//! for n in [0b1, 0b10, 0b101, 0b1011] {
//!     assert_eq!(4, format!("{n:0>pad$}", pad = max_bits).len());
//! }
//! ```
//!
//! ---
//!
//! <div class="warning">
//! Functions calls that count digits in base 10 (
//! <a href="trait.CountDigits.html#tymethod.count_digits" title="method count_digits::CountDigits::count_digits">
//!    count_digits()
//! </a>,
//! <a href="trait.CountDigits.html#tymethod.count_digits_radix" title="method count_digits::CountDigits::count_digits_radix">
//!    count_digits_radix(10)
//! </a>, and
//! <a href="trait.CountDigits.html#tymethod.checked_count_digits_radix" title="method count_digits::CountDigits::checked_count_digits_radix">
//!    checked_count_digits_radix(10)
//! </a>
//! ) do not include the negative sign in their counts because the negative sign is not a digit.
//! </div>
//!
//! ```rust
//! # use count_digits::CountDigits;
//! assert_eq!(5, 12345_i32.wrapping_neg().count_digits());
//! assert_eq!(5, 12345_i32.wrapping_neg().count_digits_radix(10));
//! ````
//!
//! ---
//!
//! <div class="warning">
//!     Negative numbers counted in base-10 are counted differently than
//!     negative numbers counted in other number bases.
//! </div>
//!
//! Since negative numbers represented in base-10 are displayed with a negative sign,
//! the base-10 digit count of a positive number will be equal to the base-10 digit count
//! of the number's negated value, assuming no wrapping occurred.
//!
//! ```rust
//! # use count_digits::CountDigits;
//! assert_eq!(
//!     867_5309_i32.count_digits(),
//!     867_5309_i32.wrapping_neg().count_digits(),
//! );
//! ````
//!
//! However, the digit counts of negative numbers represented in other bases reflect the
//! [twos-complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation,
//! and the digit count of a positive number will _not_ be the same as the count
//! of its negated value.
//!
//! ```rust
//! # use count_digits::CountDigits;
//! for radix in 2..=16 {
//!     match radix {
//!         10 => assert_eq!(
//!             0xF00D_i32.count_digits_radix(radix),
//!             0xF00D_i32.wrapping_neg().count_digits_radix(radix),
//!         ),
//!         _ => assert_ne!(
//!             0xBAD_i32.count_digits_radix(radix),
//!             0xBAD_i32.wrapping_neg().count_digits_radix(radix),
//!         ),
//!     }
//! }
//! ````
//!
//! This behavior is consistent with Rust's display format.
//! ```rust
//! # use count_digits::CountDigits;
//! assert_eq!(1, format!("{:b}",  1_i8).chars().count());
//! assert_eq!(1, format!("{:o}",  1_i8).chars().count());
//! assert_eq!(1, format!("{  }",  1_i8).chars().count());
//! assert_eq!(1, format!("{:x}",  1_i8).chars().count());
//!
//! assert_eq!(8, format!("{:b}", -1_i8).chars().count());
//! assert_eq!(3, format!("{:o}", -1_i8).chars().count());
//! assert_eq!(1, format!("{  }", -1_i8).strip_prefix('-').unwrap().chars().count());
//! assert_eq!(2, format!("{:x}", -1_i8).chars().count());
//! ```
//!
//! ## Benchmarks
//!
//! * [table](https://nordzilla.github.io/count-digits)
//! * [count_bits()](https://nordzilla.github.io/count-digits/count_bits/report/index.html)
//! * [count_octal_digits()](https://nordzilla.github.io/count-digits/count_octal_digits/report/index.html)
//! * [count_digits()](https://nordzilla.github.io/count-digits/count_digits/report/index.html)
//! * [count_hex_digits()](https://nordzilla.github.io/count-digits/count_hex_digits/report/index.html)

use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

/// A [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) trait to count
/// the digits of integer types in various number bases.
pub trait CountDigits: Copy + Sized {
    /// The type of integer that should be passed in to the
    /// [count_digits_radix()](CountDigits::count_digits_radix) function.
    ///
    /// This type is always the corresponding unsigned primitive type for an
    /// integer of a given size.
    ///
    /// For example, [u8] is the [Radix](CountDigits::Radix) type for [i8], [u8], [NonZeroI8], and [NonZeroU8].
    type Radix;

    /// Returns the count of bits in an integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use count_digits::CountDigits;
    /// # use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
    /// # use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
    ///
    /// assert_eq!(008, i8::MIN.count_bits());
    /// assert_eq!(007, i8::MAX.count_bits());
    /// assert_eq!(008, NonZeroI8::MIN.count_bits());
    /// assert_eq!(007, NonZeroI8::MAX.count_bits());
    ///
    /// assert_eq!(001, u8::MIN.count_bits());
    /// assert_eq!(008, u8::MAX.count_bits());
    /// assert_eq!(001, NonZeroU8::MIN.count_bits());
    /// assert_eq!(008, NonZeroU8::MAX.count_bits());
    ///
    /// assert_eq!(016, i16::MIN.count_bits());
    /// assert_eq!(015, i16::MAX.count_bits());
    /// assert_eq!(016, NonZeroI16::MIN.count_bits());
    /// assert_eq!(015, NonZeroI16::MAX.count_bits());
    ///
    /// assert_eq!(001, u16::MIN.count_bits());
    /// assert_eq!(016, u16::MAX.count_bits());
    /// assert_eq!(001, NonZeroU16::MIN.count_bits());
    /// assert_eq!(016, NonZeroU16::MAX.count_bits());
    ///
    /// assert_eq!(032, i32::MIN.count_bits());
    /// assert_eq!(031, i32::MAX.count_bits());
    /// assert_eq!(032, NonZeroI32::MIN.count_bits());
    /// assert_eq!(031, NonZeroI32::MAX.count_bits());
    ///
    /// assert_eq!(001, u32::MIN.count_bits());
    /// assert_eq!(032, u32::MAX.count_bits());
    /// assert_eq!(001, NonZeroU32::MIN.count_bits());
    /// assert_eq!(032, NonZeroU32::MAX.count_bits());
    ///
    /// assert_eq!(064, i64::MIN.count_bits());
    /// assert_eq!(063, i64::MAX.count_bits());
    /// assert_eq!(064, NonZeroI64::MIN.count_bits());
    /// assert_eq!(063, NonZeroI64::MAX.count_bits());
    ///
    /// assert_eq!(001, u64::MIN.count_bits());
    /// assert_eq!(064, u64::MAX.count_bits());
    /// assert_eq!(001, NonZeroU64::MIN.count_bits());
    /// assert_eq!(064, NonZeroU64::MAX.count_bits());
    ///
    /// assert_eq!(128, i128::MIN.count_bits());
    /// assert_eq!(127, i128::MAX.count_bits());
    /// assert_eq!(128, NonZeroI128::MIN.count_bits());
    /// assert_eq!(127, NonZeroI128::MAX.count_bits());
    ///
    /// assert_eq!(001, u128::MIN.count_bits());
    /// assert_eq!(128, u128::MAX.count_bits());
    /// assert_eq!(001, NonZeroU128::MIN.count_bits());
    /// assert_eq!(128, NonZeroU128::MAX.count_bits());
    ///
    /// #[cfg(target_pointer_width = "64")] {
    ///   assert_eq!(isize::MIN.count_bits(), i64::MIN.count_bits());
    ///   assert_eq!(isize::MAX.count_bits(), i64::MAX.count_bits());
    ///   assert_eq!(NonZeroIsize::MIN.count_bits(), NonZeroI64::MIN.count_bits());
    ///   assert_eq!(NonZeroIsize::MAX.count_bits(), NonZeroI64::MAX.count_bits());
    ///
    ///   assert_eq!(usize::MIN.count_bits(), u64::MIN.count_bits());
    ///   assert_eq!(usize::MAX.count_bits(), u64::MAX.count_bits());
    ///   assert_eq!(NonZeroUsize::MIN.count_bits(), NonZeroU64::MIN.count_bits());
    ///   assert_eq!(NonZeroUsize::MAX.count_bits(), NonZeroU64::MAX.count_bits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "32")] {
    ///   assert_eq!(isize::MIN.count_bits(), i32::MIN.count_bits());
    ///   assert_eq!(isize::MAX.count_bits(), i32::MAX.count_bits());
    ///   assert_eq!(NonZeroIsize::MIN.count_bits(), NonZeroI32::MIN.count_bits());
    ///   assert_eq!(NonZeroIsize::MAX.count_bits(), NonZeroI32::MAX.count_bits());
    ///
    ///   assert_eq!(usize::MIN.count_bits(), u32::MIN.count_bits());
    ///   assert_eq!(usize::MAX.count_bits(), u32::MAX.count_bits());
    ///   assert_eq!(NonZeroUsize::MIN.count_bits(), NonZeroU32::MIN.count_bits());
    ///   assert_eq!(NonZeroUsize::MAX.count_bits(), NonZeroU32::MAX.count_bits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "16")] {
    ///   assert_eq!(isize::MIN.count_bits(), i16::MIN.count_bits());
    ///   assert_eq!(isize::MAX.count_bits(), i16::MAX.count_bits());
    ///   assert_eq!(NonZeroIsize::MIN.count_bits(), NonZeroI16::MIN.count_bits());
    ///   assert_eq!(NonZeroIsize::MAX.count_bits(), NonZeroI16::MAX.count_bits());
    ///
    ///   assert_eq!(usize::MIN.count_bits(), u16::MIN.count_bits());
    ///   assert_eq!(usize::MAX.count_bits(), u16::MAX.count_bits());
    ///   assert_eq!(NonZeroUsize::MIN.count_bits(), NonZeroU16::MIN.count_bits());
    ///   assert_eq!(NonZeroUsize::MAX.count_bits(), NonZeroU16::MAX.count_bits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "8")] {
    ///   assert_eq!(isize::MIN.count_bits(), i8::MIN.count_bits());
    ///   assert_eq!(isize::MAX.count_bits(), i8::MAX.count_bits());
    ///   assert_eq!(NonZeroIsize::MIN.count_bits(), NonZeroI8::MIN.count_bits());
    ///   assert_eq!(NonZeroIsize::MAX.count_bits(), NonZeroI8::MAX.count_bits());
    ///
    ///   assert_eq!(usize::MIN.count_bits(), u8::MIN.count_bits());
    ///   assert_eq!(usize::MAX.count_bits(), u8::MAX.count_bits());
    ///   assert_eq!(NonZeroUsize::MIN.count_bits(), NonZeroU8::MIN.count_bits());
    ///   assert_eq!(NonZeroUsize::MAX.count_bits(), NonZeroU8::MAX.count_bits());
    /// }
    /// ```
    fn count_bits(self) -> u32;

    /// Returns the count of octal digits in an integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use count_digits::CountDigits;
    /// # use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
    /// # use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
    ///
    /// assert_eq!(03, i8::MIN.count_octal_digits());
    /// assert_eq!(03, i8::MAX.count_octal_digits());
    /// assert_eq!(03, NonZeroI8::MIN.count_octal_digits());
    /// assert_eq!(03, NonZeroI8::MAX.count_octal_digits());
    ///
    /// assert_eq!(01, u8::MIN.count_octal_digits());
    /// assert_eq!(03, u8::MAX.count_octal_digits());
    /// assert_eq!(01, NonZeroU8::MIN.count_octal_digits());
    /// assert_eq!(03, NonZeroU8::MAX.count_octal_digits());
    ///
    /// assert_eq!(06, i16::MIN.count_octal_digits());
    /// assert_eq!(05, i16::MAX.count_octal_digits());
    /// assert_eq!(06, NonZeroI16::MIN.count_octal_digits());
    /// assert_eq!(05, NonZeroI16::MAX.count_octal_digits());
    ///
    /// assert_eq!(01, u16::MIN.count_octal_digits());
    /// assert_eq!(06, u16::MAX.count_octal_digits());
    /// assert_eq!(01, NonZeroU16::MIN.count_octal_digits());
    /// assert_eq!(06, NonZeroU16::MAX.count_octal_digits());
    ///
    /// assert_eq!(11, i32::MIN.count_octal_digits());
    /// assert_eq!(11, i32::MAX.count_octal_digits());
    /// assert_eq!(11, NonZeroI32::MIN.count_octal_digits());
    /// assert_eq!(11, NonZeroI32::MAX.count_octal_digits());
    ///
    /// assert_eq!(01, u32::MIN.count_octal_digits());
    /// assert_eq!(11, u32::MAX.count_octal_digits());
    /// assert_eq!(01, NonZeroU32::MIN.count_octal_digits());
    /// assert_eq!(11, NonZeroU32::MAX.count_octal_digits());
    ///
    /// assert_eq!(22, i64::MIN.count_octal_digits());
    /// assert_eq!(21, i64::MAX.count_octal_digits());
    /// assert_eq!(22, NonZeroI64::MIN.count_octal_digits());
    /// assert_eq!(21, NonZeroI64::MAX.count_octal_digits());
    ///
    /// assert_eq!(01, u64::MIN.count_octal_digits());
    /// assert_eq!(22, u64::MAX.count_octal_digits());
    /// assert_eq!(01, NonZeroU64::MIN.count_octal_digits());
    /// assert_eq!(22, NonZeroU64::MAX.count_octal_digits());
    ///
    /// assert_eq!(43, i128::MIN.count_octal_digits());
    /// assert_eq!(43, i128::MAX.count_octal_digits());
    /// assert_eq!(43, NonZeroI128::MIN.count_octal_digits());
    /// assert_eq!(43, NonZeroI128::MAX.count_octal_digits());
    ///
    /// assert_eq!(01, u128::MIN.count_octal_digits());
    /// assert_eq!(43, u128::MAX.count_octal_digits());
    /// assert_eq!(01, NonZeroU128::MIN.count_octal_digits());
    /// assert_eq!(43, NonZeroU128::MAX.count_octal_digits());
    ///
    /// #[cfg(target_pointer_width = "64")] {
    ///   assert_eq!(isize::MIN.count_octal_digits(), i64::MIN.count_octal_digits());
    ///   assert_eq!(isize::MAX.count_octal_digits(), i64::MAX.count_octal_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_octal_digits(), NonZeroI64::MIN.count_octal_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_octal_digits(), NonZeroI64::MAX.count_octal_digits());
    ///
    ///   assert_eq!(usize::MIN.count_octal_digits(), u64::MIN.count_octal_digits());
    ///   assert_eq!(usize::MAX.count_octal_digits(), u64::MAX.count_octal_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_octal_digits(), NonZeroU64::MIN.count_octal_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_octal_digits(), NonZeroU64::MAX.count_octal_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "32")] {
    ///   assert_eq!(isize::MIN.count_octal_digits(), i32::MIN.count_octal_digits());
    ///   assert_eq!(isize::MAX.count_octal_digits(), i32::MAX.count_octal_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_octal_digits(), NonZeroI32::MIN.count_octal_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_octal_digits(), NonZeroI32::MAX.count_octal_digits());
    ///
    ///   assert_eq!(usize::MIN.count_octal_digits(), u32::MIN.count_octal_digits());
    ///   assert_eq!(usize::MAX.count_octal_digits(), u32::MAX.count_octal_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_octal_digits(), NonZeroU32::MIN.count_octal_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_octal_digits(), NonZeroU32::MAX.count_octal_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "16")] {
    ///   assert_eq!(isize::MIN.count_octal_digits(), i16::MIN.count_octal_digits());
    ///   assert_eq!(isize::MAX.count_octal_digits(), i16::MAX.count_octal_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_octal_digits(), NonZeroI16::MIN.count_octal_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_octal_digits(), NonZeroI16::MAX.count_octal_digits());
    ///
    ///   assert_eq!(usize::MIN.count_octal_digits(), u16::MIN.count_octal_digits());
    ///   assert_eq!(usize::MAX.count_octal_digits(), u16::MAX.count_octal_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_octal_digits(), NonZeroU16::MIN.count_octal_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_octal_digits(), NonZeroU16::MAX.count_octal_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "8")] {
    ///   assert_eq!(isize::MIN.count_octal_digits(), i8::MIN.count_octal_digits());
    ///   assert_eq!(isize::MAX.count_octal_digits(), i8::MAX.count_octal_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_octal_digits(), NonZeroI8::MIN.count_octal_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_octal_digits(), NonZeroI8::MAX.count_octal_digits());
    ///
    ///   assert_eq!(usize::MIN.count_octal_digits(), u8::MIN.count_octal_digits());
    ///   assert_eq!(usize::MAX.count_octal_digits(), u8::MAX.count_octal_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_octal_digits(), NonZeroU8::MIN.count_octal_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_octal_digits(), NonZeroU8::MAX.count_octal_digits());
    /// }
    /// ```
    fn count_octal_digits(self) -> u32;

    /// Returns the count of hexadecimal digits in an integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use count_digits::CountDigits;
    /// # use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
    /// # use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
    ///
    /// assert_eq!(02, i8::MIN.count_hex_digits());
    /// assert_eq!(02, i8::MAX.count_hex_digits());
    /// assert_eq!(02, NonZeroI8::MIN.count_hex_digits());
    /// assert_eq!(02, NonZeroI8::MAX.count_hex_digits());
    ///
    /// assert_eq!(01, u8::MIN.count_hex_digits());
    /// assert_eq!(02, u8::MAX.count_hex_digits());
    /// assert_eq!(01, NonZeroU8::MIN.count_hex_digits());
    /// assert_eq!(02, NonZeroU8::MAX.count_hex_digits());
    ///
    /// assert_eq!(04, i16::MIN.count_hex_digits());
    /// assert_eq!(04, i16::MAX.count_hex_digits());
    /// assert_eq!(04, NonZeroI16::MIN.count_hex_digits());
    /// assert_eq!(04, NonZeroI16::MAX.count_hex_digits());
    ///
    /// assert_eq!(01, u16::MIN.count_hex_digits());
    /// assert_eq!(04, u16::MAX.count_hex_digits());
    /// assert_eq!(01, NonZeroU16::MIN.count_hex_digits());
    /// assert_eq!(04, NonZeroU16::MAX.count_hex_digits());
    ///
    /// assert_eq!(08, i32::MIN.count_hex_digits());
    /// assert_eq!(08, i32::MAX.count_hex_digits());
    /// assert_eq!(08, NonZeroI32::MIN.count_hex_digits());
    /// assert_eq!(08, NonZeroI32::MAX.count_hex_digits());
    ///
    /// assert_eq!(01, u32::MIN.count_hex_digits());
    /// assert_eq!(08, u32::MAX.count_hex_digits());
    /// assert_eq!(01, NonZeroU32::MIN.count_hex_digits());
    /// assert_eq!(08, NonZeroU32::MAX.count_hex_digits());
    ///
    /// assert_eq!(16, i64::MIN.count_hex_digits());
    /// assert_eq!(16, i64::MAX.count_hex_digits());
    /// assert_eq!(16, NonZeroI64::MIN.count_hex_digits());
    /// assert_eq!(16, NonZeroI64::MAX.count_hex_digits());
    ///
    /// assert_eq!(01, u64::MIN.count_hex_digits());
    /// assert_eq!(16, u64::MAX.count_hex_digits());
    /// assert_eq!(01, NonZeroU64::MIN.count_hex_digits());
    /// assert_eq!(16, NonZeroU64::MAX.count_hex_digits());
    ///
    /// assert_eq!(32, i128::MIN.count_hex_digits());
    /// assert_eq!(32, i128::MAX.count_hex_digits());
    /// assert_eq!(32, NonZeroI128::MIN.count_hex_digits());
    /// assert_eq!(32, NonZeroI128::MAX.count_hex_digits());
    ///
    /// assert_eq!(01, u128::MIN.count_hex_digits());
    /// assert_eq!(32, u128::MAX.count_hex_digits());
    /// assert_eq!(01, NonZeroU128::MIN.count_hex_digits());
    /// assert_eq!(32, NonZeroU128::MAX.count_hex_digits());
    ///
    /// #[cfg(target_pointer_width = "64")] {
    ///   assert_eq!(isize::MIN.count_hex_digits(), i64::MIN.count_hex_digits());
    ///   assert_eq!(isize::MAX.count_hex_digits(), i64::MAX.count_hex_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_hex_digits(), NonZeroI64::MIN.count_hex_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_hex_digits(), NonZeroI64::MAX.count_hex_digits());
    ///
    ///   assert_eq!(usize::MIN.count_hex_digits(), u64::MIN.count_hex_digits());
    ///   assert_eq!(usize::MAX.count_hex_digits(), u64::MAX.count_hex_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_hex_digits(), NonZeroU64::MIN.count_hex_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_hex_digits(), NonZeroU64::MAX.count_hex_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "32")] {
    ///   assert_eq!(isize::MIN.count_hex_digits(), i32::MIN.count_hex_digits());
    ///   assert_eq!(isize::MAX.count_hex_digits(), i32::MAX.count_hex_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_hex_digits(), NonZeroI32::MIN.count_hex_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_hex_digits(), NonZeroI32::MAX.count_hex_digits());
    ///
    ///   assert_eq!(usize::MIN.count_hex_digits(), u32::MIN.count_hex_digits());
    ///   assert_eq!(usize::MAX.count_hex_digits(), u32::MAX.count_hex_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_hex_digits(), NonZeroU32::MIN.count_hex_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_hex_digits(), NonZeroU32::MAX.count_hex_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "16")] {
    ///   assert_eq!(isize::MIN.count_hex_digits(), i16::MIN.count_hex_digits());
    ///   assert_eq!(isize::MAX.count_hex_digits(), i16::MAX.count_hex_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_hex_digits(), NonZeroI16::MIN.count_hex_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_hex_digits(), NonZeroI16::MAX.count_hex_digits());
    ///
    ///   assert_eq!(usize::MIN.count_hex_digits(), u16::MIN.count_hex_digits());
    ///   assert_eq!(usize::MAX.count_hex_digits(), u16::MAX.count_hex_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_hex_digits(), NonZeroU16::MIN.count_hex_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_hex_digits(), NonZeroU16::MAX.count_hex_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "8")] {
    ///   assert_eq!(isize::MIN.count_hex_digits(), i8::MIN.count_hex_digits());
    ///   assert_eq!(isize::MAX.count_hex_digits(), i8::MAX.count_hex_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_hex_digits(), NonZeroI8::MIN.count_hex_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_hex_digits(), NonZeroI8::MAX.count_hex_digits());
    ///
    ///   assert_eq!(usize::MIN.count_hex_digits(), u8::MIN.count_hex_digits());
    ///   assert_eq!(usize::MAX.count_hex_digits(), u8::MAX.count_hex_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_hex_digits(), NonZeroU8::MIN.count_hex_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_hex_digits(), NonZeroU8::MAX.count_hex_digits());
    /// }
    /// ```
    fn count_hex_digits(self) -> u32;

    /// Returns the count of decimal digits in an integer.
    ///
    /// <div class="warning">
    /// Does not count the negative sign when counting negative, signed integers because the negative sign is not a digit.
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use count_digits::CountDigits;
    /// # use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
    /// # use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
    ///
    /// assert_eq!(03, i8::MIN.count_digits());
    /// assert_eq!(03, i8::MAX.count_digits());
    /// assert_eq!(03, NonZeroI8::MIN.count_digits());
    /// assert_eq!(03, NonZeroI8::MAX.count_digits());
    ///
    /// assert_eq!(01, u8::MIN.count_digits());
    /// assert_eq!(03, u8::MAX.count_digits());
    /// assert_eq!(01, NonZeroU8::MIN.count_digits());
    /// assert_eq!(03, NonZeroU8::MAX.count_digits());
    ///
    /// assert_eq!(05, i16::MIN.count_digits());
    /// assert_eq!(05, i16::MAX.count_digits());
    /// assert_eq!(05, NonZeroI16::MIN.count_digits());
    /// assert_eq!(05, NonZeroI16::MAX.count_digits());
    ///
    /// assert_eq!(01, u16::MIN.count_digits());
    /// assert_eq!(05, u16::MAX.count_digits());
    /// assert_eq!(01, NonZeroU16::MIN.count_digits());
    /// assert_eq!(05, NonZeroU16::MAX.count_digits());
    ///
    /// assert_eq!(10, i32::MIN.count_digits());
    /// assert_eq!(10, i32::MAX.count_digits());
    /// assert_eq!(10, NonZeroI32::MIN.count_digits());
    /// assert_eq!(10, NonZeroI32::MAX.count_digits());
    ///
    /// assert_eq!(01, u32::MIN.count_digits());
    /// assert_eq!(10, u32::MAX.count_digits());
    /// assert_eq!(01, NonZeroU32::MIN.count_digits());
    /// assert_eq!(10, NonZeroU32::MAX.count_digits());
    ///
    /// assert_eq!(19, i64::MIN.count_digits());
    /// assert_eq!(19, i64::MAX.count_digits());
    /// assert_eq!(19, NonZeroI64::MIN.count_digits());
    /// assert_eq!(19, NonZeroI64::MAX.count_digits());
    ///
    /// assert_eq!(01, u64::MIN.count_digits());
    /// assert_eq!(20, u64::MAX.count_digits());
    /// assert_eq!(01, NonZeroU64::MIN.count_digits());
    /// assert_eq!(20, NonZeroU64::MAX.count_digits());
    ///
    /// assert_eq!(39, i128::MIN.count_digits());
    /// assert_eq!(39, i128::MAX.count_digits());
    /// assert_eq!(39, NonZeroI128::MIN.count_digits());
    /// assert_eq!(39, NonZeroI128::MAX.count_digits());
    ///
    /// assert_eq!(01, u128::MIN.count_digits());
    /// assert_eq!(39, u128::MAX.count_digits());
    /// assert_eq!(01, NonZeroU128::MIN.count_digits());
    /// assert_eq!(39, NonZeroU128::MAX.count_digits());
    ///
    /// #[cfg(target_pointer_width = "64")] {
    ///   assert_eq!(isize::MIN.count_digits(), i64::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i64::MAX.count_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI64::MIN.count_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI64::MAX.count_digits());
    ///
    ///   assert_eq!(usize::MIN.count_digits(), u64::MIN.count_digits());
    ///   assert_eq!(usize::MAX.count_digits(), u64::MAX.count_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU64::MIN.count_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU64::MAX.count_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "32")] {
    ///   assert_eq!(isize::MIN.count_digits(), i32::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i32::MAX.count_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI32::MIN.count_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI32::MAX.count_digits());
    ///
    ///   assert_eq!(usize::MIN.count_digits(), u32::MIN.count_digits());
    ///   assert_eq!(usize::MAX.count_digits(), u32::MAX.count_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU32::MIN.count_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU32::MAX.count_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "16")] {
    ///   assert_eq!(isize::MIN.count_digits(), i16::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i16::MAX.count_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI16::MIN.count_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI16::MAX.count_digits());
    ///
    ///   assert_eq!(usize::MIN.count_digits(), u16::MIN.count_digits());
    ///   assert_eq!(usize::MAX.count_digits(), u16::MAX.count_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU16::MIN.count_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU16::MAX.count_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "8")] {
    ///   assert_eq!(isize::MIN.count_digits(), i8::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i8::MAX.count_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI8::MIN.count_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI8::MAX.count_digits());
    ///
    ///   assert_eq!(usize::MIN.count_digits(), u8::MIN.count_digits());
    ///   assert_eq!(usize::MAX.count_digits(), u8::MAX.count_digits());
    ///   assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU8::MIN.count_digits());
    ///   assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU8::MAX.count_digits());
    /// }
    /// ```
    fn count_digits(self) -> usize;

    /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
    ///
    /// [Panics](panic) if the provided radix is 0 or 1.
    ///
    /// See [checked_count_digits_radix()](CountDigits::checked_count_digits_radix) for a non-panicking version of this function.
    ///
    /// <div class="warning">
    /// For radix 10, does not count the negative sign when counting negative, signed integers because the negative sign is not a digit.
    ///
    /// For all other radix values, counts digits according to the
    /// <a href="https://en.wikipedia.org/wiki/Two%27s_complement">twos-complement</a> representation.
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use count_digits::CountDigits;
    ///
    /// for n in 0..100 {
    ///   assert!(std::panic::catch_unwind(|| n.count_digits_radix(0_u32)).is_err());
    ///   assert!(std::panic::catch_unwind(|| n.count_digits_radix(1_u32)).is_err());
    ///
    ///   assert_eq!(n.count_digits_radix(02_u32) as u32, n.count_bits());
    ///   assert_eq!(n.count_digits_radix(08_u32) as u32, n.count_octal_digits());
    ///   assert_eq!(n.count_digits_radix(16_u32) as u32, n.count_hex_digits());
    ///   assert_eq!(n.count_digits_radix(10_u32), n.count_digits());
    /// }
    /// ```
    fn count_digits_radix(self, radix: Self::Radix) -> usize;

    /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
    ///
    /// Returns [None] if the provided radix is 0 or 1.
    ///
    /// See [count_digits_radix()](CountDigits::count_digits_radix) for a panicking version of this function.
    ///
    /// <div class="warning">
    /// For radix 10, does not count the negative sign when counting negative, signed integers because the negative sign is not a digit.
    ///
    /// For all other radix values, counts digits according to the
    /// <a href="https://en.wikipedia.org/wiki/Two%27s_complement">twos-complement</a> representation.
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use count_digits::CountDigits;
    ///
    /// for n in 0..100 {
    ///   assert_eq!(n.checked_count_digits_radix(00_u32), None);
    ///   assert_eq!(n.checked_count_digits_radix(01_u32), None);
    ///   assert_eq!(n.checked_count_digits_radix(02_u32).unwrap() as u32, n.count_bits());
    ///   assert_eq!(n.checked_count_digits_radix(08_u32).unwrap() as u32, n.count_octal_digits());
    ///   assert_eq!(n.checked_count_digits_radix(16_u32).unwrap() as u32, n.count_hex_digits());
    ///   assert_eq!(n.checked_count_digits_radix(10_u32).unwrap(), n.count_digits());
    /// }
    /// ```
    fn checked_count_digits_radix(self, radix: Self::Radix) -> Option<usize>;
}

macro_rules! impl_count_digits {
    (
        primitive_type = $primitive_type:ty,
        non_zero_type = $non_zero_type:ty,
        radix_type = $radix_type:ty,
        min_value_bits = $min_value_bits:expr,
        min_value_octal_digits = $min_value_octal_digits:expr,
        min_value_hex_digits = $min_value_hex_digits:expr $(,)?
    ) => {
        impl CountDigits for $primitive_type {
            type Radix = $radix_type;

            #[inline(always)]
            /// Returns the count of bits in an integer.
            fn count_bits(self) -> u32 {
                if self.is_negative() {
                    $min_value_bits
                } else {
                    1 + self.checked_ilog2().unwrap_or_default()
                }
            }

            #[inline(always)]
            /// Returns the count of octal digits in an integer.
            fn count_octal_digits(self) -> u32 {
                if self.is_negative() {
                    $min_value_octal_digits
                } else {
                    1 + self.checked_ilog2().unwrap_or_default() / 3
                }
            }

            #[inline(always)]
            /// Returns the count of hexadecimal digits in an integer.
            fn count_hex_digits(self) -> u32 {
                if self.is_negative() {
                    $min_value_hex_digits
                } else {
                    1 + self.checked_ilog2().unwrap_or_default() / 4
                }
            }

            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> usize {
                1 + self.abs_diff(0).checked_ilog10().unwrap_or_default() as usize
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            ///
            /// [Panics](panic) if the provided radix is 0 or 1.
            ///
            /// See [checked_count_digits_radix()](CountDigits::checked_count_digits_radix) for a non-panicking version of this function.
            fn count_digits_radix(self, radix: Self::Radix) -> usize {
                match radix {
                    0 | 1 => panic!("base of integer logarithm must be at least 2"),
                    02 => self.count_bits() as usize,
                    08 => self.count_octal_digits() as usize,
                    10 => self.count_digits(),
                    16 => self.count_hex_digits() as usize,
                    __ => {
                        if self.is_negative() {
                            1 + <$primitive_type>::MIN.abs_diff(0).ilog(radix) as usize
                        } else {
                            1 + self.abs_diff(0).checked_ilog(radix).unwrap_or_default() as usize
                        }
                    }
                }
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            ///
            /// Returns [None] if the provided radix is 0 or 1.
            ///
            /// See [count_digits_radix()](CountDigits::count_digits_radix) for a panicking version of this function.
            fn checked_count_digits_radix(self, radix: Self::Radix) -> Option<usize> {
                match radix {
                    0 | 1 => None,
                    radix => Some(self.count_digits_radix(radix)),
                }
            }
        }

        impl CountDigits for $non_zero_type {
            type Radix = $radix_type;

            #[inline(always)]
            /// Returns the count of bits in an integer.
            fn count_bits(self) -> u32 {
                if self.is_negative() {
                    $min_value_bits
                } else {
                    1 + self.get().ilog2()
                }
            }

            #[inline(always)]
            /// Returns the count of octal digits in an integer.
            fn count_octal_digits(self) -> u32 {
                if self.is_negative() {
                    $min_value_octal_digits
                } else {
                    1 + self.get().ilog2() / 3
                }
            }

            #[inline(always)]
            /// Returns the count of hexadecimal digits in an integer.
            fn count_hex_digits(self) -> u32 {
                if self.is_negative() {
                    $min_value_hex_digits
                } else {
                    1 + self.get().ilog2() / 4
                }
            }

            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> usize {
                1 + self.get().abs_diff(0).ilog10() as usize
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            ///
            /// [Panics](panic) if the provided radix is 0 or 1.
            ///
            /// See [checked_count_digits_radix()](CountDigits::checked_count_digits_radix) for a non-panicking version of this function.
            fn count_digits_radix(self, radix: Self::Radix) -> usize {
                match radix {
                    0 | 1 => panic!("base of integer logarithm must be at least 2"),
                    02 => self.count_bits() as usize,
                    08 => self.count_octal_digits() as usize,
                    10 => self.count_digits(),
                    16 => self.count_hex_digits() as usize,
                    __ => {
                        if self.is_negative() {
                            1 + <$primitive_type>::MIN.abs_diff(0).ilog(radix) as usize
                        } else {
                            1 + self.get().abs_diff(0).ilog(radix) as usize
                        }
                    }
                }
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            ///
            /// Returns [None] if the provided radix is 0 or 1.
            ///
            /// See [count_digits_radix()](CountDigits::count_digits_radix) for a panicking version of this function.
            fn checked_count_digits_radix(self, radix: Self::Radix) -> Option<usize> {
                match radix {
                    0 | 1 => None,
                    radix => Some(self.count_digits_radix(radix)),
                }
            }
        }
    };
    (
        primitive_type = $primitive_type:ty,
        non_zero_type = $non_zero_type:ty,
    ) => {
        impl CountDigits for $primitive_type {
            type Radix = $primitive_type;

            #[inline(always)]
            /// Returns the count of bits in an integer.
            fn count_bits(self) -> u32 {
                1 + self.checked_ilog2().unwrap_or_default()
            }

            #[inline(always)]
            /// Returns the count of octal digits in an integer.
            fn count_octal_digits(self) -> u32 {
                1 + self.checked_ilog2().unwrap_or_default() / 3
            }

            #[inline(always)]
            /// Returns the count of hexadecimal digits in an integer.
            fn count_hex_digits(self) -> u32 {
                1 + self.checked_ilog2().unwrap_or_default() / 4
            }

            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> usize {
                1 + self.checked_ilog10().unwrap_or_default() as usize
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            ///
            /// [Panics](panic) if the provided radix is 0 or 1.
            ///
            /// See [checked_count_digits_radix()](CountDigits::checked_count_digits_radix) for a non-panicking version of this function.
            fn count_digits_radix(self, radix: Self::Radix) -> usize {
                match radix {
                    0 | 1 => panic!("base of integer logarithm must be at least 2"),
                    02 => self.count_bits() as usize,
                    08 => self.count_octal_digits() as usize,
                    10 => self.count_digits(),
                    16 => self.count_hex_digits() as usize,
                    __ => 1 + self.checked_ilog(radix).unwrap_or_default() as usize,
                }
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            ///
            /// Returns [None] if the provided radix is 0 or 1.
            ///
            /// See [count_digits_radix()](CountDigits::count_digits_radix) for a panicking version of this function.
            fn checked_count_digits_radix(self, radix: Self::Radix) -> Option<usize> {
                match radix {
                    0 | 1 => None,
                    radix => Some(self.count_digits_radix(radix)),
                }
            }
        }

        impl CountDigits for $non_zero_type {
            type Radix = $primitive_type;

            #[inline(always)]
            /// Returns the count of bits in an integer.
            fn count_bits(self) -> u32 {
                1 + self.ilog2()
            }

            #[inline(always)]
            /// Returns the count of octal digits in an integer.
            fn count_octal_digits(self) -> u32 {
                1 + self.get().ilog2() / 3
            }

            #[inline(always)]
            /// Returns the count of hexadecimal digits in an integer.
            fn count_hex_digits(self) -> u32 {
                1 + self.get().ilog2() / 4
            }

            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> usize {
                1 + self.ilog10() as usize
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            ///
            /// [Panics](panic) if the provided radix is 0 or 1.
            ///
            /// See [checked_count_digits_radix()](CountDigits::checked_count_digits_radix) for a non-panicking version of this function.
            fn count_digits_radix(self, radix: Self::Radix) -> usize {
                match radix {
                    0 | 1 => panic!("base of integer logarithm must be at least 2"),
                    02 => self.count_bits() as usize,
                    08 => self.count_octal_digits() as usize,
                    10 => self.count_digits(),
                    16 => self.count_hex_digits() as usize,
                    _ => 1 + self.get().ilog(radix) as usize,
                }
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            ///
            /// Returns [None] if the provided radix is 0 or 1.
            ///
            /// See [count_digits_radix()](CountDigits::count_digits_radix) for a panicking version of this function.
            fn checked_count_digits_radix(self, radix: Self::Radix) -> Option<usize> {
                match radix {
                    0 | 1 => None,
                    radix => Some(self.count_digits_radix(radix)),
                }
            }
        }
    };
}

impl<T: CountDigits> CountDigits for &T {
    type Radix = <T as CountDigits>::Radix;

    #[inline(always)]
    /// Calls [count_bits()][CountDigits::count_bits] on the inner value.
    fn count_bits(self) -> u32 {
        (*self).count_bits()
    }

    #[inline(always)]
    /// Calls [count_octal_digits()][CountDigits::count_octal_digits] on the inner value.
    fn count_octal_digits(self) -> u32 {
        (*self).count_octal_digits()
    }

    #[inline(always)]
    /// Calls [count_digits()][CountDigits::count_digits] on the inner value.
    fn count_digits(self) -> usize {
        (*self).count_digits()
    }

    #[inline(always)]
    /// Calls [count_hex_digits()][CountDigits::count_hex_digits] on the inner value.
    fn count_hex_digits(self) -> u32 {
        (*self).count_hex_digits()
    }

    #[inline(always)]
    /// Calls [count_digits_radix()][CountDigits::count_digits_radix] on the inner value.
    fn count_digits_radix(self, radix: Self::Radix) -> usize {
        (*self).count_digits_radix(radix)
    }

    #[inline(always)]
    /// Calls [checked_count_digits_radix()][CountDigits::checked_count_digits_radix] on the inner value.
    fn checked_count_digits_radix(self, radix: Self::Radix) -> Option<usize> {
        (*self).checked_count_digits_radix(radix)
    }
}

impl_count_digits! {
    primitive_type = i8,
    non_zero_type = NonZeroI8,
    radix_type = u8,
    min_value_bits = 8,
    min_value_octal_digits = 3,
    min_value_hex_digits = 2,
}

impl_count_digits! {
    primitive_type = i16,
    non_zero_type = NonZeroI16,
    radix_type = u16,
    min_value_bits = 16,
    min_value_octal_digits = 6,
    min_value_hex_digits = 4,
}

impl_count_digits! {
    primitive_type = i32,
    non_zero_type = NonZeroI32,
    radix_type = u32,
    min_value_bits = 32,
    min_value_octal_digits = 11,
    min_value_hex_digits = 8,
}

impl_count_digits! {
    primitive_type = i64,
    non_zero_type = NonZeroI64,
    radix_type = u64,
    min_value_bits = 64,
    min_value_octal_digits = 22,
    min_value_hex_digits = 16,
}

impl_count_digits! {
    primitive_type = i128,
    non_zero_type = NonZeroI128,
    radix_type = u128,
    min_value_bits = 128,
    min_value_octal_digits = 43,
    min_value_hex_digits = 32,
}

#[cfg(target_pointer_width = "64")]
impl_count_digits! {
    primitive_type = isize,
    non_zero_type = NonZeroIsize,
    radix_type = usize,
    min_value_bits = 64,
    min_value_octal_digits = 22,
    min_value_hex_digits = 16,
}

#[cfg(target_pointer_width = "32")]
impl_count_digits! {
    primitive_type = isize,
    non_zero_type = NonZeroIsize,
    radix_type = usize,
    min_value_bits = 32,
    min_value_octal_digits = 11,
    min_value_hex_digits = 8,
}

#[cfg(target_pointer_width = "16")]
impl_count_digits! {
    primitive_type = isize,
    non_zero_type = NonZeroIsize,
    radix_type = usize,
    min_value_bits = 16,
    min_value_octal_digits = 6,
    min_value_hex_digits = 4,
}

#[cfg(target_pointer_width = "8")]
impl_count_digits! {
    primitive_type = isize,
    non_zero_type = NonZeroIsize,
    radix_type = usize,
    min_value_bits = 8,
    min_value_octal_digits = 3,
    min_value_hex_digits = 2,
}

impl_count_digits! {
    primitive_type = u8,
    non_zero_type = NonZeroU8,
}

impl_count_digits! {
    primitive_type = u16,
    non_zero_type = NonZeroU16,
}

impl_count_digits! {
    primitive_type = u32,
    non_zero_type = NonZeroU32,
}

impl_count_digits! {
    primitive_type = u64,
    non_zero_type = NonZeroU64,
}

impl_count_digits! {
    primitive_type = u128,
    non_zero_type = NonZeroU128,
}

impl_count_digits! {
    primitive_type = usize,
    non_zero_type = NonZeroUsize,
}

#[cfg(test)]
mod count_digits {
    use super::*;
    use paste::paste;

    macro_rules! binary_string_count {
        ($n:expr) => {
            format!("{:b}", $n).len() as u32
        };
    }

    macro_rules! octal_string_count {
        ($n:expr) => {
            format!("{:o}", $n).len() as u32
        };
    }

    macro_rules! decimal_string_count {
        ($n:expr) => {{
            let string = format!("{}", $n);
            string.strip_prefix('-').unwrap_or(&string).len()
        }};
    }

    macro_rules! hex_string_count {
        ($n:expr) => {
            format!("{:x}", $n).len() as u32
        };
    }

    macro_rules! assert_min {
        ($type:ty, count_bits) => {
            assert_eq!(
                <$type>::MIN.count_bits(),
                binary_string_count!(<$type>::MIN)
            );
            assert_eq!(
                <$type>::MIN.count_digits_radix(2),
                binary_string_count!(<$type>::MIN) as usize,
            );
        };
        ($type:ty, count_octal_digits) => {
            assert_eq!(
                <$type>::MIN.count_octal_digits(),
                octal_string_count!(<$type>::MIN),
            );
            assert_eq!(
                <$type>::MIN.count_digits_radix(8),
                octal_string_count!(<$type>::MIN) as usize,
            );
        };
        ($type:ty, count_digits) => {
            assert_eq!(
                <$type>::MIN.count_digits(),
                decimal_string_count!(<$type>::MIN)
            );
            assert_eq!(
                <$type>::MIN.count_digits_radix(10),
                decimal_string_count!(<$type>::MIN),
            );
        };
        ($type:ty, count_hex_digits) => {
            assert_eq!(
                <$type>::MIN.count_hex_digits(),
                hex_string_count!(<$type>::MIN),
            );
            assert_eq!(
                <$type>::MIN.count_digits_radix(16),
                hex_string_count!(<$type>::MIN) as usize,
            );
        };
    }

    macro_rules! assert_max {
        ($type:ty, count_bits) => {
            assert_eq!(
                <$type>::MAX.count_bits(),
                binary_string_count!(<$type>::MAX)
            );
            assert_eq!(
                <$type>::MAX.count_digits_radix(2),
                binary_string_count!(<$type>::MAX) as usize,
            );
        };
        ($type:ty, count_octal_digits) => {
            assert_eq!(
                <$type>::MAX.count_octal_digits(),
                octal_string_count!(<$type>::MAX),
            );
            assert_eq!(
                <$type>::MAX.count_digits_radix(8),
                octal_string_count!(<$type>::MAX) as usize,
            );
        };
        ($type:ty, count_digits) => {
            assert_eq!(
                <$type>::MAX.count_digits(),
                decimal_string_count!(<$type>::MAX)
            );
            assert_eq!(
                <$type>::MAX.count_digits_radix(10),
                decimal_string_count!(<$type>::MAX),
            );
        };
        ($type:ty, count_hex_digits) => {
            assert_eq!(
                <$type>::MAX.count_hex_digits(),
                hex_string_count!(<$type>::MAX),
            );
            assert_eq!(
                <$type>::MAX.count_digits_radix(16),
                hex_string_count!(<$type>::MAX) as usize,
            );
        };
    }

    macro_rules! assert_representations {
        ($n:expr, count_bits) => {
            assert_eq!($n.count_bits(), binary_string_count!($n));
            assert_eq!($n.count_digits_radix(2), binary_string_count!($n) as usize);
        };
        ($n:expr, count_octal_digits) => {
            assert_eq!($n.count_octal_digits(), octal_string_count!($n));
            assert_eq!($n.count_digits_radix(8), octal_string_count!($n) as usize);
        };
        ($n:expr, count_digits) => {
            assert_eq!($n.count_digits(), decimal_string_count!($n));
            assert_eq!($n.count_digits_radix(10), decimal_string_count!($n));
        };
        ($n:expr, count_hex_digits) => {
            assert_eq!($n.count_hex_digits(), hex_string_count!($n));
            assert_eq!($n.count_digits_radix(16), hex_string_count!($n) as usize);
        };
        ($n:expr, count_digits_radix_ordering) => {
            assert!([
                $n.count_digits_radix(02),
                $n.count_digits_radix(03),
                $n.count_digits_radix(04),
                $n.count_digits_radix(05),
                $n.count_digits_radix(06),
                $n.count_digits_radix(07),
                $n.count_digits_radix(08),
                $n.count_digits_radix(09),
                $n.count_digits_radix(11),
                $n.count_digits_radix(12),
                $n.count_digits_radix(13),
                $n.count_digits_radix(14),
                $n.count_digits_radix(15),
                $n.count_digits_radix(16),
            ]
            .windows(2)
            .all(|window| window[0] >= window[1]));
        };
        ($n:expr, checked_count_digits_radix_ordering) => {
            assert!([
                $n.checked_count_digits_radix(02).unwrap(),
                $n.checked_count_digits_radix(03).unwrap(),
                $n.checked_count_digits_radix(04).unwrap(),
                $n.checked_count_digits_radix(05).unwrap(),
                $n.checked_count_digits_radix(06).unwrap(),
                $n.checked_count_digits_radix(07).unwrap(),
                $n.checked_count_digits_radix(08).unwrap(),
                $n.checked_count_digits_radix(09).unwrap(),
                $n.checked_count_digits_radix(11).unwrap(),
                $n.checked_count_digits_radix(12).unwrap(),
                $n.checked_count_digits_radix(13).unwrap(),
                $n.checked_count_digits_radix(14).unwrap(),
                $n.checked_count_digits_radix(15).unwrap(),
                $n.checked_count_digits_radix(16).unwrap(),
            ]
            .windows(2)
            .all(|window| window[0] >= window[1]));
        };
    }

    macro_rules! lower_bound {
        ($type:ty) => {
            -100_000 as $type
        };
    }

    macro_rules! upper_bound {
        ($type:ty) => {
            100_000 as $type
        };
    }

    macro_rules! min_or_lower_bound {
        (i8) => {
            i8::MIN
        };
        (i16) => {
            i16::MIN
        };
        (isize) => {{
            #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
            const fn min_or_lower_bound() -> isize {
                lower_bound!(isize)
            }
            #[cfg(any(target_pointer_width = "16", target_pointer_width = "8"))]
            const fn min_or_lower_bound() -> isize {
                isize::MIN
            }
            min_or_lower_bound()
        }};
        ($type:ty) => {
            lower_bound!($type)
        };
    }

    macro_rules! max_or_upper_bound {
        (i8) => {
            i8::MAX
        };
        (i16) => {
            i16::MAX
        };
        (u8) => {
            u8::MAX
        };
        (u16) => {
            u16::MAX
        };
        (isize) => {{
            #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
            const fn max_or_upper_bound() -> isize {
                upper_bound!(isize)
            }
            #[cfg(any(target_pointer_width = "16", target_pointer_width = "8"))]
            const fn max_or_upper_bound() -> isize {
                isize::MAX
            }
            max_or_upper_bound()
        }};
        (usize) => {{
            #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
            const fn max_or_upper_bound() -> usize {
                upper_bound!(usize)
            }
            #[cfg(any(target_pointer_width = "16", target_pointer_width = "8"))]
            const fn max_or_upper_bound() -> usize {
                isize::MAX
            }
            max_or_upper_bound()
        }};
        ($type:ty) => {
            upper_bound!($type)
        };
    }

    /// Returns an iterator over the pairs boundaries [n, m] for a given radix
    /// where n is the largest number of its digit-size group and m is the smallest
    /// number of its digit-size group.
    macro_rules! radix_boundaries {
        ($type:ty, $radix:expr) => {
            std::iter::successors(Some($radix as $type), move |n| {
                n.checked_mul($radix as $type)
            })
            .map(|n| [n - 1, n])
        };
    }

    #[test]
    fn helper_radix_boundaries() {
        assert_eq!(
            radix_boundaries!(i8, 2).collect::<Vec<_>>(),
            [[1, 2], [3, 4], [7, 8], [15, 16], [31, 32], [63, 64]],
        );
        assert_eq!(
            radix_boundaries!(i16, 10).collect::<Vec<_>>(),
            [[9, 10], [99, 100], [999, 1000], [9999, 10000]],
        );
        assert_eq!(
            radix_boundaries!(i16, 16).collect::<Vec<_>>(),
            [[0xF, 0x10], [0xFF, 0x100], [0xFFF, 0x1000]],
        );
    }

    /// Returns an iterator of increasing pairs staring with (1, 2).
    fn increasing_pairs() -> impl Iterator<Item = (usize, usize)> {
        std::iter::successors(Some(1usize), move |n| n.checked_add(1))
            .zip(std::iter::successors(Some(2usize), move |n| {
                n.checked_add(1)
            }))
    }

    #[test]
    fn helper_increasing_pairs() {
        assert_eq!(
            increasing_pairs().take(5).collect::<Vec<_>>(),
            [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)],
        );
    }

    macro_rules! min_and_max {
        ($type:ty, $non_zero_type:ty) => {
            min_and_max!($type, $non_zero_type, count_bits);
            min_and_max!($type, $non_zero_type, count_octal_digits);
            min_and_max!($type, $non_zero_type, count_digits);
            min_and_max!($type, $non_zero_type, count_hex_digits);
        };
        ($type:ty, $non_zero_type:ty, $function:ident) => {
            paste! {
                #[test]
                fn [<$type _min_ $function>]() {
                    assert_min!($type, $function);
                }
                #[test]
                fn [<$type _max_ $function>]() {
                    assert_max!($type, $function);
                }

                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _min_ $function>]() {
                    assert_min!($non_zero_type, $function);
                }
                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _max_ $function>]() {
                    assert_max!($non_zero_type, $function);
                }
            }
        };
    }

    macro_rules! iteration {
        ($signage:ident, $type:ty, $non_zero_type:ty) => {
            iteration!($signage, $type, $non_zero_type, count_bits);
            iteration!($signage, $type, $non_zero_type, count_octal_digits);
            iteration!($signage, $type, $non_zero_type, count_digits);
            iteration!($signage, $type, $non_zero_type, count_hex_digits);
            iteration!($signage, $type, $non_zero_type, count_digits_radix_ordering);
            iteration!(
                $signage,
                $type,
                $non_zero_type,
                checked_count_digits_radix_ordering
            );
        };
        (signed, $type:ty, $non_zero_type:ty, $function:ident) => {
            paste! {
                #[test]
                fn [<$type _iteration_ $function>]() {
                    let max = max_or_upper_bound!($type);
                    let min = min_or_lower_bound!($type);
                    for n in min..=max {
                        assert_representations!(n, $function);
                    }
                }

                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _iteration_ $function>]() {
                    let max = max_or_upper_bound!($type);
                    let min = min_or_lower_bound!($type);
                    for n in min..=max {
                        if n == 0 { continue; }
                        let n = $non_zero_type::new(n).unwrap();
                        assert_representations!(n, $function);
                    }
                }
            }
        };
        (unsigned, $type:ty, $non_zero_type:ty, $function:ident) => {
            paste! {
                #[test]
                fn [<$type _iteration_ $function>]() {
                    let max = max_or_upper_bound!($type);
                    for n in $type::MIN..=max {
                        assert_representations!(n, $function);
                    }
                }

                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _iteration_ $function>]() {
                    let max = max_or_upper_bound!($type);
                    for n in $non_zero_type::MIN.get()..=max {
                        let n = $non_zero_type::new(n).unwrap();
                        assert_representations!(n, $function);
                    }
                }
            }
        };
    }

    macro_rules! pass_by_reference {
        ($type:ty, $non_zero_type:ty) => {
            pass_by_reference!($type, $non_zero_type, count_bits);
            pass_by_reference!($type, $non_zero_type, count_octal_digits);
            pass_by_reference!($type, $non_zero_type, count_hex_digits);
            pass_by_reference!($type, $non_zero_type, count_digits);
            pass_by_reference!($type, $non_zero_type, count_digits_radix);
            pass_by_reference!($type, $non_zero_type, checked_count_digits_radix);
        };
        ($type:ty, $non_zero_type:ty, count_digits_radix) => {
            paste! {
                #[test]
                fn [<$type _pass_by_reference_count_digits_radix>]() {
                    for radix in 2..20 {
                        for n in radix_boundaries!($type, radix).flatten() {
                            assert_eq!(CountDigits::count_digits_radix(n, radix), CountDigits::count_digits_radix(&n, radix));
                        }
                    }
                }
                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _pass_by_reference_count_digits_radix>]() {
                    for radix in 2..20 {
                        for n in radix_boundaries!($type, radix).flatten() {
                            let n = $non_zero_type::new(n).unwrap();
                            assert_eq!(CountDigits::count_digits_radix(n, radix), CountDigits::count_digits_radix(&n, radix));
                        }
                    }
                }
            }
        };
        ($type:ty, $non_zero_type:ty, checked_count_digits_radix) => {
            paste! {
                #[test]
                fn [<$type _pass_by_reference_checked_count_digits_radix>]() {
                    for radix in 2..20 {
                        for n in radix_boundaries!($type, radix).flatten() {
                            assert_eq!(CountDigits::checked_count_digits_radix(n, radix), CountDigits::checked_count_digits_radix(&n, radix));
                        }
                    }
                }
                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _pass_by_reference_checked_count_digits_radix>]() {
                    for radix in 2..20 {
                        for n in radix_boundaries!($type, radix).flatten() {
                            let n = $non_zero_type::new(n).unwrap();
                            assert_eq!(CountDigits::checked_count_digits_radix(n, radix), CountDigits::checked_count_digits_radix(&n, radix));
                        }
                    }
                }
            }
        };
        ($type:ty, $non_zero_type:ty, $function:ident) => {
            paste! {
                #[test]
                fn [<$type _pass_by_reference_ $function>]() {
                    for radix in 2..20 {
                        for n in radix_boundaries!($type, radix).flatten() {
                            assert_eq!(CountDigits::$function(n), CountDigits::$function(&n));
                        }
                    }
                }
                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _pass_by_reference_ $function>]() {
                    for radix in 2..20 {
                        for n in radix_boundaries!($type, radix).flatten() {
                            let n = $non_zero_type::new(n).unwrap();
                            assert_eq!(CountDigits::$function(n), CountDigits::$function(&n));
                        }
                    }
                }
            }
        };
    }

    macro_rules! invalid_radix {
        ($type:ty, $non_zero_type:ty) => {
            invalid_radix!(0, $type, $non_zero_type);
            invalid_radix!(1, $type, $non_zero_type);
        };
        ($radix:expr, $type:ty, $non_zero_type:ty) => {
            paste! {
                #[test]
                #[should_panic(expected = "base of integer logarithm must be at least 2")]
                fn [<$type _invalid_radix_ $radix>]() {
                    (1 as $type).count_digits_radix($radix);
                }
                #[test]
                fn [<$type _invalid_radix_ $radix _checked>]() {
                    assert!((1 as $type).checked_count_digits_radix($radix).is_none());
                }


                #[test]
                #[allow(non_snake_case)]
                #[should_panic(expected = "base of integer logarithm must be at least 2")]
                fn [<$non_zero_type _invalid_radix_ $radix>]() {
                    $non_zero_type::new(1).unwrap().count_digits_radix($radix);
                }
                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _invalid_radix_ $radix _checked>]() {
                    assert!($non_zero_type::new(1).unwrap().checked_count_digits_radix($radix).is_none());
                }
            }
        };
    }

    macro_rules! boundaries_for_radix {
        ($type:ty, $non_zero_type:ty) => {
            boundaries_for_radix!(02, $type, $non_zero_type);
            boundaries_for_radix!(03, $type, $non_zero_type);
            boundaries_for_radix!(04, $type, $non_zero_type);
            boundaries_for_radix!(05, $type, $non_zero_type);
            boundaries_for_radix!(06, $type, $non_zero_type);
            boundaries_for_radix!(07, $type, $non_zero_type);
            boundaries_for_radix!(08, $type, $non_zero_type);
            boundaries_for_radix!(09, $type, $non_zero_type);
            boundaries_for_radix!(10, $type, $non_zero_type);
            boundaries_for_radix!(11, $type, $non_zero_type);
            boundaries_for_radix!(12, $type, $non_zero_type);
            boundaries_for_radix!(13, $type, $non_zero_type);
            boundaries_for_radix!(14, $type, $non_zero_type);
            boundaries_for_radix!(15, $type, $non_zero_type);
            boundaries_for_radix!(16, $type, $non_zero_type);
            boundaries_for_radix!(17, $type, $non_zero_type);
            boundaries_for_radix!(18, $type, $non_zero_type);
            boundaries_for_radix!(19, $type, $non_zero_type);
        };
        ($radix:expr, $type:ty, $non_zero_type:ty) => {
            paste! {
                #[test]
                fn [<$type _boundaries_for_radix_ $radix>]() {
                    assert!(
                        radix_boundaries!($type, $radix)
                            .map(|[n, m]| (n.count_digits_radix($radix), m.count_digits_radix($radix)))
                            .zip(increasing_pairs())
                            .all(|((lhs_actual, rhs_actual), (lhs_expected, rhs_expected))| {
                                lhs_expected == lhs_actual && rhs_expected == rhs_actual
                            })
                    )
                }
                #[test]
                fn [<$type _boundaries_for_radix_ $radix _checked>]() {
                    assert!(
                        radix_boundaries!($type, $radix)
                            .map(|[n, m]| (n.checked_count_digits_radix($radix), m.checked_count_digits_radix($radix)))
                            .zip(increasing_pairs())
                            .all(|((lhs_actual, rhs_actual), (lhs_expected, rhs_expected))| {
                                lhs_expected == lhs_actual.unwrap() && rhs_expected == rhs_actual.unwrap()
                            })
                    )
                }
                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _boundaries_for_radix_ $radix>]() {
                    assert!(
                        radix_boundaries!($type, $radix)
                            .map(|[n, m]| (<$non_zero_type>::new(n).unwrap(), <$non_zero_type>::new(m).unwrap()))
                            .map(|(n, m)| (n.count_digits_radix($radix), m.count_digits_radix($radix)))
                            .zip(increasing_pairs())
                            .all(|((lhs_actual, rhs_actual), (lhs_expected, rhs_expected))| {
                                lhs_expected == lhs_actual && rhs_expected == rhs_actual
                            })
                    )
                }
                #[test]
                #[allow(non_snake_case)]
                fn [<$non_zero_type _boundaries_for_radix_ $radix _checked>]() {
                    assert!(
                        radix_boundaries!($type, $radix)
                            .map(|[n, m]| (<$non_zero_type>::new(n).unwrap(), <$non_zero_type>::new(m).unwrap()))
                            .map(|(n, m)| (n.checked_count_digits_radix($radix), m.checked_count_digits_radix($radix)))
                            .zip(increasing_pairs())
                            .all(|((lhs_actual, rhs_actual), (lhs_expected, rhs_expected))| {
                                lhs_expected == lhs_actual.unwrap() && rhs_expected == rhs_actual.unwrap()
                            })
                    )
                }
            }
        };
    }

    macro_rules! add_test {
        ($name:ident, $($args:tt)+) => {
            $name!($($args)*);
        };
    }

    add_test!(boundaries_for_radix, i8, NonZeroI8);
    add_test!(boundaries_for_radix, i16, NonZeroI16);
    add_test!(boundaries_for_radix, i32, NonZeroI32);
    add_test!(boundaries_for_radix, i64, NonZeroI64);
    add_test!(boundaries_for_radix, i128, NonZeroI128);
    add_test!(boundaries_for_radix, isize, NonZeroIsize);

    add_test!(boundaries_for_radix, u8, NonZeroU8);
    add_test!(boundaries_for_radix, u16, NonZeroU16);
    add_test!(boundaries_for_radix, u32, NonZeroU32);
    add_test!(boundaries_for_radix, u64, NonZeroU64);
    add_test!(boundaries_for_radix, u128, NonZeroU128);
    add_test!(boundaries_for_radix, usize, NonZeroUsize);

    add_test!(invalid_radix, i8, NonZeroI8);
    add_test!(invalid_radix, i16, NonZeroI16);
    add_test!(invalid_radix, i32, NonZeroI32);
    add_test!(invalid_radix, i64, NonZeroI64);
    add_test!(invalid_radix, isize, NonZeroIsize);

    add_test!(invalid_radix, u8, NonZeroU8);
    add_test!(invalid_radix, u16, NonZeroU16);
    add_test!(invalid_radix, u32, NonZeroU32);
    add_test!(invalid_radix, u64, NonZeroU64);
    add_test!(invalid_radix, usize, NonZeroUsize);

    add_test!(iteration, signed, i8, NonZeroI8);
    add_test!(iteration, signed, i16, NonZeroI16);
    add_test!(iteration, signed, i32, NonZeroI32);
    add_test!(iteration, signed, i64, NonZeroI64);
    add_test!(iteration, signed, i128, NonZeroI128);
    add_test!(iteration, signed, isize, NonZeroIsize);

    add_test!(iteration, unsigned, u8, NonZeroU8);
    add_test!(iteration, unsigned, u16, NonZeroU16);
    add_test!(iteration, unsigned, u32, NonZeroU32);
    add_test!(iteration, unsigned, u64, NonZeroU64);
    add_test!(iteration, unsigned, u128, NonZeroU128);
    add_test!(iteration, unsigned, usize, NonZeroUsize);

    add_test!(min_and_max, i8, NonZeroI8);
    add_test!(min_and_max, i16, NonZeroI16);
    add_test!(min_and_max, i32, NonZeroI32);
    add_test!(min_and_max, i64, NonZeroI64);
    add_test!(min_and_max, i128, NonZeroI128);
    add_test!(min_and_max, isize, NonZeroIsize);

    add_test!(min_and_max, u8, NonZeroU8);
    add_test!(min_and_max, u16, NonZeroU16);
    add_test!(min_and_max, u32, NonZeroU32);
    add_test!(min_and_max, u64, NonZeroU64);
    add_test!(min_and_max, u128, NonZeroU128);
    add_test!(min_and_max, usize, NonZeroUsize);

    add_test!(pass_by_reference, i8, NonZeroI8);
    add_test!(pass_by_reference, i16, NonZeroI16);
    add_test!(pass_by_reference, i32, NonZeroI32);
    add_test!(pass_by_reference, i64, NonZeroI64);
    add_test!(pass_by_reference, i128, NonZeroI128);
    add_test!(pass_by_reference, isize, NonZeroIsize);
    add_test!(pass_by_reference, u8, NonZeroU8);
    add_test!(pass_by_reference, u16, NonZeroU16);
    add_test!(pass_by_reference, u32, NonZeroU32);
    add_test!(pass_by_reference, u64, NonZeroU64);
    add_test!(pass_by_reference, u128, NonZeroU128);
    add_test!(pass_by_reference, usize, NonZeroUsize);
}
