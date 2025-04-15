# Crate Documentation

**Version:** 0.3.1

**Format Version:** 43

# Module `rand_chacha`

The ChaCha random number generator.

## Types

### Type Alias `ChaChaRng`

ChaCha with 20 rounds

```rust
pub type ChaChaRng = ChaCha20Rng;
```

### Type Alias `ChaChaCore`

ChaCha with 20 rounds, low-level interface

```rust
pub type ChaChaCore = ChaCha20Core;
```

## Re-exports

### Re-export `rand_core`

```rust
pub use rand_core;
```

### Re-export `ChaCha12Core`

```rust
pub use crate::chacha::ChaCha12Core;
```

### Re-export `ChaCha12Rng`

```rust
pub use crate::chacha::ChaCha12Rng;
```

### Re-export `ChaCha20Core`

```rust
pub use crate::chacha::ChaCha20Core;
```

### Re-export `ChaCha20Rng`

```rust
pub use crate::chacha::ChaCha20Rng;
```

### Re-export `ChaCha8Core`

```rust
pub use crate::chacha::ChaCha8Core;
```

### Re-export `ChaCha8Rng`

```rust
pub use crate::chacha::ChaCha8Rng;
```

