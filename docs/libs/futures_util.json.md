# Crate Documentation

**Version:** 0.3.31

**Format Version:** 43

# Module `futures_util`

Combinators and utilities for working with `Future`s, `Stream`s, `Sink`s,
and the `AsyncRead` and `AsyncWrite` traits.

## Modules

## Module `future`

Asynchronous values.

This module contains:

- The [`Future`] trait.
- The [`FutureExt`] and [`TryFutureExt`] trait, which provides adapters for
  chaining and composing futures.
- Top-level future combinators like [`lazy`](lazy()) which creates a future
  from a closure that defines its return value, and [`ready`](ready()),
  which constructs a future with an immediate defined value.

```rust
pub mod future { /* ... */ }
```

### Re-exports

#### Re-export `Future`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::future::Future;
```

#### Re-export `BoxFuture`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use futures_core::future::BoxFuture;
```

#### Re-export `LocalBoxFuture`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use futures_core::future::LocalBoxFuture;
```

#### Re-export `FusedFuture`

```rust
pub use futures_core::future::FusedFuture;
```

#### Re-export `TryFuture`

```rust
pub use futures_core::future::TryFuture;
```

#### Re-export `FutureObj`

```rust
pub use futures_task::FutureObj;
```

#### Re-export `LocalFutureObj`

```rust
pub use futures_task::LocalFutureObj;
```

#### Re-export `UnsafeFutureObj`

```rust
pub use futures_task::UnsafeFutureObj;
```

#### Re-export `Flatten`

```rust
pub use self::future::Flatten;
```

#### Re-export `Fuse`

```rust
pub use self::future::Fuse;
```

#### Re-export `FutureExt`

```rust
pub use self::future::FutureExt;
```

#### Re-export `Inspect`

```rust
pub use self::future::Inspect;
```

#### Re-export `IntoStream`

```rust
pub use self::future::IntoStream;
```

#### Re-export `Map`

```rust
pub use self::future::Map;
```

#### Re-export `MapInto`

```rust
pub use self::future::MapInto;
```

#### Re-export `NeverError`

```rust
pub use self::future::NeverError;
```

#### Re-export `Then`

```rust
pub use self::future::Then;
```

#### Re-export `UnitError`

```rust
pub use self::future::UnitError;
```

#### Re-export `FlattenStream`

**⚠️ Deprecated**: This is now an alias for [Flatten](Flatten)

```rust
pub use self::future::FlattenStream;
```

#### Re-export `CatchUnwind`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use self::future::CatchUnwind;
```

#### Re-export `Remote`

**Attributes:**

- `#[<cfg>(feature = "channel")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "channel")))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::future::Remote;
```

#### Re-export `RemoteHandle`

**Attributes:**

- `#[<cfg>(feature = "channel")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "channel")))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::future::RemoteHandle;
```

#### Re-export `Shared`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use self::future::Shared;
```

#### Re-export `WeakShared`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use self::future::WeakShared;
```

#### Re-export `AndThen`

```rust
pub use self::try_future::AndThen;
```

#### Re-export `ErrInto`

```rust
pub use self::try_future::ErrInto;
```

#### Re-export `InspectErr`

```rust
pub use self::try_future::InspectErr;
```

#### Re-export `InspectOk`

```rust
pub use self::try_future::InspectOk;
```

#### Re-export `IntoFuture`

```rust
pub use self::try_future::IntoFuture;
```

#### Re-export `MapErr`

```rust
pub use self::try_future::MapErr;
```

#### Re-export `MapOk`

```rust
pub use self::try_future::MapOk;
```

#### Re-export `MapOkOrElse`

```rust
pub use self::try_future::MapOkOrElse;
```

#### Re-export `OkInto`

```rust
pub use self::try_future::OkInto;
```

#### Re-export `OrElse`

```rust
pub use self::try_future::OrElse;
```

#### Re-export `TryFlatten`

```rust
pub use self::try_future::TryFlatten;
```

#### Re-export `TryFlattenStream`

```rust
pub use self::try_future::TryFlattenStream;
```

#### Re-export `TryFutureExt`

```rust
pub use self::try_future::TryFutureExt;
```

#### Re-export `UnwrapOrElse`

```rust
pub use self::try_future::UnwrapOrElse;
```

#### Re-export `FlattenSink`

**Attributes:**

- `#[<cfg>(feature = "sink")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sink")))]`

```rust
pub use self::try_future::FlattenSink;
```

#### Re-export `lazy`

```rust
pub use self::lazy::lazy;
```

#### Re-export `Lazy`

```rust
pub use self::lazy::Lazy;
```

#### Re-export `pending`

```rust
pub use self::pending::pending;
```

#### Re-export `Pending`

```rust
pub use self::pending::Pending;
```

#### Re-export `maybe_done`

```rust
pub use self::maybe_done::maybe_done;
```

#### Re-export `MaybeDone`

```rust
pub use self::maybe_done::MaybeDone;
```

#### Re-export `try_maybe_done`

```rust
pub use self::try_maybe_done::try_maybe_done;
```

#### Re-export `TryMaybeDone`

```rust
pub use self::try_maybe_done::TryMaybeDone;
```

#### Re-export `OptionFuture`

```rust
pub use self::option::OptionFuture;
```

#### Re-export `poll_fn`

```rust
pub use self::poll_fn::poll_fn;
```

#### Re-export `PollFn`

```rust
pub use self::poll_fn::PollFn;
```

#### Re-export `poll_immediate`

```rust
pub use self::poll_immediate::poll_immediate;
```

#### Re-export `PollImmediate`

```rust
pub use self::poll_immediate::PollImmediate;
```

#### Re-export `err`

```rust
pub use self::ready::err;
```

#### Re-export `ok`

```rust
pub use self::ready::ok;
```

#### Re-export `ready`

```rust
pub use self::ready::ready;
```

#### Re-export `Ready`

```rust
pub use self::ready::Ready;
```

#### Re-export `always_ready`

```rust
pub use self::always_ready::always_ready;
```

#### Re-export `AlwaysReady`

```rust
pub use self::always_ready::AlwaysReady;
```

#### Re-export `join`

```rust
pub use self::join::join;
```

#### Re-export `join3`

```rust
pub use self::join::join3;
```

#### Re-export `join4`

```rust
pub use self::join::join4;
```

#### Re-export `join5`

```rust
pub use self::join::join5;
```

#### Re-export `Join`

```rust
pub use self::join::Join;
```

#### Re-export `Join3`

```rust
pub use self::join::Join3;
```

#### Re-export `Join4`

```rust
pub use self::join::Join4;
```

#### Re-export `Join5`

```rust
pub use self::join::Join5;
```

#### Re-export `join_all`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::join_all::join_all;
```

#### Re-export `JoinAll`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::join_all::JoinAll;
```

#### Re-export `select`

```rust
pub use self::select::select;
```

#### Re-export `Select`

```rust
pub use self::select::Select;
```

#### Re-export `select_all`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::select_all::select_all;
```

#### Re-export `SelectAll`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::select_all::SelectAll;
```

#### Re-export `try_join`

```rust
pub use self::try_join::try_join;
```

#### Re-export `try_join3`

```rust
pub use self::try_join::try_join3;
```

#### Re-export `try_join4`

```rust
pub use self::try_join::try_join4;
```

#### Re-export `try_join5`

```rust
pub use self::try_join::try_join5;
```

#### Re-export `TryJoin`

```rust
pub use self::try_join::TryJoin;
```

