# Release Notes

## Release 0.3.3 (2022-01-xx)

- Changed to Rust 2021.
- Implemented mixed-num trait to add floating point support as well.

## Release 0.3.2 (2022-01-01)

- Added comparison between `atan::atan` and `cordic::atan`.
- Changed vector argument of fft to `&mut [Complex<T>]`.
- Changed description of the `niirf` error plot, and added error comparison with `cordic::sqrt`.

## Release 0.3.1 (2021-11-29)

- Implemented the cordic crate. It is used in various sin and cos calculations allow greater precision.

## Release 0.3.0 (2021-11-21)

- Added ifft implementation.
- Improved cos implementation by utilizing the more accurate sin polynomial.
- Added accuracy comparison for cosine and sine.
- Moved the atan2 implementation to the main atan2.
- atan now utilizes the atan2 function.

## Release 0.2.6 (2021-11-20)

- Added fft implementation.

## Release 0.2.5 (2021-11-20)

- Corrected error in niirf for certain octants.
- Added comparison figure, between the std::f32::sqrt and fixed_trigonometry::sqrt::niirf implementations.
- Added comparison figure, between the various atan appriximations.

## Release 0.2.4 (2021-11-04)

- Added complex polar form with conversion to num::complex::Complex.
- Added abs, add, sub, mul for num::complex::Complex numbers for no-std environments.

## Release 0.2.3 (2021-11-01)

- Corrected edge case in atan2 function.
- Removed dependency to fixed-sqrt.

## Release 0.2.2 (2021-10-31)

- Corrected edge case in Nonlinear IIR Filter sqrt function.

## Release 0.2.1 (2021-10-24)

- Added the Nonlinear IIR Filter sqrt method.

## Release 0.2.0 (2021-10-23)

- Separated out atan approximations to own file.
- Addad a fast atan approximation.
- Added test for powi.

## Release 0.1.2 (2021-09-18)

- Added complex powi.

## Release 0.1.1 (2021-09-18)

- Reduced division denominators in sin to reduce overflow issues.

## Release 0.1.0 (2021-09-18)

- First release.

**Contributors**: ErikBuer
