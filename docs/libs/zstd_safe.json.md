# Crate Documentation

**Version:** 7.2.4

**Format Version:** 43

# Module `zstd_safe`

Minimal safe wrapper around zstd-sys.

This crates provides a minimal translation of the [zstd-sys] methods.
For a more comfortable high-level library, see the [zstd] crate.

[zstd-sys]: https://crates.io/crates/zstd-sys
[zstd]: https://crates.io/crates/zstd

Most of the functions here map 1-for-1 to a function from
[the C zstd library][zstd-c] mentioned in their descriptions.
Check the [source documentation][doc] for more information on their
behaviour.

[doc]: https://facebook.github.io/zstd/zstd_manual.html
[zstd-c]: https://facebook.github.io/zstd/

Features denoted as experimental in the C library are hidden behind an
`experimental` feature.

## Types

### Type Alias `CompressionLevel`

Represents the compression level used by zstd.

```rust
pub type CompressionLevel = i32;
```

### Type Alias `ErrorCode`

Represents a possible error from the zstd library.

```rust
pub type ErrorCode = usize;
```

### Type Alias `SafeResult`

Wrapper result around most zstd functions.

Either a success code (usually number of bytes written), or an error code.

```rust
pub type SafeResult = Result<usize, ErrorCode>;
```

### Struct `ContentSizeError`

Indicates an error happened when parsing the frame content size.

The stream may be corrupted, or the given frame prefix was too small.

```rust
pub struct ContentSizeError;
```

#### Implementations

##### Trait Implementations

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

### Struct `CCtx`

Compression context

It is recommended to allocate a single context per thread and re-use it
for many compression operations.

```rust
pub struct CCtx<''a>(/* private field */, /* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn try_create() -> Option<Self> { /* ... */ }
  ```
  Tries to create a new context.

- ```rust
  pub fn create() -> Self { /* ... */ }
  ```
  Wrap `ZSTD_createCCtx`

- ```rust
  pub fn compress<C: WriteBuf + ?Sized>(self: &mut Self, dst: &mut C, src: &[u8], compression_level: CompressionLevel) -> SafeResult { /* ... */ }
  ```
  Wraps the `ZSTD_compressCCtx()` function

- ```rust
  pub fn compress2<C: WriteBuf + ?Sized>(self: &mut Self, dst: &mut C, src: &[u8]) -> SafeResult { /* ... */ }
  ```
  Wraps the `ZSTD_compress2()` function.

- ```rust
  pub fn compress_using_dict<C: WriteBuf + ?Sized>(self: &mut Self, dst: &mut C, src: &[u8], dict: &[u8], compression_level: CompressionLevel) -> SafeResult { /* ... */ }
  ```
  Wraps the `ZSTD_compress_usingDict()` function.

- ```rust
  pub fn compress_using_cdict<C: WriteBuf + ?Sized>(self: &mut Self, dst: &mut C, src: &[u8], cdict: &CDict<''_>) -> SafeResult { /* ... */ }
  ```
  Wraps the `ZSTD_compress_usingCDict()` function.

- ```rust
  pub fn init(self: &mut Self, compression_level: CompressionLevel) -> SafeResult { /* ... */ }
  ```
  Initializes the context with the given compression level.

- ```rust
  pub fn load_dictionary(self: &mut Self, dict: &[u8]) -> SafeResult { /* ... */ }
  ```
  Tries to load a dictionary.

- ```rust
  pub fn ref_cdict<''b>(self: &mut Self, cdict: &CDict<''b>) -> SafeResult
where
    ''b: ''a { /* ... */ }
  ```
  Wraps the `ZSTD_CCtx_refCDict()` function.

- ```rust
  pub fn disable_dictionary(self: &mut Self) -> SafeResult { /* ... */ }
  ```
  Return to "no-dictionary" mode.

- ```rust
  pub fn ref_prefix<''b>(self: &mut Self, prefix: &''b [u8]) -> SafeResult
where
    ''b: ''a { /* ... */ }
  ```
  Use some prefix as single-use dictionary for the next compressed frame.

- ```rust
  pub fn compress_stream<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>, input: &mut InBuffer<''_>) -> SafeResult { /* ... */ }
  ```
  Performs a step of a streaming compression operation.

- ```rust
  pub fn compress_stream2<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>, input: &mut InBuffer<''_>, end_op: zstd_sys::ZSTD_EndDirective) -> SafeResult { /* ... */ }
  ```
  Performs a step of a streaming compression operation.

