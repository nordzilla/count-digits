#![no_std]
//! # CountDigits
//!
//! A trait to count the decimal digits of an integer.

/// Count the decimal digits of an integer.
pub trait CountDigits: Copy + Sized {
    /// Returns the count of decimal digits in an integer.
    fn count_digits(self) -> u32;
}
