# Crate Documentation

**Version:** 0.2.14

**Format Version:** 43

# Module `native_tls`

An abstraction over platform-specific TLS implementations.

Many applications require TLS/SSL communication in one form or another as
part of their implementation, but finding a library for this isn't always
trivial! The purpose of this crate is to provide a seamless integration
experience on all platforms with a cross-platform API that deals with all
the underlying details for you.

# How is this implemented?

This crate uses SChannel on Windows (via the `schannel` crate), Secure
Transport on OSX (via the `security-framework` crate), and OpenSSL (via the
`openssl` crate) on all other platforms. Future features may also enable
other TLS frameworks as well, but these initial libraries are likely to
remain as the defaults.

Note that this crate also strives to be secure-by-default. For example when
using OpenSSL it will configure validation callbacks to ensure that
hostnames match certificates, use strong ciphers, etc. This implies that
this crate is *not* just a thin abstraction around the underlying libraries,
but also an implementation that strives to strike reasonable defaults.

# Supported features

This crate supports the following features out of the box:

* TLS/SSL client communication
* TLS/SSL server communication
* PKCS#12 encoded identities
* X.509/PKCS#8 encoded identities
* Secure-by-default for client and server
    * Includes hostname verification for clients
* Supports asynchronous I/O for both the server and the client

# Cargo Features

* `vendored` - If enabled, the crate will compile and statically link to a
    vendored copy of OpenSSL. This feature has no effect on Windows and
    macOS, where OpenSSL is not used.

# Examples

To connect as a client to a remote server:

```rust
use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

let connector = TlsConnector::new().unwrap();

let stream = TcpStream::connect("google.com:443").unwrap();
let mut stream = connector.connect("google.com", stream).unwrap();

stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut res = vec![];
stream.read_to_end(&mut res).unwrap();
println!("{}", String::from_utf8_lossy(&res));
```

To accept connections as a server from remote clients:

```rust,no_run
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

let mut file = File::open("identity.pfx").unwrap();
let mut identity = vec![];
file.read_to_end(&mut identity).unwrap();
let identity = Identity::from_pkcs12(&identity, "hunter2").unwrap();

let listener = TcpListener::bind("0.0.0.0:8443").unwrap();
let acceptor = TlsAcceptor::new(identity).unwrap();
let acceptor = Arc::new(acceptor);

fn handle_client(stream: TlsStream<TcpStream>) {
    // ...
}

for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            let acceptor = acceptor.clone();
            thread::spawn(move || {
                let stream = acceptor.accept(stream).unwrap();
                handle_client(stream);
            });
        }
        Err(e) => { /* connection failed */ }
    }
}
```

## Types

### Type Alias `Result`

A typedef of the result-type returned by many methods.

```rust
pub type Result<T> = result::Result<T, Error>;
```

### Struct `Error`

An error returned from the TLS implementation.

