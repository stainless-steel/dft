//! Transformation of real data.

use number::{Complex, c64};

/// Perform the forward transform.
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
pub fn forward(data: &mut [f64]) {
    use std::f64::consts::PI;
    use std::slice::from_raw_parts_mut;

    let (data, n) = unsafe {
        let n = data.len();
        power_of_two!(n);
        (from_raw_parts_mut(data.as_mut_ptr() as *mut _, n / 2), n / 2)
    };

    ::complex::forward(data);

    let (multiplier, mut factor) = {
        let delta = -PI / n as f64;
        let sine = (0.5 * delta).sin();
        (c64(-2.0 * sine * sine, delta.sin()), c64(0.0, 1.0))
    };

    data[0] = c64(
        data[0].re() + data[0].im(),
        data[0].re() - data[0].im(),
    );

    for i in 1..(n / 2) {
        let j = n - i;

        factor = multiplier * factor + factor;
        let part1 = (data[i] + data[j].conj()) * 0.5;
        let part2 = -(data[i] - data[j].conj()) * 0.5;

        data[i] = part1 + factor * part2;
        data[j] = (part1 - factor * part2).conj();
    }
    data[n / 2] = data[n / 2].conj();
}

/// Expand a compressed representation produced by `real::forward`.
pub fn unpack(data: &[f64]) -> Vec<c64> {
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
        let j = n - i;
        cdata[i] = c64(data[2 * j], -data[2 * j + 1]);
    }

    cdata
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
