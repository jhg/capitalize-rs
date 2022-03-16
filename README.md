# Capitalize

[![Crates.io](https://img.shields.io/crates/v/capitalize)](https://crates.io/crates/capitalize)
[![Crates.io](https://img.shields.io/crates/l/capitalize)](https://unlicense.org/)
[![Crates.io](https://img.shields.io/crates/d/capitalize)](https://crates.io/crates/capitalize)

First letter to uppercase, the rest to lowercase.

## Examples

```rust
use capitalize::Capitalize;

assert_eq!("hello ✨ world".capitalize(), "Hello ✨ world");
```

Behavior is like [Python's `str.capitalize`], read [`capitalize` reference][Capitalize::capitalize] for details.

[Capitalize::capitalize]: https://docs.rs/capitalize/latest/capitalize/trait.Capitalize.html#tymethod.capitalize
[Python's `str.capitalize`]: https://docs.python.org/3/library/stdtypes.html#str.capitalize
