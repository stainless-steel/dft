//! [Algorithm][1] to compute the [discrete Fourier transform][2] and its
//! inverse.
//!
//! [1]: https://en.wikipedia.org/wiki/Fast_Fourier_transform
//! [2]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

// The implementation is based on:
// http://www.librow.com/articles/article-10

extern crate complex;

pub use complex::c64;

/// Perform the forward transform.
///
/// The number of points should be a power of two.
///
/// ## Panics
///
/// The function panics if the number of points is not a power of two.
pub fn forward(data: &mut [c64]) {
    let n = data.len();
    if n < 1 || n & (n - 1) != 0 {
        panic!("expected the number of points to be a power of two");
    }
    rearrange(data, n);
    perform(data, n, false);
}

/// Perform the inverse transform.
///
/// The number of points should be a power of two.
///
/// ## Panics
///
/// The function panics if the number of points is not a power of two.
pub fn inverse(data: &mut [c64], scaling: bool) {
    let n = data.len();
    if n < 1 || n & (n - 1) != 0 {
        panic!("expected the number of points to be a power of two");
    }
    rearrange(data, n);
    perform(data, n, true);
    if scaling {
        scale(data, n);
    }
}

#[inline(always)]
fn rearrange(data: &mut [c64], n: usize) {
    let mut target = 0;
    for position in 0..n {
        if target > position {
            data.swap(position, target);
        }
        let mut mask = n >> 1;
        while target & mask != 0 {
            target &= !mask;
            mask >>= 1;
        }
        target |= mask;
    }
}

#[inline(always)]
fn perform(data: &mut [c64], n: usize, inverse: bool) {
    use std::f64::consts::PI;

    let pi = if inverse { PI } else { -PI };
    let mut step = 1;
    while step < n {
        let jump = step << 1;
        let delta = pi / step as f64;
        let sine = (0.5 * delta).sin();
        let multiplier = c64(-2.0 * sine * sine, delta.sin());
        let mut factor = c64(1.0, 0.0);
        for group in 0..step {
            let mut pair = group;
            while pair < n {
                let match_pair = pair + step;
                let product = factor * data[match_pair];
                data[match_pair] = data[pair] - product;
                data[pair] = data[pair] + product;
                pair += jump;
            }
            factor = multiplier * factor + factor;
        }
        step <<= 1;
    }
}

#[inline(always)]
fn scale(data: &mut [c64], n: usize) {
    let factor = 1.0 / n as f64;
    for position in 0..n {
        data[position] = data[position] * factor;
    }
}
