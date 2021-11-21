# Fixed Trigonometry

No-STD fixed-point implementation of trigonometric functions in Rust.

It utilizes the [fixed](https://crates.io/crates/fixed) library to allow flexibility in fixed point sizes and precisions.

The [package](https://crates.io/crates/fixed_trigonometry).

The [documentation](https://docs.rs/fixed_trigonometry/).

Release notes are found under RELEASES.md.

## Functionality

The library currently implements:

- `fft`/`ifft` calculation, for complex fixed-point vectors.
- `sin` and `cos` using low order polynomails, for real fixed-point numbers.
- `atan` using numerical methods.
- `sqrt` using the Nonlinear IIR Filter (NIIRF) method.
- `powi` and `complex::powi`.
- no-std utilities for complex numbers.
