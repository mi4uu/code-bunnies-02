# Crate Documentation

**Version:** 0.4.12

**Format Version:** 43

# Module `lock_api`

This library provides type-safe and fully-featured [`Mutex`] and [`RwLock`]
types which wrap a simple raw mutex or rwlock type. This has several
benefits: not only does it eliminate a large portion of the work in
implementing custom lock types, it also allows users to write code which is
generic with regards to different lock implementations.

Basic usage of this crate is very straightforward:

1. Create a raw lock type. This should only contain the lock state, not any
   data protected by the lock.
2. Implement the `RawMutex` trait for your custom lock type.
3. Export your mutex as a type alias for `lock_api::Mutex`, and
   your mutex guard as a type alias for `lock_api::MutexGuard`.
   See the [example](#example) below for details.

This process is similar for [`RwLock`]s, except that two guards need to be
exported instead of one. (Or 3 guards if your type supports upgradable read
locks, see [extension traits](#extension-traits) below for details)

# Example

```
use lock_api::{RawMutex, Mutex, GuardSend};
use std::sync::atomic::{AtomicBool, Ordering};

// 1. Define our raw lock type
pub struct RawSpinlock(AtomicBool);

// 2. Implement RawMutex for this type
unsafe impl RawMutex for RawSpinlock {
    const INIT: RawSpinlock = RawSpinlock(AtomicBool::new(false));

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    fn lock(&self) {
        // Note: This isn't the best way of implementing a spinlock, but it
        // suffices for the sake of this example.
        while !self.try_lock() {}
    }

    fn try_lock(&self) -> bool {
        self.0
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        self.0.store(false, Ordering::Release);
    }
}

// 3. Export the wrappers. This are the types that your users will actually use.
pub type Spinlock<T> = lock_api::Mutex<RawSpinlock, T>;
pub type SpinlockGuard<'a, T> = lock_api::MutexGuard<'a, RawSpinlock, T>;
```

# Extension traits

In addition to basic locking & unlocking functionality, you have the option
of exposing additional functionality in your lock types by implementing
additional traits for it. Examples of extension features include:

- Fair unlocking (`RawMutexFair`, `RawRwLockFair`)
- Lock timeouts (`RawMutexTimed`, `RawRwLockTimed`)
- Downgradable write locks (`RawRwLockDowngradable`)
- Recursive read locks (`RawRwLockRecursive`)
- Upgradable read locks (`RawRwLockUpgrade`)

The `Mutex` and `RwLock` wrappers will automatically expose this additional
functionality if the raw lock type implements these extension traits.

# Cargo features

This crate supports three cargo features:

- `owning_ref`: Allows your lock types to be used with the `owning_ref` crate.
- `arc_lock`: Enables locking from an `Arc`. This enables types such as `ArcMutexGuard`. Note that this
  requires the `alloc` crate to be present.

## Types

### Struct `GuardSend`

Marker type which indicates that the Guard type for a lock is `Send`.

```rust
pub struct GuardSend(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `GuardNoSend`

Marker type which indicates that the Guard type for a lock is not `Send`.

```rust
pub struct GuardNoSend(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Unpin**
- **Send**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Re-exports

### Re-export `crate::mutex::*`

```rust
pub use crate::mutex::*;
```

### Re-export `crate::remutex::*`

**Attributes:**

- `#[<cfg>(feature = "atomic_usize")]`

```rust
pub use crate::remutex::*;
```

### Re-export `crate::rwlock::*`

```rust
pub use crate::rwlock::*;
```

