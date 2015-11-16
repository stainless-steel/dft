//! [Discrete Fourier transform][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

extern crate num;

/// A complex number with 64-bit parts.
#[allow(non_camel_case_types)]
pub type c64 = num::Complex<f64>;

/// A plan.
#[derive(Clone, Debug)]
pub struct Plan {
    factors: Vec<c64>,
    operation: Operation,
}

/// An operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    /// The forward transform.
    Forward,
    /// The backward transform.
    Backward,
    /// The inverse transform.
    Inverse,
}

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

impl Plan {
    /// Create a plan.
    #[inline]
    pub fn new(operation: Operation, n: usize) -> Plan {
        let mut factors = Vec::new();
        let sign = if let Operation::Forward = operation { -1.0 } else { 1.0 };
        let mut step = 1;
        while step < n {
            let (multiplier, mut factor) = {
                use std::f64::consts::PI;
                let theta = PI / step as f64;
                let sine = (0.5 * theta).sin();
                (c64!(-2.0 * sine * sine, sign * theta.sin()), c64!(1.0, 0.0))
            };
            for _ in 0..step {
                factors.push(factor);
                factor = multiplier * factor + factor;
            }
            step <<= 1;
        }
        Plan { factors: factors, operation: operation }
    }
}
