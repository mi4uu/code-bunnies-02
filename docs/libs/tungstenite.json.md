# Crate Documentation

**Version:** 0.24.0

**Format Version:** 43

# Module `tungstenite`

Lightweight, flexible WebSockets for Rust.

## Modules

## Module `buffer`

A buffer for reading data from the network.

The `ReadBuffer` is a buffer of bytes similar to a first-in, first-out queue.
It is filled by reading from a stream supporting `Read` and is then
accessible as a cursor for reading bytes.

```rust
pub mod buffer { /* ... */ }
```

### Types

#### Struct `ReadBuffer`

A FIFO buffer for reading packets from the network.

```rust
pub struct ReadBuffer<const CHUNK_SIZE: usize> {
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
  Create a new empty input buffer.

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```
  Create a new empty input buffer with a given `capacity`.

- ```rust
  pub fn from_partially_read(part: Vec<u8>) -> Self { /* ... */ }
  ```
  Create a input buffer filled with previously read data.

- ```rust
  pub fn as_cursor(self: &Self) -> &Cursor<Vec<u8>> { /* ... */ }
  ```
  Get a cursor to the data storage.

- ```rust
  pub fn as_cursor_mut(self: &mut Self) -> &mut Cursor<Vec<u8>> { /* ... */ }
  ```
  Get a cursor to the mutable data storage.

- ```rust
  pub fn into_vec(self: Self) -> Vec<u8> { /* ... */ }
  ```
  Consume the `ReadBuffer` and get the internal storage.

- ```rust
  pub fn read_from<S: Read>(self: &mut Self, stream: &mut S) -> IoResult<usize> { /* ... */ }
  ```
  Read next portion of data from the given input stream.

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Buf**
  - ```rust
    fn remaining(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn chunk(self: &Self) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn advance(self: &mut Self, cnt: usize) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Same**
- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
## Module `client`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

Methods to connect to a WebSocket as a client.

```rust
pub mod client { /* ... */ }
```

### Types

#### Struct `ClientRequestBuilder`

Builder for a custom [`IntoClientRequest`] with options to add
custom additional headers and sub protocols.

# Example

```rust no_run
# use crate::*;
use http::Uri;
use tungstenite::{connect, ClientRequestBuilder};

let uri: Uri = "ws://localhost:3012/socket".parse().unwrap();
let token = "my_jwt_token";
let builder = ClientRequestBuilder::new(uri)
    .with_header("Authorization", format!("Bearer {token}"))
    .with_sub_protocol("my_sub_protocol");
