# Aequa

Aequa is a fundamental Rust library in my ecosystem. It provides arbitrary precision decimal arithmetic and the core in-memory structures for XFF ([Xqhare's File Format](https://github.com/xqhare/nabu)) and my ecosystem in general.

This library's primary purpose is to provide the `XffValue` type for my ecosystem.
`XffValue` is the central dynamic enum for all XFF data types.
`XffValue` can be exported into a `JSON` or `CSV` file using [mawu](https://github.com/xqhare/mawu).
I strongly recommend using [nabu](https://github.com/xqhare/nabu) for the purpose of persisting `XffValue` instead and working with `XFF` files.

Instead of using this repo directly, consider using [athena](https://github.com/xqhare/athena).
Use this library only if you have no use for serde and no need for the other features of `Athena`.

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

In contrast to my other projects, this philosophy is made even stricter for this library: I will only be using the standard library exclusively.
This of course means that this library is explicitly not `no_std` compatible.

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

### 1. XFF Value Ecosystem (`XffValue`)

`XffValue` is the central dynamic enum for all XFF data types, representing the logical (in-memory) state of data.

- **Rich Types**: Supports `String`, `Number`, `Boolean`, `Array`, `Object`, `OrderedObject`, `Data` (bytes), `Table` (schema-based), `Uuid`, `DateTime`, `Duration`, and `Null`.
- **Order Preservation**: Includes `OrderedObject` for key-value mappings that preserve insertion order.
- **Tabular Data**: `Table` provides a schema-based structure for efficient row/column data storage.
- **Ergonomic API**: Extensive use of `is_*`, `as_*`, and `into_*` methods for intuitive data handling.
- **No Breaking Changes**: Absolutely no breaking changes to the API. Ever.
- **XFF Support**: Is fully compatible with all versions of the `XFF` specification.

### 2. High-Precision Arithmetic (`HpFloat`)

`HpFloat` is a decimal-based arbitrary precision type that avoids the pitfalls of IEEE 754 floating-point math (e.g., `0.1 + 0.2` is exactly `0.3`).

- **Drop-in Support**: Provides type aliases for `f64` and `f32` to allow easy migration by shadowing primitives.
- **Exact Decimals**: Perfect for any domain where floating-point rounding errors are unacceptable.
- **Seamless Integration**: Implements standard Rust operator traits (`Add`, `Sub`, `Mul`, `Div`) and `From` conversions.
- **Precision over Performance**: High precision math with some performance overhead.
- **XFF V4 Support**: Will be compatible with XFF v4. Will then become part of `XffValue`.

#### Road Map / To-Do

- [ ] Benchmarking against the standard primitives.
- [ ] Precision Limits: How does HpFloat handle division that results in infinite repeating decimals (e.g., 1 / 3)? Is there a default precision cutoff or an error state?
- [ ] Advanced Math: Does it support trigonometric functions, powers, or roots, or is it strictly for basic arithmetic?

### 3. Graph Data Structure (`Graph`)

A flexible, high-performance graph structure designed for serialization and complex relationship mapping.

The implementation of the `Graph` remains very bare bones, this is to increase the spectrum of use-cases for this type.

- **Stable Indices**: Uses a "Free List" approach to ensure indices remain stable during node/connection removal.
- **Traversals**: Built-in support for Breadth-First Search (BFS) and Depth-First Search (DFS).
- **Algorithms**: Includes cycle detection, pathfinding (shortest path), and Strongly Connected Components (SCCs) via Tarjan's algorithm.
- **Metadata Support**: Every node and connection can carry an `XffValue` as payload or metadata.
- **XFF V4 Support**: Will be compatible with XFF v4. Will then become part of `XffValue`.

#### Road Map / To-Do

- [ ] Usability: Never ever used any graph before. The needs of the API are still largely unknown to me

## Philosophy: No External Dependencies

While my other projects follow a "no external dependencies" rule that occasionally includes `libc`, Aequa is even stricter. It relies exclusively on the Rust standard library. This ensures:

- **Maximum Portability**: Can be easily integrated into any project or environment.
- **Security**: Zero supply-chain risk from third-party crates.
- **Long-term Stability**: Not subject to the breaking changes or deprecation cycles of external libraries.
- This Guarantee is made exclusive to `XffValue`: **Absolutely no breaking changes to the API.**

### Breaking Changes Guarantee

The API of `XffValue` is not subject to breaking changes at any point.
I ensure this by extensive testing coverage, especially using the documentation examples.

At the time of writing, Aequa has more than 200 tests, as well as several examples making use of `XffValue`.

Another way I ensure this guarantee is by defining the characteristics of `XffValue` in the [xff specification](https://github.com/xqhare/nabu/tree/master/specifications).

This guarantee is **not** made for any other value or type inside Aequa.
These types are still in development and may be subject to breaking changes at any time.

As soon as these types are merged into `XffValue`, the stability guarantee will extend to them as well.

## Usage

More examples can be found in the [examples](https://github.com/xqhare/aequa/tree/master/examples) directory.

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
    obj.insert("project", "Name");
    obj.insert("version", 0.1);
    
    let value = XffValue::from(obj);
    assert!(value.is_object());
    
    if let Some(name) = value["project"].as_string() {
        println!("Project: {}", name);
    }
}
```

### Graph Example

```rust
use aequa::{graph::Graph, XffValue};

fn main() {
    let mut graph = Graph::new();

    // Add nodes - In the format of (payload, metadata)
    let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("node1"), XffValue::Null);
    let n2 = graph.add_node(XffValue::from("node2"), XffValue::Null);

    assert_eq!(n0, 0);
    assert_eq!(n1, 1);
    assert_eq!(n2, 2);

    assert!(graph.get_node(n1).is_some());

    graph.remove_node(n1);
    assert!(graph.get_node(n1).is_none());

    // New node should reuse index 1
    let n3 = graph.add_node(XffValue::from("node3"), XffValue::Null);
    assert_eq!(n3, 1);
    assert_eq!(
        graph.get_node(n3).unwrap().payload.as_string().unwrap(),
        "node3"
    );

    // Add connections
    let c1 = graph.add_connection(n0, n1, XffValue::Null);

    assert!(c1.is_ok());
    let c1 = c1.unwrap();
    assert!(graph.get_connection(c1).is_some());

    let c1_parent = graph.get_connection(c1).unwrap().from;
    assert_eq!(c1_parent, n0);

    let c1_child = graph.get_connection(c1).unwrap().to;
    assert_eq!(c1_child, n1);

    let c1_metadata = &graph.get_connection(c1).unwrap().metadata;
    assert!(c1_metadata.is_null());

    graph.remove_connection(c1);
    assert!(graph.get_connection(c1).is_none());

    // New connection should reuse index 0
    let c2 = graph.add_connection(n0, n2, XffValue::Null).unwrap();
    assert_eq!(c2, 0);
    assert_eq!(graph.get_connection(c2).unwrap().from, n0);
    assert_eq!(graph.get_connection(c2).unwrap().to, n2);
    assert!(graph.get_connection(c2).unwrap().metadata.is_null());
}
```

## Building and Testing

As a standard Rust library:

```bash
cargo build
cargo test
cargo test --examples
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
