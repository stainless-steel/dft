pub enum Direction {
    Forward,
    Backward,
}

pub fn transform(data: &mut [f64], _: Direction) {
    let n = data.len();
    if n < 2 || n & (n - 1) != 0 {
        panic!("the data size should be a power of two");
    }
}