let socket = connect(builder).unwrap();
```

```rust
pub struct ClientRequestBuilder {
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
  pub const fn new(uri: Uri) -> Self { /* ... */ }
  ```
  Initializes an empty request builder

- ```rust
  pub fn with_header<K, V>(self: Self, key: K, value: V) -> Self
where
    K: Into<String>,
    V: Into<String> { /* ... */ }
  ```
  Adds (`key`, `value`) as an additional header to the handshake request

- ```rust
  pub fn with_sub_protocol<P>(self: Self, protocol: P) -> Self
where
    P: Into<String> { /* ... */ }
  ```
  Adds `protocol` to the handshake request subprotocols (`Sec-WebSocket-Protocol`)

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **IntoClientRequest**
  - ```rust
    fn into_client_request(self: Self) -> Result<Request> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClientRequestBuilder { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Same**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Traits

#### Trait `IntoClientRequest`

Trait for converting various types into HTTP requests used for a client connection.

This trait is implemented by default for string slices, strings, `http::Uri` and
`http::Request<()>`. Note that the implementation for `http::Request<()>` is trivial and will
simply take your request and pass it as is further without altering any headers or URLs, so
be aware of this. If you just want to connect to the endpoint with a certain URL, better pass
a regular string containing the URL in which case `tungstenite-rs` will take care for generating
the proper `http::Request<()>` for you.

```rust
pub trait IntoClientRequest {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `into_client_request`: Convert into a `Request` that can be used for a client connection.

##### Implementations

This trait is implemented for the following types:

- `&''a str` with <''a>
- `&''a String` with <''a>
- `String`
- `&''a http::Uri` with <''a>
- `http::Uri`
- `crate::handshake::client::Request`
- `httparse::Request<''h, ''b>` with <''h, ''b>
- `ClientRequestBuilder`

### Functions

#### Function `connect_with_config`

Connect to the given WebSocket in blocking mode.

Uses a websocket configuration passed as an argument to the function. Calling it with `None` is
equal to calling `connect()` function.

The URL may be either ws:// or wss://.
To support wss:// URLs, you must activate the TLS feature on the crate level. Please refer to the
project's [README][readme] for more information on available features.

This function "just works" for those who wants a simple blocking solution
similar to `std::net::TcpStream`. If you want a non-blocking or other
custom stream, call `client` instead.

This function uses `native_tls` or `rustls` to do TLS depending on the feature flags enabled. If
you want to use other TLS libraries, use `client` instead. There is no need to enable any of
the `*-tls` features if you don't call `connect` since it's the only function that uses them.

[readme]: https://github.com/snapview/tungstenite-rs/#features

```rust
pub fn connect_with_config<Req: IntoClientRequest>(request: Req, config: Option<crate::protocol::WebSocketConfig>, max_redirects: u8) -> crate::error::Result<(crate::protocol::WebSocket<crate::stream::MaybeTlsStream<std::net::TcpStream>>, crate::handshake::client::Response)> { /* ... */ }
```

#### Function `connect`

Connect to the given WebSocket in blocking mode.

The URL may be either ws:// or wss://.
To support wss:// URLs, feature `native-tls` or `rustls-tls` must be turned on.

This function "just works" for those who wants a simple blocking solution
similar to `std::net::TcpStream`. If you want a non-blocking or other
custom stream, call `client` instead.

This function uses `native_tls` or `rustls` to do TLS depending on the feature flags enabled. If
you want to use other TLS libraries, use `client` instead. There is no need to enable any of
the `*-tls` features if you don't call `connect` since it's the only function that uses them.

```rust
pub fn connect<Req: IntoClientRequest>(request: Req) -> crate::error::Result<(crate::protocol::WebSocket<crate::stream::MaybeTlsStream<std::net::TcpStream>>, crate::handshake::client::Response)> { /* ... */ }
```

#### Function `uri_mode`

Get the mode of the given URL.

This function may be used to ease the creation of custom TLS streams
in non-blocking algorithms or for use with TLS libraries other than `native_tls` or `rustls`.

```rust
pub fn uri_mode(uri: &http::Uri) -> crate::error::Result<crate::stream::Mode> { /* ... */ }
```

#### Function `client_with_config`

Do the client handshake over the given stream given a web socket configuration. Passing `None`
as configuration is equal to calling `client()` function.

Use this function if you need a nonblocking handshake support or if you
want to use a custom stream like `mio::net::TcpStream` or `openssl::ssl::SslStream`.
Any stream supporting `Read + Write` will do.

```rust
pub fn client_with_config<Stream, Req>(request: Req, stream: Stream, config: Option<crate::protocol::WebSocketConfig>) -> std::result::Result<(crate::protocol::WebSocket<Stream>, crate::handshake::client::Response), crate::handshake::HandshakeError<crate::handshake::client::ClientHandshake<Stream>>>
where
    Stream: Read + Write,
    Req: IntoClientRequest { /* ... */ }
```

#### Function `client`

Do the client handshake over the given stream.

Use this function if you need a nonblocking handshake support or if you
want to use a custom stream like `mio::net::TcpStream` or `openssl::ssl::SslStream`.
Any stream supporting `Read + Write` will do.

```rust
pub fn client<Stream, Req>(request: Req, stream: Stream) -> std::result::Result<(crate::protocol::WebSocket<Stream>, crate::handshake::client::Response), crate::handshake::HandshakeError<crate::handshake::client::ClientHandshake<Stream>>>
where
    Stream: Read + Write,
    Req: IntoClientRequest { /* ... */ }
```

## Module `error`

Error handling.

```rust
pub mod error { /* ... */ }
```

### Types

#### Type Alias `Result`

Result type of all Tungstenite library calls.

```rust
pub type Result<T, E = Error> = result::Result<T, E>;
```

#### Enum `Error`

Possible WebSocket errors.

```rust
pub enum Error {
    ConnectionClosed,
    AlreadyClosed,
    Io(io::Error),
    Tls(TlsError),
    Capacity(CapacityError),
    Protocol(ProtocolError),
    WriteBufferFull(crate::protocol::Message),
    Utf8,
    AttackAttempt,
    Url(UrlError),
    Http(http::Response<Option<Vec<u8>>>),
    HttpFormat(http::Error),
}
```

##### Variants

###### `ConnectionClosed`

WebSocket connection closed normally. This informs you of the close.
It's not an error as such and nothing wrong happened.

This is returned as soon as the close handshake is finished (we have both sent and
received a close frame) on the server end and as soon as the server has closed the
underlying connection if this endpoint is a client.

Thus when you receive this, it is safe to drop the underlying connection.

Receiving this error means that the WebSocket object is not usable anymore and the
only meaningful action with it is dropping it.

###### `AlreadyClosed`

Trying to work with already closed connection.

Trying to read or write after receiving `ConnectionClosed` causes this.

As opposed to `ConnectionClosed`, this indicates your code tries to operate on the
connection when it really shouldn't anymore, so this really indicates a programmer
error on your part.

###### `Io`

Input-output error. Apart from WouldBlock, these are generally errors with the
underlying connection and you should probably consider them fatal.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `io::Error` |  |

###### `Tls`

TLS error.

Note that this error variant is enabled unconditionally even if no TLS feature is enabled,
to provide a feature-agnostic API surface.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TlsError` |  |

###### `Capacity`

- When reading: buffer capacity exhausted.
- When writing: your message is bigger than the configured max message size
  (64MB by default).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CapacityError` |  |

###### `Protocol`

Protocol violation.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ProtocolError` |  |

###### `WriteBufferFull`

Message write buffer is full.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::protocol::Message` |  |

###### `Utf8`

UTF coding error.

###### `AttackAttempt`

Attack attempt detected.

###### `Url`

Invalid URL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `UrlError` |  |

###### `Http`

HTTP error.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `http::Response<Option<Vec<u8>>>` |  |

###### `HttpFormat`

HTTP format error.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `http::Error` |  |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: TlsError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: CapacityError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: ProtocolError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: UrlError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: http::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(_: str::Utf8Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(_: string::FromUtf8Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: http::header::InvalidHeaderValue) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: http::header::InvalidHeaderName) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(_: http::header::ToStrError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: http::uri::InvalidUri) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: http::status::InvalidStatusCode) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: httparse::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: Error) -> Self { /* ... */ }
    ```

- **Same**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **NonBlockingError**
  - ```rust
    fn into_non_blocking(self: Self) -> Option<Self> { /* ... */ }
    ```

- **Send**
#### Enum `CapacityError`

Indicates the specific type/cause of a capacity error.

```rust
pub enum CapacityError {
    TooManyHeaders,
    MessageTooLong {
        size: usize,
        max_size: usize,
    },
}
```

##### Variants

###### `TooManyHeaders`

Too many headers provided (see [`httparse::Error::TooManyHeaders`]).

###### `MessageTooLong`

Received header is too long.
Message is bigger than the maximum allowed size.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `size` | `usize` | The size of the message. |
| `max_size` | `usize` | The maximum allowed message size. |

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: CapacityError) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CapacityError { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CapacityError) -> bool { /* ... */ }
    ```

#### Enum `SubProtocolError`

Indicates the specific type/cause of a subprotocol header error.

```rust
pub enum SubProtocolError {
    ServerSentSubProtocolNoneRequested,
    InvalidSubProtocol,
    NoSubProtocol,
}
```

##### Variants

###### `ServerSentSubProtocolNoneRequested`

The server sent a subprotocol to a client handshake request but none was requested

###### `InvalidSubProtocol`

The server sent an invalid subprotocol to a client handhshake request

###### `NoSubProtocol`

The server sent no subprotocol to a client handshake request that requested one or more
subprotocols

##### Implementations

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SubProtocolError) -> bool { /* ... */ }
    ```

- **Copy**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Error**
- **RefUnwindSafe**
- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SubProtocolError { /* ... */ }
    ```

- **Eq**
- **Sync**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Enum `ProtocolError`

**Attributes:**

- `#[allow(missing_copy_implementations)]`

Indicates the specific type/cause of a protocol error.

```rust
pub enum ProtocolError {
    WrongHttpMethod,
    WrongHttpVersion,
    MissingConnectionUpgradeHeader,
    MissingUpgradeWebSocketHeader,
    MissingSecWebSocketVersionHeader,
    MissingSecWebSocketKey,
    SecWebSocketAcceptKeyMismatch,
    SecWebSocketSubProtocolError(SubProtocolError),
    JunkAfterRequest,
    CustomResponseSuccessful,
    InvalidHeader(http::header::HeaderName),
    HandshakeIncomplete,
    HttparseError(httparse::Error),
    SendAfterClosing,
    ReceivedAfterClosing,
    NonZeroReservedBits,
    UnmaskedFrameFromClient,
    MaskedFrameFromServer,
    FragmentedControlFrame,
    ControlFrameTooBig,
    UnknownControlFrameType(u8),
    UnknownDataFrameType(u8),
    UnexpectedContinueFrame,
    ExpectedFragment(crate::protocol::frame::coding::Data),
    ResetWithoutClosingHandshake,
    InvalidOpcode(u8),
    InvalidCloseSequence,
}
```

##### Variants

###### `WrongHttpMethod`

Use of the wrong HTTP method (the WebSocket protocol requires the GET method be used).

###### `WrongHttpVersion`

Wrong HTTP version used (the WebSocket protocol requires version 1.1 or higher).

###### `MissingConnectionUpgradeHeader`

Missing `Connection: upgrade` HTTP header.

###### `MissingUpgradeWebSocketHeader`

Missing `Upgrade: websocket` HTTP header.

###### `MissingSecWebSocketVersionHeader`

Missing `Sec-WebSocket-Version: 13` HTTP header.

###### `MissingSecWebSocketKey`

Missing `Sec-WebSocket-Key` HTTP header.

###### `SecWebSocketAcceptKeyMismatch`

The `Sec-WebSocket-Accept` header is either not present or does not specify the correct key value.

###### `SecWebSocketSubProtocolError`

The `Sec-WebSocket-Protocol` header was invalid

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SubProtocolError` |  |

###### `JunkAfterRequest`

Garbage data encountered after client request.

###### `CustomResponseSuccessful`

Custom responses must be unsuccessful.

###### `InvalidHeader`

Invalid header is passed. Or the header is missing in the request. Or not present at all. Check the request that you pass.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `http::header::HeaderName` |  |

###### `HandshakeIncomplete`

No more data while still performing handshake.

###### `HttparseError`

Wrapper around a [`httparse::Error`] value.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `httparse::Error` |  |

###### `SendAfterClosing`

Not allowed to send after having sent a closing frame.

###### `ReceivedAfterClosing`

Remote sent data after sending a closing frame.

###### `NonZeroReservedBits`

Reserved bits in frame header are non-zero.

###### `UnmaskedFrameFromClient`

The server must close the connection when an unmasked frame is received.

###### `MaskedFrameFromServer`

The client must close the connection when a masked frame is received.

###### `FragmentedControlFrame`

Control frames must not be fragmented.

###### `ControlFrameTooBig`

Control frames must have a payload of 125 bytes or less.

###### `UnknownControlFrameType`

Type of control frame not recognised.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `UnknownDataFrameType`

Type of data frame not recognised.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `UnexpectedContinueFrame`

Received a continue frame despite there being nothing to continue.

###### `ExpectedFragment`

Received data while waiting for more fragments.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::protocol::frame::coding::Data` |  |

###### `ResetWithoutClosingHandshake`

Connection closed without performing the closing handshake.

###### `InvalidOpcode`

Encountered an invalid opcode.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `InvalidCloseSequence`

The payload for the closing frame is invalid.

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: ProtocolError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: httparse::Error) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Same**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProtocolError { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ProtocolError) -> bool { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `UrlError`

Indicates the specific type/cause of URL error.

```rust
pub enum UrlError {
    TlsFeatureNotEnabled,
    NoHostName,
    UnableToConnect(String),
    UnsupportedUrlScheme,
    EmptyHostName,
    NoPathOrQuery,
}
```

##### Variants

###### `TlsFeatureNotEnabled`

TLS is used despite not being compiled with the TLS feature enabled.

###### `NoHostName`

The URL does not include a host name.

###### `UnableToConnect`

Failed to connect with this URL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `UnsupportedUrlScheme`

Unsupported URL scheme used (only `ws://` or `wss://` may be used).

###### `EmptyHostName`

The URL host name, though included, is empty.

###### `NoPathOrQuery`

The URL does not include a path/query.

##### Implementations

###### Trait Implementations

- **Same**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UrlError) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: UrlError) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Error**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Eq**
#### Enum `TlsError`

