/// Calculate atan(y/x) using a polynomial approximation of `atan(y/x)`.
/// 
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
/// let arg = atan::atan2( FixedI32::<U28>::from_num(0.6), FixedI32::<U28>::from_num(0.4) );
/// assert_eq!{ arg.to_num::<f32>(), 0.983006064 };
/// ``` 
pub fn atan2<T>( y: T, x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    let division: T = y/x;
    return atan(division);
}

/// Calculate atan(x) using a polynomial approximation of `atan(x)`.
/// 
/// Utilizes the following polynomial to estimate the angle θ \[radians\].
/// 
/// `atan(x) = (x+0.372003*x^3) / (1+0.703384*x^2 + 0.043562*x^4)`
/// 
/// The method is accurat within 0.003 degrees when |θ|<=π/4.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Etition, IEEE Press, 2012.
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
/// assert_eq!{ arg.to_num::<f32>(), 0.983006064 };
/// ``` 
pub fn atan<T>( x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    return ( (x) + T::from_num(0.372003f32)*super::powi(x,3) ) 
            / (T::from_num(1) + T::from_num(0.703384f32)*super::powi(x,2) + T::from_num(0.043562f32)*super::powi(x,4) );
}


/// Atan polynomial for below function.
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
/// TODO 
fn atan_poly_1<T>( y: T, x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    let phi = (x*y) / ( super::powi(x, 2) + T::from_num( 0.28125 )*super::powi(y, 2) );
    return phi;
}

/// Atan polynomial for below function.
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
/// let arg = atan::atan2_fast( FixedI32::<U28>::from_num(0.6), FixedI32::<U28>::from_num(0.4) );
/// assert_eq!{ arg.to_num::<f32>(), 0.9782037 };
/// ``` 
pub fn atan2_fast<T>( y: T, x: T ) -> T
    where T: fixed::traits::FixedSigned
{
    // Precompute
    let y_abs = y.abs(); 
    let x_abs = x.abs(); 

    // Check which quadrant the result will land in.
    if y.is_positive()
    {
        if x.is_positive()
        {
            // First octant.
            if y_abs - x_abs < T::from_num( 0.0 )
            {
                return atan_poly_1( y, x );
            }
            // Second octant.
            else
            {
                let pi_half = <T>::from_num( fixed::consts::PI/2 );
                return pi_half -atan_poly_2( y, x );
            }
        }
        else
        {
            // Third octant.
            if T::from_num( 0.0 ) <= y_abs - x_abs
            {
                let pi_half = <T>::from_num( fixed::consts::PI/2 );
                return pi_half -atan_poly_2( y, x );
            }
            // Fourth octant.
            else
            {
                let pi = <T>::from_num( fixed::consts::PI );
                return super::sign(y)*pi*atan_poly_1( y, x );
            }
        }
    }
    else
    {
        if x.is_positive()
        {
            // Fifth octant.
            if y_abs - x_abs < T::from_num( 0.0 )
            {
                let pi = <T>::from_num( fixed::consts::PI );
                return super::sign(y)*pi*atan_poly_1( y, x );
            }
            // Sixth octant.
            else
            {
                let pi_half = <T>::from_num( fixed::consts::PI );
                return - pi_half -atan_poly_2( y, x );
            }
        }
        else
        {
            // Seventh octant.
            if T::from_num( 0.0 ) <= y_abs - x_abs
            {
                let pi_half = <T>::from_num( fixed::consts::PI );
                return - pi_half -atan_poly_2( y, x );
            }
            // Eigth octant.
            else
            {
                return atan_poly_1( y, x );
            }
        }
    }
}