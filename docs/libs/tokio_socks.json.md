# Crate Documentation

**Version:** 0.5.2

**Format Version:** 43

# Module `tokio_socks`

## Modules

## Module `io`

Asynchronous I/O abstractions for sockets.

```rust
pub mod io { /* ... */ }
```

### Traits

#### Trait `AsyncSocket`

A trait for asynchronous socket I/O.

Any type that implements tokio's `AsyncRead` and `AsyncWrite` traits
has implemented `AsyncSocket` trait.

Use `FuturesIoCompatExt` to wrap `futures-io` types as `AsyncSocket` types.

```rust
pub trait AsyncSocket {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `poll_read`
- `poll_write`

##### Implementations

This trait is implemented for the following types:

- `S` with <S>

## Module `tcp`

```rust
pub mod tcp { /* ... */ }
```

### Modules

## Module `socks4`

```rust
pub mod socks4 { /* ... */ }
```

### Types

#### Struct `Socks4Stream`

A SOCKS4 client.

For convenience, it can be dereferenced to it's inner socket.

```rust
pub struct Socks4Stream<S> {
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
  pub async fn connect<''t, P, T>(proxy: P, target: T) -> Result<Socks4Stream<TcpStream>>
where
    P: ToProxyAddrs,
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Connects to a target server through a SOCKS4 proxy given the proxy

- ```rust
  pub async fn connect_with_userid<''a, ''t, P, T>(proxy: P, target: T, user_id: &''a str) -> Result<Socks4Stream<TcpStream>>
where
    P: ToProxyAddrs,
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Connects to a target server through a SOCKS4 proxy using given username,

- ```rust
  pub async fn connect_with_socket<''t, T>(socket: S, target: T) -> Result<Socks4Stream<S>>
where
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Connects to a target server through a SOCKS4 proxy given a socket to it.

- ```rust
  pub async fn connect_with_userid_and_socket<''a, ''t, T>(socket: S, target: T, user_id: &''a str) -> Result<Socks4Stream<S>>
where
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Connects to a target server through a SOCKS4 proxy using given username,

- ```rust
  pub fn into_inner(self: Self) -> S { /* ... */ }
  ```
  Consumes the `Socks4Stream`, returning the inner socket.

- ```rust
  pub fn target_addr(self: &Self) -> TargetAddr<''_> { /* ... */ }
  ```
  Returns the target address that the proxy server connects to.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **Unpin**
- **AsyncRead**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &mut tokio::io::ReadBuf<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **AsyncReadExt**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsyncSocket**
  - ```rust
    fn poll_read(self: Pin<&mut S>, cx: &mut Context<''_>, buf: &mut [u8]) -> Poll<Result<usize, Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_write(self: Pin<&mut S>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<Result<usize, Error>> { /* ... */ }
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

- **Receiver**
- **AsyncWriteExt**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **UnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **AsyncWrite**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<io::Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

#### Struct `Socks4Connector`

A `Future` which resolves to a socket to the target server through proxy.

```rust
pub struct Socks4Connector<''a, ''t, S> {
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
  pub async fn execute(self: &mut Self) -> Result<Socks4Stream<TcpStream>> { /* ... */ }
  ```
  Connect to the proxy server, authenticate and issue the SOCKS command

- ```rust
  pub async fn execute_with_socket<T: AsyncSocket + Unpin>(self: &mut Self, socket: T) -> Result<Socks4Stream<T>> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Socks4Listener`

A SOCKS4 BIND client.

Once you get an instance of `Socks4Listener`, you should send the
`bind_addr` to the remote process via the primary connection. Then, call the
`accept` function and wait for the other end connecting to the rendezvous
address.

```rust
pub struct Socks4Listener<S> {
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
  pub async fn bind<''t, P, T>(proxy: P, target: T) -> Result<Socks4Listener<TcpStream>>
where
    P: ToProxyAddrs,
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Initiates a BIND request to the specified proxy.

- ```rust
  pub async fn bind_with_userid<''a, ''t, P, T>(proxy: P, target: T, user_id: &''a str) -> Result<Socks4Listener<TcpStream>>
where
    P: ToProxyAddrs,
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Initiates a BIND request to the specified proxy using given username

- ```rust
  pub async fn bind_with_socket<''t, T>(socket: S, target: T) -> Result<Socks4Listener<S>>
where
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Initiates a BIND request to the specified proxy using the given socket

- ```rust
  pub async fn bind_with_user_and_socket<''a, ''t, T>(socket: S, target: T, user_id: &''a str) -> Result<Socks4Listener<S>>
where
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Initiates a BIND request to the specified proxy using given username,

- ```rust
  pub fn bind_addr(self: &Self) -> TargetAddr<''_> { /* ... */ }
  ```
  Returns the address of the proxy-side TCP listener.

- ```rust
  pub async fn accept(self: Self) -> Result<Socks4Stream<S>> { /* ... */ }
  ```
  Consumes this listener, returning a `Future` which resolves to the

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `socks5`

```rust
pub mod socks5 { /* ... */ }
```

### Types

#### Struct `Socks5Stream`

A SOCKS5 client.

For convenience, it can be dereferenced to it's inner socket.

```rust
pub struct Socks5Stream<S> {
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
  pub async fn connect<''t, P, T>(proxy: P, target: T) -> Result<Socks5Stream<TcpStream>>
where
    P: ToProxyAddrs,
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Connects to a target server through a SOCKS5 proxy given the proxy

- ```rust
  pub async fn connect_with_password<''a, ''t, P, T>(proxy: P, target: T, username: &''a str, password: &''a str) -> Result<Socks5Stream<TcpStream>>
where
    P: ToProxyAddrs,
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Connects to a target server through a SOCKS5 proxy using given username,

- ```rust
  pub async fn connect_with_socket<''t, T>(socket: S, target: T) -> Result<Socks5Stream<S>>
where
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Connects to a target server through a SOCKS5 proxy given a socket to it.

- ```rust
  pub async fn connect_with_password_and_socket<''a, ''t, T>(socket: S, target: T, username: &''a str, password: &''a str) -> Result<Socks5Stream<S>>
where
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Connects to a target server through a SOCKS5 proxy using given username,

- ```rust
  pub fn into_inner(self: Self) -> S { /* ... */ }
  ```
  Consumes the `Socks5Stream`, returning the inner socket.

- ```rust
  pub fn target_addr(self: &Self) -> TargetAddr<''_> { /* ... */ }
  ```
  Returns the target address that the proxy server connects to.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsyncSocket**
  - ```rust
    fn poll_read(self: Pin<&mut S>, cx: &mut Context<''_>, buf: &mut [u8]) -> Poll<Result<usize, Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_write(self: Pin<&mut S>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<Result<usize, Error>> { /* ... */ }
    ```

- **AsyncWriteExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **AsyncRead**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &mut tokio::io::ReadBuf<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **AsyncWrite**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<io::Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **Sync**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Receiver**
- **UnwindSafe**
- **AsyncReadExt**
#### Struct `SocksConnector`

A `Future` which resolves to a socket to the target server through proxy.

```rust
pub struct SocksConnector<''a, ''t, S> {
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
  pub async fn execute(self: &mut Self) -> Result<Socks5Stream<TcpStream>> { /* ... */ }
  ```
  Connect to the proxy server, authenticate and issue the SOCKS command

- ```rust
  pub async fn execute_with_socket<T: AsyncSocket + Unpin>(self: &mut Self, socket: T) -> Result<Socks5Stream<T>> { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Socks5Listener`

A SOCKS5 BIND client.

Once you get an instance of `Socks5Listener`, you should send the
`bind_addr` to the remote process via the primary connection. Then, call the
`accept` function and wait for the other end connecting to the rendezvous
address.

```rust
pub struct Socks5Listener<S> {
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
  pub async fn bind<''t, P, T>(proxy: P, target: T) -> Result<Socks5Listener<TcpStream>>
where
    P: ToProxyAddrs,
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Initiates a BIND request to the specified proxy.

- ```rust
  pub async fn bind_with_password<''a, ''t, P, T>(proxy: P, target: T, username: &''a str, password: &''a str) -> Result<Socks5Listener<TcpStream>>
where
    P: ToProxyAddrs,
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Initiates a BIND request to the specified proxy using given username

- ```rust
  pub async fn bind_with_socket<''t, T>(socket: S, target: T) -> Result<Socks5Listener<S>>
where
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Initiates a BIND request to the specified proxy using the given socket

- ```rust
  pub async fn bind_with_password_and_socket<''a, ''t, T>(socket: S, target: T, username: &''a str, password: &''a str) -> Result<Socks5Listener<S>>
where
    T: IntoTargetAddr<''t> { /* ... */ }
  ```
  Initiates a BIND request to the specified proxy using given username,

- ```rust
  pub fn bind_addr(self: &Self) -> TargetAddr<''_> { /* ... */ }
  ```
  Returns the address of the proxy-side TCP listener.

- ```rust
  pub async fn accept(self: Self) -> Result<Socks5Stream<S>> { /* ... */ }
  ```
  Consumes this listener, returning a `Future` which resolves to the

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Re-exports

#### Re-export `socks4::*`

```rust
pub use socks4::*;
```

#### Re-export `socks5::*`

```rust
pub use socks5::*;
```

## Types

### Type Alias `Result`

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

### Struct `ProxyAddrsStream`

```rust
pub struct ProxyAddrsStream(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **Sync**
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

- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, _: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryStreamExt**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **IntoEither**
- **StreamExt**
### Enum `TargetAddr`

A SOCKS connection target.

```rust
pub enum TargetAddr<''a> {
    Ip(std::net::SocketAddr),
    Domain(std::borrow::Cow<''a, str>, u16),
}
```

#### Variants

##### `Ip`

Connect to an IP address.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::net::SocketAddr` |  |

##### `Domain`

Connect to a fully-qualified domain name.

The domain name will be passed along to the proxy server and DNS lookup
will happen there.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::borrow::Cow<''a, str>` |  |
| 1 | `u16` |  |

#### Implementations

##### Methods

- ```rust
  pub fn to_owned(self: &Self) -> TargetAddr<''static> { /* ... */ }
  ```
  Creates owned `TargetAddr` by cloning. It is usually used to eliminate

##### Trait Implementations

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToSocketAddrs**
  - ```rust
    fn to_socket_addrs(self: &Self) -> IoResult<<Self as >::Iter> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TargetAddr<''a>) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **IntoEither**
- **Send**
- **StructuralPartialEq**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoTargetAddr**
  - ```rust
    fn into_target_addr(self: Self) -> Result<TargetAddr<''a>> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Traits

### Trait `ToProxyAddrs`

A trait for objects which can be converted or resolved to one or more
`SocketAddr` values, which are going to be connected as the the proxy
server.

This trait is similar to `std::net::ToSocketAddrs` but allows asynchronous
name resolution.

```rust
pub trait ToProxyAddrs {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Output`

##### Required Methods

- `to_proxy_addrs`

#### Implementations

This trait is implemented for the following types:

- `std::net::SocketAddr`
- `(std::net::IpAddr, u16)`
- `(std::net::Ipv4Addr, u16)`
- `(std::net::Ipv6Addr, u16)`
- `std::net::SocketAddrV4`
- `std::net::SocketAddrV6`
- `&''a [std::net::SocketAddr]` with <''a>
- `str`
- `(&''a str, u16)` with <''a>
- `&''a T` with <''a, T: ToProxyAddrs + ?Sized>

### Trait `IntoTargetAddr`

A trait for objects that can be converted to `TargetAddr`.

```rust
pub trait IntoTargetAddr<''a> {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `into_target_addr`: Converts the value of self to a `TargetAddr`.

#### Implementations

This trait is implemented for the following types:

- `std::net::SocketAddr` with <''a>
- `(std::net::IpAddr, u16)` with <''a>
- `(std::net::Ipv4Addr, u16)` with <''a>
- `(std::net::Ipv6Addr, u16)` with <''a>
- `std::net::SocketAddrV4` with <''a>
- `std::net::SocketAddrV6` with <''a>
- `TargetAddr<''a>` with <''a>
- `(&''a str, u16)` with <''a>
- `&''a str` with <''a>
- `String`
- `(String, u16)`
- `&''a T` with <''a, T>

## Re-exports

### Re-export `Error`

```rust
pub use error::Error;
```

