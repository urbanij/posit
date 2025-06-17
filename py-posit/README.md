```sh
uv venv
source .venv/bin/activate
uv pip install maturin
# if macOS
uv pip install pip

maturin develop
python
>>> import jposit
>>> jposit.from_bits(...)
>>>
```

---
Deprecated:


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
