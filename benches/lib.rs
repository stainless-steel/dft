#![feature(test)]

extern crate dft;
extern crate test;

use dft::{Operation, Plan, c64, complex, real};
use test::{Bencher, black_box};

#[bench] fn complex_transform_0004(bencher: &mut Bencher) { complex_transform(   4, bencher); }
#[bench] fn complex_transform_0016(bencher: &mut Bencher) { complex_transform(  16, bencher); }
#[bench] fn complex_transform_0064(bencher: &mut Bencher) { complex_transform(  64, bencher); }
#[bench] fn complex_transform_0256(bencher: &mut Bencher) { complex_transform( 256, bencher); }
#[bench] fn complex_transform_1024(bencher: &mut Bencher) { complex_transform(1024, bencher); }
#[bench] fn complex_transform_4096(bencher: &mut Bencher) { complex_transform(4096, bencher); }

#[bench] fn real_transform_0004(bencher: &mut Bencher) { real_transform(   4, bencher); }
#[bench] fn real_transform_0016(bencher: &mut Bencher) { real_transform(  16, bencher); }
#[bench] fn real_transform_0064(bencher: &mut Bencher) { real_transform(  64, bencher); }
#[bench] fn real_transform_0256(bencher: &mut Bencher) { real_transform( 256, bencher); }
#[bench] fn real_transform_1024(bencher: &mut Bencher) { real_transform(1024, bencher); }
#[bench] fn real_transform_4096(bencher: &mut Bencher) { real_transform(4096, bencher); }

fn complex_transform(size: usize, bencher: &mut Bencher) {
    let mut data = vec![c64::new(42.0, 69.0); size];
    let plan = Plan::new(Operation::Forward, size);
    bencher.iter(|| black_box(complex::transform(&mut data, &plan)));
}

fn real_transform(size: usize, bencher: &mut Bencher) {
    let mut data = vec![42.0; 2 * size];
    let plan = Plan::new(Operation::Forward, 2 * size);
    bencher.iter(|| black_box(real::transform(&mut data, &plan)));
}
