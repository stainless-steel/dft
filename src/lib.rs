//! [Discrete Fourier transform][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

extern crate complex as number;

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
