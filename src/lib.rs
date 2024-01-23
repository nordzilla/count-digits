#![no_std]
//! # CountDigits
//!
//! A trait to count the decimal digits of an integer.
//!
//! ### Examples
//! ```rust
//! use count_digits::CountDigits;
//!
//! assert_eq!(03, i8::MIN.count_digits());
//! assert_eq!(03, i8::MAX.count_digits());
//!
//! assert_eq!(05, i16::MIN.count_digits());
//! assert_eq!(05, i16::MAX.count_digits());
//!
//! assert_eq!(10, i32::MIN.count_digits());
//! assert_eq!(10, i32::MAX.count_digits());
//!
//! assert_eq!(19, i64::MIN.count_digits());
//! assert_eq!(19, i64::MAX.count_digits());
//!
//! assert_eq!(39, i128::MIN.count_digits());
//! assert_eq!(39, i128::MAX.count_digits());
//!
//! #[cfg(target_pointer_width = "64")] {
//!   assert_eq!(isize::MIN.count_digits(), i64::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i64::MAX.count_digits());
//! }
//!
//! #[cfg(target_pointer_width = "32")] {
//!   assert_eq!(isize::MIN.count_digits(), i32::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i32::MAX.count_digits());
//! }
//!
//! #[cfg(target_pointer_width = "16")] {
//!   assert_eq!(isize::MIN.count_digits(), i16::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i16::MAX.count_digits());
//! }
//!
//! #[cfg(target_pointer_width = "8")] {
//!   assert_eq!(isize::MIN.count_digits(), i8::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i8::MAX.count_digits());
//! }
//! ```

/// Count the decimal digits of an integer.
pub trait CountDigits: Copy + Sized {
    /// Returns the count of decimal digits in an integer.
    /// ### Examples
    /// ```rust
    /// use count_digits::CountDigits;
    ///
    /// assert_eq!(03, i8::MIN.count_digits());
    /// assert_eq!(03, i8::MAX.count_digits());
    ///
    /// assert_eq!(05, i16::MIN.count_digits());
    /// assert_eq!(05, i16::MAX.count_digits());
    ///
    /// assert_eq!(10, i32::MIN.count_digits());
    /// assert_eq!(10, i32::MAX.count_digits());
    ///
    /// assert_eq!(19, i64::MIN.count_digits());
    /// assert_eq!(19, i64::MAX.count_digits());
    ///
    /// assert_eq!(39, i128::MIN.count_digits());
    /// assert_eq!(39, i128::MAX.count_digits());
    ///
    /// #[cfg(target_pointer_width = "64")] {
    ///   assert_eq!(isize::MIN.count_digits(), i64::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i64::MAX.count_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "32")] {
    ///   assert_eq!(isize::MIN.count_digits(), i32::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i32::MAX.count_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "16")] {
    ///   assert_eq!(isize::MIN.count_digits(), i16::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i16::MAX.count_digits());
    /// }
    ///
    /// #[cfg(target_pointer_width = "8")] {
    ///   assert_eq!(isize::MIN.count_digits(), i8::MIN.count_digits());
    ///   assert_eq!(isize::MAX.count_digits(), i8::MAX.count_digits());
    /// }
    /// ```
    fn count_digits(self) -> u32;
}

macro_rules! impl_count_digits {
    ($type:ty) => {
        impl CountDigits for $type {
            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                self.abs_diff(0).checked_ilog10().unwrap_or_default() + 1
            }
        }
    };
}

impl_count_digits!(i8);
impl_count_digits!(i16);
impl_count_digits!(i32);
impl_count_digits!(i64);
impl_count_digits!(i128);
impl_count_digits!(isize);
