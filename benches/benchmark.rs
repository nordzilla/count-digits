use count_digits::CountDigits;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! bench_function {
    ($group:expr, $type:ty, $fn:ident, $input:expr) => {
        $group.bench_with_input(
            BenchmarkId::new(stringify!($type), $input),
            $input,
            |b, n| b.iter(|| CountDigits::$fn(black_box(*n as $type))),
        );
    };
}

macro_rules! bench_group {
    ($criterion:expr, $group_name:ident, $fn:ident, $input:expr, $($type:ty),+ $(,)?) => {{
        paste::paste! {
            let mut group = $criterion.benchmark_group(stringify!([<$group_name _ $fn>]));
            for input in $input.iter() {
                $(bench_function!(group, $type, $fn, input);)+
            }
            group.finish();
        }
    }};
}

macro_rules! create_bench {
    ($fn:ident) => {
        fn $fn(c: &mut Criterion) {
            bench_group!(
                c,
                signed,
                $fn,
                [i8::MIN, -64, 0, 64, i8::MAX],
                i8,
                i16,
                i32,
                i64,
                i128
            );
            bench_group!(
                c,
                unsigned,
                $fn,
                [u8::MIN, 64, 128, 192, u8::MAX],
                u8,
                u16,
                u32,
                u64,
                u128
            );
        }
    };
}

create_bench!(count_bits);
create_bench!(count_octal_digits);
create_bench!(count_digits);
create_bench!(count_hex_digits);

criterion_group!(
    benches,
    count_bits,
    count_octal_digits,
    count_digits,
    count_hex_digits
);

criterion_main!(benches);
