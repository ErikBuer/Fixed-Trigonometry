# Release Notes

## Release 0.2.5 (2021-11-20)

- Corrected error in niirf for certain octants.
- Added comparison figure, between the std::f32::sqrt and fixed_trigonometry::sqrt::niirf implementations.
- Added comparison figure, between the various atan appriximations.

## Release 0.2.4 (2021-11-04)

- Added complex polar form with conversion to num::complex::Complex.
- Added abs, add, sub, mul for num::complex::Complex numbers for no-std environments.

## Release 0.2.3 (2021-11-01)

- Corrected edge case in atan2_fast function.
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
