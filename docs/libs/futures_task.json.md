# Crate Documentation

**Version:** 0.3.31

**Format Version:** 43

# Module `futures_task`

Tools for working with tasks.

## Re-exports

### Re-export `LocalSpawn`

```rust
pub use crate::spawn::LocalSpawn;
```

### Re-export `Spawn`

```rust
pub use crate::spawn::Spawn;
```

### Re-export `SpawnError`

```rust
pub use crate::spawn::SpawnError;
```

### Re-export `ArcWake`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::arc_wake::ArcWake;
```

### Re-export `waker`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::waker::waker;
```

### Re-export `waker_ref`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::waker_ref::waker_ref;
```

### Re-export `WakerRef`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::waker_ref::WakerRef;
```

### Re-export `FutureObj`

```rust
pub use crate::future_obj::FutureObj;
```

### Re-export `LocalFutureObj`

```rust
pub use crate::future_obj::LocalFutureObj;
```

### Re-export `UnsafeFutureObj`

```rust
pub use crate::future_obj::UnsafeFutureObj;
```

### Re-export `noop_waker`

```rust
pub use crate::noop_waker::noop_waker;
```

### Re-export `noop_waker_ref`

```rust
pub use crate::noop_waker::noop_waker_ref;
```

### Re-export `Context`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::Context;
```

### Re-export `Poll`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::Poll;
```

### Re-export `RawWaker`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::RawWaker;
```

### Re-export `RawWakerVTable`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::RawWakerVTable;
```

### Re-export `Waker`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::Waker;
```

