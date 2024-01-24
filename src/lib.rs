#![no_std]
//! # CountDigits
//!
//! A trait to count the decimal digits of an integer.
//!
//! [CountDigits] is compatible with all the primitive integer types,
//! and all non-zero integer types.
//!
//! ### Examples
//! ```rust
//! use count_digits::CountDigits;
//! # use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
//! # use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
//!
//! assert_eq!(03, i8::MIN.count_digits());
//! assert_eq!(03, i8::MAX.count_digits());
//! assert_eq!(03, NonZeroI8::MIN.count_digits());
//! assert_eq!(03, NonZeroI8::MAX.count_digits());
//!
//! assert_eq!(01, u8::MIN.count_digits());
//! assert_eq!(03, u8::MAX.count_digits());
//! assert_eq!(01, NonZeroU8::MIN.count_digits());
//! assert_eq!(03, NonZeroU8::MAX.count_digits());
//!
//! assert_eq!(05, i16::MIN.count_digits());
//! assert_eq!(05, i16::MAX.count_digits());
//! assert_eq!(05, NonZeroI16::MIN.count_digits());
//! assert_eq!(05, NonZeroI16::MAX.count_digits());
//!
//! assert_eq!(01, u16::MIN.count_digits());
//! assert_eq!(05, u16::MAX.count_digits());
//! assert_eq!(01, NonZeroU16::MIN.count_digits());
//! assert_eq!(05, NonZeroU16::MAX.count_digits());
//!
//! assert_eq!(10, i32::MIN.count_digits());
//! assert_eq!(10, i32::MAX.count_digits());
//! assert_eq!(10, NonZeroI32::MIN.count_digits());
//! assert_eq!(10, NonZeroI32::MAX.count_digits());
//!
//! assert_eq!(01, u32::MIN.count_digits());
//! assert_eq!(10, u32::MAX.count_digits());
//! assert_eq!(01, NonZeroU32::MIN.count_digits());
//! assert_eq!(10, NonZeroU32::MAX.count_digits());
//!
//! assert_eq!(19, i64::MIN.count_digits());
//! assert_eq!(19, i64::MAX.count_digits());
//! assert_eq!(19, NonZeroI64::MIN.count_digits());
//! assert_eq!(19, NonZeroI64::MAX.count_digits());
//!
//! assert_eq!(01, u64::MIN.count_digits());
//! assert_eq!(20, u64::MAX.count_digits());
//! assert_eq!(01, NonZeroU64::MIN.count_digits());
//! assert_eq!(20, NonZeroU64::MAX.count_digits());
//!
//! assert_eq!(39, i128::MIN.count_digits());
//! assert_eq!(39, i128::MAX.count_digits());
//! assert_eq!(39, NonZeroI128::MIN.count_digits());
//! assert_eq!(39, NonZeroI128::MAX.count_digits());
//!
//! assert_eq!(01, u128::MIN.count_digits());
//! assert_eq!(39, u128::MAX.count_digits());
//! assert_eq!(01, NonZeroU128::MIN.count_digits());
//! assert_eq!(39, NonZeroU128::MAX.count_digits());
//!
//! #[cfg(target_pointer_width = "64")] {
//!   assert_eq!(isize::MIN.count_digits(), i64::MIN.count_digits());
//!   assert_eq!(isize::MAX.count_digits(), i64::MAX.count_digits());
//!   assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI64::MIN.count_digits());
//!   assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI64::MAX.count_digits());
//!
//!   assert_eq!(usize::MIN.count_digits(), u64::MIN.count_digits());
//!   assert_eq!(usize::MAX.count_digits(), u64::MAX.count_digits());
//!   assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU64::MIN.count_digits());
//!   assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU64::MAX.count_digits());
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
//!   assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU32::MIN.count_digits());
//!   assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU32::MAX.count_digits());
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
//!   assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU16::MIN.count_digits());
//!   assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU16::MAX.count_digits());
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
//!   assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU8::MIN.count_digits());
//!   assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU8::MAX.count_digits());
//! }
//! ```

use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

/// Count the decimal digits of an integer.
pub trait CountDigits: Copy + Sized {
    /// Returns the count of decimal digits in an integer.
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
}

