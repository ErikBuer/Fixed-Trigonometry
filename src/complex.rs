use crate::atan;
use num::complex::Complex;
use mixed_num::*;
use mixed_num::traits::*;


/// Cast cartesian complex fixed point number to polar form.
/// 
/// ## Arguments
/// 
/// * `x` - The number transform.
/// 
pub fn to_polar<T>( x: Complex<T> ) -> Polar<T>
    where T:  MixedNum + MixedNumSigned
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
where T: MixedNum + MixedNumSigned
{
    let r_sqr = super::powi( a.re, 2) + super::powi( a.im, 2);
    return trigonometry::sqrt::niirf(r_sqr, 2);
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
    where T: MixedNum + MixedNumSigned + MixedTrigonometry
{
    let theta = crate::wrap_phase(a.theta);
    //let (imag_s, real_s) = cordic::sin_cos( theta );
    let imag_s = theta.mixed_sin();
    let real_s = theta.mixed_cos();

    let c_cartesian = Complex::<T>{
        re: a.r*real_s,
        im: a.r*imag_s
    };
    return c_cartesian;
}

/// Add two complex fixed-point numbers in cartesian form.
pub fn add<T>( a: Complex<T>, b: Complex<T> ) -> Complex<T>
    where T: MixedNum
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
    where T: MixedNum + MixedNumSigned
{
    let c_cartesian = Complex::<T>{
        re: a.re - b.re,
        im: a.im - b.im
    };
    return c_cartesian;
}

/// Multiply fixed-point complex numbers in polar form.
pub fn mul_polar<T>( a: Polar<T>, b: Polar<T> ) -> Polar<T>
    where T: MixedNum + MixedNumSigned
{
    if a.r==T::mixed_from_num(0) || b.r==T::mixed_from_num(0)
    {
        let c = Polar::<T>{
            r:     T::mixed_from_num(0),
            theta: T::mixed_from_num(0)
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
    where T:  MixedNum + MixedNumSigned  + MixedTrigonometry
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
/// let result = Complex::new( FixedI32::<U22>::from_num( -0.019603, ), FixedI32::<U22>::from_num( 1.9959388 ));
/// assert_eq!{ y, result };
/// ```
pub fn powi<T>( base: num::complex::Complex<T>, power:usize ) -> num::complex::Complex<T>
    where T: fixed::traits::FixedSigned + cordic::CordicNumber + MixedNum + MixedNumSigned
{   
    // Calculate raised magnitude.
    let temp:T = super::powi( base.re, 2 ) + super::powi( base.im, 2 );
    let mag:T  = super::powi( trigonometry::sqrt::niirf(temp, 2), power );

    let phi:T  = super::atan::atan2( base.im, base.re )*<T>::from_num(power);

    let (imag_s, real_s) = cordic::sin_cos( phi );

    let real   = mag*real_s;
    let imag   = mag*imag_s;

    return num::complex::Complex::new( real, imag);
}

/// divide a cartesian complex by a real scalar.
/// c = a/b
pub fn div_cartesian<T>( a: Complex<T>, b: T  ) -> Complex<T>
    where T: MixedNum + MixedNumSigned
{
    let mut c = a;

    if b == T::mixed_from_num(0)
    {
        c.re = T::mixed_max_value();
        c.im = T::mixed_max_value();
    }
    else
    {
        c.re = a.re / b;
        c.im = a.im / b;
    }
    return c;
}