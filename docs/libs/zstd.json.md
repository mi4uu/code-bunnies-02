# Crate Documentation

**Version:** 0.13.3

**Format Version:** 43

# Module `zstd`

Rust binding to the [zstd library][zstd].

This crate provides:

* An [encoder](stream/write/struct.Encoder.html) to compress data using zstd
  and send the output to another write.
* A [decoder](stream/read/struct.Decoder.html) to read input data from a `Read`
  and decompress it.
* Convenient functions for common tasks.

# Example

```no_run
use std::io;

// Uncompress input and print the result.
zstd::stream::copy_decode(io::stdin(), io::stdout()).unwrap();
```

[zstd]: https://github.com/facebook/zstd

## Modules

## Module `bulk`

Compress and decompress data in bulk.

These methods process all the input data at once.
It is therefore best used with relatively small blocks
(like small network packets).

```rust
pub mod bulk { /* ... */ }
```

### Functions

#### Function `compress_to_buffer`

Compresses a single block of data to the given destination buffer.

Returns the number of bytes written, or an error if something happened
(for instance if the destination buffer was too small).

A level of `0` uses zstd's default (currently `3`).

```rust
pub fn compress_to_buffer(source: &[u8], destination: &mut [u8], level: i32) -> io::Result<usize> { /* ... */ }
```

#### Function `compress`

Compresses a block of data and returns the compressed result.

A level of `0` uses zstd's default (currently `3`).

```rust
pub fn compress(data: &[u8], level: i32) -> io::Result<Vec<u8>> { /* ... */ }
```

#### Function `decompress_to_buffer`

Deompress a single block of data to the given destination buffer.

Returns the number of bytes written, or an error if something happened
(for instance if the destination buffer was too small).

```rust
pub fn decompress_to_buffer(source: &[u8], destination: &mut [u8]) -> io::Result<usize> { /* ... */ }
```

#### Function `decompress`

Decompresses a block of data and returns the decompressed result.

The decompressed data should be at most `capacity` bytes,
or an error will be returned.

```rust
pub fn decompress(data: &[u8], capacity: usize) -> io::Result<Vec<u8>> { /* ... */ }
```

### Re-exports

#### Re-export `Compressor`

```rust
pub use self::compressor::Compressor;
```

#### Re-export `Decompressor`

```rust
pub use self::decompressor::Decompressor;
```

## Module `dict`

Train a dictionary from various sources.

A dictionary can help improve the compression of small files.
The dictionary must be present during decompression,
but can be shared across multiple "similar" files.

Creating a dictionary using the `zstd` C library,
using the `zstd` command-line interface, using this library,
or using the `train` binary provided, should give the same result,
and are therefore completely compatible.

To use, see [`Encoder::with_dictionary`] or [`Decoder::with_dictionary`].

[`Encoder::with_dictionary`]: ../struct.Encoder.html#method.with_dictionary
[`Decoder::with_dictionary`]: ../struct.Decoder.html#method.with_dictionary

```rust
pub mod dict { /* ... */ }
```

### Types

#### Struct `EncoderDictionary`

Prepared dictionary for compression

A dictionary can include its own copy of the data (if it is `'static`), or it can merely point
to a separate buffer (if it has another lifetime).

```rust
pub struct EncoderDictionary<''a> {
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
  pub fn copy(dictionary: &[u8], level: i32) -> Self { /* ... */ }
  ```
  Creates a prepared dictionary for compression.

- ```rust
  pub fn as_cdict(self: &Self) -> &CDict<''a> { /* ... */ }
  ```
  Returns reference to `CDict` inner object

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `DecoderDictionary`

Prepared dictionary for decompression

```rust
pub struct DecoderDictionary<''a> {
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
  pub fn copy(dictionary: &[u8]) -> Self { /* ... */ }
  ```
  Create a prepared dictionary for decompression.

- ```rust
  pub fn as_ddict(self: &Self) -> &DDict<''a> { /* ... */ }
  ```
  Returns reference to `DDict` inner object

###### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `from_continuous`

**Attributes:**

- `#[<cfg>(feature = "zdict_builder")]`
- `#[<cfg_attr>(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]`