**Attributes:**

- `#[allow(missing_copy_implementations)]`
- `#[non_exhaustive]`

TLS errors.

Note that even if you enable only the rustls-based TLS support, the error at runtime could still
be `Native`, as another crate in the dependency graph may enable native TLS support.

```rust
pub enum TlsError {
    Native(native_tls_crate::Error),
}
```

##### Variants

###### `Native`

Native TLS error.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `native_tls_crate::Error` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: TlsError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: native_tls_crate::Error) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Same**
- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `handshake`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

WebSocket handshake control.

```rust
pub mod handshake { /* ... */ }
```

### Modules

## Module `client`

Client handshake machine.

```rust
pub mod client { /* ... */ }
```

### Types

#### Type Alias `Request`

Client request type.

```rust
pub type Request = http::Request<()>;
```

#### Type Alias `Response`

Client response type.

```rust
pub type Response = http::Response<Option<Vec<u8>>>;
```

#### Struct `ClientHandshake`

Client handshake role.

```rust
pub struct ClientHandshake<S> {
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
  pub fn start(stream: S, request: Request, config: Option<WebSocketConfig>) -> Result<MidHandshake<Self>> { /* ... */ }
  ```
  Initiate a client handshake.

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **HandshakeRole**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Same**
- **RefUnwindSafe**
- **Sync**
### Functions

