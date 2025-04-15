# Crate Documentation

**Version:** 1.0.0

**Format Version:** 43

# Module `write16`

`write16` provides the trait `Write16`, which a UTF-16 analog of the
`core::fmt::Write` trait (the sink part—not the formatting part).

## Traits

### Trait `Write16`

A UTF-16 sink analogous to `core::fmt::Write`.

```rust
pub trait Write16 {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `write_slice`: Write a slice containing UTF-16 to the sink.

#### Provided Methods

- ```rust
  fn write_char(self: &mut Self, c: char) -> core::fmt::Result { /* ... */ }
  ```
  Write a Unicode scalar value to the sink.

- ```rust
  fn size_hint(self: &mut Self, upcoming: usize) -> core::fmt::Result { /* ... */ }
  ```
  A hint that the caller expects to write `upcoming` UTF-16

#### Implementations

This trait is implemented for the following types:

- `alloc::vec::Vec<u16>`

