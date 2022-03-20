extern crate alloc;
use alloc::vec::Vec;

use mixed_num::traits::*;
use num::complex::Complex;

/// Check if x is a power of two.
/// 
/// ## Argument
/// 
/// * `x` - The number to check.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::fft::*;
/// 
/// assert_eq!( is_power_of_two(6), false  );
/// assert_eq!( is_power_of_two(4), true );
/// ```
pub fn is_power_of_two<T>( x: T) -> bool
    where T: num::traits::int::PrimInt
{
    if  T::zero()<x && (x & (x - T::one())) == T::zero()
    {
        return true
    }
    return false
}

/// Calculate the base 2 logarithm of x.
/// 
/// ## Argument
/// 
/// * `x` - The number on which to calculate the base 2 logarithm.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::fft::*;
/// 
/// assert_eq!( log2(8), 3 );
/// ```
pub fn log2( x: usize ) -> usize
{
  let mut k: usize = x;
  let mut i = 0;
  while k != 0
  {
    k >>= 1;
    i+=1;
  }
  return i - 1;
}

/// Bit-reverse the order of the input array. Computed in place.
/// 
/// ## Arguments
/// 
/// * `arr` - A mutable reference to the array to do the computation on, and store the result in.
/// 
fn bitreverse_order<T>( arr: &mut [Complex<T>] )
    where T: core::marker::Copy
{
    let n:usize = arr.len();
    let mut target_index:usize = 0;

    for index in 0..n
    {
        if index<target_index
        {
            let temp = arr[target_index];

            arr[target_index] = arr[index];
            arr[index] = temp;
        }

        let mut mask:usize = n;
        mask >>=1;
        
        // Bit reverse index
        while target_index & mask != 0
        {
	        target_index &= !mask;

            mask >>=1;
        }
        target_index |= mask;
    }
}

/// Calculate the Raddix-2 FFT for fixed point vectors.
/// - Scaled for each butterfly computation.
/// - Requires input size to be a power of two.
/// - Computed-in-place.
/// - Decimation-in-freqency.
/// 
/// The function computes the twiddle factor each time it is called, which is suboptimal for repeating computations.
/// 
/// ## Arguments
/// 
/// * `vec` - A mutable reference to the vector to do the computation on, and store the result in.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::fft::*;
/// 
/// use fixed::FixedI32 as F;
/// use fixed::types::extra::U28 as U;
/// use num::complex::Complex;
/// 
/// const N:usize = 4;
/// let mut arr  = vec![ Complex::<F<U>>::new(F::<U>::from_num(1f32), F::<U>::from_num(0f32) ); N  ];
///
/// arr[3].re = F::<U>::from_num(0f32);
/// 
/// fft( &mut arr );
/// assert_eq!( arr, vec![  Complex::<F<U>>::new(F::<U>::from_num(0.75f32),            F::<U>::from_num(0f32)     ),
///                         Complex::<F<U>>::new(F::<U>::from_num(-0.000000004f32),    F::<U>::from_num(-0.25f32) ),
///                         Complex::<F<U>>::new(F::<U>::from_num(0.25f32),            F::<U>::from_num(0.0f32)   ),
///                         Complex::<F<U>>::new(F::<U>::from_num(0.000000004f32),     F::<U>::from_num(0.25f32)  )] );
/// ```
pub fn fft<T>( array: &mut [Complex<T>] )
    where T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt
{
    // Process fft.
    fft_processor(array, T::mixed_from_num(1));

    // Decimation-in-freqency.
    bitreverse_order(array); // Bitreverse order
}