#### Function `generate_request`

Verifies and generates a client WebSocket request from the original request and extracts a WebSocket key from it.

```rust
pub fn generate_request(request: Request) -> crate::error::Result<(Vec<u8>, String)> { /* ... */ }
```

#### Function `generate_key`

Generate a random key for the `Sec-WebSocket-Key` header.

```rust
pub fn generate_key() -> String { /* ... */ }
```

## Module `headers`

HTTP Request and response header handling.

```rust
pub mod headers { /* ... */ }
```

### Constants and Statics

#### Constant `MAX_HEADERS`

Limit for the number of header lines.

```rust
pub const MAX_HEADERS: usize = 124;
```

## Module `machine`

WebSocket handshake machine.

```rust
pub mod machine { /* ... */ }
```

### Types

#### Struct `HandshakeMachine`

A generic handshake state machine.

```rust
pub struct HandshakeMachine<Stream> {
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
  pub fn start_read(stream: Stream) -> Self { /* ... */ }
  ```
  Start reading data from the peer.

- ```rust
  pub fn start_write<D: Into<Vec<u8>>>(stream: Stream, data: D) -> Self { /* ... */ }
  ```
  Start writing data to the peer.

- ```rust
  pub fn get_ref(self: &Self) -> &Stream { /* ... */ }
  ```
  Returns a shared reference to the inner stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut Stream { /* ... */ }
  ```
  Returns a mutable reference to the inner stream.

- ```rust
  pub fn single_round<Obj: TryParse>(self: Self) -> Result<RoundResult<Obj, Stream>> { /* ... */ }
  ```
  Perform a single handshake round.

###### Trait Implementations

- **UnwindSafe**
- **Same**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Enum `RoundResult`

The result of the round.

```rust
pub enum RoundResult<Obj, Stream> {
    WouldBlock(HandshakeMachine<Stream>),
    Incomplete(HandshakeMachine<Stream>),
    StageFinished(StageResult<Obj, Stream>),
}
```

##### Variants

###### `WouldBlock`

Round not done, I/O would block.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `HandshakeMachine<Stream>` |  |

###### `Incomplete`

Round done, state unchanged.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `HandshakeMachine<Stream>` |  |

###### `StageFinished`

Stage complete.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `StageResult<Obj, Stream>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Same**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
#### Enum `StageResult`

The result of the stage.

```rust
pub enum StageResult<Obj, Stream> {
    DoneReading {
        result: Obj,
        stream: Stream,
        tail: Vec<u8>,
    },
    DoneWriting(Stream),
}
```

##### Variants

###### `DoneReading`

Reading round finished.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `result` | `Obj` |  |
| `stream` | `Stream` |  |
| `tail` | `Vec<u8>` |  |

###### `DoneWriting`

Writing round finished.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Stream` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Same**
- **UnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `TryParse`

The parseable object.

```rust
pub trait TryParse: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `try_parse`: Return Ok(None) if incomplete, Err on syntax error.

##### Implementations

This trait is implemented for the following types:

- `Response`
- `http::header::HeaderMap`
- `Request`

## Module `server`

Server handshake machine.

```rust
pub mod server { /* ... */ }
```

### Types

#### Type Alias `Request`

Server request type.

```rust
pub type Request = http::Request<()>;
```

#### Type Alias `Response`

Server response type.

```rust
pub type Response = http::Response<()>;
```

#### Type Alias `ErrorResponse`

Server error response type.

```rust
pub type ErrorResponse = http::Response<Option<String>>;
```

#### Struct `NoCallback`

Stub for callback that does nothing.

```rust
pub struct NoCallback;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NoCallback { /* ... */ }
    ```

- **Callback**
  - ```rust
    fn on_request(self: Self, _request: &Request, response: Response) -> StdResult<Response, ErrorResponse> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Copy**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Struct `ServerHandshake`

**Attributes:**

- `#[allow(missing_copy_implementations)]`