- ```rust
  pub fn flush_stream<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>) -> SafeResult { /* ... */ }
  ```
  Flush any intermediate buffer.

- ```rust
  pub fn end_stream<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>) -> SafeResult { /* ... */ }
  ```
  Ends the stream.

- ```rust
  pub fn sizeof(self: &Self) -> usize { /* ... */ }
  ```
  Returns the size currently used by this context.

- ```rust
  pub fn reset(self: &mut Self, reset: ResetDirective) -> SafeResult { /* ... */ }
  ```
  Resets the state of the context.

- ```rust
  pub fn set_parameter(self: &mut Self, param: CParameter) -> SafeResult { /* ... */ }
  ```
  Sets a compression parameter.

- ```rust
  pub fn set_pledged_src_size(self: &mut Self, pledged_src_size: Option<u64>) -> SafeResult { /* ... */ }
  ```
  Guarantee that the input size will be this value.

- ```rust
  pub fn in_size() -> usize { /* ... */ }
  ```
  Returns the recommended input buffer size.

- ```rust
  pub fn out_size() -> usize { /* ... */ }
  ```
  Returns the recommended output buffer size.

##### Trait Implementations

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `DCtx`

A Decompression Context.

The lifetime references the potential dictionary used for this context.

If no dictionary was used, it will most likely be `'static`.

Same as `DStream`.

```rust
pub struct DCtx<''a>(/* private field */, /* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn try_create() -> Option<Self> { /* ... */ }
  ```
  Try to create a new decompression context.

- ```rust
  pub fn create() -> Self { /* ... */ }
  ```
  Creates a new decoding context.

- ```rust
  pub fn decompress<C: WriteBuf + ?Sized>(self: &mut Self, dst: &mut C, src: &[u8]) -> SafeResult { /* ... */ }
  ```
  Fully decompress the given frame.

- ```rust
  pub fn decompress_using_dict<C: WriteBuf + ?Sized>(self: &mut Self, dst: &mut C, src: &[u8], dict: &[u8]) -> SafeResult { /* ... */ }
  ```
  Fully decompress the given frame using a dictionary.

- ```rust
  pub fn decompress_using_ddict<C: WriteBuf + ?Sized>(self: &mut Self, dst: &mut C, src: &[u8], ddict: &DDict<''_>) -> SafeResult { /* ... */ }
  ```
  Fully decompress the given frame using a dictionary.

- ```rust
  pub fn init(self: &mut Self) -> SafeResult { /* ... */ }
  ```
  Initializes an existing `DStream` for decompression.

- ```rust
  pub fn reset(self: &mut Self, reset: ResetDirective) -> SafeResult { /* ... */ }
  ```
  Resets the state of the context.

- ```rust
  pub fn load_dictionary(self: &mut Self, dict: &[u8]) -> SafeResult { /* ... */ }
  ```
  Loads a dictionary.

- ```rust
  pub fn disable_dictionary(self: &mut Self) -> SafeResult { /* ... */ }
  ```
  Return to "no-dictionary" mode.

- ```rust
  pub fn ref_ddict<''b>(self: &mut Self, ddict: &DDict<''b>) -> SafeResult
where
    ''b: ''a { /* ... */ }
  ```
  References a dictionary.

- ```rust
  pub fn ref_prefix<''b>(self: &mut Self, prefix: &''b [u8]) -> SafeResult
where
    ''b: ''a { /* ... */ }
  ```
  Use some prefix as single-use dictionary for the next frame.

- ```rust
  pub fn set_parameter(self: &mut Self, param: DParameter) -> SafeResult { /* ... */ }
  ```
  Sets a decompression parameter.

- ```rust
  pub fn decompress_stream<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>, input: &mut InBuffer<''_>) -> SafeResult { /* ... */ }
  ```
  Performs a step of a streaming decompression operation.

- ```rust
  pub fn in_size() -> usize { /* ... */ }
  ```
  Wraps the `ZSTD_DStreamInSize()` function.

- ```rust
  pub fn out_size() -> usize { /* ... */ }
  ```
  Wraps the `ZSTD_DStreamOutSize()` function.

- ```rust
  pub fn sizeof(self: &Self) -> usize { /* ... */ }
  ```
  Wraps the `ZSTD_sizeof_DCtx()` function.

##### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Unpin**
- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Send**
### Struct `CDict`

Compression dictionary.

