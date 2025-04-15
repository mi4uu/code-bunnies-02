# Crate Documentation

**Version:** 1.70.1

**Format Version:** 43

# Module `is_terminal_polyfill`

> Polyfill for `is_terminal` stdlib feature for use with older MSRVs

## Traits

### Trait `IsTerminal`

Trait to determine if a descriptor/handle refers to a terminal/tty.

```rust
pub trait IsTerminal: sealed::Sealed {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `is_terminal`: Returns `true` if the descriptor/handle refers to a terminal/tty.

#### Implementations

This trait is implemented for the following types:

- `std::fs::File`
- `std::io::Stdin`
- `std::io::StdinLock<''_>`
- `std::io::Stdout`
- `std::io::StdoutLock<''_>`
- `std::io::Stderr`
- `std::io::StderrLock<''_>`