Server handshake role.

```rust
pub struct ServerHandshake<S, C> {
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
  pub fn start(stream: S, callback: C, config: Option<WebSocketConfig>) -> MidHandshake<Self> { /* ... */ }
  ```
  Start server handshake. `callback` specifies a custom callback which the user can pass to

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Same**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **HandshakeRole**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

### Traits

#### Trait `Callback`

The callback trait.

The callback is called when the server receives an incoming WebSocket
handshake request from the client. Specifying a callback allows you to analyze incoming headers
and add additional headers to the response that server sends to the client and/or reject the
connection based on the incoming headers.

```rust
pub trait Callback: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `on_request`: Called whenever the server read the request from the client and is ready to reply to it.

##### Implementations

This trait is implemented for the following types:

- `F` with <F>
- `NoCallback`

### Functions

#### Function `create_response`

Create a response for the request.

```rust
pub fn create_response(request: &Request) -> crate::error::Result<Response> { /* ... */ }
```

#### Function `create_response_with_body`

Create a response for the request with a custom body.

```rust
pub fn create_response_with_body<T1, T2, /* synthetic */ impl FnOnce() -> T2: FnOnce() -> T2>(request: &http::Request<T1>, generate_body: impl FnOnce() -> T2) -> crate::error::Result<http::Response<T2>> { /* ... */ }
```

#### Function `write_response`

Write `response` to the stream `w`.

```rust
pub fn write_response<T, /* synthetic */ impl io::Write: io::Write>(w: impl io::Write, response: &http::Response<T>) -> crate::error::Result<()> { /* ... */ }
```

### Types

#### Struct `MidHandshake`

A WebSocket handshake.

```rust
pub struct MidHandshake<Role: HandshakeRole> {
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
  pub fn get_ref(self: &Self) -> &HandshakeMachine<<Role as >::InternalStream> { /* ... */ }
  ```
  Allow access to machine

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut HandshakeMachine<<Role as >::InternalStream> { /* ... */ }
  ```
  Allow mutable access to machine

- ```rust
  pub fn handshake(self: Self) -> Result<<Role as >::FinalResult, HandshakeError<Role>> { /* ... */ }
  ```
  Restarts the handshake process.

###### Trait Implementations

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Same**
- **Freeze**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
#### Enum `HandshakeError`

A handshake result.

```rust
pub enum HandshakeError<Role: HandshakeRole> {
    Interrupted(MidHandshake<Role>),
    Failure(crate::error::Error),
}
```

##### Variants

###### `Interrupted`

Handshake was interrupted (would block).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MidHandshake<Role>` |  |

###### `Failure`

Handshake failed.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::error::Error` |  |

##### Implementations

###### Trait Implementations

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Error**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

