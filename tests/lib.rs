extern crate assert;
extern crate fft;

mod fixtures;

#[test]
fn forward() {
    let mut data = fixtures::TIME_DATA.to_vec();
    fft::forward(reinterpret(&mut data));
    assert::close(&data, &fixtures::FREQUENCY_DATA[..], 1e-14);
}

#[test]
fn inverse() {
    let mut data = fixtures::FREQUENCY_DATA.to_vec();
    fft::inverse(reinterpret(&mut data), true);
    assert::close(&data, &fixtures::TIME_DATA[..], 1e-14);
}

fn reinterpret<'l>(slice: &'l mut [f64]) -> &'l mut [fft::c64] {
    unsafe {
        let length = slice.len();
        assert!(length % 2 == 0, "the number of elements should be even");
        std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut _, length / 2)
    }
}
