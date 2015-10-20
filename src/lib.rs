//! [Discrete Fourier transform][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

extern crate num;

/// A complex number with 64-bit parts.
#[allow(non_camel_case_types)]
pub type c64 = num::Complex<f64>;

macro_rules! c64(
    ($re:expr, $im:expr) => (::c64::new($re, $im));
);

macro_rules! power_of_two(
    ($data:expr) => ({
        let n = $data.len();
        if !n.is_power_of_two() {
            panic!("expected the number of points to be a power of two");
        }
        n
    });
);

pub mod complex;
pub mod real;