#### Re-export `TryJoin3`

```rust
pub use self::try_join::TryJoin3;
```

#### Re-export `TryJoin4`

```rust
pub use self::try_join::TryJoin4;
```

#### Re-export `TryJoin5`

```rust
pub use self::try_join::TryJoin5;
```

#### Re-export `try_join_all`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_join_all::try_join_all;
```

#### Re-export `TryJoinAll`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_join_all::TryJoinAll;
```

#### Re-export `try_select`

```rust
pub use self::try_select::try_select;
```

#### Re-export `TrySelect`

```rust
pub use self::try_select::TrySelect;
```

#### Re-export `select_ok`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::select_ok::select_ok;
```

#### Re-export `SelectOk`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::select_ok::SelectOk;
```

#### Re-export `Either`

```rust
pub use self::either::Either;
```

#### Re-export `AbortHandle`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::abortable::AbortHandle;
```

#### Re-export `AbortRegistration`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::abortable::AbortRegistration;
```

#### Re-export `Abortable`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::abortable::Abortable;
```

#### Re-export `Aborted`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::abortable::Aborted;
```

#### Re-export `abortable`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use abortable::abortable;
```

## Module `stream`

Asynchronous streams.

This module contains:

- The [`Stream`] trait, for objects that can asynchronously produce a
  sequence of values.
- The [`StreamExt`] and [`TryStreamExt`] trait, which provides adapters for
  chaining and composing streams.
- Top-level stream constructors like [`iter`](iter()) which creates a
  stream from an iterator.

```rust
pub mod stream { /* ... */ }
```

### Modules

## Module `futures_unordered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

An unbounded set of futures.

This module is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

```rust
pub mod futures_unordered { /* ... */ }
```

### Types

#### Struct `FuturesUnordered`

**Attributes:**

- `#[must_use = "streams do nothing unless polled"]`

A set of futures which may complete in any order.

See [`FuturesOrdered`](crate::stream::FuturesOrdered) for a version of this
type that preserves a FIFO order.

This structure is optimized to manage a large number of futures.
Futures managed by [`FuturesUnordered`] will only be polled when they
generate wake-up notifications. This reduces the required amount of work
needed to poll large numbers of futures.

[`FuturesUnordered`] can be filled by [`collect`](Iterator::collect)ing an
iterator of futures into a [`FuturesUnordered`], or by
[`push`](FuturesUnordered::push)ing futures onto an existing
[`FuturesUnordered`]. When new futures are added,
[`poll_next`](Stream::poll_next) must be called in order to begin receiving
wake-ups for new futures.

Note that you can create a ready-made [`FuturesUnordered`] via the
[`collect`](Iterator::collect) method, or you can start with an empty set
with the [`FuturesUnordered::new`] constructor.

This type is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

```rust
pub struct FuturesUnordered<Fut> {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Constructs a new, empty [`FuturesUnordered`].

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of futures contained in the set.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the set contains no futures.

- ```rust
  pub fn push(self: &Self, future: Fut) { /* ... */ }
  ```
  Push a future into the set.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, Fut>
where
    Fut: Unpin { /* ... */ }
  ```
  Returns an iterator that allows inspecting each future in the set.

- ```rust
  pub fn iter_pin_ref(self: Pin<&Self>) -> IterPinRef<''_, Fut> { /* ... */ }
  ```
  Returns an iterator that allows inspecting each future in the set.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, Fut>
where
    Fut: Unpin { /* ... */ }
  ```
  Returns an iterator that allows modifying each future in the set.

- ```rust
  pub fn iter_pin_mut(self: Pin<&mut Self>) -> IterPinMut<''_, Fut> { /* ... */ }
  ```
  Returns an iterator that allows modifying each future in the set.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the set, removing all futures.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Spawn**
  - ```rust
    fn spawn_obj(self: &Self, future_obj: FutureObj<''static, ()>) -> Result<(), SpawnError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> Self
where
    I: IntoIterator<Item = Fut> { /* ... */ }
    ```

- **Send**
- **LocalSpawn**
  - ```rust
    fn spawn_local_obj(self: &Self, future_obj: LocalFutureObj<''static, ()>) -> Result<(), SpawnError> { /* ... */ }
    ```

- **FusedStream**
  - ```rust
    fn is_terminated(self: &Self) -> bool { /* ... */ }
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

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **SpawnExt**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Sync**
- **LocalSpawnExt**
- **Extend**
  - ```rust
    fn extend<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = Fut> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StreamExt**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryStreamExt**
- **Freeze**
### Re-exports

#### Re-export `IntoIter`

**Attributes:**

- `#[allow(unreachable_pub)]`

```rust
pub use self::iter::IntoIter;
```

#### Re-export `Iter`

**Attributes:**

- `#[allow(unreachable_pub)]`

```rust
pub use self::iter::Iter;
```

#### Re-export `IterMut`

**Attributes:**

- `#[allow(unreachable_pub)]`

```rust
pub use self::iter::IterMut;
```

#### Re-export `IterPinMut`

**Attributes:**

- `#[allow(unreachable_pub)]`

```rust
pub use self::iter::IterPinMut;
```

#### Re-export `IterPinRef`

**Attributes:**

- `#[allow(unreachable_pub)]`

```rust
pub use self::iter::IterPinRef;
```

## Module `select_all`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

An unbounded set of streams

```rust
pub mod select_all { /* ... */ }
```

### Types

#### Struct `SelectAll`

**Attributes:**

- `#[must_use = "streams do nothing unless polled"]`

An unbounded set of streams

This "combinator" provides the ability to maintain a set of streams
and drive them all to completion.

Streams are pushed into this set and their realized values are
yielded as they become ready. Streams will only be polled when they
generate notifications. This allows to coordinate a large number of streams.

Note that you can create a ready-made `SelectAll` via the
`select_all` function in the `stream` module, or you can start with an
empty set with the `SelectAll::new` constructor.

