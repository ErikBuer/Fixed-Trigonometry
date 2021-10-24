# Fixed Trigonometry

No-STD fixed-point implementation of trigonometric functions in Rust.

It utilizes the [fixed](https://crates.io/crates/fixed) library to allow flexibility in fixed point sizes and precisions.

The [package](https://crates.io/crates/fixed_trigonometry).

The [documentation](https://docs.rs/fixed_trigonometry/).

Release notes are found under RELEASES.md.

## Functionality

The library currently implements:

- `sin` and `cos` using low order polynomails, for real fixed-point numbers.
- `atan` using multiple numerical methods.
- `sqrt` using the Nonlinear IIR Filter (NIIRF) method \[1\].
- `powi` and `complex::powi`.

\[1\] N.Mikami et al., A new DSP-oriented algorithm for calculation of square root using a non-linear digital filter, IEEE Trans. on Signal Processing, July 1992, pp. 1663-1669.
