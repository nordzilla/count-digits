#![cfg_attr(not(test), no_std)]
#![allow(clippy::zero_prefixed_literal)]
//! A [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) trait to count
//! the digits of integer types in various number bases.
//!
//! Compatible with all primitive integer types and all non-zero integer types.
//!
//! ### Examples
//! ```rust
//! use count_digits::CountDigits;
//! use core::num::NonZeroIsize;
//!
//! assert_eq!(16, 0b1111000000001101.count_bits());
//! assert_eq!(16, 0b1111000000001101.count_digits_radix(2_u32));
//!
//! assert_eq!(06, 0o170015.count_octal_digits());
//! assert_eq!(06, 0o170015.count_digits_radix(8_u32));
//!
//! assert_eq!(05, 61453.count_digits());
//! assert_eq!(05, 61453.count_digits_radix(10_u32));
//!
//! assert_eq!(04, 0xF00D.count_hex_digits());
//! assert_eq!(04, 0xF00D.count_digits_radix(16_u32));
//!
//! assert_eq!(
//!   0xF00D_isize.count_digits(),
//!   NonZeroIsize::new(0xF00D).unwrap().count_digits(),
//! );
//! ```
//!
//! #### Note
//!
//! Be mindful with negative signed integers in non-decimal number bases.
//!
//! Since negative decimal numbers are represented with a negative sign,
//! the decimal digit count of a negative number will be equal to its
//! positive counterpart.
//!
//! ```rust
//! # use count_digits::CountDigits;
//! assert_eq!(
//!     0xF00D_i32.count_digits(),
//!     0xF00D_i32.wrapping_neg().count_digits(),
//! );
//! ````
//!
//! However, a negative number using a different radix will not have the
//! same count of digits as its positive counterpart.
//!
//! ```rust
//! # use count_digits::CountDigits;
//! assert_ne!(
//!     0xBAD_i32.count_bits(),
//!     0xBAD_i32.wrapping_neg().count_bits(),
//! );
//! assert_ne!(
//!     0xBAD_i32.count_octal_digits(),
//!     0xBAD_i32.wrapping_neg().count_octal_digits(),
//! );
//! assert_ne!(
//!     0xBAD_i32.count_hex_digits(),
//!     0xBAD_i32.wrapping_neg().count_hex_digits(),
//! );
//!
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

use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

/// A [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) trait to count
/// the digits of an integer in various number bases.
pub trait CountDigits: Copy + Sized {
    /// The type of integer that should be passed in to the
    /// [count_digits_radix()](CountDigits::count_digits_radix) function.
    type Radix;

    /// Returns the count of bits in an integer starting with the first non-zero bit.
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

    /// Returns the count of octal digits in an integer starting with the first non-zero digit.
    /// ### Examples
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

    /// Returns the count of decimal digits in an integer.
    ///
    /// ###### Note
    ///
    /// Counts digits only: the negative sign for negative numbers is not counted.
    ///
    /// ### Examples
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
    fn count_digits(self) -> u32;

    /// Returns the count of hexadecimal digits in an integer starting with the first non-zero digit.
    /// ### Examples
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

