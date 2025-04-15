# Crate Documentation

**Version:** 0.9.18

**Format Version:** 43

# Module `crossbeam_epoch`

Epoch-based memory reclamation.

An interesting problem concurrent collections deal with comes from the remove operation.
Suppose that a thread removes an element from a lock-free map, while another thread is reading
that same element at the same time. The first thread must wait until the second thread stops
reading the element. Only then it is safe to destruct it.

Programming languages that come with garbage collectors solve this problem trivially. The
garbage collector will destruct the removed element when no thread can hold a reference to it
anymore.

This crate implements a basic memory reclamation mechanism, which is based on epochs. When an
element gets removed from a concurrent collection, it is inserted into a pile of garbage and
marked with the current epoch. Every time a thread accesses a collection, it checks the current
epoch, attempts to increment it, and destructs some garbage that became so old that no thread
can be referencing it anymore.

That is the general mechanism behind epoch-based memory reclamation, but the details are a bit
more complicated. Anyhow, memory reclamation is designed to be fully automatic and something
users of concurrent collections don't have to worry much about.

# Pointers

Concurrent collections are built using atomic pointers. This module provides [`Atomic`], which
is just a shared atomic pointer to a heap-allocated object. Loading an [`Atomic`] yields a
[`Shared`], which is an epoch-protected pointer through which the loaded object can be safely
read.

# Pinning

Before an [`Atomic`] can be loaded, a participant must be [`pin`]ned. By pinning a participant
we declare that any object that gets removed from now on must not be destructed just
yet. Garbage collection of newly removed objects is suspended until the participant gets
unpinned.

# Garbage

Objects that get removed from concurrent collections must be stashed away until all currently
pinned participants get unpinned. Such objects can be stored into a thread-local or global
storage, where they are kept until the right time for their destruction comes.

There is a global shared instance of garbage queue. You can [`defer`](Guard::defer) the execution of an
arbitrary function until the global epoch is advanced enough. Most notably, concurrent data
structures may defer the deallocation of an object.

# APIs

For majority of use cases, just use the default garbage collector by invoking [`pin`]. If you
want to create your own garbage collector, use the [`Collector`] API.

## Re-exports

### Re-export `CompareAndSetError`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`
- `#[allow(deprecated)]`

```rust
pub use crate::atomic::CompareAndSetError;
```

### Re-export `CompareAndSetOrdering`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`
- `#[allow(deprecated)]`

```rust
pub use crate::atomic::CompareAndSetOrdering;
```

### Re-export `Atomic`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::atomic::Atomic;
```

### Re-export `CompareExchangeError`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::atomic::CompareExchangeError;
```

### Re-export `Owned`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::atomic::Owned;
```

### Re-export `Pointable`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::atomic::Pointable;
```

### Re-export `Pointer`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::atomic::Pointer;
```

### Re-export `Shared`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::atomic::Shared;
```

### Re-export `Collector`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::collector::Collector;
```

### Re-export `LocalHandle`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::collector::LocalHandle;
```

### Re-export `unprotected`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::guard::unprotected;
```

### Re-export `Guard`

**Attributes:**

- `#[<cfg>(all(feature = "alloc", target_has_atomic = "ptr"))]`

```rust
pub use crate::guard::Guard;
```

### Re-export `default_collector`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::default::default_collector;
```

### Re-export `is_pinned`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::default::is_pinned;
```

### Re-export `pin`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::default::pin;
```

