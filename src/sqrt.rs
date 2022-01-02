use mixed_num;

/// A fast implementation of the square root using the Nonlinear IIR Filter (NIIRF) method \[1\].
/// 
/// Only valid for positive values of x.
/// Accurate to 5*10⁻⁴ with two iterations \[2\].
/// 
/// The structure of the estimator is illustrated below \[1\].
/// 
/// ![Alt version](https://raw.githubusercontent.com/ErikBuer/Fixed-Trigonometry/main/figures/niirf.svg)
/// 
/// The method utilizes a lookup-table for the acceleration factor β.
/// 
/// β(x) can be calculated from the following formula, yielding even greater accuracy at a computational cost.
/// ```Julia
/// β(x) = 0.763x^2-1.5688x+1.314 
/// ```
/// 
/// \[1\] N.Mikami et al., A new DSP-oriented algorithm for calculation of square root using a non-linear digital filter, IEEE Trans. on Signal Processing, July 1992, pp. 1663-1669.
/// 
/// \[2\] R. G. Lyons, Streamlining Digital Signal Processing, Second Edition, IEEE Press, 2012.
/// 
/// ## Arguments 
///
/// * `x`          - The argument which to calculate the root of.
/// * `iterations` - The number of iterations to run (start with 2).
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::*;
/// use fixed::{types::extra::U28, FixedI32};
/// 
/// let mut x =  FixedI32::<U28>::from_num(0.23);
/// let mut y = sqrt::niirf(x, 2);
/// assert_eq!{ y.to_num::<f32>(), 0.47960657f32 };
/// 
/// x = FixedI32::<U28>::from_num(1.6);
/// y = sqrt::niirf(x, 2);
/// assert_eq!{ y.to_num::<f32>(), 1.2644687f32 };
/// 
/// x = FixedI32::<U28>::from_num(0.0);
/// y = sqrt::niirf(x, 2);
/// assert_eq!{ y.to_num::<f32>(), 0.0f32 };
/// ``` 
/// 
/// ## Accuracy and Comparison
/// 
/// The figure below shows error of the NIIRF implementation, compared to the `std::f32::sqrt` implementation.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/niirf_sqrt_comparison.png?raw=true)
/// 
/// Another fixed point implementation of the square root can be found in the cordic crate. 
/// 
/// Below is the error comparison between the two implementations.
/// 
/// ![Alt version](https://github.com/ErikBuer/Fixed-Trigonometry/blob/main/figures/sqrt_error_comparison.png?raw=true)
pub fn niirf<T>( x: T, iterations: usize ) -> T 
    where T: mixed_num::MixedNum + mixed_num::MixedNumSigned
{
    if x == T::mixed_from_num(0)
    {
        return T::mixed_from_num(0);
    }

    // Only works with real numbers.
    let mut x_ = x.mixed_abs();

    // First we normalize x to the range 0.25 =< x < 1.
    let mut norm:i32 = 0; // Number of normalizations.
    while x_< T::mixed_from_num(0.25)
    {
        norm -=1;
        x_ = x_*T::mixed_from_num(4);
    }
    while T::mixed_from_num(1.0) <= x_
    {
        norm +=1;
        x_ = x_/T::mixed_from_num(4);
    }

    /// LUT for getting the acceleration factor β.
    fn beta<T>( x: T) -> T
        where T: mixed_num::MixedNum + mixed_num::MixedNumSigned
    {   
        // There is one β value for each of the 12 regions in the range 4/16 to 16/16.
        let beta_values = [  T::mixed_from_num(0.961914),
                                    T::mixed_from_num(0.840332),
                                    T::mixed_from_num(0.782715),
                                    T::mixed_from_num(0.734869),
                                    T::mixed_from_num(0.691406),
                                    T::mixed_from_num(0.654297),
                                    T::mixed_from_num(0.622070),
                                    T::mixed_from_num(0.595215),
                                    T::mixed_from_num(0.573731),
                                    T::mixed_from_num(0.556152),
                                    T::mixed_from_num(0.516113),
                                    T::mixed_from_num(0.502930)];

        if x < T::mixed_from_num(5.0/16.0) {
            return beta_values[0];
        }
        else if x < T::mixed_from_num(6.0/16.0) {
            return beta_values[1];
        }
        else if x < T::mixed_from_num(7.0/16.0) {
            return beta_values[2];
        }
        else if x < T::mixed_from_num(8.0/16.0) {
            return beta_values[3];
        }
        else if x < T::mixed_from_num(9.0/16.0) {
            return beta_values[4];
        }
        else if x < T::mixed_from_num(10.0/16.0) {
            return beta_values[5];
        }
        else if x < T::mixed_from_num(11.0/16.0) {
            return beta_values[6];
        }
        else if x < T::mixed_from_num(12.0/16.0) {
            return beta_values[7];
        }
        else if x < T::mixed_from_num(13.0/16.0) {
            return beta_values[8];
        }
        else if x < T::mixed_from_num(14.0/16.0) {
            return beta_values[9];
        }
        else if x < T::mixed_from_num(15.0/16.0) {
            return beta_values[10];
        }
        else {
            return beta_values[11];
        }
    }

    // Estimate the square root for x, when 0.25 =< x < 1.
    let mut y = (T::mixed_from_num(2)*x_)/T::mixed_from_num(3) + T::mixed_from_num(0.354167);   // y0
    for _n in 1..iterations
    {
        y = beta(x_)*(x_-y.mixed_powi(2))+y;
    }

    // Denormalize the solution.
    if 0 < norm
    {
        y = y*T::mixed_from_num(2).mixed_powi(norm as i32);
    }
    else if norm < 0
    {
        y = y/T::mixed_from_num(2).mixed_powi(norm as i32);
    }
    return y;
}