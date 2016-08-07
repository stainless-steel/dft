use num_complex::Complex;
use num_traits::Float;
use std::slice::from_raw_parts_mut;

use {Operation, Plan, Transform};

impl<T> Transform<T> for [T] where T: Float {
    fn transform(&mut self, plan: &Plan<T>) {
        let n = self.len();
        assert!(n == plan.n);
        let h = n >> 1;
        if h == 0 {
            return;
        }
        let data = unsafe { from_raw_parts_mut(self.as_mut_ptr() as *mut _, h) };
        match plan.operation {
            Operation::Forward => {
                data.transform(plan);
                compose(data, h, &plan.factors, false);
            },
            Operation::Backward | Operation::Inverse => {
                compose(data, h, &plan.factors, true);
                data.transform(plan);
            },
        }
    }
}

impl<T> Transform<T> for Vec<T> where T: Float {
    #[inline(always)]
    fn transform(&mut self, plan: &Plan<T>) {
        Transform::transform(&mut self[..], plan)
    }
}

/// Unpack the result produced by the forward transform applied to real data.
///
/// The function decodes the result of an application of `Transform::transform`
/// with `Operation::Forward` to real data. See the top-level description of the
/// crate for further details.
pub fn unpack<T>(data: &[T]) -> Vec<Complex<T>> where T: Float {
    let n = data.len();
    assert!(n.is_power_of_two());
    let zero = T::zero();
    let h = n >> 1;
    let mut result = Vec::with_capacity(n);
    unsafe { result.set_len(n) };
    result[0] = c!(data[0], zero);
    if h == 0 {
        return result;
    }
    for i in 1..h {
        result[i] = c!(data[2 * i], data[2 * i + 1]);
    }
    result[h] = c!(data[1], zero);
    for i in (h + 1)..n {
        result[i] = result[n - i].conj();
    }
    result
}

#[inline(always)]
fn compose<T>(data: &mut [Complex<T>], n: usize, factors: &[Complex<T>], inverse: bool)
    where T: Float
{
    let one = T::one();
    let half = (one + one).recip();
    let h = n >> 1;
    data[0] = c!(data[0].re + data[0].im, data[0].re - data[0].im);
    if inverse {
        data[0] = data[0].scale(half);
    }
    if h == 0 {
        return;
    }
    let m = factors.len();
    let zero = T::zero();
    let sign = if inverse { one } else { -one };
    for i in 1..h {
        let j = n - i;
        let part1 = data[i] + data[j].conj();
        let part2 = data[i] - data[j].conj();
        let product = c!(zero, sign) * factors[m - j] * part2;
        data[i] = (part1 + product).scale(half);
        data[j] = (part1 - product).scale(half).conj();
    }
    data[h] = data[h].conj();
}

#[cfg(test)]
mod tests {
    #[test]
    fn unpack() {
        let data = (0..4).map(|i| (i + 1) as f64).collect::<Vec<_>>();
        assert_eq!(super::unpack(&data), vec![
            c!(1.0, 0.0), c!(3.0, 4.0), c!(2.0, 0.0), c!(3.0, -4.0),
        ]);

        let data = (0..8).map(|i| (i + 1) as f64).collect::<Vec<_>>();
        assert_eq!(super::unpack(&data), vec![
            c!(1.0, 0.0), c!(3.0,  4.0), c!(5.0,  6.0), c!(7.0,  8.0),
            c!(2.0, 0.0), c!(7.0, -8.0), c!(5.0, -6.0), c!(3.0, -4.0),
        ]);
    }
}
