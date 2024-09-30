# Release Notes

## Release 0.4.4 (2024-09-30)

- "Add an alloc feature to control need for a global allocaror"

## Release 0.4.3 (2022-05-08)

- Corrects ifft scaling.
- Bumps `mixed-num` to latest version.

## Release 0.4.2 (2022-03-20)

- Corrects fft twiddle factor index calculation.

## Release 0.4.1 (2022-03-20)

- Improves complex cartesian multiplication.
- Corrects powi to include n⁰=1.

## Release 0.4.0 (2022-03-13)

- Bumbs version of `fixed`.
- Bumps version of mixed_num. Brings major accuracy improvement for floating point types (utilizing the complex utilities).
- Efficiency improvement of `to_cartsian()` function.

## Release 0.3.6 (2022-02-01)

- Minor improvements of polynomial sine.

## Release 0.3.5 (2022-01-30)

- Updated to support restructured traits in the mixed-num crate.

## Release 0.3.4 (2022-01-29)

- Changed to Rust 2021.
- Implemented mixed-num support for the `::complex` and `::fft` modules.

## Release 0.3.3 (2022-01-04)

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
