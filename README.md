# Capitalize

[![Crates.io](https://img.shields.io/crates/v/capitalize)](https://crates.io/crates/capitalize)
[![Crates.io](https://img.shields.io/crates/l/capitalize)](https://unlicense.org/)
[![Crates.io](https://img.shields.io/crates/d/capitalize)](https://crates.io/crates/capitalize)

Change first character to upper case and the rest to lower case.

Only affects Unicode characters equivalent in ASCII. `Capitalize` is implemented for all types that implement [`AsRef<str>`].

## Example

```rust
use capitalize::Capitalize;

assert_eq!(
    "heLLo 😊 World!".capitalize(),
    String::from("Hello 😊 world!")
);
```

[`AsRef<str>`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
