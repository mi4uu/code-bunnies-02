# Crate Documentation

**Version:** 0.4.3

**Format Version:** 43

# Module `hex`

Encoding and decoding hex strings.

For most cases, you can simply use the [`decode`], [`encode`] and
[`encode_upper`] functions. If you need a bit more control, use the traits
[`ToHex`] and [`FromHex`] instead.

# Example

```
# #[cfg(not(feature = "alloc"))]
# let mut output = [0; 0x18];
#
# #[cfg(not(feature = "alloc"))]
# hex::encode_to_slice(b"Hello world!", &mut output).unwrap();
#
# #[cfg(not(feature = "alloc"))]
# let hex_string = ::core::str::from_utf8(&output).unwrap();
#
# #[cfg(feature = "alloc")]
let hex_string = hex::encode("Hello world!");

println!("{}", hex_string); // Prints "48656c6c6f20776f726c6421"

# assert_eq!(hex_string, "48656c6c6f20776f726c6421");
```

## Traits

### Trait `ToHex`

Encoding values as hex string.

This trait is implemented for all `T` which implement `AsRef<[u8]>`. This
includes `String`, `str`, `Vec<u8>` and `[u8]`.

# Example

```
use hex::ToHex;

println!("{}", "Hello world!".encode_hex::<String>());
# assert_eq!("Hello world!".encode_hex::<String>(), "48656c6c6f20776f726c6421".to_string());
```

*Note*: instead of using this trait, you might want to use [`encode()`].

```rust
pub trait ToHex {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `encode_hex`: Encode the hex strict representing `self` into the result. Lower case
- `encode_hex_upper`: Encode the hex strict representing `self` into the result. Upper case

#### Implementations

This trait is implemented for the following types:

- `T` with <T: AsRef<[u8]>>

### Trait `FromHex`

Types that can be decoded from a hex string.

This trait is implemented for `Vec<u8>` and small `u8`-arrays.

# Example

```
use core::str;
use hex::FromHex;

let buffer = <[u8; 12]>::from_hex("48656c6c6f20776f726c6421")?;
let string = str::from_utf8(&buffer).expect("invalid buffer length");

println!("{}", string); // prints "Hello world!"
# assert_eq!("Hello world!", string);
# Ok::<(), hex::FromHexError>(())
```

```rust
pub trait FromHex: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `Error`

##### Required Methods

- `from_hex`: Creates an instance of type `Self` from the given hex string, or fails

#### Implementations

This trait is implemented for the following types:

- `alloc::vec::Vec<u8>`
- `[u8; 1]`
- `[u8; 2]`
- `[u8; 3]`
- `[u8; 4]`
- `[u8; 5]`
- `[u8; 6]`
- `[u8; 7]`
- `[u8; 8]`
- `[u8; 9]`
- `[u8; 10]`
- `[u8; 11]`
- `[u8; 12]`
- `[u8; 13]`
- `[u8; 14]`
- `[u8; 15]`
- `[u8; 16]`
- `[u8; 17]`
- `[u8; 18]`
- `[u8; 19]`
- `[u8; 20]`
- `[u8; 21]`
- `[u8; 22]`
- `[u8; 23]`
- `[u8; 24]`
- `[u8; 25]`
- `[u8; 26]`
- `[u8; 27]`
- `[u8; 28]`
- `[u8; 29]`
- `[u8; 30]`
- `[u8; 31]`
- `[u8; 32]`
- `[u8; 33]`
- `[u8; 34]`
- `[u8; 35]`
- `[u8; 36]`
- `[u8; 37]`
- `[u8; 38]`
- `[u8; 39]`
- `[u8; 40]`
- `[u8; 41]`
- `[u8; 42]`
- `[u8; 43]`
- `[u8; 44]`
- `[u8; 45]`
- `[u8; 46]`
- `[u8; 47]`
- `[u8; 48]`
- `[u8; 49]`
- `[u8; 50]`
- `[u8; 51]`
- `[u8; 52]`
- `[u8; 53]`
- `[u8; 54]`
- `[u8; 55]`
- `[u8; 56]`
- `[u8; 57]`
- `[u8; 58]`
- `[u8; 59]`
- `[u8; 60]`
- `[u8; 61]`
- `[u8; 62]`
- `[u8; 63]`
- `[u8; 64]`
- `[u8; 65]`
- `[u8; 66]`
- `[u8; 67]`
- `[u8; 68]`
- `[u8; 69]`
- `[u8; 70]`
- `[u8; 71]`
- `[u8; 72]`
- `[u8; 73]`
- `[u8; 74]`
- `[u8; 75]`
- `[u8; 76]`
- `[u8; 77]`
- `[u8; 78]`
- `[u8; 79]`
- `[u8; 80]`
- `[u8; 81]`
- `[u8; 82]`
- `[u8; 83]`
- `[u8; 84]`
- `[u8; 85]`
- `[u8; 86]`
- `[u8; 87]`
- `[u8; 88]`
- `[u8; 89]`
- `[u8; 90]`
- `[u8; 91]`
- `[u8; 92]`
- `[u8; 93]`
- `[u8; 94]`
- `[u8; 95]`
- `[u8; 96]`
- `[u8; 97]`
- `[u8; 98]`
- `[u8; 99]`
- `[u8; 100]`
- `[u8; 101]`
- `[u8; 102]`
- `[u8; 103]`
- `[u8; 104]`
- `[u8; 105]`
- `[u8; 106]`
- `[u8; 107]`
- `[u8; 108]`
- `[u8; 109]`
- `[u8; 110]`
- `[u8; 111]`
- `[u8; 112]`
- `[u8; 113]`
- `[u8; 114]`
- `[u8; 115]`
- `[u8; 116]`
- `[u8; 117]`
- `[u8; 118]`
- `[u8; 119]`
- `[u8; 120]`
- `[u8; 121]`
- `[u8; 122]`
- `[u8; 123]`
- `[u8; 124]`
- `[u8; 125]`
- `[u8; 126]`
- `[u8; 127]`
- `[u8; 128]`
- `[u8; 160]`
- `[u8; 192]`
- `[u8; 200]`
- `[u8; 224]`
- `[u8; 256]`
- `[u8; 384]`
- `[u8; 512]`
- `[u8; 768]`
- `[u8; 1024]`
- `[u8; 2048]`
- `[u8; 4096]`
- `[u8; 8192]`
- `[u8; 16384]`
- `[u8; 32768]`
- `[u8; 65536]`
- `[u8; 131072]`
- `[u8; 262144]`
- `[u8; 524288]`
- `[u8; 1048576]`
- `[u8; 2097152]`
- `[u8; 4194304]`
- `[u8; 8388608]`
- `[u8; 16777216]`
- `[u8; 33554432]`
- `[u8; 67108864]`
- `[u8; 134217728]`
- `[u8; 268435456]`
- `[u8; 536870912]`
- `[u8; 1073741824]`
- `[u8; 2147483648]`
- `[u8; 4294967296]`

