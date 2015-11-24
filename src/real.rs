use {Operation, Plan, Transform, c64};

impl Transform for [f64] {
    fn transform(&mut self, plan: &Plan) {
        use std::slice::from_raw_parts_mut;

        let n = self.len();
        assert!(n == plan.size, "the plan is not appropriate for the dataset");
        let data = unsafe { from_raw_parts_mut(self.as_mut_ptr() as *mut c64, n / 2) };
        match plan.operation {
            Operation::Forward => {
                data.transform(plan);
                compose(data, n / 2, &plan.factors, false);
            },
            Operation::Backward | Operation::Inverse => {
                compose(data, n / 2, &plan.factors, true);
                data.transform(plan);
            },
        }
    }
}

impl Transform for Vec<f64> {
    #[inline(always)]
    fn transform(&mut self, plan: &Plan) {
        Transform::transform(self as &mut [f64], plan)
    }
}

/// Unpack a compressed representation produced by `Transform::transform` with
/// `Operation::Forward` when applied to real data. See the top-level
/// description of the crate.
pub fn unpack(data: &[f64]) -> Vec<c64> {
    let n = data.len();
    assert!(n.is_power_of_two(), "the number of points should be a power of two");
    let mut cdata = Vec::with_capacity(n);
    unsafe { cdata.set_len(n) };
    cdata[0] = c64!(data[0], 0.0);
    for i in 1..(n / 2) {
        cdata[i] = c64!(data[2 * i], data[2 * i + 1]);
    }
    cdata[n / 2] = c64!(data[1], 0.0);
    for i in (n / 2 + 1)..n {
        cdata[i] = cdata[n - i].conj();
    }
    cdata
}

#[inline(always)]
fn compose(data: &mut [c64], n: usize, factors: &[c64], inverse: bool) {
    data[0] = c64!(data[0].re + data[0].im, data[0].re - data[0].im);
    if inverse {
        data[0] = data[0].scale(0.5);
    }
    let m = factors.len();
    let sign = if inverse { 1.0 } else { -1.0 };
    for i in 1..(n / 2) {
        let j = n - i;
        let part1 = data[i] + data[j].conj();
        let part2 = data[i] - data[j].conj();
        let product = c64!(0.0, sign) * factors[m - j] * part2;
        data[i] = (part1 + product).scale(0.5);
        data[j] = (part1 - product).scale(0.5).conj();
    }
    data[n / 2] = data[n / 2].conj();
}

#[cfg(test)]
mod tests {
    #[test]
    fn unpack() {
        let data = (0..4).map(|i| (i + 1) as f64).collect::<Vec<_>>();
        assert_eq!(super::unpack(&data), vec![
            c64!(1.0, 0.0), c64!(3.0, 4.0), c64!(2.0, 0.0), c64!(3.0, -4.0),
        ]);

        let data = (0..8).map(|i| (i + 1) as f64).collect::<Vec<_>>();
        assert_eq!(super::unpack(&data), vec![
            c64!(1.0, 0.0), c64!(3.0, 4.0), c64!(5.0, 6.0), c64!(7.0, 8.0),
            c64!(2.0, 0.0), c64!(7.0, -8.0), c64!(5.0, -6.0), c64!(3.0, -4.0),
        ]);
    }
}
