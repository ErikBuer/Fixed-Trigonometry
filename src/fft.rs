
use fixed::traits::FixedSigned;

extern crate alloc;
use alloc::vec::Vec;

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
/// ## Example
/// 
/// ```
/// use fixed_trigonometry::fft::*;
///
/// let mut arr =  vec![0,1,2,3,4,5,6,7];
/// bitreverse_array_order::<i32>(&mut arr);
/// assert_eq!( arr, [0,4,2,6,1,5,3,7] );
/// ```
pub fn bitreverse_array_order<T>( arr: &mut Vec<T> )
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

/// Calculating revers number
fn _reverse( n_capital: usize, n: usize ) -> usize
{
  let mut p: usize = 0;
  
    for j in 1..=log2(n_capital)
    {
        if(n & (1 << (log2(n_capital) - j))) != 0
        {
            p |= 1 << (j - 1);
        }
    }
  return p;
}

/// Using the reverse order in the array
fn _order<T>( f1: &mut Vec<Complex<T>> ) 
    where T: Copy
{
    // Create a copy of f1.
    let mut f2 = Vec::<Complex<T>>::with_capacity( f1.len() );

    for i in 0..f1.len()
    {
        f2.push( f1[_reverse(f1.len(), i)] );
    }
    for j in 0..f1.len()
    {
        f1[j] = f2[j];
    }    
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
    where T: FixedSigned
{
    let temp_a = crate::complex::add(*a,*b);
    //  let temp_b = complex::mul_cartesian(complex::sub(*a, complex::scale_cartesian(T::from_num(2), *b)), w);
    let temp_b = crate::complex::mul_cartesian(crate::complex::sub(*a, *b), w);
    
    *a = temp_a;
    *b = temp_b;
}

/// Calculate the Raddix-2 FFT for fixed point numbers.
/// Scaled for each butterfly computation.
/// Requires input size to be a power of two.
/// 
/// Computed-in-place.
/// Decimation-in-freqency.
/// 
/// The method utilizes fixed point approximations for square root, sine, cosine and atan calculations.
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
/// use fixed::types::extra::U22 as U;
/// use num::complex::Complex;
/// 
/// const N:usize = 4;
/// let mut arr  = vec![ Complex::<F<U>>::new(F::<U>::from_num(1), F::<U>::from_num(0) ); N  ];
///
/// arr[3].re = F::<U>::from_num(0);
/// 
/// fft( &mut arr );
/// assert_eq!( arr, vec![  Complex::<F<U>>::new(F::<U>::from_num(0.75),       F::<U>::from_num(0)          ),
///                         Complex::<F<U>>::new(F::<U>::from_num(-0.0100546), F::<U>::from_num(-0.2503753) ),
///                         Complex::<F<U>>::new(F::<U>::from_num(0.250376),   F::<U>::from_num(0.0)        ),
///                         Complex::<F<U>>::new(F::<U>::from_num(0.0145547),  F::<U>::from_num(0.2507293)  )] );
/// ```
pub fn fft<T>( vec: &mut Vec<Complex<T>> )
    where T: FixedSigned
{
    let n = vec.len();

    // Create heap-allocated vector
    let mut w = Vec::<Complex<T>>::with_capacity(n/2);

    // Calculate Twiddle factor W.
    w.push( Complex::new( <T>::from_num(1), <T>::from_num(0) ) );

    let angle:T = (-<T>::from_num(fixed::consts::TAU)) >> log2(n) as u32;
    for i in 1..n/2
    {
        // Calculate twiddle factor for W_i.
        let real = crate::cos( <T>::from_num(i) *angle );
	    let imag = crate::sin( <T>::from_num(i) *angle );

        w.push( Complex::new( real, imag ) );
    }

    // Number of butterfly computations per block.
    let mut num_butt:   usize = n/2;
    // Number of blocks.
    let mut num_blocks: usize = 1;
    // Bumber of stages (Left-to-right movement).
    let stages = log2(n);

    // Iterate over stages
    for stage in 1..=stages
    {
        // Iterate over blocks.
        for block in 0..num_blocks
        {   
            // Calculate indexes
            let pa = block*n/num_blocks;
            let pb = block*n/num_blocks + num_butt;

            // Iterate over butterflies in current block.
            for j in 0..num_butt
            {
                // Scale values to avoid overflow.
                let mut a = crate::complex::div_cartesian( vec[pa+j], T::from_num(2.0) );
                let mut b = crate::complex::div_cartesian( vec[pb+j], T::from_num(2.0) );
                let w_temp = w[ stage*j ];
                
                butterfly_df( &mut a, &mut b, w_temp );
                
                vec[pa+j] = a;
                vec[pb+j] = b;
            }
        }
        num_blocks = num_blocks * 2;
        num_butt   = num_butt / 2;
    }

    // Decimation-in-freqency.
    //order(vec);    // Bitreverse order
    bitreverse_array_order(vec); // Bitreverse order
}