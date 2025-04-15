# Crate Documentation

**Version:** 1.3.1

**Format Version:** 43

# Module `http`

A general purpose library of common HTTP types

This crate is a general purpose library for common types found when working
with the HTTP protocol. You'll find [`Request`] and [`Response`] types for
working as either a client or a server as well as all of their components.
Notably you'll find `Uri` for what a [`Request`] is requesting, a [`Method`]
for how it's being requested, a [`StatusCode`] for what sort of response came
back, a [`Version`] for how this was communicated, and
[`HeaderName`]/[`HeaderValue`] definitions to get grouped in a [`HeaderMap`] to
work with request/response headers.

You will notably *not* find an implementation of sending requests or
spinning up a server in this crate. It's intended that this crate is the
"standard library" for HTTP clients and servers without dictating any
particular implementation.

## Requests and Responses

Perhaps the main two types in this crate are the [`Request`] and [`Response`]
types. A [`Request`] could either be constructed to get sent off as a client
or it can also be received to generate a [`Response`] for a server. Similarly
as a client a [`Response`] is what you get after sending a [`Request`], whereas
on a server you'll be manufacturing a [`Response`] to send back to the client.

Each type has a number of accessors for the component fields. For as a
server you might want to inspect a requests URI to dispatch it:

```
use http::{Request, Response};

fn response(req: Request<()>) -> http::Result<Response<()>> {
    match req.uri().path() {
        "/" => index(req),
        "/foo" => foo(req),
        "/bar" => bar(req),
        _ => not_found(req),
    }
}
# fn index(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
# fn foo(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
# fn bar(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
# fn not_found(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
```

On a [`Request`] you'll also find accessors like [`method`][Request::method] to return a
[`Method`] and [`headers`][Request::method] to inspect the various headers. A [`Response`]
has similar methods for headers, the status code, etc.

In addition to getters, request/response types also have mutable accessors
to edit the request/response:

```
use http::{HeaderValue, Response, StatusCode};
use http::header::CONTENT_TYPE;

fn add_server_headers<T>(response: &mut Response<T>) {
    response.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    *response.status_mut() = StatusCode::OK;
}
```

And finally, one of the most important aspects of requests/responses, the
body! The [`Request`] and [`Response`] types in this crate are *generic* in
what their body is. This allows downstream libraries to use different
representations such as `Request<Vec<u8>>`, `Response<impl Read>`,
`Request<impl Stream<Item = Vec<u8>, Error = _>>`, or even
`Response<MyCustomType>` where the custom type was deserialized from JSON.

The body representation is intentionally flexible to give downstream
libraries maximal flexibility in implementing the body as appropriate.

## HTTP Headers

Another major piece of functionality in this library is HTTP header
interpretation and generation. The `HeaderName` type serves as a way to
define header *names*, or what's to the left of the colon. A `HeaderValue`
conversely is the header *value*, or what's to the right of a colon.

For example, if you have an HTTP request that looks like:

```http
GET /foo HTTP/1.1
Accept: text/html
```

Then `"Accept"` is a [`HeaderName`] while `"text/html"` is a [`HeaderValue`].
Each of these is a dedicated type to allow for a number of interesting
optimizations and to also encode the static guarantees of each type. For
example a [`HeaderName`] is always a valid `&str`, but a [`HeaderValue`] may
not be valid UTF-8.

The most common header names are already defined for you as constant values
in the [`header`] module of this crate. For example:

```
use http::header::{self, HeaderName};

let name: HeaderName = header::ACCEPT;
assert_eq!(name.as_str(), "accept");
```

You can, however, also parse header names from strings:

```
use http::header::{self, HeaderName};

let name = "Accept".parse::<HeaderName>().unwrap();
assert_eq!(name, header::ACCEPT);
```

Header values can be created from string literals through the [`from_static`][header::HeaderValue::from_static]
function:

```
use http::HeaderValue;

let value = HeaderValue::from_static("text/html");
assert_eq!(value.as_bytes(), b"text/html");
```

And header values can also be parsed like names:

```
use http::HeaderValue;

let value = "text/html";
let value = value.parse::<HeaderValue>().unwrap();
```

Most HTTP requests and responses tend to come with more than one header, so
it's not too useful to just work with names and values only! This crate also
provides a [`HeaderMap`] type which is a specialized hash map for keys as
[`HeaderName`] and generic values. This type, like header names, is optimized
for common usage but should continue to scale with your needs over time.

# URIs

Each HTTP [`Request`] has an associated URI with it. This may just be a path
like `/index.html` but it could also be an absolute URL such as
`https://www.rust-lang.org/index.html`. A [`URI`][uri::Uri] has a number of accessors to
interpret it:

```
use http::Uri;
use http::uri::Scheme;

let uri = "https://www.rust-lang.org/index.html".parse::<Uri>().unwrap();

assert_eq!(uri.scheme(), Some(&Scheme::HTTPS));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/index.html");
assert_eq!(uri.query(), None);
```

## Modules

## Module `header`

HTTP header types

The module provides [`HeaderName`], [`HeaderMap`], and a number of types
used for interacting with `HeaderMap`. These types allow representing both
HTTP/1 and HTTP/2 headers.

# `HeaderName`

The `HeaderName` type represents both standard header names as well as
custom header names. The type handles the case insensitive nature of header
names and is used as the key portion of `HeaderMap`. Header names are
normalized to lower case. In other words, when creating a `HeaderName` with
a string, even if upper case characters are included, when getting a string
representation of the `HeaderName`, it will be all lower case. This allows
for faster `HeaderMap` comparison operations.

The internal representation is optimized to efficiently handle the cases
most commonly encountered when working with HTTP. Standard header names are
special cased and are represented internally as an enum. Short custom
headers will be stored directly in the `HeaderName` struct and will not
incur any allocation overhead, however longer strings will require an
allocation for storage.

