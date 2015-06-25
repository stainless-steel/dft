#![feature(test)]

extern crate complex;
extern crate fft;
extern crate test;

use complex::c64;
use test::{Bencher, black_box};

#[bench] fn complex_forward_0004(bencher: &mut Bencher) { complex_forward(   4, bencher); }
#[bench] fn complex_forward_0016(bencher: &mut Bencher) { complex_forward(  16, bencher); }
#[bench] fn complex_forward_0064(bencher: &mut Bencher) { complex_forward(  64, bencher); }
#[bench] fn complex_forward_0256(bencher: &mut Bencher) { complex_forward( 256, bencher); }
#[bench] fn complex_forward_1024(bencher: &mut Bencher) { complex_forward(1024, bencher); }
#[bench] fn complex_forward_4096(bencher: &mut Bencher) { complex_forward(4096, bencher); }

fn complex_forward(size: usize, bencher: &mut Bencher) {
    let mut data = vec![c64(42.0, 0.0); size];
    bencher.iter(|| black_box(fft::complex::forward(&mut data)));
}
