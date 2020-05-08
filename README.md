Yet Another Units of Measurement library [![crates.io](https://img.shields.io/crates/v/yaum.svg)](https://crates.io/crates/yaum)
========================================

This crate provides type-safe basic scientific units and constants for `no_std` programs.

[Documentation](https://docs.rs/yaum/)

Example
-------

```rust
use yaum::time::*;
use yaum::length::*;
use yaum::velocity::*;

// Simple arithmetic
assert_eq!(1.0 * kph, 1.0 * km / h);
assert_eq!(1.0 * km / min, 60.0 * km / h);

// Read value in a given unit
assert_eq!(60.0, (1.0 * min).s());
assert_eq!(1_000.0, (1.0 * km).m());
```

Currently supported units:
* `time`: Time
* `frequency`: Frequency
* `length`: Length
* `velocity`: Velocity, Acceleration
* `digital`: LSB (least significant bits)

Define custom units and conversions using the `impl_unit!`, `convert_div!` and `convert_unit!` macros.

```rust
# #[macro_use] extern crate yaum;
use yaum::*;
use yaum::time::*;

yaum::impl_unit!(ByteSize, {
    B: 1.0,
    kB: 1024.0,
    MB: 1024.0 * 1024.0
});
yaum::impl_unit!(BitSize, {
    b: 1.0,
    kb: 1024.0,
    Mb: 1024.0 * 1024.0
});

// define conversion between the two units (1 byte = 8 bits):
yaum::convert_unit!(ByteSize, BitSize, 8.0);

yaum::impl_unit!(BitSpeed, {
    bps: 1.0,
    kbps: 1024.0,
    Mbps: 1024.0 * 1024.0
});

// define relationship between units (BitSpeed = BitSize/Time)
yaum::convert_div!(BitSize, Time, BitSpeed);

fn main() {
    assert_eq!(8.0 * b, (1.0 * B).into());
    assert_eq!(1.0 * kbps, 1.0 * kb/s);
}
```

Precision
---------

By default, units are implemented on top of `f32`. Enable the `double_precision` feature for `f64`.

```TOML
[dependencies.yaum]
version = "0.1.0"
default-features = false
features = ["double_precision"]
```
