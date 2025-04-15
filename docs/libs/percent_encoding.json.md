# Crate Documentation

**Version:** 2.3.1

**Format Version:** 43

# Module `percent_encoding`

URLs use special characters to indicate the parts of the request.
For example, a `?` question mark marks the end of a path and the start of a query string.
In order for that character to exist inside a path, it needs to be encoded differently.

Percent encoding replaces reserved characters with the `%` escape character
followed by a byte value as two hexadecimal digits.
For example, an ASCII space is replaced with `%20`.

When encoding, the set of characters that can (and should, for readability) be left alone
depends on the context.
The `?` question mark mentioned above is not a separator when used literally
inside of a query string, and therefore does not need to be encoded.
The [`AsciiSet`] parameter of [`percent_encode`] and [`utf8_percent_encode`]
lets callers configure this.

This crate deliberately does not provide many different sets.
Users should consider in what context the encoded string will be used,
read relevant specifications, and define their own set.
This is done by using the `add` method of an existing set.

# Examples

```
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

assert_eq!(utf8_percent_encode("foo <bar>", FRAGMENT).to_string(), "foo%20%3Cbar%3E");
```

## Types

### Struct `AsciiSet`

Represents a set of characters or bytes in the ASCII range.

This is used in [`percent_encode`] and [`utf8_percent_encode`].
This is similar to [percent-encode sets](https://url.spec.whatwg.org/#percent-encoded-bytes).

Use the `add` method of an existing set to define a new set. For example:

```
use percent_encoding::{AsciiSet, CONTROLS};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
```

```rust
pub struct AsciiSet {
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
  pub const fn add(self: &Self, byte: u8) -> Self { /* ... */ }
  ```

- ```rust
  pub const fn remove(self: &Self, byte: u8) -> Self { /* ... */ }
  ```

##### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
### Struct `PercentEncode`

The return type of [`percent_encode`] and [`utf8_percent_encode`].

```rust
pub struct PercentEncode<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
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

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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
    fn clone(self: &Self) -> PercentEncode<''a> { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a str> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(iter: PercentEncode<''a>) -> Self { /* ... */ }
    ```

### Struct `PercentDecode`

The return type of [`percent_decode`].

```rust
pub struct PercentDecode<''a> {
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
  pub fn decode_utf8(self: Self) -> Result<Cow<''a, str>, str::Utf8Error> { /* ... */ }
  ```
  Decode the result of percent-decoding as UTF-8.

- ```rust
  pub fn decode_utf8_lossy(self: Self) -> Cow<''a, str> { /* ... */ }
  ```
  Decode the result of percent-decoding as UTF-8, lossily.

##### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PercentDecode<''a> { /* ... */ }
    ```

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
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

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(iter: PercentDecode<''a>) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<u8> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

## Functions

### Function `percent_encode_byte`

**Attributes:**

- `#[inline]`

Return the percent-encoding of the given byte.

This is unconditional, unlike `percent_encode()` which has an `AsciiSet` parameter.

# Examples

```
use percent_encoding::percent_encode_byte;

assert_eq!("foo bar".bytes().map(percent_encode_byte).collect::<String>(),
           "%66%6F%6F%20%62%61%72");
```

```rust
pub fn percent_encode_byte(byte: u8) -> &''static str { /* ... */ }
```

### Function `percent_encode`

**Attributes:**

- `#[inline]`

Percent-encode the given bytes with the given set.

Non-ASCII bytes and bytes in `ascii_set` are encoded.

The return type:

* Implements `Iterator<Item = &str>` and therefore has a `.collect::<String>()` method,
* Implements `Display` and therefore has a `.to_string()` method,
* Implements `Into<Cow<str>>` borrowing `input` when none of its bytes are encoded.

# Examples

```
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

assert_eq!(percent_encode(b"foo bar?", NON_ALPHANUMERIC).to_string(), "foo%20bar%3F");
```

```rust
pub fn percent_encode<''a>(input: &''a [u8], ascii_set: &''static AsciiSet) -> PercentEncode<''a> { /* ... */ }
```

### Function `utf8_percent_encode`

**Attributes:**

- `#[inline]`

Percent-encode the UTF-8 encoding of the given string.

See [`percent_encode`] regarding the return type.

# Examples

```
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

assert_eq!(utf8_percent_encode("foo bar?", NON_ALPHANUMERIC).to_string(), "foo%20bar%3F");
```

```rust
pub fn utf8_percent_encode<''a>(input: &''a str, ascii_set: &''static AsciiSet) -> PercentEncode<''a> { /* ... */ }
```

### Function `percent_decode_str`

**Attributes:**

- `#[inline]`

Percent-decode the given string.

<https://url.spec.whatwg.org/#string-percent-decode>

See [`percent_decode`] regarding the return type.

```rust
pub fn percent_decode_str(input: &str) -> PercentDecode<''_> { /* ... */ }
```

### Function `percent_decode`

**Attributes:**

- `#[inline]`

Percent-decode the given bytes.

<https://url.spec.whatwg.org/#percent-decode>

Any sequence of `%` followed by two hexadecimal digits is decoded.
The return type:

* Implements `Into<Cow<u8>>` borrowing `input` when it contains no percent-encoded sequence,
* Implements `Iterator<Item = u8>` and therefore has a `.collect::<Vec<u8>>()` method,
* Has `decode_utf8()` and `decode_utf8_lossy()` methods.

# Examples

```
use percent_encoding::percent_decode;

assert_eq!(percent_decode(b"foo%20bar%3f").decode_utf8().unwrap(), "foo bar?");
```

```rust
pub fn percent_decode(input: &[u8]) -> PercentDecode<''_> { /* ... */ }
```

## Constants and Statics

### Constant `CONTROLS`

The set of 0x00 to 0x1F (C0 controls), and 0x7F (DEL).

Note that this includes the newline and tab characters, but not the space 0x20.

<https://url.spec.whatwg.org/#c0-control-percent-encode-set>

```rust
pub const CONTROLS: &AsciiSet = _;
```

### Constant `NON_ALPHANUMERIC`

Everything that is not an ASCII letter or digit.

This is probably more eager than necessary in any context.

```rust
pub const NON_ALPHANUMERIC: &AsciiSet = _;
```

