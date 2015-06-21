use std::ops::{Add, Mul, Sub};

/// A complex number.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct c64(pub f64, pub f64);

impl Add for c64 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: c64) -> c64 {
        c64(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul for c64 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: c64) -> c64 {
        c64(self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
    }
}

impl Mul<f64> for c64 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> c64 {
        c64(self.0 * rhs, self.1 * rhs)
    }
}

impl Sub for c64 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: c64) -> c64 {
        c64(self.0 - rhs.0, self.1 - rhs.1)
    }
}
