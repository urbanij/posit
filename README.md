[![Rust](https://github.com/urbanij/posit/actions/workflows/rust.yml/badge.svg)](https://github.com/urbanij/posit/actions/workflows/rust.yml)

# Posit

Added python bindings exploiting Python FFI lib.
Working with Python 3.9/3.10
```py
>>> import jposit
>>> jposit.from_bits(123, 8, 0)
14.0
>>> jposit.from_double(35.1, 16, 1)
35.09375
```

Jupyter notebook: https://github.com/urbanij/posit/blob/master/py-posit/notebooks/demo.ipynb

<!-- [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/urbanij/posit/master?labpath=py-posit%2Fnotebooks%2Fdemo.ipynb) -->

<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>


# Original README below:
---


# Status

This crate is **UNMAINTAINED**. You may be interested in looking at https://gitlab.com/burrbull/softposit-rs.

-- @japaric, 2018-12-08

# `posit`

> A Rust implementation of the [posit] number system

[posit]: http://web.stanford.edu/class/ee380/Abstracts/170201.html

# References

- http://web.stanford.edu/class/ee380/Abstracts/170201-slides.pdf
- http://www.johngustafson.net/presentations/Unums2.0.pdf

# Documentation

Clone this repo and run this command:

```
$ cargo doc --open
```

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
