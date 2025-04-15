# Crate Documentation

**Version:** 0.3.1

**Format Version:** 43

# Module `tokio_native_tls`

Async TLS streams

This library is an implementation of TLS streams using the most appropriate
system library by default for negotiating the connection. That is, on
Windows this library uses SChannel, on OSX it uses SecureTransport, and on
other platforms it uses OpenSSL.

Each TLS stream implements the `Read` and `Write` traits to interact and
interoperate with the rest of the futures I/O ecosystem. Client connections
initiated from this crate verify hostnames automatically and by default.

This crate primarily exports this ability through two newtypes,
`TlsConnector` and `TlsAcceptor`. These newtypes augment the
functionality provided by the `native-tls` crate, on which this crate is
built. Configuration of TLS parameters is still primarily done through the
`native-tls` crate.

## Modules

## Module `native_tls`

re-export native_tls

```rust
pub mod native_tls { /* ... */ }
```

### Re-exports

#### Re-export `native_tls::*`

```rust
pub use native_tls::*;
```

## Types

### Struct `AllowStd`

An intermediate wrapper for the inner stream `S`.

```rust
pub struct AllowStd<S> {
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
  pub fn get_ref(self: &Self) -> &S { /* ... */ }
  ```
  Returns a shared reference to the inner stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut S { /* ... */ }
  ```
  Returns a mutable reference to the inner stream.

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Freeze**
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

- **Sync**
- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

### Struct `TlsStream`

A wrapper around an underlying raw stream which implements the TLS or SSL
protocol.

A `TlsStream<S>` represents a handshake that has been completed successfully
and both the server and the client are ready for receiving and sending
data. Bytes read from a `TlsStream` are decrypted from `S` and bytes written
to a `TlsStream` are encrypted when passing through to `S`.

```rust
pub struct TlsStream<S>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn get_ref(self: &Self) -> &native_tls::TlsStream<AllowStd<S>> { /* ... */ }
  ```
  Returns a shared reference to the inner stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut native_tls::TlsStream<AllowStd<S>> { /* ... */ }
  ```
  Returns a mutable reference to the inner stream.

##### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsyncWriteExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsyncRead**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, ctx: &mut Context<''_>, buf: &mut ReadBuf<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **Send**
- **AsyncReadExt**
- **AsyncWrite**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, ctx: &mut Context<''_>, buf: &[u8]) -> Poll<io::Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, ctx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **AsRawFd**
  - ```rust
    fn as_raw_fd(self: &Self) -> RawFd { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Struct `TlsConnector`

A wrapper around a `native_tls::TlsConnector`, providing an async `connect`
method.

```rust
pub struct TlsConnector(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub async fn connect<S>(self: &Self, domain: &str, stream: S) -> Result<TlsStream<S>, Error>
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
  ```
  Connects the provided stream with this connector, assuming the provided

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TlsConnector { /* ... */ }
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

  - ```rust
    fn from(inner: native_tls::TlsConnector) -> TlsConnector { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
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

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `TlsAcceptor`

A wrapper around a `native_tls::TlsAcceptor`, providing an async `accept`
method.

```rust
pub struct TlsAcceptor(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub async fn accept<S>(self: &Self, stream: S) -> Result<TlsStream<S>, Error>
where
    S: AsyncRead + AsyncWrite + Unpin { /* ... */ }
  ```
  Accepts a new client connection with the provided stream.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(inner: native_tls::TlsAcceptor) -> TlsAcceptor { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TlsAcceptor { /* ... */ }
    ```

