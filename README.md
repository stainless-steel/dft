# DFT [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an algorithm to compute the [discrete Fourier
transform][1].

## [Documentation][doc]

## Example

```rust
use dft::{Operation, Plan, Transform};

let size = 512;
let mut data = vec![0.0; size];
let plan = Plan::new(Operation::Forward, size);

data.transform(&plan);
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
