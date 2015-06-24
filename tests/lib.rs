extern crate assert;
extern crate fft;

mod fixtures;

#[test]
fn forward() {
    let mut data = fixtures::TIME_DATA.to_vec();
    fft::forward(as_c64_mut(&mut data));
    assert::close(&data, &fixtures::FREQUENCY_DATA_FROM_COMPLEX[..], 1e-14);
}

#[test]
fn forward_real() {
    let mut data = fixtures::TIME_DATA.to_vec();
    {
        let mut data = to_c64(&data);
        fft::forward(&mut data);
        assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_FROM_REAL[..], 1e-13);
    }
    {
        fft::forward_real(&mut data);
        let data = fft::unpack_real(&data);
        assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_FROM_REAL[..], 1e-13);
    }
}

#[test]
fn inverse() {
    let mut data = fixtures::FREQUENCY_DATA_FROM_COMPLEX.to_vec();
    fft::inverse(as_c64_mut(&mut data), true);
    assert::close(&data, &fixtures::TIME_DATA[..], 1e-14);
}

fn as_f64<'l>(slice: &'l [fft::c64]) -> &'l [f64] {
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const _, 2 * slice.len())
    }
}

fn as_c64_mut<'l>(slice: &'l mut [f64]) -> &'l mut [fft::c64] {
    unsafe {
        std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut _, slice.len() / 2)
    }
}

fn to_c64(slice: &[f64]) -> Vec<fft::c64> {
    slice.iter().map(|&re| fft::c64(re, 0.0)).collect()
}
