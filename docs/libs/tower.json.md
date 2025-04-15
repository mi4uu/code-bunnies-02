# Crate Documentation

**Version:** 0.5.2

**Format Version:** 43

# Module `tower`

`async fn(Request) -> Result<Response, Error>`

# Overview

Tower is a library of modular and reusable components for building
robust networking clients and servers.

Tower provides a simple core abstraction, the [`Service`] trait, which
represents an asynchronous function taking a request and returning either a
response or an error. This abstraction can be used to model both clients and
servers.

Generic components, like [`timeout`], [rate limiting], and [load balancing],
can be modeled as [`Service`]s that wrap some inner service and apply
additional behavior before or after the inner service is called. This allows
implementing these components in a protocol-agnostic, composable way. Typically,
such services are referred to as _middleware_.

An additional abstraction, the [`Layer`] trait, is used to compose
middleware with [`Service`]s. If a [`Service`] can be thought of as an
asynchronous function from a request type to a response type, a [`Layer`] is
a function taking a [`Service`] of one type and returning a [`Service`] of a
different type. The [`ServiceBuilder`] type is used to add middleware to a
service by composing it with multiple [`Layer`]s.

## The Tower Ecosystem

Tower is made up of the following crates:

* [`tower`] (this crate)
* [`tower-service`]
* [`tower-layer`]
* [`tower-test`]

Since the [`Service`] and [`Layer`] traits are important integration points
for all libraries using Tower, they are kept as stable as possible, and
breaking changes are made rarely. Therefore, they are defined in separate
crates, [`tower-service`] and [`tower-layer`]. This crate contains
re-exports of those core traits, implementations of commonly-used
middleware, and [utilities] for working with [`Service`]s and [`Layer`]s.
Finally, the [`tower-test`] crate provides tools for testing programs using
Tower.

# Usage

Tower provides an abstraction layer, and generic implementations of various
middleware. This means that the `tower` crate on its own does *not* provide
a working implementation of a network client or server. Instead, Tower's
[`Service` trait][`Service`] provides an integration point between
application code, libraries providing middleware implementations, and
libraries that implement servers and/or clients for various network
protocols.

Depending on your particular use case, you might use Tower in several ways:

