use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};
use count_digits::CountDigits;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

trait MaybeFromU128: Sized {
    fn maybe_from_u128(u128: u128) -> Option<Self>;
}

macro_rules! impl_maybe_from_u128 {
    ($type:ty, $non_zero_type:ty) => {
        impl MaybeFromU128 for $type {
            fn maybe_from_u128(value: u128) -> Option<Self> {
                if value <= (Self::MAX as u128) {
                    return Some(value as $type);
                }
                None
            }
        }
        impl MaybeFromU128 for $non_zero_type {
            fn maybe_from_u128(value: u128) -> Option<Self> {
                if value <= (Self::MAX.get() as u128) {
                    return Self::new(value as $type);
                }
                None
            }
        }
    };
}

impl_maybe_from_u128!(i8, NonZeroI8);
impl_maybe_from_u128!(i16, NonZeroI16);
impl_maybe_from_u128!(i32, NonZeroI32);
impl_maybe_from_u128!(i64, NonZeroI64);
impl_maybe_from_u128!(i128, NonZeroI128);
impl_maybe_from_u128!(u8, NonZeroU8);
impl_maybe_from_u128!(u16, NonZeroU16);
impl_maybe_from_u128!(u32, NonZeroU32);
impl_maybe_from_u128!(u64, NonZeroU64);
impl_maybe_from_u128!(u128, NonZeroU128);

macro_rules! radix_boundaries {
    ($type:ty, $radix:expr) => {
        std::iter::successors(Some($radix as $type), move |n| n.checked_mul($radix))
            .map(|n| n - 1)
    };
}

macro_rules! comparison_bench_function {
    ($group:expr, $type:ty, $fn:ident, $input:expr) => {
        if let Some(n) = <$type>::maybe_from_u128(*$input) {
            $group.bench_with_input(
                BenchmarkId::new(stringify!($type), $input),
                $input,
                |b, _| b.iter(|| CountDigits::$fn(black_box(n))),
            );
        }
    };
}

macro_rules! comparison_bench_group {
    ($criterion:expr, $fn:ident, $input:expr, $($type:ty),+ $(,)?) => {{
        let mut group = $criterion.benchmark_group(stringify!($fn));
        for input in &$input {
            $(comparison_bench_function!(group, $type, $fn, input);)+
        }
        group.finish();
    }};
}

macro_rules! create_comparison_bench {
    ($fn:ident) => {
        fn $fn(criterion: &mut Criterion) {
            comparison_bench_group!(
                criterion,
                $fn,
                radix_boundaries!(u128, 16).collect::<Vec<_>>(),
                i8,
                i16,
                i32,
                i64,
                i128,
                u8,
                u16,
                u32,
                u64,
                u128,
                NonZeroI8,
                NonZeroI16,
                NonZeroI32,
                NonZeroI64,
                NonZeroI128,
                NonZeroU8,
                NonZeroU16,
                NonZeroU32,
                NonZeroU64,
                NonZeroU128,
            );
        }
    };
}

create_comparison_bench!(count_bits);
create_comparison_bench!(count_octal_digits);
create_comparison_bench!(count_digits);
create_comparison_bench!(count_hex_digits);

criterion_group!(
    benchmarks,
    count_bits,
    count_octal_digits,
    count_digits,
    count_hex_digits
);

criterion_main!(benchmarks);
