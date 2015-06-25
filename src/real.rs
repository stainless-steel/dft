//! Transformation of real data.

use number::{Complex, c64};

macro_rules! reinterpret(
    ($data:ident) => (unsafe {
        use std::slice::from_raw_parts_mut;
        let n = power_of_two!($data);
        (from_raw_parts_mut($data.as_mut_ptr() as *mut _, n / 2), n / 2)
    });
);

/// Perform the forward transform.
///
/// The number of points should be a power of two. The data are replaced by the
/// positive frequency half of their complex Fourier transform. The real-valued
/// first and last components of the complex transform are returned as elements
/// `data[0]` and `data[1]`, respectively.
///
/// ## References
///
/// 1. William H. Press, Saul A. Teukolsky, William T. Vetterling, Brian P.
///    Flannery, “Numerical Recipes 3rd Edition: The Art of Scientific
///    Computing,” Cambridge University Press, 2007.
pub fn forward(data: &mut [f64]) {
    let (data, n) = reinterpret!(data);
    ::complex::forward(data);
    compose(data, n, false);
}

/// Perform the backward transform.
///
/// The number of points should be a power of two. The data should be packed as
/// described in `real::forward`.
pub fn backward(data: &mut [f64]) {
    let (data, n) = reinterpret!(data);
    compose(data, n, true);
    ::complex::backward(data);
}

/// Perform the inverse transform.
///
/// The number of points should be a power of two. The data should be packed as
/// described in `real::forward`.
pub fn inverse(data: &mut [f64]) {
    let (data, n) = reinterpret!(data);
    compose(data, n, true);
    ::complex::inverse(data);
}

/// Unpack a compressed representation produced by `real::forward`.
pub fn unpack(data: &[f64]) -> Vec<c64> {
    let n = power_of_two!(data);

    let mut cdata = Vec::with_capacity(n);
    unsafe { cdata.set_len(n) };

    cdata[0] = c64(data[0], 0.0);
    for i in 1..(n / 2) {
        cdata[i] = c64(data[2 * i], data[2 * i + 1]);
    }
    cdata[n / 2] = c64(data[1], 0.0);
    for i in (n / 2 + 1)..n {
        cdata[i] = cdata[n - i].conj();
    }

    cdata
}

fn compose(data: &mut [c64], n: usize, inverse: bool) {
    data[0] = c64(data[0].re() + data[0].im(), data[0].re() - data[0].im());
    if inverse {
        data[0] = data[0] * 0.5;
    }

    let sign = if inverse { 1.0 } else { -1.0 };
    let (multiplier, mut factor) = {
        use std::f64::consts::PI;
        let theta = sign * PI / n as f64;
        let sine = (0.5 * theta).sin();
        (c64(-2.0 * sine * sine, theta.sin()), c64(1.0, 0.0))
    };
    for i in 1..(n / 2) {
        let j = n - i;
        factor = multiplier * factor + factor;
        let part1 = (data[i] + data[j].conj()) * 0.5;
        let part2 = (data[i] - data[j].conj()) * 0.5;
        let product = c64(0.0, sign) * factor * part2;
        data[i] = part1 + product;
        data[j] = (part1 - product).conj();
    }

    data[n / 2] = data[n / 2].conj();
}

#[cfg(test)]
mod tests {
    use number::c64;

    #[test]
    fn unpack() {
        let data = (0..4).map(|i| (i + 1) as f64).collect::<Vec<_>>();
        assert_eq!(super::unpack(&data), vec![
            c64(1.0, 0.0), c64(3.0, 4.0), c64(2.0, 0.0), c64(3.0, -4.0),
        ]);

        let data = (0..8).map(|i| (i + 1) as f64).collect::<Vec<_>>();
        assert_eq!(super::unpack(&data), vec![
            c64(1.0, 0.0), c64(3.0, 4.0), c64(5.0, 6.0), c64(7.0, 8.0),
            c64(2.0, 0.0), c64(7.0, -8.0), c64(5.0, -6.0), c64(3.0, -4.0),
        ]);
    }
}