  - ```rust
    fn from(err: Error) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `HandshakeRole`

Handshake role.

```rust
pub trait HandshakeRole {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `ClientHandshake<S>` with <S: Read + Write>
- `ServerHandshake<S, C>` with <S: Read + Write, C: Callback>

### Functions

#### Function `derive_accept_key`

Derive the `Sec-WebSocket-Accept` response header from a `Sec-WebSocket-Key` request header.

This function can be used to perform a handshake before passing a raw TCP stream to
[`WebSocket::from_raw_socket`][crate::protocol::WebSocket::from_raw_socket].

```rust
pub fn derive_accept_key(request_key: &[u8]) -> String { /* ... */ }
```

## Module `protocol`

Generic WebSocket message stream.

```rust
pub mod protocol { /* ... */ }
```

### Modules

## Module `frame`

Utilities to work with raw WebSocket frames.

```rust
pub mod frame { /* ... */ }
```

### Modules

## Module `coding`

Various codes defined in RFC 6455.

```rust
pub mod coding { /* ... */ }
```

### Types

#### Enum `OpCode`

WebSocket message opcode as in RFC 6455.

```rust
pub enum OpCode {
    Data(Data),
    Control(Control),
}
```

##### Variants

###### `Data`

Data (text or binary).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Data` |  |

###### `Control`

Control message (close, ping, pong).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Control` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OpCode { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OpCode) -> bool { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

  - ```rust
    fn from(code: OpCode) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(byte: u8) -> OpCode { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Same**
- **StructuralPartialEq**
- **Eq**
#### Enum `Data`

Data opcodes as in RFC 6455

```rust
pub enum Data {
    Continue,
    Text,
    Binary,
    Reserved(u8),
}
```

##### Variants

###### `Continue`

0x0 denotes a continuation frame

###### `Text`

0x1 denotes a text frame

###### `Binary`

0x2 denotes a binary frame

###### `Reserved`

0x3-7 are reserved for further non-control frames

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Trait Implementations

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Data { /* ... */ }
    ```

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Data) -> bool { /* ... */ }
    ```

#### Enum `Control`

Control opcodes as in RFC 6455

```rust
pub enum Control {
    Close,
    Ping,
    Pong,
    Reserved(u8),
}
```

##### Variants

###### `Close`

0x8 denotes a connection close

###### `Ping`

0x9 denotes a ping

###### `Pong`

0xa denotes a pong

###### `Reserved`

0xb-f are reserved for further control frames

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Same**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Control) -> bool { /* ... */ }
    ```

- **Sync**
- **Send**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Control { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Enum `CloseCode`

Status code used to indicate why an endpoint is closing the WebSocket connection.

```rust
pub enum CloseCode {
    Normal,
    Away,
    Protocol,
    Unsupported,
    Status,
    Abnormal,
    Invalid,
    Policy,
    Size,
    Extension,
    Error,
    Restart,
    Again,
    // Some variants omitted
}
```

##### Variants

###### `Normal`

Indicates a normal closure, meaning that the purpose for
which the connection was established has been fulfilled.

###### `Away`

Indicates that an endpoint is "going away", such as a server
going down or a browser having navigated away from a page.

###### `Protocol`

Indicates that an endpoint is terminating the connection due
to a protocol error.

###### `Unsupported`

Indicates that an endpoint is terminating the connection
because it has received a type of data it cannot accept (e.g., an
endpoint that understands only text data MAY send this if it
receives a binary message).

###### `Status`

Indicates that no status code was included in a closing frame. This
close code makes it possible to use a single method, `on_close` to
handle even cases where no close code was provided.

###### `Abnormal`

Indicates an abnormal closure. If the abnormal closure was due to an
error, this close code will not be used. Instead, the `on_error` method
of the handler will be called with the error. However, if the connection
is simply dropped, without an error, this close code will be sent to the
handler.

###### `Invalid`

Indicates that an endpoint is terminating the connection
because it has received data within a message that was not
consistent with the type of the message (e.g., non-UTF-8 \[RFC3629\]
data within a text message).

###### `Policy`

Indicates that an endpoint is terminating the connection
because it has received a message that violates its policy.  This
is a generic status code that can be returned when there is no
other more suitable status code (e.g., Unsupported or Size) or if there
is a need to hide specific details about the policy.

###### `Size`

Indicates that an endpoint is terminating the connection
because it has received a message that is too big for it to
process.

###### `Extension`

Indicates that an endpoint (client) is terminating the
connection because it has expected the server to negotiate one or
more extension, but the server didn't return them in the response
message of the WebSocket handshake.  The list of extensions that
are needed should be given as the reason for closing.
Note that this status code is not used by the server, because it
can fail the WebSocket handshake instead.

###### `Error`

Indicates that a server is terminating the connection because
it encountered an unexpected condition that prevented it from
fulfilling the request.

###### `Restart`

Indicates that the server is restarting. A client may choose to reconnect,
and if it does, it should use a randomized delay of 5-30 seconds between attempts.

###### `Again`

Indicates that the server is overloaded and the client should either connect
to a different IP (when multiple targets exist), or reconnect to the same IP
when a user has performed an action.

*Note: Some variants have been omitted because they are private or hidden.*

##### Implementations

###### Methods

- ```rust
  pub fn is_allowed(self: Self) -> bool { /* ... */ }
  ```
  Check if this CloseCode is allowed.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CloseCode { /* ... */ }
    ```

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(code: CloseCode) -> u16 { /* ... */ }
    ```

  - ```rust
    fn from(code: &''t CloseCode) -> u16 { /* ... */ }
    ```

  - ```rust
    fn from(code: u16) -> CloseCode { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CloseCode) -> bool { /* ... */ }
    ```

### Types

#### Struct `FrameSocket`

A reader and writer for WebSocket frames.

```rust
pub struct FrameSocket<Stream> {
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
  pub fn new(stream: Stream) -> Self { /* ... */ }
  ```
  Create a new frame socket.

- ```rust
  pub fn from_partially_read(stream: Stream, part: Vec<u8>) -> Self { /* ... */ }
  ```
  Create a new frame socket from partially read data.

- ```rust
  pub fn into_inner(self: Self) -> (Stream, Vec<u8>) { /* ... */ }
  ```
  Extract a stream from the socket.

- ```rust
  pub fn get_ref(self: &Self) -> &Stream { /* ... */ }
  ```
  Returns a shared reference to the inner stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut Stream { /* ... */ }
  ```
  Returns a mutable reference to the inner stream.

- ```rust
  pub fn read(self: &mut Self, max_size: Option<usize>) -> Result<Option<Frame>> { /* ... */ }
  ```
  Read a frame from stream.

- ```rust
  pub fn send(self: &mut Self, frame: Frame) -> Result<()> { /* ... */ }
  ```
  Writes and immediately flushes a frame.

- ```rust
  pub fn write(self: &mut Self, frame: Frame) -> Result<()> { /* ... */ }
  ```
  Write a frame to stream.

- ```rust
  pub fn flush(self: &mut Self) -> Result<()> { /* ... */ }
  ```
  Flush writes.

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **Send**
### Re-exports

#### Re-export `CloseFrame`

```rust
pub use self::frame::CloseFrame;
```

#### Re-export `Frame`

```rust
pub use self::frame::Frame;
```

#### Re-export `FrameHeader`

```rust
pub use self::frame::FrameHeader;
```

### Types

#### Enum `Role`

Indicates a Client or Server role of the websocket

```rust
pub enum Role {
    Server,
    Client,
}
```

##### Variants

###### `Server`

This socket is a server

###### `Client`

This socket is a client

##### Implementations

###### Trait Implementations

- **Sync**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Copy**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Same**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Role) -> bool { /* ... */ }
    ```

- **Eq**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Role { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `WebSocketConfig`

The configuration for WebSocket connection.

```rust
pub struct WebSocketConfig {
    pub max_send_queue: Option<usize>,
    pub write_buffer_size: usize,
    pub max_write_buffer_size: usize,
    pub max_message_size: Option<usize>,
    pub max_frame_size: Option<usize>,
    pub accept_unmasked_frames: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `max_send_queue` | `Option<usize>` | Does nothing, instead use `max_write_buffer_size`. |
| `write_buffer_size` | `usize` | The target minimum size of the write buffer to reach before writing the data<br>to the underlying stream.<br>The default value is 128 KiB.<br><br>If set to `0` each message will be eagerly written to the underlying stream.<br>It is often more optimal to allow them to buffer a little, hence the default value.<br><br>Note: [`flush`](WebSocket::flush) will always fully write the buffer regardless. |
| `max_write_buffer_size` | `usize` | The max size of the write buffer in bytes. Setting this can provide backpressure<br>in the case the write buffer is filling up due to write errors.<br>The default value is unlimited.<br><br>Note: The write buffer only builds up past [`write_buffer_size`](Self::write_buffer_size)<br>when writes to the underlying stream are failing. So the **write buffer can not<br>fill up if you are not observing write errors even if not flushing**.<br><br>Note: Should always be at least [`write_buffer_size + 1 message`](Self::write_buffer_size)<br>and probably a little more depending on error handling strategy. |
| `max_message_size` | `Option<usize>` | The maximum size of an incoming message. `None` means no size limit. The default value is 64 MiB<br>which should be reasonably big for all normal use-cases but small enough to prevent<br>memory eating by a malicious user. |
| `max_frame_size` | `Option<usize>` | The maximum size of a single incoming message frame. `None` means no size limit. The limit is for<br>frame payload NOT including the frame header. The default value is 16 MiB which should<br>be reasonably big for all normal use-cases but small enough to prevent memory eating<br>by a malicious user. |
| `accept_unmasked_frames` | `bool` | When set to `true`, the server will accept and handle unmasked frames<br>from the client. According to the RFC 6455, the server must close the<br>connection to the client in such cases, however it seems like there are<br>some popular libraries that are sending unmasked frames, ignoring the RFC.<br>By default this option is set to `false`, i.e. according to RFC 6455. |

##### Implementations

###### Trait Implementations

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Send**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WebSocketConfig { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Same**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `WebSocket`

WebSocket input-output stream.

This is THE structure you want to create to be able to speak the WebSocket protocol.
It may be created by calling `connect`, `accept` or `client` functions.

Use [`WebSocket::read`], [`WebSocket::send`] to received and send messages.

```rust
pub struct WebSocket<Stream> {
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
  pub fn from_raw_socket(stream: Stream, role: Role, config: Option<WebSocketConfig>) -> Self { /* ... */ }
  ```
  Convert a raw socket into a WebSocket without performing a handshake.

- ```rust
  pub fn from_partially_read(stream: Stream, part: Vec<u8>, role: Role, config: Option<WebSocketConfig>) -> Self { /* ... */ }
  ```
  Convert a raw socket into a WebSocket without performing a handshake.

- ```rust
  pub fn get_ref(self: &Self) -> &Stream { /* ... */ }
  ```
  Returns a shared reference to the inner stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut Stream { /* ... */ }
  ```
  Returns a mutable reference to the inner stream.

- ```rust
  pub fn set_config</* synthetic */ impl FnOnce(&mut WebSocketConfig): FnOnce(&mut WebSocketConfig)>(self: &mut Self, set_func: impl FnOnce(&mut WebSocketConfig)) { /* ... */ }
  ```
  Change the configuration.

- ```rust
  pub fn get_config(self: &Self) -> &WebSocketConfig { /* ... */ }
  ```
  Read the configuration.

- ```rust
  pub fn can_read(self: &Self) -> bool { /* ... */ }
  ```
  Check if it is possible to read messages.

- ```rust
  pub fn can_write(self: &Self) -> bool { /* ... */ }
  ```
  Check if it is possible to write messages.

- ```rust
  pub fn read(self: &mut Self) -> Result<Message> { /* ... */ }
  ```
  Read a message from stream, if possible.

- ```rust
  pub fn send(self: &mut Self, message: Message) -> Result<()> { /* ... */ }
  ```
  Writes and immediately flushes a message.

- ```rust
  pub fn write(self: &mut Self, message: Message) -> Result<()> { /* ... */ }
  ```
  Write a message to the provided stream, if possible.

- ```rust
  pub fn flush(self: &mut Self) -> Result<()> { /* ... */ }
  ```
  Flush writes.

- ```rust
  pub fn close(self: &mut Self, code: Option<CloseFrame<''_>>) -> Result<()> { /* ... */ }
  ```
  Close the connection.

- ```rust
  pub fn read_message(self: &mut Self) -> Result<Message> { /* ... */ }
  ```
  Old name for [`read`](Self::read).

- ```rust
  pub fn write_message(self: &mut Self, message: Message) -> Result<()> { /* ... */ }
  ```
  Old name for [`send`](Self::send).

- ```rust
  pub fn write_pending(self: &mut Self) -> Result<()> { /* ... */ }
  ```
  Old name for [`flush`](Self::flush).

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Same**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `WebSocketContext`

A context for managing WebSocket stream.

```rust
pub struct WebSocketContext {
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
  pub fn new(role: Role, config: Option<WebSocketConfig>) -> Self { /* ... */ }
  ```
  Create a WebSocket context that manages a post-handshake stream.

- ```rust
  pub fn from_partially_read(part: Vec<u8>, role: Role, config: Option<WebSocketConfig>) -> Self { /* ... */ }
  ```
  Create a WebSocket context that manages an post-handshake stream.

- ```rust
  pub fn set_config</* synthetic */ impl FnOnce(&mut WebSocketConfig): FnOnce(&mut WebSocketConfig)>(self: &mut Self, set_func: impl FnOnce(&mut WebSocketConfig)) { /* ... */ }
  ```
  Change the configuration.

- ```rust
  pub fn get_config(self: &Self) -> &WebSocketConfig { /* ... */ }
  ```
  Read the configuration.

- ```rust
  pub fn can_read(self: &Self) -> bool { /* ... */ }
  ```
  Check if it is possible to read messages.

- ```rust
  pub fn can_write(self: &Self) -> bool { /* ... */ }
  ```
  Check if it is possible to write messages.

- ```rust
  pub fn read<Stream>(self: &mut Self, stream: &mut Stream) -> Result<Message>
where
    Stream: Read + Write { /* ... */ }
  ```
  Read a message from the provided stream, if possible.

- ```rust
  pub fn write<Stream>(self: &mut Self, stream: &mut Stream, message: Message) -> Result<()>
where
    Stream: Read + Write { /* ... */ }
  ```
  Write a message to the provided stream.

- ```rust
  pub fn flush<Stream>(self: &mut Self, stream: &mut Stream) -> Result<()>
where
    Stream: Read + Write { /* ... */ }
  ```
  Flush writes.

- ```rust
  pub fn close<Stream>(self: &mut Self, stream: &mut Stream, code: Option<CloseFrame<''_>>) -> Result<()>
where
    Stream: Read + Write { /* ... */ }
  ```
  Close the connection.

###### Trait Implementations

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Re-exports

#### Re-export `CloseFrame`

```rust
pub use self::frame::CloseFrame;
```

#### Re-export `Message`

```rust
pub use self::message::Message;
```

## Module `stream`

Convenience wrapper for streams to switch between plain TCP and TLS at runtime.

 There is no dependency on actual TLS implementations. Everything like
`native_tls` or `openssl` will work as long as there is a TLS stream supporting standard
`Read + Write` traits.

```rust
pub mod stream { /* ... */ }
```

### Types

#### Enum `Mode`

Stream mode, either plain TCP or TLS.

```rust
pub enum Mode {
    Plain,
    Tls,
}
```

##### Variants

###### `Plain`

Plain mode (`ws://` URL).

###### `Tls`

TLS mode (`wss://` URL).

##### Implementations

###### Trait Implementations

- **Sync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Same**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Mode { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Enum `MaybeTlsStream`

**Attributes:**

- `#[non_exhaustive]`

A stream that might be protected with TLS.

```rust
pub enum MaybeTlsStream<S: Read + Write> {
    Plain(S),
    NativeTls(native_tls_crate::TlsStream<S>),
}
```

##### Variants

###### `Plain`

Unencrypted socket stream.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `S` |  |

###### `NativeTls`

Encrypted socket stream using `native-tls`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `native_tls_crate::TlsStream<S>` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **ReadBytesExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Same**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> IoResult<usize> { /* ... */ }
    ```

- **Unpin**
- **WriteBytesExt**
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
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> IoResult<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> IoResult<()> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **NoDelay**
  - ```rust
    fn set_nodelay(self: &mut Self, nodelay: bool) -> IoResult<()> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
### Traits

#### Trait `NoDelay`

Trait to switch TCP_NODELAY.

```rust
pub trait NoDelay {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `set_nodelay`: Set the TCP_NODELAY option to the given value.

##### Implementations

This trait is implemented for the following types:

- `std::net::TcpStream`
- `native_tls_crate::TlsStream<S>` with <S: Read + Write + NoDelay>
- `MaybeTlsStream<S>` with <S: Read + Write + NoDelay>

## Module `util`

Helper traits to ease non-blocking handling.

```rust
pub mod util { /* ... */ }
```

### Traits

#### Trait `NonBlockingError`

Non-blocking IO handling.

```rust
pub trait NonBlockingError: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `into_non_blocking`: Convert WouldBlock to None and don't touch other errors.

##### Implementations

This trait is implemented for the following types:

- `std::io::Error`
- `crate::error::Error`

#### Trait `NonBlockingResult`

Non-blocking IO wrapper.

This trait is implemented for `Result<T, E: NonBlockingError>`.

```rust
pub trait NonBlockingResult {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Result`: Type of the converted result: `Result<Option<T>, E>`

###### Required Methods

- `no_block`: Perform the non-block conversion.

##### Implementations

This trait is implemented for the following types:

- `std::result::Result<T, E>` with <T, E>

## Re-exports

### Re-export `http`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use http;
```

### Re-export `Error`

```rust
pub use crate::error::Error;
```

### Re-export `Result`

```rust
pub use crate::error::Result;
```

### Re-export `Message`

```rust
pub use crate::protocol::Message;
```

### Re-export `WebSocket`

```rust
pub use crate::protocol::WebSocket;
```

### Re-export `client`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::client::client;
```

### Re-export `connect`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::client::connect;
```

### Re-export `ClientRequestBuilder`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::client::ClientRequestBuilder;
```

### Re-export `ClientHandshake`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::handshake::client::ClientHandshake;
```

### Re-export `ServerHandshake`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::handshake::server::ServerHandshake;
```

### Re-export `HandshakeError`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::handshake::HandshakeError;
```

### Re-export `accept`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::server::accept;
```

### Re-export `accept_hdr`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::server::accept_hdr;
```

### Re-export `accept_hdr_with_config`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::server::accept_hdr_with_config;
```

### Re-export `accept_with_config`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

```rust
pub use crate::server::accept_with_config;
```

### Re-export `client_tls`

**Attributes:**

- `#[<cfg>(all(any(feature = "native-tls", feature = "__rustls-tls"), feature =
"handshake"))]`

```rust
pub use tls::client_tls;
```

### Re-export `client_tls_with_config`

**Attributes:**

- `#[<cfg>(all(any(feature = "native-tls", feature = "__rustls-tls"), feature =
"handshake"))]`

```rust
pub use tls::client_tls_with_config;
```

### Re-export `Connector`

**Attributes:**

- `#[<cfg>(all(any(feature = "native-tls", feature = "__rustls-tls"), feature =
"handshake"))]`

```rust
pub use tls::Connector;
```

