//! Transformation of complex data.

// The implementation is based on:
// http://www.librow.com/articles/article-10

use number::c64;

/// Perform the forward transform.
///
/// The number of points should be a power of two.
pub fn forward(data: &mut [c64]) {
    let n = data.len();
    power_of_two!(n);
    rearrange(data, n);
    perform(data, n, false);
}

/// Perform the inverse transform.
///
/// The number of points should be a power of two.
pub fn inverse(data: &mut [c64], scaling: bool) {
    let n = data.len();
    power_of_two!(n);
    rearrange(data, n);
    perform(data, n, true);
    if scaling {
        scale(data, n);
    }
}

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

fn perform(data: &mut [c64], n: usize, inverse: bool) {
    use std::f64::consts::PI;

    let pi = if inverse { PI } else { -PI };
    let mut step = 1;
    while step < n {
        let jump = step << 1;
        let (multiplier, mut factor) = {
            let delta = pi / step as f64;
            let sine = (0.5 * delta).sin();
            (c64(-2.0 * sine * sine, delta.sin()), c64(1.0, 0.0))
        };
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

fn scale(data: &mut [c64], n: usize) {
    let factor = 1.0 / n as f64;
    for position in 0..n {
        data[position] = data[position] * factor;
    }
}
