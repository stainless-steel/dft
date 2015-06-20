extern crate assert;
extern crate fft;

mod fixtures;

#[test]
fn transform_forward() {
    let mut data = fixtures::TIME_DATA.to_vec();
    fft::transform(&mut data, fft::Direction::Forward);
    assert::close(&data, &fixtures::FREQUENCY_DATA[..], 1e-13);
}

#[test]
fn transform_inverse() {
    let mut data = fixtures::FREQUENCY_DATA.to_vec();
    fft::transform(&mut data, fft::Direction::Inverse);
    assert::close(&data, &fixtures::TIME_DATA[..], 1e-13);
}