Train a dictionary from a big continuous chunk of data, with all samples
contiguous in memory.

This is the most efficient way to train a dictionary,
since this is directly fed into `zstd`.

* `sample_data` is the concatenation of all sample data.
* `sample_sizes` is the size of each sample in `sample_data`.
    The sum of all `sample_sizes` should equal the length of `sample_data`.
* `max_size` is the maximum size of the dictionary to generate.

The result is the dictionary data. You can, for example, feed it to [`CDict::create`].

```rust
pub fn from_continuous(sample_data: &[u8], sample_sizes: &[usize], max_size: usize) -> io::Result<Vec<u8>> { /* ... */ }
```

#### Function `from_samples`

**Attributes:**

- `#[<cfg>(feature = "zdict_builder")]`
- `#[<cfg_attr>(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]`

Train a dictionary from multiple samples.

The samples will internally be copied to a single continuous buffer,
so make sure you have enough memory available.

If you need to stretch your system's limits,
[`from_continuous`] directly uses the given slice.

[`from_continuous`]: ./fn.from_continuous.html

* `samples` is a list of individual samples to train on.
* `max_size` is the maximum size of the dictionary to generate.

The result is the dictionary data. You can, for example, feed it to [`CDict::create`].

```rust
pub fn from_samples<S: AsRef<[u8]>>(samples: &[S], max_size: usize) -> io::Result<Vec<u8>> { /* ... */ }
```

#### Function `from_sample_iterator`

**Attributes:**

- `#[<cfg>(feature = "zdict_builder")]`
- `#[<cfg_attr>(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]`

Train a dictionary from multiple samples.

Unlike [`from_samples`], this does not require having a list of all samples.
It also allows running into an error when iterating through the samples.

They will still be copied to a continuous array and fed to [`from_continuous`].

* `samples` is an iterator of individual samples to train on.
* `max_size` is the maximum size of the dictionary to generate.

The result is the dictionary data. You can, for example, feed it to [`CDict::create`].

# Examples

```rust,no_run
// Train from a couple of json files.
let dict_buffer = zstd::dict::from_sample_iterator(
    ["file_a.json", "file_b.json"]
        .into_iter()
        .map(|filename| std::fs::File::open(filename)),
    10_000,  // 10kB dictionary
).unwrap();
```

```rust,no_run
use std::io::BufRead as _;
// Treat each line from stdin as a separate sample.
let dict_buffer = zstd::dict::from_sample_iterator(
    std::io::stdin().lock().lines().map(|line: std::io::Result<String>| {
        // Transform each line into a `Cursor<Vec<u8>>` so they implement Read.
        line.map(String::into_bytes)
            .map(std::io::Cursor::new)
    }),
    10_000,  // 10kB dictionary
).unwrap();
```

```rust
pub fn from_sample_iterator<I, R>(samples: I, max_size: usize) -> io::Result<Vec<u8>>
where
    I: IntoIterator<Item = io::Result<R>>,
    R: Read { /* ... */ }
```

#### Function `from_files`

**Attributes:**

- `#[<cfg>(feature = "zdict_builder")]`
- `#[<cfg_attr>(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]`

Train a dict from a list of files.

* `filenames` is an iterator of files to load. Each file will be treated as an individual
    sample.
* `max_size` is the maximum size of the dictionary to generate.

The result is the dictionary data. You can, for example, feed it to [`CDict::create`].

```rust
pub fn from_files<I, P>(filenames: I, max_size: usize) -> io::Result<Vec<u8>>
where
    P: AsRef<std::path::Path>,
    I: IntoIterator<Item = P> { /* ... */ }
```

### Re-exports

#### Re-export `CDict`

```rust
pub use zstd_safe::CDict;
```

#### Re-export `DDict`

```rust
pub use zstd_safe::DDict;
```

## Module `stream`

**Attributes:**

- `#[macro_use]`

Compress and decompress Zstd streams.

Zstd streams are the main way to compress and decompress data.
They are compatible with the `zstd` command-line tool.

This module provides both `Read` and `Write` interfaces to compressing and
decompressing.

```rust
pub mod stream { /* ... */ }
```

### Modules

## Module `read`

Implement pull-based [`Read`] trait for both compressing and decompressing.

