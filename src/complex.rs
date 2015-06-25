//! Transformation of complex data.

// The implementation is based on:
// http://www.librow.com/articles/article-10

use number::c64;

/// Perform the forward transform.
///
/// The number of points should be a power of two.
pub fn forward(data: &mut [c64]) {
    let n = power_of_two!(data);
    rearrange(data, n);
    perform(data, n, false);
}

/// Perform the backward transform.
///
/// The number of points should be a power of two.
pub fn backward(data: &mut [c64]) {
    let n = power_of_two!(data);
    rearrange(data, n);
    perform(data, n, true);
}

/// Perform the inverse transform.
///
/// The number of points should be a power of two.
pub fn inverse(data: &mut [c64]) {
    let n = power_of_two!(data);
    rearrange(data, n);
    perform(data, n, true);
    scale(data, n);
}

fn rearrange(data: &mut [c64], n: usize) {
    let mut j = 0;
    for i in 0..n {
        if j > i {
            data.swap(i, j);
        }
        let mut mask = n >> 1;
        while j & mask != 0 {
            j &= !mask;
            mask >>= 1;
        }
        j |= mask;
    }
}

fn perform(data: &mut [c64], n: usize, inverse: bool) {
    let sign = if inverse { 1.0 } else { -1.0 };
    let mut step = 1;
    while step < n {
        let jump = step << 1;
        let (multiplier, mut factor) = {
            use std::f64::consts::PI;
            let theta = sign * PI / step as f64;
            let sine = (0.5 * theta).sin();
            (c64(-2.0 * sine * sine, theta.sin()), c64(1.0, 0.0))
        };
        for mut i in 0..step {
            while i < n {
                let j = i + step;
                let product = factor * data[j];
                data[j] = data[i] - product;
                data[i] = data[i] + product;
                i += jump;
            }
            factor = multiplier * factor + factor;
        }
        step <<= 1;
    }
}

fn scale(data: &mut [c64], n: usize) {
    let factor = 1.0 / n as f64;
    for i in 0..n {
        data[i] = data[i] * factor;
    }
}
