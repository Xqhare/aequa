# Aequa

Aequa is a fundamental Rust library in my ecosystem. It provides arbitrary precision decimal arithmetic and the core in-memory structures for XFF ([Xqhare's File Format](https://github.com/xqhare/nabu)) and my ecosystem in general.

Instead of using this repo directly, consider using [athena](https://github.com/xqhare/athena).

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Naming

As with all my projects, Aequa, or Aequitas in full, is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

Aequitas was a minor Roman deity of equity and accurate measurements.

## Project Overview

Aequa is designed to be a reliable, zero-dependency foundation for high-fidelity data interchange and manipulation.

- **Purpose**: High-precision math and rich, in-memory, data structures.
- **No Dependencies**: Adhering to a strict "no external dependencies" rule (standard library and libc only).
- **Naming**: Part of "The Pantheon" convention.

## Key Features

### 1. High-Precision Arithmetic (`HpFloat`)

`HpFloat` is a decimal-based arbitrary precision type that avoids the pitfalls of IEEE 754 floating-point math (e.g., `0.1 + 0.2` is exactly `0.3`).

- **Drop-in Support**: Provides type aliases for `f64` and `f32` to allow easy migration by shadowing primitives.
- **Exact Decimals**: Perfect for financial applications or any domain where floating-point rounding errors are unacceptable.
- **Seamless Integration**: Implements standard Rust operator traits (`Add`, `Sub`, `Mul`, `Div`) and `From` conversions.
- **XFF V4 Support**: Will be compatible with XFF v4. Will then become part of `XffValue`.

### 2. XFF Value Ecosystem (`XffValue`)

`XffValue` is the central dynamic enum for all XFF data types, representing the logical (in-memory) state of data.

- **Rich Types**: Supports `String`, `Number`, `Boolean`, `Array`, `Object`, `OrderedObject`, `Data` (bytes), `Table` (schema-based), `Uuid`, `DateTime`, `Duration`, and `Null`.
- **Order Preservation**: Includes `OrderedObject` for key-value mappings that preserve insertion order.
- **Tabular Data**: `Table` provides a schema-based structure for efficient row/column data storage.
- **Ergonomic API**: Extensive use of `is_*`, `as_*`, and `into_*` methods for intuitive data handling.

### 3. Graph Data Structure (`Graph`)

A flexible, high-performance graph structure designed for serialization and complex relationship mapping.

- **Stable Indices**: Uses a "Free List" approach to ensure indices remain stable during node/connection removal.
- **Traversals**: Built-in support for Breadth-First Search (BFS) and Depth-First Search (DFS).
- **Algorithms**: Includes cycle detection, pathfinding (shortest path), and Strongly Connected Components (SCCs) via Tarjan's algorithm.
- **Metadata Support**: Every node and connection can carry an `XffValue` as payload or metadata.
- **XFF V4 Support**: Will be compatible with XFF v4. Will then become part of `XffValue`.

## Philosophy: No External Dependencies

While my other projects follow a "no external dependencies" rule that occasionally includes `libc`, Aequa is even stricter. It relies exclusively on the Rust standard library. This ensures:

- **Maximum Portability**: Can be easily integrated into any project or environment.
- **Security**: Zero supply-chain risk from third-party crates.
- **Long-term Stability**: Not subject to the breaking changes or deprecation cycles of external libraries.
- This Guarantee is made exclusive to `XffValue`: **Absolutely no breaking changes to the API.**

## Usage

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aequa = { git = "https://github.com/xqhare/aequa" }
```

### High-Precision Math Example

```rust
use aequa::hp_float::f64; // Shadows primitive f64 locally

fn main() {
    let a: f64 = 0.1.into();
    let b: f64 = 0.2.into();
    let sum = a + b;

    assert_eq!(sum, 0.3.into());
    println!("0.1 + 0.2 = {}", sum); // Prints exactly 0.3
}
```

### XFF Value Example

```rust
use aequa::{XffValue, Object};

fn main() {
    let mut obj = Object::new();
    obj.insert("project", "Aequa");
    obj.insert("version", 0.1);
    
    let value = XffValue::Object(obj);
    assert!(value.is_object());
    
    if let Some(name) = value["project"].as_string() {
        println!("Project: {}", name);
    }
}
```

## Building and Testing

As a standard Rust library:

```bash
cargo build
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
