# Crate Documentation

**Version:** 0.3.17

**Format Version:** 43

# Module `mime`

# Mime

Mime is now Media Type, technically, but `Mime` is more immediately
understandable, so the main type here is `Mime`.

## What is Mime?

Example mime string: `text/plain`

```
let plain_text: mime::Mime = "text/plain".parse().unwrap();
assert_eq!(plain_text, mime::TEXT_PLAIN);
```

## Inspecting Mimes

```
let mime = mime::TEXT_PLAIN;
match (mime.type_(), mime.subtype()) {
    (mime::TEXT, mime::PLAIN) => println!("plain text!"),
    (mime::TEXT, _) => println!("structured text"),
    _ => println!("not text"),
}
```

## Types

### Struct `Mime`

A parsed mime or media type.

```rust
pub struct Mime {
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
  pub fn type_(self: &Self) -> Name<''_> { /* ... */ }
  ```
  Get the top level media type for this `Mime`.

- ```rust
  pub fn subtype(self: &Self) -> Name<''_> { /* ... */ }
  ```
  Get the subtype of this `Mime`.

- ```rust
  pub fn suffix(self: &Self) -> Option<Name<''_>> { /* ... */ }
  ```
  Get an optional +suffix for this `Mime`.

- ```rust
  pub fn get_param<''a, N>(self: &''a Self, attr: N) -> Option<Name<''a>>
where
    N: PartialEq<Name<''a>> { /* ... */ }
  ```
  Look up a parameter by name.

- ```rust
  pub fn params<''a>(self: &''a Self) -> Params<''a> { /* ... */ }
  ```
  Returns an iterator over the parameters.

- ```rust
  pub fn essence_str(self: &Self) -> &str { /* ... */ }
  ```
  Return a `&str` of the Mime's ["essence"][essence].

##### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Mime) -> Option<Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Send**
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
    fn clone(self: &Self) -> Mime { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Mime) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, s: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, mime: &Mime) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Mime) -> Ordering { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<T: Hasher>(self: &Self, hasher: &mut T) { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Mime, <Self as >::Err> { /* ... */ }
    ```

### Struct `MimeIter`

An iterator of parsed mime

```rust
pub struct MimeIter<''a> {
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
  pub fn new(s: &''a str) -> Self { /* ... */ }
  ```
  A new iterator over mimes or media types

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MimeIter<''a> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
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

### Struct `Name`

A section of a `Mime`.

For instance, for the Mime `image/svg+xml`, it contains 3 `Name`s,
`image`, `svg`, and `xml`.

In most cases, `Name`s are compared ignoring case.

```rust
pub struct Name<''a> {
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
  pub fn as_str(self: &Self) -> &''a str { /* ... */ }
  ```
  Get the value of this `Name` as a string.

##### Trait Implementations

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Name<''a> { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Name<''a>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Name<''a>) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Name<''a>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Name<''a>) -> $crate::cmp::Ordering { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

  - ```rust
    fn from(name: Name<''a>) -> &''a str { /* ... */ }
    ```

- **Sync**
- **Copy**
- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

### Struct `FromStrError`

An error when parsing a `Mime` from a string.

```rust
pub struct FromStrError {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Sync**
### Struct `Params`

An iterator over the parameters of a MIME.

```rust
pub struct Params<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<(Name<''a>, Name<''a>)> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **RefUnwindSafe**
## Constants and Statics

### Constant `STAR`

*

```rust
pub const STAR: Name<''static> = _;
```

### Constant `TEXT`

text

```rust
pub const TEXT: Name<''static> = _;
```

### Constant `IMAGE`

image

```rust
pub const IMAGE: Name<''static> = _;
```

### Constant `AUDIO`

audio

```rust
pub const AUDIO: Name<''static> = _;
```

### Constant `VIDEO`

video

```rust
pub const VIDEO: Name<''static> = _;
```

### Constant `APPLICATION`

application

```rust
pub const APPLICATION: Name<''static> = _;
```

### Constant `MULTIPART`

multipart

```rust
pub const MULTIPART: Name<''static> = _;
```

### Constant `MESSAGE`

message

```rust
pub const MESSAGE: Name<''static> = _;
```

### Constant `MODEL`

model

```rust
pub const MODEL: Name<''static> = _;
```

### Constant `FONT`

font

```rust
pub const FONT: Name<''static> = _;
```

### Constant `PLAIN`

plain

```rust
pub const PLAIN: Name<''static> = _;
```

### Constant `HTML`

html

```rust
pub const HTML: Name<''static> = _;
```

### Constant `XML`

xml

```rust
pub const XML: Name<''static> = _;
```

### Constant `JAVASCRIPT`

javascript

```rust
pub const JAVASCRIPT: Name<''static> = _;
```

### Constant `CSS`

css

```rust
pub const CSS: Name<''static> = _;
```

### Constant `CSV`

csv

```rust
pub const CSV: Name<''static> = _;
```

### Constant `EVENT_STREAM`

event-stream

```rust
pub const EVENT_STREAM: Name<''static> = _;
```

### Constant `VCARD`

vcard

```rust
pub const VCARD: Name<''static> = _;
```

### Constant `JSON`

json

```rust
pub const JSON: Name<''static> = _;
```

### Constant `WWW_FORM_URLENCODED`

x-www-form-urlencoded

```rust
pub const WWW_FORM_URLENCODED: Name<''static> = _;
```

### Constant `MSGPACK`

msgpack

```rust
pub const MSGPACK: Name<''static> = _;
```

### Constant `OCTET_STREAM`

octet-stream

```rust
pub const OCTET_STREAM: Name<''static> = _;
```

### Constant `PDF`

pdf

```rust
pub const PDF: Name<''static> = _;
```

### Constant `WOFF`

woff

```rust
pub const WOFF: Name<''static> = _;
```

### Constant `WOFF2`

woff2

```rust
pub const WOFF2: Name<''static> = _;
```

### Constant `FORM_DATA`

form-data

```rust
pub const FORM_DATA: Name<''static> = _;
```

### Constant `BMP`

bmp

```rust
pub const BMP: Name<''static> = _;
```

### Constant `GIF`

gif

```rust
pub const GIF: Name<''static> = _;
```

### Constant `JPEG`

jpeg

```rust
pub const JPEG: Name<''static> = _;
```

### Constant `PNG`

png

```rust
pub const PNG: Name<''static> = _;
```

### Constant `SVG`

svg

```rust
pub const SVG: Name<''static> = _;
```

### Constant `BASIC`

basic

```rust
pub const BASIC: Name<''static> = _;
```

### Constant `MPEG`

mpeg

```rust
pub const MPEG: Name<''static> = _;
```

### Constant `MP4`

mp4

```rust
pub const MP4: Name<''static> = _;
```

### Constant `OGG`

ogg

```rust
pub const OGG: Name<''static> = _;
```

### Constant `CHARSET`

charset

```rust
pub const CHARSET: Name<''static> = _;
```

### Constant `BOUNDARY`

boundary

```rust
pub const BOUNDARY: Name<''static> = _;
```

### Constant `UTF_8`

utf-8

```rust
pub const UTF_8: Name<''static> = _;
```

### Constant `STAR_STAR`

`
*/*
`

```rust
pub const STAR_STAR: Mime = _;
```

### Constant `TEXT_STAR`

`
text/*
`

```rust
pub const TEXT_STAR: Mime = _;
```

### Constant `TEXT_PLAIN`

`
text/plain
`

```rust
pub const TEXT_PLAIN: Mime = _;
```

### Constant `TEXT_PLAIN_UTF_8`

`
text/plain; charset=utf-8
`

```rust
pub const TEXT_PLAIN_UTF_8: Mime = _;
```

### Constant `TEXT_HTML`

`
text/html
`

```rust
pub const TEXT_HTML: Mime = _;
```

### Constant `TEXT_HTML_UTF_8`

`
text/html; charset=utf-8
`

```rust
pub const TEXT_HTML_UTF_8: Mime = _;
```

### Constant `TEXT_CSS`

`
text/css
`

```rust
pub const TEXT_CSS: Mime = _;
```

### Constant `TEXT_CSS_UTF_8`

`
text/css; charset=utf-8
`

```rust
pub const TEXT_CSS_UTF_8: Mime = _;
```

### Constant `TEXT_JAVASCRIPT`

`
text/javascript
`

```rust
pub const TEXT_JAVASCRIPT: Mime = _;
```

### Constant `TEXT_XML`

`
text/xml
`

```rust
pub const TEXT_XML: Mime = _;
```

### Constant `TEXT_EVENT_STREAM`

`
text/event-stream
`

```rust
pub const TEXT_EVENT_STREAM: Mime = _;
```

### Constant `TEXT_CSV`

`
text/csv
`

```rust
pub const TEXT_CSV: Mime = _;
```

### Constant `TEXT_CSV_UTF_8`

`
text/csv; charset=utf-8
`

```rust
pub const TEXT_CSV_UTF_8: Mime = _;
```

### Constant `TEXT_TAB_SEPARATED_VALUES`

`
text/tab-separated-values
`

```rust
pub const TEXT_TAB_SEPARATED_VALUES: Mime = _;
```

### Constant `TEXT_TAB_SEPARATED_VALUES_UTF_8`

`
text/tab-separated-values; charset=utf-8
`

```rust
pub const TEXT_TAB_SEPARATED_VALUES_UTF_8: Mime = _;
```

### Constant `TEXT_VCARD`

`
text/vcard
`

```rust
pub const TEXT_VCARD: Mime = _;
```

### Constant `IMAGE_STAR`

`
image/*
`

```rust
pub const IMAGE_STAR: Mime = _;
```

### Constant `IMAGE_JPEG`

`
image/jpeg
`

```rust
pub const IMAGE_JPEG: Mime = _;
```

### Constant `IMAGE_GIF`

`
image/gif
`

```rust
pub const IMAGE_GIF: Mime = _;
```

### Constant `IMAGE_PNG`

`
image/png
`

```rust
pub const IMAGE_PNG: Mime = _;
```

### Constant `IMAGE_BMP`

`
image/bmp
`

```rust
pub const IMAGE_BMP: Mime = _;
```

### Constant `IMAGE_SVG`

`
image/svg+xml
`

```rust
pub const IMAGE_SVG: Mime = _;
```

### Constant `FONT_WOFF`

`
font/woff
`

```rust
pub const FONT_WOFF: Mime = _;
```

### Constant `FONT_WOFF2`

`
font/woff2
`

```rust
pub const FONT_WOFF2: Mime = _;
```

### Constant `APPLICATION_JSON`

`
application/json
`

```rust
pub const APPLICATION_JSON: Mime = _;
```

### Constant `APPLICATION_JAVASCRIPT`

`
application/javascript
`

```rust
pub const APPLICATION_JAVASCRIPT: Mime = _;
```

### Constant `APPLICATION_JAVASCRIPT_UTF_8`

`
application/javascript; charset=utf-8
`

```rust
pub const APPLICATION_JAVASCRIPT_UTF_8: Mime = _;
```

### Constant `APPLICATION_WWW_FORM_URLENCODED`

`
application/x-www-form-urlencoded
`

```rust
pub const APPLICATION_WWW_FORM_URLENCODED: Mime = _;
```

### Constant `APPLICATION_OCTET_STREAM`

`
application/octet-stream
`

```rust
pub const APPLICATION_OCTET_STREAM: Mime = _;
```

### Constant `APPLICATION_MSGPACK`

`
application/msgpack
`

```rust
pub const APPLICATION_MSGPACK: Mime = _;
```

### Constant `APPLICATION_PDF`

`
application/pdf
`

```rust
pub const APPLICATION_PDF: Mime = _;
```

### Constant `MULTIPART_FORM_DATA`

`
multipart/form-data
`

```rust
pub const MULTIPART_FORM_DATA: Mime = _;
```

