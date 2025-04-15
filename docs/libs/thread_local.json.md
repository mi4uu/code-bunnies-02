# Crate Documentation

**Version:** 1.1.8

**Format Version:** 43

# Module `thread_local`

Per-object thread-local storage

This library provides the `ThreadLocal` type which allows a separate copy of
an object to be used for each thread. This allows for per-object
thread-local storage, unlike the standard library's `thread_local!` macro
which only allows static thread-local storage.

Per-thread objects are not destroyed when a thread exits. Instead, objects
are only destroyed when the `ThreadLocal` containing them is destroyed.

You can also iterate over the thread-local values of all thread in a
`ThreadLocal` object using the `iter_mut` and `into_iter` methods. This can
only be done if you have mutable access to the `ThreadLocal` object, which
guarantees that you are the only thread currently accessing it.

Note that since thread IDs are recycled when a thread exits, it is possible
for one thread to retrieve the object of another thread. Since this can only
occur after a thread has exited this does not lead to any race conditions.

# Examples

Basic usage of `ThreadLocal`:

```rust
use thread_local::ThreadLocal;
let tls: ThreadLocal<u32> = ThreadLocal::new();
assert_eq!(tls.get(), None);
assert_eq!(tls.get_or(|| 5), &5);
assert_eq!(tls.get(), Some(&5));
```

Combining thread-local values into a single result:

```rust
use thread_local::ThreadLocal;
use std::sync::Arc;
use std::cell::Cell;
use std::thread;

let tls = Arc::new(ThreadLocal::new());

// Create a bunch of threads to do stuff
for _ in 0..5 {
    let tls2 = tls.clone();
    thread::spawn(move || {
        // Increment a counter to count some event...
        let cell = tls2.get_or(|| Cell::new(0));
        cell.set(cell.get() + 1);
    }).join().unwrap();
}

// Once all threads are done, collect the counter values and return the
// sum of all thread-local counter values.
let tls = Arc::try_unwrap(tls).unwrap();
let total = tls.into_iter().fold(0, |x, y| x + y.get());
assert_eq!(total, 5);
```

## Types

### Struct `ThreadLocal`

Thread-local variable wrapper

See the [module-level documentation](index.html) for more.

```rust
pub struct ThreadLocal<T: Send> {
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
  pub const fn new() -> ThreadLocal<T> { /* ... */ }
  ```
  Creates a new empty `ThreadLocal`.

- ```rust
  pub fn with_capacity(capacity: usize) -> ThreadLocal<T> { /* ... */ }
  ```
  Creates a new `ThreadLocal` with an initial capacity. If less than the capacity threads

- ```rust
  pub fn get(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Returns the element for the current thread, if it exists.

- ```rust
  pub fn get_or<F>(self: &Self, create: F) -> &T
where
    F: FnOnce() -> T { /* ... */ }
  ```
  Returns the element for the current thread, or creates it if it doesn't

- ```rust
  pub fn get_or_try<F, E>(self: &Self, create: F) -> Result<&T, E>
where
    F: FnOnce() -> Result<T, E> { /* ... */ }
  ```
  Returns the element for the current thread, or creates it if it doesn't

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T>
where
    T: Sync { /* ... */ }
  ```
  Returns an iterator over the local values of all threads in unspecified

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, T> { /* ... */ }
  ```
  Returns a mutable iterator over the local values of all threads in

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Removes all thread-specific values from the `ThreadLocal`, effectively

- ```rust
  pub fn get_or_default(self: &Self) -> &T { /* ... */ }
  ```
  Returns the element for the current thread, or creates a default one if

##### Trait Implementations

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> ThreadLocal<T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter<T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IterMut<''a, T> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `Iter`

Iterator over the contents of a `ThreadLocal`.

```rust
pub struct Iter<''a, T: Send + Sync> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Unpin**
- **FusedIterator**
- **Sync**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `IterMut`

Mutable iterator over the contents of a `ThreadLocal`.

```rust
pub struct IterMut<''a, T: Send> {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **FusedIterator**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **ExactSizeIterator**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a mut T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
### Struct `IntoIter`

An iterator that moves out of a `ThreadLocal`.

```rust
pub struct IntoIter<T: Send> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **FusedIterator**
- **ExactSizeIterator**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Re-exports

### Re-export `CachedIntoIter`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use cached::CachedIntoIter;
```

### Re-export `CachedIterMut`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use cached::CachedIterMut;
```

### Re-export `CachedThreadLocal`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use cached::CachedThreadLocal;
```