## Limitations

`HeaderName` has a max length of 32,768 for header names. Attempting to
parse longer names will result in a panic.

# `HeaderMap`

`HeaderMap` is a map structure of header names highly optimized for use
cases common with HTTP. It is a [multimap] structure, where each header name
may have multiple associated header values. Given this, some of the APIs
diverge from [`HashMap`].

## Overview

Just like `HashMap` in Rust's stdlib, `HeaderMap` is based on [Robin Hood
hashing]. This algorithm tends to reduce the worst case search times in the
table and enables high load factors without seriously affecting performance.
Internally, keys and values are stored in vectors. As such, each insertion
will not incur allocation overhead. However, once the underlying vector
storage is full, a larger vector must be allocated and all values copied.

## Deterministic ordering

Unlike Rust's `HashMap`, values in `HeaderMap` are deterministically
ordered. Roughly, values are ordered by insertion. This means that a
function that deterministically operates on a header map can rely on the
iteration order to remain consistent across processes and platforms.

## Adaptive hashing

`HeaderMap` uses an adaptive hashing strategy in order to efficiently handle
most common cases. All standard headers have statically computed hash values
which removes the need to perform any hashing of these headers at runtime.
The default hash function emphasizes performance over robustness. However,
`HeaderMap` detects high collision rates and switches to a secure hash
function in those events. The threshold is set such that only denial of
service attacks should trigger it.

## Limitations

`HeaderMap` can store a maximum of 32,768 headers (header name / value
pairs). Attempting to insert more will result in a panic.

[`HeaderName`]: struct.HeaderName.html
[`HeaderMap`]: struct.HeaderMap.html
[multimap]: https://en.wikipedia.org/wiki/Multimap
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[Robin Hood hashing]: https://en.wikipedia.org/wiki/Hash_table#Robin_Hood_hashing

```rust
pub mod header { /* ... */ }
```

### Re-exports

#### Re-export `AsHeaderName`

```rust
pub use self::map::AsHeaderName;
```

#### Re-export `Drain`

```rust
pub use self::map::Drain;
```

#### Re-export `Entry`

```rust
pub use self::map::Entry;
```

#### Re-export `GetAll`

```rust
pub use self::map::GetAll;
```

#### Re-export `HeaderMap`

```rust
pub use self::map::HeaderMap;
```

#### Re-export `IntoHeaderName`

```rust
pub use self::map::IntoHeaderName;
```

#### Re-export `IntoIter`

```rust
pub use self::map::IntoIter;
```

#### Re-export `Iter`

```rust
pub use self::map::Iter;
```

#### Re-export `IterMut`

```rust
pub use self::map::IterMut;
```

#### Re-export `Keys`

```rust
pub use self::map::Keys;
```

#### Re-export `MaxSizeReached`

```rust
pub use self::map::MaxSizeReached;
```

#### Re-export `OccupiedEntry`

```rust
pub use self::map::OccupiedEntry;
```

#### Re-export `VacantEntry`

```rust
pub use self::map::VacantEntry;
```

#### Re-export `ValueDrain`

```rust
pub use self::map::ValueDrain;
```

#### Re-export `ValueIter`

```rust
pub use self::map::ValueIter;
```

#### Re-export `ValueIterMut`

```rust
pub use self::map::ValueIterMut;
```

#### Re-export `Values`

```rust
pub use self::map::Values;
```

#### Re-export `ValuesMut`

```rust
pub use self::map::ValuesMut;
```

#### Re-export `HeaderName`

```rust
pub use self::name::HeaderName;
```

#### Re-export `InvalidHeaderName`

```rust
pub use self::name::InvalidHeaderName;
```

#### Re-export `HeaderValue`

```rust
pub use self::value::HeaderValue;
```

#### Re-export `InvalidHeaderValue`

```rust
pub use self::value::InvalidHeaderValue;
```

#### Re-export `ToStrError`

```rust
pub use self::value::ToStrError;
```

#### Re-export `ACCEPT`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCEPT;
```

#### Re-export `ACCEPT_CHARSET`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCEPT_CHARSET;
```

#### Re-export `ACCEPT_ENCODING`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCEPT_ENCODING;
```

#### Re-export `ACCEPT_LANGUAGE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCEPT_LANGUAGE;
```

#### Re-export `ACCEPT_RANGES`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCEPT_RANGES;
```

#### Re-export `ACCESS_CONTROL_ALLOW_CREDENTIALS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCESS_CONTROL_ALLOW_CREDENTIALS;
```

#### Re-export `ACCESS_CONTROL_ALLOW_HEADERS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCESS_CONTROL_ALLOW_HEADERS;
```

#### Re-export `ACCESS_CONTROL_ALLOW_METHODS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCESS_CONTROL_ALLOW_METHODS;
```

#### Re-export `ACCESS_CONTROL_ALLOW_ORIGIN`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCESS_CONTROL_ALLOW_ORIGIN;
```

#### Re-export `ACCESS_CONTROL_EXPOSE_HEADERS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCESS_CONTROL_EXPOSE_HEADERS;
```

#### Re-export `ACCESS_CONTROL_MAX_AGE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCESS_CONTROL_MAX_AGE;
```

#### Re-export `ACCESS_CONTROL_REQUEST_HEADERS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCESS_CONTROL_REQUEST_HEADERS;
```

#### Re-export `ACCESS_CONTROL_REQUEST_METHOD`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ACCESS_CONTROL_REQUEST_METHOD;
```

#### Re-export `AGE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::AGE;
```

#### Re-export `ALLOW`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ALLOW;
```

