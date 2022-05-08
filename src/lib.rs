//! 
//! No-STD fixed-point numeric implementation of trigonometric functions in Rust.
//! 
//! It utilizes the [fixed](https://crates.io/crates/fixed) library to allow flexibility in fixed point sizes and precisions.
//! 
//! The library [mixed-num](https://crates.io/crates/mixed-num) has these functions implemented as traits.
//! 
//! The library [NDSP](https://crates.io/crates/ndsp) support fixed point numbers in vectors, with various implemented operations.
//! 
//! ## Example
//! 
//! ```
//! use fixed_trigonometry::*;
//! use fixed::{types::extra::U28, FixedI32};
//! 
//! let arg = atan::atan( FixedI32::<U28>::from_num(0.6)/FixedI32::<U28>::from_num(0.4) );
//! assert_eq!{ arg.to_num::<f32>(), 0.9782037 };
//! ``` 

#![crate_name = "fixed_trigonometry"]
#![no_std]

// Use std for test.
#[cfg(all(not(feature = "std"), test))]
extern crate std;

use fixed;
use mixed_num::traits::*;

pub mod complex;
pub mod atan;
pub mod sqrt;
pub mod fft;

/// Rase fixed number to an integer-valued power.
/// `base^power`.
/// 
/// ## Arguments
/// 
/// * `base`  - The base number.
/// * `power` - The power to raise 'base' to.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U22, FixedI32};
/// 
/// let mut x = FixedI32::<U22>::from_num(-2);
/// let y = powi(x, 2);
/// assert_eq!{ y.to_num::<f32>(), 4.0 };
/// 
/// let y = powi(x, 0);
/// assert_eq!{ y.to_num::<f32>(), 1.0 };
/// ``` 
pub fn powi<T>( base:T, power:usize ) -> T
    where T: MixedNum + MixedNumConversion<i32> + MixedOps
{
    if power==0
    {
        return T::mixed_from_num(1i32);
    }

    let mut temp:T = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
}

/// Get the sign of the argument with a unit value.
/// Zero is of positive sign.
/// 
/// ## Arguments
/// 
/// * `x`  - The function argument.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U22, FixedI32};
/// 
/// let mut x = FixedI32::<U22>::from_num(-0.2);
/// let mut y = sign(x);
/// assert_eq!{ y.to_num::<f32>(), -1.0 };
/// 
/// x = FixedI32::<U22>::from_num(0.2);
/// y = sign(x); 
/// assert_eq!{ y.to_num::<f32>(), 1.0 };
/// ``` 
pub fn sign<T>( x:T ) -> T
    where T: fixed::traits::FixedSigned
{
    if x<0
    {
        return T::from_num(-1);
    }
    else
    {
        return T::from_num(1);
    }
}
/// Calculate sin(x) using a Taylor approximation of `sin(x)`.
/// 
/// Sin is calculated using the following polynomial:
/// 
/// `sin(x) = x -( x^3/6 )+( x^5/120 )-( x^7/5040 )+( x^9/362880 )`
/// 
/// ## Argument
/// 
/// * `x` - The value to apply the operation to.
/// 
/// `x` must be wrapped to the -π=<x<π range.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U22, FixedI32};
/// 
/// let mut x = FixedI32::<U22>::from_num(0);
/// let mut y = sin(x);
/// assert_eq!{ y.to_num::<f32>(), 0.0 };
/// 
/// x = FixedI32::<U22>::from_num(3.1415/2.0);
/// y = sin(x);
/// assert_eq!{ y.to_num::<f32>(), 1.0000036 };
/// 
/// x = FixedI32::<U22>::from_num(3.1415);
/// y = sin(x);
/// assert_eq!{ y.to_num::<f32>(), 9.250641e-5 };
/// ``` 
/// 
/// ## Comparison and Error
/// 
/// The figure below shows the comparison between the polynomial sine, and the `std::f32::sin` implementation.
/// The Difference between the two is plotted as the error.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/polynomial_sine_comparison.png?raw=true)
/// 
/// The error of the method is compared to the sine implementation in the cordic crate.
/// 
/// The comparison is done for U22 signed fixed point.
/// 
/// The figure below is missing numbers on the y axis, but it is plotted on a linear scale, showing the relative error between the two methods.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/cordic_poly_sine_error_comparison.png?raw=true)
/// 
pub fn sin<T>( x: T ) -> T
    where T: fixed::traits::FixedSigned + MixedNum + MixedNumConversion<i32> + MixedOps
{
    let pi_half:T = <T>::from_num(fixed::consts::PI/2);

    let mut x_: T = x;

    // Ensure that the angle is within the accurate range of the tailor series. 
    if x_ < -pi_half
    {   
        let delta:T = x+pi_half;
        x_ = -pi_half-delta;
    }
    else if pi_half < x_
    {
        let delta:T = x-pi_half;
        x_ = pi_half-delta;
    }

    // Calculate sine by using 
    let mut sinx = x_-( powi(x_,3)/<T>::from_num(6) );
    sinx +=    powi(x_,5)/<T>::from_num(120);
    sinx -= (  powi(x_,7)/<T>::from_num(315)) >> 4;
    sinx += (((powi(x_,9)/<T>::from_num(81))/<T>::from_num(7))/<T>::from_num(5)) >> 7;
    return sinx;
}

