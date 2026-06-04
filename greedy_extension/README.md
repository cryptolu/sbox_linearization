# veclin_greedy_extension

Rust-powered Python package implementing the greedy extension algorithm for S-box vectorial linearization.

Part of the [sbox_linearization](https://github.com/cryptolu/sbox_linearization) project.

This is a temporary simple wrapper to provide easy access to the algorithm. Later, it should be embedded in the S-box analysis packages such as [sboxU](https://github.com/lpp_crypto/sboxU) or SageMath.


## Installation

```bash
pip install veclin_greedy_extension
```

## Usage

```python
from veclin_greedy_extension import GreedyExtensionU16

# SKINNY-8
sbox = [101, 76, 106, 66, 75, 99, 67, 107, 85, 117, 90, 122, 83, 115, 91, 123, 53, 140, 58, 129, 137, 51, 128, 59, 149, 37, 152, 42, 144, 35, 153, 43, 229, 204, 232, 193, 201, 224, 192, 233, 213, 245, 216, 248, 208, 240, 217, 249, 165, 28, 168, 18, 27, 160, 19, 169, 5, 181, 10, 184, 3, 176, 11, 185, 50, 136, 60, 133, 141, 52, 132, 61, 145, 34, 156, 44, 148, 36, 157, 45, 98, 74, 108, 69, 77, 100, 68, 109, 82, 114, 92, 124, 84, 116, 93, 125, 161, 26, 172, 21, 29, 164, 20, 173, 2, 177, 12, 188, 4, 180, 13, 189, 225, 200, 236, 197, 205, 228, 196, 237, 209, 241, 220, 252, 212, 244, 221, 253, 54, 142, 56, 130, 139, 48, 131, 57, 150, 38, 154, 40, 147, 32, 155, 41, 102, 78, 104, 65, 73, 96, 64, 105, 86, 118, 88, 120, 80, 112, 89, 121, 166, 30, 170, 17, 25, 163, 16, 171, 6, 182, 8, 186, 0, 179, 9, 187, 230, 206, 234, 194, 203, 227, 195, 235, 214, 246, 218, 250, 211, 243, 219, 251, 49, 138, 62, 134, 143, 55, 135, 63, 146, 33, 158, 46, 151, 39, 159, 47, 97, 72, 110, 70, 79, 103, 71, 111, 81, 113, 94, 126, 87, 119, 95, 127, 162, 24, 174, 22, 31, 167, 23, 175, 1, 178, 14, 190, 7, 183, 15, 191, 226, 202, 238, 198, 207, 231, 199, 239, 210, 242, 222, 254, 215, 247, 223, 255]

# The object essentially precomputes DDT to make consequent runs cheaper
GE = GreedyExtensionU16(sbox)

# one execution of the algorithm: the first argument is initial points to include in the final set, the second one is the random seed
xs = GE.run()
print(xs)  # e.g. [7, 10, 11, 14, 15, 23, 27, 31, 37, 39, 42, 43, 44, 45, 46, 47, 69, 71, 75, 77, 79, 85, 87, 90, 91, 92, 93, 94, 95, 133, 135, 139, 141, 143, 149, 151, 154, 155, 156, 157, 158, 159, 183, 186, 187, 190, 191]


# _ext() version returns more information
xs, profile, merges = GE.run_ext()

# approximation sizes during the internal iterations
print(profile)  # intermediate sizes: [1, 2, 4, 8, 12, 18, 24, 32, 40]

# which clique merges occurred (in order)
print(merges)  # [(145, 58), (145, 51), (145, 3), (145, 8), (145, 81), (145, 211), (145, 6), (145, 18)]


# best of 100 runs or 1 second
xs, profile, merges, seed = GE.best_of_ext(100, time=1.0)
print(xs)  # [20, 22, 26, 28, 30, 36, 38, 42, 44, 46, 59, 61, 63, 68, 70, 74, 75, 76, 77, 78, 79, 100, 102, 106, 107, 108, 109, 110, 111, 116, 118, 122, 124, 126, 196, 202, 203, 206, 207, 228, 234, 235, 238, 239, 244, 250, 254]


# reproduce
xs2, profile, merges = GE.run_ext(seed=seed)
assert xs2 == xs
```

## Building from source

Requires Rust and Python 3.8+.

```bash
pip install maturin
cd greedy_extension
maturin develop
```

## License

MIT