```rust
pub struct Error(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `Identity`

A cryptographic identity.

An identity is an X509 certificate along with its corresponding private key and chain of certificates to a trusted
root.

```rust
pub struct Identity(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn from_pkcs12(der: &[u8], password: &str) -> Result<Identity> { /* ... */ }
  ```
  Parses a DER-formatted PKCS #12 archive, using the specified password to decrypt the key.

- ```rust
  pub fn from_pkcs8(pem: &[u8], key: &[u8]) -> Result<Identity> { /* ... */ }
  ```
  Parses a chain of PEM encoded X509 certificates, with the leaf certificate first.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Identity { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
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

- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `Certificate`

An X509 certificate.

```rust
pub struct Certificate(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn from_der(der: &[u8]) -> Result<Certificate> { /* ... */ }
  ```
  Parses a DER-formatted X509 certificate.

- ```rust
  pub fn from_pem(pem: &[u8]) -> Result<Certificate> { /* ... */ }
  ```
  Parses a PEM-formatted X509 certificate.

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>> { /* ... */ }
  ```
  Returns the DER-encoded representation of this certificate.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Certificate { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `MidHandshakeTlsStream`

A TLS stream which has been interrupted midway through the handshake process.

```rust
pub struct MidHandshakeTlsStream<S>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

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

- ```rust
  pub fn handshake(self: Self) -> result::Result<TlsStream<S>, HandshakeError<S>> { /* ... */ }
  ```
  Restarts the handshake process.

##### Trait Implementations

- **Send**
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
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **RefUnwindSafe**
- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
### Enum `HandshakeError`

An error returned from `ClientBuilder::handshake`.

```rust
pub enum HandshakeError<S> {
    Failure(Error),
    WouldBlock(MidHandshakeTlsStream<S>),
}
```

#### Variants

##### `Failure`

A fatal error.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Error` |  |

##### `WouldBlock`

A stream interrupted midway through the handshake process due to a
`WouldBlock` error.

Note that this is not a fatal error and it should be safe to call
`handshake` at a later time once the stream is ready to perform I/O
again.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MidHandshakeTlsStream<S>` |  |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Unpin**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
### Enum `Protocol`

**Attributes:**

- `#[non_exhaustive]`

SSL/TLS protocol versions.

```rust
pub enum Protocol {
    Sslv3,
    Tlsv10,
    Tlsv11,
    Tlsv12,
}
```

#### Variants

##### `Sslv3`

The SSL 3.0 protocol.

# Warning

SSL 3.0 has severe security flaws, and should not be used unless absolutely necessary. If
you are not sure if you need to enable this protocol, you should not.

##### `Tlsv10`

The TLS 1.0 protocol.

##### `Tlsv11`

The TLS 1.1 protocol.

##### `Tlsv12`

The TLS 1.2 protocol.

#### Implementations

##### Trait Implementations

- **Freeze**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Protocol { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `TlsConnectorBuilder`

A builder for `TlsConnector`s.

You can get one from [`TlsConnector::builder()`](TlsConnector::builder)

```rust
pub struct TlsConnectorBuilder {
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
  pub fn identity(self: &mut Self, identity: Identity) -> &mut TlsConnectorBuilder { /* ... */ }
  ```
  Sets the identity to be used for client certificate authentication.

- ```rust
  pub fn min_protocol_version(self: &mut Self, protocol: Option<Protocol>) -> &mut TlsConnectorBuilder { /* ... */ }
  ```
  Sets the minimum supported protocol version.

- ```rust
  pub fn max_protocol_version(self: &mut Self, protocol: Option<Protocol>) -> &mut TlsConnectorBuilder { /* ... */ }
  ```
  Sets the maximum supported protocol version.

- ```rust
  pub fn add_root_certificate(self: &mut Self, cert: Certificate) -> &mut TlsConnectorBuilder { /* ... */ }
  ```
  Adds a certificate to the set of roots that the connector will trust.

- ```rust
  pub fn disable_built_in_roots(self: &mut Self, disable: bool) -> &mut TlsConnectorBuilder { /* ... */ }
  ```
  Controls the use of built-in system certificates during certificate validation.

- ```rust
  pub fn danger_accept_invalid_certs(self: &mut Self, accept_invalid_certs: bool) -> &mut TlsConnectorBuilder { /* ... */ }
  ```
  Controls the use of certificate validation.

- ```rust
  pub fn use_sni(self: &mut Self, use_sni: bool) -> &mut TlsConnectorBuilder { /* ... */ }
  ```
  Controls the use of Server Name Indication (SNI).

- ```rust
  pub fn danger_accept_invalid_hostnames(self: &mut Self, accept_invalid_hostnames: bool) -> &mut TlsConnectorBuilder { /* ... */ }
  ```
  Controls the use of hostname verification.

- ```rust
  pub fn build(self: &Self) -> Result<TlsConnector> { /* ... */ }
  ```
  Creates a new `TlsConnector`.

##### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `TlsConnector`

A builder for client-side TLS connections.

# Examples

```rust
use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

let connector = TlsConnector::new().unwrap();

let stream = TcpStream::connect("google.com:443").unwrap();
let mut stream = connector.connect("google.com", stream).unwrap();

stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut res = vec![];
stream.read_to_end(&mut res).unwrap();
println!("{}", String::from_utf8_lossy(&res));
```

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
  pub fn new() -> Result<TlsConnector> { /* ... */ }
  ```
  Returns a new connector with default settings.

- ```rust
  pub fn builder() -> TlsConnectorBuilder { /* ... */ }
  ```
  Returns a new builder for a `TlsConnector`.

- ```rust
  pub fn connect<S>(self: &Self, domain: &str, stream: S) -> result::Result<TlsStream<S>, HandshakeError<S>>
where
    S: io::Read + io::Write { /* ... */ }
  ```
  Initiates a TLS handshake.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TlsConnector { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
### Struct `TlsAcceptorBuilder`

A builder for `TlsAcceptor`s.

You can get one from [`TlsAcceptor::builder()`](TlsAcceptor::builder)

```rust
pub struct TlsAcceptorBuilder {
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
  pub fn min_protocol_version(self: &mut Self, protocol: Option<Protocol>) -> &mut TlsAcceptorBuilder { /* ... */ }
  ```
  Sets the minimum supported protocol version.

- ```rust
  pub fn max_protocol_version(self: &mut Self, protocol: Option<Protocol>) -> &mut TlsAcceptorBuilder { /* ... */ }
  ```
  Sets the maximum supported protocol version.

- ```rust
  pub fn build(self: &Self) -> Result<TlsAcceptor> { /* ... */ }
  ```
  Creates a new `TlsAcceptor`.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
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
- **UnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
### Struct `TlsAcceptor`

A builder for server-side TLS connections.

# Examples

```rust,no_run
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

let mut file = File::open("identity.pfx").unwrap();
let mut identity = vec![];
file.read_to_end(&mut identity).unwrap();
let identity = Identity::from_pkcs12(&identity, "hunter2").unwrap();

let listener = TcpListener::bind("0.0.0.0:8443").unwrap();
let acceptor = TlsAcceptor::new(identity).unwrap();
let acceptor = Arc::new(acceptor);

fn handle_client(stream: TlsStream<TcpStream>) {
    // ...
}

for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            let acceptor = acceptor.clone();
            thread::spawn(move || {
                let stream = acceptor.accept(stream).unwrap();
                handle_client(stream);
            });
        }
        Err(e) => { /* connection failed */ }
    }
}
```

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
  pub fn new(identity: Identity) -> Result<TlsAcceptor> { /* ... */ }
  ```
  Creates a acceptor with default settings.

- ```rust
  pub fn builder(identity: Identity) -> TlsAcceptorBuilder { /* ... */ }
  ```
  Returns a new builder for a `TlsAcceptor`.

- ```rust
  pub fn accept<S>(self: &Self, stream: S) -> result::Result<TlsStream<S>, HandshakeError<S>>
where
    S: io::Read + io::Write { /* ... */ }
  ```
  Initiates a TLS handshake.

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TlsAcceptor { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
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
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
### Struct `TlsStream`

A stream managing a TLS session.

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
  pub fn get_ref(self: &Self) -> &S { /* ... */ }
  ```
  Returns a shared reference to the inner stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut S { /* ... */ }
  ```
  Returns a mutable reference to the inner stream.

- ```rust
  pub fn buffered_read_size(self: &Self) -> Result<usize> { /* ... */ }
  ```
  Returns the number of bytes that can be read without resulting in any

- ```rust
  pub fn peer_certificate(self: &Self) -> Result<Option<Certificate>> { /* ... */ }
  ```
  Returns the peer's leaf certificate, if available.

- ```rust
  pub fn tls_server_end_point(self: &Self) -> Result<Option<Vec<u8>>> { /* ... */ }
  ```
  Returns the tls-server-end-point channel binding data as defined in [RFC 5929].

- ```rust
  pub fn shutdown(self: &mut Self) -> io::Result<()> { /* ... */ }
  ```
  Shuts down the TLS session.

##### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
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