```rust
pub struct CDict<''a>(/* private field */, /* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn create(dict_buffer: &[u8], compression_level: CompressionLevel) -> Self { /* ... */ }
  ```
  Prepare a dictionary to compress data.

- ```rust
  pub fn try_create(dict_buffer: &[u8], compression_level: CompressionLevel) -> Option<Self> { /* ... */ }
  ```
  Prepare a dictionary to compress data.

- ```rust
  pub fn sizeof(self: &Self) -> usize { /* ... */ }
  ```
  Returns the _current_ memory usage of this dictionary.

- ```rust
  pub fn get_dict_id(self: &Self) -> Option<NonZeroU32> { /* ... */ }
  ```
  Returns the dictionary ID for this dict.

##### Trait Implementations

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

- **Send**
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

- **Freeze**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
### Struct `DDict`

A digested decompression dictionary.

```rust
pub struct DDict<''a>(/* private field */, /* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn create(dict_buffer: &[u8]) -> Self { /* ... */ }
  ```

- ```rust
  pub fn try_create(dict_buffer: &[u8]) -> Option<Self> { /* ... */ }
  ```

- ```rust
  pub fn sizeof(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn get_dict_id(self: &Self) -> Option<NonZeroU32> { /* ... */ }
  ```
  Returns the dictionary ID for this dict.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **Send**
### Type Alias `CStream`

Compression stream.

Same as `CCtx`.

```rust
pub type CStream<''a> = CCtx<''a>;
```

### Struct `InBuffer`

Wrapper around an input buffer.

Bytes will be read starting at `src[pos]`.

`pos` will be updated after reading.

