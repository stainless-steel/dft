#![feature(test)]

extern crate fft;
extern crate test;

#[bench] fn forward_0004(bencher: &mut test::Bencher) { forward(   4, bencher); }
#[bench] fn forward_0016(bencher: &mut test::Bencher) { forward(  16, bencher); }
#[bench] fn forward_0064(bencher: &mut test::Bencher) { forward(  64, bencher); }
#[bench] fn forward_0256(bencher: &mut test::Bencher) { forward( 256, bencher); }
#[bench] fn forward_1024(bencher: &mut test::Bencher) { forward(1024, bencher); }
#[bench] fn forward_4096(bencher: &mut test::Bencher) { forward(4096, bencher); }

fn forward(size: usize, bencher: &mut test::Bencher) {
    let mut data = vec![fft::c64(42.0, 0.0); size];
    bencher.iter(|| {
        test::black_box(fft::forward(&mut data));
    });
}
