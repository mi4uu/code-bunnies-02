# Crate Documentation

**Version:** 0.22.1

**Format Version:** 43

# Module `base64`

Correct, fast, and configurable [base64][] decoding and encoding. Base64
transports binary data efficiently in contexts where only plain text is
allowed.

[base64]: https://developer.mozilla.org/en-US/docs/Glossary/Base64

# Usage

Use an [`Engine`] to decode or encode base64, configured with the base64
alphabet and padding behavior best suited to your application.

## Engine setup

There is more than one way to encode a stream of bytes as “base64”.
Different applications use different encoding
[alphabets][alphabet::Alphabet] and
[padding behaviors][engine::general_purpose::GeneralPurposeConfig].

### Encoding alphabet

Almost all base64 [alphabets][alphabet::Alphabet] use `A-Z`, `a-z`, and
`0-9`, which gives nearly 64 characters (26 + 26 + 10 = 62), but they differ
in their choice of their final 2.

Most applications use the [standard][alphabet::STANDARD] alphabet specified
in [RFC 4648][rfc-alphabet].  If that’s all you need, you can get started
quickly by using the pre-configured
[`STANDARD`][engine::general_purpose::STANDARD] engine, which is also available
in the [`prelude`] module as shown here, if you prefer a minimal `use`
footprint.

```
use base64::prelude::*;

# fn main() -> Result<(), base64::DecodeError> {
assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");
# Ok(())
# }
```

[rfc-alphabet]: https://datatracker.ietf.org/doc/html/rfc4648#section-4

Other common alphabets are available in the [`alphabet`] module.

#### URL-safe alphabet

The standard alphabet uses `+` and `/` as its two non-alphanumeric tokens,
which cannot be safely used in URL’s without encoding them as `%2B` and
`%2F`.

To avoid that, some applications use a [“URL-safe” alphabet][alphabet::URL_SAFE],
which uses `-` and `_` instead. To use that alternative alphabet, use the
[`URL_SAFE`][engine::general_purpose::URL_SAFE] engine. This example doesn't
use [`prelude`] to show what a more explicit `use` would look like.

```
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

# fn main() -> Result<(), base64::DecodeError> {
assert_eq!(URL_SAFE.decode(b"-uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
assert_eq!(URL_SAFE.encode(b"\xFF\xEC\x20\x55\0"), "_-wgVQA=");
# Ok(())
# }
```

### Padding characters

Each base64 character represents 6 bits (2⁶ = 64) of the original binary
data, and every 3 bytes of input binary data will encode to 4 base64
characters (8 bits × 3 = 6 bits × 4 = 24 bits).

When the input is not an even multiple of 3 bytes in length, [canonical][]
base64 encoders insert padding characters at the end, so that the output
length is always a multiple of 4:

[canonical]: https://datatracker.ietf.org/doc/html/rfc4648#section-3.5

```
use base64::{engine::general_purpose::STANDARD, Engine as _};

assert_eq!(STANDARD.encode(b""),    "");
assert_eq!(STANDARD.encode(b"f"),   "Zg==");
assert_eq!(STANDARD.encode(b"fo"),  "Zm8=");
assert_eq!(STANDARD.encode(b"foo"), "Zm9v");
```

Canonical encoding ensures that base64 encodings will be exactly the same,
byte-for-byte, regardless of input length. But the `=` padding characters
aren’t necessary for decoding, and they may be omitted by using a
[`NO_PAD`][engine::general_purpose::NO_PAD] configuration:

```
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

assert_eq!(STANDARD_NO_PAD.encode(b""),    "");
assert_eq!(STANDARD_NO_PAD.encode(b"f"),   "Zg");
assert_eq!(STANDARD_NO_PAD.encode(b"fo"),  "Zm8");
assert_eq!(STANDARD_NO_PAD.encode(b"foo"), "Zm9v");
```

The pre-configured `NO_PAD` engines will reject inputs containing padding
`=` characters. To encode without padding and still accept padding while
decoding, create an [engine][engine::general_purpose::GeneralPurpose] with
that [padding mode][engine::DecodePaddingMode].

