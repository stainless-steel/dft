extern crate assert;
extern crate dft;

use dft::{Operation, Plan, c64, transform, unpack};

mod fixtures;

#[test]
fn complex_forward_1() {
    let mut data = vec![c64::new(1.0, -2.0)];
    transform(&mut data, &Plan::new(Operation::Forward, 1));
    assert_eq!(data, vec![c64::new(1.0, -2.0)]);
}

#[test]
fn complex_forward_2() {
    let mut data = vec![c64::new(1.0, -2.0), c64::new(3.0, -4.0)];
    transform(&mut data, &Plan::new(Operation::Forward, 2));
    assert_eq!(data, vec![c64::new(4.0, -6.0), c64::new(-2.0, 2.0)]);
}

#[test]
fn complex_forward_128() {
    let mut data = fixtures::TIME_DATA_256.to_vec();
    transform(as_c64_mut(&mut data), &Plan::new(Operation::Forward, 128));
    assert::close(&data, &fixtures::FREQUENCY_DATA_128_COMPLEX[..], 1e-14);
}

#[test]
fn complex_forward_real_256() {
    let mut data = to_c64(&fixtures::TIME_DATA_256);
    transform(&mut data, &Plan::new(Operation::Forward, 256));
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_256_REAL_UNPACKED[..], 1e-13);
}

#[test]
fn complex_inverse_128() {
    let mut data = fixtures::FREQUENCY_DATA_128_COMPLEX.to_vec();
    transform(as_c64_mut(&mut data), &Plan::new(Operation::Inverse, 128));
    assert::close(&data, &fixtures::TIME_DATA_256[..], 1e-14);
}

#[test]
fn real_forward_1() {
    let mut data = vec![1.0];
    transform(&mut data, &Plan::new(Operation::Forward, 1));
    assert_eq!(unpack(&data), vec![c64::new(1.0, 0.0)]);
}

#[test]
fn real_forward_2() {
    let mut data = vec![1.0, -2.0];
    transform(&mut data, &Plan::new(Operation::Forward, 2));
    assert_eq!(unpack(&data), vec![c64::new(-1.0, 0.0), c64::new(3.0, 0.0)]);
}

#[test]
fn real_forward_4() {
    let mut data = vec![1.0, -2.0, 3.0, -4.0];
    transform(&mut data, &Plan::new(Operation::Forward, 4));
    assert_eq!(unpack(&data), vec![
       c64::new(-2.0, 0.0), c64::new(-2.0, -2.0), c64::new(10.0, 0.0), c64::new(-2.0, 2.0),
    ]);
}

#[test]
fn real_forward_256() {
    let mut data = fixtures::TIME_DATA_256.to_vec();
    transform(&mut data, &Plan::new(Operation::Forward, 256));
    assert::close(&data, &fixtures::FREQUENCY_DATA_256_REAL_PACKED[..], 1e-13);
    let data = unpack(&data);
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_256_REAL_UNPACKED[..], 1e-13);
}

#[test]
fn real_forward_512() {
    let mut data = fixtures::TIME_DATA_512.to_vec();
    transform(&mut data, &Plan::new(Operation::Forward, 512));
    let data = unpack(&data);
    assert::close(as_f64(&data), &fixtures::FREQUENCY_DATA_512_REAL_UNPACKED[..], 1e-12);
}

#[test]
fn real_inverse_256() {
    let mut data = fixtures::FREQUENCY_DATA_256_REAL_PACKED.to_vec();
    transform(&mut data, &Plan::new(Operation::Inverse, 256));
    assert::close(&data, &fixtures::TIME_DATA_256[..], 1e-14);
}

#[test]
fn real_inverse_512() {
    let mut data = fixtures::TIME_DATA_512.to_vec();
    transform(&mut data, &Plan::new(Operation::Forward, 512));
    transform(&mut data, &Plan::new(Operation::Inverse, 512));
    assert::close(&data, &fixtures::TIME_DATA_512[..], 1e-14);
}

fn as_f64(slice: &[c64]) -> &[f64] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const _, 2 * slice.len()) }
}

fn as_c64_mut(slice: &mut [f64]) -> &mut [c64] {
    unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut _, slice.len() / 2) }
}

fn to_c64(slice: &[f64]) -> Vec<c64> {
    slice.iter().map(|&re| c64::new(re, 0.0)).collect()
}
