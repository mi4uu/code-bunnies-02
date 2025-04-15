# Crate Documentation

**Version:** 2.9.0

**Format Version:** 43

# Module `data_encoding`

Efficient and customizable data-encoding functions like base64, base32, and hex

This [crate] provides little-endian ASCII base-conversion encodings for
bases of size 2, 4, 8, 16, 32, and 64. It supports:

- [padding] for streaming
- canonical encodings (e.g. [trailing bits] are checked)
- in-place [encoding] and [decoding] functions
- partial [decoding] functions (e.g. for error recovery)
- character [translation] (e.g. for case-insensitivity)
- most and least significant [bit-order]
- [ignoring] characters when decoding (e.g. for skipping newlines)
- [wrapping] the output when encoding
- no-std environments with `default-features = false, features = ["alloc"]`
- no-alloc environments with `default-features = false`

You may use the [binary] or the [website] to play around.

# Examples

This crate provides predefined encodings as [constants]. These constants are of type
[`Encoding`]. This type provides encoding and decoding functions with in-place or allocating
variants. Here is an example using the allocating encoding function of [`BASE64`]:

```rust
use data_encoding::BASE64;
assert_eq!(BASE64.encode(b"Hello world"), "SGVsbG8gd29ybGQ=");
```

Here is an example using the in-place decoding function of [`BASE32`]:

```rust
use data_encoding::BASE32;
let input = b"JBSWY3DPEB3W64TMMQ======";
let mut output = vec![0; BASE32.decode_len(input.len()).unwrap()];
let len = BASE32.decode_mut(input, &mut output).unwrap();
assert_eq!(&output[0 .. len], b"Hello world");
```

You are not limited to the predefined encodings. You may define your own encodings (with the
same correctness and performance properties as the predefined ones) using the [`Specification`]
type:

```rust
use data_encoding::Specification;
let hex = {
    let mut spec = Specification::new();
    spec.symbols.push_str("0123456789abcdef");
    spec.encoding().unwrap()
};
assert_eq!(hex.encode(b"hello"), "68656c6c6f");
```

You may use the [macro] library to define a compile-time custom encoding:

```rust,ignore
use data_encoding::Encoding;
use data_encoding_macro::new_encoding;
const HEX: Encoding = new_encoding!{
    symbols: "0123456789abcdef",
    translate_from: "ABCDEF",
    translate_to: "abcdef",
};
const BASE64: Encoding = new_encoding!{
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    padding: '=',
};
```

# Properties

The [`HEXUPPER`], [`BASE32`], [`BASE32HEX`], [`BASE64`], and [`BASE64URL`] predefined encodings
conform to [RFC4648].

In general, the encoding and decoding functions satisfy the following properties:

- They are deterministic: their output only depends on their input
- They have no side-effects: they do not modify any hidden mutable state
- They are correct: encoding followed by decoding gives the initial data
- They are canonical (unless [`is_canonical`] returns false): decoding followed by encoding
  gives the initial data

