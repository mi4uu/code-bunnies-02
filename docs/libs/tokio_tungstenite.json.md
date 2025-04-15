# Crate Documentation

**Version:** 0.24.0

**Format Version:** 43

# Module `tokio_tungstenite`

Async WebSocket usage.

This library is an implementation of WebSocket handshakes and streams. It
is based on the crate which implements all required WebSocket protocol
logic. So this crate basically just brings tokio support / tokio integration
to it.

Each WebSocket stream implements the required `Stream` and `Sink` traits,
so the socket is just a stream of messages coming in and going out.

## Types

### Struct `WebSocketStream`

A wrapper around an underlying raw stream which implements the WebSocket
protocol.

A `WebSocketStream<S>` represents a handshake that has been completed
successfully and both the server and the client are ready for receiving
and sending data. Message from a `WebSocketStream<S>` are accessible
through the respective `Stream` and `Sink`. Check more information about
them in `futures-rs` crate documentation or have a look on the examples
and unit tests for this crate.

```rust
pub struct WebSocketStream<S> {
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
  pub async fn from_raw_socket(stream: S, role: Role, config: Option<WebSocketConfig>) -> Self
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
  ```
  Convert a raw socket into a WebSocketStream without performing a

- ```rust
  pub async fn from_partially_read(stream: S, part: Vec<u8>, role: Role, config: Option<WebSocketConfig>) -> Self
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
  ```
  Convert a raw socket into a WebSocketStream without performing a

- ```rust
  pub fn get_ref(self: &Self) -> &S
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
  ```
  Returns a shared reference to the inner stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut S
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
  ```
  Returns a mutable reference to the inner stream.

- ```rust
  pub fn get_config(self: &Self) -> &WebSocketConfig { /* ... */ }
  ```
  Returns a reference to the configuration of the tungstenite stream.

- ```rust
  pub async fn close(self: &mut Self, msg: Option<CloseFrame<''_>>) -> Result<(), WsError>
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
  ```
  Close the underlying web socket

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sink**
  - ```rust
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn start_send(self: Pin<&mut Self>, item: Message) -> Result<(), <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
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

- **StreamExt**
- **TryStreamExt**
- **SinkExt**
- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FusedStream**
  - ```rust
    fn is_terminated(self: &Self) -> bool { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Unpin**
- **Send**
- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Functions

### Function `client_async`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

Creates a WebSocket handshake from a request and a stream.
For convenience, the user may call this with a url string, a URL,
or a `Request`. Calling with `Request` allows the user to add
a WebSocket protocol or other custom headers.

Internally, this custom creates a handshake representation and returns
a future representing the resolution of the WebSocket handshake. The
returned future will resolve to either `WebSocketStream<S>` or `Error`
depending on whether the handshake is successful.

This is typically used for clients who have already established, for
example, a TCP connection to the remote server.

```rust
pub async fn client_async<''a, R, S>(request: R, stream: S) -> Result<(WebSocketStream<S>, tungstenite::handshake::client::Response), tungstenite::error::Error>
where
    R: IntoClientRequest + Unpin,
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
```

### Function `client_async_with_config`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

The same as `client_async()` but the one can specify a websocket configuration.
Please refer to `client_async()` for more details.

```rust
pub async fn client_async_with_config<''a, R, S>(request: R, stream: S, config: Option<tungstenite::protocol::WebSocketConfig>) -> Result<(WebSocketStream<S>, tungstenite::handshake::client::Response), tungstenite::error::Error>
where
    R: IntoClientRequest + Unpin,
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
```

### Function `accept_async`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

Accepts a new WebSocket connection with the provided stream.

This function will internally call `server::accept` to create a
handshake representation and returns a future representing the
resolution of the WebSocket handshake. The returned future will resolve
to either `WebSocketStream<S>` or `Error` depending if it's successful
or not.

This is typically used after a socket has been accepted from a
`TcpListener`. That socket is then passed to this function to perform
the server half of the accepting a client's websocket connection.

```rust
pub async fn accept_async<S>(stream: S) -> Result<WebSocketStream<S>, tungstenite::error::Error>
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
```

### Function `accept_async_with_config`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

The same as `accept_async()` but the one can specify a websocket configuration.
Please refer to `accept_async()` for more details.

```rust
pub async fn accept_async_with_config<S>(stream: S, config: Option<tungstenite::protocol::WebSocketConfig>) -> Result<WebSocketStream<S>, tungstenite::error::Error>
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
```

### Function `accept_hdr_async`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

Accepts a new WebSocket connection with the provided stream.

This function does the same as `accept_async()` but accepts an extra callback
for header processing. The callback receives headers of the incoming
requests and is able to add extra headers to the reply.

```rust
pub async fn accept_hdr_async<S, C>(stream: S, callback: C) -> Result<WebSocketStream<S>, tungstenite::error::Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
    C: Callback + Unpin { /* ... */ }
```

### Function `accept_hdr_async_with_config`

**Attributes:**

- `#[<cfg>(feature = "handshake")]`

The same as `accept_hdr_async()` but the one can specify a websocket configuration.
Please refer to `accept_hdr_async()` for more details.

```rust
pub async fn accept_hdr_async_with_config<S, C>(stream: S, callback: C, config: Option<tungstenite::protocol::WebSocketConfig>) -> Result<WebSocketStream<S>, tungstenite::error::Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
    C: Callback + Unpin { /* ... */ }
```

## Re-exports

### Re-export `tungstenite`

```rust
pub use tungstenite;
```

### Re-export `Connector`

**Attributes:**

- `#[<cfg>(any(feature = "native-tls", feature = "__rustls-tls", feature =
"connect"))]`

```rust
pub use tls::Connector;
```

### Re-export `client_async_tls`

**Attributes:**

- `#[<cfg>(any(feature = "native-tls", feature = "__rustls-tls"))]`

```rust
pub use tls::client_async_tls;
```

### Re-export `client_async_tls_with_config`

**Attributes:**

- `#[<cfg>(any(feature = "native-tls", feature = "__rustls-tls"))]`

```rust
pub use tls::client_async_tls_with_config;
```

### Re-export `connect_async`

**Attributes:**

- `#[<cfg>(feature = "connect")]`

```rust
pub use connect::connect_async;
```

### Re-export `connect_async_with_config`

**Attributes:**

- `#[<cfg>(feature = "connect")]`

```rust
pub use connect::connect_async_with_config;
```

### Re-export `connect_async_tls_with_config`

**Attributes:**

- `#[<cfg>(all(any(feature = "native-tls", feature = "__rustls-tls"), feature =
"connect"))]`

```rust
pub use connect::connect_async_tls_with_config;
```

### Re-export `MaybeTlsStream`

**Attributes:**

- `#[<cfg>(feature = "stream")]`

```rust
pub use stream::MaybeTlsStream;
```

