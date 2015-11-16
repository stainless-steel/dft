//! Transformation of complex data.

// The implementation is based on:
// http://www.librow.com/articles/article-10

use {Operation, Plan, c64};

/// Perform the transform.
///
/// The number of points should be a power of two.
pub fn transform(data: &mut [c64], plan: &Plan) {
    let n = power_of_two!(data);
    rearrange(data, n);
    calculate(data, n, &plan.factors);
    if let Operation::Inverse = plan.operation {
        scale(data, n);
    }
}

fn calculate(data: &mut [c64], n: usize, factors: &[c64]) {
    let mut k = 0;
    let mut step = 1;
    while step < n {
        let jump = step << 1;
        for mut i in 0..step {
            while i < n {
                let j = i + step;
                let product = factors[k] * data[j];
                data[j] = data[i] - product;
                data[i] = data[i] + product;
                i += jump;
            }
            k += 1;
        }
        step <<= 1;
    }
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

fn scale(data: &mut [c64], n: usize) {
    let factor = 1.0 / n as f64;
    for i in 0..n {
        data[i] = data[i].scale(factor);
    }
}
