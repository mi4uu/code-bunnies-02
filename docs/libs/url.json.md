# Crate Documentation

**Version:** 2.5.4

**Format Version:** 43

# Module `url`


rust-url is an implementation of the [URL Standard](http://url.spec.whatwg.org/)
for the [Rust](http://rust-lang.org/) programming language.


# URL parsing and data structures

First, URL parsing may fail for various reasons and therefore returns a `Result`.

```
use url::{Url, ParseError};

assert!(Url::parse("http://[:::1]") == Err(ParseError::InvalidIpv6Address))
```

Let’s parse a valid URL and look at its components.

```
use url::{Url, Host, Position};
# use url::ParseError;
# fn run() -> Result<(), ParseError> {
let issue_list_url = Url::parse(
    "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
)?;


assert!(issue_list_url.scheme() == "https");
assert!(issue_list_url.username() == "");
assert!(issue_list_url.password() == None);
assert!(issue_list_url.host_str() == Some("github.com"));
assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
assert!(issue_list_url.port() == None);
assert!(issue_list_url.path() == "/rust-lang/rust/issues");
assert!(issue_list_url.path_segments().map(|c| c.collect::<Vec<_>>()) ==
        Some(vec!["rust-lang", "rust", "issues"]));
assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
assert!(&issue_list_url[Position::BeforePath..] == "/rust-lang/rust/issues?labels=E-easy&state=open");
assert!(issue_list_url.fragment() == None);
assert!(!issue_list_url.cannot_be_a_base());
# Ok(())
# }
# run().unwrap();
```

Some URLs are said to be *cannot-be-a-base*:
they don’t have a username, password, host, or port,
and their "path" is an arbitrary string rather than slash-separated segments:

```
use url::Url;
# use url::ParseError;

# fn run() -> Result<(), ParseError> {
let data_url = Url::parse("data:text/plain,Hello?World#")?;

assert!(data_url.cannot_be_a_base());
assert!(data_url.scheme() == "data");
assert!(data_url.path() == "text/plain,Hello");
assert!(data_url.path_segments().is_none());
assert!(data_url.query() == Some("World"));
assert!(data_url.fragment() == Some(""));
# Ok(())
# }
# run().unwrap();
```

## Default Features

Versions `<= 2.5.2` of the crate have no default features. Versions `> 2.5.2` have the default feature 'std'.
If you are upgrading across this boundary and you have specified `default-features = false`, then
you will need to add the 'std' feature or the 'alloc' feature to your dependency.
The 'std' feature has the same behavior as the previous versions. The 'alloc' feature
provides no_std support.

## Serde

Enable the `serde` feature to include `Deserialize` and `Serialize` implementations for `url::Url`.

# Base URL

Many contexts allow URL *references* that can be relative to a *base URL*:

```html
<link rel="stylesheet" href="../main.css">
```

Since parsed URLs are absolute, giving a base is required for parsing relative URLs:

```
use url::{Url, ParseError};

assert!(Url::parse("../main.css") == Err(ParseError::RelativeUrlWithoutBase))
```

Use the `join` method on an `Url` to use it as a base URL:

```
use url::Url;
# use url::ParseError;

# fn run() -> Result<(), ParseError> {
let this_document = Url::parse("http://servo.github.io/rust-url/url/index.html")?;
let css_url = this_document.join("../main.css")?;
assert_eq!(css_url.as_str(), "http://servo.github.io/rust-url/main.css");
# Ok(())
# }
# run().unwrap();
```

# Feature: `serde`

If you enable the `serde` feature, [`Url`](struct.Url.html) will implement
[`serde::Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html) and
[`serde::Deserialize`](https://docs.rs/serde/1/serde/trait.Deserialize.html).
See [serde documentation](https://serde.rs) for more information.

```toml
url = { version = "2", features = ["serde"] }
```

# Feature: `debugger_visualizer`

If you enable the `debugger_visualizer` feature, the `url` crate will include
a [natvis file](https://docs.microsoft.com/en-us/visualstudio/debugger/create-custom-views-of-native-objects)
for [Visual Studio](https://www.visualstudio.com/) that allows you to view
[`Url`](struct.Url.html) objects in the debugger.

This feature requires Rust 1.71 or later.

```toml
url = { version = "2", features = ["debugger_visualizer"] }
```

## Types

### Struct `Url`

A parsed URL record.

```rust
pub struct Url {
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
  pub fn parse(input: &str) -> Result<Url, crate::ParseError> { /* ... */ }
  ```
  Parse an absolute URL from a string.

- ```rust
  pub fn parse_with_params<I, K, V>(input: &str, iter: I) -> Result<Url, crate::ParseError>
where
    I: IntoIterator,
    <I as >::Item: Borrow<(K, V)>,
    K: AsRef<str>,
    V: AsRef<str> { /* ... */ }
  ```
  Parse an absolute URL from a string and add params to its query string.

- ```rust
  pub fn join(self: &Self, input: &str) -> Result<Url, crate::ParseError> { /* ... */ }
  ```
  Parse a string as an URL, with this URL as the base URL.

- ```rust
  pub fn make_relative(self: &Self, url: &Url) -> Option<String> { /* ... */ }
  ```
  Creates a relative URL if possible, with this URL as the base URL.

- ```rust
  pub fn options<''a>() -> ParseOptions<''a> { /* ... */ }
  ```
  Return a default `ParseOptions` that can fully configure the URL parser.

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Return the serialization of this URL.

- ```rust
  pub fn into_string(self: Self) -> String { /* ... */ }
  ```
  Return the serialization of this URL.

- ```rust
  pub fn origin(self: &Self) -> Origin { /* ... */ }
  ```
  Return the origin of this URL (<https://url.spec.whatwg.org/#origin>)

- ```rust
  pub fn scheme(self: &Self) -> &str { /* ... */ }
  ```
  Return the scheme of this URL, lower-cased, as an ASCII string without the ':' delimiter.

- ```rust
  pub fn is_special(self: &Self) -> bool { /* ... */ }
  ```
  Return whether the URL is special (has a special scheme)

- ```rust
  pub fn has_authority(self: &Self) -> bool { /* ... */ }
  ```
  Return whether the URL has an 'authority',

- ```rust
  pub fn authority(self: &Self) -> &str { /* ... */ }
  ```
  Return the authority of this URL as an ASCII string.

- ```rust
  pub fn cannot_be_a_base(self: &Self) -> bool { /* ... */ }
  ```
  Return whether this URL is a cannot-be-a-base URL,

- ```rust
  pub fn username(self: &Self) -> &str { /* ... */ }
  ```
  Return the username for this URL (typically the empty string)

- ```rust
  pub fn password(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Return the password for this URL, if any, as a percent-encoded ASCII string.

- ```rust
  pub fn has_host(self: &Self) -> bool { /* ... */ }
  ```
  Equivalent to `url.host().is_some()`.

- ```rust
  pub fn host_str(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Return the string representation of the host (domain or IP address) for this URL, if any.

- ```rust
  pub fn host(self: &Self) -> Option<Host<&str>> { /* ... */ }
  ```
  Return the parsed representation of the host for this URL.

- ```rust
  pub fn domain(self: &Self) -> Option<&str> { /* ... */ }
  ```
  If this URL has a host and it is a domain name (not an IP address), return it.

- ```rust
  pub fn port(self: &Self) -> Option<u16> { /* ... */ }
  ```
  Return the port number for this URL, if any.

- ```rust
  pub fn port_or_known_default(self: &Self) -> Option<u16> { /* ... */ }
  ```
  Return the port number for this URL, or the default port number if it is known.

- ```rust
  pub fn socket_addrs</* synthetic */ impl Fn() -> Option<u16>: Fn() -> Option<u16>>(self: &Self, default_port_number: impl Fn() -> Option<u16>) -> io::Result<alloc::vec::Vec<SocketAddr>> { /* ... */ }
  ```
  Resolve a URL’s host and port number to `SocketAddr`.

- ```rust
  pub fn path(self: &Self) -> &str { /* ... */ }
  ```
  Return the path for this URL, as a percent-encoded ASCII string.

- ```rust
  pub fn path_segments(self: &Self) -> Option<str::Split<''_, char>> { /* ... */ }
  ```
  Unless this URL is cannot-be-a-base,

- ```rust
  pub fn query(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Return this URL’s query string, if any, as a percent-encoded ASCII string.

- ```rust
  pub fn query_pairs(self: &Self) -> form_urlencoded::Parse<''_> { /* ... */ }
  ```
  Parse the URL’s query string, if any, as `application/x-www-form-urlencoded`

- ```rust
  pub fn fragment(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Return this URL’s fragment identifier, if any.

- ```rust
  pub fn set_fragment(self: &mut Self, fragment: Option<&str>) { /* ... */ }
  ```
  Change this URL’s fragment identifier.

- ```rust
  pub fn set_query(self: &mut Self, query: Option<&str>) { /* ... */ }
  ```
  Change this URL’s query string. If `query` is `None`, this URL's

- ```rust
  pub fn query_pairs_mut(self: &mut Self) -> form_urlencoded::Serializer<''_, UrlQuery<''_>> { /* ... */ }
  ```
  Manipulate this URL’s query string, viewed as a sequence of name/value pairs

- ```rust
  pub fn set_path(self: &mut Self, path: &str) { /* ... */ }
  ```
  Change this URL’s path.

- ```rust
  pub fn path_segments_mut(self: &mut Self) -> Result<PathSegmentsMut<''_>, ()> { /* ... */ }
  ```
  Return an object with methods to manipulate this URL’s path segments.

- ```rust
  pub fn set_port(self: &mut Self, port: Option<u16>) -> Result<(), ()> { /* ... */ }
  ```
  Change this URL’s port number.

- ```rust
  pub fn set_host(self: &mut Self, host: Option<&str>) -> Result<(), ParseError> { /* ... */ }
  ```
  Change this URL’s host.

- ```rust
  pub fn set_ip_host(self: &mut Self, address: IpAddr) -> Result<(), ()> { /* ... */ }
  ```
  Change this URL’s host to the given IP address.

- ```rust
  pub fn set_password(self: &mut Self, password: Option<&str>) -> Result<(), ()> { /* ... */ }
  ```
  Change this URL’s password.

- ```rust
  pub fn set_username(self: &mut Self, username: &str) -> Result<(), ()> { /* ... */ }
  ```
  Change this URL’s username.

- ```rust
  pub fn set_scheme(self: &mut Self, scheme: &str) -> Result<(), ()> { /* ... */ }
  ```
  Change this URL’s scheme.

- ```rust
  pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Url, ()> { /* ... */ }
  ```
  Convert a file name as `std::path::Path` into an URL in the `file` scheme.

- ```rust
  pub fn from_directory_path<P: AsRef<std::path::Path>>(path: P) -> Result<Url, ()> { /* ... */ }
  ```
  Convert a directory name as `std::path::Path` into an URL in the `file` scheme.

- ```rust
  pub fn to_file_path(self: &Self) -> Result<PathBuf, ()> { /* ... */ }
  ```
  Assuming the URL is in the `file` scheme or similar,

##### Trait Implementations

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Index**
  - ```rust
    fn index(self: &Self, _: RangeFull) -> &str { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: RangeFrom<Position>) -> &str { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: RangeTo<Position>) -> &str { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: Range<Position>) -> &str { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: Url) -> String { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: hash::Hasher { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(s: &''a str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Url { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(input: &str) -> Result<Url, crate::ParseError> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **ErasedDestructor**
### Struct `ParseOptions`

**Attributes:**

- `#[must_use]`

Full configuration for the URL parser.

```rust
pub struct ParseOptions<''a> {
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
  pub fn base_url(self: Self, new: Option<&''a Url>) -> Self { /* ... */ }
  ```
  Change the base URL

- ```rust
  pub fn encoding_override(self: Self, new: EncodingOverride<''a>) -> Self { /* ... */ }
  ```
  Override the character encoding of query strings.

- ```rust
  pub fn syntax_violation_callback(self: Self, new: Option<&''a dyn Fn(SyntaxViolation)>) -> Self { /* ... */ }
  ```
  Call the provided function or closure for a non-fatal `SyntaxViolation`

- ```rust
  pub fn parse(self: Self, input: &str) -> Result<Url, crate::ParseError> { /* ... */ }
  ```
  Parse an URL string with the configuration so far.

##### Trait Implementations

- **Sync**
- **Unpin**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
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

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParseOptions<''a> { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
### Struct `UrlQuery`

Implementation detail of `Url::query_pairs_mut`. Typically not used directly.

```rust
pub struct UrlQuery<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **RefUnwindSafe**
- **ErasedDestructor**
- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Target**
  - ```rust
    fn as_mut_string(self: &mut Self) -> &mut String { /* ... */ }
    ```

  - ```rust
    fn finish(self: Self) -> &''a mut Url { /* ... */ }
    ```

- **Send**
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

- **UnwindSafe**
- **Unpin**
## Re-exports

### Re-export `form_urlencoded`

```rust
pub use form_urlencoded;
```

### Re-export `Host`

```rust
pub use crate::host::Host;
```

### Re-export `OpaqueOrigin`

```rust
pub use crate::origin::OpaqueOrigin;
```

### Re-export `Origin`

```rust
pub use crate::origin::Origin;
```

### Re-export `ParseError`

```rust
pub use crate::parser::ParseError;
```

### Re-export `SyntaxViolation`

```rust
pub use crate::parser::SyntaxViolation;
```

### Re-export `PathSegmentsMut`

```rust
pub use crate::path_segments::PathSegmentsMut;
```

### Re-export `Position`

```rust
pub use crate::slicing::Position;
```

### Re-export `EncodingOverride`

```rust
pub use form_urlencoded::EncodingOverride;
```