```
# use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
assert_eq!(STANDARD_NO_PAD.decode(b"Zm8="), Err(base64::DecodeError::InvalidPadding));
```

### Further customization

Decoding and encoding behavior can be customized by creating an
[engine][engine::GeneralPurpose] with an [alphabet][alphabet::Alphabet] and
[padding configuration][engine::GeneralPurposeConfig]:

```
use base64::{engine, alphabet, Engine as _};

// bizarro-world base64: +/ as the first symbols instead of the last
let alphabet =
    alphabet::Alphabet::new("+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
    .unwrap();

// a very weird config that encodes with padding but requires no padding when decoding...?
let crazy_config = engine::GeneralPurposeConfig::new()
    .with_decode_allow_trailing_bits(true)
    .with_encode_padding(true)
    .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

let encoded = crazy_engine.encode(b"abc 123");

```

## Memory allocation

The [decode][Engine::decode()] and [encode][Engine::encode()] engine methods
allocate memory for their results – `decode` returns a `Vec<u8>` and
`encode` returns a `String`. To instead decode or encode into a buffer that
you allocated, use one of the alternative methods:

#### Decoding

| Method                     | Output                        | Allocates memory              |
| -------------------------- | ----------------------------- | ----------------------------- |
| [`Engine::decode`]         | returns a new `Vec<u8>`       | always                        |
| [`Engine::decode_vec`]     | appends to provided `Vec<u8>` | if `Vec` lacks capacity       |
| [`Engine::decode_slice`]   | writes to provided `&[u8]`    | never

#### Encoding

| Method                     | Output                       | Allocates memory               |
| -------------------------- | ---------------------------- | ------------------------------ |
| [`Engine::encode`]         | returns a new `String`       | always                         |
| [`Engine::encode_string`]  | appends to provided `String` | if `String` lacks capacity     |
| [`Engine::encode_slice`]   | writes to provided `&[u8]`   | never                          |

## Input and output

The `base64` crate can [decode][Engine::decode()] and
[encode][Engine::encode()] values in memory, or
[`DecoderReader`][read::DecoderReader] and
[`EncoderWriter`][write::EncoderWriter] provide streaming decoding and
encoding for any [readable][std::io::Read] or [writable][std::io::Write]
byte stream.

#### Decoding

```
# use std::io;
use base64::{engine::general_purpose::STANDARD, read::DecoderReader};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut input = io::stdin();
let mut decoder = DecoderReader::new(&mut input, &STANDARD);
io::copy(&mut decoder, &mut io::stdout())?;
# Ok(())
# }
```

#### Encoding

```
# use std::io;
use base64::{engine::general_purpose::STANDARD, write::EncoderWriter};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut output = io::stdout();
let mut encoder = EncoderWriter::new(&mut output, &STANDARD);
io::copy(&mut io::stdin(), &mut encoder)?;
# Ok(())
# }
```

#### Display

If you only need a base64 representation for implementing the
[`Display`][std::fmt::Display] trait, use
[`Base64Display`][display::Base64Display]:

```
use base64::{display::Base64Display, engine::general_purpose::STANDARD};

let value = Base64Display::new(b"\0\x01\x02\x03", &STANDARD);
assert_eq!("base64: AAECAw==", format!("base64: {}", value));
```

# Panics

If length calculations result in overflowing `usize`, a panic will result.

## Modules

## Module `display`

Enables base64'd output anywhere you might use a `Display` implementation, like a format string.

```
use base64::{display::Base64Display, engine::general_purpose::STANDARD};

let data = vec![0x0, 0x1, 0x2, 0x3];
let wrapper = Base64Display::new(&data, &STANDARD);

assert_eq!("base64: AAECAw==", format!("base64: {}", wrapper));
```

```rust
pub mod display { /* ... */ }
```

### Types

#### Struct `Base64Display`

A convenience wrapper for base64'ing bytes into a format string without heap allocation.