/// Calculate the Raddix-2 Inverse FFT for fixed point vectors.
/// - Scaled for each butterfly computation.
/// - Requires input size to be a power of two.
/// - Computed-in-place.
/// - Decimation-in-freqency.
/// 
/// The function computes the twiddle factor each time it is called, which is suboptimal for repeating computations.
/// 
/// ## Arguments
/// 
/// * `vec` - A mutable reference to the vector to do the computation on, and store the result in.
/// 
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::fft::*;
/// 
/// use fixed::FixedI32 as F;
/// use fixed::types::extra::U28 as U;
/// use num::complex::Complex;
/// 
/// const N:usize = 4;
/// let mut arr  = vec![ Complex::<F<U>>::new(F::<U>::from_num(1), F::<U>::from_num(0) ); N  ];
///
/// arr[3].re = F::<U>::from_num(0);
/// 
/// ifft( &mut arr );
/// assert_eq!( arr, vec![  Complex::<F<U>>::new(F::<U>::from_num(0.75),         F::<U>::from_num(0.0)      ),
///                         Complex::<F<U>>::new(F::<U>::from_num(-0.000000004), F::<U>::from_num(0.25)),
///                         Complex::<F<U>>::new(F::<U>::from_num(0.25),         F::<U>::from_num(0.0)      ),
///                         Complex::<F<U>>::new(F::<U>::from_num(0.000000004),  F::<U>::from_num(-0.25) )] );
/// ```
pub fn ifft<T>( vec: &mut Vec<Complex<T>> )
    where T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt
{
    // Process fft.
    fft_processor(vec, T::mixed_from_num(-1));
    // Decimation-in-freqency.
    bitreverse_order(vec); // Bitreverse order
}

/// Butterfly computation for decimate-in-frequeny.
/// 
/// ## Arguments
/// 
/// * `a` - input/output.
/// * `b` - input/output.
/// * `w` - twiddle factor.
/// 
fn butterfly_df<T>( a: &mut Complex<T>, b: &mut Complex<T>, w:Complex<T> )
    where T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt
{
    let temp_a = crate::complex::add(*a,*b);
    //  let temp_b = complex::mul_cartesian(complex::sub(*a, complex::scale_cartesian(T::from_num(2), *b)), w);
    let temp_b = crate::complex::mul_cartesian(crate::complex::sub(*a, *b), w);
    
    *a = temp_a;
    *b = temp_b;
}

/// Shared fft processor for fft and ifft.
/// Requires bit-reversion afterwards.
fn fft_processor<T>( array: &mut [Complex<T>], dir: T )
    where T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt
{
    let n = array.len();

    // Create heap-allocated vector
    let mut w = Vec::<Complex<T>>::with_capacity(n/2);

    // Calculate Twiddle factor W.
    w.push( Complex::new( <T>::mixed_from_num(1), <T>::mixed_from_num(0) ) );

    let mut angle:T = dir*-<T>::mixed_pi()*T::mixed_from_num(2);
    for _i in 0..log2(n)
    {
        angle = angle / <T>::mixed_from_num(2);
    }

    let mut phase_inc = angle;
    for _i in 1..n/2
    {
        // Calculate twiddle factor for W_i.
        let imag = phase_inc.mixed_sin();
        let real = phase_inc.mixed_cos();

        phase_inc = phase_inc+angle;

        w.push( Complex::new( real, imag ) );
    }

    // Number of butterfly computations per block.
    let mut num_butt:   usize = n/2;
    // Number of blocks.
    let mut num_blocks: usize = 1;
    // Bumber of stages (Left-to-right movement).
    let stages = log2(n);
    let mut w_idx_step_size = 1;

    // Iterate over stages
    for _stage in 1..=stages
    {
        // Iterate over blocks.
        for block in 0..num_blocks
        {   
            // Calculate indexes
            let pa = block*n/num_blocks;
            let pb = block*n/num_blocks + num_butt;

            // Iterate over butterflies in current block.
            for butt in 0..num_butt
            {
                // Scale values to avoid overflow.
                let mut a = crate::complex::div_cartesian( array[pa+butt], T::mixed_from_num(2) );
                let mut b = crate::complex::div_cartesian( array[pb+butt], T::mixed_from_num(2) );
                
                let w_idx:usize = w_idx_step_size*(butt);
                let w_temp = w[ w_idx ];
                
                butterfly_df( &mut a, &mut b, w_temp );
                
                array[pa+butt] = a;
                array[pb+butt] = b;
            }
        }
        w_idx_step_size *= 2;
        num_blocks *= 2;
        num_butt   /= 2;
    }
}