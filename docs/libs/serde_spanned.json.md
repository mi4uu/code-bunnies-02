# Crate Documentation

**Version:** 0.6.8

**Format Version:** 43

# Module `serde_spanned`

A [serde]-compatible spanned Value

This allows capturing the location, in bytes, for a value in the original parsed document for
compatible deserializers.

[serde]: https://serde.rs/

## Re-exports

### Re-export `Spanned`

```rust
pub use crate::spanned::Spanned;
```