```rust
pub struct Base64Display<''a, ''e, E: Engine> {
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
  pub fn new(bytes: &''a [u8], engine: &''e E) -> Base64Display<''a, ''e, E> { /* ... */ }
  ```
  Create a `Base64Display` with the provided engine.

###### Trait Implementations

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
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

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
## Module `read`

**Attributes:**

- `#[<cfg>(any(feature = "std", test))]`

Implementations of `io::Read` to transparently decode base64.

```rust
pub mod read { /* ... */ }
```

### Re-exports

#### Re-export `DecoderReader`

```rust
pub use self::decoder::DecoderReader;
```

## Module `write`

**Attributes:**

- `#[<cfg>(any(feature = "std", test))]`

Implementations of `io::Write` to transparently handle base64.

```rust
pub mod write { /* ... */ }
```

### Re-exports

#### Re-export `EncoderWriter`

```rust
pub use self::encoder::EncoderWriter;
```

#### Re-export `EncoderStringWriter`

```rust
pub use self::encoder_string_writer::EncoderStringWriter;
```

#### Re-export `StrConsumer`

```rust
pub use self::encoder_string_writer::StrConsumer;
```

## Module `engine`

Provides the [Engine] abstraction and out of the box implementations.

```rust
pub mod engine { /* ... */ }
```

### Modules

## Module `general_purpose`

Provides the [GeneralPurpose] engine and associated config types.

```rust
pub mod general_purpose { /* ... */ }
```

### Types

#### Struct `GeneralPurpose`

A general-purpose base64 engine.

- It uses no vector CPU instructions, so it will work on any system.
- It is reasonably fast (~2-3GiB/s).
- It is not constant-time, though, so it is vulnerable to timing side-channel attacks. For loading cryptographic keys, etc, it is suggested to use the forthcoming constant-time implementation.

