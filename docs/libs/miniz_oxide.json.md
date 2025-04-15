# Crate Documentation

**Version:** 0.8.8

**Format Version:** 43

# Module `miniz_oxide`

A pure rust replacement for the [miniz](https://github.com/richgel999/miniz)
DEFLATE/zlib encoder/decoder.
Used a rust back-end for the
[flate2](https://github.com/alexcrichton/flate2-rs) crate.

# Usage
## Simple compression/decompression:
``` rust

use miniz_oxide::inflate::decompress_to_vec;
use miniz_oxide::deflate::compress_to_vec;

fn roundtrip(data: &[u8]) {
    let compressed = compress_to_vec(data, 6);
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
#   let _ = decompressed;
}

# roundtrip(b"Test_data test data lalalal blabla");

## Modules

## Module `deflate`

**Attributes:**

- `#[<cfg>(feature = "with-alloc")]`

This module contains functionality for compression.

```rust
pub mod deflate { /* ... */ }
```

### Modules

## Module `core`

Streaming compression functionality.

```rust
pub mod core { /* ... */ }
```

### Modules

## Module `deflate_flags`

```rust
pub mod deflate_flags { /* ... */ }
```

### Constants and Statics

#### Constant `TDEFL_WRITE_ZLIB_HEADER`

Whether to use a zlib wrapper.

```rust
pub const TDEFL_WRITE_ZLIB_HEADER: u32 = 0x0000_1000;
```

#### Constant `TDEFL_COMPUTE_ADLER32`

Should we compute the adler32 checksum.

```rust
pub const TDEFL_COMPUTE_ADLER32: u32 = 0x0000_2000;
```

#### Constant `TDEFL_GREEDY_PARSING_FLAG`

Should we use greedy parsing (as opposed to lazy parsing where look ahead one or more
bytes to check for better matches.)

```rust
pub const TDEFL_GREEDY_PARSING_FLAG: u32 = 0x0000_4000;
```

#### Constant `TDEFL_NONDETERMINISTIC_PARSING_FLAG`

Used in miniz to skip zero-initializing hash and dict. We don't do this here, so
this flag is ignored.

```rust
pub const TDEFL_NONDETERMINISTIC_PARSING_FLAG: u32 = 0x0000_8000;
```

#### Constant `TDEFL_RLE_MATCHES`

Only look for matches with a distance of 0.

```rust
pub const TDEFL_RLE_MATCHES: u32 = 0x0001_0000;
```

#### Constant `TDEFL_FILTER_MATCHES`

Only use matches that are at least 6 bytes long.

```rust
pub const TDEFL_FILTER_MATCHES: u32 = 0x0002_0000;
```

#### Constant `TDEFL_FORCE_ALL_STATIC_BLOCKS`

Force the compressor to only output static blocks. (Blocks using the default huffman codes
specified in the deflate specification.)

```rust
pub const TDEFL_FORCE_ALL_STATIC_BLOCKS: u32 = 0x0004_0000;
```

#### Constant `TDEFL_FORCE_ALL_RAW_BLOCKS`

Force the compressor to only output raw/uncompressed blocks.

```rust
pub const TDEFL_FORCE_ALL_RAW_BLOCKS: u32 = 0x0008_0000;
```

### Types

#### Enum `CompressionStrategy`

**Attributes:**

- `#[repr(i32)]`

Strategy setting for compression.

The non-default settings offer some special-case compression variants.

```rust
pub enum CompressionStrategy {
    Default = 0,
    Filtered = 1,
    HuffmanOnly = 2,
    RLE = 3,
    Fixed = 4,
}
```

##### Variants

###### `Default`

Don't use any of the special strategies.

Discriminant: `0`

Discriminant value: `0`

###### `Filtered`

Only use matches that are at least 5 bytes long.

Discriminant: `1`

Discriminant value: `1`

###### `HuffmanOnly`

Don't look for matches, only huffman encode the literals.

Discriminant: `2`

Discriminant value: `2`

###### `RLE`

Only look for matches with a distance of 1, i.e do run-length encoding only.

Discriminant: `3`

Discriminant value: `3`

###### `Fixed`

Only use static/fixed blocks. (Blocks using the default huffman codes
specified in the deflate specification.)

Discriminant: `4`

Discriminant value: `4`

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompressionStrategy { /* ... */ }
    ```

- **Copy**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: CompressionStrategy) -> Self { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CompressionStrategy) -> bool { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
#### Enum `TDEFLFlush`

A list of deflate flush types.

```rust
pub enum TDEFLFlush {
    None = 0,
    Sync = 2,
    Full = 3,
    Finish = 4,
}
```

##### Variants

###### `None`

Normal operation.

Compress as much as there is space for, and then return waiting for more input.

Discriminant: `0`

Discriminant value: `0`

###### `Sync`

Try to flush all the current data and output an empty raw block.

Discriminant: `2`

Discriminant value: `2`

###### `Full`

Same as [`Sync`][Self::Sync], but reset the dictionary so that the following data does not
depend on previous data.

Discriminant: `3`

Discriminant value: `3`

###### `Finish`

Try to flush everything and end the deflate stream.

On success this will yield a [`TDEFLStatus::Done`] return status.

Discriminant: `4`

Discriminant value: `4`

##### Implementations

###### Methods

- ```rust
  pub const fn new(flush: i32) -> core::result::Result<Self, MZError> { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(flush: MZFlush) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TDEFLFlush { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TDEFLFlush) -> bool { /* ... */ }
    ```

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
- **RefUnwindSafe**
- **Eq**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `TDEFLStatus`

**Attributes:**

- `#[repr(i32)]`

Return status of compression.

```rust
pub enum TDEFLStatus {
    BadParam = -2,
    PutBufFailed = -1,
    Okay = 0,
    Done = 1,
}
```

##### Variants

###### `BadParam`

Usage error.

This indicates that either the [`CompressorOxide`] experienced a previous error, or the
stream has already been [`TDEFLFlush::Finish`]'d.

Discriminant: `-2`

Discriminant value: `-2`

###### `PutBufFailed`

Error putting data into output buffer.

This usually indicates a too-small buffer.

Discriminant: `-1`

Discriminant value: `-1`

###### `Okay`

Compression succeeded normally.

Discriminant: `0`

Discriminant value: `0`

###### `Done`

Compression succeeded and the deflate stream was ended.

This is the result of calling compression with [`TDEFLFlush::Finish`].

Discriminant: `1`

Discriminant value: `1`

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TDEFLStatus { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TDEFLStatus) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `CompressorOxide`

Main compression struct.

```rust
pub struct CompressorOxide {
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
  pub fn new(flags: u32) -> Self { /* ... */ }
  ```
  Create a new `CompressorOxide` with the given flags.

- ```rust
  pub const fn adler32(self: &Self) -> u32 { /* ... */ }
  ```
  Get the adler32 checksum of the currently encoded data.

- ```rust
  pub const fn prev_return_status(self: &Self) -> TDEFLStatus { /* ... */ }
  ```
  Get the return status of the previous [`compress`](fn.compress.html)

- ```rust
  pub const fn flags(self: &Self) -> i32 { /* ... */ }
  ```
  Get the raw compressor flags.

- ```rust
  pub const fn data_format(self: &Self) -> DataFormat { /* ... */ }
  ```
  Returns whether the compressor is wrapping the data in a zlib format or not.

- ```rust
  pub fn reset(self: &mut Self) { /* ... */ }
  ```
  Reset the state of the compressor, keeping the same parameters.

- ```rust
  pub fn set_compression_level(self: &mut Self, level: CompressionLevel) { /* ... */ }
  ```
  Set the compression level of the compressor.

- ```rust
  pub fn set_compression_level_raw(self: &mut Self, level: u8) { /* ... */ }
  ```
  Set the compression level of the compressor using an integer value.

- ```rust
  pub fn set_format_and_level(self: &mut Self, data_format: DataFormat, level: u8) { /* ... */ }
  ```
  Update the compression settings of the compressor.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Initialize the compressor with a level of 4, zlib wrapper and

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `CallbackFunc`

Callback function and user used in `compress_to_output`.

```rust
pub struct CallbackFunc<''a> {
    pub put_buf_func: &''a mut dyn FnMut(&[u8]) -> bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `put_buf_func` | `&''a mut dyn FnMut(&[u8]) -> bool` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
### Functions

#### Function `compress`

Main compression function. Tries to compress as much as possible from `in_buf` and
puts compressed output into `out_buf`.

The value of `flush` determines if the compressor should attempt to flush all output
and alternatively try to finish the stream.

Use [`TDEFLFlush::Finish`] on the final call to signal that the stream is finishing.

Note that this function does not keep track of whether a flush marker has been output, so
if called using [`TDEFLFlush::Sync`], the caller needs to ensure there is enough space in the
output buffer if they want to avoid repeated flush markers.
See #105 for details.

# Returns
Returns a tuple containing the current status of the compressor, the current position
in the input buffer and the current position in the output buffer.

```rust
pub fn compress(d: &mut CompressorOxide, in_buf: &[u8], out_buf: &mut [u8], flush: TDEFLFlush) -> (TDEFLStatus, usize, usize) { /* ... */ }
```

#### Function `compress_to_output`

Main compression function. Callbacks output.

# Returns
Returns a tuple containing the current status of the compressor, the current position
in the input buffer.

The caller is responsible for ensuring the `CallbackFunc` struct will not cause undefined
behaviour.

```rust
pub fn compress_to_output</* synthetic */ impl FnMut(&[u8]) -> bool: FnMut(&[u8]) -> bool>(d: &mut CompressorOxide, in_buf: &[u8], flush: TDEFLFlush, callback_func: impl FnMut(&[u8]) -> bool) -> (TDEFLStatus, usize) { /* ... */ }
```

#### Function `create_comp_flags_from_zip_params`

Create a set of compression flags using parameters used by zlib and other compressors.
Mainly intended for use with transition from c libraries as it deals with raw integers.

# Parameters
`level` determines compression level. Clamped to maximum of 10. Negative values result in
`CompressionLevel::DefaultLevel`.
`window_bits`: Above 0, wraps the stream in a zlib wrapper, 0 or negative for a raw deflate
stream.
`strategy`: Sets the strategy if this conforms to any of the values in `CompressionStrategy`.

# Notes
This function may be removed or moved to the `miniz_oxide_c_api` in the future.

```rust
pub fn create_comp_flags_from_zip_params(level: i32, window_bits: i32, strategy: i32) -> u32 { /* ... */ }
```

## Module `stream`

Extra streaming compression functionality.

As of now this is mainly intended for use to build a higher-level wrapper.

There is no DeflateState as the needed state is contained in the compressor struct itself.

```rust
pub mod stream { /* ... */ }
```

### Functions

#### Function `deflate`

Try to compress from input to output with the given [`CompressorOxide`].

# Errors

Returns [`MZError::Buf`] If the size of the `output` slice is empty or no progress was made due
to lack of expected input data, or if called without [`MZFlush::Finish`] after the compression
was already finished.

Returns [`MZError::Param`] if the compressor parameters are set wrong.

Returns [`MZError::Stream`] when lower-level decompressor returns a
[`TDEFLStatus::PutBufFailed`]; may not actually be possible.

```rust
pub fn deflate(compressor: &mut crate::deflate::core::CompressorOxide, input: &[u8], output: &mut [u8], flush: crate::MZFlush) -> crate::StreamResult { /* ... */ }
```

### Types

#### Enum `CompressionLevel`

**Attributes:**

- `#[repr(i32)]`

How much processing the compressor should do to compress the data.
`NoCompression` and `Bestspeed` have special meanings, the other levels determine the number
of checks for matches in the hash chains and whether to use lazy or greedy parsing.

```rust
pub enum CompressionLevel {
    NoCompression = 0,
    BestSpeed = 1,
    BestCompression = 9,
    UberCompression = 10,
    DefaultLevel = 6,
    DefaultCompression = -1,
}
```

##### Variants

###### `NoCompression`

Don't do any compression, only output uncompressed blocks.

Discriminant: `0`

Discriminant value: `0`

###### `BestSpeed`

Fast compression. Uses a special compression routine that is optimized for speed.

Discriminant: `1`

Discriminant value: `1`

###### `BestCompression`

Slow/high compression. Do a lot of checks to try to find good matches.

Discriminant: `9`

Discriminant value: `9`

###### `UberCompression`

Even more checks, can be very slow.

Discriminant: `10`

Discriminant value: `10`

###### `DefaultLevel`

Default compromise between speed and compression.

Discriminant: `6`

Discriminant value: `6`

###### `DefaultCompression`

Use the default compression level.

Discriminant: `-1`

Discriminant value: `-1`

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CompressionLevel) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompressionLevel { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Functions

#### Function `compress_to_vec`

Compress the input data to a vector, using the specified compression level (0-10).

```rust
pub fn compress_to_vec(input: &[u8], level: u8) -> crate::alloc::vec::Vec<u8> { /* ... */ }
```

#### Function `compress_to_vec_zlib`

Compress the input data to a vector, using the specified compression level (0-10), and with a
zlib wrapper.

```rust
pub fn compress_to_vec_zlib(input: &[u8], level: u8) -> crate::alloc::vec::Vec<u8> { /* ... */ }
```

## Module `inflate`

This module contains functionality for decompression.

```rust
pub mod inflate { /* ... */ }
```

### Modules

## Module `core`

Streaming decompression functionality.

```rust
pub mod core { /* ... */ }
```

### Modules

## Module `inflate_flags`

Flags to [`decompress()`] to control how inflation works.

These define bits for a bitmask argument.

```rust
pub mod inflate_flags { /* ... */ }
```

### Constants and Statics

#### Constant `TINFL_FLAG_PARSE_ZLIB_HEADER`

Should we try to parse a zlib header?

If unset, the function will expect an RFC1951 deflate stream.  If set, it will expect a
RFC1950 zlib wrapper around the deflate stream.

```rust
pub const TINFL_FLAG_PARSE_ZLIB_HEADER: u32 = 1;
```

#### Constant `TINFL_FLAG_HAS_MORE_INPUT`

There will be more input that hasn't been given to the decompressor yet.

This is useful when you want to decompress what you have so far,
even if you know there is probably more input that hasn't gotten here yet (_e.g._, over a
network connection).  When [`decompress()`][super::decompress] reaches the end of the input
without finding the end of the compressed stream, it will return
[`TINFLStatus::NeedsMoreInput`][super::TINFLStatus::NeedsMoreInput] if this is set,
indicating that you should get more data before calling again.  If not set, it will return
[`TINFLStatus::FailedCannotMakeProgress`][super::TINFLStatus::FailedCannotMakeProgress]
suggesting the stream is corrupt, since you claimed it was all there.

```rust
pub const TINFL_FLAG_HAS_MORE_INPUT: u32 = 2;
```

#### Constant `TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF`

The output buffer should not wrap around.

```rust
pub const TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF: u32 = 4;
```

#### Constant `TINFL_FLAG_COMPUTE_ADLER32`

Calculate the adler32 checksum of the output data even if we're not inflating a zlib stream.

If [`TINFL_FLAG_IGNORE_ADLER32`] is specified, it will override this.

NOTE: Enabling/disabling this between calls to decompress will result in an incorrect
checksum.

```rust
pub const TINFL_FLAG_COMPUTE_ADLER32: u32 = 8;
```

#### Constant `TINFL_FLAG_IGNORE_ADLER32`

Ignore adler32 checksum even if we are inflating a zlib stream.

Overrides [`TINFL_FLAG_COMPUTE_ADLER32`] if both are enabled.

NOTE: This flag does not exist in miniz as it does not support this and is a
custom addition for miniz_oxide.

NOTE: Should not be changed from enabled to disabled after decompression has started,
this will result in checksum failure (outside the unlikely event where the checksum happens
to match anyway).

```rust
pub const TINFL_FLAG_IGNORE_ADLER32: u32 = 64;
```

### Types

#### Struct `DecompressorOxide`

**Attributes:**

- `#[<cfg_attr>(not(feature = "rustc-dep-of-std"), derive(Clone))]`
- `#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]`

Main decompression struct.


```rust
pub struct DecompressorOxide {
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
  pub fn new() -> DecompressorOxide { /* ... */ }
  ```
  Create a new tinfl_decompressor with all fields set to 0.

- ```rust
  pub fn init(self: &mut Self) { /* ... */ }
  ```
  Set the current state to `Start`.

- ```rust
  pub fn adler32(self: &Self) -> Option<u32> { /* ... */ }
  ```
  Returns the adler32 checksum of the currently decompressed data.

- ```rust
  pub fn adler32_header(self: &Self) -> Option<u32> { /* ... */ }
  ```
  Returns the adler32 that was read from the zlib header if it exists.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Send**
- **RefUnwindSafe**
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

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DecompressorOxide { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Create a new tinfl_decompressor with all fields set to 0.

### Functions

#### Function `decompress`

Main decompression function. Keeps decompressing data from `in_buf` until the `in_buf` is
empty, `out` is full, the end of the deflate stream is hit, or there is an error in the
deflate stream.

# Arguments

`r` is a [`DecompressorOxide`] struct with the state of this stream.

`in_buf` is a reference to the compressed data that is to be decompressed. The decompressor will
start at the first byte of this buffer.

`out` is a reference to the buffer that will store the decompressed data, and that
stores previously decompressed data if any.

* The offset given by `out_pos` indicates where in the output buffer slice writing should start.
* If [`TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF`] is not set, the output buffer is used in a
  wrapping manner, and it's size is required to be a power of 2.
* The decompression function normally needs access to 32KiB of the previously decompressed data
  (or to the beginning of the decompressed data if less than 32KiB has been decompressed.)
    - If this data is not available, decompression may fail.
    - Some deflate compressors allow specifying a window size which limits match distances to
      less than this, or alternatively an RLE mode where matches will only refer to the previous byte
      and thus allows a smaller output buffer. The window size can be specified in the zlib
      header structure, however, the header data should not be relied on to be correct.

`flags` indicates settings and status to the decompression function.
* The [`TINFL_FLAG_HAS_MORE_INPUT`] has to be specified if more compressed data is to be provided
  in a subsequent call to this function.
* See the the [`inflate_flags`] module for details on other flags.

# Returns

Returns a tuple containing the status of the compressor, the number of input bytes read, and the
number of bytes output to `out`.

```rust
pub fn decompress(r: &mut DecompressorOxide, in_buf: &[u8], out: &mut [u8], out_pos: usize, flags: u32) -> (TINFLStatus, usize, usize) { /* ... */ }
```

### Constants and Statics

#### Constant `TINFL_LZ_DICT_SIZE`

```rust
pub const TINFL_LZ_DICT_SIZE: usize = 32_768;
```

## Module `stream`

**Attributes:**

- `#[<cfg>(not(feature = "rustc-dep-of-std"))]`

Extra streaming decompression functionality.

As of now this is mainly intended for use to build a higher-level wrapper.

```rust
pub mod stream { /* ... */ }
```

### Types

#### Struct `MinReset`

Resets state, without performing expensive ops (e.g. zeroing buffer)

Note that not zeroing buffer can lead to security issues when dealing with untrusted input.

```rust
pub struct MinReset;
```

##### Implementations

###### Trait Implementations

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

- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ResetPolicy**
  - ```rust
    fn reset(self: &Self, state: &mut InflateState) { /* ... */ }
    ```

#### Struct `ZeroReset`

Resets state and zero memory, continuing to use the same data format.

```rust
pub struct ZeroReset;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
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

- **RefUnwindSafe**
- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ResetPolicy**
  - ```rust
    fn reset(self: &Self, state: &mut InflateState) { /* ... */ }
    ```

#### Struct `FullReset`

Full reset of the state, including zeroing memory.

Requires to provide new data format.

```rust
pub struct FullReset(pub crate::DataFormat);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::DataFormat` |  |

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

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ResetPolicy**
  - ```rust
    fn reset(self: &Self, state: &mut InflateState) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Freeze**
- **Unpin**
#### Struct `InflateState`

A struct that compbines a decompressor with extra data for streaming decompression.


```rust
pub struct InflateState {
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
  pub fn new(data_format: DataFormat) -> InflateState { /* ... */ }
  ```
  Create a new state.

- ```rust
  pub fn new_boxed(data_format: DataFormat) -> Box<InflateState> { /* ... */ }
  ```
  Create a new state on the heap.

- ```rust
  pub fn decompressor(self: &mut Self) -> &mut DecompressorOxide { /* ... */ }
  ```
  Access the innner decompressor.

- ```rust
  pub const fn last_status(self: &Self) -> TINFLStatus { /* ... */ }
  ```
  Return the status of the last call to `inflate` with this `InflateState`.

- ```rust
  pub fn new_boxed_with_window_bits(window_bits: i32) -> Box<InflateState> { /* ... */ }
  ```
  Create a new state using miniz/zlib style window bits parameter.

- ```rust
  pub fn reset(self: &mut Self, data_format: DataFormat) { /* ... */ }
  ```
  Reset the decompressor without re-allocating memory, using the given

- ```rust
  pub fn reset_as<T: ResetPolicy>(self: &mut Self, policy: T) { /* ... */ }
  ```
  Resets the state according to specified policy.

###### Trait Implementations

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

- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
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

- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InflateState { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Traits

#### Trait `ResetPolicy`

Tag that determines reset policy of [InflateState](struct.InflateState.html)

```rust
pub trait ResetPolicy {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `reset`: Performs reset

##### Implementations

This trait is implemented for the following types:

- `MinReset`
- `ZeroReset`
- `FullReset`

### Functions

#### Function `inflate`

Try to decompress from `input` to `output` with the given [`InflateState`]

# `flush`

Generally, the various [`MZFlush`] flags have meaning only on the compression side.  They can be
supplied here, but the only one that has any semantic meaning is [`MZFlush::Finish`], which is a
signal that the stream is expected to finish, and failing to do so is an error.  It isn't
necessary to specify it when the stream ends; you'll still get returned a
[`MZStatus::StreamEnd`] anyway.  Other values either have no effect or cause errors.  It's
likely that you'll almost always just want to use [`MZFlush::None`].

# Errors

Returns [`MZError::Buf`] if the size of the `output` slice is empty or no progress was made due
to lack of expected input data, or if called with [`MZFlush::Finish`] and input wasn't all
consumed.

Returns [`MZError::Data`] if this or a a previous call failed with an error return from
[`TINFLStatus`]; probably indicates corrupted data.

Returns [`MZError::Stream`] when called with [`MZFlush::Full`] (meaningless on
decompression), or when called without [`MZFlush::Finish`] after an earlier call with
[`MZFlush::Finish`] has been made.

```rust
pub fn inflate(state: &mut InflateState, input: &[u8], output: &mut [u8], flush: crate::MZFlush) -> crate::StreamResult { /* ... */ }
```

### Types

#### Enum `TINFLStatus`

**Attributes:**

- `#[<cfg_attr>(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]`
- `#[repr(i8)]`

Return status codes.

```rust
pub enum TINFLStatus {
    FailedCannotMakeProgress = { _ },
    BadParam = { _ },
    Adler32Mismatch = { _ },
    Failed = { _ },
    Done = { _ },
    NeedsMoreInput = { _ },
    HasMoreOutput = { _ },
}
```

##### Variants

###### `FailedCannotMakeProgress`

More input data was expected, but the caller indicated that there was no more data, so the
input stream is likely truncated.

This can't happen if you have provided the
[`TINFL_FLAG_HAS_MORE_INPUT`][core::inflate_flags::TINFL_FLAG_HAS_MORE_INPUT] flag to the
decompression.  By setting that flag, you indicate more input exists but is not provided,
and so reaching the end of the input data without finding the end of the compressed stream
would instead return a [`NeedsMoreInput`][Self::NeedsMoreInput] status.

Discriminant: `{ _ }`

Discriminant value: `-4`

###### `BadParam`

The output buffer is an invalid size; consider the `flags` parameter.

Discriminant: `{ _ }`

Discriminant value: `-3`

###### `Adler32Mismatch`

The decompression went fine, but the adler32 checksum did not match the one
provided in the header.

Discriminant: `{ _ }`

Discriminant value: `-2`

###### `Failed`

Failed to decompress due to invalid data.

Discriminant: `{ _ }`

Discriminant value: `-1`

###### `Done`

Finished decompression without issues.

This indicates the end of the compressed stream has been reached.

Discriminant: `{ _ }`

Discriminant value: `0`

###### `NeedsMoreInput`

The decompressor needs more input data to continue decompressing.

This occurs when there's no more consumable input, but the end of the stream hasn't been
reached, and you have supplied the
[`TINFL_FLAG_HAS_MORE_INPUT`][core::inflate_flags::TINFL_FLAG_HAS_MORE_INPUT] flag to the
decompressor.  Had you not supplied that flag (which would mean you were asserting that you
believed all the data was available) you would have gotten a
[`FailedCannotMakeProcess`][Self::FailedCannotMakeProgress] instead.

Discriminant: `{ _ }`

Discriminant value: `1`

###### `HasMoreOutput`

There is still pending data that didn't fit in the output buffer.

Discriminant: `{ _ }`

Discriminant value: `2`

##### Implementations

###### Methods

- ```rust
  pub fn from_i32(value: i32) -> Option<TINFLStatus> { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TINFLStatus { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TINFLStatus) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Freeze**
#### Struct `DecompressError`

**Attributes:**

- `#[<cfg>(feature = "with-alloc")]`

Struct return when decompress_to_vec functions fail.

```rust
pub struct DecompressError {
    pub status: TINFLStatus,
    pub output: crate::alloc::vec::Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `TINFLStatus` | Decompressor status on failure. See [TINFLStatus] for details. |
| `output` | `crate::alloc::vec::Vec<u8>` | The currently decompressed data if any. |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `decompress_to_vec`

**Attributes:**

- `#[inline]`
- `#[<cfg>(feature = "with-alloc")]`

Decompress the deflate-encoded data in `input` to a vector.

NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error.
It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or
ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure.

```rust
pub fn decompress_to_vec(input: &[u8]) -> Result<crate::alloc::vec::Vec<u8>, DecompressError> { /* ... */ }
```

#### Function `decompress_to_vec_zlib`

**Attributes:**

- `#[inline]`
- `#[<cfg>(feature = "with-alloc")]`

Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.

NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error.
It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or
ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure.

```rust
pub fn decompress_to_vec_zlib(input: &[u8]) -> Result<crate::alloc::vec::Vec<u8>, DecompressError> { /* ... */ }
```

#### Function `decompress_to_vec_with_limit`

**Attributes:**

- `#[inline]`
- `#[<cfg>(feature = "with-alloc")]`

Decompress the deflate-encoded data in `input` to a vector.

The vector is grown to at most `max_size` bytes; if the data does not fit in that size,
the error [struct][DecompressError] will contain the status [`TINFLStatus::HasMoreOutput`] and the data that was decompressed on failure.

As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small.
It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] on failure.

```rust
pub fn decompress_to_vec_with_limit(input: &[u8], max_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError> { /* ... */ }
```

#### Function `decompress_to_vec_zlib_with_limit`

**Attributes:**

- `#[inline]`
- `#[<cfg>(feature = "with-alloc")]`

Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.
The vector is grown to at most `max_size` bytes; if the data does not fit in that size,
the error [struct][DecompressError] will contain the status [`TINFLStatus::HasMoreOutput`] and the data that was decompressed on failure.

As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small.
It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] on failure.

```rust
pub fn decompress_to_vec_zlib_with_limit(input: &[u8], max_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError> { /* ... */ }
```

#### Function `decompress_slice_iter_to_slice`

**Attributes:**

- `#[<cfg>(not(feature = "rustc-dep-of-std"))]`

Decompress one or more source slices from an iterator into the output slice.

* On success, returns the number of bytes that were written.
* On failure, returns the failure status code.

This will fail if the output buffer is not large enough, but in that case
the output buffer will still contain the partial decompression.

* `out` the output buffer.
* `it` the iterator of input slices.
* `zlib_header` if the first slice out of the iterator is expected to have a
  Zlib header. Otherwise the slices are assumed to be the deflate data only.
* `ignore_adler32` if the adler32 checksum should be calculated or not.

```rust
pub fn decompress_slice_iter_to_slice<''out, ''inp, /* synthetic */ impl Iterator<Item = &'inp [u8]>: Iterator<Item = &''inp [u8]>>(out: &''out mut [u8], it: impl Iterator<Item = &''inp [u8]>, zlib_header: bool, ignore_adler32: bool) -> Result<usize, TINFLStatus> { /* ... */ }
```

## Types

### Enum `MZFlush`

**Attributes:**

- `#[<cfg_attr>(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]`
- `#[repr(i32)]`

A list of flush types.

See <http://www.bolet.org/~pornin/deflate-flush.html> for more in-depth info.

```rust
pub enum MZFlush {
    None = 0,
    Partial = 1,
    Sync = 2,
    Full = 3,
    Finish = 4,
    Block = 5,
}
```

#### Variants

##### `None`

Don't force any flushing.
Used when more input data is expected.

Discriminant: `0`

Discriminant value: `0`

##### `Partial`

Zlib partial flush.
Currently treated as [`Sync`].

Discriminant: `1`

Discriminant value: `1`

##### `Sync`

Finish compressing the currently buffered data, and output an empty raw block.
Has no use in decompression.

Discriminant: `2`

Discriminant value: `2`

##### `Full`

Same as [`Sync`], but resets the compression dictionary so that further compressed
data does not depend on data compressed before the flush.

Has no use in decompression, and is an error to supply in that case.

Discriminant: `3`

Discriminant value: `3`

##### `Finish`

Attempt to flush the remaining data and end the stream.

Discriminant: `4`

Discriminant value: `4`

##### `Block`

Not implemented.

Discriminant: `5`

Discriminant value: `5`

#### Implementations

##### Methods

- ```rust
  pub fn new(flush: i32) -> Result<Self, MZError> { /* ... */ }
  ```
  Create an MZFlush value from an integer value.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

  - ```rust
    fn from(flush: MZFlush) -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MZFlush { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MZFlush) -> bool { /* ... */ }
    ```

- **Eq**
- **Send**
- **Sync**
- **RefUnwindSafe**
### Enum `MZStatus`

**Attributes:**

- `#[<cfg_attr>(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]`
- `#[repr(i32)]`

A list of miniz successful status codes.

These are emitted as the [`Ok`] side of a [`MZResult`] in the [`StreamResult`] returned from
[`deflate::stream::deflate()`] or [`inflate::stream::inflate()`].

```rust
pub enum MZStatus {
    Ok = 0,
    StreamEnd = 1,
    NeedDict = 2,
}
```

#### Variants

##### `Ok`

Operation succeeded.

Some data was decompressed or compressed; see the byte counters in the [`StreamResult`] for
details.

Discriminant: `0`

Discriminant value: `0`

##### `StreamEnd`

Operation succeeded and end of deflate stream was found.

X-ref [`TINFLStatus::Done`][inflate::TINFLStatus::Done] or
[`TDEFLStatus::Done`][deflate::core::TDEFLStatus::Done] for `inflate` or `deflate`
respectively.

Discriminant: `1`

Discriminant value: `1`

##### `NeedDict`

Unused

Discriminant: `2`

Discriminant value: `2`

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MZStatus { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MZStatus) -> bool { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Enum `MZError`

**Attributes:**

- `#[<cfg_attr>(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]`
- `#[repr(i32)]`

A list of miniz failed status codes.

These are emitted as the [`Err`] side of a [`MZResult`] in the [`StreamResult`] returned from
[`deflate::stream::deflate()`] or [`inflate::stream::inflate()`].

```rust
pub enum MZError {
    ErrNo = -1,
    Stream = -2,
    Data = -3,
    Mem = -4,
    Buf = -5,
    Version = -6,
    Param = -10_000,
}
```

#### Variants

##### `ErrNo`

Unused

Discriminant: `-1`

Discriminant value: `-1`

##### `Stream`

General stream error.

See [`inflate::stream::inflate()`] docs for details of how it can occur there.

See [`deflate::stream::deflate()`] docs for how it can in principle occur there, though it's
believed impossible in practice.

Discriminant: `-2`

Discriminant value: `-2`

##### `Data`

Error in inflation; see [`inflate::stream::inflate()`] for details.

Not returned from [`deflate::stream::deflate()`].

Discriminant: `-3`

Discriminant value: `-3`

##### `Mem`

Unused

Discriminant: `-4`

Discriminant value: `-4`

##### `Buf`

Buffer-related error.

See the docs of [`deflate::stream::deflate()`] or [`inflate::stream::inflate()`] for details
of when it would trigger in the one you're using.

Discriminant: `-5`

Discriminant value: `-5`

##### `Version`

Unused

Discriminant: `-6`

Discriminant value: `-6`

##### `Param`

Bad parameters.

This can be returned from [`deflate::stream::deflate()`] in the case of bad parameters.  See
[`TDEFLStatus::BadParam`][deflate::core::TDEFLStatus::BadParam].

Discriminant: `-10_000`

Discriminant value: `-10000`

#### Implementations

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **StructuralPartialEq**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MZError { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MZError) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Enum `DataFormat`

**Attributes:**

- `#[<cfg_attr>(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]`
- `#[non_exhaustive]`

How compressed data is wrapped.

```rust
pub enum DataFormat {
    Zlib,
    ZLibIgnoreChecksum,
    Raw,
}
```

#### Variants

##### `Zlib`

Wrapped using the [zlib](http://www.zlib.org/rfc-zlib.html) format.

##### `ZLibIgnoreChecksum`

Zlib wrapped but ignore and don't compute the adler32 checksum.
Currently only used for inflate, behaves the same as Zlib for compression.

##### `Raw`

Raw DEFLATE.

#### Implementations

##### Methods

- ```rust
  pub fn from_window_bits(window_bits: i32) -> DataFormat { /* ... */ }
  ```

- ```rust
  pub fn to_window_bits(self: Self) -> i32 { /* ... */ }
  ```

##### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DataFormat { /* ... */ }
    ```

- **UnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DataFormat) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Type Alias `MZResult`

`Result` alias for all miniz status codes both successful and failed.

```rust
pub type MZResult = Result<MZStatus, MZError>;
```

### Struct `StreamResult`

**Attributes:**

- `#[<cfg>(not(feature = "rustc-dep-of-std"))]`

A structure containing the result of a call to the inflate or deflate streaming functions.

```rust
pub struct StreamResult {
    pub bytes_consumed: usize,
    pub bytes_written: usize,
    pub status: MZResult,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `bytes_consumed` | `usize` | The number of bytes consumed from the input slice. |
| `bytes_written` | `usize` | The number of bytes written to the output slice. |
| `status` | `MZResult` | The return status of the call. |

#### Implementations

##### Methods

- ```rust
  pub const fn error(error: MZError) -> StreamResult { /* ... */ }
  ```

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StreamResult) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StreamResult { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
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

  - ```rust
    fn from(res: StreamResult) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(res: &StreamResult) -> Self { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

