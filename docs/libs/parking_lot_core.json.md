# Crate Documentation

**Version:** 0.9.10

**Format Version:** 43

# Module `parking_lot_core`

This library exposes a low-level API for creating your own efficient
synchronization primitives.

# The parking lot

To keep synchronization primitives small, all thread queuing and suspending
functionality is offloaded to the *parking lot*. The idea behind this is based
on the Webkit [`WTF::ParkingLot`](https://webkit.org/blog/6161/locking-in-webkit/)
class, which essentially consists of a hash table mapping of lock addresses
to queues of parked (sleeping) threads. The Webkit parking lot was itself
inspired by Linux [futexes](http://man7.org/linux/man-pages/man2/futex.2.html),
but it is more powerful since it allows invoking callbacks while holding a
queue lock.

There are two main operations that can be performed on the parking lot:

 - *Parking* refers to suspending the thread while simultaneously enqueuing it
on a queue keyed by some address.
- *Unparking* refers to dequeuing a thread from a queue keyed by some address
and resuming it.

See the documentation of the individual functions for more details.

# Building custom synchronization primitives

Building custom synchronization primitives is very simple since the parking
lot takes care of all the hard parts for you. A simple example for a
custom primitive would be to integrate a `Mutex` inside another data type.
Since a mutex only requires 2 bits, it can share space with other data.
For example, one could create an `ArcMutex` type that combines the atomic
reference count and the two mutex bits in the same atomic word.

## Re-exports

### Re-export `deadlock`

```rust
pub use self::parking_lot::deadlock;
```

### Re-export `park`

```rust
pub use self::parking_lot::park;
```

### Re-export `unpark_all`

```rust
pub use self::parking_lot::unpark_all;
```

### Re-export `unpark_filter`

```rust
pub use self::parking_lot::unpark_filter;
```

### Re-export `unpark_one`

```rust
pub use self::parking_lot::unpark_one;
```

### Re-export `unpark_requeue`

```rust
pub use self::parking_lot::unpark_requeue;
```

### Re-export `FilterOp`

```rust
pub use self::parking_lot::FilterOp;
```

### Re-export `ParkResult`

```rust
pub use self::parking_lot::ParkResult;
```

### Re-export `ParkToken`

```rust
pub use self::parking_lot::ParkToken;
```

### Re-export `RequeueOp`

```rust
pub use self::parking_lot::RequeueOp;
```

### Re-export `UnparkResult`

```rust
pub use self::parking_lot::UnparkResult;
```

### Re-export `UnparkToken`

```rust
pub use self::parking_lot::UnparkToken;
```

### Re-export `DEFAULT_PARK_TOKEN`

```rust
pub use self::parking_lot::DEFAULT_PARK_TOKEN;
```

### Re-export `DEFAULT_UNPARK_TOKEN`

```rust
pub use self::parking_lot::DEFAULT_UNPARK_TOKEN;
```

### Re-export `SpinWait`

```rust
pub use self::spinwait::SpinWait;
```

