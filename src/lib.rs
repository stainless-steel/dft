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
    size: usize,
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

macro_rules! c64(($re:expr, $im:expr) => (::c64::new($re, $im)));

pub mod complex;
pub mod real;

impl Plan {
    /// Create a plan for a specific operation and number of points.
    ///
    /// The number of points should be a power of two.
    pub fn new(operation: Operation, size: usize) -> Plan {
        use std::f64::consts::PI;

        assert!(size.is_power_of_two(), "the number of points should be a power of two");

        let mut factors = vec![];
        let sign = if let Operation::Forward = operation { -1.0 } else { 1.0 };
        let mut step = 1;
        while step < size {
            let (multiplier, mut factor) = {
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

        Plan { size: size, factors: factors, operation: operation }
    }
}
