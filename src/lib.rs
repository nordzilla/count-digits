#![no_std]
//! # CountDigits
//!
//! A trait to count the decimal digits of an integer.
//!
//! ### Examples
//! ```rust
//! use count_digits::CountDigits;
//! # use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
//!
//! assert_eq!(03, i8::MIN.count_digits());
//! assert_eq!(03, i8::MAX.count_digits());
//! assert_eq!(03, NonZeroI8::MIN.count_digits());
//! assert_eq!(03, NonZeroI8::MAX.count_digits());
//!
//! assert_eq!(01, u8::MIN.count_digits());
//! assert_eq!(03, u8::MAX.count_digits());
//!
//! assert_eq!(05, i16::MIN.count_digits());
//! assert_eq!(05, i16::MAX.count_digits());
//! assert_eq!(05, NonZeroI16::MIN.count_digits());
//! assert_eq!(05, NonZeroI16::MAX.count_digits());
//!
//! assert_eq!(01, u16::MIN.count_digits());
//! assert_eq!(05, u16::MAX.count_digits());
//!
//! assert_eq!(10, i32::MIN.count_digits());
//! assert_eq!(10, i32::MAX.count_digits());
//! assert_eq!(10, NonZeroI32::MIN.count_digits());
//! assert_eq!(10, NonZeroI32::MAX.count_digits());
//!
//! assert_eq!(01, u32::MIN.count_digits());
//! assert_eq!(10, u32::MAX.count_digits());
//!
//! assert_eq!(19, i64::MIN.count_digits());
//! assert_eq!(19, i64::MAX.count_digits());
//! assert_eq!(19, NonZeroI64::MIN.count_digits());
//! assert_eq!(19, NonZeroI64::MAX.count_digits());
//!
//! assert_eq!(01, u64::MIN.count_digits());
//! assert_eq!(20, u64::MAX.count_digits());
//!
//! assert_eq!(39, i128::MIN.count_digits());
//! assert_eq!(39, i128::MAX.count_digits());
//! assert_eq!(39, NonZeroI128::MIN.count_digits());
//! assert_eq!(39, NonZeroI128::MAX.count_digits());
//!
//! assert_eq!(01, u128::MIN.count_digits());
//! assert_eq!(39, u128::MAX.count_digits());
//!
//! #[cfg(target_pointer_width = "64")] {
//!   assert_eq!(isize::MIN.count_digits(), i64::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i64::MAX.count_digits());
//!   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI64::MIN.count_digits());
//!   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI64::MAX.count_digits());
//!
//!   assert_eq!(usize::MIN.count_digits(), u64::MIN.count_digits());
//!   assert_eq!(usize::MAX.count_digits(), u64::MAX.count_digits());
//! }
//!
//! #[cfg(target_pointer_width = "32")] {
//!   assert_eq!(isize::MIN.count_digits(), i32::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i32::MAX.count_digits());
//!   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI32::MIN.count_digits());
//!   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI32::MAX.count_digits());
//!
//!   assert_eq!(usize::MIN.count_digits(), u32::MIN.count_digits());
//!   assert_eq!(usize::MAX.count_digits(), u32::MAX.count_digits());
//! }
//!
//! #[cfg(target_pointer_width = "16")] {
//!   assert_eq!(isize::MIN.count_digits(), i16::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i16::MAX.count_digits());
//!   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI16::MIN.count_digits());
//!   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI16::MAX.count_digits());
//!
//!   assert_eq!(usize::MIN.count_digits(), u16::MIN.count_digits());
//!   assert_eq!(usize::MAX.count_digits(), u16::MAX.count_digits());
//! }
//!
//! #[cfg(target_pointer_width = "8")] {
//!   assert_eq!(isize::MIN.count_digits(), i8::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i8::MAX.count_digits());
//!   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI8::MIN.count_digits());
//!   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI8::MAX.count_digits());
//!
//!   assert_eq!(usize::MIN.count_digits(), u8::MIN.count_digits());
//!   assert_eq!(usize::MAX.count_digits(), u8::MAX.count_digits());
//! }
//! ```

use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};

