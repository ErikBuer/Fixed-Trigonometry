/// A fast implementation of the square root using the Nonlinear IIR Filter (NIIRF) method \[1\].
/// 
/// \[1\] N.Mikami et al., A new DSP-oriented algorithm for calculation of square root using a non-linear digital filter, IEEE Trans. on Signal Processing, July 1992, pp. 1663-1669.
/// \[2\] R. G. Lyons, Streamlining Digital Signal Processing, Second Edition, IEEE Press, 2012.
/// 
/// ## Arguments 
///
/// * `x` - The argument which to calculate the root of.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U28, FixedI32};
/// 
/// let x =  FixedI32::<U28>::from_num(6);
/// let y = sqrt::niirf(x);
/// assert_eq!{ y.to_num::<f32>(), -0.2831853 };
/// ``` 
pub fn niirf<T>( x: T ) -> T 
    where T: fixed::traits::FixedSigned
{
    return x;
}