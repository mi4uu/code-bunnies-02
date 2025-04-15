# Crate Documentation

**Version:** 0.4.16

**Format Version:** 43

# Module `is_terminal`

is-terminal is a simple utility that answers one question:

> Is this a terminal?

A "terminal", also known as a "tty", is an I/O device which may be
interactive and may support color and other special features. This crate
doesn't provide any of those features; it just answers this one question.

On Unix-family platforms, this is effectively the same as the [`isatty`]
function for testing whether a given stream is a terminal, though it
accepts high-level stream types instead of raw file descriptors.

On Windows, it uses a variety of techniques to determine whether the
given stream is a terminal.

# Example

```rust
use is_terminal::IsTerminal;

if std::io::stdout().is_terminal() {
    println!("stdout is a terminal")
}
```

[`isatty`]: https://man7.org/linux/man-pages/man3/isatty.3.html

## Traits

### Trait `IsTerminal`

Extension trait to check whether something is a terminal.

```rust
pub trait IsTerminal {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `is_terminal`: Returns true if this is a terminal.

#### Implementations

This trait is implemented for the following types:

- `Stream` with <Stream: AsFd>

## Functions

### Function `is_terminal`

Returns `true` if `this` is a terminal.

This is equivalent to calling `this.is_terminal()` and exists only as a
convenience to calling the trait method [`IsTerminal::is_terminal`]
without importing the trait.

# Example

```
if is_terminal::is_terminal(&std::io::stdout()) {
    println!("stdout is a terminal")
}
```

```rust
pub fn is_terminal<T: IsTerminal>(this: T) -> bool { /* ... */ }
```