This last property is usually not satisfied by base64 implementations. This is a matter of
choice and this crate has made the choice to let the user choose. Support for canonical encoding
as described by the [RFC][canonical] is provided. But it is also possible to disable checking
trailing bits, to add characters translation, to decode concatenated padded inputs, and to
ignore some characters. Note that non-canonical encodings may be an attack vector as described
in [Base64 Malleability in Practice](https://eprint.iacr.org/2022/361.pdf).

Since the RFC specifies the encoding function on all inputs and the decoding function on all
possible encoded outputs, the differences between implementations come from the decoding
function which may be more or less permissive. In this crate, the decoding function of canonical
encodings rejects all inputs that are not a possible output of the encoding function. Here are
some concrete examples of decoding differences between this crate, the `base64` crate, and the
`base64` GNU program:

| Input      | `data-encoding` | `base64`  | GNU `base64`  |
| ---------- | --------------- | --------- | ------------- |
| `AAB=`     | `Trailing(2)`   | `Last(2)` | `\x00\x00`    |
| `AA\nB=`   | `Length(4)`     | `Byte(2)` | `\x00\x00`    |
| `AAB`      | `Length(0)`     | `Padding` | Invalid input |
| `AAA`      | `Length(0)`     | `Padding` | Invalid input |
| `A\rA\nB=` | `Length(4)`     | `Byte(1)` | Invalid input |
| `-_\r\n`   | `Symbol(0)`     | `Byte(0)` | Invalid input |
| `AA==AA==` | `[0, 0]`        | `Byte(2)` | `\x00\x00`    |

We can summarize these discrepancies as follows:

| Discrepancy                | `data-encoding` | `base64` | GNU `base64` |
| -------------------------- | --------------- | -------- | ------------ |
| Check trailing bits        | Yes             | Yes      | No           |
| Ignored characters         | None            | None     | `\n`         |
| Translated characters      | None            | None     | None         |
| Check padding              | Yes             | No       | Yes          |
| Support concatenated input | Yes             | No       | Yes          |

This crate permits to disable checking trailing bits. It permits to ignore some characters. It
permits to translate characters. It permits to use unpadded encodings. However, for padded
encodings, support for concatenated inputs cannot be disabled. This is simply because it doesn't
make sense to use padding if it is not to support concatenated inputs.

[RFC4648]: https://tools.ietf.org/html/rfc4648
[`BASE32HEX`]: constant.BASE32HEX.html
[`BASE32`]: constant.BASE32.html
[`BASE64URL`]: constant.BASE64URL.html
[`BASE64`]: constant.BASE64.html
[`Encoding`]: struct.Encoding.html
[`HEXUPPER`]: constant.HEXUPPER.html
[`Specification`]: struct.Specification.html
[`is_canonical`]: struct.Encoding.html#method.is_canonical
[binary]: https://crates.io/crates/data-encoding-bin
[bit-order]: struct.Specification.html#structfield.bit_order
[canonical]: https://tools.ietf.org/html/rfc4648#section-3.5
[constants]: index.html#constants
[crate]: https://crates.io/crates/data-encoding
[decoding]: struct.Encoding.html#method.decode_mut
[encoding]: struct.Encoding.html#method.encode_mut
[ignoring]: struct.Specification.html#structfield.ignore
[macro]: https://crates.io/crates/data-encoding-macro
[padding]: struct.Specification.html#structfield.padding
[trailing bits]: struct.Specification.html#structfield.check_trailing_bits
[translation]: struct.Specification.html#structfield.translate
[website]: https://data-encoding.rs
[wrapping]: struct.Specification.html#structfield.wrap

## Types

### Enum `DecodeKind`

Decoding error kind

```rust
pub enum DecodeKind {
    Length,
    Symbol,
    Trailing,
    Padding,
}
```

#### Variants

##### `Length`

Invalid length

##### `Symbol`

Invalid symbol

##### `Trailing`

Non-zero trailing bits

##### `Padding`

Invalid padding length

#### Implementations

##### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecodeKind) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DecodeKind { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **Freeze**
### Struct `DecodeError`

Decoding error

```rust
pub struct DecodeError {
    pub position: usize,
    pub kind: DecodeKind,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `position` | `usize` | Error position<br><br>This position is always a valid input position and represents the first encountered error. |
| `kind` | `DecodeKind` | Error kind |

#### Implementations

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecodeError) -> bool { /* ... */ }
    ```

- **Error**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Eq**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DecodeError { /* ... */ }
    ```

### Struct `DecodePartial`

Decoding error with partial result

```rust
pub struct DecodePartial {
    pub read: usize,
    pub written: usize,
    pub error: DecodeError,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `read` | `usize` | Number of bytes read from input<br><br>This number does not exceed the error position: `read <= error.position`. |
| `written` | `usize` | Number of bytes written to output<br><br>This number does not exceed the decoded length: `written <= decode_len(read)`. |
| `error` | `DecodeError` | Decoding error |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DecodePartial { /* ... */ }
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

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecodePartial) -> bool { /* ... */ }
    ```

### Enum `BitOrder`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Order in which bits are read from a byte

The base-conversion encoding is always little-endian. This means that the least significant
**byte** is always first. However, we can still choose whether, within a byte, this is the most
significant or the least significant **bit** that is first. If the terminology is confusing,
testing on an asymmetrical example should be enough to choose the correct value.

# Examples

In the following example, we can see that a base with the `MostSignificantFirst` bit-order has
the most significant bit first in the encoded output. In particular, the output is in the same
order as the bits in the byte. The opposite happens with the `LeastSignificantFirst` bit-order.
The least significant bit is first and the output is in the reverse order.

```rust
use data_encoding::{BitOrder, Specification};
let mut spec = Specification::new();
spec.symbols.push_str("01");
spec.bit_order = BitOrder::MostSignificantFirst;  // default
let msb = spec.encoding().unwrap();
spec.bit_order = BitOrder::LeastSignificantFirst;
let lsb = spec.encoding().unwrap();
assert_eq!(msb.encode(&[0b01010011]), "01010011");
assert_eq!(lsb.encode(&[0b01010011]), "11001010");
```

```rust
pub enum BitOrder {
    MostSignificantFirst,
    LeastSignificantFirst,
}
```

#### Variants

##### `MostSignificantFirst`

Most significant bit first

This is the most common and most intuitive bit-order. In particular, this is the bit-order
used by [RFC4648] and thus the usual hexadecimal, base64, base32, base64url, and base32hex
encodings. This is the default bit-order when [specifying](struct.Specification.html) a
base.

[RFC4648]: https://tools.ietf.org/html/rfc4648

##### `LeastSignificantFirst`

Least significant bit first

# Examples

DNSCurve [base32] uses least significant bit first:

```rust
use data_encoding::BASE32_DNSCURVE;
assert_eq!(BASE32_DNSCURVE.encode(&[0x64, 0x88]), "4321");
assert_eq!(BASE32_DNSCURVE.decode(b"4321").unwrap(), vec![0x64, 0x88]);
```

[base32]: constant.BASE32_DNSCURVE.html

#### Implementations

##### Trait Implementations

- **StructuralPartialEq**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BitOrder) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Eq**
- **Send**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> BitOrder { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `Encoding`

**Attributes:**

- `#[repr(transparent)]`

Base-conversion encoding

See [Specification](struct.Specification.html) for technical details or how to define a new one.

```rust
pub struct Encoding(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn encode_len(self: &Self, len: usize) -> usize { /* ... */ }
  ```
  Returns the encoded length of an input of length `len`

- ```rust
  pub fn encode_mut(self: &Self, input: &[u8], output: &mut [u8]) { /* ... */ }
  ```
  Encodes `input` in `output`

- ```rust
  pub fn encode_mut_str<''a>(self: &Self, input: &[u8], output: &''a mut [u8]) -> &''a str { /* ... */ }
  ```
  Encodes `input` in `output` and returns it as a `&str`

- ```rust
  pub fn encode_append(self: &Self, input: &[u8], output: &mut String) { /* ... */ }
  ```
  Appends the encoding of `input` to `output`

- ```rust
  pub fn new_encoder<''a>(self: &''a Self, output: &''a mut String) -> Encoder<''a> { /* ... */ }
  ```
  Returns an object to encode a fragmented input and append it to `output`

- ```rust
  pub fn encode_write</* synthetic */ impl core::fmt::Write: core::fmt::Write>(self: &Self, input: &[u8], output: &mut impl core::fmt::Write) -> core::fmt::Result { /* ... */ }
  ```
  Writes the encoding of `input` to `output`

- ```rust
  pub fn encode_write_buffer</* synthetic */ impl core::fmt::Write: core::fmt::Write>(self: &Self, input: &[u8], output: &mut impl core::fmt::Write, buffer: &mut [u8]) -> core::fmt::Result { /* ... */ }
  ```
  Writes the encoding of `input` to `output` using a temporary `buffer`

- ```rust
  pub fn encode_display<''a>(self: &''a Self, input: &''a [u8]) -> Display<''a> { /* ... */ }
  ```
  Returns an object to display the encoding of `input`

- ```rust
  pub fn encode(self: &Self, input: &[u8]) -> String { /* ... */ }
  ```
  Returns encoded `input`

- ```rust
  pub fn decode_len(self: &Self, len: usize) -> Result<usize, DecodeError> { /* ... */ }
  ```
  Returns the maximum decoded length of an input of length `len`

- ```rust
  pub fn decode_mut(self: &Self, input: &[u8], output: &mut [u8]) -> Result<usize, DecodePartial> { /* ... */ }
  ```
  Decodes `input` in `output`

- ```rust
  pub fn decode(self: &Self, input: &[u8]) -> Result<Vec<u8>, DecodeError> { /* ... */ }
  ```
  Returns decoded `input`

- ```rust
  pub fn bit_width(self: &Self) -> usize { /* ... */ }
  ```
  Returns the bit-width

- ```rust
  pub fn is_canonical(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the encoding is canonical

- ```rust
  pub fn specification(self: &Self) -> Specification { /* ... */ }
  ```
  Returns the encoding specification

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Encoding) -> bool { /* ... */ }
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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Encoding { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
### Struct `Translate`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

How to translate characters when decoding

The order matters. The first character of the `from` field is translated to the first character
of the `to` field. The second to the second. Etc.

See [Specification](struct.Specification.html) for more information.

```rust
pub struct Translate {
    pub from: alloc::string::String,
    pub to: alloc::string::String,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `from` | `alloc::string::String` | Characters to translate from |
| `to` | `alloc::string::String` | Characters to translate to |

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Translate { /* ... */ }
    ```

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `Wrap`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

How to wrap the output when encoding

See [Specification](struct.Specification.html) for more information.

```rust
pub struct Wrap {
    pub width: usize,
    pub separator: alloc::string::String,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `width` | `usize` | Wrapping width<br><br>Must be a multiple of:<br><br>- 8 for a bit-width of 1 (binary), 3 (octal), and 5 (base32)<br>- 4 for a bit-width of 2 (base4) and 6 (base64)<br>- 2 for a bit-width of 4 (hexadecimal)<br><br>Wrapping is disabled if null. |
| `separator` | `alloc::string::String` | Wrapping characters<br><br>Wrapping is disabled if empty. |

#### Implementations

##### Trait Implementations

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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Wrap { /* ... */ }
    ```

- **Freeze**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
### Struct `Specification`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Base-conversion specification

It is possible to define custom encodings given a specification. To do so, it is important to
understand the theory first.

# Theory

Each subsection has an equivalent subsection in the [Practice](#practice) section.

## Basics

The main idea of a [base-conversion] encoding is to see `[u8]` as numbers written in
little-endian base256 and convert them in another little-endian base. For performance reasons,
this crate restricts this other base to be of size 2 (binary), 4 (base4), 8 (octal), 16
(hexadecimal), 32 (base32), or 64 (base64). The converted number is written as `[u8]` although
it doesn't use all the 256 possible values of `u8`. This crate encodes to ASCII, so only values
smaller than 128 are allowed.

More precisely, we need the following elements:

- The bit-width N: 1 for binary, 2 for base4, 3 for octal, 4 for hexadecimal, 5 for base32, and
  6 for base64
- The [bit-order](enum.BitOrder.html): most or least significant bit first
- The symbols function S from [0, 2<sup>N</sup>) (called values and written `uN`) to symbols
  (represented as `u8` although only ASCII symbols are allowed, i.e. smaller than 128)
- The values partial function V from ASCII to [0, 2<sup>N</sup>), i.e. from `u8` to `uN`
- Whether trailing bits are checked: trailing bits are leading zeros in theory, but since
  numbers are little-endian they come last

For the encoding to be correct (i.e. encoding then decoding gives back the initial input),
V(S(i)) must be defined and equal to i for all i in [0, 2<sup>N</sup>). For the encoding to be
[canonical][canonical] (i.e. different inputs decode to different outputs, or equivalently,
decoding then encoding gives back the initial input), trailing bits must be checked and if V(i)
is defined then S(V(i)) is equal to i for all i.

Encoding and decoding are given by the following pipeline:

```text
[u8] <--1--> [[bit; 8]] <--2--> [[bit; N]] <--3--> [uN] <--4--> [u8]
1: Map bit-order between each u8 and [bit; 8]
2: Base conversion between base 2^8 and base 2^N (check trailing bits)
3: Map bit-order between each [bit; N] and uN
4: Map symbols/values between each uN and u8 (values must be defined)
```

## Extensions

All these extensions make the encoding not canonical.

### Padding

Padding is useful if the following conditions are met:

- the bit-width is 3 (octal), 5 (base32), or 6 (base64)
- the length of the data to encode is not known in advance
- the data must be sent without buffering

Bases for which the bit-width N does not divide 8 may not concatenate encoded data. This comes
from the fact that it is not possible to make the difference between trailing bits and encoding
bits. Padding solves this issue by adding a new character to discriminate between trailing bits
and encoding bits. The idea is to work by blocks of lcm(8, N) bits, where lcm(8, N) is the least
common multiple of 8 and N. When such block is not complete, it is padded.

To preserve correctness, the padding character must not be a symbol.

### Ignore characters when decoding

Ignoring characters when decoding is useful if after encoding some characters are added for
convenience or any other reason (like wrapping). In that case we want to first ignore those
characters before decoding.

To preserve correctness, ignored characters must not contain symbols or the padding character.

### Wrap output when encoding

Wrapping output when encoding is useful if the output is meant to be printed in a document where
width is limited (typically 80-columns documents). In that case, the wrapping width and the
wrapping separator have to be defined.

To preserve correctness, the wrapping separator characters must be ignored (see previous
subsection). As such, wrapping separator characters must also not contain symbols or the padding
character.

### Translate characters when decoding

Translating characters when decoding is useful when encoded data may be copied by a humain
instead of a machine. Humans tend to confuse some characters for others. In that case we want to
translate those characters before decoding.

To preserve correctness, the characters we translate _from_ must not contain symbols or the
padding character, and the characters we translate _to_ must only contain symbols or the padding
character.

# Practice

## Basics

```rust
use data_encoding::{Encoding, Specification};
fn make_encoding(symbols: &str) -> Encoding {
    let mut spec = Specification::new();
    spec.symbols.push_str(symbols);
    spec.encoding().unwrap()
}
let binary = make_encoding("01");
let octal = make_encoding("01234567");
let hexadecimal = make_encoding("0123456789abcdef");
assert_eq!(binary.encode(b"Bit"), "010000100110100101110100");
assert_eq!(octal.encode(b"Bit"), "20464564");
assert_eq!(hexadecimal.encode(b"Bit"), "426974");
```

The `binary` base has 2 symbols `0` and `1` with value 0 and 1 respectively. The `octal` base
has 8 symbols `0` to `7` with value 0 to 7. The `hexadecimal` base has 16 symbols `0` to `9` and
`a` to `f` with value 0 to 15. The following diagram gives the idea of how encoding works in the
previous example (note that we can actually write such diagram only because the bit-order is
most significant first):

```text
[      octal] |  2  :  0  :  4  :  6  :  4  :  5  :  6  :  4  |
[     binary] |0 1 0 0 0 0 1 0|0 1 1 0 1 0 0 1|0 1 1 1 0 1 0 0|
[hexadecimal] |   4   :   2   |   6   :   9   |   7   :   4   |
               ^-- LSB                                       ^-- MSB
```

Note that in theory, these little-endian numbers are read from right to left (the most
significant bit is at the right). Since leading zeros are meaningless (in our usual decimal
notation 0123 is the same as 123), it explains why trailing bits must be zero. Trailing bits may
occur when the bit-width of a base does not divide 8. Only binary, base4, and hexadecimal don't
have trailing bits issues. So let's consider octal and base64, which have trailing bits in
similar circumstances:

```rust
use data_encoding::{Specification, BASE64_NOPAD};
let octal = {
    let mut spec = Specification::new();
    spec.symbols.push_str("01234567");
    spec.encoding().unwrap()
};
assert_eq!(BASE64_NOPAD.encode(b"B"), "Qg");
assert_eq!(octal.encode(b"B"), "204");
```

We have the following diagram, where the base64 values are written between parentheses:

```text
[base64] |   Q(16)   :   g(32)   : [has 4 zero trailing bits]
[ octal] |  2  :  0  :  4  :       [has 1 zero trailing bit ]
         |0 1 0 0 0 0 1 0|0 0 0 0
[ ascii] |       B       |
                          ^-^-^-^-- leading zeros / trailing bits
```

## Extensions

### Padding

For octal and base64, lcm(8, 3) == lcm(8, 6) == 24 bits or 3 bytes. For base32, lcm(8, 5) is 40
bits or 5 bytes. Let's consider octal and base64:

```rust
use data_encoding::{Specification, BASE64};
let octal = {
    let mut spec = Specification::new();
    spec.symbols.push_str("01234567");
    spec.padding = Some('=');
    spec.encoding().unwrap()
};
// We start encoding but we only have "B" for now.
assert_eq!(BASE64.encode(b"B"), "Qg==");
assert_eq!(octal.encode(b"B"), "204=====");
// Now we have "it".
assert_eq!(BASE64.encode(b"it"), "aXQ=");
assert_eq!(octal.encode(b"it"), "322720==");
// By concatenating everything, we may decode the original data.
assert_eq!(BASE64.decode(b"Qg==aXQ=").unwrap(), b"Bit");
assert_eq!(octal.decode(b"204=====322720==").unwrap(), b"Bit");
```

We have the following diagrams:

```text
[base64] |   Q(16)   :   g(32)   :     =     :     =     |
[ octal] |  2  :  0  :  4  :  =  :  =  :  =  :  =  :  =  |
         |0 1 0 0 0 0 1 0|. . . . . . . .|. . . . . . . .|
[ ascii] |       B       |        end of block aligned --^
         ^-- beginning of block aligned

[base64] |   a(26)   :   X(23)   :   Q(16)   :     =     |
[ octal] |  3  :  2  :  2  :  7  :  2  :  0  :  =  :  =  |
         |0 1 1 0 1 0 0 1|0 1 1 1 0 1 0 0|. . . . . . . .|
[ ascii] |       i       |       t       |
```

### Ignore characters when decoding

The typical use-case is to ignore newlines (`\r` and `\n`). But to keep the example small, we
will ignore spaces.

```rust
let mut spec = data_encoding::HEXLOWER.specification();
spec.ignore.push_str(" \t");
let base = spec.encoding().unwrap();
assert_eq!(base.decode(b"42 69 74"), base.decode(b"426974"));
```

### Wrap output when encoding

The typical use-case is to wrap after 64 or 76 characters with a newline (`\r\n` or `\n`). But
to keep the example small, we will wrap after 8 characters with a space.

```rust
let mut spec = data_encoding::BASE64.specification();
spec.wrap.width = 8;
spec.wrap.separator.push_str(" ");
let base64 = spec.encoding().unwrap();
assert_eq!(base64.encode(b"Hey you"), "SGV5IHlv dQ== ");
```

Note that the output always ends with the separator.

### Translate characters when decoding

The typical use-case is to translate lowercase to uppercase or reciprocally, but it is also used
for letters that look alike, like `O0` or `Il1`. Let's illustrate both examples.

```rust
let mut spec = data_encoding::HEXLOWER.specification();
spec.translate.from.push_str("ABCDEFOIl");
spec.translate.to.push_str("abcdef011");
let base = spec.encoding().unwrap();
assert_eq!(base.decode(b"BOIl"), base.decode(b"b011"));
```

[base-conversion]: https://en.wikipedia.org/wiki/Positional_notation#Base_conversion
[canonical]: https://tools.ietf.org/html/rfc4648#section-3.5

```rust
pub struct Specification {
    pub symbols: alloc::string::String,
    pub bit_order: BitOrder,
    pub check_trailing_bits: bool,
    pub padding: Option<char>,
    pub ignore: alloc::string::String,
    pub wrap: Wrap,
    pub translate: Translate,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `symbols` | `alloc::string::String` | Symbols<br><br>The number of symbols must be 2, 4, 8, 16, 32, or 64. Symbols must be ASCII characters<br>(smaller than 128) and they must be unique. |
| `bit_order` | `BitOrder` | Bit-order<br><br>The default is to use most significant bit first since it is the most common. |
| `check_trailing_bits` | `bool` | Check trailing bits<br><br>The default is to check trailing bits. This field is ignored when unnecessary (i.e. for<br>base2, base4, and base16). |
| `padding` | `Option<char>` | Padding<br><br>The default is to not use padding. The padding character must be ASCII and must not be a<br>symbol. |
| `ignore` | `alloc::string::String` | Characters to ignore when decoding<br><br>The default is to not ignore characters when decoding. The characters to ignore must be<br>ASCII and must not be symbols or the padding character. |
| `wrap` | `Wrap` | How to wrap the output when encoding<br><br>The default is to not wrap the output when encoding. The wrapping characters must be ASCII<br>and must not be symbols or the padding character. |
| `translate` | `Translate` | How to translate characters when decoding<br><br>The default is to not translate characters when decoding. The characters to translate from<br>must be ASCII and must not have already been assigned a semantics. The characters to<br>translate to must be ASCII and must have been assigned a semantics (symbol, padding<br>character, or ignored character). |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Specification { /* ... */ }
  ```
  Returns a default specification

- ```rust
  pub fn encoding(self: &Self) -> Result<Encoding, SpecificationError> { /* ... */ }
  ```
  Returns the specified encoding

##### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Specification { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
### Struct `Encoder`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Encodes fragmented input to an output

It is equivalent to use an [`Encoder`] with multiple calls to [`Encoder::append()`] than to
first concatenate all the input and then use [`Encoding::encode_append()`]. In particular, this
function will not introduce padding or wrapping between inputs.

# Examples

```rust
// This is a bit inconvenient but we can't take a long-term reference to data_encoding::BASE64
// because it's a constant. We need to use a static which has an address instead. This will be
// fixed in version 3 of the library.
static BASE64: data_encoding::Encoding = data_encoding::BASE64;
let mut output = String::new();
let mut encoder = BASE64.new_encoder(&mut output);
encoder.append(b"hello");
encoder.append(b"world");
encoder.finalize();
assert_eq!(output, BASE64.encode(b"helloworld"));
```

```rust
pub struct Encoder<''a> {
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
  pub fn append(self: &mut Self, input: &[u8]) { /* ... */ }
  ```
  Encodes the provided input fragment and appends the result to the output

- ```rust
  pub fn finalize(self: Self) { /* ... */ }
  ```
  Makes sure all inputs have been encoded and appended to the output

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `Display`

Wraps an encoding and input for display purposes.

```rust
pub struct Display<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

### Struct `SpecificationError`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Specification error

```rust
pub struct SpecificationError(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SpecificationError { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Constants and Statics

### Constant `HEXLOWER`

Lowercase hexadecimal encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, HEXLOWER};
let mut spec = Specification::new();
spec.symbols.push_str("0123456789abcdef");
assert_eq!(HEXLOWER, spec.encoding().unwrap());
```

# Examples

```rust
use data_encoding::HEXLOWER;
let deadbeef = vec![0xde, 0xad, 0xbe, 0xef];
assert_eq!(HEXLOWER.decode(b"deadbeef").unwrap(), deadbeef);
assert_eq!(HEXLOWER.encode(&deadbeef), "deadbeef");
```

```rust
pub const HEXLOWER: Encoding = _;
```

### Constant `HEXLOWER_PERMISSIVE`

Lowercase hexadecimal encoding with case-insensitive decoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, HEXLOWER_PERMISSIVE};
let mut spec = Specification::new();
spec.symbols.push_str("0123456789abcdef");
spec.translate.from.push_str("ABCDEF");
spec.translate.to.push_str("abcdef");
assert_eq!(HEXLOWER_PERMISSIVE, spec.encoding().unwrap());
```

# Examples

```rust
use data_encoding::HEXLOWER_PERMISSIVE;
let deadbeef = vec![0xde, 0xad, 0xbe, 0xef];
assert_eq!(HEXLOWER_PERMISSIVE.decode(b"DeadBeef").unwrap(), deadbeef);
assert_eq!(HEXLOWER_PERMISSIVE.encode(&deadbeef), "deadbeef");
```

You can also define a shorter name:

```rust
use data_encoding::{Encoding, HEXLOWER_PERMISSIVE};
const HEX: Encoding = HEXLOWER_PERMISSIVE;
```

```rust
pub const HEXLOWER_PERMISSIVE: Encoding = _;
```

### Constant `HEXUPPER`

Uppercase hexadecimal encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, HEXUPPER};
let mut spec = Specification::new();
spec.symbols.push_str("0123456789ABCDEF");
assert_eq!(HEXUPPER, spec.encoding().unwrap());
```

It is compliant with [RFC4648] and known as "base16" or "hex".

# Examples

```rust
use data_encoding::HEXUPPER;
let deadbeef = vec![0xde, 0xad, 0xbe, 0xef];
assert_eq!(HEXUPPER.decode(b"DEADBEEF").unwrap(), deadbeef);
assert_eq!(HEXUPPER.encode(&deadbeef), "DEADBEEF");
```

[RFC4648]: https://tools.ietf.org/html/rfc4648#section-8

```rust
pub const HEXUPPER: Encoding = _;
```

### Constant `HEXUPPER_PERMISSIVE`

Uppercase hexadecimal encoding with case-insensitive decoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, HEXUPPER_PERMISSIVE};
let mut spec = Specification::new();
spec.symbols.push_str("0123456789ABCDEF");
spec.translate.from.push_str("abcdef");
spec.translate.to.push_str("ABCDEF");
assert_eq!(HEXUPPER_PERMISSIVE, spec.encoding().unwrap());
```

# Examples

```rust
use data_encoding::HEXUPPER_PERMISSIVE;
let deadbeef = vec![0xde, 0xad, 0xbe, 0xef];
assert_eq!(HEXUPPER_PERMISSIVE.decode(b"DeadBeef").unwrap(), deadbeef);
assert_eq!(HEXUPPER_PERMISSIVE.encode(&deadbeef), "DEADBEEF");
```

```rust
pub const HEXUPPER_PERMISSIVE: Encoding = _;
```

### Constant `BASE32`

Padded base32 encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE32};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
spec.padding = Some('=');
assert_eq!(BASE32, spec.encoding().unwrap());
```

It conforms to [RFC4648].

[RFC4648]: https://tools.ietf.org/html/rfc4648#section-6

```rust
pub const BASE32: Encoding = _;
```

### Constant `BASE32_NOPAD`

Unpadded base32 encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE32_NOPAD};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
assert_eq!(BASE32_NOPAD, spec.encoding().unwrap());
```

```rust
pub const BASE32_NOPAD: Encoding = _;
```

### Constant `BASE32_NOPAD_NOCASE`

Unpadded base32 encoding with case-insensitive decoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE32_NOPAD_NOCASE};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
spec.translate.from.push_str("abcdefghijklmnopqrstuvwxyz");
spec.translate.to.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
assert_eq!(BASE32_NOPAD_NOCASE, spec.encoding().unwrap());
```

```rust
pub const BASE32_NOPAD_NOCASE: Encoding = _;
```

### Constant `BASE32_NOPAD_VISUAL`

Unpadded base32 encoding with visual error correction during decoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE32_NOPAD_VISUAL};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
spec.translate.from.push_str("01l8");
spec.translate.to.push_str("OIIB");
assert_eq!(BASE32_NOPAD_VISUAL, spec.encoding().unwrap());
```

```rust
pub const BASE32_NOPAD_VISUAL: Encoding = _;
```

### Constant `BASE32HEX`

Padded base32hex encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE32HEX};
let mut spec = Specification::new();
spec.symbols.push_str("0123456789ABCDEFGHIJKLMNOPQRSTUV");
spec.padding = Some('=');
assert_eq!(BASE32HEX, spec.encoding().unwrap());
```

It conforms to [RFC4648].

[RFC4648]: https://tools.ietf.org/html/rfc4648#section-7

```rust
pub const BASE32HEX: Encoding = _;
```

### Constant `BASE32HEX_NOPAD`

Unpadded base32hex encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE32HEX_NOPAD};
let mut spec = Specification::new();
spec.symbols.push_str("0123456789ABCDEFGHIJKLMNOPQRSTUV");
assert_eq!(BASE32HEX_NOPAD, spec.encoding().unwrap());
```

```rust
pub const BASE32HEX_NOPAD: Encoding = _;
```

### Constant `BASE32_DNSSEC`

DNSSEC base32 encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE32_DNSSEC};
let mut spec = Specification::new();
spec.symbols.push_str("0123456789abcdefghijklmnopqrstuv");
spec.translate.from.push_str("ABCDEFGHIJKLMNOPQRSTUV");
spec.translate.to.push_str("abcdefghijklmnopqrstuv");
assert_eq!(BASE32_DNSSEC, spec.encoding().unwrap());
```

