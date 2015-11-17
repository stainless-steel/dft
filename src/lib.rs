//! [Discrete Fourier transform][1].
//!
//! The `Transform` trait is responsible for performing the transform. The trait
//! is implemented for real and complex data, which are represented by `[f64]`
//! and `[c64]`, respectively. There are three transformation operations
//! available: forward, backward, and inverse. The desired operation is
//! specified by the `Operation` enumeration passed to the `Plan::new` function,
//! which precomputes auxiliary information needed for `Transform::transform`.
//!
//! ## Example
//!
//! ```
//! use dft::{Operation, Plan, Transform, c64};
//!
//! let size = 512;
//! let mut data = vec![c64::new(42.0, 69.0); size];
//! let plan = Plan::new(Operation::Forward, size);
//!
//! data.transform(&plan);
//! ```
//!
//! ## Real Data
//!
//! When applied to real data, `Transform::transform` works as follows. If the
//! operation is `Operation::Forward`, the data are replaced by the positive
//! frequency half of their complex Fourier transform. The real-valued first and
//! last components of the complex transform are returned as elements `self[0]`
//! and `self[1]`, respectively. If the operation is `Operation::Backward` or
//! `Operation::Inverse`, the function assumes that the data are packed in the
//! format that has just been described. See the reference below for further
//! information on the format.
//!
//! ## References
//!
//! 1. William H. Press, Saul A. Teukolsky, William T. Vetterling, Brian P. Flannery, “Numerical
//!    Recipes 3rd Edition: The Art of Scientific Computing,” Cambridge University Press, 2007.
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

extern crate num;

/// A complex number with 64-bit parts.
#[allow(non_camel_case_types)]
pub type c64 = num::Complex<f64>;

macro_rules! c64(($re:expr, $im:expr) => (::c64::new($re, $im)));

mod complex;
mod real;

pub use real::unpack;

/// A transformation operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    /// The forward transform.
    Forward,
    /// The backward transform.
    Backward,
    /// The inverse transform.
    Inverse,
}

/// A transformation plan.
#[derive(Clone, Debug)]
pub struct Plan {
    size: usize,
    factors: Vec<c64>,
    operation: Operation,
}

/// A type suitable for transformation.
pub trait Transform {
    /// Perform the transform.
    fn transform(&mut self, &Plan);
}

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
