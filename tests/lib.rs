extern crate assert;
extern crate fft;

mod fixtures;

#[test]
fn forward() {
    let mut data = fixtures::TIME_DATA.to_vec();
    fft::forward(&mut data);
    assert::close(&data, &fixtures::FREQUENCY_DATA[..], 1e-13);
}

#[test]
fn inverse() {
    let mut data = fixtures::FREQUENCY_DATA.to_vec();
    fft::inverse(&mut data, true);
    assert::close(&data, &fixtures::TIME_DATA[..], 1e-13);
}