It conforms to [RFC5155]:

- It uses a base32 extended hex alphabet.
- It is case-insensitive when decoding and uses lowercase when encoding.
- It does not use padding.

[RFC5155]: https://tools.ietf.org/html/rfc5155

```rust
pub const BASE32_DNSSEC: Encoding = _;
```

### Constant `BASE32_DNSCURVE`

**Attributes:**

- `#[allow(clippy::doc_markdown)]`

DNSCurve base32 encoding

This encoding is a static version of:

```rust
# use data_encoding::{BitOrder, Specification, BASE32_DNSCURVE};
let mut spec = Specification::new();
spec.symbols.push_str("0123456789bcdfghjklmnpqrstuvwxyz");
spec.bit_order = BitOrder::LeastSignificantFirst;
spec.translate.from.push_str("BCDFGHJKLMNPQRSTUVWXYZ");
spec.translate.to.push_str("bcdfghjklmnpqrstuvwxyz");
assert_eq!(BASE32_DNSCURVE, spec.encoding().unwrap());
```

It conforms to [DNSCurve].

[DNSCurve]: https://dnscurve.org/in-implement.html

```rust
pub const BASE32_DNSCURVE: Encoding = _;
```

### Constant `BASE64`

Padded base64 encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE64};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
spec.padding = Some('=');
assert_eq!(BASE64, spec.encoding().unwrap());
```

It conforms to [RFC4648].

[RFC4648]: https://tools.ietf.org/html/rfc4648#section-4

```rust
pub const BASE64: Encoding = _;
```

### Constant `BASE64_NOPAD`

Unpadded base64 encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE64_NOPAD};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
assert_eq!(BASE64_NOPAD, spec.encoding().unwrap());
```

