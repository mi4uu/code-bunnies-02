# Crate Documentation

**Version:** 0.1.11

**Format Version:** 43

# Module `hyper_util`

Utilities for working with hyper.

This crate is less-stable than [`hyper`](https://docs.rs/hyper). However,
does respect Rust's semantic version regarding breaking changes.

## Modules

## Module `client`

**Attributes:**

- `#[<cfg>(feature = "client")]`

HTTP client utilities

```rust
pub mod client { /* ... */ }
```

### Modules

## Module `legacy`

**Attributes:**

- `#[<cfg>(feature = "client-legacy")]`

Legacy implementations of `connect` module and `Client`

```rust
pub mod legacy { /* ... */ }
```

### Modules

## Module `connect`

Connectors used by the `Client`.

This module contains:

- A default [`HttpConnector`][] that does DNS resolution and establishes
  connections over TCP.
- Types to build custom connectors.

# Connectors

A "connector" is a [`Service`][] that takes a [`Uri`][] destination, and
its `Response` is some type implementing [`Read`][], [`Write`][],
and [`Connection`][].

## Custom Connectors

A simple connector that ignores the `Uri` destination and always returns
a TCP connection to the same address could be written like this:

```rust,ignore
let connector = tower::service_fn(|_dst| async {
    tokio::net::TcpStream::connect("127.0.0.1:1337")
})
```

Or, fully written out:

```
use std::{future::Future, net::SocketAddr, pin::Pin, task::{self, Poll}};
use http::Uri;
use tokio::net::TcpStream;
use tower_service::Service;

#[derive(Clone)]
struct LocalConnector;

impl Service<Uri> for LocalConnector {
    type Response = TcpStream;
    type Error = std::io::Error;
    // We can't "name" an `async` generated future.
    type Future = Pin<Box<
        dyn Future<Output = Result<Self::Response, Self::Error>> + Send
    >>;

    fn poll_ready(&mut self, _: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        // This connector is always ready, but others might not be.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Uri) -> Self::Future {
        Box::pin(TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], 1337))))
    }
}
```

It's worth noting that for `TcpStream`s, the [`HttpConnector`][] is a
better starting place to extend from.

[`HttpConnector`]: HttpConnector
[`Service`]: tower_service::Service
[`Uri`]: ::http::Uri
[`Read`]: hyper::rt::Read
[`Write`]: hyper::rt::Write
[`Connection`]: Connection

```rust
pub mod connect { /* ... */ }
```

### Modules

## Module `dns`

**Attributes:**

- `#[<cfg>(feature = "tokio")]`

DNS Resolution used by the `HttpConnector`.

This module contains:

- A [`GaiResolver`] that is the default resolver for the `HttpConnector`.
- The `Name` type used as an argument to custom resolvers.

# Resolvers are `Service`s

A resolver is just a
`Service<Name, Response = impl Iterator<Item = SocketAddr>>`.

A simple resolver that ignores the name and always returns a specific
address:

```rust,ignore
use std::{convert::Infallible, iter, net::SocketAddr};

let resolver = tower::service_fn(|_name| async {
    Ok::<_, Infallible>(iter::once(SocketAddr::from(([127, 0, 0, 1], 8080))))
});
```

```rust
pub mod dns { /* ... */ }
```

### Types

#### Struct `Name`

A domain name to resolve into IP addresses.

```rust
pub struct Name {
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
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  View the hostname as a string slice.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Sync**
- **Instrument**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **WithSubscriber**
- **UnwindSafe**
- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Name { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Name) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Service**
  - ```rust
    fn poll_ready(self: &mut Self, _cx: &mut task::Context<''_>) -> Poll<Result<(), io::Error>> { /* ... */ }
    ```

  - ```rust
    fn call(self: &mut Self, name: Name) -> <Self as >::Future { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(host: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `GaiResolver`

A resolver using blocking `getaddrinfo` calls in a threadpool.

```rust
pub struct GaiResolver {
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
  Construct a new `GaiResolver`.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Instrument**
- **WithSubscriber**
- **Unpin**
- **Service**
  - ```rust
    fn poll_ready(self: &mut Self, _cx: &mut task::Context<''_>) -> Poll<Result<(), io::Error>> { /* ... */ }
    ```

  - ```rust
    fn call(self: &mut Self, name: Name) -> <Self as >::Future { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GaiResolver { /* ... */ }
    ```

#### Struct `GaiAddrs`

An iterator of IP addresses returned from `getaddrinfo`.

```rust
pub struct GaiAddrs {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Instrument**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **WithSubscriber**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

#### Struct `GaiFuture`

A future to resolve a name returned by `GaiResolver`.

```rust
pub struct GaiFuture {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFutureExt**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FutureExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **Instrument**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

- **WithSubscriber**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `InvalidNameError`

Error indicating a given string was not a valid domain name.

```rust
pub struct InvalidNameError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Error**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Instrument**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **RefUnwindSafe**
### Types

#### Struct `Connected`

Extra information about the connected transport.

This can be used to inform recipients about things like if ALPN
was used, or if connected to an HTTP proxy.

```rust
pub struct Connected {
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
  pub fn new() -> Connected { /* ... */ }
  ```
  Create new `Connected` type with empty metadata.

- ```rust
  pub fn proxy(self: Self, is_proxied: bool) -> Connected { /* ... */ }
  ```
  Set whether the connected transport is to an HTTP proxy.

- ```rust
  pub fn is_proxied(self: &Self) -> bool { /* ... */ }
  ```
  Determines if the connected transport is to an HTTP proxy.

- ```rust
  pub fn extra<T: Clone + Send + Sync + ''static>(self: Self, extra: T) -> Connected { /* ... */ }
  ```
  Set extra connection information to be set in the extensions of every `Response`.

- ```rust
  pub fn get_extras(self: &Self, extensions: &mut Extensions) { /* ... */ }
  ```
  Copies the extra connection information into an `Extensions` map.

- ```rust
  pub fn negotiated_h2(self: Self) -> Connected { /* ... */ }
  ```
  Set that the connected transport negotiated HTTP/2 as its next protocol.

- ```rust
  pub fn is_negotiated_h2(self: &Self) -> bool { /* ... */ }
  ```
  Determines if the connected transport negotiated HTTP/2 as its next protocol.

- ```rust
  pub fn poison(self: &Self) { /* ... */ }
  ```
  Poison this connection

###### Trait Implementations

- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Instrument**
- **UnwindSafe**
- **WithSubscriber**
- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `Connection`

Describes a type returned by a connector.

```rust
pub trait Connection {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `connected`: Return metadata describing the connection.

##### Implementations

This trait is implemented for the following types:

- `tokio::net::TcpStream`
- `tokio::net::UnixStream`
- `crate::rt::TokioIo<T>` with <T>

### Re-exports

#### Re-export `HttpConnector`

**Attributes:**

- `#[<cfg>(feature = "tokio")]`

```rust
pub use self::http::HttpConnector;
```

#### Re-export `HttpInfo`

**Attributes:**

- `#[<cfg>(feature = "tokio")]`

```rust
pub use self::http::HttpInfo;
```

#### Re-export `capture_connection`

```rust
pub use capture::capture_connection;
```

#### Re-export `CaptureConnection`

```rust
pub use capture::CaptureConnection;
```

#### Re-export `Connect`

```rust
pub use self::sealed::Connect;
```

### Re-exports

#### Re-export `Builder`

**Attributes:**

- `#[<cfg>(any(feature = "http1", feature = "http2"))]`

```rust
pub use client::Builder;
```

#### Re-export `Client`

**Attributes:**

- `#[<cfg>(any(feature = "http1", feature = "http2"))]`

```rust
pub use client::Client;
```

#### Re-export `Error`

**Attributes:**

- `#[<cfg>(any(feature = "http1", feature = "http2"))]`

```rust
pub use client::Error;
```

#### Re-export `ResponseFuture`

**Attributes:**

- `#[<cfg>(any(feature = "http1", feature = "http2"))]`

```rust
pub use client::ResponseFuture;
```

## Module `rt`

Runtime utilities

```rust
pub mod rt { /* ... */ }
```

### Modules

## Module `tokio`

**Attributes:**

- `#[<cfg>(feature = "tokio")]`

[`tokio`] runtime components integration for [`hyper`].

[`hyper::rt`] exposes a set of traits to allow hyper to be agnostic to
its underlying asynchronous runtime. This submodule provides glue for
[`tokio`] users to bridge those types to [`hyper`]'s interfaces.

# IO

[`hyper`] abstracts over asynchronous readers and writers using [`Read`]
and [`Write`], while [`tokio`] abstracts over this using [`AsyncRead`]
and [`AsyncWrite`]. This submodule provides a collection of IO adaptors
to bridge these two IO ecosystems together: [`TokioIo<I>`],
[`WithHyperIo<I>`], and [`WithTokioIo<I>`].

To compare and constrast these IO adaptors and to help explain which
is the proper choice for your needs, here is a table showing which IO
traits these implement, given two types `T` and `H` which implement
Tokio's and Hyper's corresponding IO traits:

|                    | [`AsyncRead`]    | [`AsyncWrite`]    | [`Read`]     | [`Write`]    |
|--------------------|------------------|-------------------|--------------|--------------|
| `T`                | ✅ **true**      | ✅ **true**       | ❌ **false** | ❌ **false** |
| `H`                | ❌ **false**     | ❌ **false**      | ✅ **true**  | ✅ **true**  |
| [`TokioIo<T>`]     | ❌ **false**     | ❌ **false**      | ✅ **true**  | ✅ **true**  |
| [`TokioIo<H>`]     | ✅ **true**      | ✅ **true**       | ❌ **false** | ❌ **false** |
| [`WithHyperIo<T>`] | ✅ **true**      | ✅ **true**       | ✅ **true**  | ✅ **true**  |
| [`WithHyperIo<H>`] | ❌ **false**     | ❌ **false**      | ❌ **false** | ❌ **false** |
| [`WithTokioIo<T>`] | ❌ **false**     | ❌ **false**      | ❌ **false** | ❌ **false** |
| [`WithTokioIo<H>`] | ✅ **true**      | ✅ **true**       | ✅ **true**  | ✅ **true**  |

For most situations, [`TokioIo<I>`] is the proper choice. This should be
constructed, wrapping some underlying [`hyper`] or [`tokio`] IO, at the
call-site of a function like [`hyper::client::conn::http1::handshake`].

[`TokioIo<I>`] switches across these ecosystems, but notably does not
preserve the existing IO trait implementations of its underlying IO. If
one wishes to _extend_ IO with additional implementations,
[`WithHyperIo<I>`] and [`WithTokioIo<I>`] are the correct choice.

For example, a Tokio reader/writer can be wrapped in [`WithHyperIo<I>`].
That will implement _both_ sets of IO traits. Conversely,
[`WithTokioIo<I>`] will implement both sets of IO traits given a
reader/writer that implements Hyper's [`Read`] and [`Write`].

See [`tokio::io`] and ["_Asynchronous IO_"][tokio-async-docs] for more
information.

[`AsyncRead`]: tokio::io::AsyncRead
[`AsyncWrite`]: tokio::io::AsyncWrite
[`Read`]: hyper::rt::Read
[`Write`]: hyper::rt::Write
[tokio-async-docs]: https://docs.rs/tokio/latest/tokio/#asynchronous-io

```rust
pub mod tokio { /* ... */ }
```

### Types

#### Struct `TokioExecutor`

**Attributes:**

- `#[non_exhaustive]`

Future executor that utilises `tokio` threads.

```rust
pub struct TokioExecutor {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create new executor that relies on [`tokio::spawn`] to execute futures.

###### Trait Implementations

- **Http2ClientConnExec**
  - ```rust
    fn execute_h2_future(self: &mut Self, future: H2ClientFuture<B, T>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Instrument**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> TokioExecutor { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **WithSubscriber**
- **Executor**
  - ```rust
    fn execute(self: &Self, fut: Fut) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TokioExecutor { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `TokioIo`

A wrapper that implements Tokio's IO traits for an inner type that
implements hyper's IO traits, or vice versa (implements hyper's IO
traits for a type that implements Tokio's IO traits).

```rust
pub struct TokioIo<T> {
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
  pub fn new(inner: T) -> Self { /* ... */ }
  ```
  Wrap a type implementing Tokio's or hyper's IO traits.

- ```rust
  pub fn inner(self: &Self) -> &T { /* ... */ }
  ```
  Borrow the inner type.

- ```rust
  pub fn inner_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Mut borrow the inner type.

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Consume this wrapper and get the inner type.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Connection**
  - ```rust
    fn connected(self: &Self) -> Connected { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Read**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: hyper::rt::ReadBufCursor<''_>) -> Poll<Result<(), std::io::Error>> { /* ... */ }
    ```

- **UnwindSafe**
- **AsyncRead**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, tbuf: &mut tokio::io::ReadBuf<''_>) -> Poll<Result<(), std::io::Error>> { /* ... */ }
    ```

- **Send**
- **AsyncReadExt**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsyncWrite**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<Result<usize, std::io::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), std::io::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), std::io::Error>> { /* ... */ }
    ```

  - ```rust
    fn is_write_vectored(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<''_>, bufs: &[std::io::IoSlice<''_>]) -> Poll<Result<usize, std::io::Error>> { /* ... */ }
    ```

- **Sync**
- **WithSubscriber**
- **AsyncWriteExt**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Write**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<Result<usize, std::io::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), std::io::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), std::io::Error>> { /* ... */ }
    ```

  - ```rust
    fn is_write_vectored(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<''_>, bufs: &[std::io::IoSlice<''_>]) -> Poll<Result<usize, std::io::Error>> { /* ... */ }
    ```

#### Struct `TokioTimer`

**Attributes:**

- `#[non_exhaustive]`

A Timer that uses the tokio runtime.

```rust
pub struct TokioTimer;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new TokioTimer

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **WithSubscriber**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> TokioTimer { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Timer**
  - ```rust
    fn sleep(self: &Self, duration: Duration) -> Pin<Box<dyn Sleep>> { /* ... */ }
    ```

  - ```rust
    fn sleep_until(self: &Self, deadline: Instant) -> Pin<Box<dyn Sleep>> { /* ... */ }
    ```

  - ```rust
    fn reset(self: &Self, sleep: &mut Pin<Box<dyn Sleep>>, new_deadline: Instant) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TokioTimer { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Instrument**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Re-exports

#### Re-export `WithHyperIo`

```rust
pub use self::with_hyper_io::WithHyperIo;
```

#### Re-export `WithTokioIo`

```rust
pub use self::with_tokio_io::WithTokioIo;
```

### Re-exports

#### Re-export `TokioExecutor`

**Attributes:**

- `#[<cfg>(feature = "tokio")]`

```rust
pub use self::tokio::TokioExecutor;
```

#### Re-export `TokioIo`

**Attributes:**

- `#[<cfg>(feature = "tokio")]`

```rust
pub use self::tokio::TokioIo;
```

#### Re-export `TokioTimer`

**Attributes:**

- `#[<cfg>(feature = "tokio")]`

```rust
pub use self::tokio::TokioTimer;
```

## Module `service`

**Attributes:**

- `#[<cfg>(any(feature = "service", feature = "client-legacy"))]`

Service utilities.

```rust
pub mod service { /* ... */ }
```

