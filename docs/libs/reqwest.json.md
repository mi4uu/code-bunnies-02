# Crate Documentation

**Version:** 0.12.15

**Format Version:** 43

# Module `reqwest`

# reqwest

The `reqwest` crate provides a convenient, higher-level HTTP
[`Client`][client].

It handles many of the things that most people just expect an HTTP client
to do for them.

- Async and [blocking] Clients
- Plain bodies, [JSON](#json), [urlencoded](#forms), [multipart]
- Customizable [redirect policy](#redirect-policies)
- HTTP [Proxies](#proxies)
- Uses [TLS](#tls) by default
- Cookies

The [`reqwest::Client`][client] is asynchronous. For applications wishing
to only make a few HTTP requests, the [`reqwest::blocking`](blocking) API
may be more convenient.

Additional learning resources include:

- [The Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/web/clients.html)
- [Reqwest Repository Examples](https://github.com/seanmonstar/reqwest/tree/master/examples)

## Commercial Support

For private advice, support, reviews, access to the maintainer, and the
like, reach out for [commercial support][sponsor].

## Making a GET request

For a single request, you can use the [`get`][get] shortcut method.

```rust
# async fn run() -> Result<(), reqwest::Error> {
let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

println!("body = {body:?}");
# Ok(())
# }
```

**NOTE**: If you plan to perform multiple requests, it is best to create a
[`Client`][client] and reuse it, taking advantage of keep-alive connection
pooling.

## Making POST requests (or setting request bodies)

There are several ways you can set the body of a request. The basic one is
by using the `body()` method of a [`RequestBuilder`][builder]. This lets you set the
exact raw bytes of what the body should be. It accepts various types,
including `String` and `Vec<u8>`. If you wish to pass a custom
type, you can use the `reqwest::Body` constructors.

```rust
# use reqwest::Error;
#
# async fn run() -> Result<(), Error> {
let client = reqwest::Client::new();
let res = client.post("http://httpbin.org/post")
    .body("the exact body that is sent")
    .send()
    .await?;
# Ok(())
# }
```

### Forms

It's very common to want to send form data in a request body. This can be
done with any type that can be serialized into form data.

This can be an array of tuples, or a `HashMap`, or a custom type that
implements [`Serialize`][serde].

```rust
# use reqwest::Error;
#
# async fn run() -> Result<(), Error> {
// This will POST a body of `foo=bar&baz=quux`
let params = [("foo", "bar"), ("baz", "quux")];
let client = reqwest::Client::new();
let res = client.post("http://httpbin.org/post")
    .form(&params)
    .send()
    .await?;
# Ok(())
# }
```

### JSON

There is also a `json` method helper on the [`RequestBuilder`][builder] that works in
a similar fashion the `form` method. It can take any value that can be
serialized into JSON. The feature `json` is required.

```rust
# use reqwest::Error;
# use std::collections::HashMap;
#
# #[cfg(feature = "json")]
# async fn run() -> Result<(), Error> {
// This will POST a body of `{"lang":"rust","body":"json"}`
let mut map = HashMap::new();
map.insert("lang", "rust");
map.insert("body", "json");

let client = reqwest::Client::new();
let res = client.post("http://httpbin.org/post")
    .json(&map)
    .send()
    .await?;
# Ok(())
# }
```

## Redirect Policies

By default, a `Client` will automatically handle HTTP redirects, having a
maximum redirect chain of 10 hops. To customize this behavior, a
[`redirect::Policy`][redirect] can be used with a `ClientBuilder`.

## Cookies

The automatic storing and sending of session cookies can be enabled with
the [`cookie_store`][ClientBuilder::cookie_store] method on `ClientBuilder`.

## Proxies

**NOTE**: System proxies are enabled by default.

System proxies look in environment variables to set HTTP or HTTPS proxies.

`HTTP_PROXY` or `http_proxy` provide HTTP proxies for HTTP connections while
`HTTPS_PROXY` or `https_proxy` provide HTTPS proxies for HTTPS connections.
`ALL_PROXY` or `all_proxy` provide proxies for both HTTP and HTTPS connections.
If both the all proxy and HTTP or HTTPS proxy variables are set the more specific
HTTP or HTTPS proxies take precedence.

These can be overwritten by adding a [`Proxy`] to `ClientBuilder`
i.e. `let proxy = reqwest::Proxy::http("https://secure.example")?;`
or disabled by calling `ClientBuilder::no_proxy()`.

`socks` feature is required if you have configured socks proxy like this:

```bash
export https_proxy=socks5://127.0.0.1:1086
```

## TLS

A `Client` will use transport layer security (TLS) by default to connect to
HTTPS destinations.

- Additional server certificates can be configured on a `ClientBuilder`
  with the [`Certificate`] type.
- Client certificates can be added to a `ClientBuilder` with the
  [`Identity`] type.
- Various parts of TLS can also be configured or even disabled on the
  `ClientBuilder`.

See more details in the [`tls`] module.

## WASM

The Client implementation automatically switches to the WASM one when the target_arch is wasm32,
the usage is basically the same as the async api. Some of the features are disabled in wasm
: [`tls`], [`cookie`], [`blocking`], as well as various `ClientBuilder` methods such as `timeout()` and `connector_layer()`.


## Optional Features

The following are a list of [Cargo features][cargo-features] that can be
enabled or disabled:

- **http2** *(enabled by default)*: Enables HTTP/2 support.
- **default-tls** *(enabled by default)*: Provides TLS support to connect
  over HTTPS.
- **native-tls**: Enables TLS functionality provided by `native-tls`.
- **native-tls-vendored**: Enables the `vendored` feature of `native-tls`.
- **native-tls-alpn**: Enables the `alpn` feature of `native-tls`.
- **rustls-tls**: Enables TLS functionality provided by `rustls`.
  Equivalent to `rustls-tls-webpki-roots`.
- **rustls-tls-manual-roots**: Enables TLS functionality provided by `rustls`,
  without setting any root certificates. Roots have to be specified manually.
- **rustls-tls-webpki-roots**: Enables TLS functionality provided by `rustls`,
  while using root certificates from the `webpki-roots` crate.
- **rustls-tls-native-roots**: Enables TLS functionality provided by `rustls`,
  while using root certificates from the `rustls-native-certs` crate.
- **blocking**: Provides the [blocking][] client API.
- **charset** *(enabled by default)*: Improved support for decoding text.
- **cookies**: Provides cookie session support.
- **gzip**: Provides response body gzip decompression.
- **brotli**: Provides response body brotli decompression.
- **zstd**: Provides response body zstd decompression.
- **deflate**: Provides response body deflate decompression.
- **json**: Provides serialization and deserialization for JSON bodies.
- **multipart**: Provides functionality for multipart forms.
- **stream**: Adds support for `futures::Stream`.
- **socks**: Provides SOCKS5 proxy support.
- **hickory-dns**: Enables a hickory-dns async resolver instead of default
  threadpool using `getaddrinfo`.

## Unstable Features

Some feature flags require additional opt-in by the application, by setting
a `reqwest_unstable` flag.

- **http3** *(unstable)*: Enables support for sending HTTP/3 requests.

These features are unstable, and experimental. Details about them may be
changed in patch releases.

You can pass such a flag to the compiler via `.cargo/config`, or
environment variables, such as:

```notrust
RUSTFLAGS="--cfg reqwest_unstable" cargo build
```

## Sponsors

Support this project by becoming a [sponsor][].

[hyper]: https://hyper.rs
[blocking]: ./blocking/index.html
[client]: ./struct.Client.html
[response]: ./struct.Response.html
[get]: ./fn.get.html
[builder]: ./struct.RequestBuilder.html
[serde]: http://serde.rs
[redirect]: crate::redirect
[Proxy]: ./struct.Proxy.html
[cargo-features]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section
[sponsor]: https://seanmonstar.com/sponsor

## Modules

## Module `dns`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

DNS resolution

```rust
pub mod dns { /* ... */ }
```

### Re-exports

#### Re-export `Addrs`

```rust
pub use resolve::Addrs;
```

#### Re-export `Name`

```rust
pub use resolve::Name;
```

#### Re-export `Resolve`

```rust
pub use resolve::Resolve;
```

#### Re-export `Resolving`

```rust
pub use resolve::Resolving;
```

## Module `redirect`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

Redirect Handling

By default, a `Client` will automatically handle HTTP redirects, having a
maximum redirect chain of 10 hops. To customize this behavior, a
`redirect::Policy` can be used with a `ClientBuilder`.

```rust
pub mod redirect { /* ... */ }
```

### Types

#### Struct `Policy`

A type that controls the policy on how to handle the following of redirects.

The default value will catch redirect loops, and has a maximum of 10
redirects it will follow in a chain before returning an error.

- `limited` can be used have the same as the default behavior, but adjust
  the allowed maximum redirect hops in a chain.
- `none` can be used to disable all redirect behavior.
- `custom` can be used to create a customized policy.

```rust
pub struct Policy {
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
  pub fn limited(max: usize) -> Self { /* ... */ }
  ```
  Create a `Policy` with a maximum number of redirects.

- ```rust
  pub fn none() -> Self { /* ... */ }
  ```
  Create a `Policy` that does not follow any redirect.

- ```rust
  pub fn custom<T>(policy: T) -> Self
where
    T: Fn(Attempt<''_>) -> Action + Send + Sync + ''static { /* ... */ }
  ```
  Create a custom `Policy` using the passed function.

- ```rust
  pub fn redirect(self: &Self, attempt: Attempt<''_>) -> Action { /* ... */ }
  ```
  Apply this policy to a given [`Attempt`] to produce a [`Action`].

###### Trait Implementations

- **Freeze**
- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **WithSubscriber**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Policy { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Attempt`

A type that holds information on the next request and previous requests
in redirect chain.

```rust
pub struct Attempt<''a> {
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
  pub fn status(self: &Self) -> StatusCode { /* ... */ }
  ```
  Get the type of redirect.

- ```rust
  pub fn url(self: &Self) -> &Url { /* ... */ }
  ```
  Get the next URL to redirect to.

- ```rust
  pub fn previous(self: &Self) -> &[Url] { /* ... */ }
  ```
  Get the list of previous URLs that have already been requested in this chain.

- ```rust
  pub fn follow(self: Self) -> Action { /* ... */ }
  ```
  Returns an action meaning reqwest should follow the next URL.

- ```rust
  pub fn stop(self: Self) -> Action { /* ... */ }
  ```
  Returns an action meaning reqwest should not follow the next URL.

- ```rust
  pub fn error<E: Into<Box<dyn StdError + Send + Sync>>>(self: Self, error: E) -> Action { /* ... */ }
  ```
  Returns an action failing the redirect with an error.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **MaybeSendSync**
- **ErasedDestructor**
- **WithSubscriber**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Instrument**
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

- **IntoEither**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Action`

An action to perform when a redirect status code is found.

```rust
pub struct Action {
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

- **ErasedDestructor**
- **Send**
- **UnwindSafe**
- **Sync**
- **IntoEither**
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

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Instrument**
## Module `tls`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`
- `#[<cfg>(feature = "__tls")]`

TLS configuration and types

A `Client` will use transport layer security (TLS) by default to connect to
HTTPS destinations.

# Backends

reqwest supports several TLS backends, enabled with Cargo features.

## default-tls

reqwest will pick a TLS backend by default. This is true when the
`default-tls` feature is enabled.

While it currently uses `native-tls`, the feature set is designed to only
enable configuration that is shared among available backends. This allows
reqwest to change the default to `rustls` (or another) at some point in the
future.

<div class="warning">This feature is enabled by default, and takes
precedence if any other crate enables it. This is true even if you declare
`features = []`. You must set `default-features = false` instead.</div>

Since Cargo features are additive, other crates in your dependency tree can
cause the default backend to be enabled. If you wish to ensure your
`Client` uses a specific backend, call the appropriate builder methods
(such as [`use_rustls_tls()`][]).

[`use_rustls_tls()`]: crate::ClientBuilder::use_rustls_tls()

## native-tls

This backend uses the [native-tls][] crate. That will try to use the system
TLS on Windows and Mac, and OpenSSL on Linux targets.

Enabling the feature explicitly allows for `native-tls`-specific
configuration options.

[native-tls]: https://crates.io/crates/native-tls

## rustls-tls

This backend uses the [rustls][] crate, a TLS library written in Rust.

[rustls]: https://crates.io/crates/rustls

```rust
pub mod tls { /* ... */ }
```

### Types

#### Struct `Certificate`

Represents a server X509 certificate.

```rust
pub struct Certificate {
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
  pub fn from_der(der: &[u8]) -> crate::Result<Certificate> { /* ... */ }
  ```
  Create a `Certificate` from a binary DER encoded certificate

- ```rust
  pub fn from_pem(pem: &[u8]) -> crate::Result<Certificate> { /* ... */ }
  ```
  Create a `Certificate` from a PEM encoded certificate

- ```rust
  pub fn from_pem_bundle(pem_bundle: &[u8]) -> crate::Result<Vec<Certificate>> { /* ... */ }
  ```
  Create a collection of `Certificate`s from a PEM encoded certificate bundle.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Certificate { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
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

- **Instrument**
- **IntoEither**
- **WithSubscriber**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
#### Struct `Identity`

Represents a private key and X509 cert as a client certificate.

```rust
pub struct Identity {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **WithSubscriber**
- **UnwindSafe**
- **MaybeSendSync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Identity { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
#### Struct `Version`

A TLS protocol version.

```rust
pub struct Version(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **IntoEither**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Version) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Version) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **UnwindSafe**
- **Unpin**
- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **Freeze**
- **WithSubscriber**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Version) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Version { /* ... */ }
    ```

- **Instrument**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `TlsInfo`

Hyper extension carrying extra TLS layer information.
Made available to clients on responses when `tls_info` is set.

```rust
pub struct TlsInfo {
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
  pub fn peer_certificate(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
  Get the DER encoded leaf certificate of the peer.

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **WithSubscriber**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TlsInfo { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **Instrument**
## Functions

### Function `get`

Shortcut method to quickly make a `GET` request.

See also the methods on the [`reqwest::Response`](./struct.Response.html)
type.

**NOTE**: This function creates a new internal `Client` on each call,
and so should not be used if making many requests. Create a
[`Client`](./struct.Client.html) instead.

# Examples

```rust
# async fn run() -> Result<(), reqwest::Error> {
let body = reqwest::get("https://www.rust-lang.org").await?
    .text().await?;
# Ok(())
# }
```

# Errors

This function fails if:

- native TLS backend cannot be initialized
- supplied `Url` cannot be parsed
- there was an error while sending request
- redirect limit was exhausted

```rust
pub async fn get<T: IntoUrl>(url: T) -> crate::Result<Response> { /* ... */ }
```

## Re-exports

### Re-export `header`

```rust
pub use http::header;
```

### Re-export `Method`

```rust
pub use http::Method;
```

### Re-export `StatusCode`

```rust
pub use http::StatusCode;
```

### Re-export `Version`

```rust
pub use http::Version;
```

### Re-export `Url`

```rust
pub use url::Url;
```

### Re-export `Error`

```rust
pub use self::error::Error;
```

### Re-export `Result`

```rust
pub use self::error::Result;
```

### Re-export `IntoUrl`

```rust
pub use self::into_url::IntoUrl;
```

### Re-export `ResponseBuilderExt`

```rust
pub use self::response::ResponseBuilderExt;
```

### Re-export `Body`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::async_impl::Body;
```

### Re-export `Client`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::async_impl::Client;
```

### Re-export `ClientBuilder`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::async_impl::ClientBuilder;
```

### Re-export `Request`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::async_impl::Request;
```

### Re-export `RequestBuilder`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::async_impl::RequestBuilder;
```

### Re-export `Response`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::async_impl::Response;
```

### Re-export `Upgraded`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::async_impl::Upgraded;
```

### Re-export `Proxy`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::proxy::Proxy;
```

### Re-export `NoProxy`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`

```rust
pub use self::proxy::NoProxy;
```

### Re-export `Certificate`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`
- `#[<cfg>(feature = "__tls")]`

```rust
pub use tls::Certificate;
```

### Re-export `Identity`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`
- `#[<cfg>(feature = "__tls")]`

```rust
pub use tls::Identity;
```

### Re-export `multipart`

**Attributes:**

- `#[<cfg>(not(target_arch = "wasm32"))]`
- `#[<cfg>(feature = "multipart")]`

```rust
pub use self::async_impl::multipart;
```

