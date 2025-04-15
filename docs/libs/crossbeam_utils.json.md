# Crate Documentation

**Version:** 0.8.21

**Format Version:** 43

# Module `crossbeam_utils`

Miscellaneous tools for concurrent programming.

## Atomics

* [`AtomicCell`], a thread-safe mutable memory location.
* [`AtomicConsume`], for reading from primitive atomic types with "consume" ordering.

## Thread synchronization

* [`Parker`], a thread parking primitive.
* [`ShardedLock`], a sharded reader-writer lock with fast concurrent reads.
* [`WaitGroup`], for synchronizing the beginning or end of some computation.

## Utilities

* [`Backoff`], for exponential backoff in spin loops.
* [`CachePadded`], for padding and aligning a value to the length of a cache line.
* [`scope`], for spawning threads that borrow local variables from the stack.

[`AtomicCell`]: atomic::AtomicCell
[`AtomicConsume`]: atomic::AtomicConsume
[`Parker`]: sync::Parker
[`ShardedLock`]: sync::ShardedLock
[`WaitGroup`]: sync::WaitGroup
[`scope`]: thread::scope

## Modules

## Module `atomic`

Atomic types.

* [`AtomicCell`], a thread-safe mutable memory location.
* [`AtomicConsume`], for reading from primitive atomic types with "consume" ordering.

```rust
pub mod atomic { /* ... */ }
```

### Re-exports

#### Re-export `AtomicCell`

**Attributes:**

- `#[<cfg>(target_has_atomic = "ptr")]`
- `#[<cfg>(not(crossbeam_loom))]`

```rust
pub use atomic_cell::AtomicCell;
```

#### Re-export `AtomicConsume`

```rust
pub use consume::AtomicConsume;
```

## Module `sync`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Thread synchronization primitives.

* [`Parker`], a thread parking primitive.
* [`ShardedLock`], a sharded reader-writer lock with fast concurrent reads.
* [`WaitGroup`], for synchronizing the beginning or end of some computation.

```rust
pub mod sync { /* ... */ }
```

### Re-exports

#### Re-export `Parker`

```rust
pub use self::parker::Parker;
```

#### Re-export `Unparker`

```rust
pub use self::parker::Unparker;
```

#### Re-export `ShardedLock`

**Attributes:**

- `#[<cfg>(not(crossbeam_loom))]`

```rust
pub use self::sharded_lock::ShardedLock;
```

#### Re-export `ShardedLockReadGuard`

**Attributes:**

- `#[<cfg>(not(crossbeam_loom))]`

```rust
pub use self::sharded_lock::ShardedLockReadGuard;
```

#### Re-export `ShardedLockWriteGuard`

**Attributes:**

- `#[<cfg>(not(crossbeam_loom))]`

```rust
pub use self::sharded_lock::ShardedLockWriteGuard;
```

#### Re-export `WaitGroup`

```rust
pub use self::wait_group::WaitGroup;
```

## Module `thread`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg>(not(crossbeam_loom))]`

Threads that can borrow variables from the stack.

Create a scope when spawned threads need to access variables on the stack:

```
use crossbeam_utils::thread;

let people = vec![
    "Alice".to_string(),
    "Bob".to_string(),
    "Carol".to_string(),
];

thread::scope(|s| {
    for person in &people {
        s.spawn(move |_| {
            println!("Hello, {}!", person);
        });
    }
}).unwrap();
```

# Why scoped threads?

Suppose we wanted to re-write the previous example using plain threads:

```compile_fail,E0597
use std::thread;

let people = vec![
    "Alice".to_string(),
    "Bob".to_string(),
    "Carol".to_string(),
];

let mut threads = Vec::new();

for person in &people {
    threads.push(thread::spawn(move || {
        println!("Hello, {}!", person);
    }));
}

for thread in threads {
    thread.join().unwrap();
}
```

This doesn't work because the borrow checker complains about `people` not living long enough:

```text
error[E0597]: `people` does not live long enough
  --> src/main.rs:12:20
   |
12 |     for person in &people {
   |                    ^^^^^^ borrowed value does not live long enough
...
21 | }
   | - borrowed value only lives until here
   |
   = note: borrowed value must be valid for the static lifetime...
```

The problem here is that spawned threads are not allowed to borrow variables on stack because
the compiler cannot prove they will be joined before `people` is destroyed.

Scoped threads are a mechanism to guarantee to the compiler that spawned threads will be joined
before the scope ends.

# How scoped threads work

If a variable is borrowed by a thread, the thread must complete before the variable is
destroyed. Threads spawned using [`std::thread::spawn`] can only borrow variables with the
`'static` lifetime because the borrow checker cannot be sure when the thread will complete.

A scope creates a clear boundary between variables outside the scope and threads inside the
scope. Whenever a scope spawns a thread, it promises to join the thread before the scope ends.
This way we guarantee to the borrow checker that scoped threads only live within the scope and
can safely access variables outside it.

# Nesting scoped threads

