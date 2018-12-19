# v_escape [![Documentation](https://docs.rs/v_escape/badge.svg)](https://docs.rs/v_escape/) [![Latest version](https://img.shields.io/crates/v/v_escape.svg)](https://crates.io/crates/v_escape)
> The simd optimized escape code
# Quick start
 
```rust
#[macro_use]
extern crate v_escape;

new_escape_sized!(MyEscape, "62->bar || ");

fn main() {
    let s = b"foo<bar";
    let escaped = MyEscape::new(s);
    
    print!("#{} : {}", escaped.size(), escaped);
}
```
