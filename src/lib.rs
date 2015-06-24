//! [Algorithm][1] to compute the [discrete Fourier transform][2] and its
//! inverse.
//!
//! [1]: https://en.wikipedia.org/wiki/Fast_Fourier_transform
//! [2]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

// The implementation is based on:
// http://www.librow.com/articles/article-10

extern crate complex;

pub use complex::c64;

macro_rules! power_of_two(
    ($n:expr) => (if $n < 1 || $n & ($n - 1) != 0 {
        panic!("expected the number of points to be a power of two");
    });
);

/// Perform the forward transform.
///
/// The number of points should be a power of two.
pub fn forward(data: &mut [c64]) {
    let n = data.len();
    power_of_two!(n);
    rearrange(data, n);
    perform(data, n, false);
}

/// Perform the forward transform of real data.
///
/// The number of points should be a power of two. The data are replaced by the
/// positive frequency half of their complex Fourier transform. The real-valued
/// first and last components of the complex transform are returned as elements
/// data[0] and data[1], respectively.
///
/// ## References
///
/// 1. William H. Press, Saul A. Teukolsky, William T. Vetterling, Brian P.
///    Flannery, “Numerical Recipes 3rd Edition: The Art of Scientific
///    Computing,” Cambridge University Press, 2007.
pub fn forward_real(data: &mut [f64]) {
    use std::f64::consts::PI;
    use std::slice::from_raw_parts_mut;

    let n = data.len();
    power_of_two!(n);

    forward(unsafe {
        from_raw_parts_mut(data.as_mut_ptr() as *mut _, n / 2)
    });

    let (c1, c2) = (0.5, -0.5);
    let (wpr, wpi) = {
        let theta = -PI / (n / 2) as f64;
        let temp = (0.5 * theta).sin();
        (-2.0 * temp * temp, theta.sin())
    };
    let mut wr = 1.0 + wpr;
    let mut wi = wpi;
    for i in 1..(n / 2 / 2) {
        let i1 = i + i;
        let i2 = 1 + i1;
        let i3 = n - i1;
        let i4 = 1 + i3;

        let h1r =  c1 * (data[i1] + data[i3]);
        let h1i =  c1 * (data[i2] - data[i4]);
        let h2r = -c2 * (data[i2] + data[i4]);
        let h2i =  c2 * (data[i1] - data[i3]);

        data[i1] =  h1r + wr * h2r - wi * h2i;
        data[i2] =  h1i + wr * h2i + wi * h2r;
        data[i3] =  h1r - wr * h2r + wi * h2i;
        data[i4] = -h1i + wr * h2i + wi * h2r;

        let temp = wr;
        wr = wr * wpr - wi * wpi + wr;
        wi = wi * wpr + temp * wpi + wi;
    }
    data[n / 2 + 1] *= -1.0;
    let temp = data[0];
    data[0] = temp + data[1];
    data[1] = temp - data[1];
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

/// Expand a compressed representation produced by `forward_real`.
pub fn unpack_real(data: &[f64]) -> Vec<c64> {
    let n = data.len();
    power_of_two!(n);

    let mut cdata = Vec::with_capacity(n);
    unsafe { cdata.set_len(n) };

    cdata[0] = c64(data[0], 0.0);
    for i in 1..(n / 2) {
        cdata[i] = c64(data[2 * i], data[2 * i + 1]);
    }
    cdata[n / 2] = c64(data[1], 0.0);
    for i in (n / 2 + 1)..n {
        cdata[i] = c64(data[2 * (n - i)], -data[2 * (n - i) + 1]);
    }

    cdata
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

fn scale(data: &mut [c64], n: usize) {
    let factor = 1.0 / n as f64;
    for position in 0..n {
        data[position] = data[position] * factor;
    }
}

#[cfg(test)]
mod tests {
    use c64;

    #[test]
    fn unpack_real() {
        let data = (0..4).map(|i| (i + 1) as f64).collect::<Vec<_>>();
        assert_eq!(::unpack_real(&data), vec![
            c64(1.0, 0.0), c64(3.0, 4.0), c64(2.0, 0.0), c64(3.0, -4.0),
        ]);

        let data = (0..8).map(|i| (i + 1) as f64).collect::<Vec<_>>();
        assert_eq!(::unpack_real(&data), vec![
            c64(1.0, 0.0), c64(3.0, 4.0), c64(5.0, 6.0), c64(7.0, 8.0),
            c64(2.0, 0.0), c64(7.0, -8.0), c64(5.0, -6.0), c64(3.0, -4.0),
        ]);
    }
}
