# Crate Documentation

**Version:** 0.3.31

**Format Version:** 43

# Module `futures_core`

Core traits and types for asynchronous operations in Rust.

## Modules

## Module `future`

Futures.

```rust
pub mod future { /* ... */ }
```

### Types

#### Type Alias `BoxFuture`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

An owned dynamically typed [`Future`] for use in cases where you can't
statically type your result or need to add some indirection.

This type is often created by the [`boxed`] method on [`FutureExt`]. See its documentation for more.

[`boxed`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed
[`FutureExt`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html

```rust
pub type BoxFuture<''a, T> = core::pin::Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + ''a>>;
```

#### Type Alias `LocalBoxFuture`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

`BoxFuture`, but without the `Send` requirement.

This type is often created by the [`boxed_local`] method on [`FutureExt`]. See its documentation for more.

[`boxed_local`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed_local
[`FutureExt`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html

```rust
pub type LocalBoxFuture<''a, T> = core::pin::Pin<alloc::boxed::Box<dyn Future<Output = T> + ''a>>;
```

### Traits

#### Trait `FusedFuture`

A future which tracks whether or not the underlying future
should no longer be polled.

`is_terminated` will return `true` if a future should no longer be polled.
Usually, this state occurs after `poll` (or `try_poll`) returned
`Poll::Ready`. However, `is_terminated` may also return `true` if a future
has become inactive and can no longer make progress and should be ignored
or dropped rather than being `poll`ed again.

```rust
pub trait FusedFuture: Future {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `is_terminated`: Returns `true` if the underlying future should no longer be polled.

##### Implementations

This trait is implemented for the following types:

- `&mut F` with <F: FusedFuture + ?Sized + Unpin>
- `core::pin::Pin<P>` with <P>
- `alloc::boxed::Box<F>` with <F: FusedFuture + ?Sized + Unpin>
- `std::panic::AssertUnwindSafe<F>` with <F: FusedFuture>

#### Trait `TryFuture`

A convenience for futures that return `Result` values that includes
a variety of adapters tailored to such futures.

```rust
pub trait TryFuture: Future + private_try_future::Sealed {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Ok`: The type of successful values yielded by this future
- `Error`: The type of failures yielded by this future

###### Required Methods

- `try_poll`: Poll this `TryFuture` as if it were a `Future`.

##### Implementations

This trait is implemented for the following types:

- `F` with <F, T, E>

### Re-exports

#### Re-export `Future`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::future::Future;
```

## Module `stream`

Asynchronous streams.

```rust
pub mod stream { /* ... */ }
```

### Types

#### Type Alias `BoxStream`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

An owned dynamically typed [`Stream`] for use in cases where you can't
statically type your result or need to add some indirection.

This type is often created by the [`boxed`] method on [`StreamExt`]. See its documentation for more.

[`boxed`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.boxed
[`StreamExt`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html

```rust
pub type BoxStream<''a, T> = core::pin::Pin<alloc::boxed::Box<dyn Stream<Item = T> + Send + ''a>>;
```

#### Type Alias `LocalBoxStream`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

`BoxStream`, but without the `Send` requirement.

This type is often created by the [`boxed_local`] method on [`StreamExt`]. See its documentation for more.

[`boxed_local`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.boxed_local
[`StreamExt`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html

```rust
pub type LocalBoxStream<''a, T> = core::pin::Pin<alloc::boxed::Box<dyn Stream<Item = T> + ''a>>;
```

### Traits

#### Trait `Stream`

**Attributes:**

- `#[must_use = "streams do nothing unless polled"]`

A stream of values produced asynchronously.

If `Future<Output = T>` is an asynchronous version of `T`, then `Stream<Item
= T>` is an asynchronous version of `Iterator<Item = T>`. A stream
represents a sequence of value-producing events that occur asynchronously to
the caller.

The trait is modeled after `Future`, but allows `poll_next` to be called
even after a value has been produced, yielding `None` once the stream has
been fully exhausted.

```rust
pub trait Stream {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Item`: Values yielded by the stream.

###### Required Methods

- `poll_next`: Attempt to pull out the next value of this stream, registering the

##### Provided Methods

- ```rust
  fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
  ```
  Returns the bounds on the remaining length of the stream.

##### Implementations

This trait is implemented for the following types:

- `&mut S` with <S: ?Sized + Stream + Unpin>
- `core::pin::Pin<P>` with <P>
- `alloc::boxed::Box<S>` with <S: ?Sized + Stream + Unpin>
- `std::panic::AssertUnwindSafe<S>` with <S: Stream>

#### Trait `FusedStream`

A stream which tracks whether or not the underlying stream
should no longer be polled.

`is_terminated` will return `true` if a future should no longer be polled.
Usually, this state occurs after `poll_next` (or `try_poll_next`) returned
`Poll::Ready(None)`. However, `is_terminated` may also return `true` if a
stream has become inactive and can no longer make progress and should be
ignored or dropped rather than being polled again.

```rust
pub trait FusedStream: Stream {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `is_terminated`: Returns `true` if the stream should no longer be polled.

##### Implementations

This trait is implemented for the following types:

- `&mut F` with <F: ?Sized + FusedStream + Unpin>
- `core::pin::Pin<P>` with <P>
- `alloc::boxed::Box<S>` with <S: ?Sized + FusedStream + Unpin>

#### Trait `TryStream`

A convenience for streams that return `Result` values that includes
a variety of adapters tailored to such futures.

```rust
pub trait TryStream: Stream + private_try_stream::Sealed {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Ok`: The type of successful values yielded by this future
- `Error`: The type of failures yielded by this future

###### Required Methods

- `try_poll_next`: Poll this `TryStream` as if it were a `Stream`.

##### Implementations

This trait is implemented for the following types:

- `S` with <S, T, E>

## Module `task`

**Attributes:**

- `#[macro_use]`

Task notification.

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

## Macros

### Macro `ready`

**Attributes:**

- `#[macro_export]`

Extracts the successful type of a `Poll<T>`.

This macro bakes in propagation of `Pending` signals by returning early.

```rust
pub macro_rules! ready {
    /* macro_rules! ready {
    ($e:expr $(,)?) => { ... };
} */
}
```

## Re-exports

### Re-export `FusedFuture`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use self::future::FusedFuture;
```

### Re-export `Future`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use self::future::Future;
```

### Re-export `TryFuture`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use self::future::TryFuture;
```

### Re-export `FusedStream`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use self::stream::FusedStream;
```

### Re-export `Stream`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use self::stream::Stream;
```

### Re-export `TryStream`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use self::stream::TryStream;
```