/// Calculate cosine using a Taylor approximation of `cos(x)`.
/// 
/// Cos is calculated by adding a phase shift to x and running it through the polynomial sine method.
/// 
/// ## Argument
/// 
/// * `x` - The value to apply the operation to.
/// 
/// `x` is wrapped to the -π=<x<π range in the function.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U18, FixedI32};
/// 
/// let mut x = FixedI32::<U18>::from_num(0);
/// let mut y = cos(x);
/// assert_eq!{ y.to_num::<f32>(), 1.0000038 };
/// 
/// x = FixedI32::<U18>::from_num(3.1415/2.0);
/// y = cos(x);
/// assert_eq!{ y.to_num::<f32>(), 0.00004196167 };
/// ``` 
/// 
/// ## Comparison and Error
/// 
/// The figure below shows the comparison between the polynomial cosine, and the `std::f32::cos` implementation.
/// The Difference between the two is plotted as the error.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/polynomial_cosine_comparison.png?raw=true)
/// 
/// The error of the method is compared to the sine implementation in the cordic crate.
/// 
/// The comparison is done for U22 signed fixed point.
/// 
/// The figure below is missing numbers on the y axis, but it is plotted on a linear scale, showing the relative error between the two methods.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/cordic_poly_cos_error_comparison.png?raw=true)
/// 
pub fn cos<T>( x: T ) -> T
    where T: fixed::traits::FixedSigned + MixedNum + MixedNumSigned + MixedOps + MixedPi
{
    // shift to enable use of more accurate sinepolynomial method.
    let pi_half = <T>::from_num(fixed::consts::PI/2);

    let mut x_shifted = x+pi_half;
    x_shifted = wrap_phase(x_shifted);
    return sin(x_shifted);
}

/// Wrapps θ to the -π=<x<π range.
/// 
/// ## Arguments 
///
/// * `phi` - The unwrapped phase in radians.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U28, FixedI32};
/// 
/// let phi =  FixedI32::<U28>::from_num(6);
/// let wrapped_phi = wrap_phase(phi);
/// assert_eq!{ wrapped_phi.to_num::<f32>(), -0.2831853 };
/// ``` 
pub fn wrap_phase<T>( phi: T ) -> T 
    where T: MixedNum + MixedNumSigned + MixedNumConversion<i32> + MixedOps + MixedPi
{
    let mixed_pi  = T::mixed_pi();
    let tau = T::mixed_from_num(2)*mixed_pi;
 
    let mut temp_scalar = phi;
    
    while temp_scalar < -mixed_pi
    {
        temp_scalar = temp_scalar + tau;
    }
    while mixed_pi <= temp_scalar
    {
        temp_scalar = temp_scalar - tau;
    }
    return temp_scalar;
}