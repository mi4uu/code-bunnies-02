# Crate Documentation

**Version:** 0.23.26

**Format Version:** 43

# Module `rustls`

# Rustls - a modern TLS library

Rustls is a TLS library that aims to provide a good level of cryptographic security,
requires no configuration to achieve that security, and provides no unsafe features or
obsolete cryptography by default.

Rustls implements TLS1.2 and TLS1.3 for both clients and servers. See [the full
list of protocol features](manual::_04_features).

### Platform support

While Rustls itself is platform independent, by default it uses [`aws-lc-rs`] for implementing
the cryptography in TLS.  See [the aws-lc-rs FAQ][aws-lc-rs-platforms-faq] for more details of the
platform/architecture support constraints in aws-lc-rs.

[`ring`] is also available via the `ring` crate feature: see
[the supported `ring` target platforms][ring-target-platforms].

By providing a custom instance of the [`crypto::CryptoProvider`] struct, you
can replace all cryptography dependencies of rustls.  This is a route to being portable
to a wider set of architectures and environments, or compliance requirements.  See the
[`crypto::CryptoProvider`] documentation for more details.

Specifying `default-features = false` when depending on rustls will remove the implicit
dependency on aws-lc-rs.

Rustls requires Rust 1.71 or later. It has an optional dependency on zlib-rs which requires 1.75 or later.

[ring-target-platforms]: https://github.com/briansmith/ring/blob/2e8363b433fa3b3962c877d9ed2e9145612f3160/include/ring-core/target.h#L18-L64
[`crypto::CryptoProvider`]: crate::crypto::CryptoProvider
[`ring`]: https://crates.io/crates/ring
[aws-lc-rs-platforms-faq]: https://aws.github.io/aws-lc-rs/faq.html#can-i-run-aws-lc-rs-on-x-platform-or-architecture
[`aws-lc-rs`]: https://crates.io/crates/aws-lc-rs

### Cryptography providers

Since Rustls 0.22 it has been possible to choose the provider of the cryptographic primitives
that Rustls uses. This may be appealing if you have specific platform, compliance or feature
requirements that aren't met by the default provider, [`aws-lc-rs`].

Users that wish to customize the provider in use can do so when constructing `ClientConfig`
and `ServerConfig` instances using the `with_crypto_provider` method on the respective config
builder types. See the [`crypto::CryptoProvider`] documentation for more details.

#### Built-in providers

Rustls ships with two built-in providers controlled by associated crate features:

  * [`aws-lc-rs`] - enabled by default, available with the `aws_lc_rs` crate feature enabled.
  * [`ring`] - available with the `ring` crate feature enabled.

See the documentation for [`crypto::CryptoProvider`] for details on how providers are
selected.

#### Third-party providers

The community has also started developing third-party providers for Rustls:

  * [`rustls-mbedtls-provider`] - a provider that uses [`mbedtls`] for cryptography.
  * [`rustls-openssl`] - a provider that uses [OpenSSL] for cryptography.
  * [`boring-rustls-provider`] - a work-in-progress provider that uses [`boringssl`] for
    cryptography.
  * [`rustls-rustcrypto`] - an experimental provider that uses the crypto primitives
    from [`RustCrypto`] for cryptography.
  * [`rustls-symcrypt`] - a provider that uses Microsoft's [SymCrypt] library.
  * [`rustls-wolfcrypt-provider`] - a work-in-progress provider that uses [`wolfCrypt`] for cryptography.

[`rustls-mbedtls-provider`]: https://github.com/fortanix/rustls-mbedtls-provider
[`mbedtls`]: https://github.com/Mbed-TLS/mbedtls
[`rustls-openssl`]: https://github.com/tofay/rustls-openssl
[OpenSSL]: https://openssl-library.org/
[`rustls-symcrypt`]: https://github.com/microsoft/rustls-symcrypt
[SymCrypt]: https://github.com/microsoft/SymCrypt
[`boring-rustls-provider`]: https://github.com/janrueth/boring-rustls-provider
[`boringssl`]: https://github.com/google/boringssl
[`rustls-rustcrypto`]: https://github.com/RustCrypto/rustls-rustcrypto
[`RustCrypto`]: https://github.com/RustCrypto
[`rustls-wolfcrypt-provider`]: https://github.com/wolfSSL/rustls-wolfcrypt-provider
[`wolfCrypt`]: https://www.wolfssl.com/products/wolfcrypt

#### Custom provider

We also provide a simple example of writing your own provider in the [custom provider example].
This example implements a minimal provider using parts of the [`RustCrypto`] ecosystem.

See the [Making a custom CryptoProvider] section of the documentation for more information
on this topic.

[custom provider example]: https://github.com/rustls/rustls/tree/main/provider-example/
[`RustCrypto`]: https://github.com/RustCrypto
[Making a custom CryptoProvider]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#making-a-custom-cryptoprovider

## Design overview

Rustls is a low-level library. If your goal is to make HTTPS connections you may prefer
to use a library built on top of Rustls like [hyper] or [ureq].

[hyper]: https://crates.io/crates/hyper
[ureq]: https://crates.io/crates/ureq

### Rustls does not take care of network IO
It doesn't make or accept TCP connections, or do DNS, or read or write files.

Our [examples] directory contains demos that show how to handle I/O using the
[`stream::Stream`] helper, as well as more complex asynchronous I/O using [`mio`].
If you're already using Tokio for an async runtime you may prefer to use [`tokio-rustls`] instead
of interacting with rustls directly.

[examples]: https://github.com/rustls/rustls/tree/main/examples
[`tokio-rustls`]: https://github.com/rustls/tokio-rustls

### Rustls provides encrypted pipes
These are the [`ServerConnection`] and [`ClientConnection`] types.  You supply raw TLS traffic
on the left (via the [`read_tls()`] and [`write_tls()`] methods) and then read/write the
plaintext on the right:

[`read_tls()`]: Connection::read_tls
[`write_tls()`]: Connection::read_tls

```text
         TLS                                   Plaintext
         ===                                   =========
    read_tls()      +-----------------------+      reader() as io::Read
                    |                       |
          +--------->   ClientConnection    +--------->
                    |          or           |
          <---------+   ServerConnection    <---------+
                    |                       |
    write_tls()     +-----------------------+      writer() as io::Write
```

### Rustls takes care of server certificate verification
You do not need to provide anything other than a set of root certificates to trust.
Certificate verification cannot be turned off or disabled in the main API.

## Getting started
This is the minimum you need to do to make a TLS client connection.

First we load some root certificates.  These are used to authenticate the server.
The simplest way is to depend on the [`webpki_roots`] crate which contains
the Mozilla set of root certificates.

```rust,no_run
# #[cfg(feature = "aws-lc-rs")] {
let root_store = rustls::RootCertStore::from_iter(
    webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned(),
);
# }
```

[`webpki_roots`]: https://crates.io/crates/webpki-roots

Next, we make a `ClientConfig`.  You're likely to make one of these per process,
and use it for all connections made by that process.

```rust,no_run
# #[cfg(feature = "aws_lc_rs")] {
# let root_store: rustls::RootCertStore = panic!();
let config = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();
# }
```

Now we can make a connection.  You need to provide the server's hostname so we
know what to expect to find in the server's certificate.

```rust
# #[cfg(feature = "aws_lc_rs")] {
# use rustls;
# use webpki;
# use std::sync::Arc;
# rustls::crypto::aws_lc_rs::default_provider().install_default();
# let root_store = rustls::RootCertStore::from_iter(
#  webpki_roots::TLS_SERVER_ROOTS
#      .iter()
#      .cloned(),
# );
# let config = rustls::ClientConfig::builder()
#     .with_root_certificates(root_store)
#     .with_no_client_auth();
let rc_config = Arc::new(config);
let example_com = "example.com".try_into().unwrap();
let mut client = rustls::ClientConnection::new(rc_config, example_com);
# }
```

Now you should do appropriate IO for the `client` object.  If `client.wants_read()` yields
true, you should call `client.read_tls()` when the underlying connection has data.
Likewise, if `client.wants_write()` yields true, you should call `client.write_tls()`
when the underlying connection is able to send data.  You should continue doing this
as long as the connection is valid.

The return types of `read_tls()` and `write_tls()` only tell you if the IO worked.  No
parsing or processing of the TLS messages is done.  After each `read_tls()` you should
therefore call `client.process_new_packets()` which parses and processes the messages.
Any error returned from `process_new_packets` is fatal to the connection, and will tell you
why.  For example, if the server's certificate is expired `process_new_packets` will
return `Err(InvalidCertificate(Expired))`.  From this point on,
`process_new_packets` will not do any new work and will return that error continually.

You can extract newly received data by calling `client.reader()` (which implements the
`io::Read` trait).  You can send data to the peer by calling `client.writer()` (which
implements `io::Write` trait).  Note that `client.writer().write()` buffers data you
send if the TLS connection is not yet established: this is useful for writing (say) a
HTTP request, but this is buffered so avoid large amounts of data.

The following code uses a fictional socket IO API for illustration, and does not handle
errors.

```rust,no_run
# #[cfg(feature = "aws_lc_rs")] {
# let mut client = rustls::ClientConnection::new(panic!(), panic!()).unwrap();
# struct Socket { }
# impl Socket {
#   fn ready_for_write(&self) -> bool { false }
#   fn ready_for_read(&self) -> bool { false }
#   fn wait_for_something_to_happen(&self) { }
# }
#
# use std::io::{Read, Write, Result};
# impl Read for Socket {
#   fn read(&mut self, buf: &mut [u8]) -> Result<usize> { panic!() }
# }
# impl Write for Socket {
#   fn write(&mut self, buf: &[u8]) -> Result<usize> { panic!() }
#   fn flush(&mut self) -> Result<()> { panic!() }
# }
#
# fn connect(_address: &str, _port: u16) -> Socket {
#   panic!();
# }
use std::io;
use rustls::Connection;

client.writer().write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut socket = connect("example.com", 443);
loop {
  if client.wants_read() && socket.ready_for_read() {
    client.read_tls(&mut socket).unwrap();
    client.process_new_packets().unwrap();

    let mut plaintext = Vec::new();
    client.reader().read_to_end(&mut plaintext).unwrap();
    io::stdout().write(&plaintext).unwrap();
  }

  if client.wants_write() && socket.ready_for_write() {
    client.write_tls(&mut socket).unwrap();
  }

  socket.wait_for_something_to_happen();
}
# }
```

# Examples

You can find several client and server examples of varying complexity in the [examples]
directory, including [`tlsserver-mio`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsserver-mio.rs)
and [`tlsclient-mio`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsclient-mio.rs)
\- full worked examples using [`mio`].

[`mio`]: https://docs.rs/mio/latest/mio/

# Manual

The [rustls manual](crate::manual) explains design decisions and includes how-to guidance.

# Crate features
Here's a list of what features are exposed by the rustls crate and what
they mean.

- `std` (enabled by default): enable the high-level (buffered) Connection API and other functionality
  which relies on the `std` library.

