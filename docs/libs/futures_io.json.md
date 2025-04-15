# Crate Documentation

**Version:** 0.3.31

**Format Version:** 43

# Module `futures_io`

Asynchronous I/O

This crate contains the `AsyncRead`, `AsyncWrite`, `AsyncSeek`, and
`AsyncBufRead` traits, the asynchronous analogs to
`std::io::{Read, Write, Seek, BufRead}`. The primary difference is
that these traits integrate with the asynchronous task system.

All items of this library are only available when the `std` feature of this
library is activated, and it is activated by default.

## Re-exports

### Re-export `self::if_std::*`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use self::if_std::*;
```