## Functions

### Function `encode`

**Attributes:**

- `#[must_use]`
- `#[<cfg>(feature = "alloc")]`

Encodes `data` as hex string using lowercase characters.

Lowercase characters are used (e.g. `f9b4ca`). The resulting string's
length is always even, each byte in `data` is always encoded using two hex
digits. Thus, the resulting string contains exactly twice as many bytes as
the input data.

# Example

```
assert_eq!(hex::encode("Hello world!"), "48656c6c6f20776f726c6421");
assert_eq!(hex::encode(vec![1, 2, 3, 15, 16]), "0102030f10");
```

```rust
pub fn encode<T: AsRef<[u8]>>(data: T) -> alloc::string::String { /* ... */ }
```

### Function `encode_upper`

**Attributes:**

- `#[must_use]`
- `#[<cfg>(feature = "alloc")]`

Encodes `data` as hex string using uppercase characters.

Apart from the characters' casing, this works exactly like `encode()`.

# Example

```
assert_eq!(hex::encode_upper("Hello world!"), "48656C6C6F20776F726C6421");
assert_eq!(hex::encode_upper(vec![1, 2, 3, 15, 16]), "0102030F10");
```

```rust
pub fn encode_upper<T: AsRef<[u8]>>(data: T) -> alloc::string::String { /* ... */ }
```

### Function `decode`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Decodes a hex string into raw bytes.

Both, upper and lower case characters are valid in the input string and can
even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings).

# Example

```
assert_eq!(
    hex::decode("48656c6c6f20776f726c6421"),
    Ok("Hello world!".to_owned().into_bytes())
);

assert_eq!(hex::decode("123"), Err(hex::FromHexError::OddLength));
assert!(hex::decode("foo").is_err());
```

```rust
pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<alloc::vec::Vec<u8>, FromHexError> { /* ... */ }
```

### Function `decode_to_slice`

Decode a hex string into a mutable bytes slice.

Both, upper and lower case characters are valid in the input string and can
even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings).

# Example

```
let mut bytes = [0u8; 4];
assert_eq!(hex::decode_to_slice("6b697769", &mut bytes as &mut [u8]), Ok(()));
assert_eq!(&bytes, b"kiwi");
```

```rust
pub fn decode_to_slice<T: AsRef<[u8]>>(data: T, out: &mut [u8]) -> Result<(), FromHexError> { /* ... */ }
```

### Function `encode_to_slice`

Encodes some bytes into a mutable slice of bytes.

The output buffer, has to be able to hold at least `input.len() * 2` bytes,
otherwise this function will return an error.

# Example

```
# use hex::FromHexError;
# fn main() -> Result<(), FromHexError> {
let mut bytes = [0u8; 4 * 2];

hex::encode_to_slice(b"kiwi", &mut bytes)?;
assert_eq!(&bytes, b"6b697769");
# Ok(())
# }
```

```rust
pub fn encode_to_slice<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<(), FromHexError> { /* ... */ }
```

## Re-exports

### Re-export `FromHexError`

```rust
pub use crate::error::FromHexError;
```

