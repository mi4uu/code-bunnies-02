# Crate Documentation

**Version:** 1.1.1

**Format Version:** 43

# Module `flate2`

A DEFLATE-based stream compression/decompression library

This library provides support for compression and decompression of
DEFLATE-based streams:

* the DEFLATE format itself
* the zlib format
* gzip

These three formats are all closely related and largely only differ in their
headers/footers. This crate has three types in each submodule for dealing
with these three formats.

# Implementation

In addition to supporting three formats, this crate supports several different
backends, controlled through this crate's features:

* `default`, or `rust_backend` - this implementation uses the `miniz_oxide`
  crate which is a port of `miniz.c` to Rust. This feature does not
  require a C compiler, and only uses safe Rust code.

* `zlib-rs` - this implementation utilizes the `zlib-rs` crate, a Rust rewrite of zlib.
  This backend is the fastest, at the cost of some `unsafe` Rust code.

Several backends implemented in C are also available.
These are useful in case you are already using a specific C implementation
and need the result of compression to be bit-identical.
See the crate's README for details on the available C backends.

The `zlib-rs` backend typically outperforms all the C implementations.

# Organization

This crate consists mainly of three modules, [`read`], [`write`], and
[`bufread`]. Each module contains a number of types used to encode and
decode various streams of data.

All types in the [`write`] module work on instances of [`Write`][write],
whereas all types in the [`read`] module work on instances of
[`Read`][read] and [`bufread`] works with [`BufRead`][bufread]. If you
are decoding directly from a `&[u8]`, use the [`bufread`] types.

```
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;

# fn main() { let _ = run(); }
# fn run() -> io::Result<()> {
let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
encoder.write_all(b"Example")?;
# Ok(())
# }
```


Other various types are provided at the top-level of the crate for
management and dealing with encoders/decoders. Also note that types which
operate over a specific trait often implement the mirroring trait as well.
For example a `flate2::read::DeflateDecoder<T>` *also* implements the
`Write` trait if `T: Write`. That is, the "dual trait" is forwarded directly
to the underlying object if available.

# About multi-member Gzip files

While most `gzip` files one encounters will have a single *member* that can be read
with the [`GzDecoder`], there may be some files which have multiple members.

A [`GzDecoder`] will only read the first member of gzip data, which may unexpectedly
provide partial results when a multi-member gzip file is encountered. `GzDecoder` is appropriate
for data that is designed to be read as single members from a multi-member file. `bufread::GzDecoder`
and `write::GzDecoder` also allow non-gzip data following gzip data to be handled.

The [`MultiGzDecoder`] on the other hand will decode all members of a `gzip` file
into one consecutive stream of bytes, which hides the underlying *members* entirely.
If a file contains non-gzip data after the gzip data, MultiGzDecoder will
emit an error after decoding the gzip data. This behavior matches the `gzip`,
`gunzip`, and `zcat` command line tools.

[`read`]: read/index.html
[`bufread`]: bufread/index.html
[`write`]: write/index.html
[read]: https://doc.rust-lang.org/std/io/trait.Read.html
[write]: https://doc.rust-lang.org/std/io/trait.Write.html
[bufread]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`GzDecoder`]: read/struct.GzDecoder.html
[`MultiGzDecoder`]: read/struct.MultiGzDecoder.html

## Modules

## Module `read`

Types which operate over [`Read`] streams, both encoders and decoders for
various formats.

Note that the `read` decoder types may read past the end of the compressed
data while decoding. If the caller requires subsequent reads to start
immediately following the compressed data  wrap the `Read` type in a
[`BufReader`] and use the `BufReader` with the equivalent decoder from the
`bufread` module and also for the subsequent reads.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html

```rust
pub mod read { /* ... */ }
```

### Re-exports

#### Re-export `DeflateDecoder`

```rust
pub use crate::deflate::read::DeflateDecoder;
```

#### Re-export `DeflateEncoder`

```rust
pub use crate::deflate::read::DeflateEncoder;
```

#### Re-export `GzDecoder`

```rust
pub use crate::gz::read::GzDecoder;
```

#### Re-export `GzEncoder`

```rust
pub use crate::gz::read::GzEncoder;
```

#### Re-export `MultiGzDecoder`

