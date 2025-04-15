# Crate Documentation

**Version:** 0.2.5

**Format Version:** 43

# Module `try_lock`

A light-weight lock guarded by an atomic boolean.

Most efficient when contention is low, acquiring the lock is a single
atomic swap, and releasing it just 1 more atomic swap.

# Example

```
use std::sync::Arc;
use try_lock::TryLock;

// a thing we want to share
struct Widget {
    name: String,
}

// lock it up!
let widget1 = Arc::new(TryLock::new(Widget {
    name: "Spanner".into(),
}));

let widget2 = widget1.clone();


// mutate the widget
let mut locked = widget1.try_lock().expect("example isn't locked yet");
locked.name.push_str(" Bundle");

// hands off, buddy
let not_locked = widget2.try_lock();
assert!(not_locked.is_none(), "widget1 has the lock");

// ok, you can have it
drop(locked);

let locked2 = widget2.try_lock().expect("widget1 lock is released");

assert_eq!(locked2.name, "Spanner Bundle");
```

## Types

### Struct `TryLock`

A light-weight lock guarded by an atomic boolean.

Most efficient when contention is low, acquiring the lock is a single
atomic swap, and releasing it just 1 more atomic swap.

It is only possible to try to acquire the lock, it is not possible to
wait for the lock to become ready, like with a `Mutex`.

```rust
pub struct TryLock<T> {
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
  pub const fn new(val: T) -> TryLock<T> { /* ... */ }
  ```
  Create a `TryLock` around the value.

- ```rust
  pub fn try_lock(self: &Self) -> Option<Locked<''_, T>> { /* ... */ }
  ```
  Try to acquire the lock of this value.

- ```rust
  pub fn try_lock_order(self: &Self, lock_order: Ordering, unlock_order: Ordering) -> Option<Locked<''_, T>> { /* ... */ }
  ```
  Try to acquire the lock of this value using the lock and unlock orderings.

- ```rust
  pub fn try_lock_explicit(self: &Self, lock_order: Ordering, unlock_order: Ordering) -> Option<Locked<''_, T>> { /* ... */ }
  ```
  Try to acquire the lock of this value using the specified lock and

- ```rust
  pub unsafe fn try_lock_explicit_unchecked(self: &Self, lock_order: Ordering, unlock_order: Ordering) -> Option<Locked<''_, T>> { /* ... */ }
  ```
  Try to acquire the lock of this value using the specified lock and

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Take the value back out of the lock when this is the sole owner.

##### Trait Implementations

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> TryLock<T> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `Locked`

**Attributes:**

- `#[must_use = "TryLock will immediately unlock if not used"]`

A locked value acquired from a `TryLock`.

The type represents an exclusive view at the underlying value. The lock is
released when this type is dropped.

This type derefs to the underlying value.

```rust
pub struct Locked<''a, T: ''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Receiver**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

