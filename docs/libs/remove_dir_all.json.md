# Crate Documentation

**Version:** 0.5.3

**Format Version:** 43

# Module `remove_dir_all`

Reliably remove a directory and all of its children.

This library provides a reliable implementation of `remove_dir_all` for Windows.
For Unix systems, it re-exports `std::fs::remove_dir_all`.

## Re-exports

### Re-export `remove_dir_all`

**Attributes:**

- `#[<cfg>(not(windows))]`

```rust
pub use std::fs::remove_dir_all;
```

