# Crate Documentation

**Version:** 1.0.5

**Format Version:** 43

# Module `rustix`

`rustix` provides efficient memory-safe and [I/O-safe] wrappers to
POSIX-like, Unix-like, Linux, and Winsock syscall-like APIs, with
configurable backends.

With rustix, you can write code like this:

```
# #[cfg(feature = "net")]
# fn read(sock: std::net::TcpStream, buf: &mut [u8]) -> std::io::Result<()> {
# use rustix::net::RecvFlags;
let (nread, _received) = rustix::net::recv(&sock, buf, RecvFlags::PEEK)?;
# let _ = nread;
# Ok(())
# }
```

instead of like this:

```
# #[cfg(feature = "net")]
# fn read(sock: std::net::TcpStream, buf: &mut [u8]) -> std::io::Result<()> {
# #[cfg(unix)]
# use std::os::unix::io::AsRawFd;
# #[cfg(target_os = "wasi")]
# use std::os::wasi::io::AsRawFd;
# #[cfg(windows)]
# use windows_sys::Win32::Networking::WinSock as libc;
# #[cfg(windows)]
# use std::os::windows::io::AsRawSocket;
# const MSG_PEEK: i32 = libc::MSG_PEEK;
let nread = unsafe {
    #[cfg(any(unix, target_os = "wasi"))]
    let raw = sock.as_raw_fd();
    #[cfg(windows)]
    let raw = sock.as_raw_socket();
    match libc::recv(
        raw as _,
        buf.as_mut_ptr().cast(),
        buf.len().try_into().unwrap_or(i32::MAX as _),
        MSG_PEEK,
    ) {
        -1 => return Err(std::io::Error::last_os_error()),
        nread => nread as usize,
    }
};
# let _ = nread;
# Ok(())
# }
```

rustix's APIs perform the following tasks:
 - Error values are translated to [`Result`]s.
 - Buffers are passed as Rust slices.
 - Out-parameters are presented as return values.
 - Path arguments use [`Arg`], so they accept any string type.
 - File descriptors are passed and returned via [`AsFd`] and [`OwnedFd`]
   instead of bare integers, ensuring I/O safety.
 - Constants use `enum`s and [`bitflags`] types, and enable [support for
   externally defined flags].
 - Multiplexed functions (eg. `fcntl`, `ioctl`, etc.) are de-multiplexed.
 - Variadic functions (eg. `openat`, etc.) are presented as non-variadic.
 - Functions that return strings automatically allocate sufficient memory
   and retry the syscall as needed to determine the needed length.
 - Functions and types which need `l` prefixes or `64` suffixes to enable
   large-file support (LFS) are used automatically. File sizes and offsets
   are always presented as `u64` and `i64`.
 - Behaviors that depend on the sizes of C types like `long` are hidden.
 - In some places, more human-friendly and less historical-accident names
   are used (and documentation aliases are used so that the original names
   can still be searched for).
 - Provide y2038 compatibility, on platforms which support this.
 - Correct selected platform bugs, such as behavioral differences when
   running under seccomp.
 - Use `timespec` for timestamps and timeouts instead of `timeval` and
   `c_int` milliseconds.

Things they don't do include:
 - Detecting whether functions are supported at runtime, except in specific
   cases where new interfaces need to be detected to support y2038 and LFS.
 - Hiding significant differences between platforms.
 - Restricting ambient authorities.
 - Imposing sandboxing features such as filesystem path or network address
   sandboxing.

See [`cap-std`], [`system-interface`], and [`io-streams`] for libraries
which do hide significant differences between platforms, and [`cap-std`]
which does perform sandboxing and restricts ambient authorities.

[`cap-std`]: https://crates.io/crates/cap-std
[`system-interface`]: https://crates.io/crates/system-interface
[`io-streams`]: https://crates.io/crates/io-streams
[`bitflags`]: bitflags
[`AsFd`]: crate::fd::AsFd
[`OwnedFd`]: crate::fd::OwnedFd
[I/O-safe]: https://github.com/rust-lang/rfcs/blob/master/text/3128-io-safety.md
[`Arg`]: path::Arg
[support for externally defined flags]: bitflags#externally-defined-flags

## Modules

## Module `buffer`

**Attributes:**

- `#[allow(unsafe_code)]`

Utilities for functions that return data via buffers.

```rust
pub mod buffer { /* ... */ }
```

### Types

#### Struct `SpareCapacity`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "alloc")))]`

A type that implements [`Buffer`] by appending to a `Vec`, up to its
capacity.

To use this, use the [`spare_capacity`] function.

Because this uses the capacity, and never reallocates, the `Vec` should
have some non-empty spare capacity.

