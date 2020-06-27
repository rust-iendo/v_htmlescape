# v_jsonescape [![Documentation](https://docs.rs/v_jsonescape/badge.svg)](https://docs.rs/v_jsonescape/) [![Latest version](https://img.shields.io/crates/v/v_jsonescape.svg)](https://crates.io/crates/v_jsonescape)
> The simd optimized json escape code
# Quick start
 
```rust
extern crate v_jsonescape;
use v_jsonescape::escape;

print!("{}", escape("foo\"\\bar"));
```
