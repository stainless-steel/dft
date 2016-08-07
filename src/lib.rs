//! [Discrete Fourier transform][1].
//!
//! The `Transform` trait is responsible for performing the transform. The trait
//! is implemented for real and complex data, which are represented by `[f64]`
//! and `[c64]`, respectively. There are three operations available: forward,
//! backward, and inverse. The desired operation is specified by the `Operation`
//! enumeration passed to the `Plan::new` function, which precomputes auxiliary
//! information needed for `Transform::transform`. All the operations are
//! preformed in place.
//!
//! When applied to real data, `Transform::transform` works as follows. If the
//! operation is `Operation::Forward`, the data are replaced by the positive
//! frequency half of their complex Fourier transform. The first and last
//! components of the complex transform, which are real, are stored in `self[0]`
//! and `self[1]`, respectively. Regarding the other two operations, the data
//! are assumed to be packed in the aforementioned format. See the reference
//! below for further details.
//!
//! ## Example
//!
//! ```
//! use dft::{Operation, Plan, c64};
//!
//! let plan = Plan::new(Operation::Forward, 512);
//! let mut data = vec![c64::new(42.0, 69.0); 512];
//! dft::transform(&mut data, &plan);
//! ```
//!
//! ## References
//!
//! 1. W. Press, S. Teukolsky, W. Vetterling, and B. Flannery, “Numerical
//! Recipes 3rd Edition: The Art of Scientific Computing,” Cambridge University
//! Press, 2007.
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

extern crate num_complex as num;

/// A complex number with 64-bit parts.
#[allow(non_camel_case_types)]
pub type c64 = num::Complex<f64>;

macro_rules! c(($re:expr, $im:expr) => (::num::Complex::new($re, $im)));

mod complex;
mod real;

pub use real::unpack;

/// A transform operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    /// A forward transform.
    Forward,
    /// A backward transform.
    Backward,
    /// A inverse transform.
    Inverse,
}

/// A transform plan.
#[derive(Clone, Debug)]
pub struct Plan {
    size: usize,
    factors: Vec<c64>,
    operation: Operation,
}

/// The transform.
pub trait Transform {
    /// Perform the transform.
    fn transform(&mut self, &Plan);
}

impl Plan {
    /// Create a plan for a specific operation and specific number of points.
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
                (c!(-2.0 * sine * sine, sign * theta.sin()), c!(1.0, 0.0))
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

/// Perform the transform.
///
/// The function is a shortcut for `Transform::transform`.
#[inline(always)]
pub fn transform<T: Transform + ?Sized>(data: &mut T, plan: &Plan) {
    Transform::transform(data, plan);
}