```rust
pub mod read { /* ... */ }
```

### Types

#### Struct `Decoder`

A decoder that decompress input data from another `Read`.

This allows to read a stream of compressed data
(good for files or heavy network stream).

```rust
pub struct Decoder<''a, R> {
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
  pub fn new(reader: R) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new decoder.

- ```rust
  pub fn with_buffer(reader: R) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new decoder around a `BufRead`.

- ```rust
  pub fn with_dictionary(reader: R, dictionary: &[u8]) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new decoder, using an existing dictionary.

- ```rust
  pub fn with_context(reader: R, context: &''a mut zstd_safe::DCtx<''static>) -> Self { /* ... */ }
  ```
  Creates a new decoder which employs the provided context for deserialization.

- ```rust
  pub fn single_frame(self: Self) -> Self { /* ... */ }
  ```
  Sets this `Decoder` to stop after the first frame.

- ```rust
  pub fn with_prepared_dictionary<''b>(reader: R, dictionary: &DecoderDictionary<''b>) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new decoder, using an existing `DecoderDictionary`.

- ```rust
  pub fn with_ref_prefix<''b>(reader: R, ref_prefix: &''b [u8]) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new decoder, using a ref prefix.

- ```rust
  pub fn recommended_output_size() -> usize { /* ... */ }
  ```
  Recommendation for the size of the output buffer.

- ```rust
  pub fn get_ref(self: &Self) -> &R { /* ... */ }
  ```
  Acquire a reference to the underlying reader.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut R { /* ... */ }
  ```
  Acquire a mutable reference to the underlying reader.

- ```rust
  pub fn finish(self: Self) -> R { /* ... */ }
  ```
  Return the inner `Read`.

- ```rust
  pub fn set_parameter(self: &mut Self, parameter: zstd_safe::DParameter) -> io::Result<()> { /* ... */ }
  ```
  Sets a decompression parameter on the decompression stream.

- ```rust
  pub fn window_log_max(self: &mut Self, log_distance: u32) -> io::Result<()> { /* ... */ }
  ```
  Sets the maximum back-reference distance.

###### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

#### Struct `Encoder`

An encoder that compress input data from another `Read`.

```rust
pub struct Encoder<''a, R> {
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
  pub fn new(reader: R, level: i32) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new encoder.

- ```rust
  pub fn with_buffer(reader: R, level: i32) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new encoder around a `BufRead`.

- ```rust
  pub fn with_dictionary(reader: R, level: i32, dictionary: &[u8]) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new encoder, using an existing dictionary.

- ```rust
  pub fn with_prepared_dictionary<''b>(reader: R, dictionary: &EncoderDictionary<''b>) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new encoder, using an existing `EncoderDictionary`.

- ```rust
  pub fn recommended_output_size() -> usize { /* ... */ }
  ```
  Recommendation for the size of the output buffer.