/// Count the decimal digits of an integer.
pub trait CountDigits: Copy + Sized {
    /// Returns the count of decimal digits in an integer.
    /// ### Examples
    /// ```rust
    /// use count_digits::CountDigits;
    /// # use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
    ///
    /// assert_eq!(03, i8::MIN.count_digits());
    /// assert_eq!(03, i8::MAX.count_digits());
    /// assert_eq!(03, NonZeroI8::MIN.count_digits());
    /// assert_eq!(03, NonZeroI8::MAX.count_digits());
    ///
    /// assert_eq!(01, u8::MIN.count_digits());
    /// assert_eq!(03, u8::MAX.count_digits());
    ///
    /// assert_eq!(05, i16::MIN.count_digits());
    /// assert_eq!(05, i16::MAX.count_digits());
    /// assert_eq!(05, NonZeroI16::MIN.count_digits());
    /// assert_eq!(05, NonZeroI16::MAX.count_digits());
    ///
    /// assert_eq!(01, u16::MIN.count_digits());
    /// assert_eq!(05, u16::MAX.count_digits());
    ///
    /// assert_eq!(10, i32::MIN.count_digits());
    /// assert_eq!(10, i32::MAX.count_digits());
    /// assert_eq!(10, NonZeroI32::MIN.count_digits());
    /// assert_eq!(10, NonZeroI32::MAX.count_digits());
    ///
    /// assert_eq!(01, u32::MIN.count_digits());
    /// assert_eq!(10, u32::MAX.count_digits());
    ///
    /// assert_eq!(19, i64::MIN.count_digits());
    /// assert_eq!(19, i64::MAX.count_digits());
    /// assert_eq!(19, NonZeroI64::MIN.count_digits());
    /// assert_eq!(19, NonZeroI64::MAX.count_digits());
    ///
    /// assert_eq!(01, u64::MIN.count_digits());
    /// assert_eq!(20, u64::MAX.count_digits());
    ///
    /// assert_eq!(39, i128::MIN.count_digits());
    /// assert_eq!(39, i128::MAX.count_digits());
    /// assert_eq!(39, NonZeroI128::MIN.count_digits());
    /// assert_eq!(39, NonZeroI128::MAX.count_digits());
    ///
    /// assert_eq!(01, u128::MIN.count_digits());
    /// assert_eq!(39, u128::MAX.count_digits());
    ///
    /// #[cfg(target_pointer_width = "64")] {
    ///   assert_eq!(isize::MIN.count_digits(), i64::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i64::MAX.count_digits());
    ///   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI64::MIN.count_digits());
    ///   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI64::MAX.count_digits());
    ///
    ///   assert_eq!(usize::MIN.count_digits(), u64::MIN.count_digits());
    ///   assert_eq!(usize::MAX.count_digits(), u64::MAX.count_digits());
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
    /// }
    /// ```
    fn count_digits(self) -> u32;
}

macro_rules! impl_count_digits {
    (signed, $type:ty) => {
        impl CountDigits for $type {
            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                self.abs_diff(0).checked_ilog10().unwrap_or_default() + 1
            }
        }
    };

    (unsigned, $type:ty) => {
        impl CountDigits for $type {
            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                self.checked_ilog10().unwrap_or_default() + 1
            }
        }
    };
}

macro_rules! impl_nonzero_count_digits {
    ($type:ty) => {
        impl CountDigits for $type {
            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                self.get().abs_diff(0).ilog10() + 1
            }
        }
    };
}

impl_count_digits!(signed, i8);
impl_count_digits!(signed, i16);
impl_count_digits!(signed, i32);
impl_count_digits!(signed, i64);
impl_count_digits!(signed, i128);
impl_count_digits!(signed, isize);

impl_count_digits!(unsigned, u8);
impl_count_digits!(unsigned, u16);
impl_count_digits!(unsigned, u32);
impl_count_digits!(unsigned, u64);
impl_count_digits!(unsigned, u128);
impl_count_digits!(unsigned, usize);

impl_nonzero_count_digits!(NonZeroI8);
impl_nonzero_count_digits!(NonZeroI16);
impl_nonzero_count_digits!(NonZeroI32);
impl_nonzero_count_digits!(NonZeroI64);
impl_nonzero_count_digits!(NonZeroI128);
impl_nonzero_count_digits!(NonZeroIsize);
