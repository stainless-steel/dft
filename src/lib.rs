//! [Fast Fourier transform][1] algorithm.
//!
//! [1]: https://en.wikipedia.org/wiki/Fast_Fourier_transform

extern crate complex as number;

macro_rules! power_of_two(
    ($data:expr) => ({
        let n = $data.len();
        if n < 1 || n & (n - 1) != 0 {
            panic!("expected the number of points to be a power of two");
        }
        n
    });
);

pub mod complex;
pub mod real;