- ```rust
  pub fn get_ref(self: &Self) -> &R { /* ... */ }
  ```
  Acquire a reference to the underlying reader.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut R { /* ... */ }
  ```
  Acquire a mutable reference to the underlying reader.

- ```rust
  pub fn flush(self: &mut Self, out: &mut [u8]) -> io::Result<usize> { /* ... */ }
  ```
  Flush any internal buffer.

- ```rust
  pub fn finish(self: Self) -> R { /* ... */ }
  ```
  Return the inner `Read`.

- ```rust
  pub fn set_parameter(self: &mut Self, parameter: zstd_safe::CParameter) -> io::Result<()> { /* ... */ }
  ```
  Sets the given zstd compression parameter.

- ```rust
  pub fn set_pledged_src_size(self: &mut Self, size: Option<u64>) -> io::Result<()> { /* ... */ }
  ```
  Sets the expected size of the input.

- ```rust
  pub fn include_checksum(self: &mut Self, include_checksum: bool) -> io::Result<()> { /* ... */ }
  ```
  Controls whether zstd should include a content checksum at the end

- ```rust
  pub fn include_dictid(self: &mut Self, include_dictid: bool) -> io::Result<()> { /* ... */ }
  ```
  Enables or disables storing of the dict id.

- ```rust
  pub fn include_contentsize(self: &mut Self, include_contentsize: bool) -> io::Result<()> { /* ... */ }
  ```
  Enables or disabled storing of the contentsize.

- ```rust
  pub fn long_distance_matching(self: &mut Self, long_distance_matching: bool) -> io::Result<()> { /* ... */ }
  ```
  Enables or disables long-distance matching

- ```rust
  pub fn set_target_cblock_size(self: &mut Self, target_size: Option<u32>) -> io::Result<()> { /* ... */ }
  ```
  Sets the target size for compressed blocks.

- ```rust
  pub fn window_log(self: &mut Self, log_distance: u32) -> io::Result<()> { /* ... */ }
  ```
  Sets the maximum back-reference distance.

###### Trait Implementations

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

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `write`

Implement push-based [`Write`] trait for both compressing and decompressing.

```rust
pub mod write { /* ... */ }
```

### Types

#### Struct `Encoder`

An encoder that compress and forward data to another writer.

This allows to compress a stream of data
(good for files or heavy network stream).

Don't forget to call [`finish()`] before dropping it!

Alternatively, you can call [`auto_finish()`] to use an
[`AutoFinishEncoder`] that will finish on drop.

Note: The zstd library has its own internal input buffer (~128kb).

[`finish()`]: #method.finish
[`auto_finish()`]: #method.auto_finish
[`AutoFinishEncoder`]: AutoFinishEncoder

```rust
pub struct Encoder<''a, W: Write> {
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
  pub fn new(writer: W, level: i32) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new encoder.

- ```rust
  pub fn with_dictionary(writer: W, level: i32, dictionary: &[u8]) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new encoder, using an existing dictionary.

- ```rust
  pub fn with_writer(writer: zio::Writer<W, raw::Encoder<''a>>) -> Self { /* ... */ }
  ```
  Creates a new encoder from a prepared zio writer.

- ```rust
  pub fn with_encoder(writer: W, encoder: raw::Encoder<''a>) -> Self { /* ... */ }
  ```
  Creates a new encoder from the given `Write` and raw encoder.

- ```rust
  pub fn with_context(writer: W, context: &''a mut zstd_safe::CCtx<''static>) -> Self { /* ... */ }
  ```
  Creates an encoder that uses the provided context to compress a stream.

- ```rust
  pub fn with_prepared_dictionary<''b>(writer: W, dictionary: &EncoderDictionary<''b>) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new encoder, using an existing prepared `EncoderDictionary`.

- ```rust
  pub fn with_ref_prefix<''b>(writer: W, level: i32, ref_prefix: &''b [u8]) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new encoder, using a ref prefix

- ```rust
  pub fn auto_finish(self: Self) -> AutoFinishEncoder<''a, W> { /* ... */ }
  ```
  Returns a wrapper around `self` that will finish the stream on drop.

- ```rust
  pub fn on_finish<F: FnMut(io::Result<W>)>(self: Self, f: F) -> AutoFinishEncoder<''a, W, F> { /* ... */ }
  ```
  Returns an encoder that will finish the stream on drop.

- ```rust
  pub fn get_ref(self: &Self) -> &W { /* ... */ }
  ```
  Acquires a reference to the underlying writer.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut W { /* ... */ }
  ```
  Acquires a mutable reference to the underlying writer.

- ```rust
  pub fn finish(self: Self) -> io::Result<W> { /* ... */ }
  ```
  **Required**: Finishes the stream.

- ```rust
  pub fn try_finish(self: Self) -> Result<W, (Self, io::Error)> { /* ... */ }
  ```
  **Required**: Attempts to finish the stream.

- ```rust
  pub fn do_finish(self: &mut Self) -> io::Result<()> { /* ... */ }
  ```
  Attempts to finish the stream.

- ```rust
  pub fn recommended_input_size() -> usize { /* ... */ }
  ```
  Return a recommendation for the size of data to write at once.

- ```rust
  pub fn set_parameter(self: &mut Self, parameter: zstd_safe::CParameter) -> io::Result<()> { /* ... */ }
  ```
  Sets the given zstd compression parameter.

- ```rust
  pub fn set_pledged_src_size(self: &mut Self, size: Option<u64>) -> io::Result<()> { /* ... */ }
  ```
  Sets the expected size of the input.

- ```rust
  pub fn include_checksum(self: &mut Self, include_checksum: bool) -> io::Result<()> { /* ... */ }
  ```
  Controls whether zstd should include a content checksum at the end

- ```rust
  pub fn include_dictid(self: &mut Self, include_dictid: bool) -> io::Result<()> { /* ... */ }
  ```
  Enables or disables storing of the dict id.

- ```rust
  pub fn include_contentsize(self: &mut Self, include_contentsize: bool) -> io::Result<()> { /* ... */ }
  ```
  Enables or disabled storing of the contentsize.

- ```rust
  pub fn long_distance_matching(self: &mut Self, long_distance_matching: bool) -> io::Result<()> { /* ... */ }
  ```
  Enables or disables long-distance matching

- ```rust
  pub fn set_target_cblock_size(self: &mut Self, target_size: Option<u32>) -> io::Result<()> { /* ... */ }
  ```
  Sets the target size for compressed blocks.

- ```rust
  pub fn window_log(self: &mut Self, log_distance: u32) -> io::Result<()> { /* ... */ }
  ```
  Sets the maximum back-reference distance.

###### Trait Implementations

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **UnwindSafe**
- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
#### Struct `Decoder`

A decoder that decompress and forward data to another writer.

Note that you probably want to `flush()` after writing your stream content.
You can use [`auto_flush()`] to automatically flush the writer on drop.

[`auto_flush()`]: Decoder::auto_flush

```rust
pub struct Decoder<''a, W: Write> {
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
  pub fn new(writer: W) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new decoder.

- ```rust
  pub fn with_dictionary(writer: W, dictionary: &[u8]) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new decoder, using an existing dictionary.

- ```rust
  pub fn with_writer(writer: zio::Writer<W, raw::Decoder<''a>>) -> Self { /* ... */ }
  ```
  Creates a new decoder around the given prepared zio writer.

- ```rust
  pub fn with_decoder(writer: W, decoder: raw::Decoder<''a>) -> Self { /* ... */ }
  ```
  Creates a new decoder around the given `Write` and raw decoder.

- ```rust
  pub fn with_prepared_dictionary<''b>(writer: W, dictionary: &DecoderDictionary<''b>) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new decoder, using an existing prepared `DecoderDictionary`.

- ```rust
  pub fn get_ref(self: &Self) -> &W { /* ... */ }
  ```
  Acquires a reference to the underlying writer.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut W { /* ... */ }
  ```
  Acquires a mutable reference to the underlying writer.

- ```rust
  pub fn into_inner(self: Self) -> W { /* ... */ }
  ```
  Returns the inner `Write`.

- ```rust
  pub fn recommended_input_size() -> usize { /* ... */ }
  ```
  Return a recommendation for the size of data to write at once.

- ```rust
  pub fn auto_flush(self: Self) -> AutoFlushDecoder<''a, W> { /* ... */ }
  ```
  Returns a wrapper around `self` that will flush the stream on drop.

- ```rust
  pub fn on_flush<F: FnMut(io::Result<()>)>(self: Self, f: F) -> AutoFlushDecoder<''a, W, F> { /* ... */ }
  ```
  Returns a decoder that will flush the stream on drop.

- ```rust
  pub fn set_parameter(self: &mut Self, parameter: zstd_safe::DParameter) -> io::Result<()> { /* ... */ }
  ```
  Sets a decompression parameter on the decompression stream.

- ```rust
  pub fn window_log_max(self: &mut Self, log_distance: u32) -> io::Result<()> { /* ... */ }
  ```
  Sets the maximum back-reference distance.

###### Trait Implementations

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `AutoFinishEncoder`

A wrapper around an `Encoder<W>` that finishes the stream on drop.

This can be created by the [`auto_finish()`] method on the [`Encoder`].

[`auto_finish()`]: Encoder::auto_finish
[`Encoder`]: Encoder

```rust
pub struct AutoFinishEncoder<''a, W: Write, F: FnMut(io::Result<W>) = Box<dyn Send + FnMut(io::Result<W>)>> {
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
  pub fn get_ref(self: &Self) -> &W { /* ... */ }
  ```
  Acquires a reference to the underlying writer.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut W { /* ... */ }
  ```
  Acquires a mutable reference to the underlying writer.

###### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Freeze**
#### Struct `AutoFlushDecoder`

A wrapper around a `Decoder<W>` that flushes the stream on drop.

This can be created by the [`auto_flush()`] method on the [`Decoder`].

[`auto_flush()`]: Decoder::auto_flush
[`Decoder`]: Decoder

```rust
pub struct AutoFlushDecoder<''a, W: Write, F: FnMut(io::Result<()>) = Box<dyn Send + FnMut(io::Result<()>)>> {
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
  pub fn get_ref(self: &Self) -> &W { /* ... */ }
  ```
  Acquires a reference to the underlying writer.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut W { /* ... */ }
  ```
  Acquires a mutable reference to the underlying writer.

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `zio`

Wrappers around raw operations implementing `std::io::{Read, Write}`.

```rust
pub mod zio { /* ... */ }
```

### Re-exports

#### Re-export `Reader`

```rust
pub use self::reader::Reader;
```

#### Re-export `Writer`

```rust
pub use self::writer::Writer;
```

## Module `raw`

Raw in-memory stream compression/decompression.

This module defines a `Decoder` and an `Encoder` to decode/encode streams
of data using buffers.

They are mostly thin wrappers around `zstd_safe::{DCtx, CCtx}`.

```rust
pub mod raw { /* ... */ }
```

### Types

#### Struct `NoOp`

Dummy operation that just copies its input to the output.

```rust
pub struct NoOp;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Operation**
  - ```rust
    fn run<C: WriteBuf + ?Sized>(self: &mut Self, input: &mut InBuffer<''_>, output: &mut OutBuffer<''_, C>) -> io::Result<usize> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **UnwindSafe**
#### Struct `Status`

Describes the result of an operation.

```rust
pub struct Status {
    pub remaining: usize,
    pub bytes_read: usize,
    pub bytes_written: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `remaining` | `usize` | Number of bytes expected for next input.<br><br>* If `remaining = 0`, then we are at the end of a frame.<br>* If `remaining > 0`, then it's just a hint for how much there is still<br>  to read. |
| `bytes_read` | `usize` | Number of bytes read from the input. |
| `bytes_written` | `usize` | Number of bytes written to the output. |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
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

- **Freeze**
- **Sync**
- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Decoder`

An in-memory decoder for streams of data.

```rust
pub struct Decoder<''a> {
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
  pub fn new() -> io::Result<Self> { /* ... */ }
  ```
  Creates a new decoder.

- ```rust
  pub fn with_dictionary(dictionary: &[u8]) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new decoder initialized with the given dictionary.

- ```rust
  pub fn with_context(context: &''a mut zstd_safe::DCtx<''static>) -> Self { /* ... */ }
  ```
  Creates a new decoder which employs the provided context for deserialization.

- ```rust
  pub fn with_prepared_dictionary<''b>(dictionary: &DecoderDictionary<''b>) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new decoder, using an existing `DecoderDictionary`.

- ```rust
  pub fn with_ref_prefix<''b>(ref_prefix: &''b [u8]) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new decoder, using a ref prefix

- ```rust
  pub fn set_parameter(self: &mut Self, parameter: DParameter) -> io::Result<()> { /* ... */ }
  ```
  Sets a decompression parameter for this decoder.

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Operation**
  - ```rust
    fn run<C: WriteBuf + ?Sized>(self: &mut Self, input: &mut InBuffer<''_>, output: &mut OutBuffer<''_, C>) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn reinit(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn finish<C: WriteBuf + ?Sized>(self: &mut Self, _output: &mut OutBuffer<''_, C>, finished_frame: bool) -> io::Result<usize> { /* ... */ }
    ```

- **Sync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Encoder`

An in-memory encoder for streams of data.

```rust
pub struct Encoder<''a> {
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
  pub fn new(level: i32) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new encoder.

- ```rust
  pub fn with_dictionary(level: i32, dictionary: &[u8]) -> io::Result<Self> { /* ... */ }
  ```
  Creates a new encoder initialized with the given dictionary.

- ```rust
  pub fn with_context(context: &''a mut zstd_safe::CCtx<''static>) -> Self { /* ... */ }
  ```
  Creates a new encoder that uses the provided context for serialization.

- ```rust
  pub fn with_prepared_dictionary<''b>(dictionary: &EncoderDictionary<''b>) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new encoder using an existing `EncoderDictionary`.

- ```rust
  pub fn with_ref_prefix<''b>(level: i32, ref_prefix: &''b [u8]) -> io::Result<Self>
where
    ''b: ''a { /* ... */ }
  ```
  Creates a new encoder initialized with the given ref prefix.

- ```rust
  pub fn set_parameter(self: &mut Self, parameter: CParameter) -> io::Result<()> { /* ... */ }
  ```
  Sets a compression parameter for this encoder.

- ```rust
  pub fn set_pledged_src_size(self: &mut Self, pledged_src_size: Option<u64>) -> io::Result<()> { /* ... */ }
  ```
  Sets the size of the input expected by zstd.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Operation**
  - ```rust
    fn run<C: WriteBuf + ?Sized>(self: &mut Self, input: &mut InBuffer<''_>, output: &mut OutBuffer<''_, C>) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn finish<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>, _finished_frame: bool) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn reinit(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Traits

#### Trait `Operation`

Represents an abstract compression/decompression operation.

This trait covers both `Encoder` and `Decoder`.

```rust
pub trait Operation {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `run`: Performs a single step of this operation.

##### Provided Methods

- ```rust
  fn run_on_buffers(self: &mut Self, input: &[u8], output: &mut [u8]) -> io::Result<Status> { /* ... */ }
  ```
  Performs a single step of this operation.

- ```rust
  fn flush<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>) -> io::Result<usize> { /* ... */ }
  ```
  Flushes any internal buffer, if any.

- ```rust
  fn reinit(self: &mut Self) -> io::Result<()> { /* ... */ }
  ```
  Prepares the operation for a new frame.

- ```rust
  fn finish<C: WriteBuf + ?Sized>(self: &mut Self, output: &mut OutBuffer<''_, C>, finished_frame: bool) -> io::Result<usize> { /* ... */ }
  ```
  Finishes the operation, writing any footer if necessary.

##### Implementations

This trait is implemented for the following types:

- `NoOp`
- `Decoder<''_>`
- `Encoder<''a>` with <''a>

### Re-exports

#### Re-export `CParameter`

```rust
pub use zstd_safe::CParameter;
```

#### Re-export `DParameter`

```rust
pub use zstd_safe::DParameter;
```

#### Re-export `InBuffer`

```rust
pub use zstd_safe::InBuffer;
```

#### Re-export `OutBuffer`

```rust
pub use zstd_safe::OutBuffer;
```

#### Re-export `WriteBuf`

```rust
pub use zstd_safe::WriteBuf;
```

### Re-exports

#### Re-export `copy_decode`

```rust
pub use self::functions::copy_decode;
```

#### Re-export `copy_encode`

```rust
pub use self::functions::copy_encode;
```

#### Re-export `decode_all`

```rust
pub use self::functions::decode_all;
```

#### Re-export `encode_all`

```rust
pub use self::functions::encode_all;
```

#### Re-export `Decoder`

```rust
pub use self::read::Decoder;
```

#### Re-export `AutoFinishEncoder`

```rust
pub use self::write::AutoFinishEncoder;
```

#### Re-export `Encoder`

```rust
pub use self::write::Encoder;
```

## Functions

### Function `compression_level_range`

The accepted range of compression levels.

```rust
pub fn compression_level_range() -> std::ops::RangeInclusive<zstd_safe::CompressionLevel> { /* ... */ }
```

## Re-exports

### Re-export `zstd_safe`

```rust
pub use zstd_safe;
```

### Re-export `CLEVEL_DEFAULT`

Default compression level.

```rust
pub use zstd_safe::CLEVEL_DEFAULT as DEFAULT_COMPRESSION_LEVEL;
```

### Re-export `decode_all`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::stream::decode_all;
```

### Re-export `encode_all`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::stream::encode_all;
```

### Re-export `Decoder`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::stream::Decoder;
```

### Re-export `Encoder`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::stream::Encoder;
```

