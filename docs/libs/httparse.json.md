# Crate Documentation

**Version:** 1.10.1

**Format Version:** 43

# Module `httparse`

# httparse

A push library for parsing HTTP/1.x requests and responses.

The focus is on speed and safety. Unsafe code is used to keep parsing fast,
but unsafety is contained in a submodule, with invariants enforced. The
parsing internals use an `Iterator` instead of direct indexing, while
skipping bounds checks.

With Rust 1.27.0 or later, support for SIMD is enabled automatically.
If building an executable to be run on multiple platforms, and thus
not passing `target_feature` or `target_cpu` flags to the compiler,
runtime detection can still detect SSE4.2 or AVX2 support to provide
massive wins.

If compiling for a specific target, remembering to include
`-C target_cpu=native` allows the detection to become compile time checks,
making it *even* faster.

## Types

### Enum `Error`

An error in parsing.

```rust
pub enum Error {
    HeaderName,
    HeaderValue,
    NewLine,
    Status,
    Token,
    TooManyHeaders,
    Version,
}
```

#### Variants

##### `HeaderName`

Invalid byte in header name.

##### `HeaderValue`

Invalid byte in header value.

##### `NewLine`

Invalid byte in new line.

##### `Status`

Invalid byte in Response status.

##### `Token`

Invalid byte where token is required.

##### `TooManyHeaders`

Parsed more headers than provided buffer can contain.

##### `Version`

Invalid byte in HTTP version.

#### Implementations

##### Trait Implementations

- **StructuralPartialEq**
- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Eq**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Struct `InvalidChunkSize`

An error in parsing a chunk size.

```rust
pub struct InvalidChunkSize;
```

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InvalidChunkSize) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
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
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

### Type Alias `Result`

A Result of any parsing action.

If the input is invalid, an `Error` will be returned. Note that incomplete
data is not considered invalid, and so will not return an error, but rather
a `Ok(Status::Partial)`.

```rust
pub type Result<T> = result::Result<Status<T>, Error>;
```

### Enum `Status`

The result of a successful parse pass.

`Complete` is used when the buffer contained the complete value.
`Partial` is used when parsing did not reach the end of the expected value,
but no invalid data was found.

```rust
pub enum Status<T> {
    Complete(T),
    Partial,
}
```

#### Variants

##### `Complete`

The completed result.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### `Partial`

A partial result.

#### Implementations

##### Methods

- ```rust
  pub fn is_complete(self: &Self) -> bool { /* ... */ }
  ```
  Convenience method to check if status is complete.

- ```rust
  pub fn is_partial(self: &Self) -> bool { /* ... */ }
  ```
  Convenience method to check if status is partial.

- ```rust
  pub fn unwrap(self: Self) -> T { /* ... */ }
  ```
  Convenience method to unwrap a Complete value. Panics if the status is

##### Trait Implementations

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Status<T> { /* ... */ }
    ```

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

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **Copy**
- **Eq**
- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Status<T>) -> bool { /* ... */ }
    ```

### Struct `ParserConfig`

Parser configuration.

