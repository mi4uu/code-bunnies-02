# Crate Documentation

**Version:** 1.0.2

**Format Version:** 43

# Module `sync_wrapper`

A mutual exclusion primitive that relies on static type information only

This library is inspired by [this discussion](https://internals.rust-lang.org/t/what-shall-sync-mean-across-an-await/12020/2).

## Types

### Struct `SyncWrapper`

A mutual exclusion primitive that relies on static type information only

In some cases synchronization can be proven statically: whenever you hold an exclusive `&mut`
reference, the Rust type system ensures that no other part of the program can hold another
reference to the data. Therefore it is safe to access it even if the current thread obtained
this reference via a channel. Whenever this is the case, the overhead of allocating and locking
a [`Mutex`] can be avoided by using this static version.

One example where this is often applicable is [`Future`], which requires an exclusive reference
for its [`poll`] method: While a given `Future` implementation may not be safe to access by
multiple threads concurrently, the executor can only run the `Future` on one thread at any
given time, making it [`Sync`] in practice as long as the implementation is `Send`. You can
therefore use the static mutex to prove that your data structure is `Sync` even though it
contains such a `Future`.

# Example

```
use sync_wrapper::SyncWrapper;
use std::future::Future;

struct MyThing {
    future: SyncWrapper<Box<dyn Future<Output = String> + Send>>,
}

impl MyThing {
    // all accesses to `self.future` now require an exclusive reference or ownership
}

fn assert_sync<T: Sync>() {}

assert_sync::<MyThing>();
```

[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
[`poll`]: https://doc.rust-lang.org/std/future/trait.Future.html#method.poll
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html

```rust
pub struct SyncWrapper<T>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: T) -> Self { /* ... */ }
  ```
  Creates a new static mutex containing the given value.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Acquires a reference to the protected value.

- ```rust
  pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> { /* ... */ }
  ```
  Acquires a pinned reference to the protected value.

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Consumes this mutex, returning the underlying data.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: never) -> T { /* ... */ }
    ```

  - ```rust
    fn from(value: T) -> Self { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

### Struct `SyncFuture`

`Future` which is `Sync`.

# Examples

```
use sync_wrapper::{SyncWrapper, SyncFuture};

let fut = async { 1 };
let fut = SyncFuture::new(fut);
```

```rust
pub struct SyncFuture<F> {
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
  pub fn new(inner: F) -> Self { /* ... */ }
  ```

- ```rust
  pub fn into_inner(self: Self) -> F { /* ... */ }
  ```

##### Trait Implementations

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `SyncStream`

**Attributes:**

- `#[<cfg>(feature = "futures")]`

`Stream` which is `Sync`.

# Examples

```
use sync_wrapper::SyncStream;
use futures::stream;

let st = stream::iter(vec![1]);
let st = SyncStream::new(st);
```

```rust
pub struct SyncStream<S> {
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
  pub fn new(inner: S) -> Self { /* ... */ }
  ```

- ```rust
  pub fn into_inner(self: Self) -> S { /* ... */ }
  ```

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

