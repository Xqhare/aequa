# Aequa

A library for representing and manipulating values.

It's the home to `XffValue`, the core value interchange format for my projects and ecosystem.

This library is not supposed to be used directly, consider using [Athena](https://github.com/xqhare/athena), my rusty toolbox also reexporting `XffValue`, instead.

To learn more about `XffValue`, see [the spec](https://github.com/Xqhare/nabu/tree/master/specifications).

It follows my "no dependencies but the standard library and libc" philosophy.
Learn more [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Features

- `XffValue`
- `HpFloat`
  - High-precision floating point value, not yet incorporated into `XffValue`

## Naming

As with all my projects, it is named after an ancient deity.
Learn more [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

Aequa was the roman divine personification of equity, fairness, exact exchange and accurate weights and measurements.

## Usage

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aequa = { git = "https://github.com/xqhare/aequa" }
```

