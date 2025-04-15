# Crate Documentation

**Version:** 0.3.31

**Format Version:** 43

# Module `futures_executor`

Built-in executors and related tools.

All asynchronous computation occurs within an executor, which is
capable of spawning futures as tasks. This module provides several
built-in executors, as well as tools for building your own.

All items are only available when the `std` feature of this
library is activated, and it is activated by default.

# Using a thread pool (M:N task scheduling)

Most of the time tasks should be executed on a [thread pool](ThreadPool).
A small set of worker threads can handle a very large set of spawned tasks
(which are much lighter weight than threads). Tasks spawned onto the pool
with the [`spawn_ok`](ThreadPool::spawn_ok) function will run ambiently on
the created threads.

# Spawning additional tasks

Tasks can be spawned onto a spawner by calling its [`spawn_obj`] method
directly. In the case of `!Send` futures, [`spawn_local_obj`] can be used
instead.

# Single-threaded execution

In addition to thread pools, it's possible to run a task (and the tasks
it spawns) entirely within a single thread via the [`LocalPool`] executor.
Aside from cutting down on synchronization costs, this executor also makes
it possible to spawn non-`Send` tasks, via [`spawn_local_obj`]. The
[`LocalPool`] is best suited for running I/O-bound tasks that do relatively
little work between I/O operations.

There is also a convenience function [`block_on`] for simply running a
future to completion on the current thread.

[`spawn_obj`]: https://docs.rs/futures/0.3/futures/task/trait.Spawn.html#tymethod.spawn_obj
[`spawn_local_obj`]: https://docs.rs/futures/0.3/futures/task/trait.LocalSpawn.html#tymethod.spawn_local_obj

## Re-exports

### Re-export `block_on`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::local_pool::block_on;
```

### Re-export `block_on_stream`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::local_pool::block_on_stream;
```

### Re-export `BlockingStream`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::local_pool::BlockingStream;
```

### Re-export `LocalPool`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::local_pool::LocalPool;
```

### Re-export `LocalSpawner`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::local_pool::LocalSpawner;
```

### Re-export `enter`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::enter::enter;
```

### Re-export `Enter`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::enter::Enter;
```

### Re-export `EnterError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::enter::EnterError;
```

