# DFT [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an algorithm to compute the [discrete Fourier
transform][1].

## [Documentation][doc]

## Example

```rust
use dft::{Operation, Plan, c64};

let size = 512;
let plan = Plan::new(Operation::Forward, size);
let mut data = vec![c64::new(42.0, 69.0); size];

dft::transform(&mut data, &plan);
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[1]: https://en.wikipedia.org/wiki/Discrete_Fourier_transform

[version-img]: https://img.shields.io/crates/v/dft.svg
[version-url]: https://crates.io/crates/dft
[status-img]: https://travis-ci.org/stainless-steel/dft.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/dft
[doc]: https://stainless-steel.github.io/dft