```rust
pub struct InBuffer<''a> {
    pub src: &''a [u8],
    pub pos: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `src` | `&''a [u8]` |  |
| `pos` | `usize` |  |

#### Implementations

##### Methods

- ```rust
  pub fn around(src: &''a [u8]) -> Self { /* ... */ }
  ```
  Returns a new `InBuffer` around the given slice.

- ```rust
  pub fn pos(self: &Self) -> usize { /* ... */ }
  ```
  Returns the current cursor position.

- ```rust
  pub fn set_pos(self: &mut Self, pos: usize) { /* ... */ }
  ```
  Sets the new cursor position.

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `OutBuffer`

Wrapper around an output buffer.

`C` is usually either `[u8]` or `Vec<u8>`.

Bytes will be written starting at `dst[pos]`.

`pos` will be updated after writing.

# Invariant

`pos <= dst.capacity()`

```rust
pub struct OutBuffer<''a, C: WriteBuf + ?Sized> {
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
  pub fn around(dst: &''a mut C) -> Self { /* ... */ }
  ```
  Returns a new `OutBuffer` around the given slice.

- ```rust
  pub fn around_pos(dst: &''a mut C, pos: usize) -> Self { /* ... */ }
  ```
  Returns a new `OutBuffer` around the given slice, starting at the given position.

- ```rust
  pub fn pos(self: &Self) -> usize { /* ... */ }
  ```
  Returns the current cursor position.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the capacity of the underlying buffer.

- ```rust
  pub unsafe fn set_pos(self: &mut Self, pos: usize) { /* ... */ }
  ```
  Sets the new cursor position.

- ```rust
  pub fn as_slice<''b>(self: &''b Self) -> &''a [u8]
where
    ''b: ''a { /* ... */ }
  ```
  Returns the part of this buffer that was written to.

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut u8 { /* ... */ }
  ```
  Returns a pointer to the start of this buffer.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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

### Type Alias `DStream`

A Decompression stream.

Same as `DCtx`.

```rust
pub type DStream<''a> = DCtx<''a>;
```

### Enum `ResetDirective`

What kind of context reset should be applied.

```rust
pub enum ResetDirective {
    SessionOnly,
    Parameters,
    SessionAndParameters,
}
```

#### Variants

##### `SessionOnly`

Only the session will be reset.

All parameters will be preserved (including the dictionary).
But any frame being processed will be dropped.

It can be useful to start re-using a context after an error or when an
ongoing compression is no longer needed.

##### `Parameters`

Only reset parameters (including dictionary or referenced prefix).

All parameters will be reset to default values.

This can only be done between sessions - no compression or decompression must be ongoing.

##### `SessionAndParameters`

Reset both the session and parameters.

The result is similar to a newly created context.

#### Implementations

##### Trait Implementations

- **UnwindSafe**
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

- **Send**
- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
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

### Enum `CParameter`

**Attributes:**

- `#[non_exhaustive]`

A compression parameter.

```rust
pub enum CParameter {
    TargetCBlockSize(u32),
    CompressionLevel(CompressionLevel),
    WindowLog(u32),
    HashLog(u32),
    ChainLog(u32),
    SearchLog(u32),
    MinMatch(u32),
    TargetLength(u32),
    Strategy(Strategy),
    EnableLongDistanceMatching(bool),
    LdmHashLog(u32),
    LdmMinMatch(u32),
    LdmBucketSizeLog(u32),
    LdmHashRateLog(u32),
    ContentSizeFlag(bool),
    ChecksumFlag(bool),
    DictIdFlag(bool),
    NbWorkers(u32),
    JobSize(u32),
    OverlapSizeLog(u32),
}
```

#### Variants

##### `TargetCBlockSize`

Target CBlock size.

Tries to make compressed blocks fit in this size (not a guarantee, just a target).
Useful to reduce end-to-end latency in low-bandwidth environments.

No target when the value is 0.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `CompressionLevel`

Compression level to use.

Compression levels are global presets for the other compression parameters.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CompressionLevel` |  |

##### `WindowLog`

Maximum allowed back-reference distance.

The actual distance is 2 power "this value".

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `HashLog`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `ChainLog`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `SearchLog`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `MinMatch`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `TargetLength`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `Strategy`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Strategy` |  |

##### `EnableLongDistanceMatching`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

##### `LdmHashLog`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `LdmMinMatch`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `LdmBucketSizeLog`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `LdmHashRateLog`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `ContentSizeFlag`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

##### `ChecksumFlag`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

##### `DictIdFlag`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

##### `NbWorkers`

How many threads will be spawned.

With a default value of `0`, `compress_stream*` functions block until they complete.

With any other value (including 1, a single compressing thread), these methods directly
return, and the actual compression is done in the background (until a flush is requested).

Note: this will only work if the `zstdmt` feature is activated.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `JobSize`

Size in bytes of a compression job.

Does not have any effect when `NbWorkers` is set to 0.

The default value of 0 finds the best job size based on the compression parameters.

Note: this will only work if the `zstdmt` feature is activated.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `OverlapSizeLog`

Specifies how much overlap must be given to each worker.

Possible values:

* `0` (default value): automatic overlap based on compression strategy.
* `1`: No overlap
* `1 < n < 9`: Overlap a fraction of the window size, defined as `1/(2 ^ 9-n)`.
* `9`: Full overlap (as long as the window)
* `9 < m`: Will return an error.

Note: this will only work if the `zstdmt` feature is activated.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CParameter { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CParameter) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Enum `DParameter`

**Attributes:**

- `#[non_exhaustive]`

A decompression parameter.

```rust
pub enum DParameter {
    WindowLogMax(u32),
}
```

#### Variants

##### `WindowLogMax`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DParameter { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DParameter) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
## Traits

### Trait `WriteBuf`

Describe a bytes container, like `Vec<u8>`.

Represents a contiguous segment of allocated memory, a prefix of which is initialized.

It allows starting from an uninitializes chunk of memory and writing to it, progressively
initializing it. No re-allocation typically occur after the initial creation.

The main implementors are:
* `Vec<u8>` and similar structures. These hold both a length (initialized data) and a capacity
  (allocated memory).

