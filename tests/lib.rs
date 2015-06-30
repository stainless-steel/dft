extern crate assert;
extern crate complex;
extern crate dft;

use complex::c64;

mod fixtures;

#[test]
fn complex_forward_128() {
    let mut data = fixtures::TIME_DATA_256.to_vec();
    dft::complex::forward(as_c64_mut(&mut data));
    assert::close(&data, &fixtures::FREQUENCY_DATA_128_COMPLEX[..], 1e-14);
}

#[test]
fn complex_forward_real_256() {
    let mut data = to_c64(&fixtures::TIME_DATA_256);
    dft::complex::forward(&mut data);
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_256_REAL_UNPACKED[..], 1e-13);
}

#[test]
fn complex_inverse_128() {
    let mut data = fixtures::FREQUENCY_DATA_128_COMPLEX.to_vec();
    dft::complex::inverse(as_c64_mut(&mut data));
    assert::close(&data, &fixtures::TIME_DATA_256[..], 1e-14);
}

#[test]
fn real_forward_256() {
    let mut data = fixtures::TIME_DATA_256.to_vec();
    dft::real::forward(&mut data);
    assert::close(&data, &fixtures::FREQUENCY_DATA_256_REAL_PACKED[..], 1e-13);
    let data = dft::real::unpack(&data);
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_256_REAL_UNPACKED[..], 1e-13);
}

#[test]
fn real_forward_512() {
    let mut data = fixtures::TIME_DATA_512.to_vec();
    dft::real::forward(&mut data);
    let data = dft::real::unpack(&data);
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_512_REAL_UNPACKED[..], 1e-12);
}

#[test]
fn real_inverse_256() {
    let mut data = fixtures::FREQUENCY_DATA_256_REAL_PACKED.to_vec();
    dft::real::inverse(&mut data);
    assert::close(&data, &fixtures::TIME_DATA_256[..], 1e-14);
}

#[test]
fn real_inverse_512() {
    let mut data = fixtures::TIME_DATA_512.to_vec();
    dft::real::forward(&mut data);
    dft::real::inverse(&mut data);
    assert::close(&data, &fixtures::TIME_DATA_512[..], 1e-14);
}

fn as_f64<'l>(slice: &'l [c64]) -> &'l [f64] {
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const _, 2 * slice.len())
    }
}

fn as_c64_mut<'l>(slice: &'l mut [f64]) -> &'l mut [c64] {
    unsafe {
        std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut _, slice.len() / 2)
    }
}

fn to_c64(slice: &[f64]) -> Vec<c64> {
    slice.iter().map(|&re| c64(re, 0.0)).collect()
}