```rust
pub struct SpareCapacity<''a, T>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Buffer**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `Buffer`

A memory buffer that may be uninitialized.

There are three types that implement the `Buffer` trait, and the type you
use determines the return type of the functions that use it:

| If you pass a…           | You get back a… |
| ------------------------ | --------------- |
| `&mut [u8]`              | `usize`, indicating the number of elements initialized. |
| `&mut [MaybeUninit<u8>]` | `(&mut [u8], &mut [MaybeUninit<u8>])`, holding the initialized and uninitialized subslices. |
| [`SpareCapacity`]        | `usize`, indicating the number of elements initialized. And the `Vec` is extended. |

# Examples

Passing a `&mut [u8]`:

```
# use rustix::io::read;
# fn example(fd: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {
let mut buf = [0_u8; 64];
let nread = read(fd, &mut buf)?;
// `nread` is the number of bytes read.
# Ok(())
# }
```

Passing a `&mut [MaybeUninit<u8>]`:

```
# use rustix::io::read;
# use std::mem::MaybeUninit;
# fn example(fd: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {
let mut buf = [MaybeUninit::<u8>::uninit(); 64];
let (init, uninit) = read(fd, &mut buf)?;
// `init` is a `&mut [u8]` with the initialized bytes.
// `uninit` is a `&mut [MaybeUninit<u8>]` with the remaining bytes.
# Ok(())
# }
```

Passing a [`SpareCapacity`], via the [`spare_capacity`] helper function:

```
# use rustix::io::read;
# use rustix::buffer::spare_capacity;
# fn example(fd: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {
let mut buf = Vec::with_capacity(64);
let nread = read(fd, spare_capacity(&mut buf))?;
// `nread` is the number of bytes read.
// Also, `buf.len()` is now `nread` elements longer than it was before.
# Ok(())
# }
```

# Guide to error messages

Sometimes code using `Buffer` can encounter non-obvious error messages.
Here are some we've encountered, along with ways to fix them.

If you see errors like
"cannot move out of `self` which is behind a mutable reference"
and
"move occurs because `x` has type `&mut [u8]`, which does not implement the `Copy` trait",
replace `x` with `&mut *x`. See `error_buffer_wrapper` in
examples/buffer_errors.rs.

If you see errors like
"type annotations needed"
and
"cannot infer type of the type parameter `Buf` declared on the function `read`",
you may need to change a `&mut []` to `&mut [0_u8; 0]`. See
`error_empty_slice` in examples/buffer_errors.rs.

If you see errors like
"the trait bound `[MaybeUninit<u8>; 1]: Buffer<u8>` is not satisfied",
add a `&mut` to pass the array by reference instead of by value. See
`error_array_by_value` in examples/buffer_errors.rs.

If you see errors like
"cannot move out of `x`, a captured variable in an `FnMut` closure",
try replacing `x` with `&mut *x`, or, if that doesn't work, try moving a
`let` into the closure body. See `error_retry_closure` and
`error_retry_indirect_closure` in examples/buffer_errors.rs.

If you see errors like
"captured variable cannot escape `FnMut` closure body",
use an explicit loop instead of `retry_on_intr`, assuming you're using
that. See `error_retry_closure_uninit` in examples/buffer_errors.rs.

```rust
pub trait Buffer<T>: private::Sealed<T> {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `&mut [T]` with <T>
- `&mut [T; N]` with <T, const N: usize>
- `&mut alloc::vec::Vec<T>` with <T>
- `&mut [core::mem::MaybeUninit<T>]` with <T>
- `&mut [core::mem::MaybeUninit<T>; N]` with <T, const N: usize>
- `&mut alloc::vec::Vec<core::mem::MaybeUninit<T>>` with <T>
- `SpareCapacity<''a, T>` with <''a, T>

### Functions

#### Function `spare_capacity`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "alloc")))]`

Construct an [`SpareCapacity`], which implements [`Buffer`].

This wraps a `Vec` and uses the spare capacity of the `Vec` as the buffer
to receive data in, automatically calling `set_len` on the `Vec` to set the
length to include the received elements.

This uses the existing capacity, and never allocates, so the `Vec` should
have some non-empty spare capacity!

# Examples

```
# fn test(input: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {
use rustix::buffer::spare_capacity;
use rustix::io::{read, Errno};

let mut buf = Vec::with_capacity(1024);
match read(input, spare_capacity(&mut buf)) {
    Ok(0) => { /* end of stream */ }
    Ok(n) => { /* `buf` is now `n` bytes longer */ }
    Err(Errno::INTR) => { /* `buf` is unmodified */ }
    Err(e) => {
        return Err(e);
    }
}

# Ok(())
# }
```

```rust
pub fn spare_capacity<''a, T>(v: &''a mut alloc::vec::Vec<T>) -> SpareCapacity<''a, T> { /* ... */ }
```

## Module `fd`

Export the `*Fd` types and traits that are used in rustix's public API.

This module exports the types and traits from [`std::os::fd`], or polyills
on Rust < 1.66 or on Windows.

On Windows, the polyfill consists of aliases of the socket types and
traits, For example, [`OwnedSocket`] is aliased to `OwnedFd`, and so on,
and there are blanket impls for `AsFd` etc. that map to `AsSocket` impls.
These blanket impls suffice for using the traits, however not for
implementing them, so this module also exports `AsSocket` and the other
traits as-is so that users can implement them if needed.

[`OwnedSocket`]: https://doc.rust-lang.org/stable/std/os/windows/io/struct.OwnedSocket.html

```rust
pub mod fd { /* ... */ }
```

### Re-exports

#### Re-export `super::backend::fd::*`

```rust
pub use super::backend::fd::*;
```

## Module `ffi`

Utilities related to FFI bindings.

```rust
pub mod ffi { /* ... */ }
```

### Re-exports

#### Re-export `CStr`

**Attributes:**

- `#[<cfg>(not(windows))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use std::ffi::CStr;
```

#### Re-export `CString`

**Attributes:**

- `#[<cfg>(not(windows))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use std::ffi::CString;
```

#### Re-export `FromBytesWithNulError`

**Attributes:**

- `#[<cfg>(not(windows))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use std::ffi::FromBytesWithNulError;
```

#### Re-export `NulError`

**Attributes:**

- `#[<cfg>(not(windows))]`
- `#[<cfg>(feature = "std")]`

```rust
pub use std::ffi::NulError;
```

#### Re-export `c_char`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_char;
```

#### Re-export `c_int`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_int;
```

#### Re-export `c_long`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_long;
```

#### Re-export `c_longlong`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_longlong;
```

#### Re-export `c_short`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_short;
```

#### Re-export `c_uint`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_uint;
```

#### Re-export `c_ulong`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_ulong;
```

#### Re-export `c_ulonglong`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_ulonglong;
```

#### Re-export `c_ushort`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_ushort;
```

#### Re-export `c_void`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use std::os::raw::c_void;
```

## Module `fs`

**Attributes:**

- `#[<cfg>(not(windows))]`
- `#[<cfg>(feature = "fs")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "fs")))]`

Filesystem operations.

```rust
pub mod fs { /* ... */ }
```

### Modules

## Module `inotify`

**Attributes:**

- `#[<cfg>(linux_kernel)]`
- `#[allow(unused_qualifications)]`

inotify support for working with inotify objects.

# Examples

```
use rustix::fs::inotify;
use rustix::io;
use std::mem::MaybeUninit;

# fn test() -> io::Result<()> {
// Create an inotify object. In this example, we use `NONBLOCK` so that the
// reader fails with `WOULDBLOCK` when no events are ready. Otherwise it
// will block until at least one event is ready.
let inotify = inotify::init(inotify::CreateFlags::NONBLOCK)?;

// Add a directory to watch.
inotify::add_watch(
    &inotify,
    "/path/to/some/directory/to/watch",
    inotify::WatchFlags::ALL_EVENTS,
)?;

// Generate some events in the watched directory…

// Loop over pending events.
let mut buf = [MaybeUninit::uninit(); 512];
let mut iter = inotify::Reader::new(inotify, &mut buf);
loop {
    let entry = match iter.next() {
        // Stop iterating if there are no more events for now.
        Err(io::Errno::WOULDBLOCK) => break,
        Err(e) => return Err(e),
        Ok(entry) => entry,
    };

    // Use `entry`…
}

# Ok(())
# }

```rust
pub mod inotify { /* ... */ }
```

### Types

#### Struct `Reader`

An inotify event iterator implemented with the read syscall.

See the [`RawDir`] API for more details and usage examples as this API is
based on it.

[`RawDir`]: crate::fs::raw_dir::RawDir

```rust
pub struct Reader<''buf, Fd: AsFd> {
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
  pub fn new(fd: Fd, buf: &''buf mut [MaybeUninit<u8>]) -> Self { /* ... */ }
  ```
  Create a new iterator from the given file descriptor and buffer.

- ```rust
  pub fn next(self: &mut Self) -> io::Result<Event<''_>> { /* ... */ }
  ```
  Read the next inotify event.

- ```rust
  pub fn is_buffer_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the internal buffer is empty and will be refilled when

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **UnwindSafe**
- **Send**
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
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Event`

**Attributes:**

- `#[doc(alias = "inotify_event")]`

An inotify event.

```rust
pub struct Event<''a> {
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
  pub fn wd(self: &Self) -> i32 { /* ... */ }
  ```
  Returns the watch for which this event occurs.

- ```rust
  pub fn events(self: &Self) -> ReadFlags { /* ... */ }
  ```
  Returns a description of the events.

- ```rust
  pub fn cookie(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the unique cookie associating related events.

- ```rust
  pub fn file_name(self: &Self) -> Option<&CStr> { /* ... */ }
  ```
  Returns the file name of this event, if any.

###### Trait Implementations

- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
### Functions

#### Function `init`

**Attributes:**

- `#[doc(alias = "inotify_init1")]`
- `#[inline]`

`inotify_init1(flags)`—Creates a new inotify object.

Use the [`CreateFlags::CLOEXEC`] flag to prevent the resulting file
descriptor from being implicitly passed across `exec` boundaries.

```rust
pub fn init(flags: inotify::CreateFlags) -> io::Result<crate::fd::OwnedFd> { /* ... */ }
```

#### Function `add_watch`

**Attributes:**

- `#[doc(alias = "inotify_add_watch")]`
- `#[inline]`

`inotify_add_watch(self, path, flags)`—Adds a watch to inotify.

This registers or updates a watch for the filesystem path `path` and
returns a watch descriptor corresponding to this watch.

Note: Due to the existence of hardlinks, providing two different paths to
this method may result in it returning the same watch descriptor. An
application should keep track of this externally to avoid logic errors.

```rust
pub fn add_watch<P: crate::path::Arg, Fd: AsFd>(inot: Fd, path: P, flags: inotify::WatchFlags) -> io::Result<i32> { /* ... */ }
```

#### Function `remove_watch`

**Attributes:**

- `#[doc(alias = "inotify_rm_watch")]`
- `#[inline]`

`inotify_rm_watch(self, wd)`—Removes a watch from this inotify.

The watch descriptor provided should have previously been returned by
[`inotify::add_watch`] and not previously have been removed.

```rust
pub fn remove_watch<Fd: AsFd>(inot: Fd, wd: i32) -> io::Result<()> { /* ... */ }
```

### Re-exports

#### Re-export `CreateFlags`

```rust
pub use crate::backend::fs::inotify::CreateFlags;
```

#### Re-export `ReadFlags`

```rust
pub use crate::backend::fs::inotify::ReadFlags;
```

#### Re-export `WatchFlags`

```rust
pub use crate::backend::fs::inotify::WatchFlags;
```

### Re-exports

#### Re-export `copy_file_range`

**Attributes:**

- `#[<cfg>(linux_kernel)]`

```rust
pub use copy_file_range::copy_file_range;
```

#### Re-export `Dir`

**Attributes:**

- `#[<cfg>(all(feature = "alloc",
not(any(target_os = "espidf", target_os = "redox"))))]`

```rust
pub use dir::Dir;
```

#### Re-export `DirEntry`

**Attributes:**

- `#[<cfg>(all(feature = "alloc",
not(any(target_os = "espidf", target_os = "redox"))))]`

```rust
pub use dir::DirEntry;
```

#### Re-export `fadvise`

**Attributes:**

- `#[<cfg>(not(any(apple, netbsdlike, target_os = "dragonfly", target_os =
"espidf", target_os = "haiku", target_os = "horizon", target_os = "redox",
target_os = "solaris", target_os = "vita",)))]`

```rust
pub use fadvise::fadvise;
```

#### Re-export `memfd_create`

**Attributes:**

- `#[<cfg>(any(linux_kernel, target_os = "freebsd"))]`

```rust
pub use memfd_create::memfd_create;
```

#### Re-export `openat2`

**Attributes:**

- `#[<cfg>(linux_kernel)]`

```rust
pub use openat2::openat2;
```

#### Re-export `RawDir`

**Attributes:**

- `#[<cfg>(linux_kernel)]`

```rust
pub use raw_dir::RawDir;
```

#### Re-export `RawDirEntry`

**Attributes:**

- `#[<cfg>(linux_kernel)]`

```rust
pub use raw_dir::RawDirEntry;
```

#### Re-export `SeekFrom`

```rust
pub use seek_from::SeekFrom;
```

#### Re-export `sendfile`

**Attributes:**

- `#[<cfg>(target_os = "linux")]`

```rust
pub use sendfile::sendfile;
```

#### Re-export `sync`

**Attributes:**

- `#[<cfg>(not(any(target_os = "espidf", target_os = "horizon", target_os =
"redox", target_os = "vita", target_os = "wasi")))]`

```rust
pub use sync::sync;
```

#### Re-export `DirEntryExt`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg>(unix)]`

Re-export types common to POSIX-ish platforms.

```rust
pub use std::os::unix::fs::DirEntryExt;
```

#### Re-export `FileExt`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg>(unix)]`

Re-export types common to POSIX-ish platforms.

```rust
pub use std::os::unix::fs::FileExt;
```

#### Re-export `FileTypeExt`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg>(unix)]`

Re-export types common to POSIX-ish platforms.

```rust
pub use std::os::unix::fs::FileTypeExt;
```

#### Re-export `MetadataExt`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg>(unix)]`

Re-export types common to POSIX-ish platforms.

```rust
pub use std::os::unix::fs::MetadataExt;
```

#### Re-export `OpenOptionsExt`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg>(unix)]`

Re-export types common to POSIX-ish platforms.

```rust
pub use std::os::unix::fs::OpenOptionsExt;
```

#### Re-export `abs::*`

```rust
pub use abs::*;
```

#### Re-export `at::*`

**Attributes:**

- `#[<cfg>(not(target_os = "redox"))]`

```rust
pub use at::*;
```

#### Re-export `constants::*`

```rust
pub use constants::*;
```

#### Re-export `fcntl::*`

```rust
pub use fcntl::*;
```

#### Re-export `fd::*`

```rust
pub use fd::*;
```

#### Re-export `id::*`

**Attributes:**

- `#[<cfg>(not(target_os = "wasi"))]`

```rust
pub use id::*;
```

#### Re-export `ioctl::*`

**Attributes:**

- `#[<cfg>(linux_kernel)]`

```rust
pub use ioctl::*;
```

#### Re-export `makedev::*`

**Attributes:**

- `#[<cfg>(not(any(target_os = "espidf", target_os = "haiku", target_os =
"horizon", target_os = "redox", target_os = "vita", target_os = "wasi")))]`

```rust
pub use makedev::*;
```

#### Re-export `special::*`

**Attributes:**

- `#[<cfg>(not(any(target_os = "espidf", target_os = "redox")))]`

```rust
pub use special::*;
```

#### Re-export `statx::*`

**Attributes:**

- `#[<cfg>(linux_kernel)]`

```rust
pub use statx::*;
```

#### Re-export `xattr::*`

**Attributes:**

- `#[<cfg>(any(apple, linux_kernel, target_os = "hurd"))]`

```rust
pub use xattr::*;
```

## Module `io`

I/O operations.

If you're looking for [`SeekFrom`], it's in the [`fs`] module.

[`SeekFrom`]: crate::fs::SeekFrom
[`fs`]: crate::fs

```rust
pub mod io { /* ... */ }
```

### Re-exports

#### Re-export `retry_on_intr`

```rust
pub use errno::retry_on_intr;
```

#### Re-export `Errno`

```rust
pub use errno::Errno;
```

#### Re-export `Result`

```rust
pub use errno::Result;
```

#### Re-export `close::*`

```rust
pub use close::*;
```

#### Re-export `dup::*`

**Attributes:**

- `#[<cfg>(not(windows))]`

```rust
pub use dup::*;
```

#### Re-export `fcntl::*`

**Attributes:**

- `#[<cfg>(not(windows))]`

```rust
pub use fcntl::*;
```

#### Re-export `ioctl::*`

```rust
pub use ioctl::*;
```

#### Re-export `read_write::*`

```rust
pub use read_write::*;
```

## Module `ioctl`

**Attributes:**

- `#[allow(unsafe_code)]`

Unsafe `ioctl` API.

Unix systems expose a number of `ioctl`'s. `ioctl`s have been adopted as a
general purpose system call for making calls into the kernel. In addition
to the wide variety of system calls that are included by default in the
kernel, many drivers expose their own `ioctl`'s for controlling their
behavior, some of which are proprietary. Therefore it is impossible to make
a safe interface for every `ioctl` call, as they all have wildly varying
semantics.

This module provides an unsafe interface to write your own `ioctl` API. To
start, create a type that implements [`Ioctl`]. Then, pass it to [`ioctl`]
to make the `ioctl` call.

```rust
pub mod ioctl { /* ... */ }
```

### Modules

## Module `opcode`

**Attributes:**

- `#[<cfg>(any(linux_kernel, bsd))]`

Const functions for computing opcode values.

Linux's headers define macros such as `_IO`, `_IOR`, `_IOW`, and `_IOWR`
for defining ioctl values in a structured way that encode whether they
are reading and/or writing, and other information about the ioctl. The
functions in this module correspond to those macros.

If you're writing a driver and defining your own ioctl numbers, it's
recommended to use these functions to compute them.

```rust
pub mod opcode { /* ... */ }
```

### Functions

#### Function `from_components`

**Attributes:**

- `#[doc(alias = "_IOC")]`
- `#[inline]`

Create a new opcode from a direction, group, number, and size.

This corresponds to the C macro `_IOC(direction, group, number, size)`

```rust
pub const fn from_components(direction: Direction, group: u8, number: u8, data_size: usize) -> Opcode { /* ... */ }
```

#### Function `none`

**Attributes:**

- `#[doc(alias = "_IO")]`
- `#[inline]`

Create a new opcode from a group, a number, that uses no data.

This corresponds to the C macro `_IO(group, number)`.

```rust
pub const fn none(group: u8, number: u8) -> Opcode { /* ... */ }
```

#### Function `read`

**Attributes:**

- `#[doc(alias = "_IOR")]`
- `#[inline]`

Create a new reading opcode from a group, a number and the type of
data.

This corresponds to the C macro `_IOR(group, number, T)`.

```rust
pub const fn read<T>(group: u8, number: u8) -> Opcode { /* ... */ }
```

#### Function `write`

**Attributes:**

- `#[doc(alias = "_IOW")]`
- `#[inline]`

Create a new writing opcode from a group, a number and the type of
data.

This corresponds to the C macro `_IOW(group, number, T)`.

```rust
pub const fn write<T>(group: u8, number: u8) -> Opcode { /* ... */ }
```

#### Function `read_write`

**Attributes:**

- `#[doc(alias = "_IOWR")]`
- `#[inline]`

Create a new reading and writing opcode from a group, a number and the
type of data.

This corresponds to the C macro `_IOWR(group, number, T)`.

```rust
pub const fn read_write<T>(group: u8, number: u8) -> Opcode { /* ... */ }
```

### Types

#### Enum `Direction`

The direction that an `ioctl` is going.

The direction is relative to userspace: `Read` means reading data from the
kernel, and `Write` means the kernel writing data to userspace.

```rust
pub enum Direction {
    None,
    Read,
    Write,
    ReadWrite,
}
```

##### Variants

###### `None`

None of the above.

###### `Read`

Read data from the kernel.

###### `Write`

Write data to the kernel.

###### `ReadWrite`

Read and write data to the kernel.

##### Implementations

###### Trait Implementations

- **Unpin**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Copy**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Direction { /* ... */ }
    ```

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Direction) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Direction) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Direction) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
- **Sync**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Type Alias `IoctlOutput`

The type used by the `ioctl` to signify the output.

```rust
pub type IoctlOutput = c::c_int;
```

#### Type Alias `Opcode`

The type used by the `ioctl` to signify the command.

```rust
pub type Opcode = c::c_uint;
```

### Traits

#### Trait `Ioctl`

A trait defining the properties of an `ioctl` command.

Objects implementing this trait can be passed to [`ioctl`] to make an
`ioctl` call. The contents of the object represent the inputs to the
`ioctl` call. The inputs must be convertible to a pointer through the
`as_ptr` method. In most cases, this involves either casting a number to a
pointer, or creating a pointer to the actual data. The latter case is
necessary for `ioctl` calls that modify userspace data.

# Safety

This trait is unsafe to implement because it is impossible to guarantee
that the `ioctl` call is safe. The `ioctl` call may be proprietary, or it
may be unsafe to call in certain circumstances.

By implementing this trait, you guarantee that:

 - The `ioctl` call expects the input provided by `as_ptr` and produces the
   output as indicated by `output`.
 - That `output_from_ptr` can safely take the pointer from `as_ptr` and
   cast it to the correct type, *only* after the `ioctl` call.
 - That the return value of `opcode` uniquely identifies the `ioctl` call.
 - That, for whatever platforms you are targeting, the `ioctl` call is safe
   to make.
 - If `IS_MUTATING` is false, that no userspace data will be modified by
   the `ioctl` call.

```rust
pub unsafe trait Ioctl {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Output`: The type of the output data.

###### Associated Constants

- `IS_MUTATING`: Does the `ioctl` mutate any data in the userspace?

###### Required Methods

- `opcode`: Get the opcode used by this `ioctl` command.
- `as_ptr`: Get a pointer to the data to be passed to the `ioctl` command.
- `output_from_ptr`: Cast the output data to the correct type.

##### Implementations

This trait is implemented for the following types:

- `NoArg<OPCODE>` with <const OPCODE: super::Opcode>
- `Getter<OPCODE, Output>` with <const OPCODE: super::Opcode, Output>
- `Setter<OPCODE, Input>` with <const OPCODE: super::Opcode, Input>
- `Updater<''a, OPCODE, T>` with <''a, const OPCODE: super::Opcode, T>
- `IntegerSetter<OPCODE>` with <const OPCODE: super::Opcode>

### Functions

#### Function `ioctl`

**Attributes:**

- `#[inline]`

Perform an `ioctl` call.

`ioctl` was originally intended to act as a way of modifying the behavior
of files, but has since been adopted as a general purpose system call for
making calls into the kernel. In addition to the default calls exposed by
generic file descriptors, many drivers expose their own `ioctl` calls for
controlling their behavior, some of which are proprietary.

This crate exposes many other `ioctl` interfaces with safe and idiomatic
wrappers, like [`ioctl_fionbio`] and [`ioctl_fionread`]. It is recommended
to use those instead of this function, as they are safer and more
idiomatic. For other cases, implement the [`Ioctl`] API and pass it to this
function.

See documentation for [`Ioctl`] for more information.

[`ioctl_fionbio`]: crate::io::ioctl_fionbio
[`ioctl_fionread`]: crate::io::ioctl_fionread

# Safety

While [`Ioctl`] takes much of the unsafety out of `ioctl` calls, callers
must still ensure that the opcode value, operand type, and data access
correctly reflect what's in the device driver servicing the call. `ioctl`
calls form a protocol between the userspace `ioctl` callers and the device
drivers in the kernel, and safety depends on both sides agreeing and
upholding the expectations of the other.

And, `ioctl` calls can read and write arbitrary memory and have arbitrary
side effects. Callers must ensure that any memory accesses and side effects
are compatible with Rust language invariants.

# References
 - [Linux]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [Apple]
 - [Solaris]
 - [illumos]

[Linux]: https://man7.org/linux/man-pages/man2/ioctl.2.html
[Winsock]: https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-ioctlsocket
[FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=ioctl&sektion=2
[NetBSD]: https://man.netbsd.org/ioctl.2
[OpenBSD]: https://man.openbsd.org/ioctl.2
[Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/ioctl.2.html
[Solaris]: https://docs.oracle.com/cd/E23824_01/html/821-1463/ioctl-2.html
[illumos]: https://illumos.org/man/2/ioctl

```rust
pub unsafe fn ioctl<F: AsFd, I: Ioctl>(fd: F, ioctl: I) -> crate::io::Result<<I as >::Output> { /* ... */ }
```

### Re-exports

#### Re-export `patterns::*`

```rust
pub use patterns::*;
```

## Module `path`

**Attributes:**

- `#[<cfg>(not(windows))]`
- `#[<cfg>(any(feature = "fs", feature = "mount", feature = "net"))]`
- `#[<cfg_attr>(docsrs,
doc(cfg(any(feature = "fs", feature = "mount", feature = "net"))))]`

Filesystem path operations.

```rust
pub mod path { /* ... */ }
```

### Re-exports

#### Re-export `option_into_with_c_str`

```rust
pub use arg::option_into_with_c_str;
```

#### Re-export `Arg`

```rust
pub use arg::Arg;
```

#### Re-export `DecInt`

```rust
pub use dec_int::DecInt;
```

#### Re-export `Integer`

```rust
pub use dec_int::Integer;
```

## Module `not_implemented`

**Attributes:**

- `#[<cfg>(doc)]`
- `#[<cfg_attr>(docsrs, doc(cfg(doc)))]`

Documentation about unimplemented functions.

This module contains documentation for several functions that rustix does
not implement, either because they are out of scope, or because they are
could probably be implemented but are not yet.

```rust
pub mod not_implemented { /* ... */ }
```

### Modules

## Module `memory_allocation`

Memory-allocation functions are out of scope for rustix.

It is possible to implement `malloc`, `free`, and similar functions in
Rust, however rustix itself is focused on syscall-like functions. This
module contains an incomplete list of such functions.

There are several allocator implementations for Rust; one of them is
[dlmalloc]. For a rustix-based implementation, see [rustix-dlmalloc].
Another allocator implementation is [talc].

[dlmalloc]: https://crates.io/crates/dlmalloc
[talc]: https://crates.io/crates/talc
[rustix-dlmalloc]: https://crates.io/crates/rustix-dlmalloc

```rust
pub mod memory_allocation { /* ... */ }
```

### Functions

#### Function `malloc`

See the [module comment](self).

```rust
pub fn malloc() { /* ... */ }
```

#### Function `realloc`

See the [module comment](self).

```rust
pub fn realloc() { /* ... */ }
```

#### Function `calloc`

See the [module comment](self).

```rust
pub fn calloc() { /* ... */ }
```

#### Function `free`

See the [module comment](self).

```rust
pub fn free() { /* ... */ }
```

#### Function `posix_memalign`

See the [module comment](self).

```rust
pub fn posix_memalign() { /* ... */ }
```

#### Function `aligned_alloc`

See the [module comment](self).

```rust
pub fn aligned_alloc() { /* ... */ }
```

#### Function `malloc_usable_size`

See the [module comment](self).

```rust
pub fn malloc_usable_size() { /* ... */ }
```

## Module `libc_internals`

Functions which need access to libc internals are out of scope for rustix.

Most Rust programs have a libc present, and when a libc is present, it
expects to be the only thing in the process that can do certain operations.
For example, there can be only one `atexit` list in a process, only one
`pthread_atfork` list in a process, only one implementation of pthreads in
a process, and so on, and libc expects to own the one of each of those
things. And libc implementations may expect to be involved in signal
handling. So, these functions are believed to be out of scope for rustix.
This module contains an incomplete list of such functions.

It would be possible to make a rust library which provides safe or
ergonomic wrappers around these libc functions, however that is out of
scope for rustix itself.

If you would like to write a Rust program which does not use a libc, and
which does provide APIs for some of these functions, [Eyra] and [origin]
are two libraries which may be useful, and which provide public interfaces
for some of this functionality.

If you are otherwise writing Rust code which you know will not share a
process with a libc, perhaps because you are writing a libc or similar
yourself, rustix's codebase does include experimental implementations of
the primitives needed to implement most of these functions.

[Eyra]: https://github.com/sunfishcode/eyra?tab=readme-ov-file#eyra
[origin]: https://github.com/sunfishcode/origin?tab=readme-ov-file#origin

```rust
pub mod libc_internals { /* ... */ }
```

### Functions

#### Function `exit`

See the [module comment](self).

```rust
pub fn exit() { /* ... */ }
```

#### Function `fork`

See the [module comment](self).

```rust
pub fn fork() { /* ... */ }
```

#### Function `clone`

See the [module comment](self).

```rust
pub fn clone() { /* ... */ }
```

#### Function `clone3`

See the [module comment](self).

```rust
pub fn clone3() { /* ... */ }
```

#### Function `brk`

See the [module comment](self).

```rust
pub fn brk() { /* ... */ }
```

#### Function `sigaction`

See the [module comment](self).

```rust
pub fn sigaction() { /* ... */ }
```

#### Function `sigaltstack`

See the [module comment](self).

```rust
pub fn sigaltstack() { /* ... */ }
```

#### Function `sigprocmask`

See the [module comment](self).

```rust
pub fn sigprocmask() { /* ... */ }
```

#### Function `sigwait`

See the [module comment](self).

```rust
pub fn sigwait() { /* ... */ }
```

#### Function `sigwaitinfo`

See the [module comment](self).

```rust
pub fn sigwaitinfo() { /* ... */ }
```

#### Function `sigtimedwait`

See the [module comment](self).

```rust
pub fn sigtimedwait() { /* ... */ }
```

#### Function `set_thread_area`

See the [module comment](self).

```rust
pub fn set_thread_area() { /* ... */ }
```

#### Function `set_tid_address`

See the [module comment](self).

```rust
pub fn set_tid_address() { /* ... */ }
```

#### Function `tkill`

See the [module comment](self).

```rust
pub fn tkill() { /* ... */ }
```

#### Function `sched_setscheduler`

See the [module comment](self).

```rust
pub fn sched_setscheduler() { /* ... */ }
```

#### Function `rseq`

See the [module comment](self).

```rust
pub fn rseq() { /* ... */ }
```

#### Function `setuid`

See the [module comment](self).

```rust
pub fn setuid() { /* ... */ }
```

#### Function `setgid`

See the [module comment](self).

```rust
pub fn setgid() { /* ... */ }
```

#### Function `seteuid`

See the [module comment](self).

```rust
pub fn seteuid() { /* ... */ }
```

#### Function `setegid`

See the [module comment](self).

```rust
pub fn setegid() { /* ... */ }
```

#### Function `setreuid`

See the [module comment](self).

```rust
pub fn setreuid() { /* ... */ }
```

#### Function `setregid`

See the [module comment](self).

```rust
pub fn setregid() { /* ... */ }
```

#### Function `setresuid`

See the [module comment](self).

```rust
pub fn setresuid() { /* ... */ }
```

#### Function `setresgid`

See the [module comment](self).

```rust
pub fn setresgid() { /* ... */ }
```

#### Function `setgroups`

See the [module comment](self).

```rust
pub fn setgroups() { /* ... */ }
```

#### Function `pthread_atfork`

See the [module comment](self).

```rust
pub fn pthread_atfork() { /* ... */ }
```

#### Function `pthread_attr_destroy`

See the [module comment](self).

```rust
pub fn pthread_attr_destroy() { /* ... */ }
```

#### Function `pthread_attr_getaffinity_np`

See the [module comment](self).

```rust
pub fn pthread_attr_getaffinity_np() { /* ... */ }
```

#### Function `pthread_attr_getdetachstate`

See the [module comment](self).

```rust
pub fn pthread_attr_getdetachstate() { /* ... */ }
```

#### Function `pthread_attr_getguardsize`

See the [module comment](self).

```rust
pub fn pthread_attr_getguardsize() { /* ... */ }
```

#### Function `pthread_attr_getinheritsched`

See the [module comment](self).

```rust
pub fn pthread_attr_getinheritsched() { /* ... */ }
```

#### Function `pthread_attr_getschedparam`

See the [module comment](self).

```rust
pub fn pthread_attr_getschedparam() { /* ... */ }
```

#### Function `pthread_attr_getschedpolicy`

See the [module comment](self).

```rust
pub fn pthread_attr_getschedpolicy() { /* ... */ }
```

#### Function `pthread_attr_getscope`

See the [module comment](self).

```rust
pub fn pthread_attr_getscope() { /* ... */ }
```

#### Function `pthread_attr_getsigmask_np`

See the [module comment](self).

```rust
pub fn pthread_attr_getsigmask_np() { /* ... */ }
```

#### Function `pthread_attr_getstack`

See the [module comment](self).

```rust
pub fn pthread_attr_getstack() { /* ... */ }
```

#### Function `pthread_attr_getstackaddr`

See the [module comment](self).

```rust
pub fn pthread_attr_getstackaddr() { /* ... */ }
```

#### Function `pthread_attr_getstacksize`

See the [module comment](self).

```rust
pub fn pthread_attr_getstacksize() { /* ... */ }
```

#### Function `pthread_attr_init`

See the [module comment](self).

```rust
pub fn pthread_attr_init() { /* ... */ }
```

#### Function `pthread_attr_setaffinity_np`

See the [module comment](self).

```rust
pub fn pthread_attr_setaffinity_np() { /* ... */ }
```

#### Function `pthread_attr_setdetachstate`

See the [module comment](self).

```rust
pub fn pthread_attr_setdetachstate() { /* ... */ }
```

#### Function `pthread_attr_setguardsize`

See the [module comment](self).

```rust
pub fn pthread_attr_setguardsize() { /* ... */ }
```

#### Function `pthread_attr_setinheritsched`

See the [module comment](self).

```rust
pub fn pthread_attr_setinheritsched() { /* ... */ }
```

#### Function `pthread_attr_setschedparam`

See the [module comment](self).

```rust
pub fn pthread_attr_setschedparam() { /* ... */ }
```

#### Function `pthread_attr_setschedpolicy`

See the [module comment](self).

```rust
pub fn pthread_attr_setschedpolicy() { /* ... */ }
```

#### Function `pthread_attr_setscope`

See the [module comment](self).

```rust
pub fn pthread_attr_setscope() { /* ... */ }
```

#### Function `pthread_attr_setsigmask_np`

See the [module comment](self).

```rust
pub fn pthread_attr_setsigmask_np() { /* ... */ }
```

#### Function `pthread_attr_setstack`

See the [module comment](self).

```rust
pub fn pthread_attr_setstack() { /* ... */ }
```

#### Function `pthread_attr_setstackaddr`

See the [module comment](self).

```rust
pub fn pthread_attr_setstackaddr() { /* ... */ }
```

#### Function `pthread_attr_setstacksize`

See the [module comment](self).

```rust
pub fn pthread_attr_setstacksize() { /* ... */ }
```

#### Function `pthread_barrierattr_destroy`

See the [module comment](self).

```rust
pub fn pthread_barrierattr_destroy() { /* ... */ }
```

#### Function `pthread_barrierattr_getpshared`

See the [module comment](self).

```rust
pub fn pthread_barrierattr_getpshared() { /* ... */ }
```

#### Function `pthread_barrierattr_init`

See the [module comment](self).

```rust
pub fn pthread_barrierattr_init() { /* ... */ }
```

#### Function `pthread_barrierattr_setpshared`

See the [module comment](self).

```rust
pub fn pthread_barrierattr_setpshared() { /* ... */ }
```

#### Function `pthread_barrier_destroy`

See the [module comment](self).

```rust
pub fn pthread_barrier_destroy() { /* ... */ }
```

#### Function `pthread_barrier_wait`

See the [module comment](self).

```rust
pub fn pthread_barrier_wait() { /* ... */ }
```

#### Function `pthread_cancel`

See the [module comment](self).

```rust
pub fn pthread_cancel() { /* ... */ }
```

#### Function `pthread_cleanup_pop`

See the [module comment](self).

```rust
pub fn pthread_cleanup_pop() { /* ... */ }
```

#### Function `pthread_cleanup_pop_restore_np`

See the [module comment](self).

```rust
pub fn pthread_cleanup_pop_restore_np() { /* ... */ }
```

#### Function `pthread_cleanup_push`

See the [module comment](self).

```rust
pub fn pthread_cleanup_push() { /* ... */ }
```

#### Function `pthread_cleanup_push_defer_np`

See the [module comment](self).

```rust
pub fn pthread_cleanup_push_defer_np() { /* ... */ }
```

#### Function `pthread_condattr_destroy`

See the [module comment](self).

```rust
pub fn pthread_condattr_destroy() { /* ... */ }
```

#### Function `pthread_condattr_getclock`

See the [module comment](self).

```rust
pub fn pthread_condattr_getclock() { /* ... */ }
```

#### Function `pthread_condattr_getpshared`

See the [module comment](self).

```rust
pub fn pthread_condattr_getpshared() { /* ... */ }
```

#### Function `pthread_condattr_init`

See the [module comment](self).

```rust
pub fn pthread_condattr_init() { /* ... */ }
```

#### Function `pthread_condattr_setclock`

See the [module comment](self).

```rust
pub fn pthread_condattr_setclock() { /* ... */ }
```

#### Function `pthread_condattr_setpshared`

See the [module comment](self).

```rust
pub fn pthread_condattr_setpshared() { /* ... */ }
```

#### Function `pthread_cond_broadcast`

See the [module comment](self).

```rust
pub fn pthread_cond_broadcast() { /* ... */ }
```

#### Function `pthread_cond_destroy`

See the [module comment](self).

```rust
pub fn pthread_cond_destroy() { /* ... */ }
```

#### Function `pthread_cond_signal`

See the [module comment](self).

```rust
pub fn pthread_cond_signal() { /* ... */ }
```

#### Function `pthread_cond_timedwait`

See the [module comment](self).

```rust
pub fn pthread_cond_timedwait() { /* ... */ }
```

#### Function `pthread_create`

See the [module comment](self).

```rust
pub fn pthread_create() { /* ... */ }
```

#### Function `pthread_detach`

See the [module comment](self).

```rust
pub fn pthread_detach() { /* ... */ }
```

#### Function `pthread_equal`

See the [module comment](self).

```rust
pub fn pthread_equal() { /* ... */ }
```

#### Function `pthread_exit`

See the [module comment](self).

```rust
pub fn pthread_exit() { /* ... */ }
```

#### Function `pthread_getaffinity_np`

See the [module comment](self).

```rust
pub fn pthread_getaffinity_np() { /* ... */ }
```

#### Function `pthread_getattr_default_np`

See the [module comment](self).

```rust
pub fn pthread_getattr_default_np() { /* ... */ }
```

#### Function `pthread_getattr_np`

See the [module comment](self).

```rust
pub fn pthread_getattr_np() { /* ... */ }
```

#### Function `pthread_getconcurrency`

See the [module comment](self).

```rust
pub fn pthread_getconcurrency() { /* ... */ }
```

#### Function `pthread_getcpuclockid`

See the [module comment](self).

```rust
pub fn pthread_getcpuclockid() { /* ... */ }
```

#### Function `pthread_getname_np`

See the [module comment](self).

```rust
pub fn pthread_getname_np() { /* ... */ }
```

#### Function `pthread_getschedparam`

See the [module comment](self).

```rust
pub fn pthread_getschedparam() { /* ... */ }
```

#### Function `pthread_getspecific`

See the [module comment](self).

```rust
pub fn pthread_getspecific() { /* ... */ }
```

#### Function `pthread_join`

See the [module comment](self).

```rust
pub fn pthread_join() { /* ... */ }
```

#### Function `pthread_key_create`

See the [module comment](self).

```rust
pub fn pthread_key_create() { /* ... */ }
```

#### Function `pthread_key_delete`

See the [module comment](self).

```rust
pub fn pthread_key_delete() { /* ... */ }
```

#### Function `pthread_kill`

See the [module comment](self).

```rust
pub fn pthread_kill() { /* ... */ }
```

#### Function `pthread_kill_other_threads_np`

See the [module comment](self).

```rust
pub fn pthread_kill_other_threads_np() { /* ... */ }
```

#### Function `pthread_mutexattr_destroy`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_destroy() { /* ... */ }
```

#### Function `pthread_mutexattr_getprioceiling`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_getprioceiling() { /* ... */ }
```

#### Function `pthread_mutexattr_getprotocol`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_getprotocol() { /* ... */ }
```

#### Function `pthread_mutexattr_getpshared`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_getpshared() { /* ... */ }
```

#### Function `pthread_mutexattr_getrobust`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_getrobust() { /* ... */ }
```

#### Function `pthread_mutexattr_getrobust_np`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_getrobust_np() { /* ... */ }
```

#### Function `pthread_mutexattr_gettype`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_gettype() { /* ... */ }
```

#### Function `pthread_mutexattr_init`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_init() { /* ... */ }
```

#### Function `pthread_mutexattr_setprioceiling`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_setprioceiling() { /* ... */ }
```

#### Function `pthread_mutexattr_setprotocol`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_setprotocol() { /* ... */ }
```

#### Function `pthread_mutexattr_setpshared`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_setpshared() { /* ... */ }
```

#### Function `pthread_mutexattr_setrobust`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_setrobust() { /* ... */ }
```

#### Function `pthread_mutexattr_setrobust_np`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_setrobust_np() { /* ... */ }
```

#### Function `pthread_mutexattr_settype`

See the [module comment](self).

```rust
pub fn pthread_mutexattr_settype() { /* ... */ }
```

#### Function `pthread_mutex_consistent`

See the [module comment](self).

```rust
pub fn pthread_mutex_consistent() { /* ... */ }
```

#### Function `pthread_mutex_consistent_np`

See the [module comment](self).

```rust
pub fn pthread_mutex_consistent_np() { /* ... */ }
```

#### Function `pthread_mutex_destroy`

See the [module comment](self).

```rust
pub fn pthread_mutex_destroy() { /* ... */ }
```

#### Function `pthread_mutex_getprioceiling`

See the [module comment](self).

```rust
pub fn pthread_mutex_getprioceiling() { /* ... */ }
```

#### Function `pthread_mutex_init`

See the [module comment](self).

```rust
pub fn pthread_mutex_init() { /* ... */ }
```

#### Function `pthread_mutex_lock`

See the [module comment](self).

```rust
pub fn pthread_mutex_lock() { /* ... */ }
```

#### Function `pthread_mutex_setprioceiling`

See the [module comment](self).

```rust
pub fn pthread_mutex_setprioceiling() { /* ... */ }
```

#### Function `pthread_mutex_timedlock`

See the [module comment](self).

```rust
pub fn pthread_mutex_timedlock() { /* ... */ }
```

#### Function `pthread_mutex_trylock`

See the [module comment](self).

```rust
pub fn pthread_mutex_trylock() { /* ... */ }
```

#### Function `pthread_once`

See the [module comment](self).

```rust
pub fn pthread_once() { /* ... */ }
```

#### Function `pthread_rwlockattr_destroy`

See the [module comment](self).

```rust
pub fn pthread_rwlockattr_destroy() { /* ... */ }
```

#### Function `pthread_rwlockattr_getkind_np`

See the [module comment](self).

```rust
pub fn pthread_rwlockattr_getkind_np() { /* ... */ }
```

#### Function `pthread_rwlockattr_getpshared`

See the [module comment](self).

```rust
pub fn pthread_rwlockattr_getpshared() { /* ... */ }
```

#### Function `pthread_rwlockattr_init`

See the [module comment](self).

```rust
pub fn pthread_rwlockattr_init() { /* ... */ }
```

#### Function `pthread_rwlockattr_setkind_np`

See the [module comment](self).

```rust
pub fn pthread_rwlockattr_setkind_np() { /* ... */ }
```

#### Function `pthread_rwlockattr_setpshared`

See the [module comment](self).

```rust
pub fn pthread_rwlockattr_setpshared() { /* ... */ }
```

#### Function `pthread_rwlock_destroy`

See the [module comment](self).

```rust
pub fn pthread_rwlock_destroy() { /* ... */ }
```

#### Function `pthread_rwlock_rdlock`

See the [module comment](self).

```rust
pub fn pthread_rwlock_rdlock() { /* ... */ }
```

#### Function `pthread_rwlock_timedrdlock`

See the [module comment](self).

```rust
pub fn pthread_rwlock_timedrdlock() { /* ... */ }
```

#### Function `pthread_rwlock_timedwrlock`

See the [module comment](self).

```rust
pub fn pthread_rwlock_timedwrlock() { /* ... */ }
```

#### Function `pthread_rwlock_tryrdlock`

See the [module comment](self).

```rust
pub fn pthread_rwlock_tryrdlock() { /* ... */ }
```

#### Function `pthread_rwlock_trywrlock`

See the [module comment](self).

```rust
pub fn pthread_rwlock_trywrlock() { /* ... */ }
```

#### Function `pthread_rwlock_unlock`

See the [module comment](self).

```rust
pub fn pthread_rwlock_unlock() { /* ... */ }
```

#### Function `pthread_rwlock_wrlock`

See the [module comment](self).

```rust
pub fn pthread_rwlock_wrlock() { /* ... */ }
```

#### Function `pthread_self`

See the [module comment](self).

```rust
pub fn pthread_self() { /* ... */ }
```

#### Function `pthread_setaffinity_np`

See the [module comment](self).

```rust
pub fn pthread_setaffinity_np() { /* ... */ }
```

#### Function `pthread_setattr_default_np`

See the [module comment](self).

```rust
pub fn pthread_setattr_default_np() { /* ... */ }
```

#### Function `pthread_setcancelstate`

See the [module comment](self).

```rust
pub fn pthread_setcancelstate() { /* ... */ }
```

#### Function `pthread_setcanceltype`

See the [module comment](self).

```rust
pub fn pthread_setcanceltype() { /* ... */ }
```

#### Function `pthread_setconcurrency`

See the [module comment](self).

```rust
pub fn pthread_setconcurrency() { /* ... */ }
```

#### Function `pthread_setname_np`

See the [module comment](self).

```rust
pub fn pthread_setname_np() { /* ... */ }
```

#### Function `pthread_setschedparam`

See the [module comment](self).

```rust
pub fn pthread_setschedparam() { /* ... */ }
```

#### Function `pthread_setschedprio`

See the [module comment](self).

```rust
pub fn pthread_setschedprio() { /* ... */ }
```

#### Function `pthread_setspecific`

See the [module comment](self).

```rust
pub fn pthread_setspecific() { /* ... */ }
```

#### Function `pthread_sigmask`

See the [module comment](self).

```rust
pub fn pthread_sigmask() { /* ... */ }
```

#### Function `pthread_sigqueue`

See the [module comment](self).

```rust
pub fn pthread_sigqueue() { /* ... */ }
```

#### Function `pthread_spin_destroy`

See the [module comment](self).

```rust
pub fn pthread_spin_destroy() { /* ... */ }
```

#### Function `pthread_spin_init`

See the [module comment](self).

```rust
pub fn pthread_spin_init() { /* ... */ }
```

#### Function `pthread_spin_lock`

See the [module comment](self).

```rust
pub fn pthread_spin_lock() { /* ... */ }
```

#### Function `pthread_spin_trylock`

See the [module comment](self).

```rust
pub fn pthread_spin_trylock() { /* ... */ }
```

#### Function `pthread_spin_unlock`

See the [module comment](self).

```rust
pub fn pthread_spin_unlock() { /* ... */ }
```

#### Function `pthread_testcancel`

See the [module comment](self).

```rust
pub fn pthread_testcancel() { /* ... */ }
```

#### Function `pthread_timedjoin_np`

See the [module comment](self).

```rust
pub fn pthread_timedjoin_np() { /* ... */ }
```

#### Function `pthread_tryjoin_np`

See the [module comment](self).

```rust
pub fn pthread_tryjoin_np() { /* ... */ }
```

#### Function `pthread_yield`

See the [module comment](self).

```rust
pub fn pthread_yield() { /* ... */ }
```

## Module `higher_level`

Functions which provide higher-level functionality are out of scope for
rustix.

These functions are provided by typical libc implementations, but are
higher-level than the simple syscall-like functions that rustix focuses on.
They could be implemented as a separate library built on top of rustix,
rather than being part of rustix itself. This module contains an incomplete
list of such functions.

```rust
pub mod higher_level { /* ... */ }
```

### Functions

#### Function `getpwent`

See the [module comment](self).

```rust
pub fn getpwent() { /* ... */ }
```

#### Function `getpwuid`

See the [module comment](self).

```rust
pub fn getpwuid() { /* ... */ }
```

#### Function `getpwnam`

See the [module comment](self).

```rust
pub fn getpwnam() { /* ... */ }
```

#### Function `getpwuid_r`

See the [module comment](self).

```rust
pub fn getpwuid_r() { /* ... */ }
```

#### Function `getpwnam_r`

See the [module comment](self).

```rust
pub fn getpwnam_r() { /* ... */ }
```

#### Function `gethostbyname`

See the [module comment](self).

```rust
pub fn gethostbyname() { /* ... */ }
```

#### Function `execv`

See the [module comment](self).

```rust
pub fn execv() { /* ... */ }
```

#### Function `execvp`

See the [module comment](self).

```rust
pub fn execvp() { /* ... */ }
```

#### Function `execvpe`

See the [module comment](self).

```rust
pub fn execvpe() { /* ... */ }
```

#### Function `wordexp`

See the [module comment](self).

```rust
pub fn wordexp() { /* ... */ }
```

#### Function `localtime`

See the [module comment](self).

```rust
pub fn localtime() { /* ... */ }
```

#### Function `localtime_r`

See the [module comment](self).

```rust
pub fn localtime_r() { /* ... */ }
```

#### Function `gmtime`

See the [module comment](self).

```rust
pub fn gmtime() { /* ... */ }
```

#### Function `gmtime_r`

See the [module comment](self).

```rust
pub fn gmtime_r() { /* ... */ }
```

#### Function `ctime`

See the [module comment](self).

```rust
pub fn ctime() { /* ... */ }
```

#### Function `ctime_r`

See the [module comment](self).

```rust
pub fn ctime_r() { /* ... */ }
```

#### Function `asctime`

See the [module comment](self).

```rust
pub fn asctime() { /* ... */ }
```

#### Function `asctime_r`

See the [module comment](self).

```rust
pub fn asctime_r() { /* ... */ }
```

#### Function `mktime`

See the [module comment](self).

```rust
pub fn mktime() { /* ... */ }
```

#### Function `getifaddrs`

See the [module comment](self).

```rust
pub fn getifaddrs() { /* ... */ }
```

#### Function `closefrom`

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

```rust
pub fn closefrom() { /* ... */ }
```

#### Function `login_tty`

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

```rust
pub fn login_tty() { /* ... */ }
```

#### Function `openpty`

See [rustix-openpty](https://crates.io/crates/rustix-openpty).

```rust
pub fn openpty() { /* ... */ }
```

#### Function `isatty`

See [`std::io::IsTerminal`].

For Rust < 1.70, see [is-terminal]. For a rustix-based implementation,
see [rustix-is-terminal].

[`std::io::IsTerminal`]: std::io::IsTerminal
[is-terminal]: https://crates.io/crates/is-terminal
[rustix-is-terminal]: https://crates.io/crates/rustix-is-terminal

```rust
pub fn isatty() { /* ... */ }
```

## Module `impossible`

Functions which don't seem possible to even call from Rust with current
language features, even with `unsafe`.

```rust
pub mod impossible { /* ... */ }
```

### Functions

#### Function `vfork`

See the [module comment](self).

```rust
pub fn vfork() { /* ... */ }
```

#### Function `sigreturn`

See the [module comment](self).

```rust
pub fn sigreturn() { /* ... */ }
```

#### Function `setjmp`

See the [module comment](self).

```rust
pub fn setjmp() { /* ... */ }
```

#### Function `longjmp`

See the [module comment](self).

```rust
pub fn longjmp() { /* ... */ }
```

#### Function `sigsetjmp`

See the [module comment](self).

```rust
pub fn sigsetjmp() { /* ... */ }
```

#### Function `siglongjmp`

See the [module comment](self).

```rust
pub fn siglongjmp() { /* ... */ }
```

## Module `yet`

These functions are not yet implemented in rustix, but probably could be.

These are functions that users have asked about, and which probably are in
scope for rustix, but are not yet implemented. This module contains an
incomplete list of such functions.

```rust
pub mod yet { /* ... */ }
```

### Functions

#### Function `tgkill`

See the [module comment](self).

```rust
pub fn tgkill() { /* ... */ }
```

#### Function `raise`

See the [module comment](self).

```rust
pub fn raise() { /* ... */ }
```

#### Function `sysctl`

See the [module comment](self).

```rust
pub fn sysctl() { /* ... */ }
```

#### Function `mq_open`

See the [module comment](self).

```rust
pub fn mq_open() { /* ... */ }
```

#### Function `mq_send`

See the [module comment](self).

```rust
pub fn mq_send() { /* ... */ }
```

#### Function `mq_unlink`

See the [module comment](self).

```rust
pub fn mq_unlink() { /* ... */ }
```

#### Function `recvmmsg`

See the [module comment](self).

```rust
pub fn recvmmsg() { /* ... */ }
```

#### Function `cachestat`

See the [module comment](self).

```rust
pub fn cachestat() { /* ... */ }
```

#### Function `fanotify_init`

See the [module comment](self).

```rust
pub fn fanotify_init() { /* ... */ }
```

#### Function `fanotify_mark`

See the [module comment](self).

```rust
pub fn fanotify_mark() { /* ... */ }
```

#### Function `getifaddrs`

See the [module comment](self).

```rust
pub fn getifaddrs() { /* ... */ }
```

#### Function `signalfd`

See the [module comment](self).

```rust
pub fn signalfd() { /* ... */ }
```

#### Function `pidfd_send_signal`

See the [module comment](self).

```rust
pub fn pidfd_send_signal() { /* ... */ }
```

#### Function `mount_setattr`

See the [module comment](self).

```rust
pub fn mount_setattr() { /* ... */ }
```

#### Function `extattr_delete_fd`

See the [module comment](self).

```rust
pub fn extattr_delete_fd() { /* ... */ }
```

#### Function `extattr_delete_link`

See the [module comment](self).

```rust
pub fn extattr_delete_link() { /* ... */ }
```

#### Function `extattr_get_fd`

See the [module comment](self).

```rust
pub fn extattr_get_fd() { /* ... */ }
```

#### Function `extattr_get_link`

See the [module comment](self).

```rust
pub fn extattr_get_link() { /* ... */ }
```

#### Function `extattr_list_fd`

See the [module comment](self).

```rust
pub fn extattr_list_fd() { /* ... */ }
```

#### Function `extattr_list_link`

See the [module comment](self).

```rust
pub fn extattr_list_link() { /* ... */ }
```

#### Function `extattr_set_fd`

See the [module comment](self).

```rust
pub fn extattr_set_fd() { /* ... */ }
```

#### Function `extattr_set_link`

See the [module comment](self).

```rust
pub fn extattr_set_link() { /* ... */ }
```

#### Function `get_mempolicy`

See the [module comment](self).

```rust
pub fn get_mempolicy() { /* ... */ }
```

#### Function `mbind`

See the [module comment](self).

```rust
pub fn mbind() { /* ... */ }
```

#### Function `set_mempolicy`

See the [module comment](self).

```rust
pub fn set_mempolicy() { /* ... */ }
```

#### Function `migrate_pages`

See the [module comment](self).

```rust
pub fn migrate_pages() { /* ... */ }
```

#### Function `move_pages`

See the [module comment](self).

```rust
pub fn move_pages() { /* ... */ }
```

#### Function `fchmodat2`

See the [module comment](self).

```rust
pub fn fchmodat2() { /* ... */ }
```

#### Function `shmat`

See the [module comment](self).

```rust
pub fn shmat() { /* ... */ }
```

#### Function `shmdt`

See the [module comment](self).

```rust
pub fn shmdt() { /* ... */ }
```

#### Function `shmget`

See the [module comment](self).

```rust
pub fn shmget() { /* ... */ }
```

#### Function `shmctl`

See the [module comment](self).

```rust
pub fn shmctl() { /* ... */ }
```

## Module `quite_yet`

These functions are not quite yet finished in rustix.

Rustix's codebase includes experimental implementations of these functions,
however they are not yet publicly exposed because their API might need more
work and/or they don't yet have a libc backend implementation yet.

See [#1314] for more information, and please leave comments if there are
specific functions you're interested in.

[#1314]: https://github.com/bytecodealliance/rustix/issues/1314

```rust
pub mod quite_yet { /* ... */ }
```

### Functions

#### Function `_exit`

See the [module comment](self).

```rust
pub fn _exit() { /* ... */ }
```

#### Function `_Exit`

See the [module comment](self).

```rust
pub fn _Exit() { /* ... */ }
```

#### Function `exit_group`

See the [module comment](self).

```rust
pub fn exit_group() { /* ... */ }
```

#### Function `sigpending`

See the [module comment](self).

```rust
pub fn sigpending() { /* ... */ }
```

#### Function `sigsuspend`

See the [module comment](self).

```rust
pub fn sigsuspend() { /* ... */ }
```

#### Function `execveat`

See the [module comment](self).

```rust
pub fn execveat() { /* ... */ }
```

#### Function `execve`

See the [module comment](self).

```rust
pub fn execve() { /* ... */ }
```

#### Function `gethostname`

For now, use `rustix::process::uname().nodename()` instead.

See also the [module comment](self).

```rust
pub fn gethostname() { /* ... */ }
```

## Macros

### Macro `cstr`

**Attributes:**

- `#[allow(unused_macros)]`
- `#[macro_export]`

A macro for [`CStr`] literals.

This can make passing string literals to rustix APIs more efficient, since
most underlying system calls with string arguments expect NUL-terminated
strings, and passing strings to rustix as `CStr`s means that rustix doesn't
need to copy them into a separate buffer to NUL-terminate them.

In Rust ≥ 1.77, users can use [C-string literals] instead of this macro.

[`CStr`]: crate::ffi::CStr
[C-string literals]: https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#c-string-literals

# Examples

```
# #[cfg(feature = "fs")]
# fn main() -> rustix::io::Result<()> {
use rustix::cstr;
use rustix::fs::{statat, AtFlags, CWD};

let metadata = statat(CWD, cstr!("Cargo.toml"), AtFlags::empty())?;
# Ok(())
# }
# #[cfg(not(feature = "fs"))]
# fn main() {}
```

```rust
pub macro_rules! cstr {
    /* macro_rules! cstr {
    ($str:literal) => { ... };
} */
}
```