#### Re-export `ALT_SVC`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ALT_SVC;
```

#### Re-export `AUTHORIZATION`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::AUTHORIZATION;
```

#### Re-export `CACHE_CONTROL`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CACHE_CONTROL;
```

#### Re-export `CACHE_STATUS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CACHE_STATUS;
```

#### Re-export `CDN_CACHE_CONTROL`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CDN_CACHE_CONTROL;
```

#### Re-export `CONNECTION`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONNECTION;
```

#### Re-export `CONTENT_DISPOSITION`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_DISPOSITION;
```

#### Re-export `CONTENT_ENCODING`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_ENCODING;
```

#### Re-export `CONTENT_LANGUAGE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_LANGUAGE;
```

#### Re-export `CONTENT_LENGTH`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_LENGTH;
```

#### Re-export `CONTENT_LOCATION`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_LOCATION;
```

#### Re-export `CONTENT_RANGE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_RANGE;
```

#### Re-export `CONTENT_SECURITY_POLICY`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_SECURITY_POLICY;
```

#### Re-export `CONTENT_SECURITY_POLICY_REPORT_ONLY`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_SECURITY_POLICY_REPORT_ONLY;
```

#### Re-export `CONTENT_TYPE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::CONTENT_TYPE;
```

#### Re-export `COOKIE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::COOKIE;
```

#### Re-export `DNT`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::DNT;
```

#### Re-export `DATE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::DATE;
```

#### Re-export `ETAG`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ETAG;
```

#### Re-export `EXPECT`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::EXPECT;
```

#### Re-export `EXPIRES`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::EXPIRES;
```

#### Re-export `FORWARDED`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::FORWARDED;
```

#### Re-export `FROM`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::FROM;
```

#### Re-export `HOST`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::HOST;
```

#### Re-export `IF_MATCH`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::IF_MATCH;
```

#### Re-export `IF_MODIFIED_SINCE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::IF_MODIFIED_SINCE;
```

#### Re-export `IF_NONE_MATCH`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::IF_NONE_MATCH;
```

#### Re-export `IF_RANGE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::IF_RANGE;
```

#### Re-export `IF_UNMODIFIED_SINCE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::IF_UNMODIFIED_SINCE;
```

#### Re-export `LAST_MODIFIED`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::LAST_MODIFIED;
```

#### Re-export `LINK`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::LINK;
```

#### Re-export `LOCATION`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::LOCATION;
```

#### Re-export `MAX_FORWARDS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::MAX_FORWARDS;
```

#### Re-export `ORIGIN`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::ORIGIN;
```

#### Re-export `PRAGMA`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::PRAGMA;
```

#### Re-export `PROXY_AUTHENTICATE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::PROXY_AUTHENTICATE;
```

#### Re-export `PROXY_AUTHORIZATION`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::PROXY_AUTHORIZATION;
```

#### Re-export `PUBLIC_KEY_PINS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::PUBLIC_KEY_PINS;
```

#### Re-export `PUBLIC_KEY_PINS_REPORT_ONLY`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::PUBLIC_KEY_PINS_REPORT_ONLY;
```

#### Re-export `RANGE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::RANGE;
```

#### Re-export `REFERER`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::REFERER;
```

#### Re-export `REFERRER_POLICY`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::REFERRER_POLICY;
```

#### Re-export `REFRESH`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::REFRESH;
```

#### Re-export `RETRY_AFTER`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::RETRY_AFTER;
```

#### Re-export `SEC_WEBSOCKET_ACCEPT`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::SEC_WEBSOCKET_ACCEPT;
```

#### Re-export `SEC_WEBSOCKET_EXTENSIONS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::SEC_WEBSOCKET_EXTENSIONS;
```

#### Re-export `SEC_WEBSOCKET_KEY`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::SEC_WEBSOCKET_KEY;
```

#### Re-export `SEC_WEBSOCKET_PROTOCOL`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::SEC_WEBSOCKET_PROTOCOL;
```

#### Re-export `SEC_WEBSOCKET_VERSION`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::SEC_WEBSOCKET_VERSION;
```

#### Re-export `SERVER`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::SERVER;
```

#### Re-export `SET_COOKIE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::SET_COOKIE;
```

#### Re-export `STRICT_TRANSPORT_SECURITY`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::STRICT_TRANSPORT_SECURITY;
```

#### Re-export `TE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::TE;
```

#### Re-export `TRAILER`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::TRAILER;
```

#### Re-export `TRANSFER_ENCODING`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::TRANSFER_ENCODING;
```

#### Re-export `UPGRADE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::UPGRADE;
```

#### Re-export `UPGRADE_INSECURE_REQUESTS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::UPGRADE_INSECURE_REQUESTS;
```

#### Re-export `USER_AGENT`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::USER_AGENT;
```

#### Re-export `VARY`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::VARY;
```

#### Re-export `VIA`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::VIA;
```

#### Re-export `WARNING`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::WARNING;
```

#### Re-export `WWW_AUTHENTICATE`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::WWW_AUTHENTICATE;
```

#### Re-export `X_CONTENT_TYPE_OPTIONS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::X_CONTENT_TYPE_OPTIONS;
```

#### Re-export `X_DNS_PREFETCH_CONTROL`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::X_DNS_PREFETCH_CONTROL;
```

#### Re-export `X_FRAME_OPTIONS`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::X_FRAME_OPTIONS;
```

#### Re-export `X_XSS_PROTECTION`

**Attributes:**

- `#[rustfmt::skip]`

```rust
pub use self::name::X_XSS_PROTECTION;
```

## Module `method`

The HTTP request method

This module contains HTTP-method related structs and errors and such. The
main type of this module, `Method`, is also reexported at the root of the
crate as `http::Method` and is intended for import through that location
primarily.

