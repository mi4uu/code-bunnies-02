# Crate Documentation

**Version:** 1.10.1

**Format Version:** 43

# Module `bytes`

Provides abstractions for working with bytes.

The `bytes` crate provides an efficient byte buffer structure
([`Bytes`]) and traits for working with buffer
implementations ([`Buf`], [`BufMut`]).

# `Bytes`

`Bytes` is an efficient container for storing and operating on contiguous
slices of memory. It is intended for use primarily in networking code, but
could have applications elsewhere as well.

`Bytes` values facilitate zero-copy network programming by allowing multiple
`Bytes` objects to point to the same underlying memory. This is managed by
using a reference count to track when the memory is no longer needed and can
be freed.

A `Bytes` handle can be created directly from an existing byte store (such as `&[u8]`
or `Vec<u8>`), but usually a `BytesMut` is used first and written to. For
example:

```rust
use bytes::{BytesMut, BufMut};

let mut buf = BytesMut::with_capacity(1024);
buf.put(&b"hello world"[..]);
buf.put_u16(1234);

let a = buf.split();
assert_eq!(a, b"hello world\x04\xD2"[..]);

buf.put(&b"goodbye world"[..]);

let b = buf.split();
assert_eq!(b, b"goodbye world"[..]);

assert_eq!(buf.capacity(), 998);
```

In the above example, only a single buffer of 1024 is allocated. The handles
`a` and `b` will share the underlying buffer and maintain indices tracking
the view into the buffer represented by the handle.

See the [struct docs](`Bytes`) for more details.

# `Buf`, `BufMut`

These two traits provide read and write access to buffers. The underlying
storage may or may not be in contiguous memory. For example, `Bytes` is a
buffer that guarantees contiguous memory, but a [rope] stores the bytes in
disjoint chunks. `Buf` and `BufMut` maintain cursors tracking the current
position in the underlying byte storage. When bytes are read or written, the
cursor is advanced.

[rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)

## Relation with `Read` and `Write`

At first glance, it may seem that `Buf` and `BufMut` overlap in
functionality with [`std::io::Read`] and [`std::io::Write`]. However, they
serve different purposes. A buffer is the value that is provided as an
argument to `Read::read` and `Write::write`. `Read` and `Write` may then
perform a syscall, which has the potential of failing. Operations on `Buf`
and `BufMut` are infallible.

## Modules

## Module `buf`

Utilities for working with buffers.

A buffer is any structure that contains a sequence of bytes. The bytes may
or may not be stored in contiguous memory. This module contains traits used
to abstract over buffers as well as utilities for working with buffer types.

# `Buf`, `BufMut`

These are the two foundational traits for abstractly working with buffers.
They can be thought as iterators for byte structures. They offer additional
performance over `Iterator` by providing an API optimized for byte slices.

See [`Buf`] and [`BufMut`] for more details.

[rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)

```rust
pub mod buf { /* ... */ }
```

### Re-exports

#### Re-export `Buf`

```rust
pub use self::buf_impl::Buf;
```

#### Re-export `BufMut`

```rust
pub use self::buf_mut::BufMut;
```

#### Re-export `Chain`

```rust
pub use self::chain::Chain;
```

#### Re-export `IntoIter`

```rust
pub use self::iter::IntoIter;
```

#### Re-export `Limit`

```rust
pub use self::limit::Limit;
```

#### Re-export `Take`

```rust
pub use self::take::Take;
```

#### Re-export `UninitSlice`

```rust
pub use self::uninit_slice::UninitSlice;
```

#### Re-export `Reader`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use self::reader::Reader;
```

#### Re-export `Writer`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use self::writer::Writer;
```

## Types

### Struct `TryGetError`

Error type for the `try_get_` methods of [`Buf`].
Indicates that there were not enough remaining
bytes in the buffer while attempting
to get a value from a [`Buf`] with one
of the `try_get_` methods.

```rust
pub struct TryGetError {
    pub requested: usize,
    pub available: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `requested` | `usize` | The number of bytes necessary to get the value |
| `available` | `usize` | The number of bytes available in the buffer |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TryGetError) -> bool { /* ... */ }
    ```

- **Send**
- **Sync**
- **Error**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> Result<(), core::fmt::Error> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(error: TryGetError) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Re-exports

### Re-export `Buf`

```rust
pub use crate::buf::Buf;
```

### Re-export `BufMut`

```rust
pub use crate::buf::BufMut;
```

### Re-export `Bytes`

```rust
pub use crate::bytes::Bytes;
```

### Re-export `BytesMut`

```rust
pub use crate::bytes_mut::BytesMut;
```