```rust
pub struct SelectAll<St> {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Constructs a new, empty `SelectAll`

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of streams contained in the set.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the set contains no streams

- ```rust
  pub fn push(self: &mut Self, stream: St) { /* ... */ }
  ```
  Push a stream into the set.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, St> { /* ... */ }
  ```
  Returns an iterator that allows inspecting each stream in the set.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, St> { /* ... */ }
  ```
  Returns an iterator that allows modifying each stream in the set.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the set, removing all streams.

###### Trait Implementations

- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = St>>(iter: T) -> Self { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryStreamExt**
- **FusedStream**
  - ```rust
    fn is_terminated(self: &Self) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: IntoIterator<Item = St>>(self: &mut Self, iter: T) { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Send**
- **Sync**
- **RefUnwindSafe**
- **StreamExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Iter`

Immutable iterator over all streams in the unordered set.

```rust
pub struct Iter<''a, St: Unpin>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **ExactSizeIterator**
- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `IterMut`

Mutable iterator over all streams in the unordered set.

```rust
pub struct IterMut<''a, St: Unpin>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ExactSizeIterator**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `IntoIter`

Owned iterator over all streams in the unordered set.

```rust
pub struct IntoIter<St: Unpin>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **ExactSizeIterator**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `select_all`

Convert a list of streams into a `Stream` of results from the streams.

This essentially takes a list of streams (e.g. a vector, an iterator, etc.)
and bundles them together into a single stream.
The stream will yield items as they become available on the underlying
streams internally, in the order they become available.

Note that the returned set can also be used to dynamically push more
streams into the set as they become available.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

```rust
pub fn select_all<I>(streams: I) -> SelectAll<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: Stream + Unpin { /* ... */ }
```

### Re-exports

#### Re-export `BoxStream`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use futures_core::stream::BoxStream;
```

#### Re-export `LocalBoxStream`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use futures_core::stream::LocalBoxStream;
```

#### Re-export `FusedStream`

```rust
pub use futures_core::stream::FusedStream;
```

#### Re-export `Stream`

```rust
pub use futures_core::stream::Stream;
```

#### Re-export `TryStream`

```rust
pub use futures_core::stream::TryStream;
```

#### Re-export `All`

```rust
pub use self::stream::All;
```

#### Re-export `Any`

```rust
pub use self::stream::Any;
```

#### Re-export `Chain`

```rust
pub use self::stream::Chain;
```

#### Re-export `Collect`

```rust
pub use self::stream::Collect;
```

#### Re-export `Concat`

```rust
pub use self::stream::Concat;
```

#### Re-export `Count`

```rust
pub use self::stream::Count;
```

#### Re-export `Cycle`

```rust
pub use self::stream::Cycle;
```

#### Re-export `Enumerate`

```rust
pub use self::stream::Enumerate;
```

#### Re-export `Filter`

```rust
pub use self::stream::Filter;
```

#### Re-export `FilterMap`

```rust
pub use self::stream::FilterMap;
```

#### Re-export `FlatMap`

```rust
pub use self::stream::FlatMap;
```

#### Re-export `Flatten`

```rust
pub use self::stream::Flatten;
```

#### Re-export `Fold`

```rust
pub use self::stream::Fold;
```

#### Re-export `ForEach`

```rust
pub use self::stream::ForEach;
```

#### Re-export `Fuse`

```rust
pub use self::stream::Fuse;
```

#### Re-export `Inspect`

```rust
pub use self::stream::Inspect;
```

#### Re-export `Map`

```rust
pub use self::stream::Map;
```

#### Re-export `Next`

```rust
pub use self::stream::Next;
```

#### Re-export `NextIf`

```rust
pub use self::stream::NextIf;
```

#### Re-export `NextIfEq`

```rust
pub use self::stream::NextIfEq;
```

#### Re-export `Peek`

```rust
pub use self::stream::Peek;
```

#### Re-export `PeekMut`

```rust
pub use self::stream::PeekMut;
```

#### Re-export `Peekable`

```rust
pub use self::stream::Peekable;
```

#### Re-export `Scan`

```rust
pub use self::stream::Scan;
```

#### Re-export `SelectNextSome`

```rust
pub use self::stream::SelectNextSome;
```

#### Re-export `Skip`

```rust
pub use self::stream::Skip;
```

#### Re-export `SkipWhile`

```rust
pub use self::stream::SkipWhile;
```

#### Re-export `StreamExt`

```rust
pub use self::stream::StreamExt;
```

#### Re-export `StreamFuture`

```rust
pub use self::stream::StreamFuture;
```

#### Re-export `Take`

```rust
pub use self::stream::Take;
```

#### Re-export `TakeUntil`

```rust
pub use self::stream::TakeUntil;
```

#### Re-export `TakeWhile`

```rust
pub use self::stream::TakeWhile;
```

#### Re-export `Then`

```rust
pub use self::stream::Then;
```

#### Re-export `Unzip`

```rust
pub use self::stream::Unzip;
```

#### Re-export `Zip`

```rust
pub use self::stream::Zip;
```

#### Re-export `CatchUnwind`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use self::stream::CatchUnwind;
```

#### Re-export `Chunks`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::Chunks;
```

#### Re-export `ReadyChunks`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::ReadyChunks;
```

#### Re-export `Forward`

**Attributes:**

- `#[<cfg>(feature = "sink")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sink")))]`

```rust
pub use self::stream::Forward;
```

#### Re-export `BufferUnordered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::BufferUnordered;
```

#### Re-export `Buffered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::Buffered;
```

#### Re-export `FlatMapUnordered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::FlatMapUnordered;
```

#### Re-export `FlattenUnordered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::FlattenUnordered;
```

#### Re-export `ForEachConcurrent`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::ForEachConcurrent;
```

#### Re-export `ReuniteError`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "sink")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sink")))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::ReuniteError;
```

#### Re-export `SplitSink`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "sink")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sink")))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::SplitSink;
```

#### Re-export `SplitStream`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "sink")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sink")))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::stream::SplitStream;
```

#### Re-export `try_unfold`

```rust
pub use self::try_stream::try_unfold;
```

#### Re-export `AndThen`

```rust
pub use self::try_stream::AndThen;
```

#### Re-export `ErrInto`

```rust
pub use self::try_stream::ErrInto;
```

#### Re-export `InspectErr`

```rust
pub use self::try_stream::InspectErr;
```

#### Re-export `InspectOk`

```rust
pub use self::try_stream::InspectOk;
```

#### Re-export `IntoStream`

```rust
pub use self::try_stream::IntoStream;
```

#### Re-export `MapErr`

```rust
pub use self::try_stream::MapErr;
```

#### Re-export `MapOk`

```rust
pub use self::try_stream::MapOk;
```

#### Re-export `OrElse`

```rust
pub use self::try_stream::OrElse;
```

#### Re-export `TryAll`

```rust
pub use self::try_stream::TryAll;
```

#### Re-export `TryAny`

```rust
pub use self::try_stream::TryAny;
```

#### Re-export `TryCollect`

```rust
pub use self::try_stream::TryCollect;
```

#### Re-export `TryConcat`

```rust
pub use self::try_stream::TryConcat;
```

#### Re-export `TryFilter`

```rust
pub use self::try_stream::TryFilter;
```

#### Re-export `TryFilterMap`

```rust
pub use self::try_stream::TryFilterMap;
```

#### Re-export `TryFlatten`

```rust
pub use self::try_stream::TryFlatten;
```

#### Re-export `TryFold`

```rust
pub use self::try_stream::TryFold;
```

#### Re-export `TryForEach`

```rust
pub use self::try_stream::TryForEach;
```

#### Re-export `TryNext`

```rust
pub use self::try_stream::TryNext;
```

#### Re-export `TrySkipWhile`

```rust
pub use self::try_stream::TrySkipWhile;
```

#### Re-export `TryStreamExt`

```rust
pub use self::try_stream::TryStreamExt;
```

#### Re-export `TryTakeWhile`

```rust
pub use self::try_stream::TryTakeWhile;
```

#### Re-export `TryUnfold`

```rust
pub use self::try_stream::TryUnfold;
```

#### Re-export `IntoAsyncRead`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io")))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::try_stream::IntoAsyncRead;
```

#### Re-export `TryBufferUnordered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_stream::TryBufferUnordered;
```

#### Re-export `TryBuffered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_stream::TryBuffered;
```

#### Re-export `TryFlattenUnordered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_stream::TryFlattenUnordered;
```

#### Re-export `TryForEachConcurrent`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_stream::TryForEachConcurrent;
```

#### Re-export `TryChunks`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_stream::TryChunks;
```

#### Re-export `TryChunksError`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_stream::TryChunksError;
```

#### Re-export `TryReadyChunks`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_stream::TryReadyChunks;
```

#### Re-export `TryReadyChunksError`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::try_stream::TryReadyChunksError;
```

#### Re-export `iter`

```rust
pub use self::iter::iter;
```

#### Re-export `Iter`

```rust
pub use self::iter::Iter;
```

#### Re-export `repeat`

```rust
pub use self::repeat::repeat;
```

#### Re-export `Repeat`

```rust
pub use self::repeat::Repeat;
```

#### Re-export `repeat_with`

```rust
pub use self::repeat_with::repeat_with;
```

#### Re-export `RepeatWith`

```rust
pub use self::repeat_with::RepeatWith;
```

#### Re-export `empty`

```rust
pub use self::empty::empty;
```

#### Re-export `Empty`

```rust
pub use self::empty::Empty;
```

#### Re-export `once`

```rust
pub use self::once::once;
```

#### Re-export `Once`

```rust
pub use self::once::Once;
```

#### Re-export `pending`

```rust
pub use self::pending::pending;
```

#### Re-export `Pending`

```rust
pub use self::pending::Pending;
```

#### Re-export `poll_fn`

```rust
pub use self::poll_fn::poll_fn;
```

#### Re-export `PollFn`

```rust
pub use self::poll_fn::PollFn;
```

#### Re-export `poll_immediate`

```rust
pub use self::poll_immediate::poll_immediate;
```

#### Re-export `PollImmediate`

```rust
pub use self::poll_immediate::PollImmediate;
```

#### Re-export `select`

```rust
pub use self::select::select;
```

#### Re-export `Select`

```rust
pub use self::select::Select;
```

#### Re-export `select_with_strategy`

```rust
pub use self::select_with_strategy::select_with_strategy;
```

#### Re-export `PollNext`

```rust
pub use self::select_with_strategy::PollNext;
```

#### Re-export `SelectWithStrategy`

```rust
pub use self::select_with_strategy::SelectWithStrategy;
```

#### Re-export `unfold`

```rust
pub use self::unfold::unfold;
```

#### Re-export `Unfold`

```rust
pub use self::unfold::Unfold;
```

#### Re-export `FuturesOrdered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::futures_ordered::FuturesOrdered;
```

#### Re-export `FuturesUnordered`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`
- `#[doc(inline)]`

```rust
pub use self::futures_unordered::FuturesUnordered;
```

#### Re-export `select_all`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`
- `#[doc(inline)]`

```rust
pub use self::select_all::select_all;
```

#### Re-export `SelectAll`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`
- `#[doc(inline)]`

```rust
pub use self::select_all::SelectAll;
```

#### Re-export `AbortHandle`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::abortable::AbortHandle;
```

#### Re-export `AbortRegistration`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::abortable::AbortRegistration;
```

#### Re-export `Abortable`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::abortable::Abortable;
```

#### Re-export `Aborted`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::abortable::Aborted;
```

#### Re-export `abortable`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use abortable::abortable;
```

## Module `sink`

**Attributes:**

- `#[<cfg>(feature = "sink")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sink")))]`

Asynchronous sinks.

This module contains:

- The [`Sink`] trait, which allows you to asynchronously write data.
- The [`SinkExt`] trait, which provides adapters for chaining and composing
  sinks.

```rust
pub mod sink { /* ... */ }
```

### Traits

#### Trait `SinkExt`

An extension trait for `Sink`s that provides a variety of convenient
combinator functions.

```rust
pub trait SinkExt<Item>: Sink<Item> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn with<U, Fut, F, E>(self: Self, f: F) -> With<Self, Item, U, Fut, F>
where
    F: FnMut(U) -> Fut,
    Fut: Future<Output = Result<Item, E>>,
    E: From<<Self as >::Error>,
    Self: Sized { /* ... */ }
  ```
  Composes a function *in front of* the sink.

- ```rust
  fn with_flat_map<U, St, F>(self: Self, f: F) -> WithFlatMap<Self, Item, U, St, F>
where
    F: FnMut(U) -> St,
    St: Stream<Item = Result<Item, <Self as >::Error>>,
    Self: Sized { /* ... */ }
  ```
  Composes a function *in front of* the sink.

- ```rust
  fn sink_map_err<E, F>(self: Self, f: F) -> SinkMapErr<Self, F>
where
    F: FnOnce(<Self as >::Error) -> E,
    Self: Sized { /* ... */ }
  ```
  Transforms the error returned by the sink.

- ```rust
  fn sink_err_into<E>(self: Self) -> err_into::SinkErrInto<Self, Item, E>
where
    Self: Sized,
    <Self as >::Error: Into<E> { /* ... */ }
  ```
  Map this sink's error to a different error type using the `Into` trait.

- ```rust
  fn buffer(self: Self, capacity: usize) -> Buffer<Self, Item>
where
    Self: Sized { /* ... */ }
  ```
  Adds a fixed-size buffer to the current sink.

- ```rust
  fn close(self: &mut Self) -> Close<''_, Self, Item>
where
    Self: Unpin { /* ... */ }
  ```
  Close the sink.

- ```rust
  fn fanout<Si>(self: Self, other: Si) -> Fanout<Self, Si>
where
    Self: Sized,
    Item: Clone,
    Si: Sink<Item, Error = <Self as >::Error> { /* ... */ }
  ```
  Fanout items to multiple sinks.

- ```rust
  fn flush(self: &mut Self) -> Flush<''_, Self, Item>
where
    Self: Unpin { /* ... */ }
  ```
  Flush the sink, processing all pending items.

- ```rust
  fn send(self: &mut Self, item: Item) -> Send<''_, Self, Item>
where
    Self: Unpin { /* ... */ }
  ```
  A future that completes after the given item has been fully processed

- ```rust
  fn feed(self: &mut Self, item: Item) -> Feed<''_, Self, Item>
where
    Self: Unpin { /* ... */ }
  ```
  A future that completes after the given item has been received

- ```rust
  fn send_all<''a, St>(self: &''a mut Self, stream: &''a mut St) -> SendAll<''a, Self, St>
where
    St: TryStream<Ok = Item, Error = <Self as >::Error> + Stream + Unpin + ?Sized,
    Self: Unpin { /* ... */ }
  ```
  A future that completes after the given stream has been fully processed

- ```rust
  fn left_sink<Si2>(self: Self) -> Either<Self, Si2>
where
    Si2: Sink<Item, Error = <Self as >::Error>,
    Self: Sized { /* ... */ }
  ```
  Wrap this sink in an `Either` sink, making it the left-hand variant

- ```rust
  fn right_sink<Si1>(self: Self) -> Either<Si1, Self>
where
    Si1: Sink<Item, Error = <Self as >::Error>,
    Self: Sized { /* ... */ }
  ```
  Wrap this stream in an `Either` stream, making it the right-hand variant

- ```rust
  fn poll_ready_unpin(self: &mut Self, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>>
where
    Self: Unpin { /* ... */ }
  ```
  A convenience method for calling [`Sink::poll_ready`] on [`Unpin`]

- ```rust
  fn start_send_unpin(self: &mut Self, item: Item) -> Result<(), <Self as >::Error>
where
    Self: Unpin { /* ... */ }
  ```
  A convenience method for calling [`Sink::start_send`] on [`Unpin`]

- ```rust
  fn poll_flush_unpin(self: &mut Self, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>>
where
    Self: Unpin { /* ... */ }
  ```
  A convenience method for calling [`Sink::poll_flush`] on [`Unpin`]

- ```rust
  fn poll_close_unpin(self: &mut Self, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>>
where
    Self: Unpin { /* ... */ }
  ```
  A convenience method for calling [`Sink::poll_close`] on [`Unpin`]

##### Implementations

This trait is implemented for the following types:

- `T` with <T, Item>

### Re-exports

#### Re-export `Sink`

```rust
pub use futures_sink::Sink;
```

#### Re-export `Close`

```rust
pub use self::close::Close;
```

#### Re-export `drain`

```rust
pub use self::drain::drain;
```

#### Re-export `Drain`

```rust
pub use self::drain::Drain;
```

#### Re-export `Fanout`

```rust
pub use self::fanout::Fanout;
```

#### Re-export `Feed`

```rust
pub use self::feed::Feed;
```

#### Re-export `Flush`

```rust
pub use self::flush::Flush;
```

#### Re-export `SinkErrInto`

```rust
pub use self::err_into::SinkErrInto;
```

#### Re-export `SinkMapErr`

```rust
pub use self::map_err::SinkMapErr;
```

#### Re-export `Send`

```rust
pub use self::send::Send;
```

#### Re-export `SendAll`

```rust
pub use self::send_all::SendAll;
```

#### Re-export `unfold`

```rust
pub use self::unfold::unfold;
```

#### Re-export `Unfold`

```rust
pub use self::unfold::Unfold;
```

#### Re-export `With`

```rust
pub use self::with::With;
```

#### Re-export `WithFlatMap`

```rust
pub use self::with_flat_map::WithFlatMap;
```

#### Re-export `Buffer`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::buffer::Buffer;
```

## Module `task`

Tools for working with tasks.

This module contains:

- [`Spawn`], a trait for spawning new tasks.
- [`Context`], a context of an asynchronous task,
  including a handle for waking up the task.
- [`Waker`], a handle for waking up a task.

The remaining types and traits in the module are used for implementing
executors or dealing with synchronization issues around task wakeup.

```rust
pub mod task { /* ... */ }
```

### Re-exports

#### Re-export `Context`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::Context;
```

#### Re-export `Poll`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::Poll;
```

#### Re-export `RawWaker`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::RawWaker;
```

#### Re-export `RawWakerVTable`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::RawWakerVTable;
```

#### Re-export `Waker`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::task::Waker;
```

#### Re-export `FutureObj`

```rust
pub use futures_task::FutureObj;
```

#### Re-export `LocalFutureObj`

```rust
pub use futures_task::LocalFutureObj;
```

#### Re-export `LocalSpawn`

```rust
pub use futures_task::LocalSpawn;
```

#### Re-export `Spawn`

```rust
pub use futures_task::Spawn;
```

#### Re-export `SpawnError`

```rust
pub use futures_task::SpawnError;
```

#### Re-export `UnsafeFutureObj`

```rust
pub use futures_task::UnsafeFutureObj;
```

#### Re-export `noop_waker`

```rust
pub use futures_task::noop_waker;
```

#### Re-export `noop_waker_ref`

```rust
pub use futures_task::noop_waker_ref;
```

#### Re-export `ArcWake`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use futures_task::ArcWake;
```

#### Re-export `waker`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use futures_task::waker;
```

#### Re-export `waker_ref`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use futures_task::waker_ref;
```

#### Re-export `WakerRef`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use futures_task::WakerRef;
```

#### Re-export `AtomicWaker`

**Attributes:**

- `#[<cfg_attr>(target_os = "none",
cfg(any(target_has_atomic = "ptr", feature = "portable-atomic")))]`

```rust
pub use futures_core::task::__internal::AtomicWaker;
```

#### Re-export `LocalSpawnExt`

```rust
pub use self::spawn::LocalSpawnExt;
```

#### Re-export `SpawnExt`

```rust
pub use self::spawn::SpawnExt;
```

## Module `never`

This module contains the `Never` type.

Values of this type can never be created and will never exist.

```rust
pub mod never { /* ... */ }
```

### Types

#### Type Alias `Never`

A type with no possible values.

This is used to indicate values which can never be created, such as the
error type of infallible futures.

This type is a stable equivalent to the `!` type from `std`.

This is currently an alias for [`std::convert::Infallible`], but in
the future it may be an alias for [`!`][never].
See ["Future compatibility" section of `std::convert::Infallible`][infallible] for more.

[never]: https://doc.rust-lang.org/nightly/std/primitive.never.html
[infallible]: https://doc.rust-lang.org/nightly/std/convert/enum.Infallible.html#future-compatibility

```rust
pub type Never = core::convert::Infallible;
```

## Module `io`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io")))]`
- `#[<cfg>(feature = "std")]`

Asynchronous I/O.

This module is the asynchronous version of `std::io`. It defines four
traits, [`AsyncRead`], [`AsyncWrite`], [`AsyncSeek`], and [`AsyncBufRead`],
which mirror the `Read`, `Write`, `Seek`, and `BufRead` traits of the
standard library. However, these traits integrate with the asynchronous
task system, so that if an I/O object isn't ready for reading (or writing),
the thread is not blocked, and instead the current task is queued to be
woken when I/O is ready.

In addition, the [`AsyncReadExt`], [`AsyncWriteExt`], [`AsyncSeekExt`], and
[`AsyncBufReadExt`] extension traits offer a variety of useful combinators
for operating with asynchronous I/O objects, including ways to work with
them using futures, streams and sinks.

This module is only available when the `std` feature of this
library is activated, and it is activated by default.

```rust
pub mod io { /* ... */ }
```

### Traits

#### Trait `AsyncReadExt`

An extension trait which adds utility methods to `AsyncRead` types.

```rust
pub trait AsyncReadExt: AsyncRead {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn chain<R>(self: Self, next: R) -> Chain<Self, R>
where
    Self: Sized,
    R: AsyncRead { /* ... */ }
  ```
  Creates an adaptor which will chain this stream with another.

- ```rust
  fn read<''a>(self: &''a mut Self, buf: &''a mut [u8]) -> Read<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Tries to read some bytes directly into the given `buf` in asynchronous

- ```rust
  fn read_vectored<''a>(self: &''a mut Self, bufs: &''a mut [IoSliceMut<''a>]) -> ReadVectored<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will read from the `AsyncRead` into `bufs` using vectored

- ```rust
  fn read_exact<''a>(self: &''a mut Self, buf: &''a mut [u8]) -> ReadExact<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will read exactly enough bytes to fill `buf`,

- ```rust
  fn read_to_end<''a>(self: &''a mut Self, buf: &''a mut Vec<u8>) -> ReadToEnd<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will read all the bytes from this `AsyncRead`.

- ```rust
  fn read_to_string<''a>(self: &''a mut Self, buf: &''a mut String) -> ReadToString<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will read all the bytes from this `AsyncRead`.

- ```rust
  fn split(self: Self) -> (ReadHalf<Self>, WriteHalf<Self>)
where
    Self: AsyncWrite + Sized { /* ... */ }
  ```
  Helper method for splitting this read/write object into two halves.

- ```rust
  fn take(self: Self, limit: u64) -> Take<Self>
where
    Self: Sized { /* ... */ }
  ```
  Creates an AsyncRead adapter which will read at most `limit` bytes

##### Implementations

This trait is implemented for the following types:

- `R` with <R: AsyncRead + ?Sized>

#### Trait `AsyncWriteExt`

An extension trait which adds utility methods to `AsyncWrite` types.

```rust
pub trait AsyncWriteExt: AsyncWrite {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn flush(self: &mut Self) -> Flush<''_, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will entirely flush this `AsyncWrite`.

- ```rust
  fn close(self: &mut Self) -> Close<''_, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will entirely close this `AsyncWrite`.

- ```rust
  fn write<''a>(self: &''a mut Self, buf: &''a [u8]) -> Write<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will write bytes from `buf` into the object.

- ```rust
  fn write_vectored<''a>(self: &''a mut Self, bufs: &''a [IoSlice<''a>]) -> WriteVectored<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will write bytes from `bufs` into the object using vectored

- ```rust
  fn write_all<''a>(self: &''a mut Self, buf: &''a [u8]) -> WriteAll<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Write data into this object.

- ```rust
  fn into_sink<Item: AsRef<[u8]>>(self: Self) -> IntoSink<Self, Item>
where
    Self: Sized { /* ... */ }
  ```
  Allow using an [`AsyncWrite`] as a [`Sink`](futures_sink::Sink)`<Item: AsRef<[u8]>>`.

##### Implementations

This trait is implemented for the following types:

- `W` with <W: AsyncWrite + ?Sized>

#### Trait `AsyncSeekExt`

An extension trait which adds utility methods to `AsyncSeek` types.

```rust
pub trait AsyncSeekExt: AsyncSeek {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn seek(self: &mut Self, pos: SeekFrom) -> Seek<''_, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will seek an IO object, and then yield the

- ```rust
  fn stream_position(self: &mut Self) -> Seek<''_, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will return the current seek position from the

##### Implementations

This trait is implemented for the following types:

- `S` with <S: AsyncSeek + ?Sized>

#### Trait `AsyncBufReadExt`

An extension trait which adds utility methods to `AsyncBufRead` types.

```rust
pub trait AsyncBufReadExt: AsyncBufRead {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn fill_buf(self: &mut Self) -> FillBuf<''_, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will wait for a non-empty buffer to be available from this I/O

- ```rust
  fn consume_unpin(self: &mut Self, amt: usize)
where
    Self: Unpin { /* ... */ }
  ```
  A convenience for calling [`AsyncBufRead::consume`] on [`Unpin`] IO types.

- ```rust
  fn read_until<''a>(self: &''a mut Self, byte: u8, buf: &''a mut Vec<u8>) -> ReadUntil<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will read all the bytes associated with this I/O

- ```rust
  fn read_line<''a>(self: &''a mut Self, buf: &''a mut String) -> ReadLine<''a, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Creates a future which will read all the bytes associated with this I/O

- ```rust
  fn lines(self: Self) -> Lines<Self>
where
    Self: Sized { /* ... */ }
  ```
  Returns a stream over the lines of this reader.

##### Implementations

This trait is implemented for the following types:

- `R` with <R: AsyncBufRead + ?Sized>

### Re-exports

#### Re-export `Error`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::Error;
```

#### Re-export `ErrorKind`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::ErrorKind;
```

#### Re-export `IoSlice`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::IoSlice;
```

#### Re-export `IoSliceMut`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::IoSliceMut;
```

#### Re-export `Result`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::Result;
```

#### Re-export `SeekFrom`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::SeekFrom;
```

#### Re-export `AsyncBufRead`

```rust
pub use futures_io::AsyncBufRead;
```

#### Re-export `AsyncRead`

```rust
pub use futures_io::AsyncRead;
```

#### Re-export `AsyncSeek`

```rust
pub use futures_io::AsyncSeek;
```

#### Re-export `AsyncWrite`

```rust
pub use futures_io::AsyncWrite;
```

#### Re-export `AllowStdIo`

```rust
pub use self::allow_std::AllowStdIo;
```

#### Re-export `BufReader`

```rust
pub use self::buf_reader::BufReader;
```

#### Re-export `SeeKRelative`

```rust
pub use self::buf_reader::SeeKRelative;
```

#### Re-export `BufWriter`

```rust
pub use self::buf_writer::BufWriter;
```

#### Re-export `LineWriter`

```rust
pub use self::line_writer::LineWriter;
```

#### Re-export `Chain`

```rust
pub use self::chain::Chain;
```

#### Re-export `Close`

```rust
pub use self::close::Close;
```

#### Re-export `copy`

```rust
pub use self::copy::copy;
```

#### Re-export `Copy`

```rust
pub use self::copy::Copy;
```

#### Re-export `copy_buf`

```rust
pub use self::copy_buf::copy_buf;
```

#### Re-export `CopyBuf`

```rust
pub use self::copy_buf::CopyBuf;
```

#### Re-export `copy_buf_abortable`

```rust
pub use self::copy_buf_abortable::copy_buf_abortable;
```

#### Re-export `CopyBufAbortable`

```rust
pub use self::copy_buf_abortable::CopyBufAbortable;
```

#### Re-export `Cursor`

```rust
pub use self::cursor::Cursor;
```

#### Re-export `empty`

```rust
pub use self::empty::empty;
```

#### Re-export `Empty`

```rust
pub use self::empty::Empty;
```

#### Re-export `FillBuf`

```rust
pub use self::fill_buf::FillBuf;
```

#### Re-export `Flush`

```rust
pub use self::flush::Flush;
```

#### Re-export `IntoSink`

**Attributes:**

- `#[<cfg>(feature = "sink")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sink")))]`

```rust
pub use self::into_sink::IntoSink;
```

#### Re-export `Lines`

```rust
pub use self::lines::Lines;
```

#### Re-export `Read`

```rust
pub use self::read::Read;
```

#### Re-export `ReadVectored`

```rust
pub use self::read_vectored::ReadVectored;
```

#### Re-export `ReadExact`

```rust
pub use self::read_exact::ReadExact;
```

#### Re-export `ReadLine`

```rust
pub use self::read_line::ReadLine;
```

#### Re-export `ReadToEnd`

```rust
pub use self::read_to_end::ReadToEnd;
```

#### Re-export `ReadToString`

```rust
pub use self::read_to_string::ReadToString;
```

#### Re-export `ReadUntil`

```rust
pub use self::read_until::ReadUntil;
```

#### Re-export `repeat`

```rust
pub use self::repeat::repeat;
```

#### Re-export `Repeat`

```rust
pub use self::repeat::Repeat;
```

#### Re-export `Seek`

```rust
pub use self::seek::Seek;
```

#### Re-export `sink`

```rust
pub use self::sink::sink;
```

#### Re-export `Sink`

```rust
pub use self::sink::Sink;
```

#### Re-export `ReadHalf`

```rust
pub use self::split::ReadHalf;
```

#### Re-export `ReuniteError`

```rust
pub use self::split::ReuniteError;
```

#### Re-export `WriteHalf`

```rust
pub use self::split::WriteHalf;
```

#### Re-export `Take`

```rust
pub use self::take::Take;
```

#### Re-export `Window`

```rust
pub use self::window::Window;
```

#### Re-export `Write`

```rust
pub use self::write::Write;
```

#### Re-export `WriteVectored`

```rust
pub use self::write_vectored::WriteVectored;
```

#### Re-export `WriteAll`

```rust
pub use self::write_all::WriteAll;
```

## Module `lock`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Futures-powered synchronization primitives.

This module is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

```rust
pub mod lock { /* ... */ }
```

### Re-exports

#### Re-export `MappedMutexGuard`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::mutex::MappedMutexGuard;
```

#### Re-export `Mutex`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::mutex::Mutex;
```

#### Re-export `MutexGuard`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::mutex::MutexGuard;
```

#### Re-export `MutexLockFuture`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::mutex::MutexLockFuture;
```

#### Re-export `OwnedMutexGuard`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::mutex::OwnedMutexGuard;
```

#### Re-export `OwnedMutexLockFuture`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use self::mutex::OwnedMutexLockFuture;
```

## Macros

### Macro `poll`

**Attributes:**

- `#[macro_export]`

A macro which returns the result of polling a future once within the
current `async` context.

This macro is only usable inside of `async` functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

If you need the result of polling a [`Stream`](crate::stream::Stream),
you can use this macro with the [`next`](crate::stream::StreamExt::next) method:
`poll!(stream.next())`.

```rust
pub macro_rules! poll {
    /* macro_rules! poll {
    ($x:expr $(,)?) => { ... };
} */
}
```

### Macro `pending`

**Attributes:**

- `#[macro_export]`

A macro which yields to the event loop once.

This is equivalent to returning [`Poll::Pending`](futures_core::task::Poll)
from a [`Future::poll`](futures_core::future::Future::poll) implementation.
Similarly, when using this macro, it must be ensured that [`wake`](std::task::Waker::wake)
is called somewhere when further progress can be made.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

```rust
pub macro_rules! pending {
    /* macro_rules! pending {
    () => { ... };
} */
}
```

### Macro `stream_select`

**Attributes:**

- `#[allow(clippy::too_long_first_doc_paragraph)]`
- `#[macro_export]`

Combines several streams, all producing the same `Item` type, into one stream.
This is similar to `select_all` but does not require the streams to all be the same type.
It also keeps the streams inline, and does not require `Box<dyn Stream>`s to be allocated.
Streams passed to this macro must be `Unpin`.

If multiple streams are ready, one will be pseudo randomly selected at runtime.

# Examples

```
# futures::executor::block_on(async {
use futures::{stream, StreamExt, stream_select};
let endless_ints = |i| stream::iter(vec![i].into_iter().cycle()).fuse();

let mut endless_numbers = stream_select!(endless_ints(1i32), endless_ints(2), endless_ints(3));
match endless_numbers.next().await {
    Some(1) => println!("Got a 1"),
    Some(2) => println!("Got a 2"),
    Some(3) => println!("Got a 3"),
    _ => unreachable!(),
}
# });
```

```rust
pub macro_rules! stream_select {
    /* macro_rules! stream_select {
    ($($tokens:tt)*) => { ... };
} */
}
```

### Macro `join`

**Attributes:**

- `#[macro_export]`

Polls multiple futures simultaneously, returning a tuple
of all results once complete.

While `join!(a, b)` is similar to `(a.await, b.await)`,
`join!` polls both futures concurrently and therefore is more efficient.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

```
# futures::executor::block_on(async {
use futures::join;

let a = async { 1 };
let b = async { 2 };
assert_eq!(join!(a, b), (1, 2));

// `join!` is variadic, so you can pass any number of futures
let c = async { 3 };
let d = async { 4 };
let e = async { 5 };
assert_eq!(join!(c, d, e), (3, 4, 5));
# });
```

```rust
pub macro_rules! join {
    /* macro_rules! join {
    ($($tokens:tt)*) => { ... };
} */
}
```

### Macro `try_join`

**Attributes:**

- `#[macro_export]`

Polls multiple futures simultaneously, resolving to a [`Result`] containing
either a tuple of the successful outputs or an error.

`try_join!` is similar to [`join!`], but completes immediately if any of
the futures return an error.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

When used on multiple futures that return `Ok`, `try_join!` will return
`Ok` of a tuple of the values:

```
# futures::executor::block_on(async {
use futures::try_join;

let a = async { Ok::<i32, i32>(1) };
let b = async { Ok::<i32, i32>(2) };
assert_eq!(try_join!(a, b), Ok((1, 2)));

// `try_join!` is variadic, so you can pass any number of futures
let c = async { Ok::<i32, i32>(3) };
let d = async { Ok::<i32, i32>(4) };
let e = async { Ok::<i32, i32>(5) };
assert_eq!(try_join!(c, d, e), Ok((3, 4, 5)));
# });
```

If one of the futures resolves to an error, `try_join!` will return
that error:

```
# futures::executor::block_on(async {
use futures::try_join;

let a = async { Ok::<i32, i32>(1) };
let b = async { Err::<u64, i32>(2) };

assert_eq!(try_join!(a, b), Err(2));
# });
```

```rust
pub macro_rules! try_join {
    /* macro_rules! try_join {
    ($($tokens:tt)*) => { ... };
} */
}
```

### Macro `select_biased`

**Attributes:**

- `#[allow(clippy :: too_long_first_doc_paragraph)]`
- `#[macro_export]`

Polls multiple futures and streams simultaneously, executing the branch
for the future that finishes first. Unlike [`select!`], if multiple futures are ready,
one will be selected in order of declaration. Futures directly
passed to `select_biased!` must be `Unpin` and implement `FusedFuture`.

If an expression which yields a `Future` is passed to `select_biased!`
(e.g. an `async fn` call) instead of a `Future` by name the `Unpin`
requirement is relaxed, since the macro will pin the resulting `Future`
on the stack. However the `Future` returned by the expression must
still implement `FusedFuture`.

Futures and streams which are not already fused can be fused using the
`.fuse()` method. Note, though, that fusing a future or stream directly
in the call to `select_biased!` will not be enough to prevent it from being
polled after completion if the `select_biased!` call is in a loop, so when
`select_biased!`ing in a loop, users should take care to `fuse()` outside of
the loop.

`select_biased!` can be used as an expression and will return the return
value of the selected branch. For this reason the return type of every
branch in a `select_biased!` must be the same.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

```
# futures::executor::block_on(async {
use futures::future;
use futures::select_biased;
let mut a = future::ready(4);
let mut b = future::pending::<()>();

let res = select_biased! {
    a_res = a => a_res + 1,
    _ = b => 0,
};
assert_eq!(res, 5);
# });
```

```
# futures::executor::block_on(async {
use futures::future;
use futures::stream::{self, StreamExt};
use futures::select_biased;
let mut st = stream::iter(vec![2]).fuse();
let mut fut = future::pending::<()>();

select_biased! {
    x = st.next() => assert_eq!(Some(2), x),
    _ = fut => panic!(),
};
# });
```

As described earlier, `select_biased` can directly select on expressions
which return `Future`s - even if those do not implement `Unpin`:

```
# futures::executor::block_on(async {
use futures::future::FutureExt;
use futures::select_biased;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn async_identity_fn(arg: usize) -> usize {
    arg
}

let res = select_biased! {
    a_res = async_identity_fn(62).fuse() => a_res + 1,
    b_res = async_identity_fn(13).fuse() => b_res,
};
assert!(res == 63 || res == 12);
# });
```

If a similar async function is called outside of `select_biased` to produce
a `Future`, the `Future` must be pinned in order to be able to pass
it to `select_biased`. This can be achieved via `Box::pin` for pinning a
`Future` on the heap or the `pin_mut!` macro for pinning a `Future`
on the stack.

```
# futures::executor::block_on(async {
use futures::future::FutureExt;
use futures::select_biased;
use futures::pin_mut;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn async_identity_fn(arg: usize) -> usize {
    arg
}

let fut_1 = async_identity_fn(1).fuse();
let fut_2 = async_identity_fn(2).fuse();
let mut fut_1 = Box::pin(fut_1); // Pins the Future on the heap
pin_mut!(fut_2); // Pins the Future on the stack

let res = select_biased! {
    a_res = fut_1 => a_res,
    b_res = fut_2 => b_res,
};
assert!(res == 1 || res == 2);
# });
```

`select_biased` also accepts a `complete` branch and a `default` branch.
`complete` will run if all futures and streams have already been
exhausted. `default` will run if no futures or streams are
immediately ready. `complete` takes priority over `default` in
the case where all futures have completed.
A motivating use-case for passing `Future`s by name as well as for
`complete` blocks is to call `select_biased!` in a loop, which is
demonstrated in the following example:

```
# futures::executor::block_on(async {
use futures::future;
use futures::select_biased;
let mut a_fut = future::ready(4);
let mut b_fut = future::ready(6);
let mut total = 0;

loop {
    select_biased! {
        a = a_fut => total += a,
        b = b_fut => total += b,
        complete => break,
        default => panic!(), // never runs (futures run first, then complete)
    };
}
assert_eq!(total, 10);
# });
```

Note that the futures that have been matched over can still be mutated
from inside the `select_biased!` block's branches. This can be used to implement
more complex behavior such as timer resets or writing into the head of
a stream.

[`select!`]: macro.select.html

```rust
pub macro_rules! select_biased {
    /* macro_rules! select_biased {
    ($($tokens:tt)*) => { ... };
} */
}
```

### Macro `select`

**Attributes:**

- `#[allow(clippy :: too_long_first_doc_paragraph)]`
- `#[<cfg>(feature = "std")]`
- `#[macro_export]`

Polls multiple futures and streams simultaneously, executing the branch
for the future that finishes first. If multiple futures are ready,
one will be pseudo-randomly selected at runtime. Futures directly
passed to `select!` must be `Unpin` and implement `FusedFuture`.

If an expression which yields a `Future` is passed to `select!`
(e.g. an `async fn` call) instead of a `Future` by name the `Unpin`
requirement is relaxed, since the macro will pin the resulting `Future`
on the stack. However the `Future` returned by the expression must
still implement `FusedFuture`.

Futures and streams which are not already fused can be fused using the
`.fuse()` method. Note, though, that fusing a future or stream directly
in the call to `select!` will not be enough to prevent it from being
polled after completion if the `select!` call is in a loop, so when
`select!`ing in a loop, users should take care to `fuse()` outside of
the loop.

`select!` can be used as an expression and will return the return
value of the selected branch. For this reason the return type of every
branch in a `select!` must be the same.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

```
# futures::executor::block_on(async {
use futures::future;
use futures::select;
let mut a = future::ready(4);
let mut b = future::pending::<()>();

let res = select! {
    a_res = a => a_res + 1,
    _ = b => 0,
};
assert_eq!(res, 5);
# });
```

```
# futures::executor::block_on(async {
use futures::future;
use futures::stream::{self, StreamExt};
use futures::select;
let mut st = stream::iter(vec![2]).fuse();
let mut fut = future::pending::<()>();

select! {
    x = st.next() => assert_eq!(Some(2), x),
    _ = fut => panic!(),
};
# });
```

As described earlier, `select` can directly select on expressions
which return `Future`s - even if those do not implement `Unpin`:

```
# futures::executor::block_on(async {
use futures::future::FutureExt;
use futures::select;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn async_identity_fn(arg: usize) -> usize {
    arg
}

let res = select! {
    a_res = async_identity_fn(62).fuse() => a_res + 1,
    b_res = async_identity_fn(13).fuse() => b_res,
};
assert!(res == 63 || res == 13);
# });
```

If a similar async function is called outside of `select` to produce
a `Future`, the `Future` must be pinned in order to be able to pass
it to `select`. This can be achieved via `Box::pin` for pinning a
`Future` on the heap or the `pin_mut!` macro for pinning a `Future`
on the stack.

```
# futures::executor::block_on(async {
use futures::future::FutureExt;
use futures::select;
use futures::pin_mut;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn async_identity_fn(arg: usize) -> usize {
    arg
}

let fut_1 = async_identity_fn(1).fuse();
let fut_2 = async_identity_fn(2).fuse();
let mut fut_1 = Box::pin(fut_1); // Pins the Future on the heap
pin_mut!(fut_2); // Pins the Future on the stack

let res = select! {
    a_res = fut_1 => a_res,
    b_res = fut_2 => b_res,
};
assert!(res == 1 || res == 2);
# });
```

`select` also accepts a `complete` branch and a `default` branch.
`complete` will run if all futures and streams have already been
exhausted. `default` will run if no futures or streams are
immediately ready. `complete` takes priority over `default` in
the case where all futures have completed.
A motivating use-case for passing `Future`s by name as well as for
`complete` blocks is to call `select!` in a loop, which is
demonstrated in the following example:

```
# futures::executor::block_on(async {
use futures::future;
use futures::select;
let mut a_fut = future::ready(4);
let mut b_fut = future::ready(6);
let mut total = 0;

loop {
    select! {
        a = a_fut => total += a,
        b = b_fut => total += b,
        complete => break,
        default => panic!(), // never runs (futures run first, then complete)
    };
}
assert_eq!(total, 10);
# });
```

Note that the futures that have been matched over can still be mutated
from inside the `select!` block's branches. This can be used to implement
more complex behavior such as timer resets or writing into the head of
a stream.

```rust
pub macro_rules! select {
    /* macro_rules! select {
    ($($tokens:tt)*) => { ... };
} */
}
```

## Re-exports

### Re-export `ready`

```rust
pub use futures_core::ready;
```

### Re-export `pin_mut`

```rust
pub use pin_utils::pin_mut;
```

### Re-export `Future`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::future::Future;
```

### Re-export `FutureExt`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::future::FutureExt;
```

### Re-export `TryFuture`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::future::TryFuture;
```

### Re-export `TryFutureExt`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::future::TryFutureExt;
```

### Re-export `Stream`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::stream::Stream;
```

### Re-export `StreamExt`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::stream::StreamExt;
```

### Re-export `TryStream`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::stream::TryStream;
```

### Re-export `TryStreamExt`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::stream::TryStreamExt;
```

### Re-export `Sink`

**Attributes:**

- `#[<cfg>(feature = "sink")]`
- `#[doc(no_inline)]`

```rust
pub use crate::sink::Sink;
```

### Re-export `SinkExt`

**Attributes:**

- `#[<cfg>(feature = "sink")]`
- `#[doc(no_inline)]`

```rust
pub use crate::sink::SinkExt;
```

### Re-export `AsyncBufRead`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg>(feature = "std")]`
- `#[doc(no_inline)]`

```rust
pub use crate::io::AsyncBufRead;
```

### Re-export `AsyncBufReadExt`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg>(feature = "std")]`
- `#[doc(no_inline)]`

```rust
pub use crate::io::AsyncBufReadExt;
```

### Re-export `AsyncRead`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg>(feature = "std")]`
- `#[doc(no_inline)]`

```rust
pub use crate::io::AsyncRead;
```

### Re-export `AsyncReadExt`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg>(feature = "std")]`
- `#[doc(no_inline)]`

```rust
pub use crate::io::AsyncReadExt;
```

### Re-export `AsyncSeek`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg>(feature = "std")]`
- `#[doc(no_inline)]`

```rust
pub use crate::io::AsyncSeek;
```

### Re-export `AsyncSeekExt`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg>(feature = "std")]`
- `#[doc(no_inline)]`

```rust
pub use crate::io::AsyncSeekExt;
```

### Re-export `AsyncWrite`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg>(feature = "std")]`
- `#[doc(no_inline)]`

```rust
pub use crate::io::AsyncWrite;
```

### Re-export `AsyncWriteExt`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg>(feature = "std")]`
- `#[doc(no_inline)]`

```rust
pub use crate::io::AsyncWriteExt;
```