  Use `Vec::with_capacity` to create an empty `Vec` with non-zero capacity, and the length
  field will be updated to cover the data written to it (as long as it fits in the given
  capacity).
* `[u8]` and `[u8; N]`. These must start already-initialized, and will not be resized. It will
  be up to the caller to only use the part that was written (as returned by the various writing
  operations).
* `std::io::Cursor<T: WriteBuf>`. This will ignore data before the cursor's position, and
  append data after that.

```rust
pub unsafe trait WriteBuf {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `as_slice`: Returns the valid data part of this container. Should only cover initialized data.
- `capacity`: Returns the full capacity of this container. May include uninitialized data.
- `as_mut_ptr`: Returns a pointer to the start of the data.
- `filled_until`: Indicates that the first `n` bytes of the container have been written.

#### Provided Methods

- ```rust
  unsafe fn write_from<F>(self: &mut Self, f: F) -> SafeResult
where
    F: FnOnce(*mut c_void, usize) -> SafeResult { /* ... */ }
  ```
  Call the given closure using the pointer and capacity from `self`.

#### Implementations

This trait is implemented for the following types:

- `std::io::Cursor<T>` with <T>
- `&''a mut std::vec::Vec<u8>` with <''a>
- `std::vec::Vec<u8>`
- `[u8; N]` with <const N: usize>
- `[u8]`

## Functions

### Function `version_number`

Returns the ZSTD version.

Returns `major * 10_000 + minor * 100 + patch`.
So 1.5.3 would be returned as `10_503`.

```rust
pub fn version_number() -> u32 { /* ... */ }
```

### Function `version_string`

Returns a string representation of the ZSTD version.

For example "1.5.3".

```rust
pub fn version_string() -> &''static str { /* ... */ }
```

### Function `min_c_level`

Returns the minimum (fastest) compression level supported.

This is likely going to be a _very_ large negative number.

```rust
pub fn min_c_level() -> CompressionLevel { /* ... */ }
```

### Function `max_c_level`

Returns the maximum (slowest) compression level supported.

```rust
pub fn max_c_level() -> CompressionLevel { /* ... */ }
```

### Function `compress`

Wraps the `ZSTD_compress` function.

This will try to compress `src` entirely and write the result to `dst`, returning the number of
bytes written. If `dst` is too small to hold the compressed content, an error will be returned.

For streaming operations that don't require to store the entire input/output in memory, see
`compress_stream`.

```rust
pub fn compress<C: WriteBuf + ?Sized>(dst: &mut C, src: &[u8], compression_level: CompressionLevel) -> SafeResult { /* ... */ }
```

### Function `decompress`

Wraps the `ZSTD_decompress` function.

This is a one-step decompression (not streaming).

You will need to make sure `dst` is large enough to store all the decompressed content, or an
error will be returned.

If decompression was a success, the number of bytes written will be returned.

```rust
pub fn decompress<C: WriteBuf + ?Sized>(dst: &mut C, src: &[u8]) -> SafeResult { /* ... */ }
```

### Function `get_decompressed_size`

**⚠️ Deprecated**: Use ZSTD_getFrameContentSize instead

Wraps the `ZSTD_getDecompressedSize` function.

Returns `None` if the size could not be found, or if the content is actually empty.

```rust
pub fn get_decompressed_size(src: &[u8]) -> Option<core::num::NonZeroU64> { /* ... */ }
```

### Function `compress_bound`

Maximum compressed size in worst case single-pass scenario

```rust
pub fn compress_bound(src_size: usize) -> usize { /* ... */ }
```

### Function `get_error_name`

Returns the error string associated with an error code.

```rust
pub fn get_error_name(code: usize) -> &''static str { /* ... */ }
```

### Function `create_cdict`

Wraps the `ZSTD_createCDict()` function.

```rust
pub fn create_cdict(dict_buffer: &[u8], compression_level: CompressionLevel) -> CDict<''static> { /* ... */ }
```

### Function `compress_using_cdict`

Wraps the `ZSTD_compress_usingCDict()` function.

```rust
pub fn compress_using_cdict(cctx: &mut CCtx<''_>, dst: &mut [u8], src: &[u8], cdict: &CDict<''_>) -> SafeResult { /* ... */ }
```

### Function `create_ddict`

Wraps the `ZSTD_createDDict()` function.

It copies the dictionary internally, so the resulting `DDict` is `'static`.

```rust
pub fn create_ddict(dict_buffer: &[u8]) -> DDict<''static> { /* ... */ }
```

### Function `decompress_using_ddict`

Wraps the `ZSTD_decompress_usingDDict()` function.

```rust
pub fn decompress_using_ddict(dctx: &mut DCtx<''_>, dst: &mut [u8], src: &[u8], ddict: &DDict<''_>) -> SafeResult { /* ... */ }
```

### Function `create_cstream`

Allocates a new `CStream`.

```rust
pub fn create_cstream<''a>() -> CStream<''a> { /* ... */ }
```

### Function `init_cstream`

Prepares an existing `CStream` for compression at the given level.

```rust
pub fn init_cstream(zcs: &mut CStream<''_>, compression_level: CompressionLevel) -> SafeResult { /* ... */ }
```

### Function `find_frame_compressed_size`

Wraps the `ZSTD_findFrameCompressedSize()` function.

`src` should contain at least an entire frame.

```rust
pub fn find_frame_compressed_size(src: &[u8]) -> SafeResult { /* ... */ }
```

### Function `get_frame_content_size`

Wraps the `ZSTD_getFrameContentSize()` function.

Args:
* `src`: A prefix of the compressed frame. It should at least include the frame header.

Returns:
* `Err(ContentSizeError)` if `src` is too small of a prefix, or if it appears corrupted.
* `Ok(None)` if the frame does not include a content size.
* `Ok(Some(content_size_in_bytes))` otherwise.

```rust
pub fn get_frame_content_size(src: &[u8]) -> Result<Option<u64>, ContentSizeError> { /* ... */ }
```

### Function `get_dict_id_from_dict`

Wraps the `ZSTD_getDictID_fromDict()` function.

Returns `None` if the dictionary is not a valid zstd dictionary.

```rust
pub fn get_dict_id_from_dict(dict: &[u8]) -> Option<core::num::NonZeroU32> { /* ... */ }
```

### Function `get_dict_id_from_frame`

Wraps the `ZSTD_getDictID_fromFrame()` function.

Returns `None` if the dictionary ID could not be decoded. This may happen if:
* The frame was not encoded with a dictionary.
* The frame intentionally did not include dictionary ID.
* The dictionary was non-conformant.
* `src` is too small and does not include the frame header.
* `src` is not a valid zstd frame prefix.

```rust
pub fn get_dict_id_from_frame(src: &[u8]) -> Option<core::num::NonZeroU32> { /* ... */ }
```

### Function `train_from_buffer`

**Attributes:**

- `#[<cfg>(feature = "zdict_builder")]`
- `#[<cfg_attr>(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]`

Wraps the `ZDICT_trainFromBuffer()` function.

```rust
pub fn train_from_buffer<C: WriteBuf + ?Sized>(dict_buffer: &mut C, samples_buffer: &[u8], samples_sizes: &[usize]) -> SafeResult { /* ... */ }
```

### Function `get_dict_id`

**Attributes:**

- `#[<cfg>(feature = "zdict_builder")]`
- `#[<cfg_attr>(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]`

Wraps the `ZDICT_getDictID()` function.

```rust
pub fn get_dict_id(dict_buffer: &[u8]) -> Option<core::num::NonZeroU32> { /* ... */ }
```

## Constants and Statics

### Constant `BLOCKSIZELOG_MAX`

```rust
pub const BLOCKSIZELOG_MAX: u32 = zstd_sys::ZSTD_BLOCKSIZELOG_MAX;
```

### Constant `BLOCKSIZE_MAX`

```rust
pub const BLOCKSIZE_MAX: u32 = zstd_sys::ZSTD_BLOCKSIZE_MAX;
```

### Constant `CLEVEL_DEFAULT`

```rust
pub const CLEVEL_DEFAULT: CompressionLevel = _;
```

### Constant `CONTENTSIZE_ERROR`

```rust
pub const CONTENTSIZE_ERROR: u64 = _;
```

### Constant `CONTENTSIZE_UNKNOWN`

```rust
pub const CONTENTSIZE_UNKNOWN: u64 = _;
```

### Constant `MAGIC_DICTIONARY`

```rust
pub const MAGIC_DICTIONARY: u32 = zstd_sys::ZSTD_MAGIC_DICTIONARY;
```

### Constant `MAGICNUMBER`

```rust
pub const MAGICNUMBER: u32 = zstd_sys::ZSTD_MAGICNUMBER;
```

### Constant `MAGIC_SKIPPABLE_MASK`

```rust
pub const MAGIC_SKIPPABLE_MASK: u32 = zstd_sys::ZSTD_MAGIC_SKIPPABLE_MASK;
```

### Constant `MAGIC_SKIPPABLE_START`

```rust
pub const MAGIC_SKIPPABLE_START: u32 = zstd_sys::ZSTD_MAGIC_SKIPPABLE_START;
```

### Constant `VERSION_MAJOR`

```rust
pub const VERSION_MAJOR: u32 = zstd_sys::ZSTD_VERSION_MAJOR;
```

### Constant `VERSION_MINOR`

```rust
pub const VERSION_MINOR: u32 = zstd_sys::ZSTD_VERSION_MINOR;
```

### Constant `VERSION_NUMBER`

```rust
pub const VERSION_NUMBER: u32 = zstd_sys::ZSTD_VERSION_NUMBER;
```

### Constant `VERSION_RELEASE`

```rust
pub const VERSION_RELEASE: u32 = zstd_sys::ZSTD_VERSION_RELEASE;
```

## Re-exports

### Re-export `zstd_sys`

```rust
pub use zstd_sys;
```

### Re-export `ZSTD_strategy`

How to compress data.

```rust
pub use zstd_sys::ZSTD_strategy as Strategy;
```