```rust
pub struct GeneralPurpose {
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
  pub const fn new(alphabet: &Alphabet, config: GeneralPurposeConfig) -> Self { /* ... */ }
  ```
  Create a `GeneralPurpose` engine from an [Alphabet].

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GeneralPurpose { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Freeze**
- **Unpin**
- **UnwindSafe**
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

- **Engine**
  - ```rust
    fn config(self: &Self) -> &<Self as >::Config { /* ... */ }
    ```

- **Sync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `GeneralPurposeConfig`

Contains configuration parameters for base64 encoding and decoding.

```
# use base64::engine::GeneralPurposeConfig;
let config = GeneralPurposeConfig::new()
    .with_encode_padding(false);
    // further customize using `.with_*` methods as needed
```

The constants [PAD] and [NO_PAD] cover most use cases.

To specify the characters used, see [Alphabet].

```rust
pub struct GeneralPurposeConfig {
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
  pub const fn new() -> Self { /* ... */ }
  ```
  Create a new config with `padding` = `true`, `decode_allow_trailing_bits` = `false`, and

- ```rust
  pub const fn with_encode_padding(self: Self, padding: bool) -> Self { /* ... */ }
  ```
  Create a new config based on `self` with an updated `padding` setting.

- ```rust
  pub const fn with_decode_allow_trailing_bits(self: Self, allow: bool) -> Self { /* ... */ }
  ```
  Create a new config based on `self` with an updated `decode_allow_trailing_bits` setting.

- ```rust
  pub const fn with_decode_padding_mode(self: Self, mode: DecodePaddingMode) -> Self { /* ... */ }
  ```
  Create a new config based on `self` with an updated `decode_padding_mode` setting.

###### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
    Delegates to [GeneralPurposeConfig::new].

- **Config**
  - ```rust
    fn encode_padding(self: &Self) -> bool { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> GeneralPurposeConfig { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Constants and Statics

#### Constant `STANDARD`

A [GeneralPurpose] engine using the [alphabet::STANDARD] base64 alphabet and [PAD] config.

```rust
pub const STANDARD: GeneralPurpose = _;
```

#### Constant `STANDARD_NO_PAD`

A [GeneralPurpose] engine using the [alphabet::STANDARD] base64 alphabet and [NO_PAD] config.

```rust
pub const STANDARD_NO_PAD: GeneralPurpose = _;
```

#### Constant `URL_SAFE`

A [GeneralPurpose] engine using the [alphabet::URL_SAFE] base64 alphabet and [PAD] config.

```rust
pub const URL_SAFE: GeneralPurpose = _;
```

#### Constant `URL_SAFE_NO_PAD`

A [GeneralPurpose] engine using the [alphabet::URL_SAFE] base64 alphabet and [NO_PAD] config.

```rust
pub const URL_SAFE_NO_PAD: GeneralPurpose = _;
```

#### Constant `PAD`

Include padding bytes when encoding, and require that they be present when decoding.

This is the standard per the base64 RFC, but consider using [NO_PAD] instead as padding serves
little purpose in practice.

```rust
pub const PAD: GeneralPurposeConfig = _;
```

#### Constant `NO_PAD`

Don't add padding when encoding, and require no padding when decoding.

```rust
pub const NO_PAD: GeneralPurposeConfig = _;
```

### Types

#### Enum `DecodePaddingMode`

Controls how pad bytes are handled when decoding.

Each [Engine] must support at least the behavior indicated by
[DecodePaddingMode::RequireCanonical], and may support other modes.

```rust
pub enum DecodePaddingMode {
    Indifferent,
    RequireCanonical,
    RequireNone,
}
```

##### Variants

###### `Indifferent`

Canonical padding is allowed, but any fewer padding bytes than that is also allowed.

###### `RequireCanonical`

Padding must be canonical (0, 1, or 2 `=` as needed to produce a 4 byte suffix).

###### `RequireNone`

Padding must be absent -- for when you want predictable padding, without any wasted bytes.

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecodePaddingMode) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DecodePaddingMode { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
#### Struct `DecodeMetadata`

Metadata about the result of a decode operation

```rust
pub struct DecodeMetadata {
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecodeMetadata) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `Engine`

An `Engine` provides low-level encoding and decoding operations that all other higher-level parts of the API use. Users of the library will generally not need to implement this.

Different implementations offer different characteristics. The library currently ships with
[GeneralPurpose] that offers good speed and works on any CPU, with more choices
coming later, like a constant-time one when side channel resistance is called for, and vendor-specific vectorized ones for more speed.

See [general_purpose::STANDARD_NO_PAD] if you just want standard base64. Otherwise, when possible, it's
recommended to store the engine in a `const` so that references to it won't pose any lifetime
issues, and to avoid repeating the cost of engine setup.

Since almost nobody will need to implement `Engine`, docs for internal methods are hidden.

```rust
pub trait Engine: Send + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Config`: The config type used by this engine
- `DecodeEstimate`: The decode estimate used by this engine

###### Required Methods

- `config`: Returns the config for this engine.

##### Provided Methods

- ```rust
  fn encode<T: AsRef<[u8]>>(self: &Self, input: T) -> String { /* ... */ }
  ```
  Encode arbitrary octets as base64 using the provided `Engine`.

- ```rust
  fn encode_string<T: AsRef<[u8]>>(self: &Self, input: T, output_buf: &mut String) { /* ... */ }
  ```
  Encode arbitrary octets as base64 into a supplied `String`.

- ```rust
  fn encode_slice<T: AsRef<[u8]>>(self: &Self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> { /* ... */ }
  ```
  Encode arbitrary octets as base64 into a supplied slice.

- ```rust
  fn decode<T: AsRef<[u8]>>(self: &Self, input: T) -> Result<Vec<u8>, DecodeError> { /* ... */ }
  ```
  Decode the input into a new `Vec`.

- ```rust
  fn decode_vec<T: AsRef<[u8]>>(self: &Self, input: T, buffer: &mut Vec<u8>) -> Result<(), DecodeError> { /* ... */ }
  ```
  Decode the `input` into the supplied `buffer`.

- ```rust
  fn decode_slice<T: AsRef<[u8]>>(self: &Self, input: T, output: &mut [u8]) -> Result<usize, DecodeSliceError> { /* ... */ }
  ```
  Decode the input into the provided output slice.

- ```rust
  fn decode_slice_unchecked<T: AsRef<[u8]>>(self: &Self, input: T, output: &mut [u8]) -> Result<usize, DecodeError> { /* ... */ }
  ```
  Decode the input into the provided output slice.

##### Implementations

This trait is implemented for the following types:

- `GeneralPurpose`

#### Trait `Config`

The minimal level of configuration that engines must support.

```rust
pub trait Config {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `encode_padding`: Returns `true` if padding should be added after the encoded output.

##### Implementations

This trait is implemented for the following types:

- `GeneralPurposeConfig`

#### Trait `DecodeEstimate`

The decode estimate used by an engine implementation. Users do not need to interact with this;
it is only for engine implementors.

Implementors may store relevant data here when constructing this to avoid having to calculate
them again during actual decoding.

```rust
pub trait DecodeEstimate {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `decoded_len_estimate`: Returns a conservative (err on the side of too big) estimate of the decoded length to use

### Re-exports

#### Re-export `GeneralPurpose`

```rust
pub use general_purpose::GeneralPurpose;
```

#### Re-export `GeneralPurposeConfig`

```rust
pub use general_purpose::GeneralPurposeConfig;
```

## Module `alphabet`

Provides [Alphabet] and constants for alphabets commonly used in the wild.

```rust
pub mod alphabet { /* ... */ }
```

### Types

#### Struct `Alphabet`

An alphabet defines the 64 ASCII characters (symbols) used for base64.

Common alphabets are provided as constants, and custom alphabets
can be made via `from_str` or the `TryFrom<str>` implementation.

# Examples

Building and using a custom Alphabet:

```
let custom = base64::alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").unwrap();

let engine = base64::engine::GeneralPurpose::new(
    &custom,
    base64::engine::general_purpose::PAD);
```

Building a const:

```
use base64::alphabet::Alphabet;

static CUSTOM: Alphabet = {
    // Result::unwrap() isn't const yet, but panic!() is OK
    match Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/") {
        Ok(x) => x,
        Err(_) => panic!("creation of alphabet failed"),
    }
};
```

Building lazily:

```
use base64::{
    alphabet::Alphabet,
    engine::{general_purpose::GeneralPurpose, GeneralPurposeConfig},
};
use once_cell::sync::Lazy;

static CUSTOM: Lazy<Alphabet> = Lazy::new(||
    Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").unwrap()
);
```

```rust
pub struct Alphabet {
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
  pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> { /* ... */ }
  ```
  Create an `Alphabet` from a string of 64 unique printable ASCII bytes.

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Create a `&str` from the symbols in the `Alphabet`

###### Trait Implementations

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

- **Sync**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Alphabet) -> bool { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Alphabet { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
#### Enum `ParseAlphabetError`

Possible errors when constructing an [Alphabet] from a `str`.

```rust
pub enum ParseAlphabetError {
    InvalidLength,
    DuplicatedByte(u8),
    UnprintableByte(u8),
    ReservedByte(u8),
}
```

##### Variants

###### `InvalidLength`

Alphabets must be 64 ASCII bytes

###### `DuplicatedByte`

All bytes must be unique

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `UnprintableByte`

All bytes must be printable (in the range `[32, 126]`).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `ReservedByte`

`=` cannot be used

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParseAlphabetError) -> bool { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Error**
### Constants and Statics

#### Constant `STANDARD`

The standard alphabet (with `+` and `/`) specified in [RFC 4648][].

[RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-4

```rust
pub const STANDARD: Alphabet = _;
```

#### Constant `URL_SAFE`

The URL-safe alphabet (with `-` and `_`) specified in [RFC 4648][].

[RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-5

```rust
pub const URL_SAFE: Alphabet = _;
```

#### Constant `CRYPT`

The `crypt(3)` alphabet (with `.` and `/` as the _first_ two characters).

Not standardized, but folk wisdom on the net asserts that this alphabet is what crypt uses.

```rust
pub const CRYPT: Alphabet = _;
```

#### Constant `BCRYPT`

The bcrypt alphabet.

```rust
pub const BCRYPT: Alphabet = _;
```

#### Constant `IMAP_MUTF7`

The alphabet used in IMAP-modified UTF-7 (with `+` and `,`).

See [RFC 3501](https://tools.ietf.org/html/rfc3501#section-5.1.3)

```rust
pub const IMAP_MUTF7: Alphabet = _;
```

#### Constant `BIN_HEX`

The alphabet used in BinHex 4.0 files.

See [BinHex 4.0 Definition](http://files.stairways.com/other/binhex-40-specs-info.txt)

```rust
pub const BIN_HEX: Alphabet = _;
```

## Module `prelude`

**Attributes:**

- `#[<cfg_attr>(feature = "alloc", doc = "```")]`
- `#[<cfg_attr>(not(feature = "alloc"), doc = "```ignore")]`

Preconfigured engines for common use cases.

These are re-exports of `const` engines in [crate::engine::general_purpose], renamed with a `BASE64_`
prefix for those who prefer to `use` the entire path to a name.

# Examples

```
use base64::prelude::{Engine as _, BASE64_STANDARD_NO_PAD};

assert_eq!("c29tZSBieXRlcw", &BASE64_STANDARD_NO_PAD.encode(b"some bytes"));
```

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `Engine`

```rust
pub use crate::engine::Engine;
```

#### Re-export `STANDARD`

```rust
pub use crate::engine::general_purpose::STANDARD as BASE64_STANDARD;
```

#### Re-export `STANDARD_NO_PAD`

```rust
pub use crate::engine::general_purpose::STANDARD_NO_PAD as BASE64_STANDARD_NO_PAD;
```

#### Re-export `URL_SAFE`

```rust
pub use crate::engine::general_purpose::URL_SAFE as BASE64_URL_SAFE;
```

#### Re-export `URL_SAFE_NO_PAD`

```rust
pub use crate::engine::general_purpose::URL_SAFE_NO_PAD as BASE64_URL_SAFE_NO_PAD;
```

## Re-exports

### Re-export `Engine`

```rust
pub use engine::Engine;
```

### Re-export `encode`

**Attributes:**

- `#[allow(deprecated)]`
- `#[<cfg>(any(feature = "alloc", test))]`

```rust
pub use crate::encode::encode;
```

### Re-export `encode_engine`

**Attributes:**

- `#[allow(deprecated)]`
- `#[<cfg>(any(feature = "alloc", test))]`

```rust
pub use crate::encode::encode_engine;
```

### Re-export `encode_engine_string`

**Attributes:**

- `#[allow(deprecated)]`
- `#[<cfg>(any(feature = "alloc", test))]`

```rust
pub use crate::encode::encode_engine_string;
```

### Re-export `encode_engine_slice`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::encode::encode_engine_slice;
```

### Re-export `encoded_len`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::encode::encoded_len;
```

### Re-export `EncodeSliceError`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::encode::EncodeSliceError;
```

### Re-export `decode`

**Attributes:**

- `#[allow(deprecated)]`
- `#[<cfg>(any(feature = "alloc", test))]`

```rust
pub use crate::decode::decode;
```

### Re-export `decode_engine`

**Attributes:**

- `#[allow(deprecated)]`
- `#[<cfg>(any(feature = "alloc", test))]`

```rust
pub use crate::decode::decode_engine;
```

### Re-export `decode_engine_vec`

**Attributes:**

- `#[allow(deprecated)]`
- `#[<cfg>(any(feature = "alloc", test))]`

```rust
pub use crate::decode::decode_engine_vec;
```

### Re-export `decode_engine_slice`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::decode::decode_engine_slice;
```

### Re-export `decoded_len_estimate`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::decode::decoded_len_estimate;
```

### Re-export `DecodeError`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::decode::DecodeError;
```

### Re-export `DecodeSliceError`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::decode::DecodeSliceError;
```