# Examples

```
use http::Method;

assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
assert!(Method::GET.is_idempotent());
assert_eq!(Method::POST.as_str(), "POST");
```

```rust
pub mod method { /* ... */ }
```

### Types

#### Struct `Method`

The Request Method (VERB)

This type also contains constants for a number of common HTTP methods such
as GET, POST, etc.

Currently includes 8 variants representing the 8 methods defined in
[RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH,
and an Extension variant for all extensions.

# Examples

```
use http::Method;

assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
assert!(Method::GET.is_idempotent());
assert_eq!(Method::POST.as_str(), "POST");
```

```rust
pub struct Method(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> { /* ... */ }
  ```
  Converts a slice of bytes to an HTTP method.

- ```rust
  pub fn is_safe(self: &Self) -> bool { /* ... */ }
  ```
  Whether a method is considered "safe", meaning the request is

- ```rust
  pub fn is_idempotent(self: &Self) -> bool { /* ... */ }
  ```
  Whether a method is considered "idempotent", meaning the request has

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Return a &str representation of the HTTP method

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Method) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Method) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Method) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Method) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Method) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Method { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(t: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: &''a Method) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Method { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: &''a [u8]) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: &''a str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Send**
- **RefUnwindSafe**
#### Struct `InvalidMethod`

A possible error value when converting `Method` from bytes.

```rust
pub struct InvalidMethod {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: method::InvalidMethod) -> Error { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Error**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
## Module `request`

HTTP request types.

This module contains structs related to HTTP requests, notably the
`Request` type itself as well as a builder to create requests. Typically
you'll import the `http::Request` type rather than reaching into this
module itself.

# Examples

Creating a `Request` to send

```no_run
use http::{Request, Response};

let mut request = Request::builder()
    .uri("https://www.rust-lang.org/")
    .header("User-Agent", "my-awesome-agent/1.0");

if needs_awesome_header() {
    request = request.header("Awesome", "yes");
}

let response = send(request.body(()).unwrap());

# fn needs_awesome_header() -> bool {
#     true
# }
#
fn send(req: Request<()>) -> Response<()> {
    // ...
# panic!()
}
```

Inspecting a request to see what was sent.

```
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    if req.uri() != "/awesome-url" {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(())
    }

    let has_awesome_header = req.headers().contains_key("Awesome");
    let body = req.body();

    // ...
# panic!()
}
```

```rust
pub mod request { /* ... */ }
```

### Types

#### Struct `Request`

Represents an HTTP request.

An HTTP request consists of a head and a potentially optional body. The body
component is generic, enabling arbitrary types to represent the HTTP body.
For example, the body could be `Vec<u8>`, a `Stream` of byte chunks, or a
value that has been deserialized.

# Examples

Creating a `Request` to send

```no_run
use http::{Request, Response};

let mut request = Request::builder()
    .uri("https://www.rust-lang.org/")
    .header("User-Agent", "my-awesome-agent/1.0");

if needs_awesome_header() {
    request = request.header("Awesome", "yes");
}

let response = send(request.body(()).unwrap());

# fn needs_awesome_header() -> bool {
#     true
# }
#
fn send(req: Request<()>) -> Response<()> {
    // ...
# panic!()
}
```

Inspecting a request to see what was sent.

```
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    if req.uri() != "/awesome-url" {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(())
    }

    let has_awesome_header = req.headers().contains_key("Awesome");
    let body = req.body();

    // ...
# panic!()
}
```

Deserialize a request of bytes via json:

```
# extern crate serde;
# extern crate serde_json;
# extern crate http;
use http::Request;
use serde::de;

fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>>
    where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, body))
}
#
# fn main() {}
```

Or alternatively, serialize the body of a request to json

```
# extern crate serde;
# extern crate serde_json;
# extern crate http;
use http::Request;
use serde::ser;

