#![feature(test)]

extern crate dft;
extern crate test;

use dft::{Operation, Plan, c64};
use test::{Bencher, black_box};

#[bench] fn complex_0004(bencher: &mut Bencher) { complex(   4, bencher); }
#[bench] fn complex_0016(bencher: &mut Bencher) { complex(  16, bencher); }
#[bench] fn complex_0064(bencher: &mut Bencher) { complex(  64, bencher); }
#[bench] fn complex_0256(bencher: &mut Bencher) { complex( 256, bencher); }
#[bench] fn complex_1024(bencher: &mut Bencher) { complex(1024, bencher); }
#[bench] fn complex_4096(bencher: &mut Bencher) { complex(4096, bencher); }
#[bench] fn complex_8192(bencher: &mut Bencher) { complex(8192, bencher); }

#[bench] fn real_0004(bencher: &mut Bencher) { real(   4, bencher); }
#[bench] fn real_0016(bencher: &mut Bencher) { real(  16, bencher); }
#[bench] fn real_0064(bencher: &mut Bencher) { real(  64, bencher); }
#[bench] fn real_0256(bencher: &mut Bencher) { real( 256, bencher); }
#[bench] fn real_1024(bencher: &mut Bencher) { real(1024, bencher); }
#[bench] fn real_4096(bencher: &mut Bencher) { real(4096, bencher); }
#[bench] fn real_8192(bencher: &mut Bencher) { real(8192, bencher); }

fn complex(size: usize, bencher: &mut Bencher) {
    let mut data = vec![c64::new(42.0, 69.0); size];
    let plan = Plan::new(Operation::Forward, size);
    bencher.iter(|| black_box(dft::transform(&mut data, &plan)));
}

fn real(size: usize, bencher: &mut Bencher) {
    let mut data = vec![42.0; 2 * size];
    let plan = Plan::new(Operation::Forward, 2 * size);
    bencher.iter(|| black_box(dft::transform(&mut data, &plan)));
}
