use super::*;

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
/// let result = Complex::new( FixedI32::<U22>::from_num( 0.0417783 ), FixedI32::<U22>::from_num( 1.996042 ));
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
