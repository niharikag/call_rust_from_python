import rust_library
import numpy as np

np.random.seed(223)
n = 10
x = np.random.rand(n)
norm_x = rust_library.rust_normalize(x)
print(x)
print(norm_x)
print(sum(norm_x))