- `aws_lc_rs` (enabled by default): makes the rustls crate depend on the [`aws-lc-rs`] crate.
  Use `rustls::crypto::aws_lc_rs::default_provider().install_default()` to
  use it as the default `CryptoProvider`, or provide it explicitly
  when making a `ClientConfig` or `ServerConfig`.

  Note that aws-lc-rs has additional build-time dependencies like cmake.
  See [the documentation](https://aws.github.io/aws-lc-rs/requirements/index.html) for details.

- `ring`: makes the rustls crate depend on the *ring* crate for cryptography.
  Use `rustls::crypto::ring::default_provider().install_default()` to
  use it as the default `CryptoProvider`, or provide it explicitly
  when making a `ClientConfig` or `ServerConfig`.

- `fips`: enable support for FIPS140-3-approved cryptography, via the [`aws-lc-rs`] crate.
  This feature enables the `aws_lc_rs` crate feature, which makes the rustls crate depend
  on [aws-lc-rs](https://github.com/aws/aws-lc-rs).  It also changes the default
  for [`ServerConfig::require_ems`] and [`ClientConfig::require_ems`].

  See [manual::_06_fips] for more details.

- `prefer-post-quantum`: for the [`aws-lc-rs`]-backed provider, prioritizes post-quantum secure
  key exchange by default (using X25519MLKEM768).  This feature merely alters the order
  of `rustls::crypto::aws_lc_rs::DEFAULT_KX_GROUPS`.  We expect to add this feature
  to the default set in a future minor release.  See [the manual][x25519mlkem768-manual]
  for more details.

- `custom-provider`: disables implicit use of built-in providers (`aws-lc-rs` or `ring`). This forces
  applications to manually install one, for instance, when using a custom `CryptoProvider`.

- `tls12` (enabled by default): enable support for TLS version 1.2. Note that, due to the
  additive nature of Cargo features and because it is enabled by default, other crates
  in your dependency graph could re-enable it for your application. If you want to disable
  TLS 1.2 for security reasons, consider explicitly enabling TLS 1.3 only in the config
  builder API.

- `logging` (enabled by default): make the rustls crate depend on the `log` crate.
  rustls outputs interesting protocol-level messages at `trace!` and `debug!` level,
  and protocol-level errors at `warn!` and `error!` level.  The log messages do not
  contain secret key data, and so are safe to archive without affecting session security.

- `read_buf`: when building with Rust Nightly, adds support for the unstable
  `std::io::ReadBuf` and related APIs. This reduces costs from initializing
  buffers. Will do nothing on non-Nightly releases.

- `brotli`: uses the `brotli` crate for RFC8879 certificate compression support.

- `zlib`: uses the `zlib-rs` crate for RFC8879 certificate compression support.

[x25519mlkem768-manual]: manual::_05_defaults#about-the-post-quantum-secure-key-exchange-x25519mlkem768

## Modules

## Module `compress`

Certificate compression and decompression support

This crate supports compression and decompression everywhere
certificates are used, in accordance with [RFC8879][rfc8879].

Note that this is only supported for TLS1.3 connections.

# Getting started

Build this crate with the `brotli` and/or `zlib` crate features.  This
adds dependencies on these crates.  They are used by default if enabled.

We especially recommend `brotli` as it has the widest deployment so far.

# Custom compression/decompression implementations

1. Implement the [`CertCompressor`] and/or [`CertDecompressor`] traits
2. Provide those to:
  - [`ClientConfig::cert_compressors`][cc_cc] or [`ServerConfig::cert_compressors`][sc_cc].
  - [`ClientConfig::cert_decompressors`][cc_cd] or [`ServerConfig::cert_decompressors`][sc_cd].

These are used in these circumstances:

| Peer | Client authentication | Server authentication |
| ---- | --------------------- | --------------------- |
| *Client* | [`ClientConfig::cert_compressors`][cc_cc] | [`ClientConfig::cert_decompressors`][cc_cd] |
| *Server* | [`ServerConfig::cert_decompressors`][sc_cd] | [`ServerConfig::cert_compressors`][sc_cc] |

[rfc8879]: https://datatracker.ietf.org/doc/html/rfc8879
[cc_cc]: crate::ClientConfig::cert_compressors
[sc_cc]: crate::ServerConfig::cert_compressors
[cc_cd]: crate::ClientConfig::cert_decompressors
[sc_cd]: crate::ServerConfig::cert_decompressors

```rust
pub mod compress { /* ... */ }
```

### Types

#### Enum `CompressionLevel`

A hint for how many resources to dedicate to a compression.

```rust
pub enum CompressionLevel {
    Interactive,
    Amortized,
}
```

##### Variants

###### `Interactive`

This compression is happening interactively during a handshake.

Implementations may wish to choose a conservative compression level.

###### `Amortized`

The compression may be amortized over many connections.

Implementations may wish to choose an aggressive compression level.

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Send**
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CompressionLevel) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompressionLevel { /* ... */ }
    ```

- **Sync**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
#### Struct `DecompressionFailed`

A content-less error for when `CertDecompressor::decompress` fails.

```rust
pub struct DecompressionFailed;
```

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
#### Struct `CompressionFailed`

A content-less error for when `CertCompressor::compress` fails.

```rust
pub struct CompressionFailed;
```

##### Implementations

###### Trait Implementations

- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `CompressionCache`

An LRU cache for compressions.

The prospect of being able to reuse a given compression for many connections
means we can afford to spend more time on that compression (by passing
`CompressionLevel::Amortized` to the compressor).

```rust
pub enum CompressionCache {
    Disabled,
    Enabled(CompressionCacheInner),
}
```

##### Variants

###### `Disabled`

No caching happens, and compression happens each time using
`CompressionLevel::Interactive`.

###### `Enabled`

Compressions are stored in an LRU cache.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CompressionCacheInner` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(size: usize) -> Self { /* ... */ }
  ```
  Make a `CompressionCache` that stores up to `size` compressed

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Send**
- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `CompressionCacheInner`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Innards of an enabled CompressionCache.

You cannot make one of these directly. Use [`CompressionCache::new`].

```rust
pub struct CompressionCacheInner {
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
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

- **RefUnwindSafe**
- **Sync**
- **Unpin**
### Traits

#### Trait `CertDecompressor`

An available certificate decompression algorithm.

```rust
pub trait CertDecompressor: Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `decompress`: Decompress `input`, writing the result to `output`.
- `algorithm`: Which algorithm this decompressor handles.

#### Trait `CertCompressor`

An available certificate compression algorithm.

```rust
pub trait CertCompressor: Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `compress`: Compress `input`, returning the result.
- `algorithm`: Which algorithm this compressor handles.

### Functions

#### Function `default_cert_decompressors`

Returns the supported `CertDecompressor` implementations enabled
by crate features.

```rust
pub fn default_cert_decompressors() -> &''static [&''static dyn CertDecompressor] { /* ... */ }
```

#### Function `default_cert_compressors`

Returns the supported `CertCompressor` implementations enabled
by crate features.

```rust
pub fn default_cert_compressors() -> &''static [&''static dyn CertCompressor] { /* ... */ }
```

## Module `crypto`

Crypto provider interface.

```rust
pub mod crypto { /* ... */ }
```

### Modules

## Module `ring`

**Attributes:**

- `#[<cfg>(feature = "ring")]`

*ring* based CryptoProvider.

```rust
pub mod ring { /* ... */ }
```

### Modules

## Module `sign`

**Attributes:**

- `#[allow(clippy::duplicate_mod)]`

Using software keys for authentication.

```rust
pub mod sign { /* ... */ }
```

### Functions

#### Function `any_supported_type`

Parse `der` as any supported key encoding/type, returning
the first which works.

```rust
pub fn any_supported_type(der: &pki_types::PrivateKeyDer<''_>) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error> { /* ... */ }
```

#### Function `any_ecdsa_type`

Parse `der` as any ECDSA key type, returning the first which works.

Both SEC1 (PEM section starting with 'BEGIN EC PRIVATE KEY') and PKCS8
(PEM section starting with 'BEGIN PRIVATE KEY') encodings are supported.

```rust
pub fn any_ecdsa_type(der: &pki_types::PrivateKeyDer<''_>) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error> { /* ... */ }
```

#### Function `any_eddsa_type`

Parse `der` as any EdDSA key type, returning the first which works.

Note that, at the time of writing, Ed25519 does not have wide support
in browsers.  It is also not supported by the WebPKI, because the
CA/Browser Forum Baseline Requirements do not support it for publicly
trusted certificates.

```rust
pub fn any_eddsa_type(der: &pki_types::PrivatePkcs8KeyDer<''_>) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error> { /* ... */ }
```

## Module `cipher_suite`

All defined cipher suites supported by *ring* appear in this module.

```rust
pub mod cipher_suite { /* ... */ }
```

### Re-exports

#### Re-export `TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

```rust
pub use super::tls12::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256;
```

#### Re-export `TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

```rust
pub use super::tls12::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384;
```

#### Re-export `TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

```rust
pub use super::tls12::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256;
```

#### Re-export `TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

```rust
pub use super::tls12::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256;
```

#### Re-export `TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

```rust
pub use super::tls12::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384;
```

#### Re-export `TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

```rust
pub use super::tls12::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256;
```

#### Re-export `TLS13_AES_128_GCM_SHA256`

```rust
pub use super::tls13::TLS13_AES_128_GCM_SHA256;
```

#### Re-export `TLS13_AES_256_GCM_SHA384`

```rust
pub use super::tls13::TLS13_AES_256_GCM_SHA384;
```

#### Re-export `TLS13_CHACHA20_POLY1305_SHA256`

```rust
pub use super::tls13::TLS13_CHACHA20_POLY1305_SHA256;
```

## Module `kx_group`

All defined key exchange groups supported by *ring* appear in this module.

[`ALL_KX_GROUPS`] is provided as an array of all of these values.
[`DEFAULT_KX_GROUPS`] is provided as an array of this provider's defaults.

```rust
pub mod kx_group { /* ... */ }
```

### Re-exports

#### Re-export `SECP256R1`

```rust
pub use super::kx::SECP256R1;
```

#### Re-export `SECP384R1`

```rust
pub use super::kx::SECP384R1;
```

#### Re-export `X25519`

```rust
pub use super::kx::X25519;
```

### Functions

#### Function `default_provider`

A `CryptoProvider` backed by the [*ring*] crate.

[*ring*]: https://github.com/briansmith/ring

```rust
pub fn default_provider() -> crate::crypto::CryptoProvider { /* ... */ }
```

### Constants and Statics

#### Static `DEFAULT_CIPHER_SUITES`

The cipher suite configuration that an application should use by default.

This will be [`ALL_CIPHER_SUITES`] sans any supported cipher suites that
shouldn't be enabled by most applications.

```rust
pub static DEFAULT_CIPHER_SUITES: &[crate::suites::SupportedCipherSuite] = ALL_CIPHER_SUITES;
```

#### Static `ALL_CIPHER_SUITES`

A list of all the cipher suites supported by the rustls *ring* provider.

```rust
pub static ALL_CIPHER_SUITES: &[crate::suites::SupportedCipherSuite] = _;
```

#### Static `DEFAULT_KX_GROUPS`

A list of the default key exchange groups supported by this provider.

```rust
pub static DEFAULT_KX_GROUPS: &[&dyn SupportedKxGroup] = ALL_KX_GROUPS;
```

#### Static `ALL_KX_GROUPS`

A list of all the key exchange groups supported by this provider.

```rust
pub static ALL_KX_GROUPS: &[&dyn SupportedKxGroup] = _;
```

### Re-exports

#### Re-export `Ticketer`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "hashbrown"))]`

```rust
pub use ticketer::Ticketer;
```

## Module `cipher`

TLS message encryption/decryption interfaces.

```rust
pub mod cipher { /* ... */ }
```

### Types

#### Struct `UnsupportedOperationError`

An error indicating that the AEAD algorithm does not support the requested operation.

```rust
pub struct UnsupportedOperationError;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnsupportedOperationError { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: UnsupportedOperationError) -> Self { /* ... */ }
    ```

- **Error**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Copy**
- **Send**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UnsupportedOperationError) -> bool { /* ... */ }
    ```

- **Unpin**
#### Struct `KeyBlockShape`

How a TLS1.2 `key_block` is partitioned.

Note: ciphersuites with non-zero `mac_key_length` are  not currently supported.

```rust
pub struct KeyBlockShape {
    pub enc_key_len: usize,
    pub fixed_iv_len: usize,
    pub explicit_nonce_len: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enc_key_len` | `usize` | How long keys are.<br><br>`enc_key_length` terminology is from the standard ([RFC5246 A.6]).<br><br>[RFC5246 A.6]: <https://www.rfc-editor.org/rfc/rfc5246#appendix-A.6> |
| `fixed_iv_len` | `usize` | How long the fixed part of the 'IV' is.<br><br>`fixed_iv_length` terminology is from the standard ([RFC5246 A.6]).<br><br>This isn't usually an IV, but we continue the<br>terminology misuse to match the standard.<br><br>[RFC5246 A.6]: <https://www.rfc-editor.org/rfc/rfc5246#appendix-A.6> |
| `explicit_nonce_len` | `usize` | This is a non-standard extension which extends the<br>key block to provide an initial explicit nonce offset,<br>in a deterministic and safe way.  GCM needs this,<br>chacha20poly1305 works this way by design. |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Iv`

A write or read IV.

```rust
pub struct Iv(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: [u8; 12]) -> Self { /* ... */ }
  ```
  Create a new `Iv` from a byte array, of precisely `NONCE_LEN` bytes.

- ```rust
  pub fn copy(value: &[u8]) -> Self { /* ... */ }
  ```
  Create a new `Iv` from a byte slice, of precisely `NONCE_LEN` bytes.

###### Trait Implementations

- **Freeze**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Iv { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(bytes: [u8; 12]) -> Self { /* ... */ }
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

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Nonce`

A nonce.  This is unique for all messages on a connection.

```rust
pub struct Nonce(pub [u8; 12]);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[u8; 12]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(iv: &Iv, seq: u64) -> Self { /* ... */ }
  ```
  Combine an `Iv` and sequence number to produce a unique nonce.

###### Trait Implementations

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Sync**
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

- **Unpin**
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

#### Struct `AeadKey`

A key for an AEAD algorithm.

This is a value type for a byte string up to `AeadKey::MAX_LEN` bytes in length.

```rust
pub struct AeadKey {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **Send**
- **Freeze**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(bytes: [u8; 32]) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Traits

#### Trait `Tls13AeadAlgorithm`

Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.3 cipher suite.

```rust
pub trait Tls13AeadAlgorithm: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `encrypter`: Build a `MessageEncrypter` for the given key/iv.
- `decrypter`: Build a `MessageDecrypter` for the given key/iv.
- `key_len`: The length of key in bytes required by `encrypter()` and `decrypter()`.
- `extract_keys`: Convert the key material from `key`/`iv`, into a `ConnectionTrafficSecrets` item.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

#### Trait `Tls12AeadAlgorithm`

Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.2 cipher suite.

```rust
pub trait Tls12AeadAlgorithm: Send + Sync + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `encrypter`: Build a `MessageEncrypter` for the given key/iv and extra key block (which can be used for
- `decrypter`: Build a `MessageDecrypter` for the given key/iv.
- `key_block_shape`: Return a `KeyBlockShape` that defines how large the `key_block` is and how it
- `extract_keys`: Convert the key material from `key`/`iv`, into a `ConnectionTrafficSecrets` item.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

#### Trait `MessageDecrypter`

Objects with this trait can decrypt TLS messages.

```rust
pub trait MessageDecrypter: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `decrypt`: Decrypt the given TLS message `msg`, using the sequence number

#### Trait `MessageEncrypter`

Objects with this trait can encrypt TLS messages.

```rust
pub trait MessageEncrypter: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `encrypt`: Encrypt the given TLS message `msg`, using the sequence number
- `encrypted_payload_len`: Return the length of the ciphertext that results from encrypting plaintext of

### Functions

#### Function `make_tls13_aad`

**Attributes:**

- `#[inline]`

Returns a TLS1.3 `additional_data` encoding.

See RFC8446 s5.2 for the `additional_data` definition.

```rust
pub fn make_tls13_aad(payload_len: usize) -> [u8; 5] { /* ... */ }
```

#### Function `make_tls12_aad`

**Attributes:**

- `#[inline]`

Returns a TLS1.2 `additional_data` encoding.

See RFC5246 s6.2.3.3 for the `additional_data` definition.

```rust
pub fn make_tls12_aad(seq: u64, typ: crate::enums::ContentType, vers: crate::enums::ProtocolVersion, len: usize) -> [u8; 13] { /* ... */ }
```

### Constants and Statics

#### Constant `NONCE_LEN`

Size of TLS nonces (incorrectly termed "IV" in standard) for all supported ciphersuites
(AES-GCM, Chacha20Poly1305)

```rust
pub const NONCE_LEN: usize = 12;
```

### Re-exports

#### Re-export `BorrowedPayload`

```rust
pub use crate::msgs::message::BorrowedPayload;
```

#### Re-export `InboundOpaqueMessage`

```rust
pub use crate::msgs::message::InboundOpaqueMessage;
```

#### Re-export `InboundPlainMessage`

```rust
pub use crate::msgs::message::InboundPlainMessage;
```

#### Re-export `OutboundChunks`

```rust
pub use crate::msgs::message::OutboundChunks;
```

#### Re-export `OutboundOpaqueMessage`

```rust
pub use crate::msgs::message::OutboundOpaqueMessage;
```

#### Re-export `OutboundPlainMessage`

```rust
pub use crate::msgs::message::OutboundPlainMessage;
```

#### Re-export `PlainMessage`

```rust
pub use crate::msgs::message::PlainMessage;
```

#### Re-export `PrefixedPayload`

```rust
pub use crate::msgs::message::PrefixedPayload;
```

## Module `hash`

Hashing interfaces.

```rust
pub mod hash { /* ... */ }
```

### Types

#### Struct `Output`

A hash output, stored as a value.

```rust
pub struct Output {
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
  pub fn new(bytes: &[u8]) -> Self { /* ... */ }
  ```
  Build a `hash::Output` from a slice of no more than `Output::MAX_LEN` bytes.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
### Traits

#### Trait `Hash`

Describes a single cryptographic hash function.

This interface can do both one-shot and incremental hashing, using
[`Hash::hash()`] and [`Hash::start()`] respectively.

```rust
pub trait Hash: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `start`: Start an incremental hash computation.
- `hash`: Return the output of this hash function with input `data`.
- `output_len`: The length in bytes of this hash function's output.
- `algorithm`: Which hash function this is, eg, `HashAlgorithm::SHA256`.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

#### Trait `Context`

How to incrementally compute a hash.

```rust
pub trait Context: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `fork_finish`: Finish the computation, returning the resulting output.
- `fork`: Fork the computation, producing another context that has the
- `finish`: Terminate and finish the computation, returning the resulting output.
- `update`: Add `data` to computation.

### Re-exports

#### Re-export `HashAlgorithm`

```rust
pub use crate::msgs::enums::HashAlgorithm;
```

## Module `hmac`

HMAC interfaces.

```rust
pub mod hmac { /* ... */ }
```

### Types

#### Struct `Tag`

A HMAC tag, stored as a value.

```rust
pub struct Tag {
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
  pub fn new(bytes: &[u8]) -> Self { /* ... */ }
  ```
  Build a tag by copying a byte slice.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
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

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Tag { /* ... */ }
    ```

### Traits

#### Trait `Hmac`

A concrete HMAC implementation, for a single cryptographic hash function.

You should have one object that implements this trait for HMAC-SHA256, another
for HMAC-SHA384, etc.

```rust
pub trait Hmac: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `with_key`: Prepare to use `key` as a HMAC key.
- `hash_output_len`: Give the length of the underlying hash function.  In RFC2104 terminology this is `L`.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

#### Trait `Key`

A HMAC key that is ready for use.

The algorithm used is implicit in the `Hmac` object that produced the key.

```rust
pub trait Key: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `sign_concat`: Calculates a tag over the concatenation of `first`, the items in `middle`, and `last`.
- `tag_len`: Returns the length of the tag returned by a computation using

##### Provided Methods

- ```rust
  fn sign(self: &Self, data: &[&[u8]]) -> Tag { /* ... */ }
  ```
  Calculates a tag over `data` -- a slice of byte slices.

## Module `tls12`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

Cryptography specific to TLS1.2.

```rust
pub mod tls12 { /* ... */ }
```

### Types

#### Struct `PrfUsingHmac`

Implements [`Prf`] using a [`hmac::Hmac`].

```rust
pub struct PrfUsingHmac<''a>(pub &''a dyn hmac::Hmac);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a dyn hmac::Hmac` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Prf**
  - ```rust
    fn for_key_exchange(self: &Self, output: &mut [u8; 48], kx: Box<dyn ActiveKeyExchange>, peer_pub_key: &[u8], label: &[u8], seed: &[u8]) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn for_secret(self: &Self, output: &mut [u8], secret: &[u8], label: &[u8], seed: &[u8]) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
### Traits

#### Trait `Prf`

An instantiation of the TLS1.2 PRF with a specific, implicit hash function.

See the definition in [RFC5246 section 5](https://www.rfc-editor.org/rfc/rfc5246#section-5).

See [`PrfUsingHmac`] as a route to implementing this trait with just
an implementation of [`hmac::Hmac`].

```rust
pub trait Prf: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `for_key_exchange`: Computes `PRF(secret, label, seed)` using the secret from a completed key exchange.
- `for_secret`: Computes `PRF(secret, label, seed)`, writing the result into `output`.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

##### Implementations

This trait is implemented for the following types:

- `PrfUsingHmac<''_>`

## Module `tls13`

Cryptography specific to TLS1.3.

```rust
pub mod tls13 { /* ... */ }
```

### Types

#### Struct `HkdfExpanderUsingHmac`

Implementation of `HkdfExpander` via `hmac::Key`.

```rust
pub struct HkdfExpanderUsingHmac(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **HkdfExpander**
  - ```rust
    fn expand_slice(self: &Self, info: &[&[u8]], output: &mut [u8]) -> Result<(), OutputLengthError> { /* ... */ }
    ```

  - ```rust
    fn expand_block(self: &Self, info: &[&[u8]]) -> OkmBlock { /* ... */ }
    ```

  - ```rust
    fn hash_len(self: &Self) -> usize { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `HkdfUsingHmac`

Implementation of `Hkdf` (and thence `HkdfExpander`) via `hmac::Hmac`.

```rust
pub struct HkdfUsingHmac<''a>(pub &''a dyn hmac::Hmac);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a dyn hmac::Hmac` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
- **Hkdf**
  - ```rust
    fn extract_from_zero_ikm(self: &Self, salt: Option<&[u8]>) -> Box<dyn HkdfExpander> { /* ... */ }
    ```

  - ```rust
    fn extract_from_secret(self: &Self, salt: Option<&[u8]>, secret: &[u8]) -> Box<dyn HkdfExpander> { /* ... */ }
    ```

  - ```rust
    fn expander_for_okm(self: &Self, okm: &OkmBlock) -> Box<dyn HkdfExpander> { /* ... */ }
    ```

  - ```rust
    fn hmac_sign(self: &Self, key: &OkmBlock, message: &[u8]) -> hmac::Tag { /* ... */ }
    ```

#### Struct `OkmBlock`

Output key material from HKDF, as a value type.

```rust
pub struct OkmBlock {
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
  pub fn new(bytes: &[u8]) -> Self { /* ... */ }
  ```
  Build a single OKM block by copying a byte slice.

###### Trait Implementations

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
    fn clone(self: &Self) -> OkmBlock { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Unpin**
#### Struct `OutputLengthError`

An error type used for `HkdfExpander::expand_slice` when
the slice exceeds the maximum HKDF output length.

```rust
pub struct OutputLengthError;
```

##### Implementations

###### Trait Implementations

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

- **UnwindSafe**
- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
### Traits

#### Trait `HkdfExpander`

Implementation of `HKDF-Expand` with an implicitly stored and immutable `PRK`.

```rust
pub trait HkdfExpander: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `expand_slice`: `HKDF-Expand(PRK, info, L)` into a slice.
- `expand_block`: `HKDF-Expand(PRK, info, L=HashLen)` returned as a value.
- `hash_len`: Return what `HashLen` is for this instance.

##### Implementations

This trait is implemented for the following types:

- `HkdfExpanderUsingHmac`

#### Trait `Hkdf`

A HKDF implementation oriented to the needs of TLS1.3.

See [RFC5869](https://datatracker.ietf.org/doc/html/rfc5869) for the terminology
used in this definition.

You can use [`HkdfUsingHmac`] which implements this trait on top of an implementation
of [`hmac::Hmac`].

```rust
pub trait Hkdf: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `extract_from_zero_ikm`: `HKDF-Extract(salt, 0_HashLen)`
- `extract_from_secret`: `HKDF-Extract(salt, secret)`
- `expander_for_okm`: Build a `HkdfExpander` using `okm` as the secret PRK.
- `hmac_sign`: Signs `message` using `key` viewed as a HMAC key.

##### Provided Methods

- ```rust
  fn extract_from_kx_shared_secret(self: &Self, salt: Option<&[u8]>, kx: Box<dyn ActiveKeyExchange>, peer_pub_key: &[u8]) -> Result<Box<dyn HkdfExpander>, Error> { /* ... */ }
  ```
  `HKDF-Extract(salt, shared_secret)` where `shared_secret` is the result of a key exchange.

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

##### Implementations

This trait is implemented for the following types:

- `HkdfUsingHmac<''_>`

### Functions

#### Function `expand`

`HKDF-Expand(PRK, info, L)` to construct any type from a byte array.

- `PRK` is the implicit key material represented by this instance.
- `L := N`; N is the size of the byte array.
- `info` is a slice of byte slices, which should be processed sequentially
  (or concatenated if that is not possible).

This is infallible, because the set of types (and therefore their length) is known
at compile time.

```rust
pub fn expand<T, const N: usize>(expander: &dyn HkdfExpander, info: &[&[u8]]) -> T
where
    T: From<[u8; N]> { /* ... */ }
```

## Module `hpke`

Hybrid public key encryption (RFC 9180).

```rust
pub mod hpke { /* ... */ }
```

### Types

#### Struct `HpkeSuite`

An HPKE suite, specifying a key encapsulation mechanism and a symmetric cipher suite.

```rust
pub struct HpkeSuite {
    pub kem: crate::msgs::enums::HpkeKem,
    pub sym: crate::msgs::handshake::HpkeSymmetricCipherSuite,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `kem` | `crate::msgs::enums::HpkeKem` | The choice of HPKE key encapsulation mechanism. |
| `sym` | `crate::msgs::handshake::HpkeSymmetricCipherSuite` | The choice of HPKE symmetric cipher suite.<br><br>This combines a choice of authenticated encryption with additional data (AEAD) algorithm<br>and a key derivation function (KDF). |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **UnwindSafe**
- **Sync**
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

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> HpkeSuite { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HpkeSuite) -> bool { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `HpkePublicKey`

An HPKE public key.

```rust
pub struct HpkePublicKey(pub alloc::vec::Vec<u8>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::vec::Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> HpkePublicKey { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
#### Struct `HpkePrivateKey`

An HPKE private key.

```rust
pub struct HpkePrivateKey(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn secret_bytes(self: &Self) -> &[u8] { /* ... */ }
  ```
  Return the private key bytes.

###### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Unpin**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(bytes: Vec<u8>) -> Self { /* ... */ }
    ```

#### Struct `HpkeKeyPair`

An HPKE key pair, made of a matching public and private key.

```rust
pub struct HpkeKeyPair {
    pub public_key: HpkePublicKey,
    pub private_key: HpkePrivateKey,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `public_key` | `HpkePublicKey` | A HPKE public key. |
| `private_key` | `HpkePrivateKey` | A HPKE private key. |

##### Implementations

###### Trait Implementations

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **UnwindSafe**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `EncapsulatedSecret`

An encapsulated secret returned from setting up a sender or receiver context.

```rust
pub struct EncapsulatedSecret(pub alloc::vec::Vec<u8>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::vec::Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **RefUnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `Hpke`

An HPKE instance that can be used for base-mode single-shot encryption and decryption.

```rust
pub trait Hpke: Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `seal`: Seal the provided `plaintext` to the recipient public key `pub_key` with application supplied
- `setup_sealer`: Set up a sealer context for the receiver public key `pub_key` with application supplied `info`.
- `open`: Open the provided `ciphertext` using the encapsulated secret `enc`, with application
- `setup_opener`: Set up an opener context for the secret key `secret_key` with application supplied `info`.
- `generate_key_pair`: Generate a new public key and private key pair compatible with this HPKE instance.
- `suite`: Return the [HpkeSuite] that this HPKE instance supports.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return whether the HPKE instance is FIPS compatible.

#### Trait `HpkeSealer`

An HPKE sealer context.

This is a stateful object that can be used to seal messages for receipt by
a receiver.

```rust
pub trait HpkeSealer: Debug + Send + Sync + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `seal`: Seal the provided `plaintext` with additional data `aad`, returning

#### Trait `HpkeOpener`

An HPKE opener context.

This is a stateful object that can be used to open sealed messages sealed
by a sender.

```rust
pub trait HpkeOpener: Debug + Send + Sync + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `open`: Open the provided `ciphertext` with additional data `aad`, returning plaintext.

### Types

#### Struct `CryptoProvider`

Controls core cryptography used by rustls.

This crate comes with two built-in options, provided as
`CryptoProvider` structures:

- [`crypto::aws_lc_rs::default_provider`]: (behind the `aws_lc_rs` crate feature,
  which is enabled by default).  This provider uses the [aws-lc-rs](https://github.com/aws/aws-lc-rs)
  crate.  The `fips` crate feature makes this option use FIPS140-3-approved cryptography.
- [`crypto::ring::default_provider`]: (behind the `ring` crate feature, which
  is optional).  This provider uses the [*ring*](https://github.com/briansmith/ring)
  crate.

This structure provides defaults. Everything in it can be overridden at
runtime by replacing field values as needed.

# Using the per-process default `CryptoProvider`

There is the concept of an implicit default provider, configured at run-time once in
a given process.

It is used for functions like [`ClientConfig::builder()`] and [`ServerConfig::builder()`].

The intention is that an application can specify the [`CryptoProvider`] they wish to use
once, and have that apply to the variety of places where their application does TLS
(which may be wrapped inside other libraries).
They should do this by calling [`CryptoProvider::install_default()`] early on.

To achieve this goal:

- _libraries_ should use [`ClientConfig::builder()`]/[`ServerConfig::builder()`]
  or otherwise rely on the [`CryptoProvider::get_default()`] provider.
- _applications_ should call [`CryptoProvider::install_default()`] early
  in their `fn main()`. If _applications_ uses a custom provider based on the one built-in,
  they can activate the `custom-provider` feature to ensure its usage.

# Using a specific `CryptoProvider`

Supply the provider when constructing your [`ClientConfig`] or [`ServerConfig`]:

- [`ClientConfig::builder_with_provider()`]
- [`ServerConfig::builder_with_provider()`]

When creating and configuring a webpki-backed client or server certificate verifier, a choice of
provider is also needed to start the configuration process:

- [`client::WebPkiServerVerifier::builder_with_provider()`]
- [`server::WebPkiClientVerifier::builder_with_provider()`]

If you install a custom provider and want to avoid any accidental use of a built-in provider, the feature
`custom-provider` can be activated to ensure your custom provider is used everywhere
and not a built-in one. This will disable any implicit use of a built-in provider.

# Making a custom `CryptoProvider`

Your goal will be to populate an instance of this `CryptoProvider` struct.

## Which elements are required?

There is no requirement that the individual elements ([`SupportedCipherSuite`], [`SupportedKxGroup`],
[`SigningKey`], etc.) come from the same crate.  It is allowed and expected that uninteresting
elements would be delegated back to one of the default providers (statically) or a parent
provider (dynamically).

For example, if we want to make a provider that just overrides key loading in the config builder
API (with [`ConfigBuilder::with_single_cert`], etc.), it might look like this:

```
# #[cfg(feature = "aws_lc_rs")] {
# use std::sync::Arc;
# mod fictious_hsm_api { pub fn load_private_key(key_der: pki_types::PrivateKeyDer<'static>) -> ! { unreachable!(); } }
use rustls::crypto::aws_lc_rs;

pub fn provider() -> rustls::crypto::CryptoProvider {
  rustls::crypto::CryptoProvider{
    key_provider: &HsmKeyLoader,
    ..aws_lc_rs::default_provider()
  }
}

#[derive(Debug)]
struct HsmKeyLoader;

impl rustls::crypto::KeyProvider for HsmKeyLoader {
    fn load_private_key(&self, key_der: pki_types::PrivateKeyDer<'static>) -> Result<Arc<dyn rustls::sign::SigningKey>, rustls::Error> {
         fictious_hsm_api::load_private_key(key_der)
    }
}
# }
```

## References to the individual elements

The elements are documented separately:

- **Random** - see [`crypto::SecureRandom::fill()`].
- **Cipher suites** - see [`SupportedCipherSuite`], [`Tls12CipherSuite`], and
  [`Tls13CipherSuite`].
- **Key exchange groups** - see [`crypto::SupportedKxGroup`].
- **Signature verification algorithms** - see [`crypto::WebPkiSupportedAlgorithms`].
- **Authentication key loading** - see [`crypto::KeyProvider::load_private_key()`] and
  [`sign::SigningKey`].

# Example code

See custom [`provider-example/`] for a full client and server example that uses
cryptography from the [`RustCrypto`] and [`dalek-cryptography`] projects.

```shell
$ cargo run --example client | head -3
Current ciphersuite: TLS13_CHACHA20_POLY1305_SHA256
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 19899
```

[`provider-example/`]: https://github.com/rustls/rustls/tree/main/provider-example/
[`RustCrypto`]: https://github.com/RustCrypto
[`dalek-cryptography`]: https://github.com/dalek-cryptography

# FIPS-approved cryptography
The `fips` crate feature enables use of the `aws-lc-rs` crate in FIPS mode.

You can verify the configuration at runtime by checking
[`ServerConfig::fips()`]/[`ClientConfig::fips()`] return `true`.

```rust
pub struct CryptoProvider {
    pub cipher_suites: alloc::vec::Vec<suites::SupportedCipherSuite>,
    pub kx_groups: alloc::vec::Vec<&''static dyn SupportedKxGroup>,
    pub signature_verification_algorithms: WebPkiSupportedAlgorithms,
    pub secure_random: &''static dyn SecureRandom,
    pub key_provider: &''static dyn KeyProvider,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `cipher_suites` | `alloc::vec::Vec<suites::SupportedCipherSuite>` | List of supported ciphersuites, in preference order -- the first element<br>is the highest priority.<br><br>The `SupportedCipherSuite` type carries both configuration and implementation.<br><br>A valid `CryptoProvider` must ensure that all cipher suites are accompanied by at least<br>one matching key exchange group in [`CryptoProvider::kx_groups`]. |
| `kx_groups` | `alloc::vec::Vec<&''static dyn SupportedKxGroup>` | List of supported key exchange groups, in preference order -- the<br>first element is the highest priority.<br><br>The first element in this list is the _default key share algorithm_,<br>and in TLS1.3 a key share for it is sent in the client hello.<br><br>The `SupportedKxGroup` type carries both configuration and implementation. |
| `signature_verification_algorithms` | `WebPkiSupportedAlgorithms` | List of signature verification algorithms for use with webpki.<br><br>These are used for both certificate chain verification and handshake signature verification.<br><br>This is called by [`ConfigBuilder::with_root_certificates()`],<br>[`server::WebPkiClientVerifier::builder_with_provider()`] and<br>[`client::WebPkiServerVerifier::builder_with_provider()`]. |
| `secure_random` | `&''static dyn SecureRandom` | Source of cryptographically secure random numbers. |
| `key_provider` | `&''static dyn KeyProvider` | Provider for loading private [`SigningKey`]s from [`PrivateKeyDer`]. |

##### Implementations

###### Methods

- ```rust
  pub fn install_default(self: Self) -> Result<(), alloc::sync::Arc<Self>> { /* ... */ }
  ```
  Sets this `CryptoProvider` as the default for this process.

- ```rust
  pub fn get_default() -> Option<&''static alloc::sync::Arc<Self>> { /* ... */ }
  ```
  Returns the default `CryptoProvider` for this process.

- ```rust
  pub fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this `CryptoProvider` is operating in FIPS mode.

###### Trait Implementations

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CryptoProvider { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `CompletedKeyExchange`

The result from [`SupportedKxGroup::start_and_complete()`].

```rust
pub struct CompletedKeyExchange {
    pub group: crate::NamedGroup,
    pub pub_key: alloc::vec::Vec<u8>,
    pub secret: SharedSecret,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `group` | `crate::NamedGroup` | Which group was used. |
| `pub_key` | `alloc::vec::Vec<u8>` | Our key share (sometimes a public key). |
| `secret` | `SharedSecret` | The computed shared secret. |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Send**
- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `SharedSecret`

The result from [`ActiveKeyExchange::complete`] or [`ActiveKeyExchange::complete_hybrid_component`].

```rust
pub struct SharedSecret {
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
  pub fn secret_bytes(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns the shared secret as a slice of bytes.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: &[u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(buf: Vec<u8>) -> Self { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

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

- **Send**
- **UnwindSafe**
- **Unpin**
- **Sync**
### Traits

#### Trait `SecureRandom`

A source of cryptographically secure randomness.

```rust
pub trait SecureRandom: Send + Sync + Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `fill`: Fill the given buffer with random bytes.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

#### Trait `KeyProvider`

A mechanism for loading private [`SigningKey`]s from [`PrivateKeyDer`].

This trait is intended to be used with private key material that is sourced from DER,
such as a private-key that may be present on-disk. It is not intended to be used with
keys held in hardware security modules (HSMs) or physical tokens. For these use-cases
see the Rustls manual section on [customizing private key usage].

[customizing private key usage]: <https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#customising-private-key-usage>

```rust
pub trait KeyProvider: Send + Sync + Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `load_private_key`: Decode and validate a private signing key from `key_der`.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

#### Trait `SupportedKxGroup`

A supported key exchange group.

This type carries both configuration and implementation. Specifically,
it has a TLS-level name expressed using the [`NamedGroup`] enum, and
a function which produces a [`ActiveKeyExchange`].

Compare with [`NamedGroup`], which carries solely a protocol identifier.

```rust
pub trait SupportedKxGroup: Send + Sync + Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `start`: Start a key exchange.
- `name`: Named group the SupportedKxGroup operates in.

##### Provided Methods

- ```rust
  fn start_and_complete(self: &Self, peer_pub_key: &[u8]) -> Result<CompletedKeyExchange, Error> { /* ... */ }
  ```
  Start and complete a key exchange, in one operation.

- ```rust
  fn ffdhe_group(self: &Self) -> Option<FfdheGroup<''static>> { /* ... */ }
  ```
  FFDHE group the `SupportedKxGroup` operates in.

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

- ```rust
  fn usable_for_version(self: &Self, _version: ProtocolVersion) -> bool { /* ... */ }
  ```
  Return `true` if this should be offered/selected with the given version.

#### Trait `ActiveKeyExchange`

An in-progress key exchange originating from a [`SupportedKxGroup`].

```rust
pub trait ActiveKeyExchange: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `complete`: Completes the key exchange, given the peer's public key.
- `pub_key`: Return the public key being used.
- `group`: Return the group being used.

##### Provided Methods

- ```rust
  fn complete_for_tls_version(self: Box<Self>, peer_pub_key: &[u8], tls_version: &SupportedProtocolVersion) -> Result<SharedSecret, Error> { /* ... */ }
  ```
  Completes the key exchange for the given TLS version, given the peer's public key.

- ```rust
  fn hybrid_component(self: &Self) -> Option<(NamedGroup, &[u8])> { /* ... */ }
  ```
  For hybrid key exchanges, returns the [`NamedGroup`] and key share

- ```rust
  fn complete_hybrid_component(self: Box<Self>, _peer_pub_key: &[u8]) -> Result<SharedSecret, Error> { /* ... */ }
  ```
  Completes the classical component of the key exchange, given the peer's public key.

- ```rust
  fn ffdhe_group(self: &Self) -> Option<FfdheGroup<''static>> { /* ... */ }
  ```
  FFDHE group the `ActiveKeyExchange` is operating in.

### Re-exports

#### Re-export `WebPkiSupportedAlgorithms`

```rust
pub use crate::webpki::WebPkiSupportedAlgorithms;
```

#### Re-export `verify_tls12_signature`

```rust
pub use crate::webpki::verify_tls12_signature;
```

#### Re-export `verify_tls13_signature`

```rust
pub use crate::webpki::verify_tls13_signature;
```

#### Re-export `verify_tls13_signature_with_raw_key`

```rust
pub use crate::webpki::verify_tls13_signature_with_raw_key;
```

#### Re-export `KeyExchangeAlgorithm`

```rust
pub use crate::msgs::handshake::KeyExchangeAlgorithm;
```

#### Re-export `GetRandomFailed`

```rust
pub use crate::rand::GetRandomFailed;
```

#### Re-export `CipherSuiteCommon`

```rust
pub use crate::suites::CipherSuiteCommon;
```

## Module `unbuffered`

Unbuffered connection API

This is an alternative to the [`crate::ConnectionCommon`] API that does not internally buffer
TLS nor plaintext data. Instead those buffers are managed by the API user so they have
control over when and how to allocate, resize and dispose of them.

This API is lower level than the `ConnectionCommon` API and is built around a state machine
interface where the API user must handle each state to advance and complete the
handshake process.

Like the `ConnectionCommon` API, no IO happens internally so all IO must be handled by the API
user. Unlike the `ConnectionCommon` API, this API does not make use of the [`std::io::Read`] and
[`std::io::Write`] traits so it's usable in no-std context.

The entry points into this API are [`crate::client::UnbufferedClientConnection::new`],
[`crate::server::UnbufferedServerConnection::new`] and
[`unbuffered::UnbufferedConnectionCommon::process_tls_records`]. The state machine API is
documented in [`unbuffered::ConnectionState`].

# Examples

[`unbuffered-client`] and [`unbuffered-server`] are examples that fully exercise the API in
std, non-async context.

[`unbuffered-client`]: https://github.com/rustls/rustls/blob/main/examples/src/bin/unbuffered-client.rs
[`unbuffered-server`]: https://github.com/rustls/rustls/blob/main/examples/src/bin/unbuffered-server.rs

```rust
pub mod unbuffered { /* ... */ }
```

### Re-exports

#### Re-export `UnbufferedConnectionCommon`

```rust
pub use crate::conn::UnbufferedConnectionCommon;
```

#### Re-export `AppDataRecord`

```rust
pub use crate::conn::unbuffered::AppDataRecord;
```

#### Re-export `ConnectionState`

```rust
pub use crate::conn::unbuffered::ConnectionState;
```

#### Re-export `EncodeError`

```rust
pub use crate::conn::unbuffered::EncodeError;
```

#### Re-export `EncodeTlsData`

```rust
pub use crate::conn::unbuffered::EncodeTlsData;
```

#### Re-export `EncryptError`

```rust
pub use crate::conn::unbuffered::EncryptError;
```

#### Re-export `InsufficientSizeError`

```rust
pub use crate::conn::unbuffered::InsufficientSizeError;
```

#### Re-export `ReadEarlyData`

```rust
pub use crate::conn::unbuffered::ReadEarlyData;
```

#### Re-export `ReadTraffic`

```rust
pub use crate::conn::unbuffered::ReadTraffic;
```

#### Re-export `TransmitTlsData`

```rust
pub use crate::conn::unbuffered::TransmitTlsData;
```

#### Re-export `UnbufferedStatus`

```rust
pub use crate::conn::unbuffered::UnbufferedStatus;
```

#### Re-export `WriteTraffic`

```rust
pub use crate::conn::unbuffered::WriteTraffic;
```

## Module `client`

Items for use in a client.

```rust
pub mod client { /* ... */ }
```

### Modules

## Module `danger`

Dangerous configuration that should be audited and used with extreme care.

```rust
pub mod danger { /* ... */ }
```

### Re-exports

#### Re-export `DangerousClientConfigBuilder`

```rust
pub use super::builder::danger::DangerousClientConfigBuilder;
```

#### Re-export `DangerousClientConfig`

```rust
pub use super::client_conn::danger::DangerousClientConfig;
```

#### Re-export `HandshakeSignatureValid`

```rust
pub use crate::verify::HandshakeSignatureValid;
```

#### Re-export `ServerCertVerified`

```rust
pub use crate::verify::ServerCertVerified;
```

#### Re-export `ServerCertVerifier`

```rust
pub use crate::verify::ServerCertVerifier;
```

### Re-exports

#### Re-export `WantsClientCert`

```rust
pub use builder::WantsClientCert;
```

#### Re-export `ClientConfig`

```rust
pub use client_conn::ClientConfig;
```

#### Re-export `ClientConnectionData`

```rust
pub use client_conn::ClientConnectionData;
```

#### Re-export `ClientSessionStore`

```rust
pub use client_conn::ClientSessionStore;
```

#### Re-export `EarlyDataError`

```rust
pub use client_conn::EarlyDataError;
```

#### Re-export `ResolvesClientCert`

```rust
pub use client_conn::ResolvesClientCert;
```

#### Re-export `Resumption`

```rust
pub use client_conn::Resumption;
```

#### Re-export `Tls12Resumption`

```rust
pub use client_conn::Tls12Resumption;
```

#### Re-export `UnbufferedClientConnection`

```rust
pub use client_conn::UnbufferedClientConnection;
```

#### Re-export `ClientConnection`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use client_conn::ClientConnection;
```

#### Re-export `WriteEarlyData`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use client_conn::WriteEarlyData;
```

#### Re-export `EchConfig`

```rust
pub use ech::EchConfig;
```

#### Re-export `EchGreaseConfig`

```rust
pub use ech::EchGreaseConfig;
```

#### Re-export `EchMode`

```rust
pub use ech::EchMode;
```

#### Re-export `EchStatus`

```rust
pub use ech::EchStatus;
```

#### Re-export `AlwaysResolvesClientRawPublicKeys`

```rust
pub use handy::AlwaysResolvesClientRawPublicKeys;
```

#### Re-export `ClientSessionMemoryCache`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "hashbrown"))]`

```rust
pub use handy::ClientSessionMemoryCache;
```

#### Re-export `Tls12ClientSessionValue`

```rust
pub use crate::msgs::persist::Tls12ClientSessionValue;
```

#### Re-export `Tls13ClientSessionValue`

```rust
pub use crate::msgs::persist::Tls13ClientSessionValue;
```

#### Re-export `ServerCertVerifierBuilder`

```rust
pub use crate::webpki::ServerCertVerifierBuilder;
```

#### Re-export `VerifierBuilderError`

```rust
pub use crate::webpki::VerifierBuilderError;
```

#### Re-export `WebPkiServerVerifier`

```rust
pub use crate::webpki::WebPkiServerVerifier;
```

#### Re-export `verify_server_cert_signed_by_trust_anchor`

```rust
pub use crate::webpki::verify_server_cert_signed_by_trust_anchor;
```

#### Re-export `verify_server_name`

```rust
pub use crate::webpki::verify_server_name;
```

## Module `server`

Items for use in a server.

```rust
pub mod server { /* ... */ }
```

### Modules

## Module `danger`

Dangerous configuration that should be audited and used with extreme care.

```rust
pub mod danger { /* ... */ }
```

### Re-exports

#### Re-export `ClientCertVerified`

```rust
pub use crate::verify::ClientCertVerified;
```

#### Re-export `ClientCertVerifier`

```rust
pub use crate::verify::ClientCertVerifier;
```

### Re-exports

#### Re-export `WantsServerCert`

```rust
pub use builder::WantsServerCert;
```

#### Re-export `ResolvesServerCertUsingSni`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "hashbrown"))]`

```rust
pub use handy::ResolvesServerCertUsingSni;
```

#### Re-export `ServerSessionMemoryCache`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "hashbrown"))]`

```rust
pub use handy::ServerSessionMemoryCache;
```

#### Re-export `AlwaysResolvesServerRawPublicKeys`

```rust
pub use handy::AlwaysResolvesServerRawPublicKeys;
```

#### Re-export `NoServerSessionStorage`

```rust
pub use handy::NoServerSessionStorage;
```

#### Re-export `Accepted`

```rust
pub use server_conn::Accepted;
```

#### Re-export `ClientHello`

```rust
pub use server_conn::ClientHello;
```

#### Re-export `ProducesTickets`

```rust
pub use server_conn::ProducesTickets;
```

#### Re-export `ResolvesServerCert`

```rust
pub use server_conn::ResolvesServerCert;
```

#### Re-export `ServerConfig`

```rust
pub use server_conn::ServerConfig;
```

#### Re-export `ServerConnectionData`

```rust
pub use server_conn::ServerConnectionData;
```

#### Re-export `StoresServerSessions`

```rust
pub use server_conn::StoresServerSessions;
```

#### Re-export `UnbufferedServerConnection`

```rust
pub use server_conn::UnbufferedServerConnection;
```

#### Re-export `AcceptedAlert`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use server_conn::AcceptedAlert;
```

#### Re-export `Acceptor`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use server_conn::Acceptor;
```

#### Re-export `ReadEarlyData`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use server_conn::ReadEarlyData;
```

#### Re-export `ServerConnection`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use server_conn::ServerConnection;
```

#### Re-export `NoClientAuth`

```rust
pub use crate::verify::NoClientAuth;
```

#### Re-export `ClientCertVerifierBuilder`

```rust
pub use crate::webpki::ClientCertVerifierBuilder;
```

#### Re-export `ParsedCertificate`

```rust
pub use crate::webpki::ParsedCertificate;
```

#### Re-export `VerifierBuilderError`

```rust
pub use crate::webpki::VerifierBuilderError;
```

#### Re-export `WebPkiClientVerifier`

```rust
pub use crate::webpki::WebPkiClientVerifier;
```

## Module `version`

All defined protocol versions appear in this module.

ALL_VERSIONS is a provided as an array of all of these values.

```rust
pub mod version { /* ... */ }
```

### Re-exports

#### Re-export `TLS12`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

```rust
pub use crate::versions::TLS12;
```

#### Re-export `TLS13`

```rust
pub use crate::versions::TLS13;
```

## Module `pki_types`

Re-exports the contents of the [rustls-pki-types](https://docs.rs/rustls-pki-types) crate for easy access

```rust
pub mod pki_types { /* ... */ }
```

### Re-exports

#### Re-export `pki_types::*`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use pki_types::*;
```

## Module `sign`

Message signing interfaces.

```rust
pub mod sign { /* ... */ }
```

### Re-exports

#### Re-export `CertifiedKey`

```rust
pub use crate::crypto::signer::CertifiedKey;
```

#### Re-export `Signer`

```rust
pub use crate::crypto::signer::Signer;
```

#### Re-export `SigningKey`

```rust
pub use crate::crypto::signer::SigningKey;
```

#### Re-export `SingleCertAndKey`

```rust
pub use crate::crypto::signer::SingleCertAndKey;
```

## Module `quic`

APIs for implementing QUIC TLS

```rust
pub mod quic { /* ... */ }
```

### Types

#### Struct `Secrets`

Secrets used to encrypt/decrypt traffic

```rust
pub struct Secrets {
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
  pub fn next_packet_keys(self: &mut Self) -> PacketKeySet { /* ... */ }
  ```
  Derive the next set of packet keys

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Secrets { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `DirectionalKeys`

Keys used to communicate in a single direction

```rust
pub struct DirectionalKeys {
    pub header: alloc::boxed::Box<dyn HeaderProtectionKey>,
    pub packet: alloc::boxed::Box<dyn PacketKey>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `header` | `alloc::boxed::Box<dyn HeaderProtectionKey>` | Encrypts or decrypts a packet's headers |
| `packet` | `alloc::boxed::Box<dyn PacketKey>` | Encrypts or decrypts the payload of a packet |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Tag`

Authentication tag from an AEAD seal operation.

```rust
pub struct Tag(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
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
    fn from(value: &[u8]) -> Self { /* ... */ }
    ```

#### Struct `PacketKeySet`

Packet protection keys for bidirectional 1-RTT communication

```rust
pub struct PacketKeySet {
    pub local: alloc::boxed::Box<dyn PacketKey>,
    pub remote: alloc::boxed::Box<dyn PacketKey>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `local` | `alloc::boxed::Box<dyn PacketKey>` | Encrypts outgoing packets |
| `remote` | `alloc::boxed::Box<dyn PacketKey>` | Decrypts incoming packets |

##### Implementations

###### Trait Implementations

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
#### Struct `Suite`

Produces QUIC initial keys from a TLS 1.3 ciphersuite and a QUIC key generation algorithm.

```rust
pub struct Suite {
    pub suite: &''static crate::tls13::Tls13CipherSuite,
    pub quic: &''static dyn Algorithm,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `suite` | `&''static crate::tls13::Tls13CipherSuite` | The TLS 1.3 ciphersuite used to derive keys. |
| `quic` | `&''static dyn Algorithm` | The QUIC key generation algorithm used to derive keys. |

##### Implementations

###### Methods

- ```rust
  pub fn keys(self: &Self, client_dst_connection_id: &[u8], side: Side, version: Version) -> Keys { /* ... */ }
  ```
  Produce a set of initial keys given the connection ID, side and version

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Suite { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Unpin**
#### Struct `Keys`

Complete set of keys used to communicate with the peer

```rust
pub struct Keys {
    pub local: DirectionalKeys,
    pub remote: DirectionalKeys,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `local` | `DirectionalKeys` | Encrypts outgoing packets |
| `remote` | `DirectionalKeys` | Decrypts incoming packets |

##### Implementations

###### Methods

- ```rust
  pub fn initial(version: Version, suite: &''static Tls13CipherSuite, quic: &''static dyn Algorithm, client_dst_connection_id: &[u8], side: Side) -> Self { /* ... */ }
  ```
  Construct keys for use with initial packets

###### Trait Implementations

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

- **Unpin**
- **RefUnwindSafe**
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
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
#### Enum `KeyChange`

Key material for use in QUIC packet spaces

QUIC uses 4 different sets of keys (and progressive key updates for long-running connections):

* Initial: these can be created from [`Keys::initial()`]
* 0-RTT keys: can be retrieved from [`ConnectionCommon::zero_rtt_keys()`]
* Handshake: these are returned from [`ConnectionCommon::write_hs()`] after `ClientHello` and
  `ServerHello` messages have been exchanged
* 1-RTT keys: these are returned from [`ConnectionCommon::write_hs()`] after the handshake is done

Once the 1-RTT keys have been exchanged, either side may initiate a key update. Progressive
update keys can be obtained from the [`Secrets`] returned in [`KeyChange::OneRtt`]. Note that
only packet keys are updated by key updates; header protection keys remain the same.

```rust
pub enum KeyChange {
    Handshake {
        keys: Keys,
    },
    OneRtt {
        keys: Keys,
        next: Secrets,
    },
}
```

##### Variants

###### `Handshake`

Keys for the handshake space

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `keys` | `Keys` | Header and packet keys for the handshake space |

###### `OneRtt`

Keys for 1-RTT data

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `keys` | `Keys` | Header and packet keys for 1-RTT data |
| `next` | `Secrets` | Secrets to derive updated keys from |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `Version`

**Attributes:**

- `#[non_exhaustive]`

QUIC protocol version

Governs version-specific behavior in the TLS layer

```rust
pub enum Version {
    V1Draft,
    V1,
    V2,
}
```

##### Variants

###### `V1Draft`

Draft versions 29, 30, 31 and 32

###### `V1`

First stable RFC

###### `V2`

Anti-ossification variant of V1

##### Implementations

###### Trait Implementations

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

- **Send**
- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Version { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `Algorithm`

How a `Tls13CipherSuite` generates `PacketKey`s and `HeaderProtectionKey`s.

```rust
pub trait Algorithm: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `packet_key`: Produce a `PacketKey` encrypter/decrypter for this suite.
- `header_protection_key`: Produce a `HeaderProtectionKey` encrypter/decrypter for this suite.
- `aead_key_len`: The length in bytes of keys for this Algorithm.

##### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Whether this algorithm is FIPS-approved.

#### Trait `HeaderProtectionKey`

A QUIC header protection key

```rust
pub trait HeaderProtectionKey: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `encrypt_in_place`: Adds QUIC Header Protection.
- `decrypt_in_place`: Removes QUIC Header Protection.
- `sample_len`: Expected sample length for the key's algorithm

#### Trait `PacketKey`

Keys to encrypt or decrypt the payload of a packet

```rust
pub trait PacketKey: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `encrypt_in_place`: Encrypt a QUIC packet
- `decrypt_in_place`: Decrypt a QUIC packet
- `tag_len`: Tag length for the underlying AEAD algorithm
- `confidentiality_limit`: Number of QUIC messages that can be safely encrypted with a single key of this type.
- `integrity_limit`: Number of QUIC messages that can be safely decrypted with a single key of this type

### Re-exports

#### Re-export `ClientConnection`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use connection::ClientConnection;
```

#### Re-export `Connection`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use connection::Connection;
```

#### Re-export `ConnectionCommon`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use connection::ConnectionCommon;
```

#### Re-export `ServerConnection`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use connection::ServerConnection;
```

## Module `ticketer`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "hashbrown"))]`

APIs for implementing TLS tickets

```rust
pub mod ticketer { /* ... */ }
```

### Types

#### Struct `TicketSwitcher`

**Attributes:**

- `#[<cfg_attr>(feature = "std", derive(Debug))]`

A ticketer that has a 'current' sub-ticketer and a single
'previous' ticketer.  It creates a new ticketer every so
often, demoting the current ticketer.

```rust
pub struct TicketSwitcher {
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
  pub fn new(lifetime: u32, generator: fn() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed>) -> Result<Self, Error> { /* ... */ }
  ```
  Creates a new `TicketSwitcher`, which rotates through sub-ticketers

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
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
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ProducesTickets**
  - ```rust
    fn lifetime(self: &Self) -> u32 { /* ... */ }
    ```

  - ```rust
    fn enabled(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn encrypt(self: &Self, message: &[u8]) -> Option<Vec<u8>> { /* ... */ }
    ```

  - ```rust
    fn decrypt(self: &Self, ciphertext: &[u8]) -> Option<Vec<u8>> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `TicketRotator`

**Attributes:**

- `#[<cfg>(feature = "std")]`

A ticketer that has a 'current' sub-ticketer and a single
'previous' ticketer.  It creates a new ticketer every so
often, demoting the current ticketer.

```rust
pub struct TicketRotator {
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
  pub fn new(lifetime: u32, generator: fn() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed>) -> Result<Self, Error> { /* ... */ }
  ```
  Creates a new `TicketRotator`, which rotates through sub-ticketers

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ProducesTickets**
  - ```rust
    fn lifetime(self: &Self) -> u32 { /* ... */ }
    ```

  - ```rust
    fn enabled(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn encrypt(self: &Self, message: &[u8]) -> Option<Vec<u8>> { /* ... */ }
    ```

  - ```rust
    fn decrypt(self: &Self, ciphertext: &[u8]) -> Option<Vec<u8>> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
## Module `manual`

**Attributes:**

- `#[allow(non_snake_case)]`

 This is the rustls manual.

This documentation primarily aims to explain design decisions taken in rustls.

It does this from a few aspects: how rustls attempts to avoid construction errors
that occurred in other TLS libraries, how rustls attempts to avoid past TLS
protocol vulnerabilities, and assorted advice for achieving common tasks with rustls.

```rust
pub mod manual { /* ... */ }
```

### Modules

## Module `_01_impl_vulnerabilities`

**Attributes:**

- `#[path = "implvulns.rs"]`

 This section discusses vulnerabilities in other TLS implementations, theorising their
 root cause and how we aim to avoid them in rustls.
 # A review of TLS Implementation Vulnerabilities

An important part of engineering involves studying and learning from the mistakes of the past.
It would be tremendously unfortunate to spend effort re-discovering and re-fixing the same
vulnerabilities that were discovered in the past.

## Memory safety

Being written entirely in the safe-subset of Rust immediately offers us freedom from the entire
class of memory safety vulnerabilities.  There are too many to exhaustively list, and there will
certainly be more in the future.

Examples:

- Heartbleed [CVE-2014-0160](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2014-0160) (OpenSSL)
- Memory corruption in ASN.1 decoder [CVE-2016-2108](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2016-2108) (OpenSSL)
- Buffer overflow in read_server_hello [CVE-2014-3466](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2014-3466) (GnuTLS)

## `goto fail`

This is the name of a vulnerability in Apple Secure Transport [CVE-2014-1266](https://nvd.nist.gov/vuln/detail/CVE-2014-1266).
This boiled down to the following code, which validates the server's signature on the key exchange:

```c
    if ((err = SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
        goto fail;
    if ((err = SSLHashSHA1.update(&hashCtx, &signedParams)) != 0)
        goto fail;
>       goto fail;
    if ((err = SSLHashSHA1.final(&hashCtx, &hashOut)) != 0)
        goto fail;
```

The marked line was duplicated, likely accidentally during a merge.  This meant
the remaining part of the function (including the actual signature validation)
was unconditionally skipped.

Ultimately the one countermeasure to this type of bug is basic testing: that a
valid signature returns success, and that an invalid one does not.  rustls
has such testing, but this is really table stakes for security code.

Further than this, though, we could consider that the *lack* of an error from
this function is a poor indicator that the signature was valid.  rustls, instead,
has zero-size and non-copyable types that indicate a particular signature validation
has been performed.  These types can be thought of as *capabilities* originated only
by designated signature verification functions -- such functions can then be a focus
of manual code review.  Like capabilities, values of these types are otherwise unforgeable,
and are communicable only by Rust's move semantics.

Values of these types are threaded through the protocol state machine, leading to terminal
states that look like:

```ignore
struct ExpectTraffic {
   (...)
    _cert_verified: verify::ServerCertVerified,
    _sig_verified: verify::HandshakeSignatureValid,
    _fin_verified: verify::FinishedMessageVerified,
}
```

Since this state requires a value of these types, it will be a compile-time error to
reach that state without performing the requisite security-critical operations.

This approach is not infallible, but it has zero runtime cost.

## State machine attacks: EarlyCCS and SMACK/SKIP/FREAK

EarlyCCS [CVE-2014-0224](https://nvd.nist.gov/vuln/detail/CVE-2014-0224) was a vulnerability in OpenSSL
found in 2014.  The TLS `ChangeCipherSpec` message would be processed at inappropriate times, leading
to data being encrypted with the wrong keys (specifically, keys which were not secret).  This resulted
from OpenSSL taking a *reactive* strategy to incoming messages ("when I get a message X, I should do Y")
which allows it to diverge from the proper state machine under attacker control.

[SMACK](https://mitls.org/pages/attacks/SMACK) is a similar suite of vulnerabilities found in JSSE,
CyaSSL, OpenSSL, Mono and axTLS.  "SKIP-TLS" demonstrated that some implementations allowed handshake
messages (and in one case, the entire handshake!) to be skipped leading to breaks in security.  "FREAK"
found that some implementations incorrectly allowed export-only state transitions (i.e., transitions that
were only valid when an export ciphersuite was in use).

rustls represents its protocol state machine carefully to avoid these defects.  We model the handshake,
CCS and application data subprotocols in the same single state machine.  Each state in this machine is
represented with a single struct, and transitions are modelled as functions that consume the current state
plus one TLS message[^1] and return a struct representing the next state.  These functions fully validate
the message type before further operations.

A sample sequence for a full TLSv1.2 handshake by a client looks like:

- `hs::ExpectServerHello` (Note: ClientHello is logically sent before this state); transition to `tls12::ExpectCertificate`
- `tls12::ExpectCertificate`; transition to `tls12::ExpectServerKX`
- `tls12::ExpectServerKX`; transition to `tls12::ExpectServerDoneOrCertReq`
- `tls12::ExpectServerDoneOrCertReq`; delegates to `tls12::ExpectCertificateRequest` or `tls12::ExpectServerDone` depending on incoming message.
  - `tls12::ExpectServerDone`; transition to `tls12::ExpectCCS`
- `tls12::ExpectCCS`; transition to `tls12::ExpectFinished`
- `tls12::ExpectFinished`; transition to `tls12::ExpectTraffic`
- `tls12::ExpectTraffic`; terminal state; transitions to `tls12::ExpectTraffic`

In the future we plan to formally prove that all possible transitions modelled in this system of types
are correct with respect to the standard(s).  At the moment we rely merely on exhaustive testing.

[^1]: a logical TLS message: post-decryption, post-fragmentation.



```rust
pub mod _01_impl_vulnerabilities { /* ... */ }
```

## Module `_02_tls_vulnerabilities`

**Attributes:**

- `#[path = "tlsvulns.rs"]`

 This section discusses vulnerabilities and design errors in the TLS protocol.
 # A review of protocol vulnerabilities

## CBC MAC-then-encrypt ciphersuites

Back in 2000 [Bellare and Namprempre](https://eprint.iacr.org/2000/025) discussed how to make authenticated
encryption by composing separate encryption and authentication primitives.  That paper included this table:

| Composition Method | Privacy | | | Integrity | |
|--------------------|---------|-|-|-----------|-|
|| IND-CPA | IND-CCA | NM-CPA | INT-PTXT | INT-CTXT |
| Encrypt-and-MAC | insecure | insecure | insecure | secure | insecure |
| MAC-then-encrypt | secure | insecure | insecure | secure | insecure |
| Encrypt-then-MAC | secure | secure | secure | secure | secure |

One may assume from this fairly clear result that encrypt-and-MAC and MAC-then-encrypt compositions would be quickly abandoned
in favour of the remaining proven-secure option.  But that didn't happen, not in TLSv1.1 (2006) nor in TLSv1.2 (2008).  Worse,
both RFCs included incorrect advice on countermeasures for implementers, suggesting that the flaw was "not believed to be large
enough to be exploitable".

[Lucky 13](http://www.isg.rhul.ac.uk/tls/Lucky13.html) (2013) exploited this flaw and affected all implementations, including
those written [after discovery](https://aws.amazon.com/blogs/security/s2n-and-lucky-13/). OpenSSL even had a
[memory safety vulnerability in the fix for Lucky 13](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2016-2107), which
gives a flavour of the kind of complexity required to remove the side channel.

rustls does not implement CBC MAC-then-encrypt ciphersuites for these reasons.  TLSv1.3 removed support for these
ciphersuites in 2018.

There are some further rejected options worth mentioning: [RFC7366](https://tools.ietf.org/html/rfc7366) defines
Encrypt-then-MAC for TLS, but unfortunately cannot be negotiated without also supporting MAC-then-encrypt
(clients cannot express "I offer CBC, but only EtM and not MtE").

## RSA PKCS#1 encryption

"RSA key exchange" in TLS involves the client choosing a large random value and encrypting it using the server's
public key.  This has two overall problems:

1. It provides no _forward secrecy_: later compromise of the server's private key breaks confidentiality of
   *all* past sessions using that key.  This is a crucial property in the presence of software that is often
   [poor at keeping a secret](http://heartbleed.com/).
2. The padding used in practice in TLS ("PKCS#1", or fully "RSAES-PKCS1-v1_5") has been known to be broken since
   [1998](http://archiv.infsec.ethz.ch/education/fs08/secsem/bleichenbacher98.pdf).

In a similar pattern to the MAC-then-encrypt problem discussed above, TLSv1.0 (1999), TLSv1.1 (2006) and TLSv1.2 (2008)
continued to specify use of PKCS#1 encryption, again with incrementally more complex and incorrect advice on countermeasures.

[ROBOT](https://robotattack.org/) (2018) showed that implementations were still vulnerable to these attacks twenty years later.
[The Marvin Attack](https://people.redhat.com/~hkario/marvin/) (2023) demonstrated the same a further five years later.

rustls does not support RSA key exchange.  TLSv1.3 also removed support.

## BEAST

[BEAST](https://vnhacker.blogspot.com/2011/09/beast.html) ([CVE-2011-3389](https://nvd.nist.gov/vuln/detail/CVE-2011-3389))
was demonstrated in 2011 by Thai Duong and Juliano Rizzo,
and was another vulnerability in CBC-based ciphersuites in SSLv3.0 and TLSv1.0.  CBC mode is vulnerable to adaptive
chosen-plaintext attacks if the IV is predictable.  In the case of these protocol versions, the IV was the previous
block of ciphertext (as if the entire TLS session was one CBC ciphertext, albeit revealed incrementally).  This was
obviously predictable, since it was published on the wire.

OpenSSL contained a countermeasure for this problem from 2002 onwards: it encrypts an empty message before each real
one, so that the IV used in the real message is unpredictable.  This was turned off by default due to bugs in IE6.

TLSv1.1 fix this vulnerability, but not any of the other deficiencies of CBC mode (see above).

rustls does not support these ciphersuites.

## CRIME

In 2002 [John Kelsey](https://www.iacr.org/cryptodb/archive/2002/FSE/3091/3091.pdf) discussed the length side channel
as applied to compression of combined secret and attacker-chosen strings.

Compression continued to be an option in TLSv1.1 (2006) and in TLSv1.2 (2008).  Support in libraries was widespread.

[CRIME](https://en.wikipedia.org/wiki/CRIME) ([CVE-2012-4929](https://nvd.nist.gov/vuln/detail/CVE-2012-4929))
was demonstrated in 2012, again by Thai Duong and Juliano Rizzo.  It attacked several protocols offering transparent
compression of application data, allowing quick adaptive chosen-plaintext attacks against secret values like cookies.

rustls does not implement compression.  TLSv1.3 also removed support.

## Logjam / FREAK

Way back when SSL was first being born, circa 1995, the US government considered cryptography a munition requiring
export control.  SSL contained specific ciphersuites with dramatically small key sizes that were not subject
to export control.  These controls were dropped in 2000.

Since the "export-grade" ciphersuites no longer fulfilled any purpose, and because they were actively harmful to users,
one may have expected software support to disappear quickly. This did not happen.

In 2015 [the FREAK attack](https://mitls.org/pages/attacks/SMACK#freak) ([CVE-2015-0204](https://nvd.nist.gov/vuln/detail/CVE-2015-0204))
and [the Logjam attack](https://weakdh.org/) ([CVE-2015-4000](https://nvd.nist.gov/vuln/detail/CVE-2015-4000)) both
demonstrated total breaks of security in the presence of servers that accepted export ciphersuites.  FREAK factored
512-bit RSA keys, while Logjam optimised solving discrete logs in the 512-bit group used by many different servers.

Naturally, rustls does not implement any of these ciphersuites.

## SWEET32

Block ciphers are vulnerable to birthday attacks, where the probability of repeating a block increases dramatically
once a particular key has been used for many blocks.  For block ciphers with 64-bit blocks, this becomes probable
once a given key encrypts the order of 32GB of data.

[Sweet32](https://sweet32.info/) ([CVE-2016-2183](https://nvd.nist.gov/vuln/detail/CVE-2016-2183)) attacked this fact
in the context of TLS support for 3DES, breaking confidentiality by analysing a large amount of attacker-induced traffic
in one session.

rustls does not support any 64-bit block ciphers.

## DROWN

[DROWN](https://drownattack.com/) ([CVE-2016-0800](https://nvd.nist.gov/vuln/detail/CVE-2016-0800)) is a cross-protocol
attack that breaks the security of TLSv1.2 and earlier (when used with RSA key exchange) by using SSLv2.  It is required
that the server uses the same key for both protocol versions.

rustls naturally does not support SSLv2, but most importantly does not support RSA key exchange for TLSv1.2.

## Poodle

[POODLE](https://cdn1.vox-cdn.com/uploads/chorus_asset/file/2354994/ssl-poodle.0.pdf) ([CVE-2014-3566](https://nvd.nist.gov/vuln/detail/CVE-2014-3566))
is an attack against CBC mode ciphersuites in SSLv3.  This was possible in most cases because some clients willingly
downgraded to SSLv3 after failed handshakes for later versions.

rustls does not support CBC mode ciphersuites, or SSLv3.  Note that rustls does not need to implement `TLS_FALLBACK_SCSV`
introduced as a countermeasure because it contains no ability to downgrade from TLS 1.2 to earlier protocol versions,
and TLS 1.3 has protocol-level downgrade protection based on the [ServerHello server random value](https://www.rfc-editor.org/rfc/rfc8446#section-4.1.3).

## GCM nonces

[RFC5288](https://tools.ietf.org/html/rfc5288) introduced GCM-based ciphersuites for use in TLS.  Unfortunately
the design was poor; it reused design for an unrelated security setting proposed in RFC5116.

GCM is a typical nonce-based AEAD: it requires a unique (but not necessarily unpredictable) 96-bit nonce for each encryption
with a given key.  The design specified by RFC5288 left two-thirds of the nonce construction up to implementations:

- wasting 8 bytes per TLS ciphertext,
- meaning correct operation cannot be tested for (e.g., in protocol-level test vectors).

There were no trade-offs here: TLS has a 64-bit sequence number that is not allowed to wrap and would make an ideal nonce.

As a result, a [2016 study](https://eprint.iacr.org/2016/475.pdf) found:

- implementations from IBM, A10 and Citrix used randomly-chosen nonces, which are unlikely to be unique over long connections,
- an implementation from Radware used the same nonce for the first two messages.

rustls uses a counter from a random starting point for GCM nonces.  TLSv1.3 and the Chacha20-Poly1305 TLSv1.2 ciphersuite
standardise this method.

## Renegotiation

In 2009 Marsh Ray and Steve Dispensa [discovered](https://kryptera.se/Renegotiating%20TLS.pdf) that the renegotiation
feature of all versions of TLS allows a MitM to splice a request of their choice onto the front of the client's real HTTP
request.  A countermeasure was proposed and widely implemented to bind renegotiations to their previous negotiations;
unfortunately this was insufficient.

rustls does not support renegotiation in TLSv1.2.  TLSv1.3 also no longer supports renegotiation.

## 3SHAKE

[3SHAKE](https://www.mitls.org/pages/attacks/3SHAKE) (2014) described a complex attack that broke the "Secure Renegotiation" extension
introduced as a countermeasure to the previous protocol flaw.

rustls does not support renegotiation for TLSv1.2 connections, or RSA key exchange, and both are required for this attack
to work.  rustls implements the "Extended Master Secret" (RFC7627) extension for TLSv1.2 which was standardised as a countermeasure.

TLSv1.3 no longer supports renegotiation and RSA key exchange.  It also effectively incorporates the improvements made in RFC7627.

## KCI

[This vulnerability](https://kcitls.org/) makes use of TLS ciphersuites (those offering static DH) which were standardised
yet not widely used. However, they were implemented by libraries, and as a result enabled for various clients.  It coupled
this with misconfigured certificates (on services including facebook.com) which allowed their misuse to MitM connections.

rustls does not support static DH/EC-DH ciphersuites.  We assert that it is misissuance to sign an EC certificate
with the keyUsage extension allowing both signatures and key exchange.  That it isn't is probably a failure
of CAB Forum baseline requirements.

```rust
pub mod _02_tls_vulnerabilities { /* ... */ }
```

## Module `_03_howto`

**Attributes:**

- `#[path = "howto.rs"]`

 This section collects together goal-oriented documentation.
 # Customising private key usage

By default rustls supports PKCS#8-format[^1] RSA or ECDSA keys, plus PKCS#1-format RSA keys.

However, if your private key resides in a HSM, or in another process, or perhaps
another machine, rustls has some extension points to support this:

The main trait you must implement is [`sign::SigningKey`][signing_key]. The primary method here
is [`choose_scheme()`][choose_scheme] where you are given a set of [`SignatureScheme`s][sig_scheme] the client says
it supports: you must choose one (or return `None` -- this aborts the handshake). Having
done that, you return an implementation of the [`sign::Signer`][signer] trait.
The [`sign()`][sign_method] performs the signature and returns it.

(Unfortunately this is currently designed for keys with low latency access, like in a
PKCS#11 provider, Microsoft CryptoAPI, etc. so is blocking rather than asynchronous.
It's a TODO to make these and other extension points async.)

Once you have these two pieces, configuring a server to use them involves, briefly:

- packaging your [`sign::SigningKey`][signing_key] with the matching certificate chain into a [`sign::CertifiedKey`][certified_key]
- making a [`ResolvesServerCertUsingSni`][cert_using_sni] and feeding in your [`sign::CertifiedKey`][certified_key] for all SNI hostnames you want to use it for,
- setting that as your `ServerConfig`'s [`cert_resolver`][cert_resolver]

For a complete example of implementing a custom [`sign::SigningKey`][signing_key] and
[`sign::Signer`][signer] see the [`signer` module in the `rustls-cng` crate][rustls-cng-signer].

[signing_key]: crate::crypto::signer::SigningKey
[choose_scheme]: crate::crypto::signer::SigningKey::choose_scheme
[sig_scheme]: crate::SignatureScheme
[signer]: crate::crypto::signer::Signer
[sign_method]: crate::crypto::signer::Signer::sign
[certified_key]: crate::crypto::signer::CertifiedKey
[cert_using_sni]: crate::server::ResolvesServerCertUsingSni
[cert_resolver]: crate::ServerConfig::cert_resolver
[rustls-cng-signer]: https://github.com/rustls/rustls-cng/blob/dev/src/signer.rs

[^1]: For PKCS#8 it does not support password encryption -- there's not a meaningful threat
      model addressed by this, and the encryption supported is typically extremely poor.

# Unexpected EOF

TLS has a `close_notify` mechanism to prevent truncation attacks[^2].
According to the TLS RFCs, each party is required to send a `close_notify` message before
closing the write side of the connection. However, some implementations don't send it.
So long as the application layer protocol (for instance HTTP/2) has message length framing
and can reject truncated messages, this is not a security problem.

Rustls treats an EOF without `close_notify` as an error of type `std::io::Error` with
`ErrorKind::UnexpectedEof`. In some situations it's appropriate for the application to handle
this error the same way it would handle a normal EOF (a read returning `Ok(0)`). In particular
if `UnexpectedEof` occurs on an idle connection it is appropriate to treat it the same way as a
clean shutdown. And if an application always uses messages with length framing (in other words,
messages are never delimited by the close of the TCP connection), it can unconditionally
ignore `UnexpectedEof` errors from rustls.

[^2]: <https://datatracker.ietf.org/doc/html/rfc8446#section-6.1>

```rust
pub mod _03_howto { /* ... */ }
```

## Module `_04_features`

**Attributes:**

- `#[path = "features.rs"]`

 This section documents rustls itself: what protocol features are and are not implemented.

The below list reflects the support provided with the default crate features.
Items marked with an asterisk `*` can be extended or altered via public
APIs ([`CryptoProvider`] for example).

[`CryptoProvider`]: crate::crypto::CryptoProvider

## Current features

* TLS1.2 and TLS1.3
* ECDSA, Ed25519 or RSA server authentication by clients `*`
* ECDSA, Ed25519[^1] or RSA server authentication by servers `*`
* Forward secrecy using ECDHE; with curve25519, nistp256 or nistp384 curves `*`
* Post-quantum hybrid key exchange with [X25519MLKEM768](https://datatracker.ietf.org/doc/draft-ietf-tls-ecdhe-mlkem/) [^2] `*`
* AES128-GCM and AES256-GCM bulk encryption, with safe nonces `*`
* ChaCha20-Poly1305 bulk encryption ([RFC7905](https://tools.ietf.org/html/rfc7905)) `*`
* ALPN support
* SNI support
* Tunable fragment size to make TLS messages match size of underlying transport
* Optional use of vectored IO to minimise system calls
* TLS1.2 session resumption
* TLS1.2 resumption via tickets ([RFC5077](https://tools.ietf.org/html/rfc5077))
* TLS1.3 resumption via tickets or session storage
* TLS1.3 0-RTT data
* Server and optional client authentication
* Extended master secret support ([RFC7627](https://tools.ietf.org/html/rfc7627))
* Exporters ([RFC5705](https://tools.ietf.org/html/rfc5705))
* OCSP stapling by servers
* [RFC7250](https://tools.ietf.org/html/rfc7250) raw public keys for TLS1.3
* [RFC8879](https://tools.ietf.org/html/rfc8879) certificate compression by clients
  and servers `*`
* Client-side Encrypted client hello (ECH)
   ([draft-ietf-tls-esni](https://datatracker.ietf.org/doc/draft-ietf-tls-esni/)).

[^1]: Note that, at the time of writing, Ed25519 does not have wide support
      in browsers.  It is also not supported by the WebPKI, because the
      CA/Browser Forum Baseline Requirements do not support it for publicly
      trusted certificates.
[^2]: See [the documentation][crate::manual::_05_defaults#about-the-post-quantum-secure-key-exchange-x25519mlkem768]

## Non-features

For reasons explained in the other sections of this manual, rustls does not
and will not support:

* SSL1, SSL2, SSL3, TLS1 or TLS1.1
* RC4
* DES or triple DES
* EXPORT ciphersuites
* MAC-then-encrypt ciphersuites
* Ciphersuites without forward secrecy
* Renegotiation
* Kerberos
* TLS 1.2 protocol compression
* Discrete-log Diffie-Hellman `*`
* Automatic protocol version downgrade
* Using CA certificates directly to authenticate a server/client (often called "self-signed
  certificates"). _Rustls' default certificate verifier does not support using a trust anchor as
  both a CA certificate and an end-entity certificate in order to limit complexity and risk in
  path building. While dangerous, all authentication can be turned off if required --
  see the [example code](https://github.com/rustls/rustls/blob/v/0.23.23/examples/src/bin/tlsclient-mio.rs#L338)_ `*`

### About "custom extensions"

OpenSSL allows an application to add arbitrary TLS extensions (via
the `SSL_CTX_add_custom_ext` function and associated APIs).  We don't
support this, with the following rationale:

Such an API is limited to extensions that are quite narrow in scope:
they cannot change the meaning of standard messages, or introduce new
messages, or make any changes to the connection's cryptography.

However, there is no reasonable way to technically limit that API to
that set of extensions.  That makes the API pretty unsafe (in the
TLS and cryptography sense, not memory safety sense).  This could
cause security or interop failures.

Instead, we suggest that potential users of that API consider:

- whether their use can fit in standard extensions such as ALPN,
  or [ALPS][alps][^3].
- if not, whether they can fit in a more general extension, and define
  and standardize that in the [IETF TLSWG][tlswg].

Note the above is not a guarantee or offer that rustls will implement
any specific extensions that are standardized by the IETF TLSWG.
It is a non-goal of this project to implement absolutely everything.

For experimentation and pre-standardization testing, we suggest
forking rustls.

See also: [Go's position on such an API][golang].

[alps]: https://datatracker.ietf.org/doc/html/draft-vvv-tls-alps
[golang]: https://github.com/golang/go/issues/51497
[tlswg]: https://datatracker.ietf.org/wg/tls/charter/
[^3]: rustls does not currently implement ALPS, but it is something we
  would consider once standardised and deployed.

```rust
pub mod _04_features { /* ... */ }
```

## Module `_05_defaults`

**Attributes:**

- `#[path = "defaults.rs"]`

 This section provides rationale for the defaults in rustls.

## Rationale for defaults

### Why is AES-256 preferred over AES-128?

This is a trade-off between:

1. classical security level: searching a 2^128 key space is as implausible as 2^256.
2. post-quantum security level: the difference is more meaningful, and AES-256 seems like the conservative choice.
3. performance: AES-256 is around 40% slower than AES-128, though hardware acceleration typically narrows this gap.

The choice is frankly quite marginal.

### Why is AES-GCM preferred over chacha20-poly1305?

Hardware support for accelerating AES-GCM is widespread, and hardware-accelerated AES-GCM
is quicker than un-accelerated chacha20-poly1305.

However, if you know your application will run on a platform without that, you should
_definitely_ change the default order to prefer chacha20-poly1305: both the performance and
the implementation security will be improved.  We think this is an uncommon case.

### Why is x25519 preferred for key exchange over nistp256?

Both provide roughly the same classical security level, but x25519 has better performance and
it's _much_ more likely that both peers will have good quality implementations.

### About the post-quantum-secure key exchange `X25519MLKEM768`

[`X25519MLKEM768`] -- a hybrid[^1], post-quantum-secure[^2] key exchange
algorithm -- is available when using the aws-lc-rs provider.

The `prefer-post-quantum` crate feature makes `X25519MLKEM768` the
highest-priority key exchange algorithm.  Otherwise, it is available but
not highest-priority.

[X25519MLKEM768] is pre-standardization, but is now widely deployed,
for example, by [Chrome] and [Cloudflare].

You may see unexpected connection failures (such as [tldr.fail])
-- [please report these to us][interop-bug].

The two components of this key exchange are well regarded:
X25519 alone is already used by default by rustls, and tends to have
higher quality implementations than other elliptic curves.
ML-KEM-768 was standardized by NIST in [FIPS203].

[`MLKEM768`] is available separately, but is not currently enabled
by default out of conservatism.

[^1]: meaning: a construction that runs a classical and post-quantum
      key exchange, and uses the output of both together.  This is a hedge
      against the post-quantum half being broken.

[^2]: a "post-quantum-secure" algorithm is one posited to be invulnerable
      to attack using a cryptographically-relevant quantum computer.  In contrast,
      classical algorithms would be broken by such a computer.  Note that such computers
      do not currently exist, and may never exist, but current traffic could be captured
      now and attacked later.

[X25519MLKEM768]: <https://datatracker.ietf.org/doc/draft-ietf-tls-ecdhe-mlkem/>
[`X25519MLKEM768`]: crate::crypto::aws_lc_rs::kx_group::X25519MLKEM768
[`MLKEM768`]: crate::crypto::aws_lc_rs::kx_group::MLKEM768
[FIPS203]: <https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf>
[Chrome]: <https://security.googleblog.com/2024/09/a-new-path-for-kyber-on-web.html>
[Cloudflare]: <https://blog.cloudflare.com/pq-2024/#ml-kem-768-and-x25519>
[interop-bug]: <https://github.com/rustls/rustls/issues/new?assignees=&labels=&projects=&template=bug_report.md&title=>
[tldr.fail]: <https://tldr.fail/>

```rust
pub mod _05_defaults { /* ... */ }
```

## Module `_06_fips`

**Attributes:**

- `#[path = "fips.rs"]`

 This section provides guidance on using rustls with FIPS-approved cryptography.
 # Using rustls with FIPS-approved cryptography

To use FIPS-approved cryptography with rustls, you should take
these actions:

## 1. Enable the `fips` crate feature for rustls.

Use:

```toml
rustls = { version = "0.23", features = [ "fips" ] }
```

## 2. Use the FIPS `CryptoProvider`

This is [`default_fips_provider()`]:

```rust,ignore
rustls::crypto::default_fips_provider()
    .install_default()
    .expect("default provider already set elsewhere");
```

This snippet makes use of the process-default provider,
and that assumes all your uses of rustls use that.
See [`CryptoProvider`] documentation for other ways to
specify which `CryptoProvider` to use.

## 3. Validate the FIPS status of your `ClientConfig`/`ServerConfig` at run-time

See [`ClientConfig::fips()`] or [`ServerConfig::fips()`].

You could, for example:

```rust,ignore
# let client_config = unreachable!();
assert!(client_config.fips());
```

But maybe your application has an error handling
or health-check strategy better than panicking.

# aws-lc-rs FIPS approval status

This is covered by [FIPS 140-3 certificate #4816][cert-4816].
See [the security policy][policy-4816] for precisely which
environments and functions this certificate covers.

Later releases of aws-lc-rs may be covered by later certificates,
or be pending certification.

For the most up-to-date details see the latest documentation
for the [`aws-lc-fips-sys`] crate.

[`aws-lc-fips-sys`]: https://crates.io/crates/aws-lc-fips-sys
[`default_fips_provider()`]: crate::crypto::default_fips_provider
[`CryptoProvider`]: crate::crypto::CryptoProvider
[`ClientConfig::fips()`]: crate::client::ClientConfig::fips
[`ServerConfig::fips()`]: crate::server::ServerConfig::fips
[cert-4816]: https://csrc.nist.gov/projects/cryptographic-module-validation-program/certificate/4816
[policy-4816]: https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/security-policies/140sp4816.pdf

```rust
pub mod _06_fips { /* ... */ }
```

## Module `time_provider`

The library's source of time.

```rust
pub mod time_provider { /* ... */ }
```

### Types

#### Struct `DefaultTimeProvider`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Default `TimeProvider` implementation that uses `std`

```rust
pub struct DefaultTimeProvider;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
- **Unpin**
- **TimeProvider**
  - ```rust
    fn current_time(self: &Self) -> Option<UnixTime> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `TimeProvider`

An object that provides the current time.

This is used to, for example, check if a certificate has expired during
certificate validation, or to check the age of a ticket.

```rust
pub trait TimeProvider: Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `current_time`: Returns the current wall time.

##### Implementations

This trait is implemented for the following types:

- `DefaultTimeProvider`

## Module `lock`

APIs abstracting over locking primitives.

```rust
pub mod lock { /* ... */ }
```

### Re-exports

#### Re-export `std_lock::*`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std_lock::*;
```

## Re-exports

### Re-export `ConfigBuilder`

```rust
pub use crate::builder::ConfigBuilder;
```

### Re-export `ConfigSide`

```rust
pub use crate::builder::ConfigSide;
```

### Re-export `WantsVerifier`

```rust
pub use crate::builder::WantsVerifier;
```

### Re-export `WantsVersions`

```rust
pub use crate::builder::WantsVersions;
```

### Re-export `CommonState`

```rust
pub use crate::common_state::CommonState;
```

### Re-export `HandshakeKind`

```rust
pub use crate::common_state::HandshakeKind;
```

### Re-export `IoState`

```rust
pub use crate::common_state::IoState;
```

### Re-export `Side`

```rust
pub use crate::common_state::Side;
```

### Re-export `Connection`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::conn::Connection;
```

### Re-export `Reader`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::conn::Reader;
```

### Re-export `Writer`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::conn::Writer;
```

### Re-export `ConnectionCommon`

```rust
pub use crate::conn::ConnectionCommon;
```

### Re-export `SideData`

```rust
pub use crate::conn::SideData;
```

### Re-export `AlertDescription`

```rust
pub use crate::enums::AlertDescription;
```

### Re-export `CertificateCompressionAlgorithm`

```rust
pub use crate::enums::CertificateCompressionAlgorithm;
```

### Re-export `CipherSuite`

```rust
pub use crate::enums::CipherSuite;
```

### Re-export `ContentType`

```rust
pub use crate::enums::ContentType;
```

### Re-export `HandshakeType`

```rust
pub use crate::enums::HandshakeType;
```

### Re-export `ProtocolVersion`

```rust
pub use crate::enums::ProtocolVersion;
```

### Re-export `SignatureAlgorithm`

```rust
pub use crate::enums::SignatureAlgorithm;
```

### Re-export `SignatureScheme`

```rust
pub use crate::enums::SignatureScheme;
```

### Re-export `CertRevocationListError`

```rust
pub use crate::error::CertRevocationListError;
```

### Re-export `CertificateError`

```rust
pub use crate::error::CertificateError;
```

### Re-export `EncryptedClientHelloError`

```rust
pub use crate::error::EncryptedClientHelloError;
```

### Re-export `Error`

```rust
pub use crate::error::Error;
```

### Re-export `InconsistentKeys`

```rust
pub use crate::error::InconsistentKeys;
```

### Re-export `InvalidMessage`

```rust
pub use crate::error::InvalidMessage;
```

### Re-export `OtherError`

```rust
pub use crate::error::OtherError;
```

### Re-export `PeerIncompatible`

```rust
pub use crate::error::PeerIncompatible;
```

### Re-export `PeerMisbehaved`

```rust
pub use crate::error::PeerMisbehaved;
```

### Re-export `KeyLog`

```rust
pub use crate::key_log::KeyLog;
```

### Re-export `NoKeyLog`

```rust
pub use crate::key_log::NoKeyLog;
```

### Re-export `KeyLogFile`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::key_log_file::KeyLogFile;
```

### Re-export `NamedGroup`

```rust
pub use crate::msgs::enums::NamedGroup;
```

### Re-export `ffdhe_groups`

```rust
pub use crate::msgs::ffdhe_groups;
```

### Re-export `DistinguishedName`

```rust
pub use crate::msgs::handshake::DistinguishedName;
```

### Re-export `Stream`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::stream::Stream;
```

### Re-export `StreamOwned`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::stream::StreamOwned;
```

### Re-export `CipherSuiteCommon`

```rust
pub use crate::suites::CipherSuiteCommon;
```

### Re-export `ConnectionTrafficSecrets`

```rust
pub use crate::suites::ConnectionTrafficSecrets;
```

### Re-export `ExtractedSecrets`

```rust
pub use crate::suites::ExtractedSecrets;
```

### Re-export `SupportedCipherSuite`

```rust
pub use crate::suites::SupportedCipherSuite;
```

### Re-export `TicketRotator`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::ticketer::TicketRotator;
```

### Re-export `TicketSwitcher`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "hashbrown"))]`

```rust
pub use crate::ticketer::TicketSwitcher;
```

### Re-export `Tls12CipherSuite`

**Attributes:**

- `#[<cfg>(feature = "tls12")]`

```rust
pub use crate::tls12::Tls12CipherSuite;
```

### Re-export `Tls13CipherSuite`

```rust
pub use crate::tls13::Tls13CipherSuite;
```

### Re-export `DigitallySignedStruct`

```rust
pub use crate::verify::DigitallySignedStruct;
```

### Re-export `ALL_VERSIONS`

```rust
pub use crate::versions::ALL_VERSIONS;
```

### Re-export `DEFAULT_VERSIONS`

```rust
pub use crate::versions::DEFAULT_VERSIONS;
```

### Re-export `SupportedProtocolVersion`

```rust
pub use crate::versions::SupportedProtocolVersion;
```

### Re-export `RootCertStore`

```rust
pub use crate::webpki::RootCertStore;
```

### Re-export `ClientConfig`

```rust
pub use client::ClientConfig;
```

### Re-export `ClientConnection`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use client::ClientConnection;
```

### Re-export `ServerConfig`

```rust
pub use server::ServerConfig;
```

### Re-export `ServerConnection`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use server::ServerConnection;
```

