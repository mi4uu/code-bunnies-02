# Crate Documentation

**Version:** 0.1.3

**Format Version:** 43

# Module `http_body_util`

Utilities for [`http_body::Body`].

[`BodyExt`] adds extensions to the common trait.

[`Empty`] and [`Full`] provide simple implementations.

## Modules

## Module `combinators`

Combinators for the `Body` trait.

```rust
pub mod combinators { /* ... */ }
```

### Re-exports

#### Re-export `BoxBody`

```rust
pub use self::box_body::BoxBody;
```

#### Re-export `UnsyncBoxBody`

```rust
pub use self::box_body::UnsyncBoxBody;
```

#### Re-export `Collect`

```rust
pub use self::collect::Collect;
```

#### Re-export `Frame`

```rust
pub use self::frame::Frame;
```

#### Re-export `MapErr`

```rust
pub use self::map_err::MapErr;
```

#### Re-export `MapFrame`

```rust
pub use self::map_frame::MapFrame;
```

#### Re-export `WithTrailers`

```rust
pub use self::with_trailers::WithTrailers;
```

## Traits

### Trait `BodyExt`

An extension trait for [`http_body::Body`] adding various combinators and adapters

```rust
pub trait BodyExt: http_body::Body {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Provided Methods

- ```rust
  fn frame(self: &mut Self) -> combinators::Frame<''_, Self>
where
    Self: Unpin { /* ... */ }
  ```
  Returns a future that resolves to the next [`Frame`], if any.

- ```rust
  fn map_frame<F, B>(self: Self, f: F) -> MapFrame<Self, F>
where
    Self: Sized,
    F: FnMut(http_body::Frame<<Self as >::Data>) -> http_body::Frame<B>,
    B: bytes::Buf { /* ... */ }
  ```
  Maps this body's frame to a different kind.

- ```rust
  fn map_err<F, E>(self: Self, f: F) -> MapErr<Self, F>
where
    Self: Sized,
    F: FnMut(<Self as >::Error) -> E { /* ... */ }
  ```
  Maps this body's error value to a different value.

- ```rust
  fn boxed(self: Self) -> BoxBody<<Self as >::Data, <Self as >::Error>
where
    Self: Sized + Send + Sync + ''static { /* ... */ }
  ```
  Turn this body into a boxed trait object.

- ```rust
  fn boxed_unsync(self: Self) -> UnsyncBoxBody<<Self as >::Data, <Self as >::Error>
where
    Self: Sized + Send + ''static { /* ... */ }
  ```
  Turn this body into a boxed trait object that is !Sync.

- ```rust
  fn collect(self: Self) -> combinators::Collect<Self>
where
    Self: Sized { /* ... */ }
  ```
  Turn this body into [`Collected`] body which will collect all the DATA frames

- ```rust
  fn with_trailers<F>(self: Self, trailers: F) -> combinators::WithTrailers<Self, F>
where
    Self: Sized,
    F: std::future::Future<Output = Option<Result<http::HeaderMap, <Self as >::Error>>> { /* ... */ }
  ```
  Add trailers to the body.

- ```rust
  fn into_data_stream(self: Self) -> BodyDataStream<Self>
where
    Self: Sized { /* ... */ }
  ```
  Turn this body into [`BodyDataStream`].

#### Implementations

This trait is implemented for the following types:

- `T` with <T>

## Re-exports

### Re-export `Collected`

```rust
pub use self::collected::Collected;
```

### Re-export `Either`

```rust
pub use self::either::Either;
```

### Re-export `Empty`

```rust
pub use self::empty::Empty;
```

### Re-export `Full`

```rust
pub use self::full::Full;
```

### Re-export `LengthLimitError`

```rust
pub use self::limited::LengthLimitError;
```

### Re-export `Limited`

```rust
pub use self::limited::Limited;
```

### Re-export `BodyDataStream`

```rust
pub use self::stream::BodyDataStream;
```

### Re-export `BodyStream`

```rust
pub use self::stream::BodyStream;
```

### Re-export `StreamBody`

```rust
pub use self::stream::StreamBody;
```

