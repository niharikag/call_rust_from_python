# call_rust_from_python
A simple example of calling rust from python

Here is a simple example of calling a function "rust_normalize" defined in Rust that converts (normalizes)   
very small likelihood values to probability from python. 

This uses PyO3 the Rust bindings for Python interpreter. See https://github.com/PyO3/pyo3 for details.

## Building Rust Library
```
cargo build --release
```

The library "librust_library.so" is generated in the path "target/release/librust_library.so"

## Rename Library and move it to the same folder to your Python path 
```
cp target/release/librust_library.so rust_library.so
```

## Importing from Python and calling the "rust_normalize" function
```
import rust_library
import numpy as np


n = 10
x = np.random.rand(n)
norm_x = rust_library.rust_normalize(x)
print(sum(norm_x))
```