```rust
pub struct ParserConfig {
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
  pub fn allow_spaces_after_header_name_in_responses(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether spaces and tabs should be allowed after header names in responses.

- ```rust
  pub fn allow_multiple_spaces_in_request_line_delimiters(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether multiple spaces are allowed as delimiters in request lines.

- ```rust
  pub fn multiple_spaces_in_request_line_delimiters_are_allowed(self: &Self) -> bool { /* ... */ }
  ```
  Whether multiple spaces are allowed as delimiters in request lines.

- ```rust
  pub fn allow_multiple_spaces_in_response_status_delimiters(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether multiple spaces are allowed as delimiters in response status lines.

- ```rust
  pub fn multiple_spaces_in_response_status_delimiters_are_allowed(self: &Self) -> bool { /* ... */ }
  ```
  Whether multiple spaces are allowed as delimiters in response status lines.

- ```rust
  pub fn allow_obsolete_multiline_headers_in_responses(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether obsolete multiline headers should be allowed.

- ```rust
  pub fn obsolete_multiline_headers_in_responses_are_allowed(self: &Self) -> bool { /* ... */ }
  ```
  Whether obsolete multiline headers should be allowed.

- ```rust
  pub fn allow_space_before_first_header_name(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether white space before the first header is allowed

- ```rust
  pub fn space_before_first_header_name_are_allowed(self: &Self) -> bool { /* ... */ }
  ```
  Whether white space before first header is allowed or not

- ```rust
  pub fn parse_request<''buf>(self: &Self, request: &mut Request<''_, ''buf>, buf: &''buf [u8]) -> Result<usize> { /* ... */ }
  ```
  Parses a request with the given config.

- ```rust
  pub fn parse_request_with_uninit_headers<''headers, ''buf>(self: &Self, request: &mut Request<''headers, ''buf>, buf: &''buf [u8], headers: &''headers mut [MaybeUninit<Header<''buf>>]) -> Result<usize> { /* ... */ }
  ```
  Parses a request with the given config and buffer for headers

- ```rust
  pub fn ignore_invalid_headers_in_responses(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether invalid header lines should be silently ignored in responses.

- ```rust
  pub fn ignore_invalid_headers_in_requests(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets whether invalid header lines should be silently ignored in requests.

- ```rust
  pub fn parse_response<''buf>(self: &Self, response: &mut Response<''_, ''buf>, buf: &''buf [u8]) -> Result<usize> { /* ... */ }
  ```
  Parses a response with the given config.

- ```rust
  pub fn parse_response_with_uninit_headers<''headers, ''buf>(self: &Self, response: &mut Response<''headers, ''buf>, buf: &''buf [u8], headers: &''headers mut [MaybeUninit<Header<''buf>>]) -> Result<usize> { /* ... */ }
  ```
  Parses a response with the given config and buffer for headers

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> ParserConfig { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParserConfig { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

### Struct `Request`

A parsed Request.

The optional values will be `None` if a parse was not complete, and did not
parse the associated property. This allows you to inspect the parts that
could be parsed, before reading more, in case you wish to exit early.

# Example

```no_run
let buf = b"GET /404 HTTP/1.1\r\nHost:";
let mut headers = [httparse::EMPTY_HEADER; 16];
let mut req = httparse::Request::new(&mut headers);
let res = req.parse(buf).unwrap();
if res.is_partial() {
    match req.path {
        Some(ref path) => {
            // check router for path.
            // /404 doesn't exist? we could stop parsing
        },
        None => {
            // must read more and parse again
        }
    }
}
```

```rust
pub struct Request<''headers, ''buf> {
    pub method: Option<&''buf str>,
    pub path: Option<&''buf str>,
    pub version: Option<u8>,
    pub headers: &''headers mut [Header<''buf>],
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `method` | `Option<&''buf str>` | The request method, such as `GET`. |
| `path` | `Option<&''buf str>` | The request path, such as `/about-us`. |
| `version` | `Option<u8>` | The request minor version, such as `1` for `HTTP/1.1`. |
| `headers` | `&''headers mut [Header<''buf>]` | The request headers. |

#### Implementations

##### Methods

- ```rust
  pub fn new(headers: &''h mut [Header<''b>]) -> Request<''h, ''b> { /* ... */ }
  ```
  Creates a new Request, using a slice of headers you allocate.

- ```rust
  pub fn parse_with_uninit_headers(self: &mut Self, buf: &''b [u8], headers: &''h mut [MaybeUninit<Header<''b>>]) -> Result<usize> { /* ... */ }
  ```
  Try to parse a buffer of bytes into the Request,

- ```rust
  pub fn parse(self: &mut Self, buf: &''b [u8]) -> Result<usize> { /* ... */ }
  ```
  Try to parse a buffer of bytes into the Request.

##### Trait Implementations

- **StructuralPartialEq**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Request<''headers, ''buf>) -> bool { /* ... */ }
    ```

- **Send**
- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `Response`

A parsed Response.

See `Request` docs for explanation of optional values.

```rust
pub struct Response<''headers, ''buf> {
    pub version: Option<u8>,
    pub code: Option<u16>,
    pub reason: Option<&''buf str>,
    pub headers: &''headers mut [Header<''buf>],
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `version` | `Option<u8>` | The response minor version, such as `1` for `HTTP/1.1`. |
| `code` | `Option<u16>` | The response code, such as `200`. |
| `reason` | `Option<&''buf str>` | The response reason-phrase, such as `OK`.<br><br>Contains an empty string if the reason-phrase was missing or contained invalid characters. |
| `headers` | `&''headers mut [Header<''buf>]` | The response headers. |

#### Implementations

##### Methods

- ```rust
  pub fn new(headers: &''h mut [Header<''b>]) -> Response<''h, ''b> { /* ... */ }
  ```
  Creates a new `Response` using a slice of `Header`s you have allocated.

- ```rust
  pub fn parse(self: &mut Self, buf: &''b [u8]) -> Result<usize> { /* ... */ }
  ```
  Try to parse a buffer of bytes into this `Response`.

##### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **UnwindSafe**
- **StructuralPartialEq**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Response<''headers, ''buf>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `Header`

Represents a parsed header.

```rust
pub struct Header<''a> {
    pub name: &''a str,
    pub value: &''a [u8],
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `&''a str` | The name portion of a header.<br><br>A header name must be valid ASCII-US, so it's safe to store as a `&str`. |
| `value` | `&''a [u8]` | The value portion of a header.<br><br>While headers **should** be ASCII-US, the specification allows for<br>values that may not be, and so the value is stored as bytes. |

#### Implementations

##### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Header<''a>) -> bool { /* ... */ }
    ```

- **Send**
- **Unpin**
- **UnwindSafe**
- **Eq**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Header<''a> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Functions

### Function `parse_headers`

Parse a buffer of bytes as headers.

The return value, if complete and successful, includes the index of the
buffer that parsing stopped at, and a sliced reference to the parsed
headers. The length of the slice will be equal to the number of properly
parsed headers.

# Example

```
let buf = b"Host: foo.bar\nAccept: */*\n\nblah blah";
let mut headers = [httparse::EMPTY_HEADER; 4];
assert_eq!(httparse::parse_headers(buf, &mut headers),
           Ok(httparse::Status::Complete((27, &[
               httparse::Header { name: "Host", value: b"foo.bar" },
               httparse::Header { name: "Accept", value: b"*/*" }
           ][..]))));
```

```rust
pub fn parse_headers<''b: ''h, ''h>(src: &''b [u8], dst: &''h mut [Header<''b>]) -> Result<(usize, &''h [Header<''b>])> { /* ... */ }
```

### Function `parse_chunk_size`

Parse a buffer of bytes as a chunk size.

The return value, if complete and successful, includes the index of the
buffer that parsing stopped at, and the size of the following chunk.

# Example

```
let buf = b"4\r\nRust\r\n0\r\n\r\n";
assert_eq!(httparse::parse_chunk_size(buf),
           Ok(httparse::Status::Complete((3, 4))));
```

```rust
pub fn parse_chunk_size(buf: &[u8]) -> result::Result<Status<(usize, u64)>, InvalidChunkSize> { /* ... */ }
```

## Constants and Statics

### Constant `EMPTY_HEADER`

An empty header, useful for constructing a `Header` array to pass in for
parsing.

# Example

```
let headers = [httparse::EMPTY_HEADER; 64];
```

```rust
pub const EMPTY_HEADER: Header<''static> = _;
```

