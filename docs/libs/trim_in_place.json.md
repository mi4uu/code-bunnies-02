# Crate Documentation

**Version:** 0.1.7

**Format Version:** 43

# Module `trim_in_place`

# Trim in-place

This crate is used for extending `String` in order to do in-place trimming.

## Usage

```rust
use trim_in_place::TrimInPlace;

let mut s = String::from(" 1234 abcd  ");

s.trim_in_place();

assert_eq!("1234 abcd", s);
```

## Benchmark

```bash
cargo bench
```

## Traits

### Trait `TrimInPlace`

```rust
pub trait TrimInPlace {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `trim_in_place`
- `trim_start_in_place`
- `trim_end_in_place`
- `trim_matches_in_place`
- `trim_start_matches_in_place`
- `trim_end_matches_in_place`

#### Implementations

This trait is implemented for the following types:

- `alloc::string::String`

