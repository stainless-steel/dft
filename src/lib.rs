//! [Algorithm][1] to compute the [discrete Fourier transform][2] and its
//! inverse.
//!
//! [1]: https://en.wikipedia.org/wiki/Fast_Fourier_transform
//! [2]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

extern crate complex as number;

macro_rules! power_of_two(
    ($n:expr) => (if $n < 1 || $n & ($n - 1) != 0 {
        panic!("expected the number of points to be a power of two");
    });
);

pub mod complex;
pub mod real;

pub use complex::{forward, inverse};