macro_rules! impl_count_digits {
    (
        primitive_type = $primitive_type:ty,
        non_zero_type = $non_zero_type:ty,
    ) => {
        impl CountDigits for $primitive_type {
            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                1 + self.abs_diff(0).checked_ilog10().unwrap_or_default()
            }
        }

        impl CountDigits for $non_zero_type {
            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                1 + self.get().abs_diff(0).ilog10()
            }
        }
    };
    (
        primitive_type = $primitive_type:ty,
        non_zero_type = $non_zero_type:ty,
    ) => {
        impl CountDigits for $primitive_type {
            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                1 + self.checked_ilog10().unwrap_or_default()
            }
        }

        impl CountDigits for $non_zero_type {
            #[inline(always)]
            /// Returns the count of decimal digits in an integer.
            fn count_digits(self) -> u32 {
                1 + self.ilog10()
            }
        }
    };
}

impl_count_digits! {
    primitive_type = i8,
    non_zero_type = NonZeroI8,
}

impl_count_digits! {
    primitive_type = i16,
    non_zero_type = NonZeroI16,
}

impl_count_digits! {
    primitive_type = i32,
    non_zero_type = NonZeroI32,
}

impl_count_digits! {
    primitive_type = i64,
    non_zero_type = NonZeroI64,
}

impl_count_digits! {
    primitive_type = i128,
    non_zero_type = NonZeroI128,
}

#[cfg(target_pointer_width = "64")]
impl_count_digits! {
    primitive_type = isize,
    non_zero_type = NonZeroIsize,
}

#[cfg(target_pointer_width = "32")]
impl_count_digits! {
    primitive_type = isize,
    non_zero_type = NonZeroIsize,
}

#[cfg(target_pointer_width = "16")]
impl_count_digits! {
    primitive_type = isize,
    non_zero_type = NonZeroIsize,
}