* **Implementing application logic** for a networked program. You might
  use the [`Service`] trait to model your application's behavior, and use
  the middleware [provided by this crate](#modules) and by other libraries
  to add functionality to clients and servers provided by one or more
  protocol implementations.
* **Implementing middleware** to add custom behavior to network clients and
  servers in a reusable manner. This might be general-purpose middleware
  (and if it is, please consider releasing your middleware as a library for
  other Tower users!) or application-specific behavior that needs to be
  shared between multiple clients or servers.
* **Implementing a network protocol**. Libraries that implement network
  protocols (such as HTTP) can depend on `tower-service` to use the
  [`Service`] trait as an integration point between the protocol and user
  code. For example, a client for some protocol might implement [`Service`],
  allowing users to add arbitrary Tower middleware to those clients.
  Similarly, a server might be created from a user-provided [`Service`].

  Additionally, when a network protocol requires functionality already
  provided by existing Tower middleware, a protocol implementation might use
  Tower middleware internally, as well as as an integration point.

## Library Support

A number of third-party libraries support Tower and the [`Service`] trait.
The following is an incomplete list of such libraries:

* [`hyper`]: A fast and correct low-level HTTP implementation.
* [`tonic`]: A [gRPC-over-HTTP/2][grpc] implementation built on top of
  [`hyper`]. See [here][tonic-examples] for examples of using [`tonic`] with
  Tower.
* [`warp`]: A lightweight, composable web framework. See
  [here][warp-service] for details on using [`warp`] with Tower.
* [`tower-lsp`]: implementations of the [Language
  Server Protocol][lsp] based on Tower.

[`hyper`]: https://crates.io/crates/hyper
[`tonic`]: https://crates.io/crates/tonic
[tonic-examples]: https://github.com/hyperium/tonic/tree/master/examples/src/tower
[grpc]: https://grpc.io
[`warp`]: https://crates.io/crates/warp
[warp-service]: https://docs.rs/warp/0.2.5/warp/fn.service.html
[`tower-lsp`]: https://crates.io/crates/tower-lsp
[lsp]: https://microsoft.github.io/language-server-protocol/

If you're the maintainer of a crate that supports Tower, we'd love to add
your crate to this list! Please [open a PR] adding a brief description of
your library!

## Getting Started

If you're brand new to Tower and want to start with the basics, we recommend you
check out some of our [guides].

The various middleware implementations provided by this crate are feature
flagged, so that users can only compile the parts of Tower they need. By
default, all the optional middleware are disabled.

To get started using all of Tower's optional middleware, add this to your
`Cargo.toml`:

```toml
tower = { version = "0.4", features = ["full"] }
```

Alternatively, you can only enable some features. For example, to enable
only the [`retry`] and [`timeout`] middleware, write:

```toml
tower = { version = "0.4", features = ["retry", "timeout"] }
```

See [here](#modules) for a complete list of all middleware provided by
Tower.


## Supported Rust Versions

Tower will keep a rolling MSRV (minimum supported Rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.64.0.

[`Service`]: crate::Service
[`Layer`]: crate::Layer
[rate limiting]: crate::limit::rate
[load balancing]: crate::balance
[`ServiceBuilder`]: crate::ServiceBuilder
[utilities]: crate::ServiceExt
[`tower`]: https://crates.io/crates/tower
[`tower-service`]: https://crates.io/crates/tower-service
[`tower-layer`]: https://crates.io/crates/tower-layer
[`tower-test`]: https://crates.io/crates/tower-test
[`retry`]: crate::retry
[open a PR]: https://github.com/tower-rs/tower/compare
[guides]: https://github.com/tower-rs/tower/tree/master/guides

## Modules

## Module `timeout`

**Attributes:**

- `#[<cfg>(feature = "timeout")]`

Middleware that applies a timeout to requests.

If the response does not complete within the specified timeout, the response
will be aborted.

```rust
pub mod timeout { /* ... */ }
```

### Modules

## Module `error`

Error types

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `Elapsed`

The timeout elapsed.

```rust
pub struct Elapsed(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Construct a new elapsed error

###### Trait Implementations

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

- **RefUnwindSafe**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Error**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Elapsed { /* ... */ }
    ```

## Module `future`

Future types

```rust
pub mod future { /* ... */ }
```

### Types

#### Struct `ResponseFuture`

[`Timeout`] response future

[`Timeout`]: crate::timeout::Timeout

```rust
pub struct ResponseFuture<T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FutureExt**
- **TryFutureExt**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

### Types

#### Struct `Timeout`

Applies a timeout to requests.

```rust
pub struct Timeout<T> {
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
  pub const fn new(inner: T, timeout: Duration) -> Self { /* ... */ }
  ```
  Creates a new [`Timeout`]

- ```rust
  pub fn get_ref(self: &Self) -> &T { /* ... */ }
  ```
  Get a reference to the inner service

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Get a mutable reference to the inner service

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Consume `self`, returning the inner service

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Timeout<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Service**
  - ```rust
    fn poll_ready(self: &mut Self, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn call(self: &mut Self, request: Request) -> <Self as >::Future { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **ServiceExt**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
### Re-exports

#### Re-export `TimeoutLayer`

```rust
pub use self::layer::TimeoutLayer;
```

## Module `util`

**Attributes:**

- `#[<cfg>(feature = "util")]`

Various utility types and functions that are generally used with Tower.

```rust
pub mod util { /* ... */ }
```

### Modules

## Module `rng`

[PRNG] utilities for tower middleware.

This module provides a generic [`Rng`] trait and a [`HasherRng`] that
implements the trait based on [`RandomState`] or any other [`Hasher`].

These utilities replace tower's internal usage of `rand` with these smaller,
more lightweight methods. Most of the implementations are extracted from
their corresponding `rand` implementations.

[PRNG]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator

```rust
pub mod rng { /* ... */ }
```

### Types

#### Struct `HasherRng`

A [`Rng`] implementation that uses a [`Hasher`] to generate the random
values. The implementation uses an internal counter to pass to the hasher
for each iteration of [`Rng::next_u64`].

# Default

This hasher has a default type of [`RandomState`] which just uses the
libstd method of getting a random u64.

```rust
pub struct HasherRng<H = std::collections::hash_map::RandomState> {
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
  Create a new default [`HasherRng`].

- ```rust
  pub fn with_hasher(hasher: H) -> Self { /* ... */ }
  ```
  Create a new [`HasherRng`] with the provided hasher.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> HasherRng<H> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Rng**
  - ```rust
    fn next_u64(self: &mut Self) -> u64 { /* ... */ }
    ```

- **Unpin**
### Traits

#### Trait `Rng`

A simple [PRNG] trait for use within tower middleware.

[PRNG]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator

```rust
pub trait Rng {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `next_u64`: Generate a random [`u64`].

##### Provided Methods

- ```rust
  fn next_f64(self: &mut Self) -> f64 { /* ... */ }
  ```
  Generate a random [`f64`] between `[0, 1)`.

- ```rust
  fn next_range(self: &mut Self, range: Range<u64>) -> u64 { /* ... */ }
  ```
  Randomly pick a value within the range.

##### Implementations

This trait is implemented for the following types:

- `Box<R>` with <R: Rng + ?Sized>
- `HasherRng<H>` with <H>

## Module `error`

Error types

```rust
pub mod error { /* ... */ }
```

### Re-exports

#### Re-export `error`

```rust
pub use super::optional::error as optional;
```

## Module `future`

Future types

```rust
pub mod future { /* ... */ }
```

### Re-exports

#### Re-export `AndThenFuture`

```rust
pub use super::and_then::AndThenFuture;
```

#### Re-export `EitherResponseFuture`

```rust
pub use super::either::EitherResponseFuture;
```

#### Re-export `MapErrFuture`

```rust
pub use super::map_err::MapErrFuture;
```

#### Re-export `MapResponseFuture`

```rust
pub use super::map_response::MapResponseFuture;
```

#### Re-export `MapResultFuture`

```rust
pub use super::map_result::MapResultFuture;
```

#### Re-export `future`

```rust
pub use super::optional::future as optional;
```

#### Re-export `ThenFuture`

```rust
pub use super::then::ThenFuture;
```

### Traits

#### Trait `ServiceExt`

An extension trait for `Service`s that provides a variety of convenient
adapters

```rust
pub trait ServiceExt<Request>: tower_service::Service<Request> {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn ready(self: &mut Self) -> Ready<''_, Self, Request>
where
    Self: Sized { /* ... */ }
  ```
  Yields a mutable reference to the service when it is ready to accept a request.

- ```rust
  fn ready_oneshot(self: Self) -> ReadyOneshot<Self, Request>
where
    Self: Sized { /* ... */ }
  ```
  Yields the service when it is ready to accept a request.

- ```rust
  fn oneshot(self: Self, req: Request) -> Oneshot<Self, Request>
where
    Self: Sized { /* ... */ }
  ```
  Consume this `Service`, calling it with the provided request once it is ready.

- ```rust
  fn call_all<S>(self: Self, reqs: S) -> CallAll<Self, S>
where
    Self: Sized,
    S: futures_core::Stream<Item = Request> { /* ... */ }
  ```
  Process all requests from the given [`Stream`], and produce a [`Stream`] of their responses.

- ```rust
  fn and_then<F>(self: Self, f: F) -> AndThen<Self, F>
where
    Self: Sized,
    F: Clone { /* ... */ }
  ```
  Executes a new future after this service's future resolves. This does

- ```rust
  fn map_response<F, Response>(self: Self, f: F) -> MapResponse<Self, F>
where
    Self: Sized,
    F: FnOnce(<Self as >::Response) -> Response + Clone { /* ... */ }
  ```
  Maps this service's response value to a different value. This does not

- ```rust
  fn map_err<F, Error>(self: Self, f: F) -> MapErr<Self, F>
where
    Self: Sized,
    F: FnOnce(<Self as >::Error) -> Error + Clone { /* ... */ }
  ```
  Maps this service's error value to a different value. This does not

- ```rust
  fn map_result<F, Response, Error>(self: Self, f: F) -> MapResult<Self, F>
where
    Self: Sized,
    Error: From<<Self as >::Error>,
    F: FnOnce(Result<<Self as >::Response, <Self as >::Error>) -> Result<Response, Error> + Clone { /* ... */ }
  ```
  Maps this service's result type (`Result<Self::Response, Self::Error>`)

- ```rust
  fn map_request<F, NewRequest>(self: Self, f: F) -> MapRequest<Self, F>
where
    Self: Sized,
    F: FnMut(NewRequest) -> Request { /* ... */ }
  ```
  Composes a function *in front of* the service.

- ```rust
  fn then<F, Response, Error, Fut>(self: Self, f: F) -> Then<Self, F>
where
    Self: Sized,
    Error: From<<Self as >::Error>,
    F: FnOnce(Result<<Self as >::Response, <Self as >::Error>) -> Fut + Clone,
    Fut: Future<Output = Result<Response, Error>> { /* ... */ }
  ```
  Composes an asynchronous function *after* this service.

- ```rust
  fn map_future<F, Fut, Response, Error>(self: Self, f: F) -> MapFuture<Self, F>
where
    Self: Sized,
    F: FnMut(<Self as >::Future) -> Fut,
    Error: From<<Self as >::Error>,
    Fut: Future<Output = Result<Response, Error>> { /* ... */ }
  ```
  Composes a function that transforms futures produced by the service.

- ```rust
  fn boxed(self: Self) -> BoxService<Request, <Self as >::Response, <Self as >::Error>
where
    Self: Sized + Send + ''static,
    <Self as >::Future: Send + ''static { /* ... */ }
  ```
  Convert the service into a [`Service`] + [`Send`] trait object.

- ```rust
  fn boxed_clone(self: Self) -> BoxCloneService<Request, <Self as >::Response, <Self as >::Error>
where
    Self: Clone + Sized + Send + ''static,
    <Self as >::Future: Send + ''static { /* ... */ }
  ```
  Convert the service into a [`Service`] + [`Clone`] + [`Send`] trait object.

##### Implementations

This trait is implemented for the following types:

- `T` with <T, Request>

### Functions

#### Function `option_layer`

Convert an `Option<Layer>` into a [`Layer`].

```
# use std::time::Duration;
# use tower::Service;
# use tower::builder::ServiceBuilder;
use tower::util::option_layer;
# use tower::timeout::TimeoutLayer;
# async fn wrap<S>(svc: S) where S: Service<(), Error = &'static str> + 'static + Send, S::Future: Send {
# let timeout = Some(Duration::new(10, 0));
// Layer to apply a timeout if configured
let maybe_timeout = option_layer(timeout.map(TimeoutLayer::new));

ServiceBuilder::new()
    .layer(maybe_timeout)
    .service(svc);
# }
```

[`Layer`]: crate::layer::Layer

```rust
pub fn option_layer<L>(layer: Option<L>) -> Either<L, crate::layer::util::Identity> { /* ... */ }
```

### Re-exports

#### Re-export `AndThen`

```rust
pub use self::and_then::AndThen;
```

#### Re-export `AndThenLayer`

```rust
pub use self::and_then::AndThenLayer;
```

#### Re-export `BoxCloneServiceLayer`

```rust
pub use self::boxed::BoxCloneServiceLayer;
```

#### Re-export `BoxCloneSyncServiceLayer`

```rust
pub use self::boxed::BoxCloneSyncServiceLayer;
```

#### Re-export `BoxLayer`

```rust
pub use self::boxed::BoxLayer;
```

#### Re-export `BoxService`

```rust
pub use self::boxed::BoxService;
```

#### Re-export `UnsyncBoxService`

```rust
pub use self::boxed::UnsyncBoxService;
```

#### Re-export `BoxCloneService`

```rust
pub use self::boxed_clone::BoxCloneService;
```

#### Re-export `BoxCloneSyncService`

```rust
pub use self::boxed_clone_sync::BoxCloneSyncService;
```

#### Re-export `Either`

```rust
pub use self::either::Either;
```

#### Re-export `future_service`

```rust
pub use self::future_service::future_service;
```

#### Re-export `FutureService`

```rust
pub use self::future_service::FutureService;
```

#### Re-export `MapErr`

```rust
pub use self::map_err::MapErr;
```

#### Re-export `MapErrLayer`

```rust
pub use self::map_err::MapErrLayer;
```

#### Re-export `MapFuture`

```rust
pub use self::map_future::MapFuture;
```

#### Re-export `MapFutureLayer`

```rust
pub use self::map_future::MapFutureLayer;
```

#### Re-export `MapRequest`

```rust
pub use self::map_request::MapRequest;
```

#### Re-export `MapRequestLayer`

```rust
pub use self::map_request::MapRequestLayer;
```

#### Re-export `MapResponse`

```rust
pub use self::map_response::MapResponse;
```

#### Re-export `MapResponseLayer`

```rust
pub use self::map_response::MapResponseLayer;
```

#### Re-export `MapResult`

```rust
pub use self::map_result::MapResult;
```

#### Re-export `MapResultLayer`

```rust
pub use self::map_result::MapResultLayer;
```

#### Re-export `Oneshot`

```rust
pub use self::oneshot::Oneshot;
```

#### Re-export `Optional`

```rust
pub use self::optional::Optional;
```

#### Re-export `Ready`

```rust
pub use self::ready::Ready;
```

#### Re-export `ReadyOneshot`

```rust
pub use self::ready::ReadyOneshot;
```

#### Re-export `service_fn`

```rust
pub use self::service_fn::service_fn;
```

#### Re-export `ServiceFn`

```rust
pub use self::service_fn::ServiceFn;
```

#### Re-export `Then`

```rust
pub use self::then::Then;
```

#### Re-export `ThenLayer`

```rust
pub use self::then::ThenLayer;
```

#### Re-export `CallAll`

```rust
pub use self::call_all::CallAll;
```

#### Re-export `CallAllUnordered`

```rust
pub use self::call_all::CallAllUnordered;
```

## Module `builder`

Builder types to compose layers and services

```rust
pub mod builder { /* ... */ }
```

### Types

#### Struct `ServiceBuilder`

Declaratively construct [`Service`] values.

[`ServiceBuilder`] provides a [builder-like interface][builder] for composing
layers to be applied to a [`Service`].

# Service

A [`Service`] is a trait representing an asynchronous function of a request
to a response. It is similar to `async fn(Request) -> Result<Response, Error>`.

A [`Service`] is typically bound to a single transport, such as a TCP
connection.  It defines how _all_ inbound or outbound requests are handled
by that connection.

[builder]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html

# Order

The order in which layers are added impacts how requests are handled. Layers
that are added first will be called with the request first. The argument to
`service` will be last to see the request.

```
# // this (and other) doctest is ignored because we don't have a way
# // to say that it should only be run with cfg(feature = "...")
# use tower::Service;
# use tower::builder::ServiceBuilder;
# #[cfg(all(feature = "buffer", feature = "limit"))]
# async fn wrap<S>(svc: S) where S: Service<(), Error = &'static str> + 'static + Send, S::Future: Send {
ServiceBuilder::new()
    .buffer(100)
    .concurrency_limit(10)
    .service(svc)
# ;
# }
```

In the above example, the buffer layer receives the request first followed
by `concurrency_limit`. `buffer` enables up to 100 request to be in-flight
**on top of** the requests that have already been forwarded to the next
layer. Combined with `concurrency_limit`, this allows up to 110 requests to be
in-flight.

```
# use tower::Service;
# use tower::builder::ServiceBuilder;
# #[cfg(all(feature = "buffer", feature = "limit"))]
# async fn wrap<S>(svc: S) where S: Service<(), Error = &'static str> + 'static + Send, S::Future: Send {
ServiceBuilder::new()
    .concurrency_limit(10)
    .buffer(100)
    .service(svc)
# ;
# }
```

The above example is similar, but the order of layers is reversed. Now,
`concurrency_limit` applies first and only allows 10 requests to be in-flight
total.

# Examples

A [`Service`] stack with a single layer:

```
# use tower::Service;
# use tower::builder::ServiceBuilder;
# #[cfg(feature = "limit")]
# use tower::limit::concurrency::ConcurrencyLimitLayer;
# #[cfg(feature = "limit")]
# async fn wrap<S>(svc: S) where S: Service<(), Error = &'static str> + 'static + Send, S::Future: Send {
ServiceBuilder::new()
    .concurrency_limit(5)
    .service(svc);
# ;
# }
```

A [`Service`] stack with _multiple_ layers that contain rate limiting,
in-flight request limits, and a channel-backed, clonable [`Service`]:

```
# use tower::Service;
# use tower::builder::ServiceBuilder;
# use std::time::Duration;
# #[cfg(all(feature = "buffer", feature = "limit"))]
# async fn wrap<S>(svc: S) where S: Service<(), Error = &'static str> + 'static + Send, S::Future: Send {
ServiceBuilder::new()
    .buffer(5)
    .concurrency_limit(5)
    .rate_limit(5, Duration::from_secs(1))
    .service(svc);
# ;
# }
```

[`Service`]: crate::Service

```rust
pub struct ServiceBuilder<L> {
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
  pub const fn new() -> Self { /* ... */ }
  ```
  Create a new [`ServiceBuilder`].

- ```rust
  pub fn layer<T>(self: Self, layer: T) -> ServiceBuilder<Stack<T, L>> { /* ... */ }
  ```
  Add a new layer `T` into the [`ServiceBuilder`].

- ```rust
  pub fn option_layer<T>(self: Self, layer: Option<T>) -> ServiceBuilder<Stack<crate::util::Either<T, Identity>, L>> { /* ... */ }
  ```
  Optionally add a new layer `T` into the [`ServiceBuilder`].

- ```rust
  pub fn layer_fn<F>(self: Self, f: F) -> ServiceBuilder<Stack<crate::layer::LayerFn<F>, L>> { /* ... */ }
  ```
  Add a [`Layer`] built from a function that accepts a service and returns another service.

- ```rust
  pub fn timeout(self: Self, timeout: std::time::Duration) -> ServiceBuilder<Stack<crate::timeout::TimeoutLayer, L>> { /* ... */ }
  ```
  Fail requests that take longer than `timeout`.

- ```rust
  pub fn map_request<F, R1, R2>(self: Self, f: F) -> ServiceBuilder<Stack<crate::util::MapRequestLayer<F>, L>>
where
    F: FnMut(R1) -> R2 + Clone { /* ... */ }
  ```
  Map one request type to another.

- ```rust
  pub fn map_response<F>(self: Self, f: F) -> ServiceBuilder<Stack<crate::util::MapResponseLayer<F>, L>> { /* ... */ }
  ```
  Map one response type to another.

- ```rust
  pub fn map_err<F>(self: Self, f: F) -> ServiceBuilder<Stack<crate::util::MapErrLayer<F>, L>> { /* ... */ }
  ```
  Map one error type to another.

- ```rust
  pub fn map_future<F>(self: Self, f: F) -> ServiceBuilder<Stack<crate::util::MapFutureLayer<F>, L>> { /* ... */ }
  ```
  Composes a function that transforms futures produced by the service.

- ```rust
  pub fn then<F>(self: Self, f: F) -> ServiceBuilder<Stack<crate::util::ThenLayer<F>, L>> { /* ... */ }
  ```
  Apply an asynchronous function after the service, regardless of whether the future

- ```rust
  pub fn and_then<F>(self: Self, f: F) -> ServiceBuilder<Stack<crate::util::AndThenLayer<F>, L>> { /* ... */ }
  ```
  Executes a new future after this service's future resolves. This does

- ```rust
  pub fn map_result<F>(self: Self, f: F) -> ServiceBuilder<Stack<crate::util::MapResultLayer<F>, L>> { /* ... */ }
  ```
  Maps this service's result type (`Result<Self::Response, Self::Error>`)

- ```rust
  pub fn into_inner(self: Self) -> L { /* ... */ }
  ```
  Returns the underlying `Layer` implementation.

- ```rust
  pub fn service<S>(self: &Self, service: S) -> <L as >::Service
where
    L: Layer<S> { /* ... */ }
  ```
  Wrap the service `S` with the middleware provided by this

- ```rust
  pub fn service_fn<F>(self: Self, f: F) -> <L as >::Service
where
    L: Layer<crate::util::ServiceFn<F>> { /* ... */ }
  ```
  Wrap the async function `F` with the middleware provided by this [`ServiceBuilder`]'s

- ```rust
  pub fn check_clone(self: Self) -> Self
where
    Self: Clone { /* ... */ }
  ```
  Check that the builder implements `Clone`.

- ```rust
  pub fn check_service_clone<S>(self: Self) -> Self
where
    L: Layer<S>,
    <L as >::Service: Clone { /* ... */ }
  ```
  Check that the builder when given a service of type `S` produces a service that implements

- ```rust
  pub fn check_service<S, T, U, E>(self: Self) -> Self
where
    L: Layer<S>,
    <L as >::Service: Service<T, Response = U, Error = E> { /* ... */ }
  ```
  Check that the builder when given a service of type `S` produces a service with the given

- ```rust
  pub fn boxed<S, R>(self: Self) -> ServiceBuilder<Stack<tower_layer::LayerFn<fn(<L as >::Service) -> crate::util::BoxService<R, <<L as >::Service as Service<R>>::Response, <<L as >::Service as Service<R>>::Error>>, L>>
where
    L: Layer<S>,
    <L as >::Service: Service<R> + Send + ''static,
    <<L as >::Service as Service<R>>::Future: Send + ''static { /* ... */ }
  ```
  This wraps the inner service with the [`Layer`] returned by [`BoxService::layer()`].

- ```rust
  pub fn boxed_clone<S, R>(self: Self) -> ServiceBuilder<Stack<tower_layer::LayerFn<fn(<L as >::Service) -> crate::util::BoxCloneService<R, <<L as >::Service as Service<R>>::Response, <<L as >::Service as Service<R>>::Error>>, L>>
where
    L: Layer<S>,
    <L as >::Service: Service<R> + Clone + Send + ''static,
    <<L as >::Service as Service<R>>::Future: Send + ''static { /* ... */ }
  ```
  This wraps the inner service with the [`Layer`] returned by [`BoxCloneService::layer()`].

###### Trait Implementations

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Layer**
  - ```rust
    fn layer(self: &Self, inner: S) -> <Self as >::Service { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ServiceBuilder<L> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

## Module `layer`

A collection of [`Layer`] based tower services

[`Layer`]: crate::Layer

```rust
pub mod layer { /* ... */ }
```

### Modules

## Module `util`

Utilities for combining layers

[`Identity`]: crate::layer::util::Identity
[`Layer`]: crate::Layer
[`Stack`]: crate::layer::util::Stack

```rust
pub mod util { /* ... */ }
```

### Re-exports

#### Re-export `Identity`

```rust
pub use tower_layer::Identity;
```

#### Re-export `Stack`

```rust
pub use tower_layer::Stack;
```

### Re-exports

#### Re-export `layer_fn`

```rust
pub use tower_layer::layer_fn;
```

#### Re-export `Layer`

```rust
pub use tower_layer::Layer;
```

#### Re-export `LayerFn`

```rust
pub use tower_layer::LayerFn;
```

## Types

### Type Alias `BoxError`

Alias for a type-erased error type.

```rust
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
```

## Re-exports

### Re-export `service_fn`

**Attributes:**

- `#[<cfg>(feature = "util")]`
- `#[doc(inline)]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "util")))]`

```rust
pub use self::util::service_fn;
```

### Re-export `ServiceExt`

**Attributes:**

- `#[<cfg>(feature = "util")]`
- `#[doc(inline)]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "util")))]`

```rust
pub use self::util::ServiceExt;
```

### Re-export `ServiceBuilder`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::builder::ServiceBuilder;
```

### Re-export `Layer`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use tower_layer::Layer;
```

### Re-export `Service`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use tower_service::Service;
```

