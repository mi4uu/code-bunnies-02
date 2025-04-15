# Crate Documentation

**Version:** 1.1.2

**Format Version:** 43

# Module `atomic_waker`

`futures::task::AtomicWaker` extracted into its own crate.

# Features

This crate adds a feature, `portable-atomic`, which uses a polyfill
from the [`portable-atomic`] crate in order to provide functionality
to targets without atomics. See the [`README`] for the [`portable-atomic`]
crate for more information on how to use it.

[`portable-atomic`]: https://crates.io/crates/portable-atomic
[`README`]: https://github.com/taiki-e/portable-atomic/blob/main/README.md#optional-cfg

## Types

### Struct `AtomicWaker`

A synchronization primitive for task wakeup.

Sometimes the task interested in a given event will change over time.
An `AtomicWaker` can coordinate concurrent notifications with the consumer
potentially "updating" the underlying task to wake up. This is useful in
scenarios where a computation completes in another thread and wants to
notify the consumer, but the consumer is in the process of being migrated to
a new logical task.

Consumers should call `register` before checking the result of a computation
and producers should call `wake` after producing the computation (this
differs from the usual `thread::park` pattern). It is also permitted for
`wake` to be called **before** `register`. This results in a no-op.

A single `AtomicWaker` may be reused for any number of calls to `register` or
`wake`.

# Memory ordering

Calling `register` "acquires" all memory "released" by calls to `wake`
before the call to `register`.  Later calls to `wake` will wake the
registered waker (on contention this wake might be triggered in `register`).

For concurrent calls to `register` (should be avoided) the ordering is only
guaranteed for the winning call.

# Examples

Here is a simple example providing a `Flag` that can be signalled manually
when it is ready.

```
use futures::future::Future;
use futures::task::{Context, Poll, AtomicWaker};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::pin::Pin;

struct Inner {
    waker: AtomicWaker,
    set: AtomicBool,
}

#[derive(Clone)]
struct Flag(Arc<Inner>);

impl Flag {
    pub fn new() -> Self {
        Flag(Arc::new(Inner {
            waker: AtomicWaker::new(),
            set: AtomicBool::new(false),
        }))
    }

    pub fn signal(&self) {
        self.0.set.store(true, Relaxed);
        self.0.waker.wake();
    }
}

impl Future for Flag {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // quick check to avoid registration if already done.
        if self.0.set.load(Relaxed) {
            return Poll::Ready(());
        }

        self.0.waker.register(cx.waker());

        // Need to check condition **after** `register` to avoid a race
        // condition that would result in lost notifications.
        if self.0.set.load(Relaxed) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
```

```rust
pub struct AtomicWaker {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Create an `AtomicWaker`.

- ```rust
  pub fn register(self: &Self, waker: &Waker) { /* ... */ }
  ```
  Registers the waker to be notified on calls to `wake`.

- ```rust
  pub fn wake(self: &Self) { /* ... */ }
  ```
  Calls `wake` on the last `Waker` passed to `register`.

- ```rust
  pub fn take(self: &Self) -> Option<Waker> { /* ... */ }
  ```
  Returns the last `Waker` passed to `register`, so that the user can wake it.

##### Trait Implementations

- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