#[cfg(target_pointer_width = "8")]
impl_count_digits! {
    primitive_type = isize,
    non_zero_type = NonZeroIsize,
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

    trait MinAndMax {
        const MAX_VALUE_DIGITS: u32;
        const MAX_VALUE: Self;

        const MIN_VALUE_DIGITS: u32;
        const MIN_VALUE: Self;
    }

    macro_rules! impl_min_and_max {
        ($type:ty, $min_digits:expr, $max_digits:expr) => {
            impl MinAndMax for $type {
                const MAX_VALUE_DIGITS: u32 = $max_digits;
                const MAX_VALUE: Self = Self::MAX;

                const MIN_VALUE_DIGITS: u32 = $min_digits;
                const MIN_VALUE: Self = Self::MIN;
            }
        };
    }

    impl_min_and_max!(i8, 3, 3);
    impl_min_and_max!(NonZeroI8, 3, 3);

    impl_min_and_max!(i16, 5, 5);
    impl_min_and_max!(NonZeroI16, 5, 5);

    impl_min_and_max!(i32, 10, 10);
    impl_min_and_max!(NonZeroI32, 10, 10);

    impl_min_and_max!(i64, 19, 19);
    impl_min_and_max!(NonZeroI64, 19, 19);

    impl_min_and_max!(i128, 39, 39);
    impl_min_and_max!(NonZeroI128, 39, 39);

    impl_min_and_max!(u8, 1, 3);
    impl_min_and_max!(NonZeroU8, 1, 3);

    impl_min_and_max!(u16, 1, 5);
    impl_min_and_max!(NonZeroU16, 1, 5);

    impl_min_and_max!(u32, 1, 10);
    impl_min_and_max!(NonZeroU32, 1, 10);

    impl_min_and_max!(u64, 1, 20);
    impl_min_and_max!(NonZeroU64, 1, 20);

    impl_min_and_max!(u128, 1, 39);
    impl_min_and_max!(NonZeroU128, 1, 39);

    #[cfg(target_pointer_width = "64")]
    impl_min_and_max!(isize, i64::MIN_VALUE_DIGITS, i64::MAX_VALUE_DIGITS);
    #[cfg(target_pointer_width = "64")]
    impl_min_and_max!(NonZeroIsize, i64::MIN_VALUE_DIGITS, i64::MAX_VALUE_DIGITS);

    #[cfg(target_pointer_width = "32")]
    impl_min_and_max!(isize, i32::MIN_VALUE_DIGITS, i32::MAX_VALUE_DIGITS);
    #[cfg(target_pointer_width = "32")]
    impl_min_and_max!(NonZeroIsize, i32::MIN_VALUE_DIGITS, i32::MAX_VALUE_DIGITS);

    #[cfg(target_pointer_width = "16")]
    impl_min_and_max!(isize, i16::MIN_VALUE_DIGITS, i16::MAX_VALUE_DIGITS);
    #[cfg(target_pointer_width = "16")]
    impl_min_and_max!(NonZeroIsize, i16::MIN_VALUE_DIGITS, i16::MAX_VALUE_DIGITS);

    #[cfg(target_pointer_width = "8")]
    impl_min_and_max!(isize, i8::MIN_VALUE_DIGITS, i8::MAX_VALUE_DIGITS);
    #[cfg(target_pointer_width = "8")]
    impl_min_and_max!(NonZeroIsize, i8::MIN_VALUE_DIGITS, i8::MAX_VALUE_DIGITS);

    #[cfg(target_pointer_width = "64")]
    impl_min_and_max!(usize, u64::MIN_VALUE_DIGITS, u64::MAX_VALUE_DIGITS);
    #[cfg(target_pointer_width = "64")]
    impl_min_and_max!(NonZeroUsize, u64::MIN_VALUE_DIGITS, u64::MAX_VALUE_DIGITS);

    #[cfg(target_pointer_width = "32")]
    impl_min_and_max!(usize, u32::MIN_VALUE_DIGITS, u32::MAX_VALUE_DIGITS);
    #[cfg(target_pointer_width = "32")]
    impl_min_and_max!(NonZeroUsize, u32::MIN_VALUE_DIGITS, u32::MAX_VALUE_DIGITS);

    #[cfg(target_pointer_width = "16")]
    impl_min_and_max!(usize, u16::MIN_VALUE_DIGITS, u16::MAX_VALUE_DIGITS);
    #[cfg(target_pointer_width = "16")]
    impl_min_and_max!(NonZeroUsize, u16::MIN_VALUE_DIGITS, u16::MAX_VALUE_DIGITS);

    #[cfg(target_pointer_width = "8")]
    impl_min_and_max!(usize, u8::MIN_VALUE_DIGITS, u8::MAX_VALUE_DIGITS);
    #[cfg(target_pointer_width = "8")]
    impl_min_and_max!(NonZeroUsize, u8::MIN_VALUE_DIGITS, u8::MAX_VALUE_DIGITS);

    macro_rules! min_and_max {
        ($type:ty, $non_zero_type:ty) => {
            paste! {
                #[test]
                fn [<min_and_max_ $type>]() {
                    assert_eq!($type::MIN_VALUE.count_digits(), $type::MIN_VALUE_DIGITS,);
                    assert_eq!($type::MAX_VALUE.count_digits(), $type::MAX_VALUE_DIGITS,);
                }

                #[test]
                #[allow(non_snake_case)]
                fn [<min_and_max_ $non_zero_type>]() {
                    assert_eq!($non_zero_type::MIN_VALUE.count_digits(), $non_zero_type::MIN_VALUE_DIGITS,);
                    assert_eq!($non_zero_type::MAX_VALUE.count_digits(), $non_zero_type::MAX_VALUE_DIGITS,);
                }
            }
        }
    }

    macro_rules! min_to_max_or_one_million {
        ($type:ty, $non_zero_type:ty) => {
            paste! {
                #[test]
                #[allow(overflowing_literals)]
                fn [<min_to_max_or_one_million_ $type>]() {
                    let max = if ($type::MAX as u128) < 1_000_000 { $type::MAX } else { 1_000_000 };
                    for n in $type::MIN..=max {
                        let i128 = n as i128;
                             if i128  < 10        { assert_eq!(n.count_digits(), 1, "Expect count of 1 for {n}") }
                        else if i128  < 100       { assert_eq!(n.count_digits(), 2, "Expect count of 2 for {n}") }
                        else if i128  < 1_000     { assert_eq!(n.count_digits(), 3, "Expect count of 3 for {n}") }
                        else if i128  < 10_000    { assert_eq!(n.count_digits(), 4, "Expect count of 4 for {n}") }
                        else if i128  < 100_000   { assert_eq!(n.count_digits(), 5, "Expect count of 5 for {n}") }
                        else if i128  < 1_000_000 { assert_eq!(n.count_digits(), 6, "Expect count of 6 for {n}") }
                        else if i128 == 1_000_000 { assert_eq!(n.count_digits(), 7, "Expect count of 7 for {n}") }
                    }
                }

                #[test]
                #[allow(non_snake_case)]
                #[allow(overflowing_literals)]
                fn [<min_to_max_or_one_million_ $non_zero_type>]() {
                    let max = if ($type::MAX as u128) < 1_000_000 { $type::MAX } else { 1_000_000 };
                    for n in $non_zero_type::MIN.get()..=max {
                        let i128 = n as u128;
                        let n = $non_zero_type::new(n).unwrap();
                             if i128  < 10        { assert_eq!(n.count_digits(), 1, "Expect count of 1 for {n}") }
                        else if i128  < 100       { assert_eq!(n.count_digits(), 2, "Expect count of 2 for {n}") }
                        else if i128  < 1_000     { assert_eq!(n.count_digits(), 3, "Expect count of 3 for {n}") }
                        else if i128  < 10_000    { assert_eq!(n.count_digits(), 4, "Expect count of 4 for {n}") }
                        else if i128  < 100_000   { assert_eq!(n.count_digits(), 5, "Expect count of 5 for {n}") }
                        else if i128  < 1_000_000 { assert_eq!(n.count_digits(), 6, "Expect count of 6 for {n}") }
                        else if i128 == 1_000_000 { assert_eq!(n.count_digits(), 7, "Expect count of 7 for {n}") }
                    }
                }
            }
        };
    }

    macro_rules! min_or_negative_one_million_to_max_or_one_million {
        ($type:ty, $non_zero_type:ty) => {
            paste! {
                #[test]
                #[allow(overflowing_literals)]
                fn [<min_or_negative_one_million_to_max_or_one_million_ $type>]() {
                    let max = if ($type::MAX as u128) <  1_000_000 { $type::MAX } else {  1_000_000 };
                    let min = if ($type::MIN as i128) > -1_000_000 { $type::MIN } else { -1_000_000 };
                    for n in min..=max {
                        let i128 = n as i128;
                             if i128 == -1_000_000 { assert_eq!(n.count_digits(), 7, "Expect count of 7 for {n}") }
                        else if i128  < -99_999    { assert_eq!(n.count_digits(), 6, "Expect count of 6 for {n}") }
                        else if i128  < -9_999     { assert_eq!(n.count_digits(), 5, "Expect count of 5 for {n}") }
                        else if i128  < -999       { assert_eq!(n.count_digits(), 4, "Expect count of 4 for {n}") }
                        else if i128  < -99        { assert_eq!(n.count_digits(), 3, "Expect count of 3 for {n}") }
                        else if i128  < -9         { assert_eq!(n.count_digits(), 2, "Expect count of 2 for {n}") }
                        else if i128  < 10         { assert_eq!(n.count_digits(), 1, "Expect count of 1 for {n}") }
                        else if i128  < 100        { assert_eq!(n.count_digits(), 2, "Expect count of 2 for {n}") }
                        else if i128  < 1_000      { assert_eq!(n.count_digits(), 3, "Expect count of 3 for {n}") }
                        else if i128  < 10_000     { assert_eq!(n.count_digits(), 4, "Expect count of 4 for {n}") }
                        else if i128  < 100_000    { assert_eq!(n.count_digits(), 5, "Expect count of 5 for {n}") }
                        else if i128  < 1_000_000  { assert_eq!(n.count_digits(), 6, "Expect count of 6 for {n}") }
                        else if i128 == 1_000_000  { assert_eq!(n.count_digits(), 7, "Expect count of 7 for {n}") }
                    }
                }

                #[test]
                #[allow(non_snake_case)]
                #[allow(overflowing_literals)]
                fn [<min_or_negative_one_million_to_max_or_one_million_ $non_zero_type>]() {
                    let max = if ($type::MAX as u128) <  1_000_000 { $type::MAX } else {  1_000_000 };
                    let min = if ($type::MIN as i128) > -1_000_000 { $type::MIN } else { -1_000_000 };
                    for n in min..=max {
                        if n == 0 { continue; }
                        let i128 = n as i128;
                        let n = $non_zero_type::new(n).unwrap();
                             if i128 == -1_000_000 { assert_eq!(n.count_digits(), 7, "Expect count of 6 for {n}") }
                        else if i128  < -99_999    { assert_eq!(n.count_digits(), 6, "Expect count of 6 for {n}") }
                        else if i128  < -9_999     { assert_eq!(n.count_digits(), 5, "Expect count of 5 for {n}") }
                        else if i128  < -999       { assert_eq!(n.count_digits(), 4, "Expect count of 4 for {n}") }
                        else if i128  < -99        { assert_eq!(n.count_digits(), 3, "Expect count of 3 for {n}") }
                        else if i128  < -9         { assert_eq!(n.count_digits(), 2, "Expect count of 2 for {n}") }
                        else if i128  < 10         { assert_eq!(n.count_digits(), 1, "Expect count of 1 for {n}") }
                        else if i128  < 100        { assert_eq!(n.count_digits(), 2, "Expect count of 2 for {n}") }
                        else if i128  < 1_000      { assert_eq!(n.count_digits(), 3, "Expect count of 3 for {n}") }
                        else if i128  < 10_000     { assert_eq!(n.count_digits(), 4, "Expect count of 4 for {n}") }
                        else if i128  < 100_000    { assert_eq!(n.count_digits(), 5, "Expect count of 5 for {n}") }
                        else if i128  < 1_000_000  { assert_eq!(n.count_digits(), 6, "Expect count of 6 for {n}") }
                        else if i128 == 1_000_000  { assert_eq!(n.count_digits(), 7, "Expect count of 7 for {n}") }
                    }
                }
            }
        };
    }

    macro_rules! add_test {
        ($name:ident, $type:ty, $non_zero_type:ty) => {
            $name!($type, $non_zero_type);
        };
    }

    add_test!(min_and_max, u8, NonZeroU8);
    add_test!(min_and_max, u16, NonZeroU16);
    add_test!(min_and_max, u32, NonZeroU32);
    add_test!(min_and_max, u64, NonZeroU64);
    add_test!(min_and_max, u128, NonZeroU128);
    add_test!(min_and_max, usize, NonZeroUsize);

    add_test!(min_and_max, i8, NonZeroI8);
    add_test!(min_and_max, i16, NonZeroI16);
    add_test!(min_and_max, i32, NonZeroI32);
    add_test!(min_and_max, i64, NonZeroI64);
    add_test!(min_and_max, i128, NonZeroI128);
    add_test!(min_and_max, isize, NonZeroIsize);

    add_test!(min_to_max_or_one_million, u8, NonZeroU8);
    add_test!(min_to_max_or_one_million, u16, NonZeroU16);
    add_test!(min_to_max_or_one_million, u32, NonZeroU32);
    add_test!(min_to_max_or_one_million, u64, NonZeroU64);
    add_test!(min_to_max_or_one_million, u128, NonZeroU128);
    add_test!(min_to_max_or_one_million, usize, NonZeroUsize);

    add_test!(
        min_or_negative_one_million_to_max_or_one_million,
        i8,
        NonZeroI8
    );
    add_test!(
        min_or_negative_one_million_to_max_or_one_million,
        i16,
        NonZeroI16
    );
    add_test!(
        min_or_negative_one_million_to_max_or_one_million,
        i32,
        NonZeroI32
    );
    add_test!(
        min_or_negative_one_million_to_max_or_one_million,
        i64,
        NonZeroI64
    );
    add_test!(
        min_or_negative_one_million_to_max_or_one_million,
        i128,
        NonZeroI128
    );
    add_test!(
        min_or_negative_one_million_to_max_or_one_million,
        isize,
        NonZeroIsize
    );
}
