# Crate Documentation

**Version:** 1.6.0

**Format Version:** 43

# Module `hyper`

# hyper

hyper is a **fast** and **correct** HTTP implementation written in and for Rust.

## Features

- HTTP/1 and HTTP/2
- Asynchronous design
- Leading in performance
- Tested and **correct**
- Extensive production use
- [Client](client/index.html) and [Server](server/index.html) APIs

If just starting out, **check out the [Guides](https://hyper.rs/guides/1/)
first.**

## "Low-level"

hyper is a lower-level HTTP library, meant to be a building block
for libraries and applications.

If looking for just a convenient HTTP client, consider the
[reqwest](https://crates.io/crates/reqwest) crate.

# Optional Features

hyper uses a set of [feature flags] to reduce the amount of compiled code.
It is possible to just enable certain features over others. By default,
hyper does not enable any features but allows one to enable a subset for
their use case. Below is a list of the available feature flags. You may
also notice above each function, struct and trait there is listed one or
more feature flags that are required for that item to be used.

If you are new to hyper it is possible to enable the `full` feature flag
which will enable all public APIs. Beware though that this will pull in
many extra dependencies that you may not need.

The following optional features are available:

- `http1`: Enables HTTP/1 support.
- `http2`: Enables HTTP/2 support.
- `client`: Enables the HTTP `client`.
- `server`: Enables the HTTP `server`.

[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

## Unstable Features

hyper includes a set of unstable optional features that can be enabled through the use of a
feature flag and a [configuration flag].

The following is a list of feature flags and their corresponding `RUSTFLAG`:

- `ffi`: Enables C API for hyper `hyper_unstable_ffi`.
- `tracing`: Enables debug logging with `hyper_unstable_tracing`.

For example:

```notrust
RUSTFLAGS="--cfg hyper_unstable_tracing" cargo build
```

[configuration flag]: https://doc.rust-lang.org/reference/conditional-compilation.html

# Stability

It's worth talking a bit about the stability of hyper. hyper's API follows
[SemVer](https://semver.org). Breaking changes will only be introduced in
major versions, if ever. New additions to the API, such as new types,
methods, or traits will only be added in minor versions.

Some parts of hyper are documented as NOT being part of the stable API. The
following is a brief list, you can read more about each one in the relevant
part of the documentation.

- Downcasting error types from `Error::source()` is not considered stable.
- Private dependencies use of global variables is not considered stable.
  So, if a dependency uses `log` or `tracing`, hyper doesn't promise it
  will continue to do so.
- Behavior from default options is not stable. hyper reserves the right to
  add new options that are enabled by default which might alter the
  behavior, for the purposes of protection. It is also possible to _change_
  what the default options are set to, also in efforts to protect the
  most people possible.

## Modules

## Module `body`

Streaming bodies for Requests and Responses

For both [Clients](crate::client) and [Servers](crate::server), requests and
responses use streaming bodies, instead of complete buffering. This
allows applications to not use memory they don't need, and allows exerting
back-pressure on connections by only reading when asked.

There are two pieces to this in hyper:

- **The [`Body`] trait** describes all possible bodies.
  hyper allows any body type that implements `Body`, allowing
  applications to have fine-grained control over their streaming.
- **The [`Incoming`] concrete type**, which is an implementation
  of `Body`, and returned by hyper as a "receive stream" (so, for server
  requests and client responses).

There are additional implementations available in [`http-body-util`][],
such as a `Full` or `Empty` body.

[`http-body-util`]: https://docs.rs/http-body-util

```rust
pub mod body { /* ... */ }
```

### Re-exports

#### Re-export `Buf`

```rust
pub use bytes::Buf;
```

#### Re-export `Bytes`

```rust
pub use bytes::Bytes;
```

#### Re-export `Body`

```rust
pub use http_body::Body;
```

#### Re-export `Frame`

```rust
pub use http_body::Frame;
```

#### Re-export `SizeHint`

```rust
pub use http_body::SizeHint;
```

#### Re-export `Incoming`

```rust
pub use self::incoming::Incoming;
```

## Module `ext`

HTTP extensions.

```rust
pub mod ext { /* ... */ }
```

### Types

#### Struct `Protocol`

**Attributes:**

- `#[<cfg>(feature = "http2")]`

Represents the `:protocol` pseudo-header used by
the [Extended CONNECT Protocol].

[Extended CONNECT Protocol]: https://datatracker.ietf.org/doc/html/rfc8441#section-4

```rust
pub struct Protocol {
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
  pub const fn from_static(value: &''static str) -> Self { /* ... */ }
  ```
  Converts a static string to a protocol name.

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Returns a str representation of the header.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Protocol { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Protocol) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: &''a str) -> Self { /* ... */ }
    ```

- **Instrument**
- **WithSubscriber**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
### Re-exports

#### Re-export `ReasonPhrase`

**Attributes:**

- `#[<cfg>(any(feature = "http1", feature = "ffi"))]`

```rust
pub use h1_reason_phrase::ReasonPhrase;
```

#### Re-export `on_informational`

**Attributes:**

- `#[<cfg>(all(feature = "http1", feature = "client"))]`

```rust
pub use informational::on_informational;
```

## Module `rt`

Runtime components

The traits and types within this module are used to allow plugging in
runtime types. These include:

- Executors
- Timers
- IO transports

```rust
pub mod rt { /* ... */ }
```

### Modules

## Module `bounds`

Trait aliases

Traits in this module ease setting bounds and usually automatically
implemented by implementing another trait.

```rust
pub mod bounds { /* ... */ }
```

### Re-exports

#### Re-export `Http2ClientConnExec`

**Attributes:**

- `#[<cfg>(all(feature = "client", feature = "http2"))]`

```rust
pub use self::h2_client::Http2ClientConnExec;
```

### Traits

#### Trait `Executor`

An executor of futures.

This trait allows Hyper to abstract over async runtimes. Implement this trait for your own type.

# Example

```
# use hyper::rt::Executor;
# use std::future::Future;
#[derive(Clone)]
struct TokioExecutor;

impl<F> Executor<F> for TokioExecutor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, future: F) {
        tokio::spawn(future);
    }
}
```

```rust
pub trait Executor<Fut> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `execute`: Place the future into the executor to be run.

### Re-exports

#### Re-export `Read`

```rust
pub use self::io::Read;
```

#### Re-export `ReadBuf`

```rust
pub use self::io::ReadBuf;
```

#### Re-export `ReadBufCursor`

```rust
pub use self::io::ReadBufCursor;
```

#### Re-export `Write`

```rust
pub use self::io::Write;
```

#### Re-export `Sleep`

```rust
pub use self::timer::Sleep;
```

#### Re-export `Timer`

```rust
pub use self::timer::Timer;
```

## Module `service`

Asynchronous Services

A [`Service`] is a trait representing an asynchronous
function of a request to a response. It's similar to
`async fn(Request) -> Result<Response, Error>`.

The argument and return value isn't strictly required to be for HTTP.
Therefore, hyper uses several "trait aliases" to reduce clutter around
bounds. These are:

- `HttpService`: This is blanketly implemented for all types that
  implement `Service<http::Request<B1>, Response = http::Response<B2>>`.

# HttpService

In hyper, especially in the server setting, a `Service` is usually bound
to a single connection. It defines how to respond to **all** requests that
connection will receive.

The helper [`service_fn`] should be sufficient for most cases, but
if you need to implement `Service` for a type manually, you can follow the example
in `service_struct_impl.rs`.

```rust
pub mod service { /* ... */ }
```

### Re-exports

#### Re-export `HttpService`

```rust
pub use self::http::HttpService;
```

#### Re-export `Service`

```rust
pub use self::service::Service;
```

#### Re-export `service_fn`

```rust
pub use self::util::service_fn;
```

## Module `upgrade`

HTTP Upgrades

This module deals with managing [HTTP Upgrades][mdn] in hyper. Since
several concepts in HTTP allow for first talking HTTP, and then converting
to a different protocol, this module conflates them into a single API.
Those include:

- HTTP/1.1 Upgrades
- HTTP `CONNECT`

You are responsible for any other pre-requisites to establish an upgrade,
such as sending the appropriate headers, methods, and status codes. You can
then use [`on`][] to grab a `Future` which will resolve to the upgraded
connection object, or an error if the upgrade fails.

[mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Protocol_upgrade_mechanism

# Client

Sending an HTTP upgrade from the [`client`](super::client) involves setting
either the appropriate method, if wanting to `CONNECT`, or headers such as
`Upgrade` and `Connection`, on the `http::Request`. Once receiving the
`http::Response` back, you must check for the specific information that the
upgrade is agreed upon by the server (such as a `101` status code), and then
get the `Future` from the `Response`.

# Server

Receiving upgrade requests in a server requires you to check the relevant
headers in a `Request`, and if an upgrade should be done, you then send the
corresponding headers in a response. To then wait for hyper to finish the
upgrade, you call `on()` with the `Request`, and then can spawn a task
awaiting it.

# Example

See [this example][example] showing how upgrades work with both
Clients and Servers.

[example]: https://github.com/hyperium/hyper/blob/master/examples/upgrades.rs

```rust
pub mod upgrade { /* ... */ }
```

### Types

#### Struct `Upgraded`

An upgraded HTTP connection.

This type holds a trait object internally of the original IO that
was used to speak HTTP before the upgrade. It can be used directly
as a [`Read`] or [`Write`] for convenience.

Alternatively, if the exact type is known, this can be deconstructed
into its parts.

```rust
pub struct Upgraded {
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
  pub fn downcast<T: Read + Write + Unpin + ''static>(self: Self) -> Result<Parts<T>, Self> { /* ... */ }
  ```
  Tries to downcast the internal trait object to the type passed.

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Write**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<io::Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<''_>, bufs: &[io::IoSlice<''_>]) -> Poll<io::Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn is_write_vectored(self: &Self) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Read**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: ReadBufCursor<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Instrument**
- **RefUnwindSafe**
#### Struct `OnUpgrade`

A future for a possible HTTP upgrade.

If no upgrade was available, or it doesn't succeed, yields an `Error`.

```rust
pub struct OnUpgrade {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryFutureExt**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Instrument**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **FutureExt**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> OnUpgrade { /* ... */ }
    ```

#### Struct `Parts`

**Attributes:**

- `#[non_exhaustive]`

The deconstructed parts of an [`Upgraded`] type.

Includes the original IO type, and a read buffer of bytes that the
HTTP state machine may have already read before completing an upgrade.

```rust
pub struct Parts<T> {
    pub io: T,
    pub read_buf: bytes::Bytes,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `io` | `T` | The original IO object used before the upgrade. |
| `read_buf` | `bytes::Bytes` | A buffer of bytes that have been read but not processed as HTTP.<br><br>For instance, if the `Connection` is used for an HTTP upgrade request,<br>it is possible the server sent back the first bytes of the new protocol<br>along with the response upgrade.<br><br>You will want to check for any existing bytes if you plan to continue<br>communicating on the IO object. |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Instrument**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Functions

#### Function `on`

Gets a pending HTTP upgrade from this message.

This can be called on the following types:

- `http::Request<B>`
- `http::Response<B>`
- `&mut http::Request<B>`
- `&mut http::Response<B>`

```rust
pub fn on<T: sealed::CanUpgrade>(msg: T) -> OnUpgrade { /* ... */ }
```

## Module `client`

**Attributes:**

- `#[<cfg>(feature = "client")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "client")))]`

HTTP Client

hyper provides HTTP over a single connection. See the [`conn`] module.

## Examples

* [`client`] - A simple CLI http client that requests the url passed in parameters and outputs the response content and details to the stdout, reading content chunk-by-chunk.

* [`client_json`] - A simple program that GETs some json, reads the body asynchronously, parses it with serde and outputs the result.

[`client`]: https://github.com/hyperium/hyper/blob/master/examples/client.rs
[`client_json`]: https://github.com/hyperium/hyper/blob/master/examples/client_json.rs

```rust
pub mod client { /* ... */ }
```

### Modules

## Module `conn`

**Attributes:**

- `#[<cfg>(any(feature = "http1", feature = "http2"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]`

Lower-level client connection API.

The types in this module are to provide a lower-level API based around a
single connection. Connecting to a host, pooling connections, and the like
are not handled at this level. This module provides the building blocks to
customize those things externally.

If you are looking for a convenient HTTP client, then you may wish to
consider [reqwest](https://github.com/seanmonstar/reqwest) for a high level
client or [`hyper-util`'s client](https://docs.rs/hyper-util/latest/hyper_util/client/index.html)
if you want to keep it more low level / basic.

## Example

See the [client guide](https://hyper.rs/guides/1/client/basic/).

```rust
pub mod conn { /* ... */ }
```

### Modules

## Module `http1`

**Attributes:**

- `#[<cfg>(feature = "http1")]`

HTTP/1 client connections

```rust
pub mod http1 { /* ... */ }
```

### Types

#### Struct `SendRequest`

The sender side of an established connection.

```rust
pub struct SendRequest<B> {
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
  pub fn poll_ready(self: &mut Self, cx: &mut Context<''_>) -> Poll<crate::Result<()>> { /* ... */ }
  ```
  Polls to determine whether this sender can be used yet for a request.

- ```rust
  pub async fn ready(self: &mut Self) -> crate::Result<()> { /* ... */ }
  ```
  Waits until the dispatcher is ready

- ```rust
  pub fn is_ready(self: &Self) -> bool { /* ... */ }
  ```
  Checks if the connection is currently ready to send a request.

- ```rust
  pub fn is_closed(self: &Self) -> bool { /* ... */ }
  ```
  Checks if the connection side has been closed.

- ```rust
  pub fn send_request(self: &mut Self, req: Request<B>) -> impl Future<Output = crate::Result<Response<IncomingBody>>> { /* ... */ }
  ```
  Sends a `Request` on the associated connection.

- ```rust
  pub fn try_send_request(self: &mut Self, req: Request<B>) -> impl Future<Output = Result<Response<IncomingBody>, TrySendError<Request<B>>>> { /* ... */ }
  ```
  Sends a `Request` on the associated connection.

###### Trait Implementations

- **Instrument**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Parts`

**Attributes:**

- `#[non_exhaustive]`

Deconstructed parts of a `Connection`.

This allows taking apart a `Connection` at a later time, in order to
reclaim the IO object, and additional related pieces.

```rust
pub struct Parts<T> {
    pub io: T,
    pub read_buf: bytes::Bytes,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `io` | `T` | The original IO object used in the handshake. |
| `read_buf` | `bytes::Bytes` | A buffer of bytes that have been read but not processed as HTTP.<br><br>For instance, if the `Connection` is used for an HTTP upgrade request,<br>it is possible the server sent back the first bytes of the new protocol<br>along with the response upgrade.<br><br>You will want to check for any existing bytes if you plan to continue<br>communicating on the IO object. |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **WithSubscriber**
- **RefUnwindSafe**
- **Send**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Instrument**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Connection`

**Attributes:**

- `#[must_use = "futures do nothing unless polled"]`

A future that processes all HTTP state for the IO object.

In most cases, this should just be spawned into an executor, so that it
can process incoming and outgoing messages, notice hangups, and the like.

Instances of this type are typically created via the [`handshake`] function

```rust
pub struct Connection<T, B> {
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
  pub fn into_parts(self: Self) -> Parts<T> { /* ... */ }
  ```
  Return the inner IO object, and additional information.

- ```rust
  pub fn poll_without_shutdown(self: &mut Self, cx: &mut Context<''_>) -> Poll<crate::Result<()>> { /* ... */ }
  ```
  Poll the connection for completion, but without calling `shutdown`

- ```rust
  pub async fn without_shutdown(self: Self) -> crate::Result<Parts<T>> { /* ... */ }
  ```
  Prevent shutdown of the underlying IO object at the end of service the request,

- ```rust
  pub fn with_upgrades(self: Self) -> upgrades::UpgradeableConnection<T, B> { /* ... */ }
  ```
  Enable this connection to support higher-level HTTP upgrades.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Instrument**
- **FutureExt**
- **Unpin**
- **RefUnwindSafe**
- **TryFutureExt**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **WithSubscriber**
- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Builder`

A builder to configure an HTTP connection.

After setting options, the builder is used to create a handshake future.

**Note**: The default values of options are *not considered stable*. They
are subject to change at any time.

```rust
pub struct Builder {
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
  pub fn new() -> Builder { /* ... */ }
  ```
  Creates a new connection builder.

- ```rust
  pub fn http09_responses(self: &mut Self, enabled: bool) -> &mut Builder { /* ... */ }
  ```
  Set whether HTTP/0.9 responses should be tolerated.

- ```rust
  pub fn allow_spaces_after_header_name_in_responses(self: &mut Self, enabled: bool) -> &mut Builder { /* ... */ }
  ```
  Set whether HTTP/1 connections will accept spaces between header names

- ```rust
  pub fn allow_obsolete_multiline_headers_in_responses(self: &mut Self, enabled: bool) -> &mut Builder { /* ... */ }
  ```
  Set whether HTTP/1 connections will accept obsolete line folding for

- ```rust
  pub fn ignore_invalid_headers_in_responses(self: &mut Self, enabled: bool) -> &mut Builder { /* ... */ }
  ```
  Set whether HTTP/1 connections will silently ignored malformed header lines.

- ```rust
  pub fn writev(self: &mut Self, enabled: bool) -> &mut Builder { /* ... */ }
  ```
  Set whether HTTP/1 connections should try to use vectored writes,

- ```rust
  pub fn title_case_headers(self: &mut Self, enabled: bool) -> &mut Builder { /* ... */ }
  ```
  Set whether HTTP/1 connections will write header names as title case at

- ```rust
  pub fn preserve_header_case(self: &mut Self, enabled: bool) -> &mut Builder { /* ... */ }
  ```
  Set whether to support preserving original header cases.

- ```rust
  pub fn max_headers(self: &mut Self, val: usize) -> &mut Self { /* ... */ }
  ```
  Set the maximum number of headers.

- ```rust
  pub fn read_buf_exact_size(self: &mut Self, sz: Option<usize>) -> &mut Builder { /* ... */ }
  ```
  Sets the exact size of the read buffer to *always* use.

- ```rust
  pub fn max_buf_size(self: &mut Self, max: usize) -> &mut Self { /* ... */ }
  ```
  Set the maximum buffer size for the connection.

- ```rust
  pub fn handshake<T, B>(self: &Self, io: T) -> impl Future<Output = crate::Result<(SendRequest<B>, Connection<T, B>)>>
where
    T: Read + Write + Unpin,
    B: Body + ''static,
    <B as >::Data: Send,
    <B as >::Error: Into<Box<dyn StdError + Send + Sync>> { /* ... */ }
  ```
  Constructs a connection with the configured options and IO.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Instrument**
- **Send**
- **Sync**
- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **WithSubscriber**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `handshake`

Returns a handshake future over some IO.

This is a shortcut for `Builder::new().handshake(io)`.
See [`client::conn`](crate::client::conn) for more.

```rust
pub async fn handshake<T, B>(io: T) -> crate::Result<(SendRequest<B>, Connection<T, B>)>
where
    T: Read + Write + Unpin,
    B: Body + ''static,
    <B as >::Data: Send,
    <B as >::Error: Into<Box<dyn StdError + Send + Sync>> { /* ... */ }
```

## Module `http2`

**Attributes:**

- `#[<cfg>(feature = "http2")]`

HTTP/2 client connections

```rust
pub mod http2 { /* ... */ }
```

### Types

#### Struct `SendRequest`

The sender side of an established connection.

```rust
pub struct SendRequest<B> {
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
  pub fn poll_ready(self: &mut Self, _cx: &mut Context<''_>) -> Poll<crate::Result<()>> { /* ... */ }
  ```
  Polls to determine whether this sender can be used yet for a request.

- ```rust
  pub async fn ready(self: &mut Self) -> crate::Result<()> { /* ... */ }
  ```
  Waits until the dispatcher is ready

- ```rust
  pub fn is_ready(self: &Self) -> bool { /* ... */ }
  ```
  Checks if the connection is currently ready to send a request.

- ```rust
  pub fn is_closed(self: &Self) -> bool { /* ... */ }
  ```
  Checks if the connection side has been closed.

- ```rust
  pub fn send_request(self: &mut Self, req: Request<B>) -> impl Future<Output = crate::Result<Response<IncomingBody>>> { /* ... */ }
  ```
  Sends a `Request` on the associated connection.

- ```rust
  pub fn try_send_request(self: &mut Self, req: Request<B>) -> impl Future<Output = Result<Response<IncomingBody>, TrySendError<Request<B>>>> { /* ... */ }
  ```
  Sends a `Request` on the associated connection.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Instrument**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SendRequest<B> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Connection`

**Attributes:**

- `#[must_use = "futures do nothing unless polled"]`

A future that processes all HTTP state for the IO object.

In most cases, this should just be spawned into an executor, so that it
can process incoming and outgoing messages, notice hangups, and the like.

Instances of this type are typically created via the [`handshake`] function

```rust
pub struct Connection<T, B, E> {
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
  pub fn is_extended_connect_protocol_enabled(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the [extended CONNECT protocol][1] is enabled or not.

###### Trait Implementations

- **FutureExt**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **Unpin**
- **UnwindSafe**
- **Freeze**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFutureExt**
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

- **Sync**
- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **Instrument**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

#### Struct `Builder`

A builder to configure an HTTP connection.

After setting options, the builder is used to create a handshake future.

**Note**: The default values of options are *not considered stable*. They
are subject to change at any time.

```rust
pub struct Builder<Ex> {
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
  pub fn new(exec: Ex) -> Builder<Ex> { /* ... */ }
  ```
  Creates a new connection builder.

- ```rust
  pub fn timer<M>(self: &mut Self, timer: M) -> &mut Builder<Ex>
where
    M: Timer + Send + Sync + ''static { /* ... */ }
  ```
  Provide a timer to execute background HTTP2 tasks.

- ```rust
  pub fn initial_stream_window_size</* synthetic */ impl Into<Option<u32>>: Into<Option<u32>>>(self: &mut Self, sz: impl Into<Option<u32>>) -> &mut Self { /* ... */ }
  ```
  Sets the [`SETTINGS_INITIAL_WINDOW_SIZE`][spec] option for HTTP2

- ```rust
  pub fn initial_connection_window_size</* synthetic */ impl Into<Option<u32>>: Into<Option<u32>>>(self: &mut Self, sz: impl Into<Option<u32>>) -> &mut Self { /* ... */ }
  ```
  Sets the max connection-level flow control for HTTP2

- ```rust
  pub fn initial_max_send_streams</* synthetic */ impl Into<Option<usize>>: Into<Option<usize>>>(self: &mut Self, initial: impl Into<Option<usize>>) -> &mut Self { /* ... */ }
  ```
  Sets the initial maximum of locally initiated (send) streams.

- ```rust
  pub fn adaptive_window(self: &mut Self, enabled: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether to use an adaptive flow control.

- ```rust
  pub fn max_frame_size</* synthetic */ impl Into<Option<u32>>: Into<Option<u32>>>(self: &mut Self, sz: impl Into<Option<u32>>) -> &mut Self { /* ... */ }
  ```
  Sets the maximum frame size to use for HTTP2.

- ```rust
  pub fn max_header_list_size(self: &mut Self, max: u32) -> &mut Self { /* ... */ }
  ```
  Sets the max size of received header frames.

- ```rust
  pub fn header_table_size</* synthetic */ impl Into<Option<u32>>: Into<Option<u32>>>(self: &mut Self, size: impl Into<Option<u32>>) -> &mut Self { /* ... */ }
  ```
  Sets the header table size.

- ```rust
  pub fn max_concurrent_streams</* synthetic */ impl Into<Option<u32>>: Into<Option<u32>>>(self: &mut Self, max: impl Into<Option<u32>>) -> &mut Self { /* ... */ }
  ```
  Sets the maximum number of concurrent streams.

- ```rust
  pub fn keep_alive_interval</* synthetic */ impl Into<Option<Duration>>: Into<Option<Duration>>>(self: &mut Self, interval: impl Into<Option<Duration>>) -> &mut Self { /* ... */ }
  ```
  Sets an interval for HTTP2 Ping frames should be sent to keep a

- ```rust
  pub fn keep_alive_timeout(self: &mut Self, timeout: Duration) -> &mut Self { /* ... */ }
  ```
  Sets a timeout for receiving an acknowledgement of the keep-alive ping.

- ```rust
  pub fn keep_alive_while_idle(self: &mut Self, enabled: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether HTTP2 keep-alive should apply while the connection is idle.

- ```rust
  pub fn max_concurrent_reset_streams(self: &mut Self, max: usize) -> &mut Self { /* ... */ }
  ```
  Sets the maximum number of HTTP2 concurrent locally reset streams.

- ```rust
  pub fn max_send_buf_size(self: &mut Self, max: usize) -> &mut Self { /* ... */ }
  ```
  Set the maximum write buffer size for each HTTP/2 stream.

- ```rust
  pub fn max_pending_accept_reset_streams</* synthetic */ impl Into<Option<usize>>: Into<Option<usize>>>(self: &mut Self, max: impl Into<Option<usize>>) -> &mut Self { /* ... */ }
  ```
  Configures the maximum number of pending reset streams allowed before a GOAWAY will be sent.

- ```rust
  pub fn handshake<T, B>(self: &Self, io: T) -> impl Future<Output = crate::Result<(SendRequest<B>, Connection<T, B, Ex>)>>
where
    T: Read + Write + Unpin,
    B: Body + ''static,
    <B as >::Data: Send,
    <B as >::Error: Into<Box<dyn Error + Send + Sync>>,
    Ex: Http2ClientConnExec<B, T> + Unpin { /* ... */ }
  ```
  Constructs a connection with the configured options and IO.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Instrument**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **WithSubscriber**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder<Ex> { /* ... */ }
    ```

- **RefUnwindSafe**
### Functions

#### Function `handshake`

Returns a handshake future over some IO.

This is a shortcut for `Builder::new(exec).handshake(io)`.
See [`client::conn`](crate::client::conn) for more.

```rust
pub async fn handshake<E, T, B>(exec: E, io: T) -> crate::Result<(SendRequest<B>, Connection<T, B, E>)>
where
    T: Read + Write + Unpin,
    B: Body + ''static,
    <B as >::Data: Send,
    <B as >::Error: Into<Box<dyn Error + Send + Sync>>,
    E: Http2ClientConnExec<B, T> + Unpin + Clone { /* ... */ }
```

### Re-exports

#### Re-export `TrySendError`

```rust
pub use super::dispatch::TrySendError;
```

## Re-exports

### Re-export `header`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use http::header;
```

### Re-export `HeaderMap`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use http::HeaderMap;
```

### Re-export `Method`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use http::Method;
```

### Re-export `Request`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use http::Request;
```

### Re-export `Response`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use http::Response;
```

### Re-export `StatusCode`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use http::StatusCode;
```

### Re-export `Uri`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use http::Uri;
```

### Re-export `Version`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use http::Version;
```

### Re-export `Error`

```rust
pub use crate::error::Error;
```

### Re-export `Result`

```rust
pub use crate::error::Result;
```