```rust
pub use crate::gz::read::MultiGzDecoder;
```

#### Re-export `ZlibDecoder`

```rust
pub use crate::zlib::read::ZlibDecoder;
```

#### Re-export `ZlibEncoder`

```rust
pub use crate::zlib::read::ZlibEncoder;
```

## Module `write`

Types which operate over [`Write`] streams, both encoders and decoders for
various formats.

[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

```rust
pub mod write { /* ... */ }
```

### Re-exports

#### Re-export `DeflateDecoder`

```rust
pub use crate::deflate::write::DeflateDecoder;
```

#### Re-export `DeflateEncoder`

```rust
pub use crate::deflate::write::DeflateEncoder;
```

#### Re-export `GzDecoder`

```rust
pub use crate::gz::write::GzDecoder;
```

#### Re-export `GzEncoder`

```rust
pub use crate::gz::write::GzEncoder;
```

#### Re-export `MultiGzDecoder`

```rust
pub use crate::gz::write::MultiGzDecoder;
```

#### Re-export `ZlibDecoder`

```rust
pub use crate::zlib::write::ZlibDecoder;
```

#### Re-export `ZlibEncoder`

```rust
pub use crate::zlib::write::ZlibEncoder;
```

## Module `bufread`

Types which operate over [`BufRead`] streams, both encoders and decoders for
various formats.

[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html

```rust
pub mod bufread { /* ... */ }
```

### Re-exports

#### Re-export `DeflateDecoder`

```rust
pub use crate::deflate::bufread::DeflateDecoder;
```

#### Re-export `DeflateEncoder`

```rust
pub use crate::deflate::bufread::DeflateEncoder;
```

#### Re-export `GzDecoder`

```rust
pub use crate::gz::bufread::GzDecoder;
```

#### Re-export `GzEncoder`

```rust
pub use crate::gz::bufread::GzEncoder;
```

#### Re-export `MultiGzDecoder`

```rust
pub use crate::gz::bufread::MultiGzDecoder;
```

#### Re-export `ZlibDecoder`

```rust
pub use crate::zlib::bufread::ZlibDecoder;
```

#### Re-export `ZlibEncoder`

```rust
pub use crate::zlib::bufread::ZlibEncoder;
```

## Types

### Struct `Compression`

When compressing data, the compression level can be specified by a value in
this struct.

```rust
pub struct Compression(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(level: u32) -> Compression { /* ... */ }
  ```
  Creates a new description of the compression level with an explicitly

- ```rust
  pub const fn none() -> Compression { /* ... */ }
  ```
  No compression is to be performed, this may actually inflate data

- ```rust
  pub const fn fast() -> Compression { /* ... */ }
  ```
  Optimize for the best speed of encoding.

- ```rust
  pub const fn best() -> Compression { /* ... */ }
  ```
  Optimize for the size of data being encoded.

- ```rust
  pub fn level(self: &Self) -> u32 { /* ... */ }
  ```
  Returns an integer representing the compression level, typically on a

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
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

- **Copy**
- **Default**
  - ```rust
    fn default() -> Compression { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Compression) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Compression { /* ... */ }
    ```

- **Eq**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Re-exports

### Re-export `Crc`

```rust
pub use crate::crc::Crc;
```

### Re-export `CrcReader`

```rust
pub use crate::crc::CrcReader;
```

### Re-export `CrcWriter`

```rust
pub use crate::crc::CrcWriter;
```

### Re-export `GzBuilder`

```rust
pub use crate::gz::GzBuilder;
```

### Re-export `GzHeader`

```rust
pub use crate::gz::GzHeader;
```

### Re-export `Compress`

```rust
pub use crate::mem::Compress;
```

### Re-export `CompressError`

```rust
pub use crate::mem::CompressError;
```

### Re-export `Decompress`

```rust
pub use crate::mem::Decompress;
```

### Re-export `DecompressError`

```rust
pub use crate::mem::DecompressError;
```

### Re-export `Status`

```rust
pub use crate::mem::Status;
```

### Re-export `FlushCompress`

```rust
pub use crate::mem::FlushCompress;
```

### Re-export `FlushDecompress`

```rust
pub use crate::mem::FlushDecompress;
```