Sometimes scoped threads need to spawn more threads within the same scope. This is a little
tricky because argument `s` lives *inside* the invocation of `thread::scope()` and as such
cannot be borrowed by scoped threads:

```compile_fail,E0521
use crossbeam_utils::thread;

thread::scope(|s| {
    s.spawn(|_| {
        // Not going to compile because we're trying to borrow `s`,
        // which lives *inside* the scope! :(
        s.spawn(|_| println!("nested thread"));
    });
});
```

Fortunately, there is a solution. Every scoped thread is passed a reference to its scope as an
argument, which can be used for spawning nested threads:

```
use crossbeam_utils::thread;

thread::scope(|s| {
    // Note the `|s|` here.
    s.spawn(|s| {
        // Yay, this works because we're using a fresh argument `s`! :)
        s.spawn(|_| println!("nested thread"));
    });
}).unwrap();
```

```rust
pub mod thread { /* ... */ }
```

### Types

#### Struct `Scope`

A scope for spawning threads.

```rust
pub struct Scope<''env> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn spawn<''scope, F, T>(self: &''scope Self, f: F) -> ScopedJoinHandle<''scope, T>
where
    F: FnOnce(&Scope<''env>) -> T + Send + ''env,
    T: Send + ''env { /* ... */ }
  ```
  Spawns a scoped thread.

- ```rust
  pub fn builder<''scope>(self: &''scope Self) -> ScopedThreadBuilder<''scope, ''env> { /* ... */ }
  ```
  Creates a builder that can configure a thread before spawning.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `ScopedThreadBuilder`

Configures the properties of a new thread.

The two configurable properties are:

- [`name`]: Specifies an [associated name for the thread][naming-threads].
- [`stack_size`]: Specifies the [desired stack size for the thread][stack-size].

The [`spawn`] method will take ownership of the builder and return an [`io::Result`] of the
thread handle with the given configuration.

The [`Scope::spawn`] method uses a builder with default configuration and unwraps its return
value. You may want to use this builder when you want to recover from a failure to launch a
thread.

# Examples

```
use crossbeam_utils::thread;

thread::scope(|s| {
    s.builder()
        .spawn(|_| println!("Running a child thread"))
        .unwrap();
}).unwrap();
```

[`name`]: ScopedThreadBuilder::name
[`stack_size`]: ScopedThreadBuilder::stack_size
[`spawn`]: ScopedThreadBuilder::spawn
[`io::Result`]: std::io::Result
[naming-threads]: std::thread#naming-threads
[stack-size]: std::thread#stack-size

```rust
pub struct ScopedThreadBuilder<''scope, ''env> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn name(self: Self, name: String) -> ScopedThreadBuilder<''scope, ''env> { /* ... */ }
  ```
  Sets the name for the new thread.

- ```rust
  pub fn stack_size(self: Self, size: usize) -> ScopedThreadBuilder<''scope, ''env> { /* ... */ }
  ```
  Sets the size of the stack for the new thread.

- ```rust
  pub fn spawn<F, T>(self: Self, f: F) -> io::Result<ScopedJoinHandle<''scope, T>>
where
    F: FnOnce(&Scope<''env>) -> T + Send + ''env,
    T: Send + ''env { /* ... */ }
  ```
  Spawns a scoped thread with this configuration.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ScopedJoinHandle`

A handle that can be used to join its scoped thread.

This struct is created by the [`Scope::spawn`] method and the
[`ScopedThreadBuilder::spawn`] method.

```rust
pub struct ScopedJoinHandle<''scope, T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn join(self: Self) -> thread::Result<T> { /* ... */ }
  ```
  Waits for the thread to finish and returns its result.

- ```rust
  pub fn thread(self: &Self) -> &thread::Thread { /* ... */ }
  ```
  Returns a handle to the underlying thread.

###### Trait Implementations

- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **JoinHandleExt**
  - ```rust
    fn as_pthread_t(self: &Self) -> RawPthread { /* ... */ }
    ```

  - ```rust
    fn into_pthread_t(self: Self) -> RawPthread { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
### Functions

#### Function `scope`

Creates a new scope for spawning threads.

All child threads that haven't been manually joined will be automatically joined just before
this function invocation ends. If all joined threads have successfully completed, `Ok` is
returned with the return value of `f`. If any of the joined threads has panicked, an `Err` is
returned containing errors from panicked threads. Note that if panics are implemented by
aborting the process, no error is returned; see the notes of [std::panic::catch_unwind].

**Note:** Since Rust 1.63, this function is soft-deprecated in favor of the more efficient [`std::thread::scope`].

# Examples

```
use crossbeam_utils::thread;

let var = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|_| {
        println!("A child thread borrowing `var`: {:?}", var);
    });
}).unwrap();
```

```rust
pub fn scope<''env, F, R>(f: F) -> thread::Result<R>
where
    F: FnOnce(&Scope<''env>) -> R { /* ... */ }
```

## Re-exports

### Re-export `CachePadded`

```rust
pub use crate::cache_padded::CachePadded;
```

### Re-export `Backoff`

```rust
pub use crate::backoff::Backoff;
```

