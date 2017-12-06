# DFT [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides an algorithm to compute the [discrete Fourier
transform][1].

## Example

```rust
use dft::{Operation, Plan, c64};

let plan = Plan::new(Operation::Forward, 512);
let mut data = vec![c64::new(42.0, 69.0); 512];
dft::transform(&mut data, &plan);
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

[build-img]: https://travis-ci.org/stainless-steel/dft.svg?branch=master
[build-url]: https://travis-ci.org/stainless-steel/dft
[documentation-img]: https://docs.rs/dft/badge.svg
[documentation-url]: https://docs.rs/dft
[package-img]: https://img.shields.io/crates/v/dft.svg
[package-url]: https://crates.io/crates/dft
