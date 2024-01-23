# CountDigits
A Rust trait to count the decimal digits of integers

---

```rust
use count_digits::CountDigits;
use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

assert_eq!(03, i8::MIN.count_digits());
assert_eq!(03, i8::MAX.count_digits());
assert_eq!(03, NonZeroI8::MIN.count_digits());
assert_eq!(03, NonZeroI8::MAX.count_digits());

assert_eq!(01, u8::MIN.count_digits());
assert_eq!(03, u8::MAX.count_digits());
assert_eq!(01, NonZeroU8::MIN.count_digits());
assert_eq!(03, NonZeroU8::MAX.count_digits());

assert_eq!(05, i16::MIN.count_digits());
assert_eq!(05, i16::MAX.count_digits());
assert_eq!(05, NonZeroI16::MIN.count_digits());
assert_eq!(05, NonZeroI16::MAX.count_digits());

assert_eq!(01, u16::MIN.count_digits());
assert_eq!(05, u16::MAX.count_digits());
assert_eq!(01, NonZeroU16::MIN.count_digits());
assert_eq!(05, NonZeroU16::MAX.count_digits());

assert_eq!(10, i32::MIN.count_digits());
assert_eq!(10, i32::MAX.count_digits());
assert_eq!(10, NonZeroI32::MIN.count_digits());
assert_eq!(10, NonZeroI32::MAX.count_digits());

assert_eq!(01, u32::MIN.count_digits());
assert_eq!(10, u32::MAX.count_digits());
assert_eq!(01, NonZeroU32::MIN.count_digits());
assert_eq!(10, NonZeroU32::MAX.count_digits());

assert_eq!(19, i64::MIN.count_digits());
assert_eq!(19, i64::MAX.count_digits());
assert_eq!(19, NonZeroI64::MIN.count_digits());
assert_eq!(19, NonZeroI64::MAX.count_digits());

assert_eq!(01, u64::MIN.count_digits());
assert_eq!(20, u64::MAX.count_digits());
assert_eq!(01, NonZeroU64::MIN.count_digits());
assert_eq!(20, NonZeroU64::MAX.count_digits());

assert_eq!(39, i128::MIN.count_digits());
assert_eq!(39, i128::MAX.count_digits());
assert_eq!(39, NonZeroI128::MIN.count_digits());
assert_eq!(39, NonZeroI128::MAX.count_digits());

assert_eq!(01, u128::MIN.count_digits());
assert_eq!(39, u128::MAX.count_digits());
assert_eq!(01, NonZeroU128::MIN.count_digits());
assert_eq!(39, NonZeroU128::MAX.count_digits());

#[cfg(target_pointer_width = "64")] {
  assert_eq!(isize::MIN.count_digits(), i64::MIN.count_digits());
  assert_eq!(isize::MAX.count_digits(), i64::MAX.count_digits());
  assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI64::MIN.count_digits());
  assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI64::MAX.count_digits());

  assert_eq!(usize::MIN.count_digits(), u64::MIN.count_digits());
  assert_eq!(usize::MAX.count_digits(), u64::MAX.count_digits());
  assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU64::MIN.count_digits());
  assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU64::MAX.count_digits());
}

#[cfg(target_pointer_width = "32")] {
  assert_eq!(isize::MIN.count_digits(), i32::MIN.count_digits());
  assert_eq!(isize::MAX.count_digits(), i32::MAX.count_digits());
  assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI32::MIN.count_digits());
  assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI32::MAX.count_digits());

  assert_eq!(usize::MIN.count_digits(), u32::MIN.count_digits());
  assert_eq!(usize::MAX.count_digits(), u32::MAX.count_digits());
  assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU32::MIN.count_digits());
  assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU32::MAX.count_digits());
}

#[cfg(target_pointer_width = "16")] {
  assert_eq!(isize::MIN.count_digits(), i16::MIN.count_digits());
  assert_eq!(isize::MAX.count_digits(), i16::MAX.count_digits());
  assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI16::MIN.count_digits());
  assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI16::MAX.count_digits());

  assert_eq!(usize::MIN.count_digits(), u16::MIN.count_digits());
  assert_eq!(usize::MAX.count_digits(), u16::MAX.count_digits());
  assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU16::MIN.count_digits());
  assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU16::MAX.count_digits());
}

#[cfg(target_pointer_width = "8")] {
  assert_eq!(isize::MIN.count_digits(), i8::MIN.count_digits());
  assert_eq!(isize::MAX.count_digits(), i8::MAX.count_digits());
  assert_eq!(NonZeroIsize::MIN.count_digits(), NonZeroI8::MIN.count_digits());
  assert_eq!(NonZeroIsize::MAX.count_digits(), NonZeroI8::MAX.count_digits());

  assert_eq!(usize::MIN.count_digits(), u8::MIN.count_digits());
  assert_eq!(usize::MAX.count_digits(), u8::MAX.count_digits());
  assert_eq!(NonZeroUsize::MIN.count_digits(), NonZeroU8::MIN.count_digits());
  assert_eq!(NonZeroUsize::MAX.count_digits(), NonZeroU8::MAX.count_digits());
}
```

