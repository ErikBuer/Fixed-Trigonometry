#![crate_name = "fixed_trigonometry"]
#![no_std]

// Use std for test.
#[cfg(all(not(feature = "std"), test))]
extern crate std;

use fixed::traits::Fixed;

/// Rase fixed number to an integer-valued power.
/// - `base^power`.
pub fn powi<T>( base:T, power:usize ) -> T
    where T: fixed::traits::Fixed
{
    let mut temp:T = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
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
/// # Arguments 
///
/// * `y` - Is the argument along the y or imaginary axis.
/// * `x` - Is the argument along the x or real axis.
/// 
/// # Example
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
/// # Arguments 
///
/// * `y` - Is the argument along the y or imaginary axis.
/// * `x` - Is the argument along the x or real axis.
/// 
/// # Example
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
