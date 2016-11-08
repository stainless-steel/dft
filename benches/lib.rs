#![feature(test)]

extern crate dft;
extern crate test;

use std::mem;
use dft::{Operation, Plan, c64};
use test::{Bencher, black_box};

#[bench] fn complex_0004(bencher: &mut Bencher) { complex(   4, bencher); }
#[bench] fn complex_0008(bencher: &mut Bencher) { complex(   8, bencher); }
#[bench] fn complex_0016(bencher: &mut Bencher) { complex(  16, bencher); }
#[bench] fn complex_0032(bencher: &mut Bencher) { complex(  32, bencher); }
#[bench] fn complex_0064(bencher: &mut Bencher) { complex(  64, bencher); }
#[bench] fn complex_0128(bencher: &mut Bencher) { complex( 128, bencher); }
#[bench] fn complex_0256(bencher: &mut Bencher) { complex( 256, bencher); }
#[bench] fn complex_0512(bencher: &mut Bencher) { complex( 512, bencher); }
#[bench] fn complex_1024(bencher: &mut Bencher) { complex(1024, bencher); }
#[bench] fn complex_2048(bencher: &mut Bencher) { complex(2048, bencher); }
#[bench] fn complex_4096(bencher: &mut Bencher) { complex(4096, bencher); }
#[bench] fn complex_8192(bencher: &mut Bencher) { complex(8192, bencher); }

#[bench] fn real_0004(bencher: &mut Bencher) { real(   4, bencher); }
#[bench] fn real_0008(bencher: &mut Bencher) { real(   8, bencher); }
#[bench] fn real_0016(bencher: &mut Bencher) { real(  16, bencher); }
#[bench] fn real_0032(bencher: &mut Bencher) { real(  32, bencher); }
#[bench] fn real_0064(bencher: &mut Bencher) { real(  64, bencher); }
#[bench] fn real_0128(bencher: &mut Bencher) { real( 128, bencher); }
#[bench] fn real_0256(bencher: &mut Bencher) { real( 256, bencher); }
#[bench] fn real_0512(bencher: &mut Bencher) { real( 512, bencher); }
#[bench] fn real_1024(bencher: &mut Bencher) { real(1024, bencher); }
#[bench] fn real_2048(bencher: &mut Bencher) { real(2048, bencher); }
#[bench] fn real_4096(bencher: &mut Bencher) { real(4096, bencher); }
#[bench] fn real_8192(bencher: &mut Bencher) { real(8192, bencher); }

fn complex(size: usize, bencher: &mut Bencher) {
    let mut data = vec![c64::new(42.0, 69.0); size];
    let plan = Plan::new(Operation::Forward, size);
    bencher.bytes = (data.len() * mem::size_of_val(&data[0])) as u64;
    bencher.iter(|| black_box(dft::transform(&mut data, &plan)));
}

fn real(size: usize, bencher: &mut Bencher) {
    let mut data = vec![42.0; 2 * size];
    let plan = Plan::new(Operation::Forward, 2 * size);
    bencher.bytes = (data.len() * mem::size_of_val(&data[0])) as u64;
    bencher.iter(|| black_box(dft::transform(&mut data, &plan)));
}