    /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
    /// ### Examples
    /// ```rust
    /// use count_digits::CountDigits;
    ///
    /// for n in 0..1_000_000 {
    ///   assert_eq!(n.count_digits_radix(02_u32), n.count_bits());
    ///   assert_eq!(n.count_digits_radix(08_u32), n.count_octal_digits());
    ///   assert_eq!(n.count_digits_radix(10_u32), n.count_digits());
    ///   assert_eq!(n.count_digits_radix(16_u32), n.count_hex_digits());
    /// }
    /// ```
    fn count_digits_radix(self, radix: Self::Radix) -> u32;
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
            /// Returns the count of bits in an integer starting with the first non-zero bit.
            fn count_bits(self) -> u32 {
                if self.is_negative() {
                    $min_value_bits
                } else {
                    1 + self.abs_diff(0).checked_ilog2().unwrap_or_default()
                }
            }

            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                1 + self.abs_diff(0).checked_ilog10().unwrap_or_default()
            }

            #[inline(always)]
            /// Returns the count of octal digits in an integer starting with the first non-zero digit.
            fn count_octal_digits(self) -> u32 {
                if self.is_negative() {
                    $min_value_octal_digits
                } else {
                    1 + self.abs_diff(0).checked_ilog(8).unwrap_or_default()
                }
            }

            #[inline(always)]
            /// Returns the count of hexadecimal digits in an integer starting with the first non-zero digit.
            fn count_hex_digits(self) -> u32 {
                if self.is_negative() {
                    $min_value_hex_digits
                } else {
                    1 + self.abs_diff(0).checked_ilog(16).unwrap_or_default()
                }
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            fn count_digits_radix(self, radix: Self::Radix) -> u32 {
                match radix {
                    02 => self.count_bits(),
                    08 => self.count_octal_digits(),
                    10 => self.count_digits(),
                    16 => self.count_hex_digits(),
                    __ => {
                        if self.is_negative() {
                            1 + <$primitive_type>::MIN
                                .abs_diff(0)
                                .checked_ilog(radix)
                                .unwrap_or_default()
                        } else {
                            1 + self.abs_diff(0).checked_ilog(radix).unwrap_or_default()
                        }
                    }
                }
            }
        }

        impl CountDigits for $non_zero_type {
            type Radix = $radix_type;

            #[inline(always)]
            /// Returns the count of bits in an integer starting with the first non-zero bit.
            fn count_bits(self) -> u32 {
                if self.is_negative() {
                    $min_value_bits
                } else {
                    1 + self.get().abs_diff(0).ilog2()
                }
            }

            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                1 + self.get().abs_diff(0).ilog10()
            }

            #[inline(always)]
            /// Returns the count of octal digits in an integer starting with the first non-zero digit.
            fn count_octal_digits(self) -> u32 {
                if self.is_negative() {
                    $min_value_octal_digits
                } else {
                    1 + self.get().abs_diff(0).ilog(8)
                }
            }

            #[inline(always)]
            /// Returns the count of hexadecimal digits in an integer starting with the first non-zero digit.
            fn count_hex_digits(self) -> u32 {
                if self.is_negative() {
                    $min_value_hex_digits
                } else {
                    1 + self.get().abs_diff(0).ilog(16)
                }
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            fn count_digits_radix(self, radix: Self::Radix) -> u32 {
                match radix {
                    02 => self.count_bits(),
                    08 => self.count_octal_digits(),
                    10 => self.count_digits(),
                    16 => self.count_hex_digits(),
                    __ => {
                        if self.is_negative() {
                            1 + <$non_zero_type>::MIN.get().abs_diff(0).ilog(radix)
                        } else {
                            1 + self.get().abs_diff(0).ilog(radix)
                        }
                    }
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
            /// Returns the count of bits in an integer starting with the first non-zero bit.
            fn count_bits(self) -> u32 {
                1 + self.checked_ilog2().unwrap_or_default()
            }

            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                1 + self.checked_ilog10().unwrap_or_default()
            }

            #[inline(always)]
            /// Returns the count of octal digits in an integer starting with the first non-zero digit.
            fn count_octal_digits(self) -> u32 {
                1 + self.checked_ilog(8).unwrap_or_default()
            }

            #[inline(always)]
            /// Returns the count of hexadecimal digits in an integer starting with the first non-zero digit.
            fn count_hex_digits(self) -> u32 {
                1 + self.checked_ilog(16).unwrap_or_default()
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            fn count_digits_radix(self, radix: Self::Radix) -> u32 {
                match radix {
                    02 => self.count_bits(),
                    08 => self.count_octal_digits(),
                    10 => self.count_digits(),
                    16 => self.count_hex_digits(),
                    __ => 1 + self.checked_ilog(radix).unwrap_or_default(),
                }
            }
        }

        impl CountDigits for $non_zero_type {
            type Radix = $primitive_type;

            #[inline(always)]
            /// Returns the count of bits in an integer starting with the first non-zero bit.
            fn count_bits(self) -> u32 {
                1 + self.ilog2()
            }

            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                1 + self.ilog10()
            }

            #[inline(always)]
            /// Returns the count of octal digits in an integer starting with the first non-zero digit.
            fn count_octal_digits(self) -> u32 {
                1 + self.get().ilog(8)
            }

            #[inline(always)]
            /// Returns the count of hexadecimal digits in an integer starting with the first non-zero digit.
            fn count_hex_digits(self) -> u32 {
                1 + self.get().ilog(16)
            }

            #[inline(always)]
            /// Returns the count of digits in an integer as interpreted with the given [radix](https://en.wikipedia.org/wiki/Radix).
            fn count_digits_radix(self, radix: Self::Radix) -> u32 {
                match radix {
                    02 => self.count_bits(),
                    08 => self.count_octal_digits(),
                    10 => self.count_digits(),
                    16 => self.count_hex_digits(),
                    _ => 1 + self.get().ilog(radix),
                }
            }
        }
    };
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
            string.strip_prefix('-').unwrap_or(&string).len() as u32
        }};
    }

    macro_rules! hex_string_count {
        ($n:expr) => {
            format!("{:x}", $n).len() as u32
        };
    }

    macro_rules! assert_min_and_max {
        ($type:ty) => {
            assert_eq!(
                <$type>::MIN.count_bits(),
                binary_string_count!(<$type>::MIN)
            );
            assert_eq!(
                <$type>::MIN.count_digits_radix(2),
                binary_string_count!(<$type>::MIN)
            );

            assert_eq!(
                <$type>::MAX.count_bits(),
                binary_string_count!(<$type>::MAX)
            );
            assert_eq!(
                <$type>::MAX.count_digits_radix(2),
                binary_string_count!(<$type>::MAX)
            );

            assert_eq!(
                <$type>::MIN.count_octal_digits(),
                octal_string_count!(<$type>::MIN),
            );
            assert_eq!(
                <$type>::MIN.count_digits_radix(8),
                octal_string_count!(<$type>::MIN),
            );

            assert_eq!(
                <$type>::MAX.count_octal_digits(),
                octal_string_count!(<$type>::MAX),
            );
            assert_eq!(
                <$type>::MAX.count_digits_radix(8),
                octal_string_count!(<$type>::MAX),
            );

            assert_eq!(
                <$type>::MIN.count_digits(),
                decimal_string_count!(<$type>::MIN)
            );
            assert_eq!(
                <$type>::MIN.count_digits_radix(10),
                decimal_string_count!(<$type>::MIN),
            );

            assert_eq!(
                <$type>::MAX.count_digits(),
                decimal_string_count!(<$type>::MAX)
            );
            assert_eq!(
                <$type>::MAX.count_digits_radix(10),
                decimal_string_count!(<$type>::MAX),
            );

            assert_eq!(
                <$type>::MIN.count_hex_digits(),
                hex_string_count!(<$type>::MIN),
            );
            assert_eq!(
                <$type>::MIN.count_digits_radix(16),
                hex_string_count!(<$type>::MIN),
            );

            assert_eq!(
                <$type>::MAX.count_hex_digits(),
                hex_string_count!(<$type>::MAX),
            );
            assert_eq!(
                <$type>::MAX.count_digits_radix(16),
                hex_string_count!(<$type>::MAX),
            );
        };
    }

    macro_rules! assert_representations {
        ($n:expr) => {
            assert_eq!($n.count_bits(), binary_string_count!($n));
            assert_eq!($n.count_digits_radix(2), binary_string_count!($n));

            assert_eq!($n.count_octal_digits(), octal_string_count!($n));
            assert_eq!($n.count_digits_radix(8), octal_string_count!($n));

            assert_eq!($n.count_digits(), decimal_string_count!($n));
            assert_eq!($n.count_digits_radix(10), decimal_string_count!($n));

            assert_eq!($n.count_hex_digits(), hex_string_count!($n));
            assert_eq!($n.count_digits_radix(16), hex_string_count!($n));

            assert!([
                $n.count_digits_radix(2),
                $n.count_digits_radix(3),
                $n.count_digits_radix(4),
                $n.count_digits_radix(5),
                $n.count_digits_radix(6),
                $n.count_digits_radix(7),
                $n.count_digits_radix(8),
                $n.count_digits_radix(9),
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
    }

    macro_rules! min_and_max {
        ($type:ty, $non_zero_type:ty) => {
            paste! {
                #[test]
                fn [<min_and_max_ $type>]() {
                    assert_min_and_max!($type);
                }

                #[test]
                #[allow(non_snake_case)]
                fn [<min_and_max_ $non_zero_type>]() {
                    assert_min_and_max!($type);
                }
            }
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

    macro_rules! iteration {
        (signed, $type:ty, $non_zero_type:ty) => {
            paste! {
                #[test]
                #[allow(overflowing_literals)]
                fn [<iteration_ $type>]() {
                    let max = max_or_upper_bound!($type);
                    let min = min_or_lower_bound!($type);
                    for n in min..=max {
                        assert_representations!(n);
                    }
                }

                #[test]
                #[allow(non_snake_case)]
                #[allow(overflowing_literals)]
                fn [<iteration_ $non_zero_type>]() {
                    let max = max_or_upper_bound!($type);
                    let min = min_or_lower_bound!($type);
                    for n in min..=max {
                        if n == 0 { continue; }
                        let n = $non_zero_type::new(n).unwrap();
                        assert_representations!(n);
                    }
                }
            }
        };
        (unsigned, $type:ty, $non_zero_type:ty) => {
            paste! {
                #[test]
                #[allow(overflowing_literals)]
                fn [<iteration_ $type>]() {
                    let max = max_or_upper_bound!($type);
                    for n in $type::MIN..=max {
                        assert_representations!(n);
                    }
                }

                #[test]
                #[allow(non_snake_case)]
                #[allow(overflowing_literals)]
                fn [<iteration_ $non_zero_type>]() {
                    let max = max_or_upper_bound!($type);
                    for n in $non_zero_type::MIN.get()..=max {
                        let n = $non_zero_type::new(n).unwrap();
                        assert_representations!(n);
                    }
                }
            }
        };
    }

    macro_rules! add_test {
        ($name:ident, $($args:tt)+) => {
            $name!($($args)*);
        };
    }

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
}