```rust
pub const BASE64_NOPAD: Encoding = _;
```

### Constant `BASE64_MIME`

MIME base64 encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE64_MIME};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
spec.padding = Some('=');
spec.wrap.width = 76;
spec.wrap.separator.push_str("\r\n");
assert_eq!(BASE64_MIME, spec.encoding().unwrap());
```

It does not exactly conform to [RFC2045] because it does not print the header
and does not ignore all characters.

[RFC2045]: https://tools.ietf.org/html/rfc2045

```rust
pub const BASE64_MIME: Encoding = _;
```

### Constant `BASE64_MIME_PERMISSIVE`

MIME base64 encoding without trailing bits check

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE64_MIME_PERMISSIVE};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
spec.padding = Some('=');
spec.wrap.width = 76;
spec.wrap.separator.push_str("\r\n");
spec.check_trailing_bits = false;
assert_eq!(BASE64_MIME_PERMISSIVE, spec.encoding().unwrap());
```

It does not exactly conform to [RFC2045] because it does not print the header
and does not ignore all characters.

[RFC2045]: https://tools.ietf.org/html/rfc2045

```rust
pub const BASE64_MIME_PERMISSIVE: Encoding = _;
```

### Constant `BASE64URL`

Padded base64url encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE64URL};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
spec.padding = Some('=');
assert_eq!(BASE64URL, spec.encoding().unwrap());
```

It conforms to [RFC4648].

[RFC4648]: https://tools.ietf.org/html/rfc4648#section-5

```rust
pub const BASE64URL: Encoding = _;
```

### Constant `BASE64URL_NOPAD`

Unpadded base64url encoding

This encoding is a static version of:

```rust
# use data_encoding::{Specification, BASE64URL_NOPAD};
let mut spec = Specification::new();
spec.symbols.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
assert_eq!(BASE64URL_NOPAD, spec.encoding().unwrap());
```

```rust
pub const BASE64URL_NOPAD: Encoding = _;
```

