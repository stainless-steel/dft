#![feature(step_by)]

/// A direction of the Fourier transform.
pub enum Direction {
    /// From the time domain to the frequency domain.
    Forward,
    /// From the frequency domain to the time domain.
    Inverse,
}

/// Perform the Fourier transform.
pub fn transform(data: &mut [f64], direction: Direction) {
    use std::f64::consts::PI;

    let n = data.len() / 2;
    let nn = n << 1;

    let isign = match direction {
        Direction::Forward => 1.0,
        Direction::Inverse => -1.0,
    };

    let mut j = 1;
    for i in (1..nn).step_by(2) {
        if j > i {
            data.swap(j - 1, i - 1);
            data.swap(j, i);
        }
        let mut m = n;
        while m >= 2 && j > m {
            j -= m;
            m >>= 1;
        }
        j += m;
    }

    let mut mmax = 2;
    while nn > mmax {
        let istep = mmax << 1;
        let theta = isign * (2.0 * PI / mmax as f64);
        let wtemp = (0.5 * theta).sin();
        let wpr = -2.0 * wtemp * wtemp;
        let wpi = theta.sin();
        let mut wr = 1.0;
        let mut wi = 0.0;
        for m in (1..mmax).step_by(2) {
            for i in (m..(nn + 1)).step_by(istep) {
                let j = i + mmax;
                let tempr = wr * data[j - 1] - wi * data[j];
                let tempi = wr * data[j] + wi * data[j - 1];
                data[j - 1] = data[i - 1] - tempr;
                data[j] = data[i] - tempi;
                data[i - 1] += tempr;
                data[i] += tempi;
            }
            let wtemp = wr;
            wr = wr * wpr - wi * wpi + wr;
            wi = wi * wpr + wtemp * wpi + wi;
        }
        mmax = istep;
    }

    if let Direction::Inverse = direction {
        let scale = 1.0 / n as f64;
        for i in 0..(2 * n) {
            data[i] *= scale;
        }
    }
}
