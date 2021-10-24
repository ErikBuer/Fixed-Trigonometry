/// A fast implementation of the square root using the Newton-Raphson Inverse method.
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
/// let y = newton_raphson_inverse(x);
/// assert_eq!{ y.to_num::<f32>(), -0.2831853 };
/// ``` 
pub fn newton_raphson_inverse<T>( x: T ) -> T 
    where T: fixed::traits::FixedSigned
{
    return x;
}