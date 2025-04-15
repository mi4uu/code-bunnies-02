# Crate Documentation

**Version:** 0.4.0

**Format Version:** 43

# Module `retry_policies`

A collection of plug-and-play retry policies.

## Modules

## Module `policies`

```rust
pub mod policies { /* ... */ }
```

### Re-exports

#### Re-export `ExponentialBackoff`

```rust
pub use exponential_backoff::ExponentialBackoff;
```

#### Re-export `ExponentialBackoffBuilder`

```rust
pub use exponential_backoff::ExponentialBackoffBuilder;
```

#### Re-export `ExponentialBackoffTimed`

```rust
pub use exponential_backoff::ExponentialBackoffTimed;
```

## Re-exports

### Re-export `Jitter`

```rust
pub use retry_policy::Jitter;
```

### Re-export `RetryDecision`

```rust
pub use retry_policy::RetryDecision;
```

### Re-export `RetryPolicy`

```rust
pub use retry_policy::RetryPolicy;
```

