// The implementation is based on:
// http://www.librow.com/articles/article-10

use num_complex::Complex;
use num_traits::Float;

use {Operation, Plan, Transform};

impl<T> Transform<T> for [Complex<T>] where T: Float {
    fn transform(&mut self, plan: &Plan<T>) {
        let n = self.len();
        assert!(n <= plan.n);
        rearrange(self, n);
        calculate(self, n, &plan.factors);
        if let Operation::Inverse = plan.operation {
            scale(self, n);
        }
    }
}

impl<T> Transform<T> for Vec<Complex<T>> where T: Float {
    #[inline(always)]
    fn transform(&mut self, plan: &Plan<T>) {
        Transform::transform(&mut self[..], plan)
    }
}

#[inline(always)]
fn calculate<T>(data: &mut [Complex<T>], n: usize, factors: &[Complex<T>]) where T: Float {
    let mut k = 0;
    let mut step = 1;
    while step < n {
        let jump = step << 1;
        for mut i in 0..step {
            while i < n {
                let j = i + step;
                let product = factors[k] * data[j];
                data[j] = data[i] - product;
                data[i] = data[i] + product;
                i += jump;
            }
            k += 1;
        }
        step <<= 1;
    }
}

#[inline(always)]
fn rearrange<T>(data: &mut [Complex<T>], n: usize) {
    let mut j = 0;
    for i in 0..n {
        if j > i {
            data.swap(i, j);
        }
        let mut mask = n >> 1;
        while j & mask != 0 {
            j &= !mask;
            mask >>= 1;
        }
        j |= mask;
    }
}

#[inline(always)]
fn scale<T>(data: &mut [Complex<T>], n: usize) where T: Float {
    let factor = T::from(n).unwrap().recip();
    for i in 0..n {
        data[i] = data[i].scale(factor);
    }
}
