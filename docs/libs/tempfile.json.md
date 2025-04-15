# Crate Documentation

**Version:** 3.19.1

**Format Version:** 43

# Module `tempfile`

Temporary files and directories.

- Use the [`tempfile()`] function for temporary files
- Use the [`tempdir()`] function for temporary directories.

# Design

This crate provides several approaches to creating temporary files and directories.
[`tempfile()`] relies on the OS to remove the temporary file once the last handle is closed.
[`TempDir`] and [`NamedTempFile`] both rely on Rust destructors for cleanup.

When choosing between the temporary file variants, prefer `tempfile`
unless you either need to know the file's path or to be able to persist it.

## Resource Leaking

`tempfile` will (almost) never fail to cleanup temporary resources. However `TempDir` and
`NamedTempFile` will fail if their destructors don't run. This is because `tempfile` relies on
the OS to cleanup the underlying file, while `TempDir` and `NamedTempFile` rely on rust
destructors to do so. Destructors may fail to run if the process exits through an unhandled
signal interrupt (like `SIGINT`), or if the instance is declared statically (like with
[`lazy_static`]), among other possible reasons.

## Unexpected File Deletion

Most operating systems periodically clean up temporary files that haven't been accessed recently
(often on the order of multiple days). This issue does not affect unnamed temporary files but
can invalidate the paths associated with named temporary files on Unix-like systems because the
temporary file can be unlinked from the filesystem while still open and in-use. See the
[temporary file cleaner](#temporary-file-cleaners) section for more security implications.

## Security

This section discusses security issues relevant to Unix-like operating systems that use shared
temporary directories by default. Importantly, it's not relevant for Windows or macOS as both
operating systems use private per-user temporary directories by default.

Applications can mitigate the issues described below by using [`env::override_temp_dir`] to
change the default temporary directory but should do so if and only if default the temporary
directory ([`env::temp_dir`]) is unsuitable (is world readable, world writable, managed by a
temporary file cleaner, etc.).

### Temporary File Cleaners

In the presence of pathological temporary file cleaner, relying on file paths is unsafe because
a temporary file cleaner could delete the temporary file which an attacker could then replace.

This isn't an issue for [`tempfile`] as it doesn't rely on file paths. However, [`NamedTempFile`]
and temporary directories _do_ rely on file paths for _some_ operations. See the security
documentation on the [`NamedTempFile`] and the [`TempDir`] types for more information.

Mitigation:

- This is rarely an issue for short-lived files as temporary file cleaners usually only remove
  temporary files that haven't been modified or accessed within many (10-30) days.
- Very long lived temporary files should be placed in directories not managed by temporary file
  cleaners.

### Access Permissions

Temporary _files_ created with this library are private by default on all operating systems.
However, temporary _directories_ are created with the default permissions and will therefore be
world-readable by default unless the user has changed their umask and/or default temporary
directory.

### Denial of Service

If the file-name randomness ([`Builder::rand_bytes`]) is too small and/or this crate is built
without the `getrandom` feature, it may be possible for an attacker to predict the random file
names chosen by this library, preventing temporary file creation by creating temporary files
with these predicted file names. By default, this library mitigates this denial of service
attack by:

1. Defaulting to 6 random characters per temporary file forcing an attacker to create billions
   of files before random collisions are expected (at which point you probably have larger
   problems).
2. Re-seeding the random filename generator from system randomness after 3 failed attempts to
   create temporary a file (when the `getrandom` feature is enabled as it is by default on all
   major platforms).

## Early drop pitfall

Because `TempDir` and `NamedTempFile` rely on their destructors for cleanup, this can lead
to an unexpected early removal of the directory/file, usually when working with APIs which are
generic over `AsRef<Path>`. Consider the following example:

```no_run
use tempfile::tempdir;
use std::process::Command;

// Create a directory inside of `env::temp_dir()`.
let temp_dir = tempdir()?;

// Spawn the `touch` command inside the temporary directory and collect the exit status
// Note that `temp_dir` is **not** moved into `current_dir`, but passed as a reference
let exit_status = Command::new("touch").arg("tmp").current_dir(&temp_dir).status()?;
assert!(exit_status.success());

# Ok::<(), std::io::Error>(())
```

This works because a reference to `temp_dir` is passed to `current_dir`, resulting in the
destructor of `temp_dir` being run after the `Command` has finished execution. Moving the
`TempDir` into the `current_dir` call would result in the `TempDir` being converted into
an internal representation, with the original value being dropped and the directory thus
being deleted, before the command can be executed.

The `touch` command would fail with an `No such file or directory` error.

## Examples

Create a temporary file and write some data into it:

```
use tempfile::tempfile;
use std::io::Write;

// Create a file inside of `env::temp_dir()`.
let mut file = tempfile()?;

writeln!(file, "Brian was here. Briefly.")?;
# Ok::<(), std::io::Error>(())
```

Create a named temporary file and open an independent file handle:

```
use tempfile::NamedTempFile;
use std::io::{Write, Read};

let text = "Brian was here. Briefly.";

// Create a file inside of `env::temp_dir()`.
let mut file1 = NamedTempFile::new()?;

// Re-open it.
let mut file2 = file1.reopen()?;

// Write some test data to the first handle.
file1.write_all(text.as_bytes())?;

// Read the test data using the second handle.
let mut buf = String::new();
file2.read_to_string(&mut buf)?;
assert_eq!(buf, text);
# Ok::<(), std::io::Error>(())
```

Create a temporary directory and add a file to it:

```
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

// Create a directory inside of `env::temp_dir()`.
let dir = tempdir()?;

let file_path = dir.path().join("my-temporary-note.txt");
let mut file = File::create(file_path)?;
writeln!(file, "Brian was here. Briefly.")?;

// By closing the `TempDir` explicitly, we can check that it has
// been deleted successfully. If we don't close it explicitly,
// the directory will still be deleted when `dir` goes out
// of scope, but we won't know whether deleting the directory
// succeeded.
drop(file);
dir.close()?;
# Ok::<(), std::io::Error>(())
```

[`tempfile()`]: fn.tempfile.html
[`tempdir()`]: fn.tempdir.html
[`TempDir`]: struct.TempDir.html
[`NamedTempFile`]: struct.NamedTempFile.html
[`lazy_static`]: https://github.com/rust-lang-nursery/lazy-static.rs/issues/62

## Modules

## Module `env`

```rust
pub mod env { /* ... */ }
```

### Functions

#### Function `override_temp_dir`

Override the default temporary directory (defaults to [`std::env::temp_dir`]). This function
changes the _global_ default temporary directory for the entire program and should not be called
except in exceptional cases where it's not configured correctly by the platform. Applications
should first check if the path returned by [`env::temp_dir`] is acceptable.

Only the first call to this function will succeed. All further calls will fail with `Err(path)`
where `path` is previously set default temporary directory override.

**NOTE:** This function does not check if the specified directory exists and/or is writable.

```rust
pub fn override_temp_dir(path: &std::path::Path) -> Result<(), std::path::PathBuf> { /* ... */ }
```

#### Function `temp_dir`

Returns the default temporary directory, used for both temporary directories and files if no
directory is explicitly specified.

This function simply delegates to [`std::env::temp_dir`] unless the default temporary directory
has been override by a call to [`override_temp_dir`].

**NOTE:** This function does check if the returned directory exists and/or is writable.

```rust
pub fn temp_dir() -> std::path::PathBuf { /* ... */ }
```

## Types

### Struct `Builder`

Create a new temporary file or directory with custom options.

```rust
pub struct Builder<''a, ''b> {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new `Builder`.

- ```rust
  pub fn prefix<S: AsRef<OsStr> + ?Sized>(self: &mut Self, prefix: &''a S) -> &mut Self { /* ... */ }
  ```
  Set a custom filename prefix.

- ```rust
  pub fn suffix<S: AsRef<OsStr> + ?Sized>(self: &mut Self, suffix: &''b S) -> &mut Self { /* ... */ }
  ```
  Set a custom filename suffix.

- ```rust
  pub fn rand_bytes(self: &mut Self, rand: usize) -> &mut Self { /* ... */ }
  ```
  Set the number of random bytes.

- ```rust
  pub fn append(self: &mut Self, append: bool) -> &mut Self { /* ... */ }
  ```
  Set the file to be opened in append mode.

- ```rust
  pub fn permissions(self: &mut Self, permissions: std::fs::Permissions) -> &mut Self { /* ... */ }
  ```
  The permissions to create the tempfile or [tempdir](Self::tempdir) with.

- ```rust
  pub fn keep(self: &mut Self, keep: bool) -> &mut Self { /* ... */ }
  ```
  Set the file/folder to be kept even when the [`NamedTempFile`]/[`TempDir`] goes out of

- ```rust
  pub fn tempfile(self: &Self) -> io::Result<NamedTempFile> { /* ... */ }
  ```
  Create the named temporary file.

- ```rust
  pub fn tempfile_in<P: AsRef<Path>>(self: &Self, dir: P) -> io::Result<NamedTempFile> { /* ... */ }
  ```
  Create the named temporary file in the specified directory.

- ```rust
  pub fn tempdir(self: &Self) -> io::Result<TempDir> { /* ... */ }
  ```
  Attempts to make a temporary directory inside of [`env::temp_dir()`] whose

- ```rust
  pub fn tempdir_in<P: AsRef<Path>>(self: &Self, dir: P) -> io::Result<TempDir> { /* ... */ }
  ```
  Attempts to make a temporary directory inside of `dir`.

- ```rust
  pub fn make<F, R>(self: &Self, f: F) -> io::Result<NamedTempFile<R>>
where
    F: FnMut(&Path) -> io::Result<R> { /* ... */ }
  ```
  Attempts to create a temporary file (or file-like object) using the

- ```rust
  pub fn make_in<F, R, P>(self: &Self, dir: P, f: F) -> io::Result<NamedTempFile<R>>
where
    F: FnMut(&Path) -> io::Result<R>,
    P: AsRef<Path> { /* ... */ }
  ```
  This is the same as [`Builder::make`], except `dir` is used as the base

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Builder<''a, ''b>) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder<''a, ''b> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Re-exports

### Re-export `tempdir`

```rust
pub use crate::dir::tempdir;
```

### Re-export `tempdir_in`

```rust
pub use crate::dir::tempdir_in;
```

### Re-export `TempDir`

```rust
pub use crate::dir::TempDir;
```

### Re-export `tempfile`

```rust
pub use crate::file::tempfile;
```

### Re-export `tempfile_in`

```rust
pub use crate::file::tempfile_in;
```

### Re-export `NamedTempFile`

```rust
pub use crate::file::NamedTempFile;
```

### Re-export `PathPersistError`

```rust
pub use crate::file::PathPersistError;
```

### Re-export `PersistError`

```rust
pub use crate::file::PersistError;
```

### Re-export `TempPath`

```rust
pub use crate::file::TempPath;
```

### Re-export `spooled_tempfile`

```rust
pub use crate::spooled::spooled_tempfile;
```

### Re-export `SpooledData`

```rust
pub use crate::spooled::SpooledData;
```

### Re-export `SpooledTempFile`

```rust
pub use crate::spooled::SpooledTempFile;
```

