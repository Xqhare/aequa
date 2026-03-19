# Aequa

A library for representing and manipulating numbers with arbitrary precision.
It trades off speed for precision, and is intended to be used as a drop in replacement for both `f64` and `f32`.

It follows my "no dependencies but the standard library and libc" philosophy.

## Features

- Arbitrary precision
- Fast
- No dependencies
- Drop-in replacement for `f64` and `f32`

## Naming

As with all my projects, it is named after an ancient deity.

Aequa was the roman divine personification of equity, fairness, exact exchange and accurate weights and measurements.

## Usage

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aequa = { git = "https://github.com/xqhare/aequa" }
```

### In Your Code

Simply shadow the primitive type to use `Aequa` throughout your module with full precision.

```rust
use aequa::f64; // Shadows primitive f64 locally

let a: f64 = 0.1.into();
let b: f64 = 0.2.into();

// Standard operators work seamlessly
let sum = a + b;

// Even mixing with literals is supported
let result = sum + 0.4;

assert_eq!(result, 0.7.into());
```

## How It Works

The idea is really quite simple.

Let's take the infamous example of 0.1 + 0.2:

If done using IEEE floats: 0.1 + 0.2 = 0.30000000000000004

If done using Aequa: 0.1 + 0.2 = 0.3

### Examples

0.1 => (1 * 10^-1) => 1 | 1
0.2 => (2 * 10^-1) => 2 | 1
0.1 + 0.2 => 3(3 * 10^-1) => 3 | 1

1.2 + 0.004 = 1.204
1.2 => (12 * 10^-1) => 12 | 1
0.004 => (4 * 10^-3) => 4 | 3

To add, you align the scales by multiplying the value of the smaller scale by 10^(scale_diff):

scale_diff = 3 - 1 = 2
1.2 => (12 * 10^2) | 1 = 1200 | 1

Then add the values together:
1200 | 1 + 4 | 3 = 1204 | 3

To go backwards:

1. Take the value as a string: "1204"
2. Insert the decimal point 3 places from the right: "1.204"

If the value is smaller than the scale (e.g., 3 | 1):

1. Pad the string with leading zeros: "03"
2. Insert the decimal point: "0.3"
