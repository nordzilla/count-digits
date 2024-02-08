# Changelog

## v0.4.0 (2024-02-08)

**Major Changes**

* Fixes an issue where primitive integer implementations were not properly panicking
for passing radix 0 and 1 to the `CountDigits::count_digits_radix()` function.

**Minor Changes**

* Update documentation clarifying the Radix type.

## v0.3.1 (2024-02-07)

**Minor Changes**

* Updates the documentation to more closely match the changes in v0.3.0

## v0.3.0 (2024-02-04)

**Major Changes**

* Changes the return type of `CountDigits::count_digits()` from `u32` to `usize`.
* Changes the return type of `CountDigits::count_digits_radix()` from `u32` to `usize`.
* Implements `CountDigits` for references to types that implement `CountDigits`.

**Adds**

* Adds new tests for reference types.

The rationale behind changing the return types listed above is that bit-related functions
typically return `u32` and take `u32` as parameters, but `usize` is often used for formatting
and display. 

In this way, the inherently bit-related functions, `count_bits()`, 
`count_octal_digits()`, and `count_hex_digits()` continue to return `u32` and can be used
with the `BITS` constants as well as bit-shifting functions without the need to cast. 

Likewise, the `count_digits()` function as well as the `count_digits_radix()` function can be used
as arguments for formatting without the need to cast. 

I believe this provides the best compromise,
allowing the trait to be used smoothly across various use cases without the `as` keyword. 

## v0.2.6 (2024-01-31)

**Changes**

* Includes only the files necessary to build the crate for publishing in the Cargo.toml.

## v0.2.5 (2024-01-31)

**Improves**

* Refactors count_octal_digits to use ilog2 internally.
* Refactors count_hex_digits to use ilog2 internally.

The v0.2.4 benchmarks show that `ilog2`, used by `count_bits`, has the same performance regardless of the size of the integer input. 

The v0.2.4 benchmarks show that `count_octal_digits` and `count_hex_digits` functions, which were previously using `ilog(8)` and `ilog(16)` respectively, did show decreased performance based on the size of the integer input.

![A graph of count-octal-digits benchmarks for v0.2.4](https://raw.githubusercontent.com/nordzilla/count-digits/main/benches/images/count-octal-digits-v0.2.4.png)

The v0.2.5 `count_octal_digits` and `count_hex_digits` are now implemented internally using `ilog2`, and now share that consistent, improved performance across the board.

![A graph of count-octal-digits benchmarks for v0.2.5](https://raw.githubusercontent.com/nordzilla/count-digits/main/benches/images/count-octal-digits-v0.2.5.png)

## v0.2.4 (2024-01-30)

**Adds**

* Adds links to [benchmarks](https://nordzilla.github.io/count-digits/) to documentation, hosted via [GitHub Pages](https://pages.github.com/).
* Adds CHANGELOG.md (that's me) to the project.

**Changes**

* Reworks testing macros to make the test output more granular.
* Reworks the benchmark test boundaries to test iterations for each hexadecimal digit added to an integer, from one to max, for each integer type.

**Improves**

* Removes unnecessary `checked_ilog` functions from the implementations of various `CountDigits` functions.

## v0.2.3 (2024-01-26)

**Changes**

* Updates documentation.

## v0.2.2 (2024-01-25)

**Adds**

* Adds benchmarks using [Criterion](https://docs.rs/criterion/latest/criterion/).
* Adds unit tests for helper functions used throughout the test suite.

**Changes**

* Updates documentation.

## v0.2.1 (2024-01-24)

**Adds**

* Adds [CodeCov](https://about.codecov.io/) jobs to CI.
* Adds radix boundary tests for all integer types.

**Changes**

* Updates documentation.
* Refactors iteration tests.

**Fixes**

* Fixes an issue where the unsigned, non-zero integer iteration tests were calling the `CountDigits` functions for their corresponding primitive types, resulting in a gap of test coverage for these types.

## v0.2.0 (2024-01-24)

**Major Changes**
* Adds new trait functions to `CountDigits`, listed below.

**Adds**

* Adds `CountDigits::count_bits()`.
* Adds `CountDigits::count_octal_digits()`.
* Adds `CountDigits::count_hex_digits()`.
* Adds `CountDigits::count_digits_radix()`.
* Implements the above functions for signed, primitive integer types.
* Implements the above functions for signed, non-zero integer types.
* Implements the above functions for unsigned, primitive integer types.
* Implements the above functions for unsigned, non-zero integer types.
* Bumps the minimum supported Rust version `(1.67.1 -> 1.71.1)` due to using `is_negative()` for non-zero integer types.

**Minor Changes**
* Refactors iteration tests.
* Refactors min_and_max tests.

## v0.1.0 (2024-01-22)
> Initial Release

**Adds**

* Adds README.md.
* Adds Cargo.toml.
* Adds `CountDigits` trait.
* Adds `CountDigits::count_digits()` function.
* Implements `CountDigits` for signed, primitive integer types.
* Implements `CountDigits` for signed, non-zero integer types.
* Implements `CountDigits` for unsigned, primitive integer types.
* Implements `CountDigits` for unsigned, non-zero integer types.
* Adds tests for min and max values.
* Adds iterative tests for unsigned types.
* Adds iterative tests for signed types.
* Adds rust.yml to run tests in CI via GitHub Actions.
