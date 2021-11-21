/// Atan polynomial for below function.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Edition, IEEE Press, 2012.
/// 
/// ## Arguments 
///
/// * `y` - Is the argument along the y or imaginary axis.
/// * `x` - Is the argument along the x or real axis.
/// 
/// ## Example
/// 
/// TODO 
fn atan_poly_1<T>( y: T, x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    let phi = (x*y) / ( super::powi(x, 2) + T::from_num( 0.28125 )*super::powi(y, 2) );
    return phi;
}

/// Atan polynomial for below function.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Edition, IEEE Press, 2012.
/// 
/// ## Arguments 
///
/// * `y` - Is the argument along the y or imaginary axis.
/// * `x` - Is the argument along the x or real axis.
/// 
/// ## Example
/// 
/// TODO 
fn atan_poly_2<T>( y: T, x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    let phi     = (x*y) / ( super::powi(y, 2) + T::from_num( 0.28125 )*super::powi(x, 2) );
    return phi;
}


/// Calculate atan2(y,x) using a selection of polynomial approximations, one for each octant in the unit circle.
/// 
/// The method is accurat within 0.028 degrees.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Edition, IEEE Press, 2012.
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
/// let arg = atan::atan2( FixedI32::<U28>::from_num(0.6), FixedI32::<U28>::from_num(0.4) );
/// assert_eq!{ arg.to_num::<f32>(), 0.9782037 };
/// 
/// let arg = atan::atan2( FixedI32::<U28>::from_num(0.0), FixedI32::<U28>::from_num(0.4) );
/// assert_eq!{ arg.to_num::<f32>(), 0.0 };
/// 
/// let arg = atan::atan2( FixedI32::<U28>::from_num(0.0), FixedI32::<U28>::from_num(0.0) );
/// assert_eq!{ arg.to_num::<f32>(), 0.0 };
/// ``` 
/// 
/// ## Comparisons
/// 
/// The figure below shows the comparison between the various implementations and the `std::f32::atan` implementation.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/atan2_comparisons.png?raw=true)
pub fn atan2<T>( y: T, x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    // Precompute
    let y_abs = y.abs();
    let x_abs = x.abs();

    let pi      = <T>::from_num( fixed::consts::PI );
    let pi_half = <T>::from_num( fixed::consts::PI/2 );

    // Check which quadrant the result will land in.
    if  y == 0
    {
        if  x.is_negative()
        {
            return T::from_num( fixed::consts::PI );
        }
        else
        {
            return T::from_num( 0 );
        }
    }
    else if y.is_positive()
    {
        if  x == 0
        {
            return T::from_num( fixed::consts::PI/2 );
        }
        else if x.is_positive()
        {
            // First octant.
            if y_abs - x_abs < T::from_num( 0.0 )
            {
                return atan_poly_1( y, x );
            }
            // Second octant.
            else
            {
                return pi_half -atan_poly_2( y, x );
            }
        }
        else
        {
            // Third octant.
            if T::from_num( 0.0 ) <= y_abs - x_abs
            {
                return pi_half - atan_poly_2( y, x );
            }
            // Fourth octant.
            else
            {
                return pi + atan_poly_1( y, x );
            }
        }
    }
    else
    {
        if  x == 0
        {
            return -T::from_num( fixed::consts::PI/2 );
        }
        if x.is_positive()
        {
            // Fifth octant.
            if y_abs - x_abs < T::from_num( 0.0 )
            {
                return atan_poly_1( y, x );
            }
            // Sixth octant.
            else
            {
                return - pi_half -atan_poly_2( y, x );
            }
        }
        else
        {
            // Seventh octant.
            if T::from_num( 0.0 ) <= y_abs - x_abs
            {
                return -pi_half - atan_poly_2( y, x );
            }
            // Eigth octant.
            else
            {
                return -pi + atan_poly_1( y, x );
            }
        }
    }
}

/// Calculate atan(x) using a polynomial approximation of `atan(x)`.
/// 
/// Utilizes a polynomial methodto estimate the angle θ \[radians\].
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Edition, IEEE Press, 2012.
/// 
/// ## Arguments 
///
/// * `x` - Is the function argument.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U28, FixedI32};
/// 
/// let arg = atan::atan( FixedI32::<U28>::from_num(0.6)/FixedI32::<U28>::from_num(0.4) );
/// assert_eq!{ arg.to_num::<f32>(), 0.9782037 };
/// ``` 
/// ## Comparisons
/// 
/// The figure below shows the comparison between the various implementations and the `std::f32::atan` implementation.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/atan_comparisons.png?raw=true)
/// 
pub fn atan<T>( x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    return atan2(x,T::from_num(1));
}