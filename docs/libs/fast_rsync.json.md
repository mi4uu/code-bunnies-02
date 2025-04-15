# Crate Documentation

**Version:** 0.2.0

**Format Version:** 43

# Module `fast_rsync`

A faster implementation of [librsync](https://github.com/librsync/librsync) in
pure Rust.

This crate offers three major APIs:

1. [Signature::calculate()], which takes a block of data and returns a
   "signature" of that data which is much smaller than the original data.
2. [diff()], which takes a signature for some block A, and a block of data B, and
   returns a delta between block A and block B. If A and B are "similar", then
   the delta is usually much smaller than block B.
3. [apply()], which takes a block A and a delta (as constructed by [diff()]), and
   (usually) returns the block B.

## Re-exports

### Re-export `diff`

```rust
pub use diff::diff;
```

### Re-export `DiffError`

```rust
pub use diff::DiffError;
```

### Re-export `apply`

```rust
pub use patch::apply;
```

### Re-export `apply_limited`

```rust
pub use patch::apply_limited;
```

### Re-export `ApplyError`

```rust
pub use patch::ApplyError;
```

### Re-export `IndexedSignature`

```rust
pub use signature::IndexedSignature;
```

### Re-export `Signature`

```rust
pub use signature::Signature;
```

### Re-export `SignatureOptions`

```rust
pub use signature::SignatureOptions;
```

### Re-export `SignatureParseError`

```rust
pub use signature::SignatureParseError;
```