fn serialize<T>(req: Request<T>) -> serde_json::Result<Request<Vec<u8>>>
    where T: ser::Serialize,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Request::from_parts(parts, body))
}
#
# fn main() {}
```

```rust
pub struct Request<T> {
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
  pub fn builder() -> Builder { /* ... */ }
  ```
  Creates a new builder-style object to manufacture a `Request`

- ```rust
  pub fn get<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a GET method and the given URI.

- ```rust
  pub fn put<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a PUT method and the given URI.

- ```rust
  pub fn post<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a POST method and the given URI.

- ```rust
  pub fn delete<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a DELETE method and the given URI.

- ```rust
  pub fn options<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with an OPTIONS method and the given URI.

- ```rust
  pub fn head<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a HEAD method and the given URI.

- ```rust
  pub fn connect<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a CONNECT method and the given URI.

- ```rust
  pub fn patch<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a PATCH method and the given URI.

- ```rust
  pub fn trace<T>(uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a TRACE method and the given URI.

- ```rust
  pub fn new(body: T) -> Request<T> { /* ... */ }
  ```
  Creates a new blank `Request` with the body

- ```rust
  pub fn from_parts(parts: Parts, body: T) -> Request<T> { /* ... */ }
  ```
  Creates a new `Request` with the given components parts and body.

- ```rust
  pub fn method(self: &Self) -> &Method { /* ... */ }
  ```
  Returns a reference to the associated HTTP method.

- ```rust
  pub fn method_mut(self: &mut Self) -> &mut Method { /* ... */ }
  ```
  Returns a mutable reference to the associated HTTP method.

- ```rust
  pub fn uri(self: &Self) -> &Uri { /* ... */ }
  ```
  Returns a reference to the associated URI.

- ```rust
  pub fn uri_mut(self: &mut Self) -> &mut Uri { /* ... */ }
  ```
  Returns a mutable reference to the associated URI.

- ```rust
  pub fn version(self: &Self) -> Version { /* ... */ }
  ```
  Returns the associated version.

- ```rust
  pub fn version_mut(self: &mut Self) -> &mut Version { /* ... */ }
  ```
  Returns a mutable reference to the associated version.

- ```rust
  pub fn headers(self: &Self) -> &HeaderMap<HeaderValue> { /* ... */ }
  ```
  Returns a reference to the associated header field map.

- ```rust
  pub fn headers_mut(self: &mut Self) -> &mut HeaderMap<HeaderValue> { /* ... */ }
  ```
  Returns a mutable reference to the associated header field map.

- ```rust
  pub fn extensions(self: &Self) -> &Extensions { /* ... */ }
  ```
  Returns a reference to the associated extensions.

- ```rust
  pub fn extensions_mut(self: &mut Self) -> &mut Extensions { /* ... */ }
  ```
  Returns a mutable reference to the associated extensions.

- ```rust
  pub fn body(self: &Self) -> &T { /* ... */ }
  ```
  Returns a reference to the associated HTTP body.

- ```rust
  pub fn body_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Returns a mutable reference to the associated HTTP body.

- ```rust
  pub fn into_body(self: Self) -> T { /* ... */ }
  ```
  Consumes the request, returning just the body.

- ```rust
  pub fn into_parts(self: Self) -> (Parts, T) { /* ... */ }
  ```
  Consumes the request returning the head and body parts.

- ```rust
  pub fn map<F, U>(self: Self, f: F) -> Request<U>
where
    F: FnOnce(T) -> U { /* ... */ }
  ```
  Consumes the request returning a new request with body mapped to the

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Request<T> { /* ... */ }
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

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Request<T> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Parts`

Component parts of an HTTP `Request`

The HTTP request head consists of a method, uri, version, and a set of
header fields.

```rust
pub struct Parts {
    pub method: crate::method::Method,
    pub uri: crate::Uri,
    pub version: crate::version::Version,
    pub headers: crate::header::HeaderMap<crate::header::HeaderValue>,
    pub extensions: crate::Extensions,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `method` | `crate::method::Method` | The request's method |
| `uri` | `crate::Uri` | The request's URI |
| `version` | `crate::version::Version` | The request's version |
| `headers` | `crate::header::HeaderMap<crate::header::HeaderValue>` | The request's headers |
| `extensions` | `crate::Extensions` | The request's extensions |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Parts { /* ... */ }
    ```

#### Struct `Builder`

An HTTP request builder

This type can be used to construct an instance or `Request`
through a builder-like pattern.

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
  Creates a new default instance of `Builder` to construct a `Request`.

- ```rust
  pub fn method<T>(self: Self, method: T) -> Builder
where
    T: TryInto<Method>,
    <T as TryInto<Method>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Set the HTTP method for this request.

- ```rust
  pub fn method_ref(self: &Self) -> Option<&Method> { /* ... */ }
  ```
  Get the HTTP Method for this request.

- ```rust
  pub fn uri<T>(self: Self, uri: T) -> Builder
where
    T: TryInto<Uri>,
    <T as TryInto<Uri>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Set the URI for this request.

- ```rust
  pub fn uri_ref(self: &Self) -> Option<&Uri> { /* ... */ }
  ```
  Get the URI for this request

- ```rust
  pub fn version(self: Self, version: Version) -> Builder { /* ... */ }
  ```
  Set the HTTP version for this request.

- ```rust
  pub fn version_ref(self: &Self) -> Option<&Version> { /* ... */ }
  ```
  Get the HTTP version for this request

- ```rust
  pub fn header<K, V>(self: Self, key: K, value: V) -> Builder
where
    K: TryInto<HeaderName>,
    <K as TryInto<HeaderName>>::Error: Into<crate::Error>,
    V: TryInto<HeaderValue>,
    <V as TryInto<HeaderValue>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Appends a header to this request builder.

- ```rust
  pub fn headers_ref(self: &Self) -> Option<&HeaderMap<HeaderValue>> { /* ... */ }
  ```
  Get header on this request builder.

- ```rust
  pub fn headers_mut(self: &mut Self) -> Option<&mut HeaderMap<HeaderValue>> { /* ... */ }
  ```
  Get headers on this request builder.

- ```rust
  pub fn extension<T>(self: Self, extension: T) -> Builder
where
    T: Clone + Any + Send + Sync + ''static { /* ... */ }
  ```
  Adds an extension to this builder

- ```rust
  pub fn extensions_ref(self: &Self) -> Option<&Extensions> { /* ... */ }
  ```
  Get a reference to the extensions for this request builder.

- ```rust
  pub fn extensions_mut(self: &mut Self) -> Option<&mut Extensions> { /* ... */ }
  ```
  Get a mutable reference to the extensions for this request builder.

- ```rust
  pub fn body<T>(self: Self, body: T) -> Result<Request<T>> { /* ... */ }
  ```
  "Consumes" this builder, using the provided `body` to return a

###### Trait Implementations

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
## Module `response`

HTTP response types.

This module contains structs related to HTTP responses, notably the
`Response` type itself as well as a builder to create responses. Typically
you'll import the `http::Response` type rather than reaching into this
module itself.

# Examples

Creating a `Response` to return

```
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}
```

A simple 404 handler

```
use http::{Request, Response, StatusCode};

fn not_found(_req: Request<()>) -> http::Result<Response<()>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
}
```

Or otherwise inspecting the result of a request:

```no_run
use http::{Request, Response};

fn get(url: &str) -> http::Result<Response<()>> {
    // ...
# panic!()
}

let response = get("https://www.rust-lang.org/").unwrap();

if !response.status().is_success() {
    panic!("failed to get a successful response status!");
}

