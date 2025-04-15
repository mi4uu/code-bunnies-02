# Crate Documentation

**Version:** 0.3.7

**Format Version:** 43

# Module `tempdir`

Temporary directories of files.

The [`TempDir`] type creates a directory on the file system that
is deleted once it goes out of scope. At construction, the
`TempDir` creates a new directory with a randomly generated name
and a prefix of your choosing.

[`TempDir`]: struct.TempDir.html
[`std::env::temp_dir()`]: https://doc.rust-lang.org/std/env/fn.temp_dir.html

# Examples

```
extern crate tempdir;

use std::fs::File;
use std::io::{self, Write};
use tempdir::TempDir;

fn main() {
    if let Err(_) = run() {
        ::std::process::exit(1);
    }
}

fn run() -> Result<(), io::Error> {
    // Create a directory inside of `std::env::temp_dir()`, named with
    // the prefix "example".
    let tmp_dir = TempDir::new("example")?;
    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = File::create(file_path)?;
    writeln!(tmp_file, "Brian was here. Briefly.")?;

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `tmp_dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    drop(tmp_file);
    tmp_dir.close()?;
    Ok(())
}
```

## Types

### Struct `TempDir`

A directory in the filesystem that is automatically deleted when
it goes out of scope.

The [`TempDir`] type creates a directory on the file system that
is deleted once it goes out of scope. At construction, the
`TempDir` creates a new directory with a randomly generated name,
and with a prefix of your choosing.

The default constructor, [`TempDir::new`], creates directories in
the location returned by [`std::env::temp_dir()`], but `TempDir`
can be configured to manage a temporary directory in any location
by constructing with [`TempDir::new_in`].

After creating a `TempDir`, work with the file system by doing
standard [`std::fs`] file system operations on its [`Path`],
which can be retrieved with [`TempDir::path`]. Once the `TempDir`
value is dropped, the directory at the path will be deleted, along
with any files and directories it contains. It is your responsibility
to ensure that no further file system operations are attempted
inside the temporary directory once it has been deleted.

Various platform-specific conditions may cause `TempDir` to fail
to delete the underlying directory. It's important to ensure that
handles (like [`File`] and [`ReadDir`]) to files inside the
directory are dropped before the `TempDir` goes out of scope. The
`TempDir` destructor will silently ignore any errors in deleting
the directory; to instead handle errors call [`TempDir::close`].

Note that if the program exits before the `TempDir` destructor is
run, such as via [`std::process::exit`], by segfaulting, or by
receiving a signal like `SIGINT`, then the temporary directory
will not be deleted.
 
[`File`]: http://doc.rust-lang.org/std/fs/struct.File.html
[`Path`]: http://doc.rust-lang.org/std/path/struct.Path.html
[`ReadDir`]: http://doc.rust-lang.org/std/fs/struct.ReadDir.html
[`TempDir::close`]: struct.TempDir.html#method.close
[`TempDir::new`]: struct.TempDir.html#method.new
[`TempDir::new_in`]: struct.TempDir.html#method.new_in
[`TempDir::path`]: struct.TempDir.html#method.path
[`TempDir`]: struct.TempDir.html
[`std::env::temp_dir()`]: https://doc.rust-lang.org/std/env/fn.temp_dir.html
[`std::fs`]: http://doc.rust-lang.org/std/fs/index.html
[`std::process::exit`]: http://doc.rust-lang.org/std/process/fn.exit.html

```rust
pub struct TempDir {
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
  pub fn new(prefix: &str) -> io::Result<TempDir> { /* ... */ }
  ```
  Attempts to make a temporary directory inside of `env::temp_dir()` whose

- ```rust
  pub fn new_in<P: AsRef<Path>>(tmpdir: P, prefix: &str) -> io::Result<TempDir> { /* ... */ }
  ```
  Attempts to make a temporary directory inside of `tmpdir`

- ```rust
  pub fn path(self: &Self) -> &path::Path { /* ... */ }
  ```
  Accesses the [`Path`] to the temporary directory.

- ```rust
  pub fn into_path(self: Self) -> PathBuf { /* ... */ }
  ```
  Unwraps the [`Path`] contained in the `TempDir` and

- ```rust
  pub fn close(self: Self) -> io::Result<()> { /* ... */ }
  ```
  Closes and removes the temporary directory, returing a `Result`.

##### Trait Implementations

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
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Path { /* ... */ }
    ```

