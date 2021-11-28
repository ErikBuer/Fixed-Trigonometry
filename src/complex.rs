use super::*;

use num::complex::Complex;
use fixed::traits::Fixed;
use fixed::traits::FixedSigned;

/// Cast cartesian complex fixed point number to polar form.
/// 
/// ## Arguments
/// 
/// * `x` - The number transform.
/// 
pub fn to_polar<T>( x: Complex<T> ) -> Polar<T>
    where T: FixedSigned
{
    let c_polar = Polar::<T>{
        r:     abs(x),
        theta: atan::atan2( x.im, x.re )
    };
    return c_polar;
}

/// Calculate the absolute value of the argument.
/// 
/// ## Arguments
/// 
/// * `a` - The argument to apply the function to.
/// 
pub fn abs<T>( a: Complex<T> ) -> T
where T: FixedSigned
{
    let r_sqr = super::powi( a.re, 2) + super::powi( a.im, 2);
    //return r_sqr.sqrt();
    return sqrt::niirf(r_sqr, 2);
}

/// Polar complex nuber.
pub struct Polar<T> {
    pub r: T,
    pub theta: T,
}

/// Cast cartesian complex fixed point number to polar form.
/// 
/// ## Arguments
/// 
/// * `a` - The number transform.
/// 
pub fn to_cartsian<T>( a: Polar<T> ) -> Complex<T>
    where T: FixedSigned + cordic::CordicNumber
{
    let theta = wrap_phase(a.theta);
    let c_cartesian = Complex::<T>{
        re: a.r*cordic::cos(theta),
        im: a.r*cordic::sin(theta)
    };
    return c_cartesian;
}

/// Add two complex fixed-point numbers in cartesian form.
pub fn add<T>( a: Complex<T>, b: Complex<T> ) -> Complex<T>
    where T: Fixed
{
    let c_cartesian = Complex::<T>{
        re: a.re + b.re,
        im: a.im + b.im
    };
    return c_cartesian;
}

/// Subtract b from a.
/// c = a-b
pub fn sub<T>( a: Complex<T>, b: Complex<T> ) -> Complex<T>
    where T: Fixed
{
    let c_cartesian = Complex::<T>{
        re: a.re - b.re,
        im: a.im - b.im
    };
    return c_cartesian;
}

/// Multiply fixed-point complex numbers in polar form.
pub fn mul_polar<T>( a: Polar<T>, b: Polar<T> ) -> Polar<T>
    where T: FixedSigned
{
    if a.r==0 || b.r==0
    {
        let c = Polar::<T>{
            r:     T::from_num(0),
            theta: T::from_num(0)
        };
        return c;
    }
    else
    {
        let c = Polar::<T>{
            r:     a.r*b.r,
            theta: a.theta+b.theta
        };
        return c;
    }
}

/// Multiply two polar numbers by transforming to polar, multiplying and transfomring back.
pub fn mul_cartesian<T>( a: Complex<T>, b: Complex<T> ) -> Complex<T>
    where T: FixedSigned + cordic::CordicNumber
{
    let a_pol = to_polar(a);
    let b_pol = to_polar(b);

    let c_pol = mul_polar(a_pol, b_pol);
    return to_cartsian(c_pol);
}

/// Rase a complex fixed-point number to an real-valued integer power.
/// `base^power`.
/// 
/// ## Arguments
/// 
/// * `base`  - The complex, fixed-point base number.
/// * `power` - The power to raise 'base' to.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry as trig;
/// use fixed::{types::extra::U22, FixedI32};
/// use num::complex::Complex;
/// 
/// 
/// let x = Complex::new( FixedI32::<U22>::from_num(1), FixedI32::<U22>::from_num(1) );
/// let y = trig::complex::powi( x, 2 );
/// 
/// let result = Complex::new( FixedI32::<U22>::from_num( -0.019602, ), FixedI32::<U22>::from_num( 1.995946 ));
/// assert_eq!{ y, result };
/// ```
pub fn powi<T>( base: num::complex::Complex<T>, power:usize ) -> num::complex::Complex<T>
    where T: fixed::traits::FixedSigned
{   
    // Calculate raised magnitude.
    let temp:T = super::powi( base.re, 2 ) + super::powi( base.im, 2 );
    let mag:T  = super::powi( sqrt::niirf(temp, 2), power );

    let phi:T  = super::atan::atan2( base.im, base.re )*<T>::from_num(power);

    let real   = mag*super::cos(phi);
    let imag   = mag*super::sin(phi);

    return num::complex::Complex::new( real, imag);
}

/// divide a cartesian complex by a real scalar.
/// c = a/b
pub fn div_cartesian<T>( a: Complex<T>, b: T  ) -> Complex<T>
    where T: FixedSigned
{
    let mut c = a;

    if b == 0
    {
        c.re = T::MAX;
        c.im = T::MAX;
    }
    else
    {
        c.re = a.re / b;
        c.im = a.im / b;
    }
    return c;
}