if let Some(date) = response.headers().get("Date") {
    // we've got a `Date` header!
}

let body = response.body();
// ...
```

```rust
pub mod response { /* ... */ }
```

### Types

#### Struct `Response`

Represents an HTTP response

An HTTP response consists of a head and a potentially optional body. The body
component is generic, enabling arbitrary types to represent the HTTP body.
For example, the body could be `Vec<u8>`, a `Stream` of byte chunks, or a
value that has been deserialized.

Typically you'll work with responses on the client side as the result of
sending a `Request` and on the server you'll be generating a `Response` to
send back to the client.

# Examples

Creating a `Response` to return

```
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}
```

A simple 404 handler

```
use http::{Request, Response, StatusCode};

fn not_found(_req: Request<()>) -> http::Result<Response<()>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
}
```

Or otherwise inspecting the result of a request:

```no_run
use http::{Request, Response};

fn get(url: &str) -> http::Result<Response<()>> {
    // ...
# panic!()
}

let response = get("https://www.rust-lang.org/").unwrap();

if !response.status().is_success() {
    panic!("failed to get a successful response status!");
}

if let Some(date) = response.headers().get("Date") {
    // we've got a `Date` header!
}

let body = response.body();
// ...
```

Deserialize a response of bytes via json:

```
# extern crate serde;
# extern crate serde_json;
# extern crate http;
use http::Response;
use serde::de;

fn deserialize<T>(res: Response<Vec<u8>>) -> serde_json::Result<Response<T>>
    where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = res.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Response::from_parts(parts, body))
}
#
# fn main() {}
```

Or alternatively, serialize the body of a response to json

```
# extern crate serde;
# extern crate serde_json;
# extern crate http;
use http::Response;
use serde::ser;

