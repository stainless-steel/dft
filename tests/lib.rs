extern crate assert;
extern crate dft;

use dft::{Operation, Plan, c64, complex, real};

mod fixtures;

#[test]
fn complex_forward_128() {
    let mut data = fixtures::TIME_DATA_256.to_vec();
    complex::transform(as_c64_mut(&mut data), &Plan::new(Operation::Forward, 256));
    assert::close(&data, &fixtures::FREQUENCY_DATA_128_COMPLEX[..], 1e-14);
}

#[test]
fn complex_forward_real_256() {
    let mut data = to_c64(&fixtures::TIME_DATA_256);
    complex::transform(&mut data, &Plan::new(Operation::Forward, 256));
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_256_REAL_UNPACKED[..], 1e-13);
}

#[test]
fn complex_inverse_128() {
    let mut data = fixtures::FREQUENCY_DATA_128_COMPLEX.to_vec();
    complex::transform(as_c64_mut(&mut data), &Plan::new(Operation::Inverse, 128));
    assert::close(&data, &fixtures::TIME_DATA_256[..], 1e-14);
}

#[test]
fn real_forward_256() {
    let mut data = fixtures::TIME_DATA_256.to_vec();
    real::transform(&mut data, &Plan::new(Operation::Forward, 256));
    assert::close(&data, &fixtures::FREQUENCY_DATA_256_REAL_PACKED[..], 1e-13);
    let data = real::unpack(&data);
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_256_REAL_UNPACKED[..], 1e-13);
}

#[test]
fn real_forward_512() {
    let mut data = fixtures::TIME_DATA_512.to_vec();
    real::transform(&mut data, &Plan::new(Operation::Forward, 512));
    let data = real::unpack(&data);
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_512_REAL_UNPACKED[..], 1e-12);
}

#[test]
fn real_inverse_256() {
    let mut data = fixtures::FREQUENCY_DATA_256_REAL_PACKED.to_vec();
    real::transform(&mut data, &Plan::new(Operation::Inverse, 256));
    assert::close(&data, &fixtures::TIME_DATA_256[..], 1e-14);
}

#[test]
fn real_inverse_512() {
    let mut data = fixtures::TIME_DATA_512.to_vec();
    real::transform(&mut data, &Plan::new(Operation::Forward, 512));
    real::transform(&mut data, &Plan::new(Operation::Inverse, 512));
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
    slice.iter().map(|&re| c64::new(re, 0.0)).collect()
}
