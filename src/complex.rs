use fixed_sqrt::FixedSqrt;

/// Rase a complex number to an real-valued integer power.
/// `base^power`.
/// 
/// ## Arguments
/// 
/// * `base`  - The base number.
/// * `power` - The power to raise 'base' to.
/// 
pub fn powi<T>( base: num::complex::Complex<T>, power:usize ) -> num::complex::Complex<T>
    where T: fixed_sqrt::FixedSqrt + fixed::traits::FixedSigned
{   
    // Calculate raised magnitude.
    let temp:T = super::powi( base.re, 2 ) + super::powi( base.im, 2 );
    let mag:T  = super::powi( temp.sqrt(), power );

    let phi:T  = super::atan2( base.im, base.re )*<T>::from_num(power);

    let real   = mag*super::cos(phi);
    let imag   = mag*super::sin(phi);

    return num::complex::Complex::new( real, imag);
}