fn serialize<T>(res: Response<T>) -> serde_json::Result<Response<Vec<u8>>>
    where T: ser::Serialize,
{
    let (parts, body) = res.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Response::from_parts(parts, body))
}
#
# fn main() {}
```

```rust
pub struct Response<T> {
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
  pub fn builder() -> Builder { /* ... */ }
  ```
  Creates a new builder-style object to manufacture a `Response`

- ```rust
  pub fn new(body: T) -> Response<T> { /* ... */ }
  ```
  Creates a new blank `Response` with the body

- ```rust
  pub fn from_parts(parts: Parts, body: T) -> Response<T> { /* ... */ }
  ```
  Creates a new `Response` with the given head and body

- ```rust
  pub fn status(self: &Self) -> StatusCode { /* ... */ }
  ```
  Returns the `StatusCode`.

- ```rust
  pub fn status_mut(self: &mut Self) -> &mut StatusCode { /* ... */ }
  ```
  Returns a mutable reference to the associated `StatusCode`.

- ```rust
  pub fn version(self: &Self) -> Version { /* ... */ }
  ```
  Returns a reference to the associated version.

- ```rust
  pub fn version_mut(self: &mut Self) -> &mut Version { /* ... */ }
  ```
  Returns a mutable reference to the associated version.

- ```rust
  pub fn headers(self: &Self) -> &HeaderMap<HeaderValue> { /* ... */ }
  ```
  Returns a reference to the associated header field map.

- ```rust
  pub fn headers_mut(self: &mut Self) -> &mut HeaderMap<HeaderValue> { /* ... */ }
  ```
  Returns a mutable reference to the associated header field map.

- ```rust
  pub fn extensions(self: &Self) -> &Extensions { /* ... */ }
  ```
  Returns a reference to the associated extensions.

- ```rust
  pub fn extensions_mut(self: &mut Self) -> &mut Extensions { /* ... */ }
  ```
  Returns a mutable reference to the associated extensions.

- ```rust
  pub fn body(self: &Self) -> &T { /* ... */ }
  ```
  Returns a reference to the associated HTTP body.

- ```rust
  pub fn body_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Returns a mutable reference to the associated HTTP body.

- ```rust
  pub fn into_body(self: Self) -> T { /* ... */ }
  ```
  Consumes the response, returning just the body.

- ```rust
  pub fn into_parts(self: Self) -> (Parts, T) { /* ... */ }
  ```
  Consumes the response returning the head and body parts.

- ```rust
  pub fn map<F, U>(self: Self, f: F) -> Response<U>
where
    F: FnOnce(T) -> U { /* ... */ }
  ```
  Consumes the response returning a new response with body mapped to the

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Response<T> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Response<T> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `Parts`

Component parts of an HTTP `Response`

The HTTP response head consists of a status, version, and a set of
header fields.

```rust
pub struct Parts {
    pub status: crate::status::StatusCode,
    pub version: crate::version::Version,
    pub headers: crate::header::HeaderMap<crate::header::HeaderValue>,
    pub extensions: crate::Extensions,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `crate::status::StatusCode` | The response's status |
| `version` | `crate::version::Version` | The response's version |
| `headers` | `crate::header::HeaderMap<crate::header::HeaderValue>` | The response's headers |
| `extensions` | `crate::Extensions` | The response's extensions |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Parts { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Builder`

An HTTP response builder

This type can be used to construct an instance of `Response` through a
builder-like pattern.

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
  Creates a new default instance of `Builder` to construct either a

- ```rust
  pub fn status<T>(self: Self, status: T) -> Builder
where
    T: TryInto<StatusCode>,
    <T as TryInto<StatusCode>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Set the HTTP status for this response.

- ```rust
  pub fn version(self: Self, version: Version) -> Builder { /* ... */ }
  ```
  Set the HTTP version for this response.

- ```rust
  pub fn header<K, V>(self: Self, key: K, value: V) -> Builder
where
    K: TryInto<HeaderName>,
    <K as TryInto<HeaderName>>::Error: Into<crate::Error>,
    V: TryInto<HeaderValue>,
    <V as TryInto<HeaderValue>>::Error: Into<crate::Error> { /* ... */ }
  ```
  Appends a header to this response builder.

- ```rust
  pub fn headers_ref(self: &Self) -> Option<&HeaderMap<HeaderValue>> { /* ... */ }
  ```
  Get header on this response builder.

- ```rust
  pub fn headers_mut(self: &mut Self) -> Option<&mut HeaderMap<HeaderValue>> { /* ... */ }
  ```
  Get header on this response builder.

- ```rust
  pub fn extension<T>(self: Self, extension: T) -> Builder
where
    T: Clone + Any + Send + Sync + ''static { /* ... */ }
  ```
  Adds an extension to this builder

- ```rust
  pub fn extensions_ref(self: &Self) -> Option<&Extensions> { /* ... */ }
  ```
  Get a reference to the extensions for this response builder.

- ```rust
  pub fn extensions_mut(self: &mut Self) -> Option<&mut Extensions> { /* ... */ }
  ```
  Get a mutable reference to the extensions for this response builder.

- ```rust
  pub fn body<T>(self: Self, body: T) -> Result<Response<T>> { /* ... */ }
  ```
  "Consumes" this builder, using the provided `body` to return a

###### Trait Implementations

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `status`

HTTP status codes

This module contains HTTP-status code related structs an errors. The main
type in this module is `StatusCode` which is not intended to be used through
this module but rather the `http::StatusCode` type.

# Examples

```
use http::StatusCode;

assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
assert_eq!(StatusCode::NOT_FOUND, 404);
assert!(StatusCode::OK.is_success());
```

```rust
pub mod status { /* ... */ }
```

### Types

#### Struct `StatusCode`

An HTTP status code (`status-code` in RFC 9110 et al.).

Constants are provided for known status codes, including those in the IANA
[HTTP Status Code Registry](
https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml).

Status code values in the range 100-999 (inclusive) are supported by this
type. Values in the range 100-599 are semantically classified by the most
significant digit. See [`StatusCode::is_success`], etc. Values above 599
are unclassified but allowed for legacy compatibility, though their use is
discouraged. Applications may interpret such values as protocol errors.

# Examples

```
use http::StatusCode;

assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
assert!(StatusCode::OK.is_success());
```

```rust
pub struct StatusCode(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode> { /* ... */ }
  ```
  Converts a u16 to a status code.

- ```rust
  pub fn from_bytes(src: &[u8]) -> Result<StatusCode, InvalidStatusCode> { /* ... */ }
  ```
  Converts a `&[u8]` to a status code.

- ```rust
  pub const fn as_u16(self: &Self) -> u16 { /* ... */ }
  ```
  Returns the `u16` corresponding to this `StatusCode`.

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Returns a &str representation of the `StatusCode`

- ```rust
  pub fn canonical_reason(self: &Self) -> Option<&''static str> { /* ... */ }
  ```
  Get the standardised `reason-phrase` for this status code.

- ```rust
  pub fn is_informational(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 100-199.

- ```rust
  pub fn is_success(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 200-299.

- ```rust
  pub fn is_redirection(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 300-399.

- ```rust
  pub fn is_client_error(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 400-499.

- ```rust
  pub fn is_server_error(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 500-599.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StatusCode) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &StatusCode) -> bool { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StatusCode { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(status: StatusCode) -> u16 { /* ... */ }
    ```

  - ```rust
    fn from(t: &''a StatusCode) -> Self { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<StatusCode, InvalidStatusCode> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &StatusCode) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: &''a [u8]) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: &''a str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: u16) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

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

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &StatusCode) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> StatusCode { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `InvalidStatusCode`

A possible error value when converting a `StatusCode` from a `u16` or `&str`.

This error indicates that the supplied input was not a valid number, was less
than 100, or was greater than 999.

```rust
pub struct InvalidStatusCode {
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: status::InvalidStatusCode) -> Error { /* ... */ }
    ```

- **Error**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `uri`

URI component of request and response lines

This module primarily contains the `Uri` type which is a component of all
HTTP requests and also reexports this type at the root of the crate. A URI
is not always a "full URL" in the sense of something you'd type into a web
browser, but HTTP requests may only have paths on servers but may have full
schemes and hostnames on clients.

# Examples

```
use http::Uri;

let uri = "/foo/bar?baz".parse::<Uri>().unwrap();
assert_eq!(uri.path(), "/foo/bar");
assert_eq!(uri.query(), Some("baz"));
assert_eq!(uri.host(), None);

let uri = "https://www.rust-lang.org/install.html".parse::<Uri>().unwrap();
assert_eq!(uri.scheme_str(), Some("https"));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/install.html");
```

```rust
pub mod uri { /* ... */ }
```

### Types

#### Struct `Uri`

The URI component of a request.

For HTTP 1, this is included as part of the request line. From Section 5.3,
Request Target:

> Once an inbound connection is obtained, the client sends an HTTP
> request message (Section 3) with a request-target derived from the
> target URI.  There are four distinct formats for the request-target,
> depending on both the method being requested and whether the request
> is to a proxy.
>
> ```notrust
> request-target = origin-form
>                / absolute-form
>                / authority-form
>                / asterisk-form
> ```

The URI is structured as follows:

```notrust
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
|-|   |-------------------------------||--------| |-------------------| |-----|
 |                  |                       |               |              |
scheme          authority                 path            query         fragment
```

For HTTP 2.0, the URI is encoded using pseudoheaders.

# Examples

```
use http::Uri;

let uri = "/foo/bar?baz".parse::<Uri>().unwrap();
assert_eq!(uri.path(), "/foo/bar");
assert_eq!(uri.query(), Some("baz"));
assert_eq!(uri.host(), None);

let uri = "https://www.rust-lang.org/install.html".parse::<Uri>().unwrap();
assert_eq!(uri.scheme_str(), Some("https"));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/install.html");
```

```rust
pub struct Uri {
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
  pub fn builder() -> Builder { /* ... */ }
  ```
  Creates a new builder-style object to manufacture a `Uri`.

- ```rust
  pub fn from_parts(src: Parts) -> Result<Uri, InvalidUriParts> { /* ... */ }
  ```
  Attempt to convert a `Parts` into a `Uri`.

- ```rust
  pub fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>
where
    T: AsRef<[u8]> + ''static { /* ... */ }
  ```
  Attempt to convert a `Bytes` buffer to a `Uri`.

- ```rust
  pub fn from_static(src: &''static str) -> Self { /* ... */ }
  ```
  Convert a `Uri` from a static string.

- ```rust
  pub fn into_parts(self: Self) -> Parts { /* ... */ }
  ```
  Convert a `Uri` into `Parts`.

- ```rust
  pub fn path_and_query(self: &Self) -> Option<&PathAndQuery> { /* ... */ }
  ```
  Returns the path & query components of the Uri

- ```rust
  pub fn path(self: &Self) -> &str { /* ... */ }
  ```
  Get the path of this `Uri`.

- ```rust
  pub fn scheme(self: &Self) -> Option<&Scheme> { /* ... */ }
  ```
  Get the scheme of this `Uri`.

- ```rust
  pub fn scheme_str(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Get the scheme of this `Uri` as a `&str`.

- ```rust
  pub fn authority(self: &Self) -> Option<&Authority> { /* ... */ }
  ```
  Get the authority of this `Uri`.

- ```rust
  pub fn host(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Get the host of this `Uri`.

- ```rust
  pub fn port(self: &Self) -> Option<Port<&str>> { /* ... */ }
  ```
  Get the port part of this `Uri`.

- ```rust
  pub fn port_u16(self: &Self) -> Option<u16> { /* ... */ }
  ```
  Get the port of this `Uri` as a `u16`.

- ```rust
  pub fn query(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Get the query string of this `Uri`, starting after the `?`.

###### Trait Implementations

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(uri: Uri) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(authority: Authority) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(path_and_query: PathAndQuery) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(src: Uri) -> Self { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: &''a [u8]) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: &''a str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: &''a String) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(t: String) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(src: Parts) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(src: &''a Uri) -> Result<Self, <Self as >::Error> { /* ... */ }
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
    fn clone(self: &Self) -> Uri { /* ... */ }
    ```

- **Eq**
- **Default**
  - ```rust
    fn default() -> Uri { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: Hasher { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Uri) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, uri: &Uri) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, uri: &Uri) -> bool { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Uri, InvalidUri> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Parts`

The various parts of a URI.

This struct is used to provide to and retrieve from a URI.

```rust
pub struct Parts {
    pub scheme: Option<Scheme>,
    pub authority: Option<Authority>,
    pub path_and_query: Option<PathAndQuery>,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `scheme` | `Option<Scheme>` | The scheme component of a URI |
| `authority` | `Option<Authority>` | The authority component of a URI |
| `path_and_query` | `Option<PathAndQuery>` | The origin-form component of a URI |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Parts { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(src: Parts) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: Uri) -> Self { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `InvalidUri`

An error resulting from a failed attempt to construct a URI.

```rust
pub struct InvalidUri(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Sync**
- **Error**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: uri::InvalidUri) -> Error { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `InvalidUriParts`

An error resulting from a failed attempt to construct a URI.

```rust
pub struct InvalidUriParts(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: uri::InvalidUriParts) -> Error { /* ... */ }
    ```

- **Error**
### Re-exports

#### Re-export `Authority`

```rust
pub use self::authority::Authority;
```

#### Re-export `Builder`

```rust
pub use self::builder::Builder;
```

#### Re-export `PathAndQuery`

```rust
pub use self::path::PathAndQuery;
```

#### Re-export `Port`

```rust
pub use self::port::Port;
```

#### Re-export `Scheme`

```rust
pub use self::scheme::Scheme;
```

## Module `version`

HTTP version

This module contains a definition of the `Version` type. The `Version`
type is intended to be accessed through the root of the crate
(`http::Version`) rather than this module.

The `Version` type contains constants that represent the various versions
of the HTTP protocol.

# Examples

```
use http::Version;

let http11 = Version::HTTP_11;
let http2 = Version::HTTP_2;
assert!(http11 != http2);

println!("{:?}", http2);
```

```rust
pub mod version { /* ... */ }
```

### Types

#### Struct `Version`

Represents a version of the HTTP spec.

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Version { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Version) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Version { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Version) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Version) -> $crate::cmp::Ordering { /* ... */ }
    ```

## Re-exports

### Re-export `Error`

```rust
pub use crate::error::Error;
```

### Re-export `Result`

```rust
pub use crate::error::Result;
```

### Re-export `Extensions`

```rust
pub use crate::extensions::Extensions;
```

### Re-export `HeaderMap`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::header::HeaderMap;
```

### Re-export `HeaderName`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::header::HeaderName;
```

### Re-export `HeaderValue`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::header::HeaderValue;
```

### Re-export `Method`

```rust
pub use crate::method::Method;
```

### Re-export `Request`

```rust
pub use crate::request::Request;
```

### Re-export `Response`

```rust
pub use crate::response::Response;
```

### Re-export `StatusCode`

```rust
pub use crate::status::StatusCode;
```

### Re-export `Uri`

```rust
pub use crate::uri::Uri;
```

### Re-export `Version`

```rust
pub use crate::version::Version;
```

