# Crate Documentation

**Version:** 0.5.13

**Format Version:** 43

# Module `zip`

An ergonomic API for reading and writing ZIP files.

The current implementation is based on [PKWARE's APPNOTE.TXT v6.3.9](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT)

## Modules

## Module `read`

Types for reading ZIP archives

```rust
pub mod read { /* ... */ }
```

### Types

#### Struct `ZipArchive`

ZIP archive reader

```no_run
use std::io::prelude::*;
fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
        std::io::copy(&mut file, &mut std::io::stdout());
    }

    Ok(())
}
```

```rust
pub struct ZipArchive<R> {
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
  pub fn new(reader: R) -> ZipResult<ZipArchive<R>> { /* ... */ }
  ```
  Read a ZIP archive, collecting the files it contains

- ```rust
  pub fn extract<P: AsRef<Path>>(self: &mut Self, directory: P) -> ZipResult<()> { /* ... */ }
  ```
  Extract a Zip archive into a directory, overwriting files if they

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Number of files contained in this zip.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether this zip archive contains no files

- ```rust
  pub fn offset(self: &Self) -> u64 { /* ... */ }
  ```
  Get the offset from the beginning of the underlying reader that this zip begins at, in bytes.

- ```rust
  pub fn comment(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the comment of the zip archive.

- ```rust
  pub fn file_names(self: &Self) -> impl Iterator<Item = &str> { /* ... */ }
  ```
  Returns an iterator over all the file and directory names in this archive.

- ```rust
  pub fn by_name_decrypt<''a>(self: &''a mut Self, name: &str, password: &[u8]) -> ZipResult<Result<ZipFile<''a>, InvalidPassword>> { /* ... */ }
  ```
  Search for a file entry by name, decrypt with given password

- ```rust
  pub fn by_name<''a>(self: &''a mut Self, name: &str) -> ZipResult<ZipFile<''a>> { /* ... */ }
  ```
  Search for a file entry by name

- ```rust
  pub fn by_index_decrypt<''a>(self: &''a mut Self, file_number: usize, password: &[u8]) -> ZipResult<Result<ZipFile<''a>, InvalidPassword>> { /* ... */ }
  ```
  Get a contained file by index, decrypt with given password

- ```rust
  pub fn by_index<''a>(self: &''a mut Self, file_number: usize) -> ZipResult<ZipFile<''a>> { /* ... */ }
  ```
  Get a contained file by index

- ```rust
  pub fn by_index_raw<''a>(self: &''a mut Self, file_number: usize) -> ZipResult<ZipFile<''a>> { /* ... */ }
  ```
  Get a contained file by index without decompressing it

- ```rust
  pub fn into_inner(self: Self) -> R { /* ... */ }
  ```
  Unwrap and return the inner reader object

###### Trait Implementations

- **Sync**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZipArchive<R> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
#### Struct `ZipFile`

A struct for reading a zip file

```rust
pub struct ZipFile<''a> {
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
  pub fn version_made_by(self: &Self) -> (u8, u8) { /* ... */ }
  ```
  Get the version of the file

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Get the name of the file

- ```rust
  pub fn name_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the name of the file, in the raw (internal) byte representation.

- ```rust
  pub fn sanitized_name(self: &Self) -> ::std::path::PathBuf { /* ... */ }
  ```
  Get the name of the file in a sanitized form. It truncates the name to the first NULL byte,

- ```rust
  pub fn mangled_name(self: &Self) -> ::std::path::PathBuf { /* ... */ }
  ```
  Rewrite the path, ignoring any path components with special meaning.

- ```rust
  pub fn enclosed_name(self: &Self) -> Option<&Path> { /* ... */ }
  ```
  Ensure the file path is safe to use as a [`Path`].

- ```rust
  pub fn comment(self: &Self) -> &str { /* ... */ }
  ```
  Get the comment of the file

- ```rust
  pub fn compression(self: &Self) -> CompressionMethod { /* ... */ }
  ```
  Get the compression method used to store the file

- ```rust
  pub fn compressed_size(self: &Self) -> u64 { /* ... */ }
  ```
  Get the size of the file in the archive

- ```rust
  pub fn size(self: &Self) -> u64 { /* ... */ }
  ```
  Get the size of the file when uncompressed

- ```rust
  pub fn last_modified(self: &Self) -> DateTime { /* ... */ }
  ```
  Get the time the file was last modified

- ```rust
  pub fn is_dir(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the file is actually a directory

- ```rust
  pub fn is_file(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the file is a regular file

- ```rust
  pub fn unix_mode(self: &Self) -> Option<u32> { /* ... */ }
  ```
  Get unix mode for the file

- ```rust
  pub fn crc32(self: &Self) -> u32 { /* ... */ }
  ```
  Get the CRC32 hash of the original file

- ```rust
  pub fn extra_data(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the extra data of the zip header for this file

- ```rust
  pub fn data_start(self: &Self) -> u64 { /* ... */ }
  ```
  Get the starting offset of the data of the compressed file

- ```rust
  pub fn header_start(self: &Self) -> u64 { /* ... */ }
  ```
  Get the starting offset of the zip header for this file

- ```rust
  pub fn central_header_start(self: &Self) -> u64 { /* ... */ }
  ```
  Get the starting offset of the zip header in the central directory for this file

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **ReadBytesExt**
### Functions

#### Function `read_zipfile_from_stream`

Read ZipFile structures from a non-seekable reader.

This is an alternative method to read a zip file. If possible, use the ZipArchive functions
as some information will be missing when reading this manner.

Reads a file header from the start of the stream. Will return `Ok(Some(..))` if a file is
present at the start of the stream. Returns `Ok(None)` if the start of the central directory
is encountered. No more files should be read after this.

The Drop implementation of ZipFile ensures that the reader will be correctly positioned after
the structure is done.

Missing fields are:
* `comment`: set to an empty string
* `data_start`: set to 0
* `external_attributes`: `unix_mode()`: will return None

```rust
pub fn read_zipfile_from_stream<''a, R: io::Read>(reader: &''a mut R) -> crate::result::ZipResult<Option<ZipFile<''_>>> { /* ... */ }
```

## Module `result`

Error types that can be emitted from this library

```rust
pub mod result { /* ... */ }
```

### Types

#### Type Alias `ZipResult`

Generic result type with ZipError as its error variant

```rust
pub type ZipResult<T> = Result<T, ZipError>;
```

#### Struct `InvalidPassword`

**Attributes:**

- `#[error("invalid password for file in archive")]`

The given password is wrong

```rust
pub struct InvalidPassword;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Error**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Enum `ZipError`

Error type for Zip

```rust
pub enum ZipError {
    Io(io::Error),
    InvalidArchive(&''static str),
    UnsupportedArchive(&''static str),
    FileNotFound,
}
```

##### Variants

###### `Io`

An Error caused by I/O

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `io::Error` |  |

###### `InvalidArchive`

This file is probably not a zip archive

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `UnsupportedArchive`

This archive is not supported

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `FileNotFound`

The requested file could not be found in the archive

##### Implementations

###### Methods

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn from(source: io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: ZipError) -> io::Error { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `write`

Types for creating ZIP archives

```rust
pub mod write { /* ... */ }
```

### Types

#### Struct `ZipWriter`

ZIP archive generator

Handles the bookkeeping involved in building an archive, and provides an
API to edit its contents.

```
# fn doit() -> zip::result::ZipResult<()>
# {
# use zip::ZipWriter;
use std::io::Write;
use zip::write::FileOptions;

// We use a buffer here, though you'd normally use a `File`
let mut buf = [0; 65536];
let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf[..]));

let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
zip.start_file("hello_world.txt", options)?;
zip.write(b"Hello, World!")?;

// Apply the changes you've made.
// Dropping the `ZipWriter` will have the same effect, but may silently fail
zip.finish()?;

# Ok(())
# }
# doit().unwrap();
```

```rust
pub struct ZipWriter<W: Write + io::Seek> {
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
  pub fn new_append(readwriter: A) -> ZipResult<ZipWriter<A>> { /* ... */ }
  ```
  Initializes the archive from an existing ZIP archive, making it ready for append.

- ```rust
  pub fn new(inner: W) -> ZipWriter<W> { /* ... */ }
  ```
  Initializes the archive.

- ```rust
  pub fn set_comment<S>(self: &mut Self, comment: S)
where
    S: Into<String> { /* ... */ }
  ```
  Set ZIP archive comment.

- ```rust
  pub fn set_raw_comment(self: &mut Self, comment: Vec<u8>) { /* ... */ }
  ```
  Set ZIP archive comment.

- ```rust
  pub fn start_file<S>(self: &mut Self, name: S, options: FileOptions) -> ZipResult<()>
where
    S: Into<String> { /* ... */ }
  ```
  Create a file in the archive and start writing its' contents.

- ```rust
  pub fn start_file_from_path(self: &mut Self, path: &std::path::Path, options: FileOptions) -> ZipResult<()> { /* ... */ }
  ```
  Starts a file, taking a Path as argument.

- ```rust
  pub fn start_file_aligned<S>(self: &mut Self, name: S, options: FileOptions, align: u16) -> Result<u64, ZipError>
where
    S: Into<String> { /* ... */ }
  ```
  Create an aligned file in the archive and start writing its' contents.

- ```rust
  pub fn start_file_with_extra_data<S>(self: &mut Self, name: S, options: FileOptions) -> ZipResult<u64>
where
    S: Into<String> { /* ... */ }
  ```
  Create a file in the archive and start writing its extra data first.

- ```rust
  pub fn end_local_start_central_extra_data(self: &mut Self) -> ZipResult<u64> { /* ... */ }
  ```
  End local and start central extra data. Requires [`ZipWriter::start_file_with_extra_data`].

- ```rust
  pub fn end_extra_data(self: &mut Self) -> ZipResult<u64> { /* ... */ }
  ```
  End extra data and start file data. Requires [`ZipWriter::start_file_with_extra_data`].

- ```rust
  pub fn raw_copy_file_rename<S>(self: &mut Self, file: ZipFile<''_>, name: S) -> ZipResult<()>
where
    S: Into<String> { /* ... */ }
  ```
  Add a new file using the already compressed data from a ZIP file being read and renames it, this

- ```rust
  pub fn raw_copy_file(self: &mut Self, file: ZipFile<''_>) -> ZipResult<()> { /* ... */ }
  ```
  Add a new file using the already compressed data from a ZIP file being read, this allows faster

- ```rust
  pub fn add_directory<S>(self: &mut Self, name: S, options: FileOptions) -> ZipResult<()>
where
    S: Into<String> { /* ... */ }
  ```
  Add a directory entry.

- ```rust
  pub fn add_directory_from_path(self: &mut Self, path: &std::path::Path, options: FileOptions) -> ZipResult<()> { /* ... */ }
  ```
  Add a directory entry, taking a Path as argument.

- ```rust
  pub fn finish(self: &mut Self) -> ZipResult<W> { /* ... */ }
  ```
  Finish the last file and write all other zip-structures

###### Trait Implementations

- **Sync**
- **WriteBytesExt**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `FileOptions`

Metadata for a file to be written

```rust
pub struct FileOptions {
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
  pub fn default() -> FileOptions { /* ... */ }
  ```
  Construct a new FileOptions object

- ```rust
  pub fn compression_method(self: Self, method: CompressionMethod) -> FileOptions { /* ... */ }
  ```
  Set the compression method for the new file

- ```rust
  pub fn last_modified_time(self: Self, mod_time: DateTime) -> FileOptions { /* ... */ }
  ```
  Set the last modified time

- ```rust
  pub fn unix_permissions(self: Self, mode: u32) -> FileOptions { /* ... */ }
  ```
  Set the permissions for the new file.

- ```rust
  pub fn large_file(self: Self, large: bool) -> FileOptions { /* ... */ }
  ```
  Set whether the new file's compressed and uncompressed size is less than 4 GiB.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FileOptions { /* ... */ }
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

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **UnwindSafe**
- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Re-exports

### Re-export `CompressionMethod`

```rust
pub use crate::compression::CompressionMethod;
```

### Re-export `ZipArchive`

```rust
pub use crate::read::ZipArchive;
```

### Re-export `DateTime`

```rust
pub use crate::types::DateTime;
```

### Re-export `ZipWriter`

```rust
pub use crate::write::ZipWriter;
```

