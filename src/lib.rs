#![crate_name = "fixed_trigonometry"]
#![no_std]

// Use std for test.
#[cfg(all(not(feature = "std"), test))]
extern crate std;

use fixed::traits::Fixed;
use fixed::*;

/// Rase fixed number to an integer-valued power.
/// `base^power`.
/// 
/// /// ## Argument
/// 
/// * `base`  - The base number.
/// * `power` - The power to raise 'base' to.
/// 
pub fn powi<T>( base:T, power:usize ) -> T
    where T: fixed::traits::Fixed
{
    let mut temp:T = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
}

/// Take the elemtent-wise sine using a Taylor approximation of sin(x).
/// 
/// Sin is calculated using the following polynomial:
/// 
/// `sin(x) = x -( x^3/6.0 )+( x^5/120.0 )-( x^7/5040.0 )+( x^9/362880.0 )`
/// 
/// ## Argument
/// 
/// * `x` - The value to apply the operation to.
/// 
/// x must be wrapped to the -π=<x<π range.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U18, FixedI32};
/// 
/// let mut x = FixedI32::<U18>::from_num(0);
/// let mut y = sin(x);
/// assert_eq!{ y.to_num::<f32>(), 0.0 };
/// 
/// x = FixedI32::<U18>::from_num(3.14/2.0);
/// y = sin(x);
/// assert_eq!{ y.to_num::<f32>(), 1.0000038 };
/// ``` 
pub fn sin<T>( x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    let pi_half:T = <T>::from_num(fixed::consts::PI/2);

    let mut x_: T = x;

    // Ensure that the angle is within the accurate range of the tailor series. 
    if x_ < -pi_half
    {   
        let delta:T = x+pi_half;
        x_ = -pi_half+delta.abs();
    }
    else if pi_half < x
    {
        let delta:T = x-pi_half;
        x_ = pi_half-delta.abs();
    }

    // Calculate sine by using 
    let mut sinx = x_-( powi(x_,3)/<T>::from_num(6) );
    sinx += powi(x_,5)/<T>::from_num(120);
    sinx -= (powi(x_,7)/<T>::from_num(2520)) >> 1;
    sinx += (powi(x_,9)/<T>::from_num(2835)) >> 7;
    return sinx;
}

/// Calculate atan(y/x) using a polynomial approximation.
/// Utilizes the following polynomial to estimate the angle θ \[radians\].
/// 
/// `atan(y,x) = ((y,x)+0.372003(y,x)^3) / (1+0.703384(y/x)^2 + 0.043562(y/x)^4)`
/// 
/// The method is accurat within 0.003 degrees when |θ|<=π/4.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Etition, IEEE Press, 2012.
/// 
/// ## Arguments 
///
/// * `y` - Is the argument along the y or imaginary axis.
/// * `x` - Is the argument along the x or real axis.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U28, FixedI32};
/// let arg = atan2( FixedI32::<U28>::from_num(0.6), FixedI32::<U28>::from_num(0.4) );
/// assert_eq!{ arg.to_num::<f32>(), 0.983006064 };
/// ``` 
pub fn atan2<T>( y: T, x: T ) -> T
    where T: Fixed
{
    let division: T = y/x;
    return ( (division) + T::from_num(0.372003f32)*powi(division,3) ) 
                        / (T::from_num(1) + T::from_num(0.703384f32)*powi(division,2) + T::from_num(0.043562f32)*powi(division,4) );
}

/// Calculate atan(x) using a polynomial approximation.
/// Utilizes the following polynomial to estimate the angle θ \[radians\].
/// 
/// `atan(x) = ((x)+0.372003(x)^3) / (1+0.703384(x)^2 + 0.043562(x)^4)`
/// 
/// The method is accurat within 0.003 degrees when |θ|<=π/4.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Etition, IEEE Press, 2012.
/// 
/// ## Arguments 
///
/// * `y` - Is the argument along the y or imaginary axis.
/// * `x` - Is the argument along the x or real axis.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U28, FixedI32};
/// 
/// let arg = atan( FixedI32::<U28>::from_num(0.6)/FixedI32::<U28>::from_num(0.4) );
/// assert_eq!{ arg.to_num::<f32>(), 0.983006064 };
/// ``` 
pub fn atan<T>( x: T ) -> T
    where T: Fixed
{
    return ( (x) + T::from_num(0.372003f32)*powi(x,3) ) 
            / (T::from_num(1) + T::from_num(0.703384f32)*powi(x,2) + T::from_num(0.043562f32)*powi(x,4) );
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
