# Crate Documentation

**Version:** 0.8.10

**Format Version:** 43

# Module `rkyv`

rkyv is a zero-copy deserialization framework for Rust.

## Overview

rkyv uses Rust's powerful trait system to serialize data without reflection.
Many zero-copy deserialization frameworks use external schemas and heavily
restrict the available data types. By contrast, rkyv allows all serialized
types to be defined in code and can serialize a wide variety of types that
other frameworks cannot.

rkyv scales to highly-capable as well as highly-restricted environments. Not
only does rkyv support "no-std" builds for targets without a standard
library implementation, it also supports "no-alloc" builds for targets where
allocations cannot be made.

rkyv supports limited in-place data mutation, and so can access and update
data without ever deserializing back to native types. When rkyv's in-place
mutation is too limited, rkyv also provides ergonomic and performant
deserialization back into native types.

rkyv prioritizes performance, and is one of the fastest serialization
frameworks available. All of rkyv's features can be individually enabled and
disabled, so you only pay for what you use. Additionally, all of rkyv's
zero-copy types are designed to have little to no overhead. In most cases,
rkyv's types will have exactly the same performance as native types.

See the [rkyv book] for guide-level documentation and usage examples.

[rkyv book]: https://rkyv.org

## Components

rkyv has [a hash map implementation] that is built for zero-copy
deserialization, with the same lookup and iteration performance as the
standard library hash maps. The hash map implementation is based on
[Swiss Tables] and uses a target-independent version of FxHash to ensure
that all targets compute the same hashes.

It also has [a B-tree implementation] that has the same performance
characteristics as the standard library B-tree maps. Its compact
representation and localized data storage is best-suited for very large
amounts of data.

rkyv supports [shared pointers] by default, and is able to serialize and
deserialize them without duplicating the underlying data. Shared pointers
which point to the same data when serialized will still point to the same
data when deserialized. By default, rkyv only supports non-cyclic data
structures.

Alongside its [unchecked API], rkyv also provides optional [validation] so
you can ensure safety and data integrity at the cost of some overhead.
Because checking serialized data can generally be done without allocations,
the cost of checking and zero-copy access can be much lower than that of
traditional deserialization.

rkyv is trait-oriented from top to bottom, and is made to be extended with
custom and specialized types. Serialization, deserialization, and
validation traits all accept generic context types, making it easy to add
new capabilities without degrading ergonomics.

[a hash map implementation]: collections::swiss_table::ArchivedHashMap
[Swiss Tables]: https://abseil.io/about/design/swisstables
[a B-tree implementation]: collections::btree_map::ArchivedBTreeMap
[shared pointers]: rc
[unchecked API]: access_unchecked
[validation]: access

## Features

rkyv has several feature flags which can be used to modify its behavior. By
default, rkyv enables the `std`, `alloc`, and `bytecheck` features.

### Format control

These features control how rkyv formats its serialized data. Enabling and
disabling these features may change rkyv's serialized format, and as such
can cause previously-serialized data to become unreadable. Enabling format
control features that are not the default should be considered a breaking
change to rkyv's serialized format.

Binaries should consider explicitly choosing format control options from the
start, even though doing so is not required. This ensures that developers
stay informed about the specific choices being made, and prevents any
unexpected compatibility issues with libraries they depend on.

Libraries should avoid enabling format control features unless they intend
to only support rkyv when those specific format control features are
enabled. In general, libraries should be able to support all format control
options if they use rkyv's exported types and aliases.

#### Endianness

If an endianness feature is not enabled, rkyv will use little-endian byte
ordering by default.

- `little_endian`: Forces data serialization to use little-endian byte
  ordering. This optimizes serialized data for little-endian architectures.
- `big_endian`: Forces data serialization to use big-endian byte ordering.
  This optimizes serialized data for big-endian architectures.

#### Alignment

If an alignment feature is not enabled, rkyv will use aligned primitives by
default.

- `aligned`: Forces data serialization to use aligned primitives. This adds
  alignment requirements for accessing data and prevents rkyv from working
  with unaligned data.
- `unaligned`: Forces data serialization to use unaligned primitives. This
  removes alignment requirements for accessing data and allows rkyv to work
  with unaligned data more easily.

#### Pointer width

If a pointer width feature is not enabled, rkyv will serialize `isize` and
`usize` as 32-bit integers by default.

- `pointer_width_16`: Serializes `isize` and `usize` as 16-bit integers.
  This is intended to be used only for small data sizes and may not handle
  large amounts of data.
- `pointer_width_32`: Serializes `isize` and `usize` as 32-bit integers.
  This is a good choice for most data, and balances the storage overhead
  with support for large data sizes.
- `pointer_width_64`: Serializes `isize` and `usize` as 64-bit integers.
  This is intended to be used only for extremely large data sizes and may
  cause unnecessary data bloat for smaller amounts of data.

### Functionality

These features enable more built-in functionality and provide more powerful
and ergonomic APIs. Enabling and disabling these features does not change
rkyv's serialized format.

- `alloc`: Enables support for the `alloc` crate. Enabled by default.
- `std`: Enables standard library support. Enabled by default.
- `bytecheck`: Enables data validation through `bytecheck`. Enabled by
  default.

### Crates

rkyv provides integrations for some common crates by default. In the future,
crates should depend on rkyv and provide their own integration. Enabling and
disabling these features does not change rkyv's serialized format.

- [`arrayvec-0_7`](https://docs.rs/arrayvec/0.7)
- [`bytes-1`](https://docs.rs/bytes/1)
- [`hashbrown-0_14`](https://docs.rs/hashbrown/0.14)
- [`hashbrown-0_15`](https://docs.rs/hashbrown/0.15)
- [`indexmap-2`](https://docs.rs/indexmap/2)
- [`smallvec-1`](https://docs.rs/smallvec/1)
- [`smol_str-0_2`](https://docs.rs/smol_str/0.2)
- [`smol_str-0_3`](https://docs.rs/smol_str/0.3)
- [`thin-vec-0_2`](https://docs.rs/thin-vec/0.2)
- [`tinyvec-1`](https://docs.rs/tinyvec/1)
- [`triomphe-0_1`](https://docs.rs/triomphe/0.1)
- [`uuid-1`](https://docs.rs/uuid/1)

## Compatibility

Serialized data can be accessed later as long as:

- The underlying schema has not changed
- The serialized format has not been changed by format control features
- The data was serialized by a semver-compatible version of rkyv

## Modules

## Module `api`

APIs for producing and using archived data.

# Accessing byte slices

The safety requirements for accessing a byte slice will often state that a
byte slice must "represent a valid archived type". The specific validity
requirements may vary widely depending on the types being accessed, and so
in general the only way to guarantee that this call is safe is to have
previously validated the byte slice.

Using techniques such as cryptographic signing can provide a more performant
way to verify data integrity from trusted sources.

It is generally safe to assume that unchanged and properly-aligned
serialized bytes are always safe to access without validation. By contrast,
bytes from a potentially-malicious source should always be validated prior
to access.

```rust
pub mod api { /* ... */ }
```

### Modules

## Module `high`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

APIs for environments where allocations can be made.

These APIs have default writers, automatically manage allocators, and
support shared pointers.

```rust
pub mod high { /* ... */ }
```

### Types

#### Type Alias `HighSerializer`

A high-level serializer.

This is part of the [high-level API](crate::api::high).

```rust
pub type HighSerializer<W, A, E> = rancor::Strategy<crate::ser::Serializer<W, A, crate::ser::sharing::Share>, E>;
```

#### Type Alias `HighDeserializer`

A high-level deserializer.

This is part of the [high-level API](crate::api::high).

```rust
pub type HighDeserializer<E> = rancor::Strategy<crate::de::Pool, E>;
```

### Functions

#### Function `to_bytes`

Serialize a value to bytes.

Returns the serialized bytes in an [`AlignedVec`].

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    from_bytes, rancor::Error, to_bytes, Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let deserialized = from_bytes::<Example, Error>(&bytes).unwrap();

assert_eq!(deserialized, value);
```

```rust
pub fn to_bytes<E, /* synthetic */ impl for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, E>>: for<''a> Serialize<HighSerializer<crate::util::AlignedVec, crate::ser::allocator::ArenaHandle<''a>, E>>>(value: &impl for<''a> Serialize<HighSerializer<crate::util::AlignedVec, crate::ser::allocator::ArenaHandle<''a>, E>>) -> Result<crate::util::AlignedVec, E>
where
    E: rancor::Source { /* ... */ }
```

#### Function `to_bytes_in`

Serialize a value and write the bytes to the given writer.

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    api::high::to_bytes_in, from_bytes, rancor::Error, util::AlignedVec,
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes =
    to_bytes_in::<_, Error>(&value, AlignedVec::<8>::new()).unwrap();
let deserialized = from_bytes::<Example, Error>(&bytes).unwrap();

assert_eq!(deserialized, value);
```

```rust
pub fn to_bytes_in<W, E, /* synthetic */ impl for<'a> Serialize<HighSerializer<W, ArenaHandle<'a>, E>>: for<''a> Serialize<HighSerializer<W, crate::ser::allocator::ArenaHandle<''a>, E>>>(value: &impl for<''a> Serialize<HighSerializer<W, crate::ser::allocator::ArenaHandle<''a>, E>>, writer: W) -> Result<W, E>
where
    W: Writer<E>,
    E: rancor::Source { /* ... */ }
```

#### Function `to_bytes_with_alloc`

Serialize a value using the given allocator.

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    api::high::to_bytes_with_alloc, from_bytes, rancor::Error,
    util::with_arena, Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

with_arena(|arena| {
    let bytes =
        to_bytes_with_alloc::<_, Error>(&value, arena.acquire()).unwrap();
    let deserialized = from_bytes::<Example, Error>(&bytes).unwrap();

    assert_eq!(deserialized, value);
});
```

```rust
pub fn to_bytes_with_alloc<A, E, /* synthetic */ impl Serialize<HighSerializer<AlignedVec, A, E>>: Serialize<HighSerializer<crate::util::AlignedVec, A, E>>>(value: &impl Serialize<HighSerializer<crate::util::AlignedVec, A, E>>, alloc: A) -> Result<crate::util::AlignedVec, E>
where
    A: Allocator<E>,
    E: rancor::Source { /* ... */ }
```

#### Function `to_bytes_in_with_alloc`

Serialize a value using the given allocator and write the bytes to the given
writer.

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    api::high::to_bytes_in_with_alloc,
    from_bytes,
    rancor::Error,
    util::{with_arena, AlignedVec},
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

with_arena(|arena| {
    let bytes = to_bytes_in_with_alloc::<_, _, Error>(
        &value,
        AlignedVec::<8>::new(),
        arena.acquire(),
    )
    .expect("failed to serialize vec");

    let deserialized = from_bytes::<Example, Error>(&bytes)
        .expect("failed to deserialize vec");

    assert_eq!(deserialized, value);
});
```

```rust
pub fn to_bytes_in_with_alloc<W, A, E, /* synthetic */ impl Serialize<HighSerializer<W, A, E>>: Serialize<HighSerializer<W, A, E>>>(value: &impl Serialize<HighSerializer<W, A, E>>, writer: W, alloc: A) -> Result<W, E>
where
    W: Writer<E>,
    A: Allocator<E>,
    E: rancor::Source { /* ... */ }
```

#### Function `from_bytes_unchecked`

Deserialize a value from the given bytes.

This function does not check that the data is valid. Use [`from_bytes`] to
validate the data instead.

This is part of the [high-level API](crate::api::high).

# Safety

The byte slice must represent a valid archived type when accessed at the
default root position. See the [module docs](crate::api) for more
information.

# Example

```
use rkyv::{
    from_bytes_unchecked, rancor::Error, to_bytes, Archive, Deserialize,
    Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let deserialized =
    unsafe { from_bytes_unchecked::<Example, Error>(&bytes).unwrap() };

assert_eq!(deserialized, value);
```

```rust
pub unsafe fn from_bytes_unchecked<T, E>(bytes: &[u8]) -> Result<T, E>
where
    T: Archive,
    <T as >::Archived: Deserialize<T, HighDeserializer<E>> { /* ... */ }
```

#### Function `deserialize`

Deserialize a value from the given archived value.

This is part of the [high-level API](crate::api::high).

# Example

```
use rkyv::{
    access, deserialize, rancor::Error, to_bytes, Archive, Deserialize,
    Serialize,
};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let archived = access::<ArchivedExample, Error>(&*bytes).unwrap();
let deserialized = deserialize::<Example, Error>(archived).unwrap();

assert_eq!(deserialized, value);
```

```rust
pub fn deserialize<T, E, /* synthetic */ impl Deserialize<T, HighDeserializer<E>>: Deserialize<T, HighDeserializer<E>>>(value: &impl Deserialize<T, HighDeserializer<E>>) -> Result<T, E> { /* ... */ }
```

### Re-exports

#### Re-export `self::checked::*`

**Attributes:**

- `#[<cfg>(feature = "bytecheck")]`

```rust
pub use self::checked::*;
```

## Module `low`

APIs for environments where allocations cannot be made.

These APIs require user-provided writers and allocators, and do not support
shared pointers.

```rust
pub mod low { /* ... */ }
```

### Types

#### Type Alias `LowSerializer`

A general-purpose serializer suitable for environments where allocations
cannot be made.

This is part of the [low-level API](crate::api::low).

```rust
pub type LowSerializer<W, A, E> = rancor::Strategy<crate::ser::Serializer<W, A, ()>, E>;
```

#### Type Alias `LowDeserializer`

A general-purpose deserializer suitable for environments where allocations
cannot be made.

This is part of the [low-level API](crate::api::low).

```rust
pub type LowDeserializer<E> = rancor::Strategy<(), E>;
```

### Functions

#### Function `to_bytes_in_with_alloc`

Serialize a value using the given allocator and write the bytes to the given
writer.

This is part of the [low-level API](crate::api::low).

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    access_unchecked,
    api::low::to_bytes_in_with_alloc,
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    with::InlineAsBox,
    Archive, Serialize,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize)]
struct Example<'a> {
    #[rkyv(with = InlineAsBox)]
    inner: &'a i32,
}

let forty_two = 42;
let value = Example { inner: &forty_two };

let bytes = to_bytes_in_with_alloc::<_, _, Failure>(
    &value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut alloc),
)
.unwrap();

let archived = unsafe { access_unchecked::<ArchivedExample<'_>>(&*bytes) };
assert_eq!(*archived.inner, 42);
```

```rust
pub fn to_bytes_in_with_alloc<W, A, E, /* synthetic */ impl Serialize<LowSerializer<W, A, E>>: Serialize<LowSerializer<W, A, E>>>(value: &impl Serialize<LowSerializer<W, A, E>>, writer: W, alloc: A) -> Result<W, E>
where
    W: Writer<E>,
    A: Allocator<E>,
    E: rancor::Source { /* ... */ }
```

#### Function `from_bytes_unchecked`

Deserialize a value from the given bytes.

This function does not check that the data is valid. Use [`from_bytes`] to
validate the data instead.

This is part of the [low-level API](crate::api::low).

# Safety

The byte slice must represent a valid archived type when accessed at the
default root position. See the [module docs](crate::api) for more
information.

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    access_unchecked,
    api::low::{from_bytes_unchecked, to_bytes_in_with_alloc},
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    with::InlineAsBox,
    Archive, Deserialize, Serialize,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    inner: i32,
}

let value = Example { inner: 42 };

let bytes = to_bytes_in_with_alloc::<_, _, Failure>(
    &value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut alloc),
)
.unwrap();

let deserialized =
    unsafe { from_bytes_unchecked::<Example, Failure>(&*bytes).unwrap() };
assert_eq!(value, deserialized);
```

```rust
pub unsafe fn from_bytes_unchecked<T, E>(bytes: &[u8]) -> Result<T, E>
where
    T: Archive,
    <T as >::Archived: Deserialize<T, LowDeserializer<E>> { /* ... */ }
```

#### Function `deserialize`

Deserialize a value from the given archived value.

This is part of the [low-level API](crate::api::low).

# Example

```
use core::mem::MaybeUninit;

use rkyv::{
    access_unchecked,
    api::low::{deserialize, to_bytes_in_with_alloc},
    rancor::Failure,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
    with::InlineAsBox,
    Archive, Deserialize, Serialize,
};

let mut output = Align([MaybeUninit::<u8>::uninit(); 256]);
let mut alloc = [MaybeUninit::<u8>::uninit(); 256];

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    inner: i32,
}

let value = Example { inner: 42 };

let bytes = to_bytes_in_with_alloc::<_, _, Failure>(
    &value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut alloc),
)
.unwrap();

let archived = unsafe { access_unchecked::<ArchivedExample>(&*bytes) };
let deserialized = deserialize::<Example, Failure>(archived).unwrap();
assert_eq!(value, deserialized);
```

```rust
pub fn deserialize<T, E, /* synthetic */ impl Deserialize<T, LowDeserializer<E>>: Deserialize<T, LowDeserializer<E>>>(value: &impl Deserialize<T, LowDeserializer<E>>) -> Result<T, E> { /* ... */ }
```

### Re-exports

#### Re-export `self::checked::*`

**Attributes:**

- `#[<cfg>(feature = "bytecheck")]`

```rust
pub use self::checked::*;
```

### Functions

#### Function `root_position`

Return the position of the root within a buffer of `length` bytes.

Most accessing functions have a variant which automatically calculates this
value for you. For example, prefer to call [`access_unchecked`] over
[`access_pos_unchecked`].

The root position of a buffer is calculated by subtracing the size of the
root object from the end of the buffer. If the buffer size is too small to
accomodate a root of the given type, then this function will return zero.

# Example

```
use rkyv::{api::root_position, Archive};

#[derive(Archive)]
pub struct MyData {
    inner: u32,
}

assert_eq!(size_of::<ArchivedMyData>(), 4);

// This is too small, and so returns 0
assert_eq!(root_position::<ArchivedMyData>(3), 0);
assert_eq!(root_position::<ArchivedMyData>(4), 0);
assert_eq!(root_position::<ArchivedMyData>(5), 1);
```

```rust
pub fn root_position<T: Portable>(size: usize) -> usize { /* ... */ }
```

#### Function `access_pos_unchecked`

Access a byte slice with a given root position.

Most of the time, the root position should be calculated using the root type
and size of the buffer. Prefer [`access_unchecked`] whenever possible.

While the root of the archived data is located at the given position, the
reachable data may be located throughout the byte slice.

This function does not check that the bytes are valid to access. Use
[`access_pos`](high::access_pos) to safely access the buffer using
validation.

# Safety

The byte slice must represent a valid archived type when accessed with the
given root position. See the [module docs](crate::api) for more information.

# Example

```
use rkyv::{
    api::{access_pos_unchecked, root_position},
    rancor::Error,
    to_bytes, Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();

let archived = unsafe {
    access_pos_unchecked::<ArchivedExample>(
        &*bytes,
        root_position::<ArchivedExample>(bytes.len()),
    )
};
assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);
```

```rust
pub unsafe fn access_pos_unchecked<T: Portable>(bytes: &[u8], pos: usize) -> &T { /* ... */ }
```

#### Function `access_pos_unchecked_mut`

Mutably access a byte slice with a given root position.

Most of the time, the root position should be calculated using the root type
and size of the buffer. Prefer [`access_unchecked_mut`] whenever possible.

While the root of the archived data is located at the given position, the
reachable data may be located throughout the byte slice.

This function does not check that the bytes are valid to access. Use
[`access_pos_mut`](high::access_pos_mut) to safely access the buffer using
validation.

The returned `Seal` restricts the mutating operations that may be safely
performed on the returned reference. See [`Seal`] for more information.

# Safety

The byte slice must represent a valid archived type when accessed with the
given root position. See the [module docs](crate::api) for more information.

# Example

```
use rkyv::{
    to_bytes, api::{root_position, access_pos_unchecked_mut}, util::Align,
    Archive, Serialize, Deserialize, munge::munge, rancor::Error,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let mut bytes = to_bytes::<Error>(&value).unwrap();
let root_pos = root_position::<ArchivedExample>(bytes.len());

let mut archived = unsafe {
    access_pos_unchecked_mut::<ArchivedExample>(&mut *bytes, root_pos)
};
assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);

// Because the access is mutable, we can mutate the archived data
munge!(let ArchivedExample { mut value, .. } = archived);
assert_eq!(*value, 31415926);
*value = 12345.into();
assert_eq!(*value, 12345);
```

```rust
pub unsafe fn access_pos_unchecked_mut<T: Portable>(bytes: &mut [u8], pos: usize) -> crate::seal::Seal<''_, T> { /* ... */ }
```

#### Function `access_unchecked`

Access a byte slice.

This function does not check that the bytes are valid to access. Use
[`access`](high::access) to safely access the buffer using validation.

# Safety

The byte slice must represent a valid archived type when accessed at the
default root position. See the [module docs](crate::api) for more
information.

# Example

```
use rkyv::{
    access_unchecked, rancor::Error, to_bytes, Archive, Deserialize,
    Serialize,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();

let archived = unsafe { access_unchecked::<ArchivedExample>(&*bytes) };
assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);
```

```rust
pub unsafe fn access_unchecked<T: Portable>(bytes: &[u8]) -> &T { /* ... */ }
```

#### Function `access_unchecked_mut`

Mutably access a byte slice.

This function does not check that the bytes are valid to access. Use
[`access_mut`](high::access_mut) to safely access the buffer using
validation.

# Safety

The byte slice must represent a valid archived type when accessed at the
default root position. See the [module docs](crate::api) for more
information.

# Example

```
use rkyv::{
    to_bytes, access_unchecked_mut, util::Align, Archive,
    munge::munge, Serialize, Deserialize, rancor::Error,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let mut bytes = to_bytes::<Error>(&value).unwrap();

let mut archived = unsafe {
    access_unchecked_mut::<ArchivedExample>(&mut *bytes)
};
assert_eq!(archived.name, "pi");
assert_eq!(archived.value, 31415926);

// Because the access is mutable, we can mutate the archived data
munge!(let ArchivedExample { mut value, .. } = archived);
assert_eq!(*value, 31415926);
*value = 12345.into();
assert_eq!(*value, 12345);
```

```rust
pub unsafe fn access_unchecked_mut<T: Portable>(bytes: &mut [u8]) -> crate::seal::Seal<''_, T> { /* ... */ }
```

#### Function `serialize_using`

Serialize a value using the given serializer.

Returns the position of the serialized value.

Most of the time, [`to_bytes`](high::to_bytes) is a more ergonomic way to
serialize a value to bytes.

# Example

```
use rkyv::{
    access,
    api::serialize_using,
    rancor::Error,
    ser::{sharing::Share, Serializer},
    util::{with_arena, AlignedVec},
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let bytes = with_arena(|arena| {
    let mut serializer = Serializer::new(
        AlignedVec::<4>::new(),
        arena.acquire(),
        Share::new(),
    );

    let value = Example {
        name: "pi".to_string(),
        value: 31415926,
    };

    serialize_using::<_, Error>(&value, &mut serializer).unwrap();
    serializer.into_writer()
});

let archived = access::<ArchivedExample, Error>(&*bytes).unwrap();
assert_eq!(archived.value, 31415926);
```

```rust
pub fn serialize_using<S, E, /* synthetic */ impl SerializeUnsized<Strategy<S, E>>: SerializeUnsized<rancor::Strategy<S, E>>>(value: &impl SerializeUnsized<rancor::Strategy<S, E>>, serializer: &mut S) -> Result<usize, E>
where
    S: Writer<E> + ?Sized { /* ... */ }
```

#### Function `deserialize_using`

Deserialize a value using the given deserializer.

Most of the time, [`deserialize`](high::deserialize) is a more ergonomic way
to deserialize an archived value.

# Example

```
use rkyv::{
    access, api::deserialize_using, de::Pool, rancor::Error, to_bytes,
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize)]
struct Example {
    name: String,
    value: i32,
}

let value = Example {
    name: "pi".to_string(),
    value: 31415926,
};

let bytes = to_bytes::<Error>(&value).unwrap();
let archived = access::<ArchivedExample, Error>(&bytes).unwrap();
let deserialized =
    deserialize_using::<Example, _, Error>(archived, &mut Pool::new())
        .unwrap();
```

```rust
pub fn deserialize_using<T, D, E, /* synthetic */ impl Deserialize<T, Strategy<D, E>>: Deserialize<T, rancor::Strategy<D, E>>>(value: &impl Deserialize<T, rancor::Strategy<D, E>>, deserializer: &mut D) -> Result<T, E> { /* ... */ }
```

### Re-exports

#### Re-export `self::checked::*`

**Attributes:**

- `#[<cfg>(feature = "bytecheck")]`

```rust
pub use self::checked::*;
```

## Module `boxed`

An archived version of `Box`.

```rust
pub mod boxed { /* ... */ }
```

### Types

#### Struct `ArchivedBox`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`

An archived [`Box`].

This is a thin `#[repr(transparent)]` wrapper around a [`RelPtr`] to the
archived type.

```rust
pub struct ArchivedBox<T: ArchivePointee + ?Sized> {
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
  pub fn get(self: &Self) -> &T { /* ... */ }
  ```
  Returns a reference to the value of this archived box.

- ```rust
  pub fn get_seal(this: Seal<''_, Self>) -> Seal<''_, T> { /* ... */ }
  ```
  Returns a sealed mutable reference to the value of this archived box.

- ```rust
  pub fn resolve_from_ref<U: ArchiveUnsized<Archived = T> + ?Sized>(value: &U, resolver: BoxResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived box from the given value and parameters.

- ```rust
  pub fn serialize_from_ref<U, S>(value: &U, serializer: &mut S) -> Result<BoxResolver, <S as >::Error>
where
    U: SerializeUnsized<S, Archived = T> + ?Sized,
    S: Fallible + ?Sized { /* ... */ }
  ```
  Serializes an archived box from the given value and serializer.

- ```rust
  pub fn resolve_from_raw_parts(resolver: BoxResolver, metadata: <T as >::ArchivedMetadata, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived box from a [`BoxResolver`] and the raw metadata

###### Trait Implementations

- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedBox<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedBox<T>>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedBox<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedBox<T>>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Box<U>) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<Box<T>, <D as >::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Freeze**
- **Pointer**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedBox<U>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Box<U>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Receiver**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **Portable**
- **Sync**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedBox<<F as >::Archived>, deserializer: &mut D) -> Result<F, <D as >::Error> { /* ... */ }
    ```

#### Struct `BoxResolver`

The resolver for `Box`.

```rust
pub struct BoxResolver {
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
  pub fn from_pos(pos: usize) -> Self { /* ... */ }
  ```
  Creates a new [`BoxResolver`] from the position of a serialized value.

###### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **Send**
- **Sync**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `collections`

Archived versions of standard library containers.

```rust
pub mod collections { /* ... */ }
```

### Modules

## Module `btree_map`

[`Archive`](crate::Archive) implementation for B-tree maps.

```rust
pub mod btree_map { /* ... */ }
```

### Types

#### Struct `ArchivedBTreeMap`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`BTreeMap`](crate::alloc::collections::BTreeMap).

```rust
pub struct ArchivedBTreeMap<K, V, const E: usize = 5> {
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
  pub fn iter(self: &Self) -> Iter<''_, K, V, E> { /* ... */ }
  ```
  Gets an iterator over the entries of the map, sorted by key.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> IterSeal<''_, K, V, E> { /* ... */ }
  ```
  Gets a mutable iterator over the entires of the map, sorted by key.

- ```rust
  pub fn keys(self: &Self) -> Keys<''_, K, V, E> { /* ... */ }
  ```
  Gets an iterator over the sorted keys of the map.

- ```rust
  pub fn values(self: &Self) -> Values<''_, K, V, E> { /* ... */ }
  ```
  Gets an iterator over the values of the map.

- ```rust
  pub fn values_seal(this: Seal<''_, Self>) -> ValuesSeal<''_, K, V, E> { /* ... */ }
  ```
  Gets a mutable iterator over the values of the map.

- ```rust
  pub fn contains_key<Q>(self: &Self, key: &Q) -> bool
where
    Q: Ord + ?Sized,
    K: Borrow<Q> + Ord { /* ... */ }
  ```
  Returns whether the B-tree map contains the given key.

- ```rust
  pub fn get<Q>(self: &Self, key: &Q) -> Option<&V>
where
    Q: Ord + ?Sized,
    K: Borrow<Q> + Ord { /* ... */ }
  ```
  Returns the value associated with the given key, or `None` if the key is

- ```rust
  pub fn get_seal<''a, Q>(this: Seal<''a, Self>, key: &Q) -> Option<Seal<''a, V>>
where
    Q: Ord + ?Sized,
    K: Borrow<Q> + Ord { /* ... */ }
  ```
  Returns the mutable value associated with the given key, or `None` if

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the B-tree map contains no entries.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of entries in the B-tree map.

- ```rust
  pub fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&K, &V)>
where
    Q: Ord + ?Sized,
    K: Borrow<Q> + Ord { /* ... */ }
  ```
  Gets the key-value pair associated with the given key, or `None` if the

- ```rust
  pub fn get_key_value_seal<''a, Q>(this: Seal<''a, Self>, key: &Q) -> Option<(&''a K, Seal<''a, V>)>
where
    Q: Ord + ?Sized,
    K: Borrow<Q> + Ord { /* ... */ }
  ```
  Gets the mutable key-value pair associated with the given key, or `None`

- ```rust
  pub fn resolve_from_len(len: usize, resolver: BTreeMapResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedBTreeMap` from the given length, resolver, and

- ```rust
  pub fn serialize_from_ordered_iter<I, BKU, BVU, KU, VU, S>(iter: I, serializer: &mut S) -> Result<BTreeMapResolver, <S as >::Error>
where
    I: ExactSizeIterator<Item = (BKU, BVU)>,
    BKU: Borrow<KU>,
    BVU: Borrow<VU>,
    KU: Serialize<S, Archived = K>,
    VU: Serialize<S, Archived = V>,
    S: Fallible + Allocator + Writer + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an `ArchivedBTreeMap` from the given iterator and serializer.

- ```rust
  pub fn visit<T, /* synthetic */ impl FnMut(&K, &V) -> ControlFlow<T>: FnMut(&K, &V) -> ControlFlow<T>>(self: &Self, f: impl FnMut(&K, &V) -> ControlFlow<T>) -> Option<T> { /* ... */ }
  ```
  Visits every key-value pair in the B-tree with a function.

- ```rust
  pub fn visit_seal<T, /* synthetic */ impl FnMut(&K, Seal<'_, V>) -> ControlFlow<T>: FnMut(&K, Seal<''_, V>) -> ControlFlow<T>>(this: Seal<''_, Self>, f: impl FnMut(&K, Seal<''_, V>) -> ControlFlow<T>) -> Option<T> { /* ... */ }
  ```
  Visits every mutable key-value pair in the B-tree with a function.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, key: &Q) -> &<Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedBTreeMap<K, V, E2>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &BTreeMap<K, V>) -> bool { /* ... */ }
    ```

- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<BTreeMap<K, V>, <D as >::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedBTreeMap<<A as ArchiveWith<K>>::Archived, <B as ArchiveWith<V>>::Archived>, deserializer: &mut D) -> Result<BTreeMap<K, V>, <D as Fallible>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Portable**
- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

#### Struct `BTreeMapResolver`

The resolver for [`ArchivedBTreeMap`].

```rust
pub struct BTreeMapResolver {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Pointee**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `btree_set`

[`Archive`](crate::Archive) implementation for B-tree sets.

```rust
pub mod btree_set { /* ... */ }
```

### Types

#### Struct `ArchivedBTreeSet`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`

An archived `BTreeSet`. This is a wrapper around a B-tree map with the same
key and a value of `()`.

```rust
pub struct ArchivedBTreeSet<K, const E: usize = 5>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn contains_key<Q: Ord + ?Sized>(self: &Self, key: &Q) -> bool
where
    K: Borrow<Q> + Ord { /* ... */ }
  ```
  Returns `true` if the set contains a value for the specified key.

- ```rust
  pub fn get<Q: Ord + ?Sized>(self: &Self, value: &Q) -> Option<&K>
where
    K: Borrow<Q> + Ord { /* ... */ }
  ```
  Returns a reference to the value in the set, if any, that is equal to

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the set contains no elements.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of items in the archived B-tree set.

- ```rust
  pub fn resolve_from_len(len: usize, resolver: BTreeSetResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves a B-tree set from its length.

- ```rust
  pub fn serialize_from_ordered_iter<I, KU, S>(iter: I, serializer: &mut S) -> Result<BTreeSetResolver, <S as >::Error>
where
    I: ExactSizeIterator,
    <I as >::Item: Borrow<KU>,
    KU: Serialize<S, Archived = K>,
    S: Fallible + Allocator + Writer + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an `ArchivedBTreeSet` from the given iterator and serializer.

- ```rust
  pub fn visit<T, /* synthetic */ impl FnMut(&K) -> ControlFlow<T>: FnMut(&K) -> ControlFlow<T>>(self: &Self, f: impl FnMut(&K) -> ControlFlow<T>) -> Option<T> { /* ... */ }
  ```
  Visits every key in the B-tree with a function.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<BTreeSet<K>, <D as >::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Portable**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BTreeSet<K>) -> bool { /* ... */ }
    ```

#### Struct `BTreeSetResolver`

The resolver for archived B-tree sets.

```rust
pub struct BTreeSetResolver(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Pointee**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
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
- **UnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `swiss_table`

SwissTable-based implementation for archived hash map and hash set.

```rust
pub mod swiss_table { /* ... */ }
```

### Modules

## Module `index_map`

An archived index map implementation based on Google's high-performance
SwissTable hash map.

```rust
pub mod index_map { /* ... */ }
```

### Types

#### Struct `ArchivedIndexMap`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived `IndexMap`.

```rust
pub struct ArchivedIndexMap<K, V, H = crate::hash::FxHasher64> {
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
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the map contains no elements.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, K, V> { /* ... */ }
  ```
  Returns an iterator over the key-value pairs of the map in order

- ```rust
  pub fn keys(self: &Self) -> Keys<''_, K, V> { /* ... */ }
  ```
  Returns an iterator over the keys of the map in order

- ```rust
  pub const fn len(self: &Self) -> usize { /* ... */ }
  ```
  Gets the number of items in the index map.

- ```rust
  pub fn values(self: &Self) -> Values<''_, K, V> { /* ... */ }
  ```
  Returns an iterator over the values of the map in order.

- ```rust
  pub fn get_full_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<(usize, &K, &V)>
where
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Gets the index, key, and value corresponding to the supplied key using

- ```rust
  pub fn get_full<Q>(self: &Self, key: &Q) -> Option<(usize, &K, &V)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Gets the index, key, and value corresponding to the supplied key.

- ```rust
  pub fn get_key_value_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<(&K, &V)>
where
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Returns the key-value pair corresponding to the supplied key using the

- ```rust
  pub fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&K, &V)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns the key-value pair corresponding to the supplied key.

- ```rust
  pub fn get_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<&V>
where
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Returns a reference to the value corresponding to the supplied key using

- ```rust
  pub fn get<Q>(self: &Self, key: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a reference to the value corresponding to the supplied key.

- ```rust
  pub fn get_full_seal_with<''a, Q, C>(this: Seal<''a, Self>, key: &Q, cmp: C) -> Option<(usize, &''a K, Seal<''a, V>)>
where
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Gets the mutable index, key, and value corresponding to the supplied key

- ```rust
  pub fn get_full_seal<''a, Q>(this: Seal<''a, Self>, key: &Q) -> Option<(usize, &''a K, Seal<''a, V>)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Gets the mutable index, key, and value corresponding to the supplied

- ```rust
  pub fn get_key_value_seal_with<''a, Q, C>(this: Seal<''a, Self>, key: &Q, cmp: C) -> Option<(&''a K, Seal<''a, V>)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Returns the mutable key-value pair corresponding to the supplied key

- ```rust
  pub fn get_key_value_seal<''a, Q>(this: Seal<''a, Self>, key: &Q) -> Option<(&''a K, Seal<''a, V>)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns the mutable key-value pair corresponding to the supplied key.

- ```rust
  pub fn get_seal_with<''a, Q, C>(this: Seal<''a, Self>, key: &Q, cmp: C) -> Option<Seal<''a, V>>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Returns a mutable reference to the value corresponding to the supplied

- ```rust
  pub fn get_seal<''a, Q>(this: Seal<''a, Self>, key: &Q) -> Option<Seal<''a, V>>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a mutable reference to the value corresponding to the supplied

- ```rust
  pub fn contains_key<Q>(self: &Self, key: &Q) -> bool
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns whether a key is present in the hash map.

- ```rust
  pub fn get_index(self: &Self, index: usize) -> Option<(&K, &V)> { /* ... */ }
  ```
  Gets a key-value pair by index.

- ```rust
  pub fn get_index_of_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<usize>
where
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Gets the index of a key if it exists in the map using the given

- ```rust
  pub fn get_index_of<Q>(self: &Self, key: &Q) -> Option<usize>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Gets the index of a key if it exists in the map.

- ```rust
  pub fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: IndexMapResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived index map from a given length and parameters.

- ```rust
  pub fn serialize_from_iter<I, BKU, BVU, KU, VU, S>(iter: I, load_factor: (usize, usize), serializer: &mut S) -> Result<IndexMapResolver, <S as >::Error>
where
    I: Clone + ExactSizeIterator<Item = (BKU, BVU)>,
    BKU: Borrow<KU>,
    BVU: Borrow<VU>,
    KU: Serialize<S, Archived = K> + Hash + Eq,
    VU: Serialize<S, Archived = V>,
    S: Fallible + Writer + Allocator + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an iterator of key-value pairs as an index map.

###### Trait Implementations

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Unpin**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Portable**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Eq**
- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as Fallible>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Iter`

An iterator over the key-value pairs of an index map.

```rust
pub struct Iter<''a, K, V> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **FusedIterator**
- **Unpin**
- **Sync**
- **ExactSizeIterator**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
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

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `Keys`

An iterator over the keys of an index map.

```rust
pub struct Keys<''a, K, V> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **FusedIterator**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Freeze**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
- **Sync**
- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
#### Struct `Values`

An iterator over the values of an index map.

```rust
pub struct Values<''a, K, V> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **FusedIterator**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Pointee**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `IndexMapResolver`

The resolver for an `IndexMap`.

```rust
pub struct IndexMapResolver {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

## Module `index_set`

An archived index set implementation based on Google's high-performance
SwissTable hash map.

```rust
pub mod index_set { /* ... */ }
```

### Types

#### Struct `ArchivedIndexSet`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`

An archived `IndexSet`.

```rust
pub struct ArchivedIndexSet<K, H = crate::hash::FxHasher64> {
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
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the index set contains no values.

- ```rust
  pub fn iter(self: &Self) -> Keys<''_, K, ()> { /* ... */ }
  ```
  Returns an iterator over the keys of the index set in order.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the index set.

- ```rust
  pub fn contains<Q>(self: &Self, k: &Q) -> bool
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns whether a key is present in the hash set.

- ```rust
  pub fn get<Q>(self: &Self, k: &Q) -> Option<&K>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns the value stored in the set, if any.

- ```rust
  pub fn get_full<Q>(self: &Self, k: &Q) -> Option<(usize, &K)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns the item index and value stored in the set, if any.

- ```rust
  pub fn get_index(self: &Self, index: usize) -> Option<&K> { /* ... */ }
  ```
  Gets a key by index.

- ```rust
  pub fn get_index_of<Q>(self: &Self, key: &Q) -> Option<usize>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns the index of a key if it exists in the set.

- ```rust
  pub fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: IndexSetResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived index map from a given length and parameters.

- ```rust
  pub fn serialize_from_iter<I, UK, S>(iter: I, load_factor: (usize, usize), serializer: &mut S) -> Result<IndexSetResolver, <S as >::Error>
where
    I: Clone + ExactSizeIterator,
    <I as >::Item: Borrow<UK>,
    UK: Serialize<S, Archived = K> + Hash + Eq,
    S: Fallible + Writer + Allocator + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an iterator of keys as an index set.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Pointee**
- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Freeze**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Portable**
- **RefUnwindSafe**
#### Struct `IndexSetResolver`

The resolver for archived index sets.

```rust
pub struct IndexSetResolver(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
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

- **Pointee**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Freeze**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
## Module `map`

Archived hash map implementation using an archived SwissTable.

```rust
pub mod map { /* ... */ }
```

### Types

#### Struct `ArchivedHashMap`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`

An archived SwissTable hash map.

```rust
pub struct ArchivedHashMap<K, V, H = crate::hash::FxHasher64> {
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
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the hash map is empty.

- ```rust
  pub const fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the hash map.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total capacity of the hash map.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, K, V, H> { /* ... */ }
  ```
  Returns an iterator over the key-value entries in the hash map.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> IterMut<''_, K, V, H> { /* ... */ }
  ```
  Returns an iterator over the sealed key-value entries in the hash map.

- ```rust
  pub fn keys(self: &Self) -> Keys<''_, K, V, H> { /* ... */ }
  ```
  Returns an iterator over the keys in the hash map.

- ```rust
  pub fn values(self: &Self) -> Values<''_, K, V, H> { /* ... */ }
  ```
  Returns an iterator over the values in the hash map.

- ```rust
  pub fn values_seal(this: Seal<''_, Self>) -> ValuesMut<''_, K, V, H> { /* ... */ }
  ```
  Returns an iterator over the mutable values in the hash map.

- ```rust
  pub fn get_key_value_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<(&K, &V)>
where
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Returns the key-value pair corresponding to the supplied key using the

- ```rust
  pub fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&K, &V)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns the key-value pair corresponding to the supplied key.

- ```rust
  pub fn get_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<&V>
where
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Returns a reference to the value corresponding to the supplied key using

- ```rust
  pub fn get<Q>(self: &Self, key: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a reference to the value corresponding to the supplied key.

- ```rust
  pub fn get_key_value_seal_with<''a, Q, C>(this: Seal<''a, Self>, key: &Q, cmp: C) -> Option<(&''a K, Seal<''a, V>)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Returns the mutable key-value pair corresponding to the supplied key

- ```rust
  pub fn get_key_value_seal<''a, Q>(this: Seal<''a, Self>, key: &Q) -> Option<(&''a K, Seal<''a, V>)>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns the mutable key-value pair corresponding to the supplied key.

- ```rust
  pub fn get_seal_with<''a, Q, C>(this: Seal<''a, Self>, key: &Q, cmp: C) -> Option<Seal<''a, V>>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    C: Fn(&Q, &K) -> bool { /* ... */ }
  ```
  Returns a mutable reference to the value corresponding to the supplied

- ```rust
  pub fn get_seal<''a, Q>(this: Seal<''a, Self>, key: &Q) -> Option<Seal<''a, V>>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a mutable reference to the value corresponding to the supplied

- ```rust
  pub fn contains_key<Q>(self: &Self, key: &Q) -> bool
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns whether the hash map contains the given key.

- ```rust
  pub fn serialize_from_iter<I, BKU, BVU, KU, VU, S>(iter: I, load_factor: (usize, usize), serializer: &mut S) -> Result<HashMapResolver, <S as >::Error>
where
    I: Clone + ExactSizeIterator<Item = (BKU, BVU)>,
    BKU: Borrow<KU>,
    BVU: Borrow<VU>,
    KU: Serialize<S, Archived = K> + Hash + Eq,
    VU: Serialize<S, Archived = V>,
    S: Fallible + Writer + Allocator + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an iterator of key-value pairs as a hash map.

- ```rust
  pub fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: HashMapResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived hash map from a given length and parameters.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &HashMap<K, V, S>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedHashMap<AK, AV>) -> bool { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<HashMap<K, V, S>, <D as >::Error> { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedHashMap<<A as ArchiveWith<K>>::Archived, <B as ArchiveWith<V>>::Archived>, deserializer: &mut D) -> Result<HashMap<K, V, S>, <D as Fallible>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
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

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, key: &Q) -> &V { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Portable**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
#### Struct `HashMapResolver`

The resolver for [`ArchivedHashMap`].

```rust
pub struct HashMapResolver(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Iter`

An iterator over the key-value pairs of an [`ArchivedHashMap`].

```rust
pub struct Iter<''a, K, V, H> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **FusedIterator**
#### Struct `IterMut`

An iterator over the mutable key-value pairs of an [`ArchivedHashMap`].

```rust
pub struct IterMut<''a, K, V, H> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Pointee**
- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **FusedIterator**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
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

#### Struct `Keys`

An iterator over the keys of an [`ArchivedHashMap`].

```rust
pub struct Keys<''a, K, V, H> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Unpin**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **FusedIterator**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Pointee**
- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Values`

An iterator over the values of an [`ArchivedHashMap`].

```rust
pub struct Values<''a, K, V, H> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **FusedIterator**
#### Struct `ValuesMut`

An iterator over the mutable values of an [`ArchivedHashMap`].

```rust
pub struct ValuesMut<''a, K, V, H> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **FusedIterator**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

## Module `set`

Archived hash set implementation using an archived SwissTable.

```rust
pub mod set { /* ... */ }
```

### Types

#### Struct `ArchivedHashSet`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`

An archived `HashSet`. This is a wrapper around a hash map with the same key
and unit value.

```rust
pub struct ArchivedHashSet<K, H = crate::hash::FxHasher64> {
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
  pub const fn len(self: &Self) -> usize { /* ... */ }
  ```
  Gets the number of items in the hash set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether there are no items in the hash set.

- ```rust
  pub fn iter(self: &Self) -> Keys<''_, K, (), H> { /* ... */ }
  ```
  Gets an iterator over the keys of the underlying hash map.

- ```rust
  pub fn get<Q>(self: &Self, k: &Q) -> Option<&K>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Gets the key corresponding to the given key in the hash set.

- ```rust
  pub fn contains<Q>(self: &Self, k: &Q) -> bool
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns whether the given key is in the hash set.

- ```rust
  pub fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: HashSetResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived hash set from the given length and parameters.

- ```rust
  pub fn serialize_from_iter<I, KU, S>(iter: I, load_factor: (usize, usize), serializer: &mut S) -> Result<HashSetResolver, <S as >::Error>
where
    I: Clone + ExactSizeIterator,
    <I as >::Item: Borrow<KU>,
    KU: Serialize<S, Archived = K> + Hash + Eq,
    S: Fallible + Writer + Allocator + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an iterator of keys as a hash set.

###### Trait Implementations

- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Portable**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<HashSet<K, S>, <D as >::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &HashSet<K, S>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedHashSet<AK>) -> bool { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Eq**
#### Struct `HashSetResolver`

The resolver for archived hash sets.

```rust
pub struct HashSetResolver(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `table`

An archived hash table implementation based on Google's high-performance
SwissTable hash map.

Notable differences from other implementations:

- The number of control bytes is rounded up to a maximum group width (16)
  instead of the next power of two. This reduces the number of empty buckets
  on the wire. Since this collection is immutable after writing, we'll never
  benefit from having more buckets than we need.
- Because the bucket count is not a power of two, the triangular probing
  sequence simply skips any indices larger than the actual size of the
  buckets array.
- Instead of the final control bytes always being marked EMPTY, the last
  control bytes repeat the first few. This helps reduce the number of
  lookups when probing at the end of the control bytes.
- Because the available SIMD group width may be less than the maximum group
  width, each probe reads N groups before striding where N is the maximum
  group width divided by the SIMD group width.

```rust
pub mod table { /* ... */ }
```

### Types

#### Struct `ArchivedHashTable`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

A low-level archived SwissTable hash table with explicit hashing.

```rust
pub struct ArchivedHashTable<T> {
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
  pub fn get_with<C>(self: &Self, hash: u64, cmp: C) -> Option<&T>
where
    C: Fn(&T) -> bool { /* ... */ }
  ```
  Returns the key-value pair corresponding to the supplied key.

- ```rust
  pub fn get_seal_with<C>(this: Seal<''_, Self>, hash: u64, cmp: C) -> Option<Seal<''_, T>>
where
    C: Fn(&T) -> bool { /* ... */ }
  ```
  Returns the mutable key-value pair corresponding to the supplied key.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the hash table is empty.

- ```rust
  pub const fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the hash table.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total capacity of the hash table.

- ```rust
  pub fn raw_iter(self: &Self) -> RawIter<T> { /* ... */ }
  ```
  Returns an iterator over the entry pointers in the hash table.

- ```rust
  pub fn raw_iter_seal(this: Seal<''_, Self>) -> RawIter<T> { /* ... */ }
  ```
  Returns a sealed iterator over the entry pointers in the hash table.

- ```rust
  pub fn serialize_from_iter<I, U, H, S>(items: I, hashes: H, load_factor: (usize, usize), serializer: &mut S) -> Result<HashTableResolver, <S as >::Error>
where
    I: Clone + ExactSizeIterator,
    <I as >::Item: Borrow<U>,
    U: Serialize<S, Archived = T>,
    H: ExactSizeIterator<Item = u64>,
    S: Fallible + Writer + Allocator + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an iterator of items as a hash table.

- ```rust
  pub fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: HashTableResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived hash table from a given length and parameters.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Portable**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Freeze**
- **Send**
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

- **UnwindSafe**
- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `HashTableResolver`

The resolver for [`ArchivedHashTable`].

```rust
pub struct HashTableResolver {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **Sync**
- **Unpin**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

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

#### Struct `RawIter`

An iterator over the entry pointers of an [`ArchivedHashTable`].

```rust
pub struct RawIter<T> {
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
  pub fn empty() -> Self { /* ... */ }
  ```
  Returns a raw iterator which yields no elements.

###### Trait Implementations

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Pointee**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
### Re-exports

#### Re-export `ArchivedIndexMap`

```rust
pub use index_map::ArchivedIndexMap;
```

#### Re-export `IndexMapResolver`

```rust
pub use index_map::IndexMapResolver;
```

#### Re-export `ArchivedIndexSet`

```rust
pub use index_set::ArchivedIndexSet;
```

#### Re-export `IndexSetResolver`

```rust
pub use index_set::IndexSetResolver;
```

#### Re-export `ArchivedHashMap`

```rust
pub use map::ArchivedHashMap;
```

#### Re-export `HashMapResolver`

```rust
pub use map::HashMapResolver;
```

#### Re-export `ArchivedHashSet`

```rust
pub use set::ArchivedHashSet;
```

#### Re-export `HashSetResolver`

```rust
pub use set::HashSetResolver;
```

#### Re-export `ArchivedHashTable`

```rust
pub use table::ArchivedHashTable;
```

#### Re-export `HashTableResolver`

```rust
pub use table::HashTableResolver;
```

## Module `util`

Utilities for archived collections.

```rust
pub mod util { /* ... */ }
```

### Types

#### Struct `EntryAdapter`

An adapter which serializes and resolves its key and value references.

```rust
pub struct EntryAdapter<BK, BV, K, V> {
    pub key: BK,
    pub value: BV,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `BK` | The key to serialize and resolve. |
| `value` | `BV` | The value to serialize and resolve. |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(key: BK, value: BV) -> Self { /* ... */ }
  ```
  Returns a new `EntryAdapter` for the given key and value.

###### Trait Implementations

- **Archive**
  - ```rust
    fn resolve(self: &Self, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **Sync**
- **SerializeUnsized**
  - ```rust
    fn serialize_unsized(self: &Self, serializer: &mut S) -> Result<usize, <S as Fallible>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchiveUnsized**
  - ```rust
    fn archived_metadata(self: &Self) -> <<T as ArchiveUnsized>::Archived as ArchivePointee>::ArchivedMetadata { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

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

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Freeze**
- **Serialize**
  - ```rust
    fn serialize(self: &Self, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

#### Struct `EntryResolver`

A resolver for a key-value pair.

```rust
pub struct EntryResolver<K, V> {
    pub key: K,
    pub value: V,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `K` | The key resolver. |
| `value` | `V` | The value resolver. |

##### Implementations

###### Trait Implementations

- **Pointee**
- **Freeze**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Unpin**
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

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Entry`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

A key-value entry.

```rust
pub struct Entry<K, V> {
    pub key: K,
    pub value: V,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `K` | The entry's key. |
| `value` | `V` | The entry's value. |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Entry<K, V>) -> bool { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Portable**
- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Entry<K, V>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Entry<K, V>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `IteratorLengthMismatch`

An error describing that an iterator's length did not match the number of
elements it yielded.

```rust
pub struct IteratorLengthMismatch {
    pub expected: usize,
    pub actual: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expected` | `usize` | The number of expected elements. |
| `actual` | `usize` | The actual number of elements. |

##### Implementations

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Error**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
## Module `de`

Deserialization traits, deserializers, and adapters.

```rust
pub mod de { /* ... */ }
```

### Modules

## Module `pooling`

Deserializers that can be used standalone and provide basic capabilities.

```rust
pub mod pooling { /* ... */ }
```

### Types

#### Union `Metadata`

Type-erased pointer metadata.

```rust
pub union Metadata {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - `borrow_mut`: 
- **CloneToUninit**
  - `clone_to_uninit`: 
- **Borrow**
  - `borrow`: 
- **LayoutRaw**
  - `layout_raw`: 
- **Send**
- **UnwindSafe**
- **Unpin**
- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Debug**
  - `fmt`: 
- **From**
  - `from`: Returns the argument unchanged.
  - `from`: 
  - `from`: 
  - `from`: 
- **ToOwned**
  - `Owned`: 
  - `to_owned`: 
  - `clone_into`: 
- **Clone**
  - `clone`: 
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **Niching**
  - `is_niched`: 
  - `resolve_niched`: 
- **Sync**
- **Any**
  - `type_id`: 
- **Pointee**
  - `Metadata`: 
- **Copy**
- **Into**
  - `into`: Calls `U::from(self)`.
- **ArchivePointee**
  - `ArchivedMetadata`: 
  - `pointer_metadata`: 

#### Struct `ErasedPtr`

A type-erased pointer.

```rust
pub struct ErasedPtr {
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
  pub fn new<T>(ptr: NonNull<T>) -> Self
where
    T: Pointee + ?Sized,
    <T as >::Metadata: Into<Metadata> { /* ... */ }
  ```
  Returns an erased pointer corresponding to the given pointer.

- ```rust
  pub fn data_address(self: &Self) -> *mut () { /* ... */ }
  ```
  Returns the data address corresponding to this erased pointer.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ErasedPtr { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **Unpin**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

#### Enum `PoolingState`

The result of starting to deserialize a shared pointer.

```rust
pub enum PoolingState {
    Started,
    Pending,
    Finished(ErasedPtr),
}
```

##### Variants

###### `Started`

The caller started pooling this value. They should proceed to
deserialize the shared value and call `finish_pooling`.

###### `Pending`

Another caller started pooling this value, but has not finished yet.
This can only occur with cyclic shared pointer structures, and so rkyv
treats this as an error by default.

###### `Finished`

This value has already been pooled. The caller should use the returned
pointer to pool its value.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ErasedPtr` |  |

##### Implementations

###### Trait Implementations

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `FromMetadata`

A type which can be extracted from `Metadata`.

```rust
pub trait FromMetadata {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `from_metadata`: Extracts this type from [`Metadata`].

##### Implementations

This trait is implemented for the following types:

- `()`
- `usize`
- `ptr_meta::DynMetadata<T>` with <T: ?Sized>

#### Trait `SharedPointer`

A deserializable shared pointer type.

# Safety

`alloc` and `from_value` must return pointers which are non-null, writeable,
and properly aligned for `T`.

```rust
pub unsafe trait SharedPointer<T: Pointee + ?Sized> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `alloc`: Allocates space for a value with the given metadata.
- `from_value`: Creates a new `Self` from a pointer to a valid `T`.
- `drop`: Drops a pointer created by `from_value`.

##### Implementations

This trait is implemented for the following types:

- `sync::Arc<T>` with <T: LayoutRaw + Pointee + ?Sized>
- `rc::Rc<T>` with <T: LayoutRaw + Pointee + ?Sized>

#### Trait `Pooling`

A shared pointer deserialization strategy.

This trait is required to deserialize `Rc` and `Arc`.

```rust
pub trait Pooling<E = <Self as Fallible>::Error> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `start_pooling`: Starts pooling the value associated with the given address.
- `finish_pooling`: Finishes pooling the value associated with the given address.

##### Implementations

This trait is implemented for the following types:

- `Pool` with <E: Source>
- `Unpool` with <E>
- `rancor::Strategy<T, E>` with <T, E>

#### Trait `PoolingExt`

Helper methods for [`Pooling`].

```rust
pub trait PoolingExt<E>: Pooling<E> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn deserialize_shared<T, P>(self: &mut Self, value: &<T as >::Archived) -> Result<*mut T, <Self as >::Error>
where
    T: ArchiveUnsized + Pointee + LayoutRaw + ?Sized,
    <T as >::Metadata: Into<Metadata> + FromMetadata,
    <T as >::Archived: DeserializeUnsized<T, Self>,
    P: SharedPointer<T>,
    Self: Fallible<Error = E>,
    E: Source { /* ... */ }
  ```
  Checks whether the given reference has been deserialized and either uses

##### Implementations

This trait is implemented for the following types:

- `T` with <T, E>

### Re-exports

#### Re-export `self::alloc::*`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::alloc::*;
```

#### Re-export `self::core::*`

```rust
pub use self::core::*;
```

### Re-exports

#### Re-export `self::pooling::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::pooling::*;
```

## Module `ffi`

Archived versions of FFI types.

```rust
pub mod ffi { /* ... */ }
```

### Types

#### Struct `ArchivedCString`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`

An archived [`CString`](crate::alloc::ffi::CString).

Uses a [`RelPtr`] to a `CStr` under the hood.

```rust
pub struct ArchivedCString {
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
  pub fn as_bytes(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns the contents of this CString as a slice of bytes.

- ```rust
  pub fn as_bytes_with_nul(self: &Self) -> &[u8] { /* ... */ }
  ```
  Equivalent to [`as_bytes`][ArchivedCString::as_bytes()] except that the

- ```rust
  pub fn as_c_str(self: &Self) -> &CStr { /* ... */ }
  ```
  Extracts a `CStr` slice containing the entire string.

- ```rust
  pub fn resolve_from_c_str(c_str: &CStr, resolver: CStringResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived C string from the given C string and parameters.

- ```rust
  pub fn serialize_from_c_str<S: Fallible + Writer + ?Sized>(c_str: &CStr, serializer: &mut S) -> Result<CStringResolver, <S as >::Error> { /* ... */ }
  ```
  Serializes a C string.

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<CString, <D as >::Error> { /* ... */ }
    ```

- **Eq**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedCString, deserializer: &mut D) -> Result<Cow<''a, CStr>, <D as >::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &CStr { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &CStr { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Receiver**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Send**
- **Pointee**
- **Portable**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&CStr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedCString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &CString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedCString) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, _: RangeFull) -> &<Self as >::Output { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
#### Struct `CStringResolver`

The resolver for `CString`.

```rust
pub struct CStringResolver {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
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

- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Pointee**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **RefUnwindSafe**
## Module `hash`

Hashing support for archived hash maps and sets.

```rust
pub mod hash { /* ... */ }
```

### Types

#### Struct `FxHasher64`

A cross-platform 64-bit implementation of fxhash.

```rust
pub struct FxHasher64 {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> FxHasher64 { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Freeze**
- **Pointee**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hasher**
  - ```rust
    fn write(self: &mut Self, bytes: &[u8]) { /* ... */ }
    ```

  - ```rust
    fn finish(self: &Self) -> u64 { /* ... */ }
    ```

  - ```rust
    fn write_u8(self: &mut Self, i: u8) { /* ... */ }
    ```

  - ```rust
    fn write_u16(self: &mut Self, i: u16) { /* ... */ }
    ```

  - ```rust
    fn write_u32(self: &mut Self, i: u32) { /* ... */ }
    ```

  - ```rust
    fn write_u64(self: &mut Self, i: u64) { /* ... */ }
    ```

  - ```rust
    fn write_u128(self: &mut Self, i: u128) { /* ... */ }
    ```

  - ```rust
    fn write_usize(self: &mut Self, i: usize) { /* ... */ }
    ```

  - ```rust
    fn write_isize(self: &mut Self, i: isize) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

### Functions

#### Function `hash_value`

Hashes the given value with the default value of the specified `Hasher`.

```rust
pub fn hash_value<Q, H: Hasher + Default>(value: &Q) -> u64
where
    Q: Hash + ?Sized { /* ... */ }
```

## Module `net`

Archived versions of network types.

```rust
pub mod net { /* ... */ }
```

### Types

#### Struct `ArchivedIpv4Addr`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`

An archived [`Ipv4Addr`].

```rust
pub struct ArchivedIpv4Addr {
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
  pub const fn octets(self: &Self) -> [u8; 4] { /* ... */ }
  ```
  Returns the four eight-bit integers that make up this address.

- ```rust
  pub const fn as_ipv4(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Returns an [`Ipv4Addr`] with the same value.

- ```rust
  pub const fn is_broadcast(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is a broadcast address (255.255.255.255).

- ```rust
  pub const fn is_documentation(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this address is in a range designated for

- ```rust
  pub const fn is_link_local(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the address is link-local (169.254.0.0/16).

- ```rust
  pub const fn is_loopback(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is a loopback address (127.0.0.0/8).

- ```rust
  pub const fn is_multicast(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is a multicast address (224.0.0.0/4).

- ```rust
  pub const fn is_private(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is a private address.

- ```rust
  pub const fn is_unspecified(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` for the special 'unspecified' address (0.0.0.0).

- ```rust
  pub const fn to_ipv6_compatible(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Converts this address to an IPv4-compatible

- ```rust
  pub const fn to_ipv6_mapped(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Converts this address to an IPv4-mapped

- ```rust
  pub fn emplace(octets: [u8; 4], out: Place<Self>) { /* ... */ }
  ```
  Emplaces an `ArchivedIpv4Addr` with the given octets into a place.

###### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedIpv4Addr { /* ... */ }
    ```

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

- **Sync**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Portable**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedIpv4Addr) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, _: &mut D) -> Result<Ipv4Addr, <D as >::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **Eq**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ipv4Addr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedIpv4Addr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedIpv4Addr) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedIpv4Addr { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Ipv4Addr) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedIpv4Addr) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedIpv4Addr) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

#### Struct `ArchivedIpv6Addr`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`

An archived [`Ipv6Addr`].

```rust
pub struct ArchivedIpv6Addr {
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
  pub const fn segments(self: &Self) -> [u16; 8] { /* ... */ }
  ```
  Returns the eight 16-bit segments that make up this address.

- ```rust
  pub const fn as_ipv6(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Returns an [`Ipv6Addr`] with the same value.

- ```rust
  pub const fn is_loopback(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is a loopback address (::1).

- ```rust
  pub const fn is_multicast(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is a multicast address (ff00::/8).

- ```rust
  pub const fn is_unspecified(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` for the special 'unspecified' address (::).

- ```rust
  pub const fn octets(self: &Self) -> [u8; 16] { /* ... */ }
  ```
  Returns the sixteen eight-bit integers the IPv6 address consists of.

- ```rust
  pub const fn to_ipv4(self: &Self) -> Option<Ipv4Addr> { /* ... */ }
  ```
  Converts this address to an [`IPv4` address](std::net::Ipv4Addr).

- ```rust
  pub fn emplace(octets: [u8; 16], out: Place<Self>) { /* ... */ }
  ```
  Emplaces an `ArchivedIpv6Addr` with the given octets into a place.

###### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedIpv6Addr) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedIpv6Addr { /* ... */ }
    ```

- **Pointee**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Copy**
- **Send**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedIpv6Addr { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Portable**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ipv6Addr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedIpv6Addr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedIpv6Addr) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Ipv6Addr) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedIpv6Addr) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedIpv6Addr) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, _: &mut D) -> Result<Ipv6Addr, <D as >::Error> { /* ... */ }
    ```

#### Enum `ArchivedIpAddr`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[repr(u8)]`

An archived [`IpAddr`].

```rust
pub enum ArchivedIpAddr {
    V4(ArchivedIpv4Addr),
    V6(ArchivedIpv6Addr),
}
```

##### Variants

###### `V4`

An IPv4 address.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ArchivedIpv4Addr` |  |

###### `V6`

An IPv6 address.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ArchivedIpv6Addr` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn is_ipv4(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this address is an [`IPv4`

- ```rust
  pub const fn is_ipv6(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this address is an [`IPv6`

- ```rust
  pub const fn as_ipaddr(self: &Self) -> IpAddr { /* ... */ }
  ```
  Returns an [`IpAddr`] with the same value.

- ```rust
  pub const fn is_loopback(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is a loopback address.

- ```rust
  pub const fn is_multicast(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is a multicast address.

- ```rust
  pub const fn is_unspecified(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` for the special 'unspecified' address.

###### Trait Implementations

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedIpAddr) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Pointee**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedIpAddr { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &IpAddr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedIpAddr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedIpAddr) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Portable**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Copy**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<IpAddr, <D as >::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &IpAddr) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedIpAddr) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedIpAddr) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

#### Struct `ArchivedSocketAddrV4`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`SocketAddrV4`].

```rust
pub struct ArchivedSocketAddrV4 {
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
  pub const fn ip(self: &Self) -> &ArchivedIpv4Addr { /* ... */ }
  ```
  Returns the IP address associated with this socket address.

- ```rust
  pub const fn port(self: &Self) -> u16 { /* ... */ }
  ```
  Returns the port number associated with this socket address.

- ```rust
  pub fn as_socket_addr_v4(self: &Self) -> SocketAddrV4 { /* ... */ }
  ```
  Returns a [`SocketAddrV4`] with the same value.

- ```rust
  pub fn emplace(value: &SocketAddrV4, out: Place<Self>) { /* ... */ }
  ```
  Emplaces an `ArchivedSocketAddrV4` of the given `value` into a place.

###### Trait Implementations

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SocketAddrV4) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedSocketAddrV4) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedSocketAddrV4) -> bool { /* ... */ }
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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedSocketAddrV4) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Portable**
- **Unpin**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToSocketAddrs**
  - ```rust
    fn to_socket_addrs(self: &Self) -> io::Result<<Self as >::Iter> { /* ... */ }
    ```

- **Freeze**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<SocketAddrV4, <D as >::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedSocketAddrV4 { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedSocketAddrV4 { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SocketAddrV4) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedSocketAddrV4) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedSocketAddrV4) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **Send**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `ArchivedSocketAddrV6`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`SocketAddrV6`].

```rust
pub struct ArchivedSocketAddrV6 {
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
  pub const fn flowinfo(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the flow information associated with this address.

- ```rust
  pub const fn ip(self: &Self) -> &ArchivedIpv6Addr { /* ... */ }
  ```
  Returns the IP address associated with this socket address.

- ```rust
  pub const fn port(self: &Self) -> u16 { /* ... */ }
  ```
  Returns the port number associated with this socket address.

- ```rust
  pub const fn scope_id(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the scope ID associated with this address.

- ```rust
  pub fn as_socket_addr_v6(self: &Self) -> SocketAddrV6 { /* ... */ }
  ```
  Returns a [`SocketAddrV6`] with the same value.

- ```rust
  pub fn emplace(value: &SocketAddrV6, out: Place<Self>) { /* ... */ }
  ```
  Emplaces an `ArchivedSocketAddrV6` of the given `value` into a place.

###### Trait Implementations

- **Eq**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<SocketAddrV6, <D as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> ArchivedSocketAddrV6 { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedSocketAddrV6) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Pointee**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SocketAddrV6) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedSocketAddrV6) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedSocketAddrV6) -> bool { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedSocketAddrV6 { /* ... */ }
    ```

- **Portable**
- **StructuralPartialEq**
- **ToSocketAddrs**
  - ```rust
    fn to_socket_addrs(self: &Self) -> io::Result<<Self as >::Iter> { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SocketAddrV6) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedSocketAddrV6) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedSocketAddrV6) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

#### Enum `ArchivedSocketAddr`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[repr(u8)]`

An archived [`SocketAddr`].

```rust
pub enum ArchivedSocketAddr {
    V4(ArchivedSocketAddrV4),
    V6(ArchivedSocketAddrV6),
}
```

##### Variants

###### `V4`

An IPv4 socket address.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ArchivedSocketAddrV4` |  |

###### `V6`

An IPv6 socket address.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ArchivedSocketAddrV6` |  |

##### Implementations

###### Methods

- ```rust
  pub fn port(self: &Self) -> u16 { /* ... */ }
  ```
  Returns the port number associated with this socket address.

- ```rust
  pub fn is_ipv4(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the [IP address](std::net::IpAddr) in this

- ```rust
  pub fn is_ipv6(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the [IP address](std::net::IpAddr) in this

- ```rust
  pub fn as_socket_addr(self: &Self) -> SocketAddr { /* ... */ }
  ```
  Returns a [`SocketAddr`] with the same value.

- ```rust
  pub fn ip(self: &Self) -> IpAddr { /* ... */ }
  ```
  Returns the IP address associated with this socket address.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedSocketAddr) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Sync**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SocketAddr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedSocketAddr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedSocketAddr) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Copy**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SocketAddr) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedSocketAddr) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedSocketAddr) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<SocketAddr, <D as >::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **ToSocketAddrs**
  - ```rust
    fn to_socket_addrs(self: &Self) -> io::Result<<Self as >::Iter> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Portable**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedSocketAddr { /* ... */ }
    ```

## Module `niche`

Manually niched type replacements.

```rust
pub mod niche { /* ... */ }
```

### Modules

## Module `niched_option`

A niched `ArchivedOption<T>` that uses less space based on a [`Niching`].

```rust
pub mod niched_option { /* ... */ }
```

### Types

#### Struct `NichedOption`

**Attributes:**

- `#[rkyv(crate)]`

A niched `ArchivedOption<T>`.

It has the same layout as `T`, and thus uses less space by storing the
`None` variant in a custom way based on `N`.

```rust
pub struct NichedOption<T, N: ?Sized> {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Converts to an `Option<&T>`.

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Converts to an `Option<&mut T>`.

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, T>> { /* ... */ }
  ```
  Converts from `Seal<'_, NichedOption<T, N>>` to `Option<Seal<'_, T>>`.

- ```rust
  pub fn iter(self: &Self) -> Iter<&T> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut T> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, T>> { /* ... */ }
  ```
  Returns an iterator over the sealed possibly-contained value.

- ```rust
  pub fn resolve_from_option<U>(option: Option<&U>, resolver: Option<<U as >::Resolver>, out: Place<Self>)
where
    U: Archive<Archived = T> { /* ... */ }
  ```
  Resolves a `NichedOption<U::Archived, N>` from an `Option<&U>`.

- ```rust
  pub fn serialize_from_option<U, S>(option: Option<&U>, serializer: &mut S) -> Result<Option<<U as >::Resolver>, <S as >::Error>
where
    U: Serialize<S, Archived = T>,
    S: Fallible + ?Sized { /* ... */ }
  ```
  Serializes a `NichedOption<U::Archived, N>` from an `Option<&U>`.

- ```rust
  pub fn as_deref(self: &Self) -> Option<&<T as Deref>::Target> { /* ... */ }
  ```
  Converts from `&NichedOption<T, N>` to `Option<&T::Target>`.

###### Trait Implementations

- **RefUnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<Option<T>, <D as >::Error> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Portable**
- **Eq**
- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Option<Rhs>) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &NichedOption<<T as >::Archived, N>, deserializer: &mut D) -> Result<Option<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &NichedOption<<W as ArchiveWith<T>>::Archived, N>, deserializer: &mut D) -> Result<Option<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &NichedOption<<T as >::Archived, Self>, deserializer: &mut D) -> Result<Option<T>, <D as >::Error> { /* ... */ }
    ```

- **Sync**
#### Type Alias `Iter`

An iterator over a reference to the `Some` variant of a `NichedOption`.

This iterator yields one value if the `NichedOption` is a `Some`, otherwise
none.

```rust
pub type Iter<P> = crate::option::Iter<P>;
```

## Module `niching`

[`Niching`] implementors for [`NicheInto`].

[`NicheInto`]: crate::with::NicheInto

```rust
pub mod niching { /* ... */ }
```

### Types

#### Struct `DefaultNiche`

Default [`Niching`] for various types.

Also serves as with-wrapper by being shorthand for
`NicheInto<DefaultNiche>`.

```rust
pub struct DefaultNiche;
```

##### Implementations

###### Trait Implementations

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &Option<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const NonZeroU8) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NonZeroU8>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroU16) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroU16>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroU32) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroU32>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroU64) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroU64>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroU128) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroU128>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const NonZeroI8) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NonZeroI8>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroI16) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroI16>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroI32) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroI32>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroI64) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroI64>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroI128) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroI128>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const bool) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<bool>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedBox<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedBox<T>>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &NichedOption<<T as >::Archived, Self>, deserializer: &mut D) -> Result<Option<T>, <D as >::Error> { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &Option<T>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Pointee**
- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `Zero`

[`Niching`] for zero-niched values.

```rust
pub struct Zero;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const NonZeroU8) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NonZeroU8>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroU16) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroU16>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroU32) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroU32>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroU64) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroU64>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroU128) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroU128>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const NonZeroI8) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NonZeroI8>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroI16) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroI16>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroI32) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroI32>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroI64) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroI64>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedNonZeroI128) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedNonZeroI128>) { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

#### Struct `NaN`

[`Niching`] for NaN-niched values.

```rust
pub struct NaN;
```

##### Implementations

###### Trait Implementations

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedF32) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedF32>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedF64) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedF64>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **Unpin**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Null`

[`Niching`] for null-pointer-niched values.

```rust
pub struct Null;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Pointee**
- **UnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const ArchivedBox<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<ArchivedBox<T>>) { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Bool`

[`Niching`] for booleans.

```rust
pub struct Bool;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

  - ```rust
    unsafe fn is_niched(niched: *const bool) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<bool>) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
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

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

### Traits

#### Trait `Niching`

A type that can be used to niche a value with [`NicheInto`].

# Example

```
use rkyv::{
    niche::niching::Niching, primitive::ArchivedU32, with::NicheInto,
    Archive, Archived, Place, Serialize,
};

// Let's niche `Option<u32>` by using odd values
struct NeverOdd;

impl Niching<ArchivedU32> for NeverOdd {
    unsafe fn is_niched(niched: *const ArchivedU32) -> bool {
        // Interprete odd values as "niched"
        unsafe { *niched % 2 == 1 }
    }

    fn resolve_niched(out: Place<ArchivedU32>) {
        // To niche, we use the value `1`
        out.write(ArchivedU32::from_native(1))
    }
}

#[derive(Archive)]
struct Basic {
    field: Option<u32>,
}

#[derive(Archive, Serialize)]
struct Niched {
    #[rkyv(with = NicheInto<NeverOdd>)]
    field: Option<u32>,
}

# fn main() -> Result<(), rkyv::rancor::Error> {
// Indeed, we have a smaller archived representation
assert!(size_of::<ArchivedNiched>() < size_of::<ArchivedBasic>());

let values: Vec<Niched> =
    (0..4).map(|n| Niched { field: Some(n) }).collect();

let bytes = rkyv::to_bytes(&values)?;
let archived = rkyv::access::<Archived<Vec<Niched>>, _>(&bytes)?;
assert_eq!(archived[0].field.as_ref(), Some(&0.into()));
assert_eq!(archived[1].field.as_ref(), None);
assert_eq!(archived[2].field.as_ref(), Some(&2.into()));
assert_eq!(archived[3].field.as_ref(), None);
# Ok(()) }
```

[`NicheInto`]: crate::with::NicheInto

```rust
pub trait Niching<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `is_niched`: Returns whether the given value has been niched.
- `resolve_niched`: Writes data to `out` indicating that a `T` is niched.

##### Implementations

This trait is implemented for the following types:

- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `Zero`
- `DefaultNiche`
- `NaN`
- `NaN`
- `crate::niche::niching::Bool`
- `DefaultNiche`
- `crate::niche::niching::Null` with <T>
- `crate::niche::niching::DefaultNiche` with <T>
- `N2` with <T, N1, N2>

#### Trait `SharedNiching`

Trait to allow `NichedOption<Self, N1>` to be niched further by `N2`.

# Safety

Implementors must ensure that the memory regions within `Self` that are used
for [`Niching`] impls of `N1` and `N2` are mutually exclusive.

```rust
pub unsafe trait SharedNiching<N1, N2> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

## Module `option_box`

A niched archived `Option<Box<T>>` that uses less space.

```rust
pub mod option_box { /* ... */ }
```

### Types

#### Struct `ArchivedOptionBox`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`

A niched archived `Option<Box<T>>`.

It uses less space by storing the `None` variant as a null pointer.

```rust
pub struct ArchivedOptionBox<T: ArchivePointee + ?Sized> {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option box is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option box is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&ArchivedBox<T>> { /* ... */ }
  ```
  Converts to an `Option<&ArchivedBox<T>>`.

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut ArchivedBox<T>> { /* ... */ }
  ```
  Converts to an `Option<&mut ArchivedBox<T>>`.

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, ArchivedBox<T>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOption<T>>` to `Option<Seal<'_,

- ```rust
  pub fn iter(self: &Self) -> Iter<&ArchivedBox<T>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut ArchivedBox<T>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, ArchivedBox<T>>> { /* ... */ }
  ```
  Returns an iterator over the sealed possibly-contained value.

- ```rust
  pub fn as_deref(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Converts from `&ArchivedOptionBox<T>` to `Option<&T>`.

- ```rust
  pub fn resolve_from_option<U: ArchiveUnsized<Archived = T> + ?Sized>(field: Option<&U>, resolver: OptionBoxResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionBox<T::Archived>` from an `Option<&T>`.

- ```rust
  pub fn serialize_from_option<U, S>(field: Option<&U>, serializer: &mut S) -> Result<OptionBoxResolver, <S as >::Error>
where
    U: SerializeUnsized<S, Archived = T> + ?Sized,
    S: Fallible + Writer + ?Sized { /* ... */ }
  ```
  Serializes an `ArchivedOptionBox<T::Archived>` from an `Option<&T>`.

###### Trait Implementations

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
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

- **Sync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Option<Box<T>>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionBox<<T as >::Archived>, deserializer: &mut D) -> Result<Option<Box<T>>, <D as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Portable**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Type Alias `Iter`

An iterator over a reference to the `Some` variant of an
`ArchivedOptionBox`.

This iterator yields one value if the `ArchivedOptionBox` is a `Some`,
otherwise none.

```rust
pub type Iter<P> = crate::option::Iter<P>;
```

#### Enum `OptionBoxResolver`

The resolver for [`ArchivedOptionBox`].

```rust
pub enum OptionBoxResolver {
    None,
    Some(crate::boxed::BoxResolver),
}
```

##### Variants

###### `None`

The `ArchivedOptionBox` was `None`

###### `Some`

The resolver for the `ArchivedBox`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::boxed::BoxResolver` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Pointee**
- **Send**
- **Freeze**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

## Module `option_nonzero`

Niched archived `Option<NonZero>` integers that use less space.

```rust
pub mod option_nonzero { /* ... */ }
```

### Types

#### Struct `ArchivedOptionNonZeroI8`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroI8>`

```rust
pub struct ArchivedOptionNonZeroI8 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroI8>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroI8>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroI8>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroI8>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroI8>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroI8>` to `Option<Seal<'_, Archived<NonZeroI8>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroI8>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroI8>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroI8>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroI8>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroI8) -> &mut Archived<NonZeroI8> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroI8>
where
    F: FnOnce() -> NonZeroI8 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroI8>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI8, _: &mut D) -> Result<Option<NonZeroI8>, <D as >::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Unpin**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Sync**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Copy**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **NoUndef**
- **Portable**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroI8 { /* ... */ }
    ```

- **Eq**
- **Pointee**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ArchivedOptionNonZeroI16`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroI16>`

```rust
pub struct ArchivedOptionNonZeroI16 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroI16>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroI16>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroI16>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroI16>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroI16>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroI16>` to `Option<Seal<'_, Archived<NonZeroI16>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroI16>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroI16>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroI16>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroI16>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroI16) -> &mut Archived<NonZeroI16> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroI16>
where
    F: FnOnce() -> NonZeroI16 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroI16>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI16, _: &mut D) -> Result<Option<NonZeroI16>, <D as >::Error> { /* ... */ }
    ```

- **Pointee**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Eq**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Portable**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **NoUndef**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

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

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroI16 { /* ... */ }
    ```

#### Struct `ArchivedOptionNonZeroI32`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroI32>`

```rust
pub struct ArchivedOptionNonZeroI32 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroI32>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroI32>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroI32>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroI32>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroI32>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroI32>` to `Option<Seal<'_, Archived<NonZeroI32>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroI32>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroI32>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroI32>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroI32>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroI32) -> &mut Archived<NonZeroI32> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroI32>
where
    F: FnOnce() -> NonZeroI32 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroI32>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Pointee**
- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroI32 { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI32, _: &mut D) -> Result<Option<NonZeroI32>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroIsize, _: &mut D) -> Result<Option<NonZeroIsize>, <D as >::Error> { /* ... */ }
    ```

- **Portable**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Copy**
- **NoUndef**
- **Send**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

#### Struct `ArchivedOptionNonZeroI64`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroI64>`

```rust
pub struct ArchivedOptionNonZeroI64 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroI64>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroI64>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroI64>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroI64>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroI64>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroI64>` to `Option<Seal<'_, Archived<NonZeroI64>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroI64>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroI64>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroI64>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroI64>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroI64) -> &mut Archived<NonZeroI64> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroI64>
where
    F: FnOnce() -> NonZeroI64 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroI64>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Pointee**
- **Unpin**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroI64 { /* ... */ }
    ```

- **Eq**
- **Send**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **NoUndef**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI64, _: &mut D) -> Result<Option<NonZeroI64>, <D as >::Error> { /* ... */ }
    ```

- **Portable**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Copy**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `ArchivedOptionNonZeroI128`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroI128>`

```rust
pub struct ArchivedOptionNonZeroI128 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroI128>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroI128>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroI128>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroI128>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroI128>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroI128>` to `Option<Seal<'_, Archived<NonZeroI128>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroI128>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroI128>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroI128>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroI128>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroI128) -> &mut Archived<NonZeroI128> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroI128>
where
    F: FnOnce() -> NonZeroI128 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroI128>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Portable**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI128, _: &mut D) -> Result<Option<NonZeroI128>, <D as >::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroI128 { /* ... */ }
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

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **NoUndef**
- **Eq**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

#### Type Alias `ArchivedOptionNonZeroIsize`

A niched archived `Option<NonZeroIsize>`

```rust
pub type ArchivedOptionNonZeroIsize = ArchivedOptionNonZeroI32;
```

#### Struct `ArchivedOptionNonZeroU8`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroU8>`

```rust
pub struct ArchivedOptionNonZeroU8 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroU8>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroU8>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroU8>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroU8>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroU8>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroU8>` to `Option<Seal<'_, Archived<NonZeroU8>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroU8>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroU8>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroU8>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroU8>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroU8) -> &mut Archived<NonZeroU8> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroU8>
where
    F: FnOnce() -> NonZeroU8 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroU8>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **NoUndef**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

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
    fn clone(self: &Self) -> ArchivedOptionNonZeroU8 { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU8, _: &mut D) -> Result<Option<NonZeroU8>, <D as >::Error> { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Pointee**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Freeze**
- **Portable**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `ArchivedOptionNonZeroU16`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroU16>`

```rust
pub struct ArchivedOptionNonZeroU16 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroU16>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroU16>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroU16>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroU16>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroU16>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroU16>` to `Option<Seal<'_, Archived<NonZeroU16>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroU16>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroU16>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroU16>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroU16>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroU16) -> &mut Archived<NonZeroU16> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroU16>
where
    F: FnOnce() -> NonZeroU16 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroU16>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroU16 { /* ... */ }
    ```

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Pointee**
- **Portable**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **NoUndef**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU16, _: &mut D) -> Result<Option<NonZeroU16>, <D as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `ArchivedOptionNonZeroU32`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroU32>`

```rust
pub struct ArchivedOptionNonZeroU32 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroU32>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroU32>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroU32>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroU32>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroU32>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroU32>` to `Option<Seal<'_, Archived<NonZeroU32>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroU32>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroU32>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroU32>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroU32>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroU32) -> &mut Archived<NonZeroU32> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroU32>
where
    F: FnOnce() -> NonZeroU32 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroU32>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **NoUndef**
- **RefUnwindSafe**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

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

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Portable**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU32, _: &mut D) -> Result<Option<NonZeroU32>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroUsize, _: &mut D) -> Result<Option<NonZeroUsize>, <D as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroU32 { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `ArchivedOptionNonZeroU64`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroU64>`

```rust
pub struct ArchivedOptionNonZeroU64 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroU64>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroU64>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroU64>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroU64>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroU64>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroU64>` to `Option<Seal<'_, Archived<NonZeroU64>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroU64>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroU64>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroU64>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroU64>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroU64) -> &mut Archived<NonZeroU64> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroU64>
where
    F: FnOnce() -> NonZeroU64 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroU64>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointee**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroU64 { /* ... */ }
    ```

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **NoUndef**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Portable**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU64, _: &mut D) -> Result<Option<NonZeroU64>, <D as >::Error> { /* ... */ }
    ```

#### Struct `ArchivedOptionNonZeroU128`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`

A niched archived `Option<NonZeroU128>`

```rust
pub struct ArchivedOptionNonZeroU128 {
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
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Archived<NonZeroU128>> { /* ... */ }
  ```
  Converts to an `Option<&Archived<NonZeroU128>>`

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut Archived<NonZeroU128>> { /* ... */ }
  ```
  Converts to an `Option<&mut Archived<NonZeroU128>>`

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, Archived<NonZeroU128>>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOptionNonZeroU128>` to `Option<Seal<'_, Archived<NonZeroU128>>>`.

- ```rust
  pub fn take(self: &mut Self) -> Option<Archived<NonZeroU128>> { /* ... */ }
  ```
  Takes the value out of the option, leaving a `None` in its

- ```rust
  pub fn iter(self: &Self) -> Iter<&Archived<NonZeroU128>> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut Archived<NonZeroU128>> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, Archived<NonZeroU128>>> { /* ... */ }
  ```
  Returns an iterator over the sealed mutable possibly-contained

- ```rust
  pub fn get_or_insert(self: &mut Self, v: NonZeroU128) -> &mut Archived<NonZeroU128> { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a

- ```rust
  pub fn get_or_insert_with<F>(self: &mut Self, f: F) -> &mut Archived<NonZeroU128>
where
    F: FnOnce() -> NonZeroU128 { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is

- ```rust
  pub fn resolve_from_option(field: Option<NonZeroU128>, out: Place<Self>) { /* ... */ }
  ```
  Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

###### Trait Implementations

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOptionNonZeroU128 { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Portable**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU128, _: &mut D) -> Result<Option<NonZeroU128>, <D as >::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Pointee**
- **UnwindSafe**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **NoUndef**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Type Alias `ArchivedOptionNonZeroUsize`

A niched archived `Option<NonZeroUsize>`

```rust
pub type ArchivedOptionNonZeroUsize = ArchivedOptionNonZeroU32;
```

#### Type Alias `Iter`

An iterator over a reference to the `Some` variant of an
`ArchivedOptionNonZero` integer.

This iterator yields one value if the `ArchivedOptionNonZero` integer is a
`Some`, otherwise none.

```rust
pub type Iter<P> = crate::option::Iter<P>;
```

## Module `ops`

Archived versions of `ops` types.

```rust
pub mod ops { /* ... */ }
```

### Types

#### Struct `ArchivedRangeFull`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`RangeFull`](::core::ops::RangeFull).

```rust
pub struct ArchivedRangeFull;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedRangeFull { /* ... */ }
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

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, _: &mut D) -> Result<RangeFull, <D as >::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, _: &RangeFull) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedRangeFull) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Portable**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedRangeFull { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `ArchivedRange`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`Range`](::core::ops::Range).

```rust
pub struct ArchivedRange<T> {
    pub start: T,
    pub end: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `T` | The lower bound of the range (inclusive). |
| `end` | `T` | The upper bound of the range (inclusive). |

##### Implementations

###### Methods

- ```rust
  pub fn contains<U>(self: &Self, item: &U) -> bool
where
    T: PartialOrd<U>,
    U: PartialOrd<T> + ?Sized { /* ... */ }
  ```
  Returns `true` if `item` is contained in the range.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the range contains no items.

###### Trait Implementations

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
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

- **Eq**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<Range<T>, <D as >::Error> { /* ... */ }
    ```

- **Portable**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Default**
  - ```rust
    fn default() -> ArchivedRange<T> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **RangeBounds**
  - ```rust
    fn start_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

  - ```rust
    fn end_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Range<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedRange<T>) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Pointee**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedRange<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
#### Struct `ArchivedRangeInclusive`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`RangeInclusive`](::core::ops::RangeInclusive).

```rust
pub struct ArchivedRangeInclusive<T> {
    pub start: T,
    pub end: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `T` | The lower bound of the range (inclusive). |
| `end` | `T` | The upper bound of the range (inclusive). |

##### Implementations

###### Methods

- ```rust
  pub fn contains<U>(self: &Self, item: &U) -> bool
where
    T: PartialOrd<U>,
    U: PartialOrd<T> + ?Sized { /* ... */ }
  ```
  Returns `true` if `item` is contained in the range.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the range contains no items.

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Portable**
- **StructuralPartialEq**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RangeInclusive<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedRangeInclusive<T>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Pointee**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **RangeBounds**
  - ```rust
    fn start_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

  - ```rust
    fn end_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<RangeInclusive<T>, <D as >::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedRangeInclusive<T> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> ArchivedRangeInclusive<T> { /* ... */ }
    ```

- **Eq**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

#### Struct `ArchivedRangeFrom`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`RangeFrom`](::core::ops::RangeFrom).

```rust
pub struct ArchivedRangeFrom<T> {
    pub start: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `T` | The lower bound of the range (inclusive). |

##### Implementations

###### Methods

- ```rust
  pub fn contains<U>(self: &Self, item: &U) -> bool
where
    T: PartialOrd<U>,
    U: ?Sized + PartialOrd<T> { /* ... */ }
  ```
  Returns `true` if `item` is contained in the range.

###### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<RangeFrom<T>, <D as >::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Portable**
- **Sync**
- **Freeze**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RangeFrom<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedRangeFrom<T>) -> bool { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedRangeFrom<T> { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedRangeFrom<T> { /* ... */ }
    ```

- **RangeBounds**
  - ```rust
    fn start_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

  - ```rust
    fn end_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

#### Struct `ArchivedRangeTo`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`RangeTo`](::core::ops::RangeTo).

```rust
pub struct ArchivedRangeTo<T> {
    pub end: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `end` | `T` | The upper bound of the range (exclusive). |

##### Implementations

###### Methods

- ```rust
  pub fn contains<U>(self: &Self, item: &U) -> bool
where
    T: PartialOrd<U>,
    U: ?Sized + PartialOrd<T> { /* ... */ }
  ```
  Returns `true` if `item` is contained in the range.

###### Trait Implementations

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<RangeTo<T>, <D as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Portable**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RangeBounds**
  - ```rust
    fn start_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

  - ```rust
    fn end_bound(self: &Self) -> Bound<&T> { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedRangeTo<T> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedRangeTo<T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Pointee**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RangeTo<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedRangeTo<T>) -> bool { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ArchivedRangeToInclusive`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`RangeToInclusive`](::core::ops::RangeToInclusive).

```rust
pub struct ArchivedRangeToInclusive<T> {
    pub end: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `end` | `T` | The upper bound of the range (inclusive). |

##### Implementations

###### Methods

- ```rust
  pub fn contains<U>(self: &Self, item: &U) -> bool
where
    T: PartialOrd<U>,
    U: ?Sized + PartialOrd<T> { /* ... */ }
  ```
  Returns `true` if `item` is contained in the range.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedRangeToInclusive<T> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
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

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedRangeToInclusive<T> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RangeToInclusive<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedRangeToInclusive<T>) -> bool { /* ... */ }
    ```

- **Portable**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<RangeToInclusive<T>, <D as >::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **StructuralPartialEq**
- **RangeBounds**
  - ```rust
    fn start_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

  - ```rust
    fn end_bound(self: &Self) -> Bound<&T> { /* ... */ }
    ```

#### Enum `ArchivedBound`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(u8)]`

An archived [`Bound`].

```rust
pub enum ArchivedBound<T> {
    Included(T),
    Excluded(T),
    Unbounded,
}
```

##### Variants

###### `Included`

An inclusive bound.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Excluded`

An exclusive bound.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Unbounded`

An infinite endpoint. Indicates that there is no bound in this
direction.

##### Implementations

###### Methods

- ```rust
  pub fn as_ref(self: &Self) -> Bound<&T> { /* ... */ }
  ```
  Converts from `&ArchivedBound<T>` to `Bound<&T>`.

- ```rust
  pub fn as_mut(self: &mut Self) -> Bound<&mut T> { /* ... */ }
  ```
  Converts from `&mut ArchivedBound<T>` to `Bound<&mut T>`.

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Bound<Seal<''_, T>> { /* ... */ }
  ```
  Converts from `Seal<&ArchivedBound<T>>` to `Bound<Seal<&T>>`.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Bound<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedBound<T>) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Portable**
- **UnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Copy**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<Bound<T>, <D as Fallible>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedBound<T> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Unpin**
## Module `option`

An archived version of `Option`.

```rust
pub mod option { /* ... */ }
```

### Types

#### Enum `ArchivedOption`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(u8)]`

An archived [`Option`].

It functions identically to [`Option`] but has a different internal
representation to allow for archiving.

```rust
pub enum ArchivedOption<T> {
    None,
    Some(T),
}
```

##### Variants

###### `None`

No value

###### `Some`

Some value `T`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn ok_or<E>(self: Self, err: E) -> Result<T, E> { /* ... */ }
  ```
  Transforms the `ArchivedOption<T>` into a `Result<T, E>`, mapping

- ```rust
  pub fn unwrap(self: Self) -> T { /* ... */ }
  ```
  Returns the contained [`Some`] value, consuming the `self` value.

- ```rust
  pub fn unwrap_or(self: Self, default: T) -> T { /* ... */ }
  ```
  Returns the contained [`Some`] value or a provided default.

- ```rust
  pub fn unwrap_or_else<F: FnOnce() -> T>(self: Self, f: F) -> T { /* ... */ }
  ```
  Returns the contained [`Some`] value or computes it from a closure.

- ```rust
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `None` value.

- ```rust
  pub fn is_some(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the option is a `Some` value.

- ```rust
  pub const fn as_ref(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Converts to an `Option<&T>`.

- ```rust
  pub fn as_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Converts to an `Option<&mut T>`.

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Option<Seal<''_, T>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedOption<T>>` to `Option<Seal<'_, T>>`.

- ```rust
  pub const fn iter(self: &Self) -> Iter<&T> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut T> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, T>> { /* ... */ }
  ```
  Returns an iterator over the sealed possibly-contained value.

- ```rust
  pub fn get_or_insert(self: &mut Self, v: T) -> &mut T { /* ... */ }
  ```
  Inserts `v` into the option if it is `None`, then returns a mutable

- ```rust
  pub fn get_or_insert_with<F: FnOnce() -> T>(self: &mut Self, f: F) -> &mut T { /* ... */ }
  ```
  Inserts a value computed from `f` into the option if it is `None`, then

- ```rust
  pub fn as_deref(self: &Self) -> Option<&<T as Deref>::Target> { /* ... */ }
  ```
  Converts from `&ArchivedOption<T>` to `Option<&T::Target>`.

- ```rust
  pub fn as_deref_mut(self: &mut Self) -> Option<&mut <T as Deref>::Target> { /* ... */ }
  ```
  Converts from `&mut ArchivedOption<T>` to `Option<&mut T::Target>`.

###### Trait Implementations

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedOption<T> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<Option<T>, <D as >::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Option<T>) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Portable**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Eq**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOption<<A as ArchiveWith<O>>::Archived>, d: &mut D) -> Result<Option<O>, <D as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: never) -> T { /* ... */ }
    ```

  - ```rust
    fn from(val: T) -> ArchivedOption<T> { /* ... */ }
    ```
    Moves `val` into a new [`Some`].

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Option<T>) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `Iter`

An iterator over a reference to the `Some` variant of an `ArchivedOption`.

This iterator yields one value if the `ArchivedOption` is a `Some`,
otherwise none.

```rust
pub struct Iter<P> {
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
  pub fn new(inner: Option<P>) -> Self { /* ... */ }
  ```
  Returns an iterator over the given `Option`.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

## Module `place`

An initialized, writeable location in memory.

```rust
pub mod place { /* ... */ }
```

### Types

#### Struct `Place`

A place to write a `T` paired with its position in the output buffer.

```rust
pub struct Place<T: ?Sized> {
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
  pub unsafe fn new_unchecked(pos: usize, ptr: *mut T) -> Self { /* ... */ }
  ```
  Creates a new `Place` from an output pointer.

- ```rust
  pub unsafe fn from_field_unchecked<U: ?Sized>(parent: Place<U>, ptr: *mut T) -> Self { /* ... */ }
  ```
  Creates a new `Place` from a parent pointer and the field the place

- ```rust
  pub fn pos(self: &Self) -> usize { /* ... */ }
  ```
  Returns the position of the place.

- ```rust
  pub unsafe fn ptr(self: &Self) -> *mut T { /* ... */ }
  ```
  Returns the pointer associated with this place.

- ```rust
  pub unsafe fn write_unchecked(self: &Self, value: T)
where
    T: Sized { /* ... */ }
  ```
  Writes the provided value to this place.

- ```rust
  pub fn write(self: &Self, value: T)
where
    T: NoUndef + Sized { /* ... */ }
  ```
  Writes the provided value to this place.

- ```rust
  pub unsafe fn cast_unchecked<U>(self: &Self) -> Place<U>
where
    T: Sized { /* ... */ }
  ```
  Returns this place casted to the given type.

- ```rust
  pub fn as_slice(self: &Self) -> &[u8]
where
    T: LayoutRaw { /* ... */ }
  ```
  Returns a slice of the bytes this place points to.

- ```rust
  pub unsafe fn index(self: &Self, i: usize) -> Place<T> { /* ... */ }
  ```
  Gets a `Place` to the `i`-th element of the slice.

- ```rust
  pub unsafe fn index(self: &Self, i: usize) -> Place<T> { /* ... */ }
  ```
  Gets a `Place` to the `i`-th element of the array.

###### Trait Implementations

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Pointee**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Destructure**
  - ```rust
    fn underlying(self: &mut Self) -> *mut <Self as >::Underlying { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Restructure**
  - ```rust
    unsafe fn restructure(self: &Self, ptr: *mut U) -> <Self as >::Restructured { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
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

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
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

## Module `primitive`

Definitions of archived primitives and type aliases based on enabled
features.

```rust
pub mod primitive { /* ... */ }
```

### Types

#### Type Alias `ArchivedI16`

The archived version of `i16`.

```rust
pub type ArchivedI16 = crate::rend::i16_le;
```

#### Type Alias `ArchivedI32`

The archived version of `i32`.

```rust
pub type ArchivedI32 = crate::rend::i32_le;
```

#### Type Alias `ArchivedI64`

The archived version of `i64`.

```rust
pub type ArchivedI64 = crate::rend::i64_le;
```

#### Type Alias `ArchivedI128`

The archived version of `i128`.

```rust
pub type ArchivedI128 = crate::rend::i128_le;
```

#### Type Alias `ArchivedU16`

The archived version of `u16`.

```rust
pub type ArchivedU16 = crate::rend::u16_le;
```

#### Type Alias `ArchivedU32`

The archived version of `u32`.

```rust
pub type ArchivedU32 = crate::rend::u32_le;
```

#### Type Alias `ArchivedU64`

The archived version of `u64`.

```rust
pub type ArchivedU64 = crate::rend::u64_le;
```

#### Type Alias `ArchivedU128`

The archived version of `u128`.

```rust
pub type ArchivedU128 = crate::rend::u128_le;
```

#### Type Alias `ArchivedF32`

The archived version of `f32`.

```rust
pub type ArchivedF32 = crate::rend::f32_le;
```

#### Type Alias `ArchivedF64`

The archived version of `f64`.

```rust
pub type ArchivedF64 = crate::rend::f64_le;
```

#### Type Alias `ArchivedChar`

The archived version of `char`.

```rust
pub type ArchivedChar = crate::rend::char_le;
```

#### Type Alias `FixedIsize`

The native type that `isize` is converted to for archiving.

This will be `i16`, `i32`, or `i64` when the `pointer_width_16`,
`pointer_width_32`, or `pointer_width_64` features are enabled,
respectively. With no pointer width features enabled, it defaults to `i32`.

```rust
pub type FixedIsize = i32;
```

#### Type Alias `ArchivedIsize`

The archived version of `isize` chosen based on the currently-enabled
`pointer_width_*` feature.

```rust
pub type ArchivedIsize = ArchivedI32;
```

#### Type Alias `FixedUsize`

The native type that `usize` is converted to for archiving.

This will be `u16`, `u32`, or `u64` when the `pointer_width_16`,
`pointer_width_32`, or `pointer_width_64` features are enabled,
respectively. With no pointer width features enabled, it defaults to `u32`.

```rust
pub type FixedUsize = u32;
```

#### Type Alias `ArchivedUsize`

The archived version of `isize` chosen based on the currently-enabled
`pointer_width_*` feature.

```rust
pub type ArchivedUsize = ArchivedU32;
```

#### Type Alias `ArchivedNonZeroI16`

The archived version of `NonZeroI16`.

```rust
pub type ArchivedNonZeroI16 = crate::rend::NonZeroI16_le;
```

#### Type Alias `ArchivedNonZeroI32`

The archived version of `NonZeroI32`.

```rust
pub type ArchivedNonZeroI32 = crate::rend::NonZeroI32_le;
```

#### Type Alias `ArchivedNonZeroI64`

The archived version of `NonZeroI64`.

```rust
pub type ArchivedNonZeroI64 = crate::rend::NonZeroI64_le;
```

#### Type Alias `ArchivedNonZeroI128`

The archived version of `NonZeroI128`.

```rust
pub type ArchivedNonZeroI128 = crate::rend::NonZeroI128_le;
```

#### Type Alias `ArchivedNonZeroU16`

The archived version of `NonZeroU16`.

```rust
pub type ArchivedNonZeroU16 = crate::rend::NonZeroU16_le;
```

#### Type Alias `ArchivedNonZeroU32`

The archived version of `NonZeroU32`.

```rust
pub type ArchivedNonZeroU32 = crate::rend::NonZeroU32_le;
```

#### Type Alias `ArchivedNonZeroU64`

The archived version of `NonZeroU64`.

```rust
pub type ArchivedNonZeroU64 = crate::rend::NonZeroU64_le;
```

#### Type Alias `ArchivedNonZeroU128`

The archived version of `NonZeroU128`.

```rust
pub type ArchivedNonZeroU128 = crate::rend::NonZeroU128_le;
```

#### Type Alias `FixedNonZeroIsize`

The native type that `NonZeroIsize` is converted to for archiving.

This will be `NonZeroI16`, `NonZeroI32`, or `NonZeroI64` when the
`pointer_width_16`, `pointer_width_32`, or `pointer_width_64` features are
enabled, respectively. With no pointer width features enabled, it defaults
to `NonZeroI32`.

```rust
pub type FixedNonZeroIsize = ::core::num::NonZeroI32;
```

#### Type Alias `ArchivedNonZeroIsize`

The archived version of `NonZeroIsize` chosen based on the currently-enabled
`pointer_width_*` feature.

```rust
pub type ArchivedNonZeroIsize = ArchivedNonZeroI32;
```

#### Type Alias `FixedNonZeroUsize`

The native type that `NonZeroUsize` is converted to for archiving.

This will be `NonZeroU16`, `NonZeroU32`, or `NonZeroU64` when the
`pointer_width_16`, `pointer_width_32`, or `pointer_width_64` features are
enabled, respectively. With no pointer width features enabled, it defaults
to `NonZeroU32`.

```rust
pub type FixedNonZeroUsize = ::core::num::NonZeroU32;
```

#### Type Alias `ArchivedNonZeroUsize`

The archived version of `NonZeroUsize` chosen based on the currently-enabled
`pointer_width_*` feature.

```rust
pub type ArchivedNonZeroUsize = ArchivedNonZeroU32;
```

## Module `rc`

Archived versions of shared pointers.

```rust
pub mod rc { /* ... */ }
```

### Types

#### Struct `RcFlavor`

The flavor type for [`Rc`](crate::alloc::rc::Rc).

```rust
pub struct RcFlavor;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
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

- **Flavor**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `ArcFlavor`

The flavor type for [`Arc`](crate::alloc::sync::Arc).

```rust
pub struct ArcFlavor;
```

##### Implementations

###### Trait Implementations

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Pointee**
- **Flavor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Unpin**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `ArchivedRc`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`

An archived `Rc`.

This is a thin wrapper around a [`RelPtr`] to the archived type paired with
a "flavor" type. Because there may be many varieties of shared pointers and
they may not be used together, the flavor helps check that memory is not
being shared incorrectly during validation.

```rust
pub struct ArchivedRc<T: ArchivePointee + ?Sized, F> {
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
  pub fn get(self: &Self) -> &T { /* ... */ }
  ```
  Gets the value of the `ArchivedRc`.

- ```rust
  pub unsafe fn get_seal_unchecked(this: Seal<''_, Self>) -> Seal<''_, T> { /* ... */ }
  ```
  Gets the sealed value of this `ArchivedRc`.

- ```rust
  pub fn resolve_from_ref<U: ArchiveUnsized<Archived = T> + ?Sized>(value: &U, resolver: RcResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived `Rc` from a given reference.

- ```rust
  pub fn serialize_from_ref<U, S>(value: &U, serializer: &mut S) -> Result<RcResolver, <S as >::Error>
where
    U: SerializeUnsized<S> + ?Sized,
    S: Fallible + Writer + Sharing + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an archived `Rc` from a given reference.

###### Trait Implementations

- **Sync**
- **Pointer**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Portable**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<sync::Arc<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<rc::Rc<T>, <D as >::Error> { /* ... */ }
    ```

- **Pointee**
- **Receiver**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedRc<U, UF>) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &sync::Arc<U>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &rc::Rc<U>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedRc<U, UF>) -> bool { /* ... */ }
    ```

- **Eq**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

#### Struct `RcResolver`

The resolver for `Rc`.

```rust
pub struct RcResolver {
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
  pub fn from_pos(pos: usize) -> Self { /* ... */ }
  ```
  Creates a new [`RcResolver`] from the position of a serialized value.

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
- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

#### Struct `ArchivedRcWeak`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`

An archived `rc::Weak`.

This is essentially just an optional [`ArchivedRc`].

```rust
pub struct ArchivedRcWeak<T: ArchivePointee + ?Sized, F> {
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
  pub fn upgrade(self: &Self) -> Option<&ArchivedRc<T, F>> { /* ... */ }
  ```
  Attempts to upgrade the weak pointer to an `ArchivedArc`.

- ```rust
  pub fn upgrade_seal(this: Seal<''_, Self>) -> Option<Seal<''_, ArchivedRc<T, F>>> { /* ... */ }
  ```
  Attempts to upgrade a sealed weak pointer.

- ```rust
  pub fn resolve_from_ref<U: ArchiveUnsized<Archived = T> + ?Sized>(value: Option<&U>, resolver: RcWeakResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived `Weak` from a given optional reference.

- ```rust
  pub fn serialize_from_ref<U, S>(value: Option<&U>, serializer: &mut S) -> Result<RcWeakResolver, <S as >::Error>
where
    U: SerializeUnsized<S, Archived = T> + ?Sized,
    S: Fallible + Writer + Sharing + ?Sized,
    <S as >::Error: Source { /* ... */ }
  ```
  Serializes an archived `Weak` from a given optional reference.

###### Trait Implementations

- **Send**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<sync::Weak<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<rc::Weak<T>, <D as >::Error> { /* ... */ }
    ```

- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Pointee**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Portable**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `RcWeakResolver`

The resolver for `rc::Weak`.

```rust
pub struct RcWeakResolver {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

### Traits

#### Trait `Flavor`

A type marker for `ArchivedRc`.

```rust
pub trait Flavor: ''static {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `ALLOW_CYCLES`: If `true`, cyclic `ArchivedRc`s with this flavor will not fail

##### Implementations

This trait is implemented for the following types:

- `RcFlavor`
- `ArcFlavor`

## Module `rel_ptr`

Relative pointer implementations and options.

```rust
pub mod rel_ptr { /* ... */ }
```

### Types

#### Struct `RawRelPtr`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`

An untyped pointer which resolves relative to its position in memory.

This is the most fundamental building block in rkyv. It allows the
construction and use of pointers that can be safely relocated as long as the
source and target are moved together. This is what allows memory to be moved
from disk into memory and accessed without decoding.

Regular pointers are *absolute*, meaning that the pointer can be moved
without being invalidated. However, the pointee **cannot** be moved,
otherwise the pointer is invalidated.

Relative pointers are *relative*, meaning that the **pointer** can be moved
with the **pointee** without invalidating the pointer. However, if either
the **pointer** or the **pointee** move independently, the pointer will be
invalidated.

```rust
pub struct RawRelPtr<O> {
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
  pub fn try_emplace_invalid<E: Source>(out: Place<Self>) -> Result<(), E> { /* ... */ }
  ```
  Attempts to create an invalid `RawRelPtr` in-place.

- ```rust
  pub fn emplace_invalid(out: Place<Self>) { /* ... */ }
  ```
  Creates an invalid `RawRelPtr` in-place.

- ```rust
  pub fn try_emplace<E: Source>(to: usize, out: Place<Self>) -> Result<(), E> { /* ... */ }
  ```
  Attempts to create a new `RawRelPtr` in-place between the given `from`

- ```rust
  pub fn emplace(to: usize, out: Place<Self>) { /* ... */ }
  ```
  Creates a new `RawRelPtr` in-place between the given `from` and `to`

- ```rust
  pub fn base_raw(this: *mut Self) -> *mut u8 { /* ... */ }
  ```
  Gets the base pointer for the pointed-to relative pointer.

- ```rust
  pub unsafe fn offset_raw(this: *mut Self) -> isize { /* ... */ }
  ```
  Gets the offset of the pointed-to relative pointer from its base.

- ```rust
  pub unsafe fn as_ptr_raw(this: *mut Self) -> *mut () { /* ... */ }
  ```
  Calculates the memory address being pointed to by the pointed-to

- ```rust
  pub unsafe fn as_ptr_wrapping_raw(this: *mut Self) -> *mut () { /* ... */ }
  ```
  Calculates the memory address being pointed to by the pointed-to

- ```rust
  pub unsafe fn is_invalid_raw(this: *mut Self) -> bool { /* ... */ }
  ```
  Gets whether the offset of the pointed-to relative pointer is invalid.

- ```rust
  pub fn base(self: &Self) -> *const u8 { /* ... */ }
  ```
  Gets the base pointer for the relative pointer.

- ```rust
  pub fn base_mut(this: Seal<''_, Self>) -> *mut u8 { /* ... */ }
  ```
  Gets the mutable base pointer for the relative pointer.

- ```rust
  pub fn offset(self: &Self) -> isize { /* ... */ }
  ```
  Gets the offset of the relative pointer from its base.

- ```rust
  pub fn is_invalid(self: &Self) -> bool { /* ... */ }
  ```
  Gets whether the offset of the relative pointer is invalid.

- ```rust
  pub unsafe fn as_ptr(self: &Self) -> *const () { /* ... */ }
  ```
  Calculates the memory address being pointed to by this relative pointer.

- ```rust
  pub unsafe fn as_mut_ptr(this: Seal<''_, Self>) -> *mut () { /* ... */ }
  ```
  Calculates the mutable memory address being pointed to by this relative

- ```rust
  pub fn as_ptr_wrapping(self: &Self) -> *const () { /* ... */ }
  ```
  Calculates the memory address being pointed to by this relative pointer

- ```rust
  pub fn as_mut_ptr_wrapping(this: Seal<''_, Self>) -> *mut () { /* ... */ }
  ```
  Calculates the mutable memory address being pointed to by this relative

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Pointer**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Portable**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Type Alias `RawRelPtrI8`

A raw relative pointer that uses an archived `i8` as the underlying offset.

```rust
pub type RawRelPtrI8 = RawRelPtr<i8>;
```

#### Type Alias `RawRelPtrI16`

A raw relative pointer that uses an archived `i16` as the underlying offset.

```rust
pub type RawRelPtrI16 = RawRelPtr<crate::primitive::ArchivedI16>;
```

#### Type Alias `RawRelPtrI32`

A raw relative pointer that uses an archived `i32` as the underlying offset.

```rust
pub type RawRelPtrI32 = RawRelPtr<crate::primitive::ArchivedI32>;
```

#### Type Alias `RawRelPtrI64`

A raw relative pointer that uses an archived `i64` as the underlying offset.

```rust
pub type RawRelPtrI64 = RawRelPtr<crate::primitive::ArchivedI64>;
```

#### Type Alias `RawRelPtrU8`

A raw relative pointer that uses an archived `u8` as the underlying offset.

```rust
pub type RawRelPtrU8 = RawRelPtr<u8>;
```

#### Type Alias `RawRelPtrU16`

A raw relative pointer that uses an archived `u16` as the underlying offset.

```rust
pub type RawRelPtrU16 = RawRelPtr<crate::primitive::ArchivedU16>;
```

#### Type Alias `RawRelPtrU32`

A raw relative pointer that uses an archived `u32` as the underlying offset.

```rust
pub type RawRelPtrU32 = RawRelPtr<crate::primitive::ArchivedU32>;
```

#### Type Alias `RawRelPtrU64`

A raw relative pointer that uses an archived `u64` as the underlying offset.

```rust
pub type RawRelPtrU64 = RawRelPtr<crate::primitive::ArchivedU64>;
```

#### Struct `RelPtr`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

A pointer which resolves to relative to its position in memory.

This is a strongly-typed version of [`RawRelPtr`].

See [`Archive`](crate::Archive) for an example of creating one.

```rust
pub struct RelPtr<T: ArchivePointee + ?Sized, O> {
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
  pub fn try_emplace<E: Source>(to: usize, out: Place<Self>) -> Result<(), E> { /* ... */ }
  ```
  Attempts to create a relative pointer from one position to another.

- ```rust
  pub fn emplace(to: usize, out: Place<Self>) { /* ... */ }
  ```
  Creates a relative pointer from one position to another.

- ```rust
  pub fn try_emplace_invalid<E: Source>(out: Place<Self>) -> Result<(), E> { /* ... */ }
  ```
  Attempts to create an invalid relative pointer with default metadata.

- ```rust
  pub fn emplace_invalid(out: Place<Self>) { /* ... */ }
  ```
  Creates an invalid relative pointer with default metadata.

- ```rust
  pub fn try_emplace_unsized<E: Source>(to: usize, metadata: <T as >::ArchivedMetadata, out: Place<Self>) -> Result<(), E> { /* ... */ }
  ```
  Attempts to create a relative pointer from one position to another.

- ```rust
  pub fn emplace_unsized(to: usize, metadata: <T as >::ArchivedMetadata, out: Place<Self>) { /* ... */ }
  ```
  Creates a relative pointer from one position to another.

- ```rust
  pub fn base_raw(this: *mut Self) -> *mut u8 { /* ... */ }
  ```
  Gets the base pointer for the pointed-to relative pointer.

- ```rust
  pub unsafe fn offset_raw(this: *mut Self) -> isize { /* ... */ }
  ```
  Gets the offset of the pointed-to relative pointer from its base.

- ```rust
  pub unsafe fn as_ptr_raw(this: *mut Self) -> *mut T { /* ... */ }
  ```
  Calculates the memory address being pointed to by the pointed-to

- ```rust
  pub unsafe fn as_ptr_wrapping_raw(this: *mut Self) -> *mut T { /* ... */ }
  ```
  Calculates the memory address being pointed to by the pointed-to

- ```rust
  pub unsafe fn is_invalid_raw(this: *mut Self) -> bool { /* ... */ }
  ```
  Gets whether the offset of the pointed-to relative pointer is invalid.

- ```rust
  pub fn base(self: &Self) -> *const u8 { /* ... */ }
  ```
  Gets the base pointer for the relative pointer.

- ```rust
  pub fn base_mut(this: Seal<''_, Self>) -> *mut u8 { /* ... */ }
  ```
  Gets the mutable base pointer for this relative pointer.

- ```rust
  pub fn offset(self: &Self) -> isize { /* ... */ }
  ```
  Gets the offset of the relative pointer from its base.

- ```rust
  pub fn is_invalid(self: &Self) -> bool { /* ... */ }
  ```
  Gets whether the offset of the relative pointer is 0.

- ```rust
  pub fn metadata(self: &Self) -> &<T as >::ArchivedMetadata { /* ... */ }
  ```
  Gets the metadata of the relative pointer.

- ```rust
  pub unsafe fn as_ptr(self: &Self) -> *const T { /* ... */ }
  ```
  Calculates the memory address being pointed to by this relative pointer.

- ```rust
  pub unsafe fn as_mut_ptr(this: Seal<''_, Self>) -> *mut T { /* ... */ }
  ```
  Calculates the mutable memory address being pointed to by this relative

- ```rust
  pub fn as_ptr_wrapping(self: &Self) -> *const T { /* ... */ }
  ```
  Calculates the memory address being pointed to by this relative pointer

- ```rust
  pub fn as_mut_ptr_wrapping(this: Seal<''_, Self>) -> *mut T { /* ... */ }
  ```
  Calculates the mutable memory address being pointed to by this relative

###### Trait Implementations

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
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

- **Pointer**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Portable**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

### Traits

#### Trait `Offset`

A offset that can be used with [`RawRelPtr`].

```rust
pub trait Offset: Copy + NoUndef {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `from_isize`: Creates a new offset between a `from` position and a `to` position.
- `to_isize`: Gets the offset as an `isize`.

##### Implementations

This trait is implemented for the following types:

- `i8`
- `u8`
- `crate::primitive::ArchivedI16`
- `crate::primitive::ArchivedI32`
- `crate::primitive::ArchivedI64`
- `crate::primitive::ArchivedU16`
- `crate::primitive::ArchivedU32`
- `crate::primitive::ArchivedU64`

### Functions

#### Function `signed_offset`

Calculates the offset between two positions as an `isize`.

This function exists solely to get the distance between two `usizes` as an
`isize` with a full range of values.

# Examples

```
# use rkyv::rel_ptr::signed_offset;
# use rancor::Error;
assert!(signed_offset::<Error>(0, 1).is_ok_and(|x| x == 1));
assert!(signed_offset::<Error>(1, 0).is_ok_and(|x| x == -1));
assert!(signed_offset::<Error>(0, isize::MAX as usize)
    .is_ok_and(|x| x == isize::MAX));
assert!(signed_offset::<Error>(isize::MAX as usize, 0)
    .is_ok_and(|x| x == -isize::MAX));
assert!(signed_offset::<Error>(0, isize::MAX as usize + 1).is_err());
assert!(signed_offset::<Error>(isize::MAX as usize + 1, 0)
    .is_ok_and(|x| x == isize::MIN));
assert!(signed_offset::<Error>(0, isize::MAX as usize + 2).is_err());
assert!(signed_offset::<Error>(isize::MAX as usize + 2, 0).is_err());
```

```rust
pub fn signed_offset<E: Source>(from: usize, to: usize) -> Result<isize, E> { /* ... */ }
```

## Module `result`

An archived version of `Result`.

```rust
pub mod result { /* ... */ }
```

### Types

#### Enum `ArchivedResult`

**Attributes:**

- `#[rkyv(crate)]`
- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes))]`
- `#[repr(u8)]`

An archived [`Result`] that represents either success
([`Ok`](ArchivedResult::Ok)) or failure ([`Err`](ArchivedResult::Err)).

```rust
pub enum ArchivedResult<T, E> {
    Ok(T),
    Err(E),
}
```

##### Variants

###### `Ok`

Contains the success value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Err`

Contains the error value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `E` |  |

##### Implementations

###### Methods

- ```rust
  pub fn ok(self: Self) -> Option<T> { /* ... */ }
  ```
  Converts from `ArchivedResult<T, E>` to `Option<T>`.

- ```rust
  pub fn unwrap(self: Self) -> T { /* ... */ }
  ```
  Returns the contained [`Ok`](ArchivedResult::Ok) value, consuming the

- ```rust
  pub fn unwrap_or_else<F>(self: Self, op: F) -> T
where
    F: FnOnce(E) -> T { /* ... */ }
  ```
  Returns the contained `Ok` value or computes it from a closure.

- ```rust
  pub const fn is_ok(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the result is [`Ok`](ArchivedResult::Ok).

- ```rust
  pub const fn is_err(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the result is [`Err`](ArchivedResult::Err).

- ```rust
  pub fn as_ref(self: &Self) -> Result<&T, &E> { /* ... */ }
  ```
  Returns a `Result` containing the success and error values of this

- ```rust
  pub fn as_mut(self: &mut Self) -> Result<&mut T, &mut E> { /* ... */ }
  ```
  Converts from `&mut ArchivedResult<T, E>` to `Result<&mut T, &mut E>`.

- ```rust
  pub fn as_seal(this: Seal<''_, Self>) -> Result<Seal<''_, T>, Seal<''_, E>> { /* ... */ }
  ```
  Converts from `Seal<'_, ArchivedResult<T, E>>` to

- ```rust
  pub fn iter(self: &Self) -> Iter<&T> { /* ... */ }
  ```
  Returns an iterator over the possibly-contained value.

- ```rust
  pub fn iter_mut(self: &mut Self) -> Iter<&mut T> { /* ... */ }
  ```
  Returns an iterator over the mutable possibly-contained value.

- ```rust
  pub fn iter_seal(this: Seal<''_, Self>) -> Iter<Seal<''_, T>> { /* ... */ }
  ```
  Returns an iterator over the sealed possibly-contained value.

- ```rust
  pub fn as_deref(self: &Self) -> Result<&<T as Deref>::Target, &E> { /* ... */ }
  ```
  Converts from `&ArchivedResult<T, E>` to `Result<&<T as Deref>::Target,

- ```rust
  pub fn as_deref_mut(self: &mut Self) -> Result<&mut <T as Deref>::Target, &mut E> { /* ... */ }
  ```
  Converts from `&mut ArchivedResult<T, E>` to `Result<&mut <T as

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
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

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Pointee**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Portable**
- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<Result<T, U>, <D as >::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Result<T, E>) -> bool { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
#### Type Alias `Iter`

An iterator over a reference to the `Ok` variant of an [`ArchivedResult`].

The iterator yields one value if the result is `Ok`, otherwise none.

Created by [`ArchivedResult::iter`].

```rust
pub type Iter<P> = crate::option::Iter<P>;
```

## Module `seal`

Mutable references to values which may not be moved or de-initialized.

```rust
pub mod seal { /* ... */ }
```

### Types

#### Struct `Seal`

A mutable reference which may not be moved or assigned.

A `Seal` restricts a mutable reference so that the referenced value cannot
be moved or assigned unless it is `Unpin` and `NoUndef`. These properties
allow the safe use of mutable archived values.

Unlike `Pin`, all fields of `Seal`ed values are also sealed. There is no
notion of "structural sealing" as there is structural pinning. This has the
upside that a `Seal` can be uniformly destructured with `munge`, which is
the recommended replacement for `Pin`'s `map_unchecked_mut` function. Also
unlike `Pin`, `Seal`ing a reference does not require upholding the invariant
that the sealed value is dropped before its backing memory is reused. This
means that creating a `Seal` from a mutable reference is completely safe to
do.

```rust
pub struct Seal<''a, T: ?Sized> {
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
  pub fn new(inner: &''a mut T) -> Self { /* ... */ }
  ```
  Returns a new `Seal` wrapping the given reference.

- ```rust
  pub fn unseal(self: Self) -> &''a mut T
where
    T: NoUndef + Unpin { /* ... */ }
  ```
  Returns the underlying reference for types that implement `NoUndef`

- ```rust
  pub fn unseal_ref(self: Self) -> &''a T { /* ... */ }
  ```
  Returns the underlying reference as shared for types that implement

- ```rust
  pub unsafe fn unseal_unchecked(self: Self) -> &''a mut T { /* ... */ }
  ```
  Returns the underlying reference.

- ```rust
  pub fn as_mut(self: &mut Self) -> Seal<''_, T> { /* ... */ }
  ```
  Mutably reborrows the `Seal`.

- ```rust
  pub fn index<I: SliceIndex<[T]>>(self: Self, index: I) -> Seal<''a, <I as SliceIndex<[T]>>::Output> { /* ... */ }
  ```
  Indexes the `Seal`.

###### Trait Implementations

- **UnwindSafe**
- **Pointee**
- **RefUnwindSafe**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Sync**
- **Restructure**
  - ```rust
    unsafe fn restructure(self: &Self, ptr: *mut U) -> <Self as >::Restructured { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Receiver**
- **Destructure**
  - ```rust
    fn underlying(self: &mut Self) -> *mut <Self as >::Underlying { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

## Module `ser`

Serialization traits and adapters.

```rust
pub mod ser { /* ... */ }
```

### Modules

## Module `allocator`

Allocators for serializers to use during serialization.

```rust
pub mod allocator { /* ... */ }
```

### Types

#### Struct `AllocationStats`

Statistics for the allocations which occurred during serialization.

```rust
pub struct AllocationStats {
    pub max_bytes_allocated: usize,
    pub max_allocations: usize,
    pub max_alignment: usize,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `max_bytes_allocated` | `usize` | Returns the maximum number of bytes that were concurrently allocated. |
| `max_allocations` | `usize` | Returns the maximum number of concurrent allocations. |
| `max_alignment` | `usize` | Returns the maximum alignment of requested allocations. |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn min_arena_capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the minimum arena capacity required to serialize the same data.

- ```rust
  pub fn min_arena_capacity_max_error(self: &Self) -> usize { /* ... */ }
  ```
  Returns the maximum error term for the minimum arena capacity

###### Trait Implementations

- **Unpin**
- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `AllocationTracker`

A passthrough allocator that tracks usage.

```rust
pub struct AllocationTracker<T> {
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
  pub fn new(inner: T) -> Self { /* ... */ }
  ```
  Returns a new allocation tracker wrapping the given allocator.

- ```rust
  pub fn into_stats(self: Self) -> AllocationStats { /* ... */ }
  ```
  Returns the allocation stats accumulated during serialization.

###### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: never) -> T { /* ... */ }
    ```

  - ```rust
    fn from(inner: T) -> Self { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Freeze**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Allocator**
  - ```rust
    unsafe fn push_alloc(self: &mut Self, layout: Layout) -> Result<NonNull<[u8]>, E> { /* ... */ }
    ```

  - ```rust
    unsafe fn pop_alloc(self: &mut Self, ptr: NonNull<u8>, layout: Layout) -> Result<(), E> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Traits

#### Trait `Allocator`

A serializer that can allocate scratch space.

# Safety

`push_alloc` must return a pointer to unaliased memory which fits the
provided layout.

```rust
pub unsafe trait Allocator<E = <Self as Fallible>::Error> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

##### Required Items

###### Required Methods

- `push_alloc`: Allocates scratch space of the requested size.
- `pop_alloc`: Deallocates previously allocated scratch space.

##### Implementations

This trait is implemented for the following types:

- `ArenaHandle<''_>` with <E>
- `SubAllocator<''_>` with <E>
- `rancor::Strategy<T, E>` with <T: Allocator<E>, E>
- `AllocationTracker<T>` with <T: Allocator<E>, E>
- `Serializer<W, A, S>` with <W, A: Allocator<E>, S, E>

### Re-exports

#### Re-export `self::alloc::*`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::alloc::*;
```

#### Re-export `self::core::*`

```rust
pub use self::core::*;
```

## Module `sharing`

Shared pointer serialization.

```rust
pub mod sharing { /* ... */ }
```

### Types

#### Enum `SharingState`

The result of starting to serialize a shared pointer.

```rust
pub enum SharingState {
    Started,
    Pending,
    Finished(usize),
}
```

##### Variants

###### `Started`

The caller started sharing this value. They should proceed to serialize
the shared value and call `finish_sharing`.

###### `Pending`

Another caller started sharing this value, but has not finished yet.
This can only occur with cyclic shared pointer structures, and so rkyv
treats this as an error by default.

###### `Finished`

This value has already been shared. The caller should use the returned
address to share its value.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Traits

#### Trait `Sharing`

A shared pointer serialization strategy.

This trait is required to serialize `Rc` and `Arc`.

```rust
pub trait Sharing<E = <Self as Fallible>::Error> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `start_sharing`: Starts sharing the value associated with the given address.
- `finish_sharing`: Finishes sharing the value associated with the given address.

##### Implementations

This trait is implemented for the following types:

- `Share` with <E: Source>
- `Unshare` with <E>
- `&mut T` with <T, E>
- `rancor::Strategy<T, E>` with <T, E>
- `Serializer<W, A, S>` with <W, A, S: Sharing<E>, E>

#### Trait `SharingExt`

Helper methods for [`Sharing`].

```rust
pub trait SharingExt<E>: Sharing<E> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn serialize_shared<T: SerializeUnsized<Self> + ?Sized>(self: &mut Self, value: &T) -> Result<usize, <Self as Fallible>::Error>
where
    Self: Fallible<Error = E>,
    E: Source { /* ... */ }
  ```
  Serializes the given shared value and returns its position. If the value

##### Implementations

This trait is implemented for the following types:

- `S` with <S, E>

### Re-exports

#### Re-export `self::alloc::*`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::alloc::*;
```

#### Re-export `self::core::*`

```rust
pub use self::core::*;
```

## Module `writer`

Writing backends for serializers.

```rust
pub mod writer { /* ... */ }
```

### Traits

#### Trait `Positional`

A writer that knows its current position.

```rust
pub trait Positional {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `pos`: Returns the current position of the writer.

##### Implementations

This trait is implemented for the following types:

- `crate::alloc::vec::Vec<u8>`
- `crate::util::AlignedVec<A>` with <const A: usize>
- `Buffer<''_>`
- `IoWriter<W>` with <W>
- `&T` with <T>
- `&mut T` with <T>
- `rancor::Strategy<T, E>` with <T, E>
- `Serializer<W, A, S>` with <W: Positional, A, S>

#### Trait `Writer`

A type that writes bytes to some output.

A type that is [`Write`](::std::io::Write) can be wrapped in an [`IoWriter`]
to equip it with `Writer`.

It's important that the memory for archived objects is properly aligned
before attempting to read objects out of it; use an
[`AlignedVec`](crate::util::AlignedVec) or the [`Align`](crate::util::Align)
wrapper as appropriate.

```rust
pub trait Writer<E = <Self as Fallible>::Error>: Positional {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `write`: Attempts to write the given bytes to the serializer.

##### Implementations

This trait is implemented for the following types:

- `crate::alloc::vec::Vec<u8>` with <E>
- `crate::util::AlignedVec<A>` with <E, const A: usize>
- `Buffer<''_>` with <E: Source>
- `IoWriter<W>` with <W: io::Write, E: Source>
- `&mut T` with <T, E>
- `rancor::Strategy<T, E>` with <T, E>
- `Serializer<W, A, S>` with <W: Writer<E>, A, S, E>

#### Trait `WriterExt`

Helper methods for [`Writer`].

```rust
pub trait WriterExt<E>: Writer<E> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn pad(self: &mut Self, padding: usize) -> Result<(), E> { /* ... */ }
  ```
  Advances the given number of bytes as padding.

- ```rust
  fn align(self: &mut Self, align: usize) -> Result<usize, E> { /* ... */ }
  ```
  Aligns the position of the serializer to the given alignment.

- ```rust
  fn align_for<T>(self: &mut Self) -> Result<usize, E> { /* ... */ }
  ```
  Aligns the position of the serializer to be suitable to write the given

- ```rust
  unsafe fn resolve_aligned<T: Archive + ?Sized>(self: &mut Self, value: &T, resolver: <T as >::Resolver) -> Result<usize, E> { /* ... */ }
  ```
  Resolves the given value with its resolver and writes the archived type.

- ```rust
  unsafe fn resolve_unsized_aligned<T: ArchiveUnsized + ?Sized>(self: &mut Self, value: &T, to: usize) -> Result<usize, E> { /* ... */ }
  ```
  Resolves the given reference with its resolver and writes the archived

##### Implementations

This trait is implemented for the following types:

- `T` with <T, E>

### Re-exports

#### Re-export `self::core::*`

```rust
pub use self::core::*;
```

#### Re-export `self::std::*`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use self::std::*;
```

### Types

#### Struct `Serializer`

A serializer built from composeable pieces.

```rust
pub struct Serializer<W, A, S> {
    pub writer: W,
    pub allocator: A,
    pub sharing: S,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `writer` | `W` | The writer of the serializer. |
| `allocator` | `A` | The allocator of the serializer. |
| `sharing` | `S` | The pointer sharing of the serializer. |

##### Implementations

###### Methods

- ```rust
  pub fn new(writer: W, allocator: A, sharing: S) -> Self { /* ... */ }
  ```
  Creates a new serializer from a writer, allocator, and pointer sharing.

- ```rust
  pub fn into_raw_parts(self: Self) -> (W, A, S) { /* ... */ }
  ```
  Consumes the serializer and returns the components.

- ```rust
  pub fn into_writer(self: Self) -> W { /* ... */ }
  ```
  Consumes the serializer and returns the writer.

###### Trait Implementations

- **WriterExt**
- **RefUnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **UnwindSafe**
- **SharingExt**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Serializer<W, A, S> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocator**
  - ```rust
    unsafe fn push_alloc(self: &mut Self, layout: Layout) -> Result<NonNull<[u8]>, E> { /* ... */ }
    ```

  - ```rust
    unsafe fn pop_alloc(self: &mut Self, ptr: NonNull<u8>, layout: Layout) -> Result<(), E> { /* ... */ }
    ```

- **Sync**
- **Pointee**
- **Writer**
  - ```rust
    fn write(self: &mut Self, bytes: &[u8]) -> Result<(), E> { /* ... */ }
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

- **Positional**
  - ```rust
    fn pos(self: &Self) -> usize { /* ... */ }
    ```

- **Sharing**
  - ```rust
    fn start_sharing(self: &mut Self, address: usize) -> sharing::SharingState { /* ... */ }
    ```

  - ```rust
    fn finish_sharing(self: &mut Self, address: usize, pos: usize) -> Result<(), E> { /* ... */ }
    ```

### Re-exports

#### Re-export `Allocator`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::allocator::Allocator;
```

#### Re-export `Sharing`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::sharing::Sharing;
```

#### Re-export `SharingExt`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::sharing::SharingExt;
```

#### Re-export `Positional`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::writer::Positional;
```

#### Re-export `Writer`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::writer::Writer;
```

#### Re-export `WriterExt`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::writer::WriterExt;
```

## Module `string`

Archived versions of string types.

```rust
pub mod string { /* ... */ }
```

### Modules

## Module `repr`

An archived string representation that supports inlining short strings.

```rust
pub mod repr { /* ... */ }
```

### Types

#### Union `ArchivedStringRepr`

**Attributes:**

- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived string representation that can inline short strings.

```rust
pub union ArchivedStringRepr {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- `is_inline`: Returns whether the representation is inline.
- `out_of_line_offset`: Returns the offset of the representation.
- `as_ptr`: Returns a pointer to the bytes of the string.
- `as_mut_ptr`: Returns a mutable pointer to the bytes of the string.
- `len`: Returns the length of the string.
- `is_empty`: Returns whether the string is empty.
- `as_str_ptr`: Returns a pointer to the string as a `str`.
- `as_bytes`: Returns a slice of the bytes of the string.
- `as_bytes_seal`: Returns a mutable slice of the bytes of the string.
- `as_str`: Returns a reference to the string as a `str`.
- `as_str_seal`: Returns a mutable reference to the string as a `str`.
- `emplace_inline`: Emplaces a new inline representation for the given `str`.
- `try_emplace_out_of_line`: Emplaces a new out-of-line representation for the given `str`.
- `emplace_out_of_line`: Emplaces a new out-of-line representation for the given `str`.

###### Trait Implementations

- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **Unpin**
- **Pointee**
  - `Metadata`: 
- **Send**
- **Into**
  - `into`: Calls `U::from(self)`.
- **BorrowMut**
  - `borrow_mut`: 
- **Freeze**
- **RefUnwindSafe**
- **Niching**
  - `is_niched`: 
  - `resolve_niched`: 
- **From**
  - `from`: Returns the argument unchanged.
- **Any**
  - `type_id`: 
- **Portable**
- **LayoutRaw**
  - `layout_raw`: 
- **ArchivePointee**
  - `ArchivedMetadata`: 
  - `pointer_metadata`: 
- **CheckBytes**
  - `check_bytes`: 
- **UnwindSafe**
- **Borrow**
  - `borrow`: 
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Sync**

### Constants and Statics

#### Constant `INLINE_CAPACITY`

The maximum number of bytes that can be inlined.

```rust
pub const INLINE_CAPACITY: usize = _;
```

#### Constant `OUT_OF_LINE_CAPACITY`

The maximum number of bytes that can be out-of-line.

```rust
pub const OUT_OF_LINE_CAPACITY: usize = _;
```

### Types

#### Struct `ArchivedString`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`
- `#[rkyv(crate)]`

An archived [`String`].

This has inline and out-of-line representations. Short strings will use the
available space inside the structure to store the string, and long strings
will store a [`RelPtr`](crate::RelPtr) to a `str` instead.

```rust
pub struct ArchivedString {
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
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Extracts a string slice containing the entire `ArchivedString`.

- ```rust
  pub fn as_str_seal(this: Seal<''_, Self>) -> Seal<''_, str> { /* ... */ }
  ```
  Extracts a sealed mutable string slice containing the entire

- ```rust
  pub fn resolve_from_str(value: &str, resolver: StringResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived string from a given `str`.

- ```rust
  pub fn serialize_from_str<S: Fallible + ?Sized>(value: &str, serializer: &mut S) -> Result<StringResolver, <S as >::Error>
where
    <S as >::Error: Source,
    str: SerializeUnsized<S> { /* ... */ }
  ```
  Serializes an archived string from a given `str`.

###### Trait Implementations

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: Range<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: RangeFrom<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: RangeFull) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: RangeInclusive<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: RangeTo<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: RangeToInclusive<usize>) -> &<Self as >::Output { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Receiver**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &String) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedString) -> bool { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &String) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedString) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&str) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedString) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedString) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Portable**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedString, deserializer: &mut D) -> Result<Cow<''a, str>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedString, _: &mut D) -> Result<OsString, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedString, _: &mut D) -> Result<PathBuf, <D as >::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, _: &mut D) -> Result<String, <D as >::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &str { /* ... */ }
    ```

#### Struct `StringResolver`

The resolver for `String`.

```rust
pub struct StringResolver {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
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

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Sync**
## Module `time`

Archived versions of `time` types.

```rust
pub mod time { /* ... */ }
```

### Types

#### Struct `ArchivedDuration`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`Duration`](core::time::Duration).

```rust
pub struct ArchivedDuration {
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
  pub const fn as_secs(self: &Self) -> u64 { /* ... */ }
  ```
  Returns the number of _whole_ seconds contained by this

- ```rust
  pub const fn subsec_millis(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the fractional part of this `ArchivedDuration`, in whole

- ```rust
  pub const fn subsec_micros(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the fractional part of this `ArchivedDuration`, in whole

- ```rust
  pub const fn subsec_nanos(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the fractional part of this `Duration`, in nanoseconds.

- ```rust
  pub const fn as_millis(self: &Self) -> u128 { /* ... */ }
  ```
  Returns the total number of whole milliseconds contained by this

- ```rust
  pub const fn as_micros(self: &Self) -> u128 { /* ... */ }
  ```
  Returns the total number of whole microseconds contained by this

- ```rust
  pub const fn as_nanos(self: &Self) -> u128 { /* ... */ }
  ```
  Returns the total number of nanoseconds contained by this

- ```rust
  pub fn as_secs_f64(self: &Self) -> f64 { /* ... */ }
  ```
  Returns the number of seconds contained by this `ArchivedDuration` as

- ```rust
  pub fn as_secs_f32(self: &Self) -> f32 { /* ... */ }
  ```
  Returns the number of seconds contained by this `ArchivedDuration` as

- ```rust
  pub unsafe fn emplace(secs: u64, nanos: u32, out: *mut ArchivedDuration) { /* ... */ }
  ```
  Constructs an archived duration at the given position.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArchivedDuration { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Duration) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedDuration) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedDuration) -> bool { /* ... */ }
    ```

- **Copy**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, _: &mut D) -> Result<Duration, <D as >::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedDuration { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedDuration) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Verify**
  - ```rust
    fn verify(self: &Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Pointee**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Portable**
- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedDuration, _: &mut D) -> Result<SystemTime, <D as >::Error> { /* ... */ }
    ```

- **Freeze**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedDuration) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

  - ```rust
    fn from(duration: ArchivedDuration) -> Self { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `traits`

The core traits provided by rkyv.

```rust
pub mod traits { /* ... */ }
```

### Types

#### Struct `CopyOptimization`

An optimization hint about whether `T` is trivially copyable.

```rust
pub struct CopyOptimization<T: ?Sized>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const unsafe fn enable() -> Self { /* ... */ }
  ```
  Returns a `CopyOptimization` hint with the optimization enabled for `T`.

- ```rust
  pub const unsafe fn enable_if(value: bool) -> Self { /* ... */ }
  ```
  Returns a `CopyOptimization` hint with the optimization enabled for `T`

- ```rust
  pub const fn disable() -> Self { /* ... */ }
  ```
  Returns a `CopyOptimization` hint with the optimization disabled for

- ```rust
  pub const fn is_enabled(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the optimization is enabled for `T`.

###### Trait Implementations

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `Portable`

A type with a stable, well-defined layout that is the same on all targets.

# Safety

The implementing type must have a stable, well-defined layout that is the
same on all targets. Structs and unions must be `#[repr(transparent)]` or
`#[repr(C)]`. Enums must be `#[repr(C)]`, `#[repr(int)]`, or `#[repr(C,
int)]`.

The implementing type must not have interior mutability (i.e. no
`UnsafeCell`s).

```rust
pub unsafe trait Portable {
    /* Associated items */
}
```

> This trait is unsafe to implement.

##### Implementations

This trait is implemented for the following types:

- `ArchivedBox<T>` with <T: ArchivePointee + ?Sized>
- `ArchivedBTreeMap<K, V, E>` with <K, V, const E: usize>
- `ArchivedBTreeSet<K, E>` with <K, const E: usize>
- `ArchivedIndexMap<K, V, H>` with <K, V, H>
- `ArchivedIndexSet<K, H>` with <K, H>
- `ArchivedHashMap<K, V, H>` with <K, V, H>
- `ArchivedHashSet<K, H>` with <K, H>
- `ArchivedHashTable<T>` with <T>
- `Entry<K, V>` with <K, V>
- `ArchivedCString`
- `core::ffi::CStr`
- `()`
- `bool`
- `i8`
- `u8`
- `core::num::NonZeroI8`
- `core::num::NonZeroU8`
- `rend::NonZeroI16_be`
- `rend::NonZeroI16_le`
- `rend::NonZeroI32_be`
- `rend::NonZeroI32_le`
- `rend::NonZeroI64_be`
- `rend::NonZeroI64_le`
- `rend::NonZeroI128_be`
- `rend::NonZeroI128_le`
- `rend::NonZeroU16_be`
- `rend::NonZeroU16_le`
- `rend::NonZeroU32_be`
- `rend::NonZeroU32_le`
- `rend::NonZeroU64_be`
- `rend::NonZeroU64_le`
- `rend::NonZeroU128_be`
- `rend::NonZeroU128_le`
- `rend::char_be`
- `rend::char_le`
- `rend::f32_be`
- `rend::f32_le`
- `rend::f64_be`
- `rend::f64_le`
- `rend::i16_be`
- `rend::i16_le`
- `rend::i32_be`
- `rend::i32_le`
- `rend::i64_be`
- `rend::i64_le`
- `rend::i128_be`
- `rend::i128_le`
- `rend::u16_be`
- `rend::u16_le`
- `rend::u32_be`
- `rend::u32_le`
- `rend::u64_be`
- `rend::u64_le`
- `rend::u128_be`
- `rend::u128_le`
- `rend::unaligned::NonZeroI16_ube`
- `rend::unaligned::NonZeroI16_ule`
- `rend::unaligned::NonZeroI32_ube`
- `rend::unaligned::NonZeroI32_ule`
- `rend::unaligned::NonZeroI64_ube`
- `rend::unaligned::NonZeroI64_ule`
- `rend::unaligned::NonZeroI128_ube`
- `rend::unaligned::NonZeroI128_ule`
- `rend::unaligned::NonZeroU16_ube`
- `rend::unaligned::NonZeroU16_ule`
- `rend::unaligned::NonZeroU32_ube`
- `rend::unaligned::NonZeroU32_ule`
- `rend::unaligned::NonZeroU64_ube`
- `rend::unaligned::NonZeroU64_ule`
- `rend::unaligned::NonZeroU128_ube`
- `rend::unaligned::NonZeroU128_ule`
- `rend::unaligned::char_ube`
- `rend::unaligned::char_ule`
- `rend::unaligned::f32_ube`
- `rend::unaligned::f32_ule`
- `rend::unaligned::f64_ube`
- `rend::unaligned::f64_ule`
- `rend::unaligned::i16_ube`
- `rend::unaligned::i16_ule`
- `rend::unaligned::i32_ube`
- `rend::unaligned::i32_ule`
- `rend::unaligned::i64_ube`
- `rend::unaligned::i64_ule`
- `rend::unaligned::i128_ube`
- `rend::unaligned::i128_ule`
- `rend::unaligned::u16_ube`
- `rend::unaligned::u16_ule`
- `rend::unaligned::u32_ube`
- `rend::unaligned::u32_ule`
- `rend::unaligned::u64_ube`
- `rend::unaligned::u64_ule`
- `rend::unaligned::u128_ube`
- `rend::unaligned::u128_ule`
- `[T; N]` with <T: Portable, const N: usize>
- `[T]` with <T: Portable>
- `str`
- `core::marker::PhantomData<T>` with <T: ?Sized>
- `core::marker::PhantomPinned`
- `core::mem::ManuallyDrop<T>` with <T: Portable>
- `core::mem::MaybeUninit<T>` with <T: Portable>
- `ArchivedIpv4Addr`
- `ArchivedIpv6Addr`
- `ArchivedIpAddr`
- `ArchivedSocketAddrV4`
- `ArchivedSocketAddrV6`
- `ArchivedSocketAddr`
- `NichedOption<T, N>` with <T, N: ?Sized>
- `ArchivedOptionBox<T>` with <T: ArchivePointee + ?Sized>
- `ArchivedOptionNonZeroI8`
- `ArchivedOptionNonZeroI16`
- `ArchivedOptionNonZeroI32`
- `ArchivedOptionNonZeroI64`
- `ArchivedOptionNonZeroI128`
- `ArchivedOptionNonZeroU8`
- `ArchivedOptionNonZeroU16`
- `ArchivedOptionNonZeroU32`
- `ArchivedOptionNonZeroU64`
- `ArchivedOptionNonZeroU128`
- `ArchivedRangeFull`
- `ArchivedRange<T>` with <T>
- `ArchivedRangeInclusive<T>` with <T>
- `ArchivedRangeFrom<T>` with <T>
- `ArchivedRangeTo<T>` with <T>
- `ArchivedRangeToInclusive<T>` with <T>
- `ArchivedBound<T>` with <T>
- `ArchivedOption<T>` with <T>
- `ArchivedRc<T, F>` with <T: ArchivePointee + ?Sized, F>
- `ArchivedRcWeak<T, F>` with <T: ArchivePointee + ?Sized, F>
- `RawRelPtr<O>` with <O>
- `RelPtr<T, O>` with <T: ArchivePointee + ?Sized, O>
- `ArchivedResult<T, E>` with <T, E>
- `ArchivedStringRepr`
- `ArchivedString`
- `ArchivedDuration`
- `ArchivedTuple1<T0>` with <T0>
- `ArchivedTuple2<T0, T1>` with <T0, T1>
- `ArchivedTuple3<T0, T1, T2>` with <T0, T1, T2>
- `ArchivedTuple4<T0, T1, T2, T3>` with <T0, T1, T2, T3>
- `ArchivedTuple5<T0, T1, T2, T3, T4>` with <T0, T1, T2, T3, T4>
- `ArchivedTuple6<T0, T1, T2, T3, T4, T5>` with <T0, T1, T2, T3, T4, T5>
- `ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>` with <T0, T1, T2, T3, T4, T5, T6>
- `ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>` with <T0, T1, T2, T3, T4, T5, T6, T7>
- `ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8>
- `ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
- `ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
- `ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
- `ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
- `ArchivedVec<T>` with <T>

#### Trait `NoUndef`

A type with no undefined bytes.

# Safety

The bytes of types implementing `NoUndef` must always be well-defined. Among
other things, this means that `NoUndef` types may not contain padding or
uninitialized `MaybeUninit`s.

```rust
pub unsafe trait NoUndef {
    /* Associated items */
}
```

> This trait is unsafe to implement.

##### Implementations

This trait is implemented for the following types:

- `()`
- `bool`
- `i8`
- `u8`
- `core::num::NonZeroI8`
- `core::num::NonZeroU8`
- `rend::NonZeroI16_be`
- `rend::NonZeroI16_le`
- `rend::NonZeroI32_be`
- `rend::NonZeroI32_le`
- `rend::NonZeroI64_be`
- `rend::NonZeroI64_le`
- `rend::NonZeroI128_be`
- `rend::NonZeroI128_le`
- `rend::NonZeroU16_be`
- `rend::NonZeroU16_le`
- `rend::NonZeroU32_be`
- `rend::NonZeroU32_le`
- `rend::NonZeroU64_be`
- `rend::NonZeroU64_le`
- `rend::NonZeroU128_be`
- `rend::NonZeroU128_le`
- `rend::char_be`
- `rend::char_le`
- `rend::f32_be`
- `rend::f32_le`
- `rend::f64_be`
- `rend::f64_le`
- `rend::i16_be`
- `rend::i16_le`
- `rend::i32_be`
- `rend::i32_le`
- `rend::i64_be`
- `rend::i64_le`
- `rend::i128_be`
- `rend::i128_le`
- `rend::u16_be`
- `rend::u16_le`
- `rend::u32_be`
- `rend::u32_le`
- `rend::u64_be`
- `rend::u64_le`
- `rend::u128_be`
- `rend::u128_le`
- `rend::unaligned::NonZeroI16_ube`
- `rend::unaligned::NonZeroI16_ule`
- `rend::unaligned::NonZeroI32_ube`
- `rend::unaligned::NonZeroI32_ule`
- `rend::unaligned::NonZeroI64_ube`
- `rend::unaligned::NonZeroI64_ule`
- `rend::unaligned::NonZeroI128_ube`
- `rend::unaligned::NonZeroI128_ule`
- `rend::unaligned::NonZeroU16_ube`
- `rend::unaligned::NonZeroU16_ule`
- `rend::unaligned::NonZeroU32_ube`
- `rend::unaligned::NonZeroU32_ule`
- `rend::unaligned::NonZeroU64_ube`
- `rend::unaligned::NonZeroU64_ule`
- `rend::unaligned::NonZeroU128_ube`
- `rend::unaligned::NonZeroU128_ule`
- `rend::unaligned::char_ube`
- `rend::unaligned::char_ule`
- `rend::unaligned::f32_ube`
- `rend::unaligned::f32_ule`
- `rend::unaligned::f64_ube`
- `rend::unaligned::f64_ule`
- `rend::unaligned::i16_ube`
- `rend::unaligned::i16_ule`
- `rend::unaligned::i32_ube`
- `rend::unaligned::i32_ule`
- `rend::unaligned::i64_ube`
- `rend::unaligned::i64_ule`
- `rend::unaligned::i128_ube`
- `rend::unaligned::i128_ule`
- `rend::unaligned::u16_ube`
- `rend::unaligned::u16_ule`
- `rend::unaligned::u32_ube`
- `rend::unaligned::u32_ule`
- `rend::unaligned::u64_ube`
- `rend::unaligned::u64_ule`
- `rend::unaligned::u128_ube`
- `rend::unaligned::u128_ule`
- `str`
- `ArchivedOptionNonZeroI8`
- `ArchivedOptionNonZeroI16`
- `ArchivedOptionNonZeroI32`
- `ArchivedOptionNonZeroI64`
- `ArchivedOptionNonZeroI128`
- `ArchivedOptionNonZeroU8`
- `ArchivedOptionNonZeroU16`
- `ArchivedOptionNonZeroU32`
- `ArchivedOptionNonZeroU64`
- `ArchivedOptionNonZeroU128`
- `[T; N]` with <T: NoUndef, const N: usize>

#### Trait `LayoutRaw`

Returns the layout of a type from its metadata.

```rust
pub trait LayoutRaw
where
    Self: Pointee {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `layout_raw`: Returns the layout of the type.

##### Implementations

This trait is implemented for the following types:

- `core::ffi::CStr`
- `T` with <T>
- `[T]` with <T>
- `str`

#### Trait `Archive`

A type that can be used without deserializing.

`Archive` is one of three basic traits used to work with zero-copy data and
controls the layout of the data in its archived zero-copy representation.
The [`Serialize`] trait helps transform types into that representation, and
the [`Deserialize`] trait helps transform types back out.

Types that implement `Archive` must have a well-defined archived size.
Unsized types can be supported using the [`ArchiveUnsized`] trait, along
with [`SerializeUnsized`] and [`DeserializeUnsized`].

Archiving is done depth-first, writing any data owned by a type before
writing the data for the type itself. The type must be able to create the
archived type from only its own data and its resolver.

Archived data is always treated as if it is tree-shaped, with the root
owning its direct descendents and so on. Data that is not tree-shaped can be
supported using special serializer and deserializer bounds (see
[`ArchivedRc`](crate::rc::ArchivedRc) for example). In a buffer of
serialized data, objects are laid out in *reverse order*. This means that
the root object is located near the end of the buffer and leaf objects are
located near the beginning.

# Examples

Most of the time, `#[derive(Archive)]` will create an acceptable
implementation. You can use the `#[rkyv(...)]` attribute to control how the
implementation is generated. See the [`Archive`](macro@crate::Archive)
derive macro for more details.
```
use rkyv::{deserialize, rancor::Error, Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
struct Test {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
}

fn main() {
    let value = Test {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
    };

    // Serializing is as easy as a single function call
    let _bytes = rkyv::to_bytes::<Error>(&value).unwrap();

    // Or you can customize your serialization for better performance or control
    // over resource usage
    use rkyv::{api::high::to_bytes_with_alloc, ser::allocator::Arena};

    let mut arena = Arena::new();
    let bytes =
        to_bytes_with_alloc::<_, Error>(&value, arena.acquire()).unwrap();

    // You can use the safe API for fast zero-copy deserialization
    let archived = rkyv::access::<ArchivedTest, Error>(&bytes[..]).unwrap();
    assert_eq!(archived, &value);

    // Or you can use the unsafe API for maximum performance
    let archived =
        unsafe { rkyv::access_unchecked::<ArchivedTest>(&bytes[..]) };
    assert_eq!(archived, &value);

    // And you can always deserialize back to the original type
    let deserialized = deserialize::<Test, Error>(archived).unwrap();
    assert_eq!(deserialized, value);
}
```
_Note: the safe API requires the `bytecheck` feature._

Many of the core and standard library types already have `Archive`
implementations available, but you may need to implement `Archive` for your
own types in some cases the derive macro cannot handle.

In this example, we add our own wrapper that serializes a `&'static str` as
if it's owned. Normally you can lean on the archived version of `String` to
do most of the work, or use the [`Inline`](crate::with::Inline) to do
exactly this. This example does everything to demonstrate how to implement
`Archive` for your own types.
```
use core::{slice, str};

use rkyv::{
    access_unchecked,
    rancor::{Error, Fallible},
    ser::Writer,
    to_bytes,
    Archive, ArchiveUnsized, Archived, Portable, RelPtr, Serialize,
    SerializeUnsized, munge::munge, Place,
};

struct OwnedStr {
    inner: &'static str,
}

#[derive(Portable)]
#[repr(transparent)]
struct ArchivedOwnedStr {
    // This will be a relative pointer to our string
    ptr: RelPtr<str>,
}

impl ArchivedOwnedStr {
    // This will help us get the bytes of our type as a str again.
    fn as_str(&self) -> &str {
        unsafe {
            // The as_ptr() function of RelPtr will get a pointer the str
            &*self.ptr.as_ptr()
        }
    }
}

struct OwnedStrResolver {
    // This will be the position that the bytes of our string are stored at.
    // We'll use this to resolve the relative pointer of our
    // ArchivedOwnedStr.
    pos: usize,
}

// The Archive implementation defines the archived version of our type and
// determines how to turn the resolver into the archived form. The Serialize
// implementations determine how to make a resolver from the original value.
impl Archive for OwnedStr {
    type Archived = ArchivedOwnedStr;
    // This is the resolver we can create our Archived version from.
    type Resolver = OwnedStrResolver;

    // The resolve function consumes the resolver and produces the archived
    // value at the given position.
    fn resolve(
        &self,
        resolver: Self::Resolver,
        out: Place<Self::Archived>,
    ) {
        munge!(let ArchivedOwnedStr { ptr } = out);
        RelPtr::emplace_unsized(
            resolver.pos,
            self.inner.archived_metadata(),
            ptr,
        );
    }
}

// We restrict our serializer types with Writer because we need its
// capabilities to serialize the inner string. For other types, we might
// need more or less restrictive bounds on the type of S.
impl<S: Fallible + Writer + ?Sized> Serialize<S> for OwnedStr {
    fn serialize(
        &self,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        // This is where we want to write the bytes of our string and return
        // a resolver that knows where those bytes were written.
        // We also need to serialize the metadata for our str.
        Ok(OwnedStrResolver {
            pos: self.inner.serialize_unsized(serializer)?,
        })
    }
}

const STR_VAL: &'static str = "I'm in an OwnedStr!";
let value = OwnedStr { inner: STR_VAL };
// It works!
let buf = to_bytes::<Error>(&value).expect("failed to serialize");
let archived =
    unsafe { access_unchecked::<ArchivedOwnedStr>(buf.as_ref()) };
// Let's make sure our data got written correctly
assert_eq!(archived.as_str(), STR_VAL);
```

```rust
pub trait Archive {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Archived`: The archived representation of this type.
- `Resolver`: The resolver for this type. It must contain all the additional

###### Required Methods

- `resolve`: Creates the archived version of this value at the given position and

##### Provided Methods

##### Implementations

This trait is implemented for the following types:

- `EntryAdapter<BK, BV, K, V>` with <BK, BV, K, V>
- `crate::alloc::boxed::Box<T>` with <T: ArchiveUnsized + ?Sized>
- `crate::alloc::collections::BTreeMap<K, V>` with <K: Archive + Ord, V: Archive>
- `crate::alloc::collections::BTreeSet<K>` with <K: Archive + Ord>
- `crate::alloc::collections::VecDeque<T>` with <T: Archive>
- `crate::alloc::ffi::CString`
- `sync::Arc<T>` with <T: ArchiveUnsized + ?Sized>
- `sync::Weak<T>` with <T: ArchiveUnsized + ?Sized>
- `rc::Rc<T>` with <T: ArchiveUnsized + ?Sized>
- `rc::Weak<T>` with <T: ArchiveUnsized + ?Sized>
- `crate::alloc::string::String`
- `crate::alloc::vec::Vec<T>` with <T: Archive>
- `core::net::Ipv4Addr`
- `core::net::Ipv6Addr`
- `core::net::IpAddr`
- `core::net::SocketAddrV4`
- `core::net::SocketAddrV6`
- `core::net::SocketAddr`
- `core::ops::RangeFull`
- `core::ops::Range<T>` with <T: Archive>
- `core::ops::RangeInclusive<T>` with <T: Archive>
- `core::ops::RangeFrom<T>` with <T: Archive>
- `core::ops::RangeTo<T>` with <T: Archive>
- `core::ops::RangeToInclusive<T>` with <T: Archive>
- `core::ops::Bound<T>` with <T: Archive>
- `Option<T>` with <T: Archive>
- `()`
- `bool`
- `i8`
- `u8`
- `core::num::NonZeroI8`
- `core::num::NonZeroU8`
- `i16`
- `i32`
- `i64`
- `i128`
- `u16`
- `u32`
- `u64`
- `u128`
- `f32`
- `f64`
- `char`
- `core::num::NonZeroI16`
- `core::num::NonZeroI32`
- `core::num::NonZeroI64`
- `core::num::NonZeroI128`
- `core::num::NonZeroU16`
- `core::num::NonZeroU32`
- `core::num::NonZeroU64`
- `core::num::NonZeroU128`
- `usize`
- `isize`
- `core::num::NonZeroUsize`
- `core::num::NonZeroIsize`
- `Result<T, U>` with <T: Archive, U: Archive>
- `core::time::Duration`
- `(T0)` with <T0>
- `(T0, T1)` with <T0, T1>
- `(T0, T1, T2)` with <T0, T1, T2>
- `(T0, T1, T2, T3)` with <T0, T1, T2, T3>
- `(T0, T1, T2, T3, T4)` with <T0, T1, T2, T3, T4>
- `(T0, T1, T2, T3, T4, T5)` with <T0, T1, T2, T3, T4, T5>
- `(T0, T1, T2, T3, T4, T5, T6)` with <T0, T1, T2, T3, T4, T5, T6>
- `(T0, T1, T2, T3, T4, T5, T6, T7)` with <T0, T1, T2, T3, T4, T5, T6, T7>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
- `[T; N]` with <T: Archive, const N: usize>
- `core::marker::PhantomData<T>` with <T: ?Sized>
- `core::marker::PhantomPinned`
- `core::mem::ManuallyDrop<T>` with <T: Archive>
- `i16_le`
- `i32_le`
- `i64_le`
- `i128_le`
- `u16_le`
- `u32_le`
- `u64_le`
- `u128_le`
- `f32_le`
- `f64_le`
- `char_le`
- `NonZeroI16_le`
- `NonZeroI32_le`
- `NonZeroI64_le`
- `NonZeroI128_le`
- `NonZeroU16_le`
- `NonZeroU32_le`
- `NonZeroU64_le`
- `NonZeroU128_le`
- `i16_be`
- `i32_be`
- `i64_be`
- `i128_be`
- `u16_be`
- `u32_be`
- `u64_be`
- `u128_be`
- `f32_be`
- `f64_be`
- `char_be`
- `NonZeroI16_be`
- `NonZeroI32_be`
- `NonZeroI64_be`
- `NonZeroI128_be`
- `NonZeroU16_be`
- `NonZeroU32_be`
- `NonZeroU64_be`
- `NonZeroU128_be`
- `std::collections::HashMap<K, V, S>` with <K, V: Archive, S>
- `std::collections::HashSet<K, S>` with <K, S>
- `With<F, W>` with <F: ?Sized, W: ArchiveWith<F>>

#### Trait `Serialize`

Converts a type to its archived form.

Objects perform any supportive serialization during
[`serialize`](Serialize::serialize). For types that reference nonlocal
(pointed-to) data, this is when that data must be serialized to the output.
These types will need to bound `S` to implement
[`Writer`](crate::ser::Writer) and any other required traits (e.g.
[`Sharing`](crate::ser::Sharing)). They should then serialize their
dependencies during `serialize`.

See [`Archive`] for examples of implementing `Serialize`.

```rust
pub trait Serialize<S: Fallible + ?Sized>: Archive {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `serialize`: Writes the dependencies for the object and returns a resolver that can

##### Implementations

This trait is implemented for the following types:

- `EntryAdapter<BK, BV, K, V>` with <S, BK, BV, K, V>
- `crate::alloc::boxed::Box<T>` with <T, S>
- `crate::alloc::collections::BTreeMap<K, V>` with <K, V, S>
- `crate::alloc::collections::BTreeSet<K>` with <K, S>
- `crate::alloc::collections::VecDeque<T>` with <T, S>
- `crate::alloc::ffi::CString` with <S: Fallible + Writer + ?Sized>
- `sync::Arc<T>` with <T, S>
- `sync::Weak<T>` with <T, S>
- `rc::Rc<T>` with <T, S>
- `rc::Weak<T>` with <T, S>
- `crate::alloc::string::String` with <S: Fallible + ?Sized>
- `crate::alloc::vec::Vec<T>` with <T: Serialize<S>, S: Fallible + Allocator + Writer + ?Sized>
- `core::net::Ipv4Addr` with <S: Fallible + ?Sized>
- `core::net::Ipv6Addr` with <S: Fallible + ?Sized>
- `core::net::IpAddr` with <S: Fallible + ?Sized>
- `core::net::SocketAddrV4` with <S: Fallible + ?Sized>
- `core::net::SocketAddrV6` with <S: Fallible + ?Sized>
- `core::net::SocketAddr` with <S: Fallible + ?Sized>
- `core::ops::RangeFull` with <S: Fallible + ?Sized>
- `core::ops::Range<T>` with <T: Serialize<S>, S: Fallible + ?Sized>
- `core::ops::RangeInclusive<T>` with <T: Serialize<S>, S: Fallible + ?Sized>
- `core::ops::RangeFrom<T>` with <T: Serialize<S>, S: Fallible + ?Sized>
- `core::ops::RangeTo<T>` with <T: Serialize<S>, S: Fallible + ?Sized>
- `core::ops::RangeToInclusive<T>` with <T, S>
- `core::ops::Bound<T>` with <T: Serialize<S>, S: Fallible + ?Sized>
- `Option<T>` with <T: Serialize<S>, S: Fallible + ?Sized>
- `()` with <S: Fallible + ?Sized>
- `bool` with <S: Fallible + ?Sized>
- `i8` with <S: Fallible + ?Sized>
- `u8` with <S: Fallible + ?Sized>
- `core::num::NonZeroI8` with <S: Fallible + ?Sized>
- `core::num::NonZeroU8` with <S: Fallible + ?Sized>
- `i16` with <S: Fallible + ?Sized>
- `i32` with <S: Fallible + ?Sized>
- `i64` with <S: Fallible + ?Sized>
- `i128` with <S: Fallible + ?Sized>
- `u16` with <S: Fallible + ?Sized>
- `u32` with <S: Fallible + ?Sized>
- `u64` with <S: Fallible + ?Sized>
- `u128` with <S: Fallible + ?Sized>
- `f32` with <S: Fallible + ?Sized>
- `f64` with <S: Fallible + ?Sized>
- `char` with <S: Fallible + ?Sized>
- `core::num::NonZeroI16` with <S: Fallible + ?Sized>
- `core::num::NonZeroI32` with <S: Fallible + ?Sized>
- `core::num::NonZeroI64` with <S: Fallible + ?Sized>
- `core::num::NonZeroI128` with <S: Fallible + ?Sized>
- `core::num::NonZeroU16` with <S: Fallible + ?Sized>
- `core::num::NonZeroU32` with <S: Fallible + ?Sized>
- `core::num::NonZeroU64` with <S: Fallible + ?Sized>
- `core::num::NonZeroU128` with <S: Fallible + ?Sized>
- `usize` with <S: Fallible + ?Sized>
- `isize` with <S: Fallible + ?Sized>
- `core::num::NonZeroUsize` with <S: Fallible + ?Sized>
- `core::num::NonZeroIsize` with <S: Fallible + ?Sized>
- `Result<T, U>` with <T, U, S>
- `core::time::Duration` with <S: Fallible + ?Sized>
- `(T0)` with <T0, S>
- `(T0, T1)` with <T0, T1, S>
- `(T0, T1, T2)` with <T0, T1, T2, S>
- `(T0, T1, T2, T3)` with <T0, T1, T2, T3, S>
- `(T0, T1, T2, T3, T4)` with <T0, T1, T2, T3, T4, S>
- `(T0, T1, T2, T3, T4, T5)` with <T0, T1, T2, T3, T4, T5, S>
- `(T0, T1, T2, T3, T4, T5, T6)` with <T0, T1, T2, T3, T4, T5, T6, S>
- `(T0, T1, T2, T3, T4, T5, T6, T7)` with <T0, T1, T2, T3, T4, T5, T6, T7, S>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, S>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, S>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, S>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, S>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, S>
- `[T; N]` with <T, S, const N: usize>
- `core::marker::PhantomData<T>` with <T: ?Sized, S: Fallible + ?Sized>
- `core::marker::PhantomPinned` with <S: Fallible + ?Sized>
- `core::mem::ManuallyDrop<T>` with <T: Serialize<S>, S: Fallible + ?Sized>
- `i16_le` with <S: Fallible + ?Sized>
- `i32_le` with <S: Fallible + ?Sized>
- `i64_le` with <S: Fallible + ?Sized>
- `i128_le` with <S: Fallible + ?Sized>
- `u16_le` with <S: Fallible + ?Sized>
- `u32_le` with <S: Fallible + ?Sized>
- `u64_le` with <S: Fallible + ?Sized>
- `u128_le` with <S: Fallible + ?Sized>
- `f32_le` with <S: Fallible + ?Sized>
- `f64_le` with <S: Fallible + ?Sized>
- `char_le` with <S: Fallible + ?Sized>
- `NonZeroI16_le` with <S: Fallible + ?Sized>
- `NonZeroI32_le` with <S: Fallible + ?Sized>
- `NonZeroI64_le` with <S: Fallible + ?Sized>
- `NonZeroI128_le` with <S: Fallible + ?Sized>
- `NonZeroU16_le` with <S: Fallible + ?Sized>
- `NonZeroU32_le` with <S: Fallible + ?Sized>
- `NonZeroU64_le` with <S: Fallible + ?Sized>
- `NonZeroU128_le` with <S: Fallible + ?Sized>
- `i16_be` with <S: Fallible + ?Sized>
- `i32_be` with <S: Fallible + ?Sized>
- `i64_be` with <S: Fallible + ?Sized>
- `i128_be` with <S: Fallible + ?Sized>
- `u16_be` with <S: Fallible + ?Sized>
- `u32_be` with <S: Fallible + ?Sized>
- `u64_be` with <S: Fallible + ?Sized>
- `u128_be` with <S: Fallible + ?Sized>
- `f32_be` with <S: Fallible + ?Sized>
- `f64_be` with <S: Fallible + ?Sized>
- `char_be` with <S: Fallible + ?Sized>
- `NonZeroI16_be` with <S: Fallible + ?Sized>
- `NonZeroI32_be` with <S: Fallible + ?Sized>
- `NonZeroI64_be` with <S: Fallible + ?Sized>
- `NonZeroI128_be` with <S: Fallible + ?Sized>
- `NonZeroU16_be` with <S: Fallible + ?Sized>
- `NonZeroU32_be` with <S: Fallible + ?Sized>
- `NonZeroU64_be` with <S: Fallible + ?Sized>
- `NonZeroU128_be` with <S: Fallible + ?Sized>
- `std::collections::HashMap<K, V, RandomState>` with <K, V, S, RandomState>
- `std::collections::HashSet<K, RS>` with <K, S, RS>
- `With<F, W>` with <S, F, W>

#### Trait `Deserialize`

Converts a type back from its archived form.

Some types may require specific deserializer capabilities, such as `Rc` and
`Arc`. In these cases, the deserializer type `D` should be bound so that it
implements traits that provide those capabilities (e.g.
[`Pooling`](crate::de::Pooling)).

This can be derived with [`Deserialize`](macro@crate::Deserialize).

```rust
pub trait Deserialize<T, D: Fallible + ?Sized> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `deserialize`: Deserializes using the given deserializer

##### Implementations

This trait is implemented for the following types:

- `crate::boxed::ArchivedBox<<T as >::Archived>` with <T, D>
- `crate::collections::btree_map::ArchivedBTreeMap<<K as >::Archived, <V as >::Archived>` with <K, V, D>
- `crate::collections::btree_set::ArchivedBTreeSet<<K as >::Archived>` with <K, D>
- `crate::vec::ArchivedVec<<T as >::Archived>` with <T, D>
- `crate::ffi::ArchivedCString` with <D>
- `crate::rc::ArchivedRc<<T as >::Archived, crate::rc::ArcFlavor>` with <T, D>
- `crate::rc::ArchivedRcWeak<<T as >::Archived, crate::rc::ArcFlavor>` with <T, D>
- `crate::rc::ArchivedRc<<T as >::Archived, crate::rc::RcFlavor>` with <T, D>
- `crate::rc::ArchivedRcWeak<<T as >::Archived, crate::rc::RcFlavor>` with <T, D>
- `crate::string::ArchivedString` with <D: Fallible + ?Sized>
- `crate::vec::ArchivedVec<<T as >::Archived>` with <T, D>
- `crate::net::ArchivedIpv4Addr` with <D: Fallible + ?Sized>
- `crate::net::ArchivedIpv6Addr` with <D: Fallible + ?Sized>
- `crate::net::ArchivedIpAddr` with <D: Fallible + ?Sized>
- `crate::net::ArchivedSocketAddrV4` with <D>
- `crate::net::ArchivedSocketAddrV6` with <D: Fallible + ?Sized>
- `crate::net::ArchivedSocketAddr` with <D: Fallible + ?Sized>
- `crate::ops::ArchivedRangeFull` with <D: Fallible + ?Sized>
- `crate::ops::ArchivedRange<<T as >::Archived>` with <T, D>
- `crate::ops::ArchivedRangeInclusive<<T as >::Archived>` with <T, D>
- `crate::ops::ArchivedRangeFrom<<T as >::Archived>` with <T, D>
- `crate::ops::ArchivedRangeTo<<T as >::Archived>` with <T, D>
- `crate::ops::ArchivedRangeToInclusive<<T as >::Archived>` with <T, D>
- `crate::ops::ArchivedBound<<T as >::Archived>` with <T, D>
- `crate::option::ArchivedOption<<T as >::Archived>` with <T, D>
- `()` with <D: Fallible + ?Sized>
- `bool` with <D: Fallible + ?Sized>
- `i8` with <D: Fallible + ?Sized>
- `u8` with <D: Fallible + ?Sized>
- `core::num::NonZeroI8` with <D: Fallible + ?Sized>
- `core::num::NonZeroU8` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedI16` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedI32` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedI64` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedI128` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedU16` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedU32` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedU64` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedU128` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedF32` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedF64` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedChar` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroI16` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroI32` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroI64` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroI128` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroU16` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroU32` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroU64` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroU128` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedUsize` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedIsize` with <D: Fallible + ?Sized>
- `crate::primitive::ArchivedNonZeroUsize` with <D>
- `crate::primitive::ArchivedNonZeroIsize` with <D>
- `crate::result::ArchivedResult<<T as >::Archived, <U as >::Archived>` with <T, U, D>
- `crate::time::ArchivedDuration` with <D: Fallible + ?Sized>
- `crate::niche::niched_option::NichedOption<<T as >::Archived, N>` with <T, N, D>
- `ArchivedTuple1<<T0 as >::Archived>` with <T0, D>
- `ArchivedTuple2<<T0 as >::Archived, <T1 as >::Archived>` with <T0, T1, D>
- `ArchivedTuple3<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived>` with <T0, T1, T2, D>
- `ArchivedTuple4<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived>` with <T0, T1, T2, T3, D>
- `ArchivedTuple5<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived>` with <T0, T1, T2, T3, T4, D>
- `ArchivedTuple6<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived, <T5 as >::Archived>` with <T0, T1, T2, T3, T4, T5, D>
- `ArchivedTuple7<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived, <T5 as >::Archived, <T6 as >::Archived>` with <T0, T1, T2, T3, T4, T5, T6, D>
- `ArchivedTuple8<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived, <T5 as >::Archived, <T6 as >::Archived, <T7 as >::Archived>` with <T0, T1, T2, T3, T4, T5, T6, T7, D>
- `ArchivedTuple9<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived, <T5 as >::Archived, <T6 as >::Archived, <T7 as >::Archived, <T8 as >::Archived>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, D>
- `ArchivedTuple10<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived, <T5 as >::Archived, <T6 as >::Archived, <T7 as >::Archived, <T8 as >::Archived, <T9 as >::Archived>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, D>
- `ArchivedTuple11<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived, <T5 as >::Archived, <T6 as >::Archived, <T7 as >::Archived, <T8 as >::Archived, <T9 as >::Archived, <T10 as >::Archived>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, D>
- `ArchivedTuple12<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived, <T5 as >::Archived, <T6 as >::Archived, <T7 as >::Archived, <T8 as >::Archived, <T9 as >::Archived, <T10 as >::Archived, <T11 as >::Archived>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, D>
- `ArchivedTuple13<<T0 as >::Archived, <T1 as >::Archived, <T2 as >::Archived, <T3 as >::Archived, <T4 as >::Archived, <T5 as >::Archived, <T6 as >::Archived, <T7 as >::Archived, <T8 as >::Archived, <T9 as >::Archived, <T10 as >::Archived, <T11 as >::Archived, <T12 as >::Archived>` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, D>
- `[<T as >::Archived; N]` with <T, D, const N: usize>
- `core::marker::PhantomData<T>` with <T: ?Sized, D: Fallible + ?Sized>
- `core::marker::PhantomPinned` with <D: Fallible + ?Sized>
- `core::mem::ManuallyDrop<<T as >::Archived>` with <T, D>
- `i16_le` with <D: Fallible + ?Sized>
- `i32_le` with <D: Fallible + ?Sized>
- `i64_le` with <D: Fallible + ?Sized>
- `i128_le` with <D: Fallible + ?Sized>
- `u16_le` with <D: Fallible + ?Sized>
- `u32_le` with <D: Fallible + ?Sized>
- `u64_le` with <D: Fallible + ?Sized>
- `u128_le` with <D: Fallible + ?Sized>
- `f32_le` with <D: Fallible + ?Sized>
- `f64_le` with <D: Fallible + ?Sized>
- `char_le` with <D: Fallible + ?Sized>
- `NonZeroI16_le` with <D: Fallible + ?Sized>
- `NonZeroI32_le` with <D: Fallible + ?Sized>
- `NonZeroI64_le` with <D: Fallible + ?Sized>
- `NonZeroI128_le` with <D: Fallible + ?Sized>
- `NonZeroU16_le` with <D: Fallible + ?Sized>
- `NonZeroU32_le` with <D: Fallible + ?Sized>
- `NonZeroU64_le` with <D: Fallible + ?Sized>
- `NonZeroU128_le` with <D: Fallible + ?Sized>
- `i16_be` with <D: Fallible + ?Sized>
- `i32_be` with <D: Fallible + ?Sized>
- `i64_be` with <D: Fallible + ?Sized>
- `i128_be` with <D: Fallible + ?Sized>
- `u16_be` with <D: Fallible + ?Sized>
- `u32_be` with <D: Fallible + ?Sized>
- `u64_be` with <D: Fallible + ?Sized>
- `u128_be` with <D: Fallible + ?Sized>
- `f32_be` with <D: Fallible + ?Sized>
- `f64_be` with <D: Fallible + ?Sized>
- `char_be` with <D: Fallible + ?Sized>
- `NonZeroI16_be` with <D: Fallible + ?Sized>
- `NonZeroI32_be` with <D: Fallible + ?Sized>
- `NonZeroI64_be` with <D: Fallible + ?Sized>
- `NonZeroI128_be` with <D: Fallible + ?Sized>
- `NonZeroU16_be` with <D: Fallible + ?Sized>
- `NonZeroU32_be` with <D: Fallible + ?Sized>
- `NonZeroU64_be` with <D: Fallible + ?Sized>
- `NonZeroU128_be` with <D: Fallible + ?Sized>
- `crate::collections::swiss_table::map::ArchivedHashMap<<K as >::Archived, <V as >::Archived>` with <K, V, D, S>
- `crate::collections::swiss_table::set::ArchivedHashSet<<K as >::Archived>` with <K, D, S>
- `With<F, W>` with <T, D, F, W>

#### Trait `ArchiveUnsized`

A counterpart of [`Archive`] that's suitable for unsized types.

Unlike `Archive`, types that implement `ArchiveUnsized` must be serialized
separately from their owning object. For example, whereas an `i32` might be
laid out as part of a larger struct, a `Box<i32>` would serialize the `i32`
somewhere in the archive and the `Box` would point to it as part of the
larger struct. Because of this, the equivalent
[`Resolver`](Archive::Resolver) type for `ArchiveUnsized` is always a
`usize` representing the position of the serialized value.

`ArchiveUnsized` is automatically implemented for all types that implement
[`Archive`]. Nothing special needs to be done to use them with types like
`Box`, `Rc`, and `Arc`. It is also already implemented for slices and string
slices, and the `rkyv_dyn` crate can be used to archive trait objects. Other
unsized types must manually implement `ArchiveUnsized`.

# Examples

This example shows how to manually implement `ArchiveUnsized` for an unsized
type. Special care must be taken to ensure that the types are laid out
correctly.

```
use core::ops::{Deref, DerefMut};

use ptr_meta::Pointee;
use rkyv::{
    access_unchecked,
    primitive::ArchivedUsize,
    rancor::{Error, Fallible},
    ser::{Positional, Writer, WriterExt as _},
    to_bytes,
    traits::ArchivePointee,
    Archive, ArchiveUnsized, Archived, ArchivedMetadata, Portable, RelPtr,
    Serialize, SerializeUnsized,
};

// We're going to be dealing mostly with blocks that have a trailing slice
#[derive(Portable)]
#[repr(C)]
pub struct Block<H, T: ?Sized> {
    head: H,
    tail: T,
}

unsafe impl<H, T> Pointee for Block<H, [T]> {
    type Metadata = <[T] as Pointee>::Metadata;
}

// ArchivePointee is automatically derived for sized types because pointers
// to sized types don't need to store any extra information. Because we're
// making an unsized block, we need to define what metadata gets stored with
// our data pointer.
impl<H, T> ArchivePointee for Block<H, [T]> {
    // This is the extra data that needs to get stored for blocks with
    // trailing slices
    type ArchivedMetadata = <[T] as ArchivePointee>::ArchivedMetadata;

    // We need to be able to turn our archived metadata into regular
    // metadata for our type
    fn pointer_metadata(
        metadata: &Self::ArchivedMetadata,
    ) -> <Self as Pointee>::Metadata {
        metadata.to_native() as usize
    }
}

// We're implementing ArchiveUnsized for just Block<H, [T]>. We can still
// implement Archive for blocks with sized tails and they won't conflict.
impl<H: Archive, T: Archive> ArchiveUnsized for Block<H, [T]> {
    // We'll reuse our block type as our archived type.
    type Archived = Block<Archived<H>, [Archived<T>]>;

    // Here's where we make the metadata for our archived type.
    fn archived_metadata(&self) -> ArchivedMetadata<Self> {
        // Because the metadata for our `ArchivedBlock` is the metadata of
        // the trailing slice, we just need to return that archived
        // metadata.
        self.tail.archived_metadata()
    }
}

// The bounds we use on our serializer type indicate that we need basic
// serializer capabilities, and then whatever capabilities our head and tail
// types need to serialize themselves.
impl<H, T, S> SerializeUnsized<S> for Block<H, [T]>
where
    H: Serialize<S>,
    T: Serialize<S>,
    S: Fallible + Writer + ?Sized,
{
    // This is where we construct our unsized type in the serializer
    fn serialize_unsized(
        &self,
        serializer: &mut S,
    ) -> Result<usize, S::Error> {
        // First, we serialize the head and all the tails. This will make
        // sure that when we finally build our block, we don't accidentally
        // mess up the structure with serialized dependencies.
        let head_resolver = self.head.serialize(serializer)?;
        let mut resolvers = Vec::new();
        for tail in self.tail.iter() {
            resolvers.push(tail.serialize(serializer)?);
        }
        // Now we align our serializer for our archived type and resolve it.
        // We can't align for unsized types so we treat the trailing slice
        // like an array of 0 length for now.
        let result = serializer
            .align_for::<Block<Archived<H>, [Archived<T>; 0]>>()?;
        unsafe {
            serializer.resolve_aligned(&self.head, head_resolver)?;
        }
        serializer.align_for::<Archived<T>>()?;
        for (item, resolver) in self.tail.iter().zip(resolvers.drain(..)) {
            unsafe {
                serializer.resolve_aligned(item, resolver)?;
            }
        }
        Ok(result)
    }
}

let value = Box::new(Block {
    head: "Numbers 1-4".to_string(),
    tail: [1, 2, 3, 4],
});

// We have a Box<Block<String, [i32; 4]>> but we want to it to be a
// Box<Block<String, [i32]>>, so we need manually "unsize" the pointer.
let ptr = Box::into_raw(value);
let unsized_ptr = ptr_meta::from_raw_parts_mut::<Block<String, [i32]>>(
    ptr.cast::<()>(),
    4,
);
let unsized_value = unsafe { Box::from_raw(unsized_ptr) };

let bytes = to_bytes::<Error>(&unsized_value).unwrap();

let archived = unsafe {
    access_unchecked::<Archived<Box<Block<String, [i32]>>>>(&bytes)
};
assert_eq!(archived.head, "Numbers 1-4");
assert_eq!(archived.tail.len(), 4);
assert_eq!(archived.tail, [1, 2, 3, 4]);
```

```rust
pub trait ArchiveUnsized: Pointee {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Archived`: The archived counterpart of this type. Unlike `Archive`, it may be

###### Required Methods

- `archived_metadata`: Creates the archived version of the metadata for this value.

##### Implementations

This trait is implemented for the following types:

- `core::ffi::CStr`
- `T` with <T: Archive>
- `[T]` with <T: Archive>
- `str`

#### Trait `ArchivePointee`

An archived type with associated metadata for its relative pointer.

This is mostly used in the context of smart pointers and unsized types, and
is implemented for all sized types by default.

```rust
pub trait ArchivePointee: Pointee {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `ArchivedMetadata`: The archived version of the pointer metadata for this type.

###### Required Methods

- `pointer_metadata`: Converts some archived metadata to the pointer metadata for itself.

##### Implementations

This trait is implemented for the following types:

- `core::ffi::CStr`
- `T` with <T>
- `[T]` with <T>
- `str`

#### Trait `SerializeUnsized`

A counterpart of [`Serialize`] that's suitable for unsized types.

See [`ArchiveUnsized`] for examples of implementing `SerializeUnsized`.

```rust
pub trait SerializeUnsized<S: Fallible + ?Sized>: ArchiveUnsized {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `serialize_unsized`: Writes the object and returns the position of the archived type.

##### Implementations

This trait is implemented for the following types:

- `core::ffi::CStr` with <S: Fallible + Writer + ?Sized>
- `T` with <T, S>
- `[T]` with <T, S>
- `str` with <S: Fallible + Writer + ?Sized>

#### Trait `DeserializeUnsized`

A counterpart of [`Deserialize`] that's suitable for unsized types.

```rust
pub trait DeserializeUnsized<T: Pointee + ?Sized, D: Fallible + ?Sized>: ArchivePointee {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `deserialize_unsized`: Deserializes a reference to the given value.
- `deserialize_metadata`: Deserializes the metadata for the given type.

##### Implementations

This trait is implemented for the following types:

- `core::ffi::CStr` with <D: Fallible + ?Sized>
- `<T as >::Archived` with <T, D>
- `[T]` with <T, U, D>
- `str` with <D: Fallible + ?Sized>

### Re-exports

#### Re-export `Archive`

```rust
pub use ::rkyv_derive::Archive;
```

#### Re-export `Deserialize`

```rust
pub use ::rkyv_derive::Deserialize;
```

#### Re-export `Portable`

```rust
pub use ::rkyv_derive::Portable;
```

#### Re-export `Serialize`

```rust
pub use ::rkyv_derive::Serialize;
```

## Module `tuple`

Archived versions of tuple types.

```rust
pub mod tuple { /* ... */ }
```

### Types

#### Struct `ArchivedTuple1`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 1 elements

```rust
pub struct ArchivedTuple1<T0>(pub T0);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple1<T0>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple1<T0>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> ArchivedTuple1<T0> { /* ... */ }
    ```

- **Sync**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Send**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Portable**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Eq**
- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0), <D as >::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple1<T0>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0)) -> bool { /* ... */ }
    ```

#### Struct `ArchivedTuple2`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 2 elements

```rust
pub struct ArchivedTuple2<T0, T1>(pub T0, pub T1);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple2<T0, T1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1)) -> bool { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple2<T0, T1>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedTuple2<T0, T1> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Portable**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1), <D as >::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple2<T0, T1>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Pointee**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `ArchivedTuple3`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 3 elements

```rust
pub struct ArchivedTuple3<T0, T1, T2>(pub T0, pub T1, pub T2);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple3<T0, T1, T2>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2), <D as >::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple3<T0, T1, T2>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2)) -> bool { /* ... */ }
    ```

- **Eq**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **Default**
  - ```rust
    fn default() -> ArchivedTuple3<T0, T1, T2> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple3<T0, T1, T2>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Portable**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

#### Struct `ArchivedTuple4`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 4 elements

```rust
pub struct ArchivedTuple4<T0, T1, T2, T3>(pub T0, pub T1, pub T2, pub T3);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |

##### Implementations

###### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3), <D as >::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple4<T0, T1, T2, T3>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple4<T0, T1, T2, T3>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3)) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Eq**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple4<T0, T1, T2, T3>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **Portable**
- **Default**
  - ```rust
    fn default() -> ArchivedTuple4<T0, T1, T2, T3> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `ArchivedTuple5`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 5 elements

```rust
pub struct ArchivedTuple5<T0, T1, T2, T3, T4>(pub T0, pub T1, pub T2, pub T3, pub T4);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple5<T0, T1, T2, T3, T4>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4)) -> bool { /* ... */ }
    ```

- **Portable**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedTuple5<T0, T1, T2, T3, T4> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple5<T0, T1, T2, T3, T4>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Pointee**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4), <D as >::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple5<T0, T1, T2, T3, T4>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **Eq**
- **Sync**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

#### Struct `ArchivedTuple6`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 6 elements

```rust
pub struct ArchivedTuple6<T0, T1, T2, T3, T4, T5>(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |
| 5 | `T5` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple6<T0, T1, T2, T3, T4, T5>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5)) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple6<T0, T1, T2, T3, T4, T5>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4, T5), <D as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple6<T0, T1, T2, T3, T4, T5>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Eq**
- **Send**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> ArchivedTuple6<T0, T1, T2, T3, T4, T5> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Portable**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `ArchivedTuple7`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 7 elements

```rust
pub struct ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |
| 5 | `T5` |  |
| 6 | `T6` |  |

##### Implementations

###### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Unpin**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6), <D as >::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6> { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Portable**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6)) -> bool { /* ... */ }
    ```

- **Send**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Pointee**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `ArchivedTuple8`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 8 elements

```rust
pub struct ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |
| 5 | `T5` |  |
| 6 | `T6` |  |
| 7 | `T7` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7> { /* ... */ }
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

- **StructuralPartialEq**
- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7), <D as >::Error> { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7)) -> bool { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Portable**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Sync**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `ArchivedTuple9`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 9 elements

```rust
pub struct ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |
| 5 | `T5` |  |
| 6 | `T6` |  |
| 7 | `T7` |  |
| 8 | `T8` |  |

##### Implementations

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Pointee**
- **Sync**
- **Eq**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8)) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8> { /* ... */ }
    ```

- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Portable**
- **RefUnwindSafe**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), <D as >::Error> { /* ... */ }
    ```

#### Struct `ArchivedTuple10`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 10 elements

```rust
pub struct ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |
| 5 | `T5` |  |
| 6 | `T6` |  |
| 7 | `T7` |  |
| 8 | `T8` |  |
| 9 | `T9` |  |

##### Implementations

###### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)) -> bool { /* ... */ }
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

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Portable**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), <D as >::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `ArchivedTuple11`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 11 elements

```rust
pub struct ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |
| 5 | `T5` |  |
| 6 | `T6` |  |
| 7 | `T7` |  |
| 8 | `T8` |  |
| 9 | `T9` |  |
| 10 | `T10` |  |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), <D as >::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)) -> bool { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Portable**
- **Default**
  - ```rust
    fn default() -> ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
#### Struct `ArchivedTuple12`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 12 elements

```rust
pub struct ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |
| 5 | `T5` |  |
| 6 | `T6` |  |
| 7 | `T7` |  |
| 8 | `T8` |  |
| 9 | `T9` |  |
| 10 | `T10` |  |
| 11 | `T11` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Pointee**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), <D as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)) -> bool { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Portable**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

#### Struct `ArchivedTuple13`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck :: CheckBytes))]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived tuple with 13 elements

```rust
pub struct ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T0` |  |
| 1 | `T1` |  |
| 2 | `T2` |  |
| 3 | `T3` |  |
| 4 | `T4` |  |
| 5 | `T5` |  |
| 6 | `T6` |  |
| 7 | `T7` |  |
| 8 | `T8` |  |
| 9 | `T9` |  |
| 10 | `T10` |  |
| 11 | `T11` |  |
| 12 | `T12` |  |

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), <D as >::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Portable**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12)) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `util`

Utilities for common operations.

```rust
pub mod util { /* ... */ }
```

### Types

#### Struct `Align`

**Attributes:**

- `#[repr(C, align(16))]`

A wrapper which aligns its inner value to 16 bytes.

```rust
pub struct Align<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` | The inner value. |

##### Implementations

###### Trait Implementations

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Align<T> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
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

- **Receiver**
- **Copy**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

### Re-exports

#### Re-export `InlineVec`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::inline_vec::InlineVec;
```

#### Re-export `SerVec`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::ser_vec::SerVec;
```

#### Re-export `self::alloc::*`

**Attributes:**

- `#[doc(inline)]`
- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::alloc::*;
```

## Module `validation`

**Attributes:**

- `#[<cfg>(feature = "bytecheck")]`

Validation implementations and helper types.

```rust
pub mod validation { /* ... */ }
```

### Modules

## Module `archive`

Basic archive buffer validation.

```rust
pub mod archive { /* ... */ }
```

### Traits

#### Trait `ArchiveContext`

A context that can validate nonlocal archive memory.

# Safety

`check_subtree_ptr` must only return true if `ptr` is located entirely
within the subtree range and is safe to dereference.

```rust
pub unsafe trait ArchiveContext<E = <Self as Fallible>::Error> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

##### Required Items

###### Required Methods

- `check_subtree_ptr`: Checks that the given data address and layout is located completely
- `push_subtree_range`: Pushes a new subtree range onto the validator and starts validating it.
- `pop_subtree_range`: Pops the given range, restoring the original state with the pushed range

##### Implementations

This trait is implemented for the following types:

- `ArchiveValidator<''_>` with <E: Source>
- `bytecheck::rancor::Strategy<T, E>` with <T, E>
- `Validator<A, S>` with <A, S, E>

#### Trait `ArchiveContextExt`

Helper methods for [`ArchiveContext`].

```rust
pub trait ArchiveContextExt<E>: ArchiveContext<E> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `in_subtree_raw`: Checks that the given pointer and layout are within the current subtree
- `in_subtree`: Checks that the value the given pointer points to is within the current

##### Implementations

This trait is implemented for the following types:

- `C` with <C: ArchiveContext<E> + ?Sized, E: Source>

### Re-exports

#### Re-export `self::validator::*`

```rust
pub use self::validator::*;
```

## Module `shared`

Shared pointer validation.

```rust
pub mod shared { /* ... */ }
```

### Types

#### Enum `ValidationState`

The result of starting to validate a shared pointer.

```rust
pub enum ValidationState {
    Started,
    Pending,
    Finished,
}
```

##### Variants

###### `Started`

The caller started validating this value. They should proceed to check
the shared value and call `finish_shared`.

###### `Pending`

Another caller started validating this value, but has not finished yet.
This can only occur with cyclic shared pointer structures, and so rkyv
treats this as an error by default.

###### `Finished`

This value has already been validated.

##### Implementations

###### Trait Implementations

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
### Traits

#### Trait `SharedContext`

A context that can validate shared archive memory.

Shared pointers require this kind of context to validate.

```rust
pub trait SharedContext<E = <Self as Fallible>::Error> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `start_shared`: Starts validating the value associated with the given address.
- `finish_shared`: Finishes validating the value associated with the given address.

##### Implementations

This trait is implemented for the following types:

- `SharedValidator` with <E: Source>
- `rancor::Strategy<T, E>` with <T, E>
- `Validator<A, S>` with <A, S, E>

### Re-exports

#### Re-export `self::validator::*`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::validator::*;
```

### Types

#### Struct `Validator`

The default validator.

```rust
pub struct Validator<A, S> {
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
  pub fn new(archive: A, shared: S) -> Self { /* ... */ }
  ```
  Creates a new validator from a byte range.

###### Trait Implementations

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Pointee**
- **ArchiveContextExt**
  - ```rust
    fn in_subtree_raw<R>(self: &mut Self, ptr: *const u8, layout: Layout, f: impl FnOnce(&mut C) -> Result<R, E>) -> Result<R, E> { /* ... */ }
    ```

  - ```rust
    fn in_subtree<T, R>(self: &mut Self, ptr: *const T, f: impl FnOnce(&mut C) -> Result<R, E>) -> Result<R, E>
where
    T: LayoutRaw + ?Sized { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Send**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchiveContext**
  - ```rust
    fn check_subtree_ptr(self: &mut Self, ptr: *const u8, layout: &core::alloc::Layout) -> Result<(), E> { /* ... */ }
    ```

  - ```rust
    unsafe fn push_subtree_range(self: &mut Self, root: *const u8, end: *const u8) -> Result<Range<usize>, E> { /* ... */ }
    ```

  - ```rust
    unsafe fn pop_subtree_range(self: &mut Self, range: Range<usize>) -> Result<(), E> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **SharedContext**
  - ```rust
    fn start_shared(self: &mut Self, address: usize, type_id: TypeId) -> Result<shared::ValidationState, E> { /* ... */ }
    ```

  - ```rust
    fn finish_shared(self: &mut Self, address: usize, type_id: TypeId) -> Result<(), E> { /* ... */ }
    ```

- **UnwindSafe**
### Re-exports

#### Re-export `ArchiveContext`

```rust
pub use self::archive::ArchiveContext;
```

#### Re-export `ArchiveContextExt`

```rust
pub use self::archive::ArchiveContextExt;
```

#### Re-export `SharedContext`

```rust
pub use self::shared::SharedContext;
```

## Module `vec`

An archived version of `Vec`.

```rust
pub mod vec { /* ... */ }
```

### Types

#### Struct `ArchivedVec`

**Attributes:**

- `#[<cfg_attr>(feature = "bytecheck", derive(bytecheck::CheckBytes),
bytecheck(verify))]`
- `#[bytecheck(verify)]`
- `#[rkyv(crate)]`
- `#[repr(C)]`

An archived [`Vec`].

This uses a [`RelPtr`] to a `[T]` under the hood. Unlike
[`ArchivedString`](crate::string::ArchivedString), it does not have an
inline representation.

```rust
pub struct ArchivedVec<T> {
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
  pub fn as_ptr(self: &Self) -> *const T { /* ... */ }
  ```
  Returns a pointer to the first element of the archived vec.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the archived vec.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the archived vec is empty.

- ```rust
  pub fn as_slice(self: &Self) -> &[T] { /* ... */ }
  ```
  Gets the elements of the archived vec as a slice.

- ```rust
  pub fn as_slice_seal(this: Seal<''_, Self>) -> Seal<''_, [T]> { /* ... */ }
  ```
  Gets the elements of the archived vec as a sealed mutable slice.

- ```rust
  pub fn resolve_from_slice<U: Archive<Archived = T>>(slice: &[U], resolver: VecResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived `Vec` from a given slice.

- ```rust
  pub fn resolve_from_len(len: usize, resolver: VecResolver, out: Place<Self>) { /* ... */ }
  ```
  Resolves an archived `Vec` from a given length.

- ```rust
  pub fn serialize_from_slice<U: Serialize<S, Archived = T>, S: Fallible + Allocator + Writer + ?Sized>(slice: &[U], serializer: &mut S) -> Result<VecResolver, <S as >::Error> { /* ... */ }
  ```
  Serializes an archived `Vec` from a given slice.

- ```rust
  pub fn serialize_from_iter<U, I, S>(iter: I, serializer: &mut S) -> Result<VecResolver, <S as >::Error>
where
    U: Serialize<S, Archived = T>,
    I: ExactSizeIterator + Clone,
    <I as >::Item: Borrow<U>,
    S: Fallible + Allocator + Writer + ?Sized { /* ... */ }
  ```
  Serializes an archived `Vec` from a given iterator.

- ```rust
  pub fn serialize_from_unknown_length_iter<B, I, S>(iter: &mut I, serializer: &mut S) -> Result<VecResolver, <S as >::Error>
where
    B: Serialize<S, Archived = T>,
    I: Iterator<Item = B>,
    S: Fallible + Allocator + Writer + ?Sized { /* ... */ }
  ```
  Serializes an archived `Vec` from a given iterator. Compared to

###### Trait Implementations

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &VecDeque<T>) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Vec<U>) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedVec<T>) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &[T]) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &ArchivedVec<T>) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &[T] { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Pointee**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Portable**
- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VecDeque<U>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Vec<U>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedVec<U>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[U; N]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedVec<T>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[U]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &ArchivedVec<U>) -> bool { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedVec<<A as ArchiveWith<O>>::Archived>, d: &mut D) -> Result<Vec<O>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: &mut D) -> Result<Cow<''a, [T]>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<Entry<<K as >::Archived, <V as >::Archived>>, deserializer: &mut D) -> Result<BTreeMap<K, V>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: &mut D) -> Result<BTreeSet<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<Entry<<K as >::Archived, <V as >::Archived>>, deserializer: &mut D) -> Result<HashMap<K, V, H>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: &mut D) -> Result<HashSet<T, H>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<u8>, _: &mut D) -> Result<AlignedVec<A>, <D as >::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[T] { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<VecDeque<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<Vec<T>, <D as >::Error> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: I) -> &<Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Receiver**
- **Verify**
  - ```rust
    fn verify(self: &Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
#### Struct `VecResolver`

The resolver for [`ArchivedVec`].

```rust
pub struct VecResolver {
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
  pub fn from_pos(pos: usize) -> Self { /* ... */ }
  ```
  Creates a new `VecResolver` from a position in the output buffer where

###### Trait Implementations

- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
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

- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `with`

Wrapper type support and commonly used wrappers.

Wrappers can be applied with the `#[rkyv(with = ..)]` attribute in the
[`Archive`](macro@crate::Archive) macro.

```rust
pub mod with { /* ... */ }
```

### Types

#### Struct `With`

A transparent wrapper which applies a "with" type.

`With` wraps a reference to a type and applies the specified wrapper type
when serializing and deserializing.

```rust
pub struct With<F: ?Sized, W> {
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
  pub fn cast(field: &F) -> &Self { /* ... */ }
  ```
  Casts a `With` reference from a reference to the underlying field.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ArchiveUnsized**
  - ```rust
    fn archived_metadata(self: &Self) -> <<T as ArchiveUnsized>::Archived as ArchivePointee>::ArchivedMetadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **SerializeUnsized**
  - ```rust
    fn serialize_unsized(self: &Self, serializer: &mut S) -> Result<usize, <S as Fallible>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Unpin**
- **Pointee**
- **Serialize**
  - ```rust
    fn serialize(self: &Self, serializer: &mut S) -> Result<<Self as >::Resolver, <S as Fallible>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize(self: &Self, deserializer: &mut D) -> Result<T, <D as Fallible>::Error> { /* ... */ }
    ```

- **Archive**
  - ```rust
    fn resolve(self: &Self, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Map`

A wrapper that applies another wrapper to the values contained in a type.
This can be applied to a vector to map each element, or an option to map any
contained value.

See [ArchiveWith] for more details.

# Example

```
use rkyv::{
    with::{InlineAsBox, Map},
    Archive,
};

#[derive(Archive)]
struct Example<'a> {
    // This will apply `InlineAsBox` to the `&i32` contained in this option
    #[rkyv(with = Map<InlineAsBox>)]
    option: Option<&'a i32>,
    // This will apply `InlineAsBox` to each `&i32` contained in this vector
    #[rkyv(with = Map<InlineAsBox>)]
    vec: Vec<&'a i32>,
}
```

```rust
pub struct Map<T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Pointee**
- **Send**
- **Unpin**
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **SerializeWith**
  - ```rust
    fn serialize_with(field: &Vec<O>, s: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &Option<O>, s: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedVec<<A as ArchiveWith<O>>::Archived>, d: &mut D) -> Result<Vec<O>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOption<<A as ArchiveWith<O>>::Archived>, d: &mut D) -> Result<Option<O>, <D as >::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &Vec<O>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<O>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `MapKV`

A wrapper that applies key and value wrappers to the key-value pairs
contained in a type. This can be applied to a hash map or B-tree map to map
the key-value pairs.

# Example
```
use std::collections::HashMap;

use rkyv::{
    with::{Inline, InlineAsBox, MapKV},
    Archive,
};

#[derive(Archive)]
struct Example<'a> {
    // This will apply `InlineAsBox` to the `&str` key, and `Inline` to the
    // `&str` value.
    #[rkyv(with = MapKV<InlineAsBox, Inline>)]
    hash_map: HashMap<&'a str, &'a str>,
}
```

```rust
pub struct MapKV<K, V> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointee**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &BTreeMap<K, V>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &HashMap<K, V, H>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **UnwindSafe**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedBTreeMap<<A as ArchiveWith<K>>::Archived, <B as ArchiveWith<V>>::Archived>, deserializer: &mut D) -> Result<BTreeMap<K, V>, <D as Fallible>::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedHashMap<<A as ArchiveWith<K>>::Archived, <B as ArchiveWith<V>>::Archived>, deserializer: &mut D) -> Result<HashMap<K, V, S>, <D as Fallible>::Error> { /* ... */ }
    ```

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
- **Send**
- **SerializeWith**
  - ```rust
    fn serialize_with(field: &BTreeMap<K, V>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as Fallible>::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &HashMap<K, V, H>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as Fallible>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
#### Struct `Relaxed`

A type indicating relaxed atomic loads.

```rust
pub struct Relaxed;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **UnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Acquire`

A type indicating acquire atomic loads.

```rust
pub struct Acquire;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
#### Struct `SeqCst`

A type indicating sequentially-consistent atomic loads.

```rust
pub struct SeqCst;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

#### Struct `AtomicLoad`

A wrapper that archives an atomic by loading its value with a particular
ordering.

When serializing, the specified ordering will be used to load the value from
the source atomic. The underlying archived type is still a non-atomic value.

# Example

```
# #[cfg(target_has_atomic = "32")]
use core::sync::atomic::AtomicU32;

use rkyv::{
    with::{AtomicLoad, Relaxed},
    Archive,
};

# #[cfg(target_has_atomic = "32")]
#[derive(Archive)]
struct Example {
    #[rkyv(with = AtomicLoad<Relaxed>)]
    a: AtomicU32,
}
```

```rust
pub struct AtomicLoad<SO> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedI16, _: &mut D) -> Result<core::sync::atomic::AtomicI16, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedU16, _: &mut D) -> Result<core::sync::atomic::AtomicU16, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &rend::i16_le, _: &mut D) -> Result<rend::AtomicI16_le, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &rend::i16_be, _: &mut D) -> Result<rend::AtomicI16_be, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &rend::u16_le, _: &mut D) -> Result<rend::AtomicU16_le, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &rend::u16_be, _: &mut D) -> Result<rend::AtomicU16_be, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedI32, _: &mut D) -> Result<core::sync::atomic::AtomicI32, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedU32, _: &mut D) -> Result<core::sync::atomic::AtomicU32, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedI32, _: &mut D) -> Result<rend::AtomicI32_le, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedI32, _: &mut D) -> Result<rend::AtomicI32_be, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedU32, _: &mut D) -> Result<rend::AtomicU32_le, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedU32, _: &mut D) -> Result<rend::AtomicU32_be, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedI64, _: &mut D) -> Result<core::sync::atomic::AtomicI64, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedU64, _: &mut D) -> Result<core::sync::atomic::AtomicU64, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedI64, _: &mut D) -> Result<rend::AtomicI64_le, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedI64, _: &mut D) -> Result<rend::AtomicI64_be, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedU64, _: &mut D) -> Result<rend::AtomicU64_le, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedU64, _: &mut D) -> Result<rend::AtomicU64_be, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedIsize, _: &mut D) -> Result<core::sync::atomic::AtomicIsize, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &crate::primitive::ArchivedUsize, _: &mut D) -> Result<core::sync::atomic::AtomicUsize, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &bool, _: &mut D) -> Result<AtomicBool, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &i8, _: &mut D) -> Result<AtomicI8, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &u8, _: &mut D) -> Result<AtomicU8, <D as >::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **UnwindSafe**
- **Pointee**
- **SerializeWith**
  - ```rust
    fn serialize_with(_: &core::sync::atomic::AtomicI16, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &core::sync::atomic::AtomicU16, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicI16_le, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicI16_be, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicU16_le, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicU16_be, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &core::sync::atomic::AtomicI32, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &core::sync::atomic::AtomicU32, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicI32_le, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicI32_be, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicU32_le, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicU32_be, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &core::sync::atomic::AtomicI64, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &core::sync::atomic::AtomicU64, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicI64_le, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicI64_be, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicU64_le, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &rend::AtomicU64_be, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &core::sync::atomic::AtomicIsize, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &core::sync::atomic::AtomicUsize, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &AtomicBool, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &AtomicI8, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &AtomicU8, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &core::sync::atomic::AtomicI16, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &core::sync::atomic::AtomicU16, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicI16_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicI16_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicU16_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicU16_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &core::sync::atomic::AtomicI32, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &core::sync::atomic::AtomicU32, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicI32_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicI32_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicU32_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicU32_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &core::sync::atomic::AtomicI64, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &core::sync::atomic::AtomicU64, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicI64_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicI64_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicU64_le, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &rend::AtomicU64_be, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &core::sync::atomic::AtomicIsize, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &core::sync::atomic::AtomicUsize, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &AtomicBool, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &AtomicI8, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &AtomicU8, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Inline`

A wrapper that serializes a reference inline.

References serialized with `Inline` cannot be deserialized because the
struct cannot own the deserialized value.

# Example

```
use rkyv::{with::Inline, Archive};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = Inline)]
    a: &'a i32,
}
```

```rust
pub struct Inline;
```

##### Implementations

###### Trait Implementations

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **SerializeWith**
  - ```rust
    fn serialize_with(field: &&F, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &&F, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `AsBox`

A wrapper that serializes a field into a box.

This functions similarly to [`InlineAsBox`], but is for regular fields
instead of references.

# Example

```
use rkyv::{with::AsBox, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = AsBox)]
    a: i32,
    #[rkyv(with = AsBox)]
    b: str,
}
```

```rust
pub struct AsBox;
```

##### Implementations

###### Trait Implementations

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &F, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedBox<<F as >::Archived>, deserializer: &mut D) -> Result<F, <D as >::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &F, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

#### Struct `InlineAsBox`

A wrapper that serializes a reference as if it were boxed.

Unlike [`Inline`], unsized references can be serialized with `InlineAsBox`.

References serialized with `InlineAsBox` cannot be deserialized because the
struct cannot own the deserialized value.

# Example

```
use rkyv::{with::InlineAsBox, Archive};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = InlineAsBox)]
    a: &'a i32,
    #[rkyv(with = InlineAsBox)]
    b: &'a str,
}
```

```rust
pub struct InlineAsBox;
```

##### Implementations

###### Trait Implementations

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &&F, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &&F, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
#### Struct `AsString`

A wrapper that attempts to convert a type to and from UTF-8.

Types like `OsString` and `PathBuf` aren't guaranteed to be encoded as
UTF-8, but they usually are anyway. Using this wrapper will archive them as
if they were regular `String`s.

# Example

```
use std::{ffi::OsString, path::PathBuf};

use rkyv::{with::AsString, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = AsString)]
    os_string: OsString,
    #[rkyv(with = AsString)]
    path: PathBuf,
}
```

```rust
pub struct AsString;
```

##### Implementations

###### Trait Implementations

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &OsString, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &PathBuf, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &OsString, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &PathBuf, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **Pointee**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Sync**
- **Send**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedString, _: &mut D) -> Result<OsString, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedString, _: &mut D) -> Result<PathBuf, <D as >::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
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

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Lock`

A wrapper that locks a lock and serializes the value immutably.

This wrapper can panic under very specific circumstances when:

1. `serialize_with` is called and succeeds in locking the value to serialize
   it.
2. Another thread locks the value and panics, poisoning the lock
3. `resolve_with` is called and gets a poisoned value.

Unfortunately, it's not possible to work around this issue internally. Users
must ensure this doesn't happen on their own through manual synchronization
or guaranteeing that panics do not occur while holding locks.

# Example

```
use std::sync::Mutex;

use rkyv::{with::Lock, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = Lock)]
    a: Mutex<i32>,
}
```

```rust
pub struct Lock;
```

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **Send**
- **Unpin**
- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &Mutex<F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &RwLock<F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **SerializeWith**
  - ```rust
    fn serialize_with(field: &Mutex<F>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &RwLock<F>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &F, deserializer: &mut D) -> Result<Mutex<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &F, deserializer: &mut D) -> Result<RwLock<T>, <D as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `AsOwned`

A wrapper that serializes a `Cow` as if it were owned.

# Example

```
use std::borrow::Cow;

use rkyv::{with::AsOwned, Archive};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = AsOwned)]
    a: Cow<'a, str>,
}
```

```rust
pub struct AsOwned;
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &Cow<''a, F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Cow<''a, [T]>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Cow<''a, str>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Cow<''a, CStr>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **Freeze**
- **SerializeWith**
  - ```rust
    fn serialize_with(field: &Cow<''a, F>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &Cow<''a, [T]>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &Cow<''a, str>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &Cow<''a, CStr>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Pointee**
- **RefUnwindSafe**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &<T as >::Archived, deserializer: &mut D) -> Result<T, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: &mut D) -> Result<Cow<''a, [T]>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedString, deserializer: &mut D) -> Result<Cow<''a, str>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedCString, deserializer: &mut D) -> Result<Cow<''a, CStr>, <D as >::Error> { /* ... */ }
    ```

#### Struct `AsVec`

A wrapper that serializes associative containers as a `Vec` of key-value
pairs.

This provides faster serialization for containers like `HashMap` and
`BTreeMap` by serializing the key-value pairs directly instead of building a
data structure in the buffer.

# Example

```
use std::collections::HashMap;

use rkyv::{with::AsVec, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = AsVec)]
    values: HashMap<String, u32>,
}
```

```rust
pub struct AsVec;
```

##### Implementations

###### Trait Implementations

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
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
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedVec<Entry<<K as >::Archived, <V as >::Archived>>, deserializer: &mut D) -> Result<BTreeMap<K, V>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: &mut D) -> Result<BTreeSet<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<Entry<<K as >::Archived, <V as >::Archived>>, deserializer: &mut D) -> Result<HashMap<K, V, H>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<<T as >::Archived>, deserializer: &mut D) -> Result<HashSet<T, H>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedVec<u8>, _: &mut D) -> Result<AlignedVec<A>, <D as >::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Pointee**
- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &BTreeMap<K, V>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &BTreeSet<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &HashMap<K, V, H>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &HashSet<T, H>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &AlignedVec<A>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &BTreeMap<K, V>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &BTreeSet<T>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &HashMap<K, V, H>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &HashSet<T, H>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &AlignedVec<A>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Niche`

A wrapper that niches some type combinations.

A common type combination is `Option<Box<T>>`. By using a null pointer, the
archived version can save some space on-disk.

# Example

```
use core::mem::size_of;

use rkyv::{with::Niche, Archive, Archived};

#[derive(Archive)]
struct BasicExample {
    value: Option<Box<str>>,
}

#[derive(Archive)]
struct NichedExample {
    #[rkyv(with = Niche)]
    value: Option<Box<str>>,
}

assert!(
    size_of::<Archived<BasicExample>>()
        > size_of::<Archived<NichedExample>>()
);
```

```rust
pub struct Niche;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedOptionBox<<T as >::Archived>, deserializer: &mut D) -> Result<Option<Box<T>>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI8, _: &mut D) -> Result<Option<NonZeroI8>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI16, _: &mut D) -> Result<Option<NonZeroI16>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI32, _: &mut D) -> Result<Option<NonZeroI32>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI64, _: &mut D) -> Result<Option<NonZeroI64>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroI128, _: &mut D) -> Result<Option<NonZeroI128>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU8, _: &mut D) -> Result<Option<NonZeroU8>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU16, _: &mut D) -> Result<Option<NonZeroU16>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU32, _: &mut D) -> Result<Option<NonZeroU32>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU64, _: &mut D) -> Result<Option<NonZeroU64>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroU128, _: &mut D) -> Result<Option<NonZeroU128>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroIsize, _: &mut D) -> Result<Option<NonZeroIsize>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &ArchivedOptionNonZeroUsize, _: &mut D) -> Result<Option<NonZeroUsize>, <D as >::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **Send**
- **Sync**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &Option<Box<T>>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroI8>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroI16>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroI32>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroI64>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroI128>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroU8>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroU16>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroU32>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroU64>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroU128>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroIsize>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Option<NonZeroUsize>, _: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &Option<Box<T>>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroI8>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroI16>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroI32>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroI64>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroI128>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroU8>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroU16>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroU32>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroU64>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroU128>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroIsize>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(_: &Option<NonZeroUsize>, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

#### Struct `NicheInto`

A wrapper that niches based on a generic [`Niching`].

A common type combination is `Option<Box<T>>`. By niching `None` into the
null pointer, the archived version can save some space on-disk.

# Example

```
use core::mem::size_of;

use rkyv::{
    niche::niching::{NaN, Null},
    with::NicheInto,
    Archive, Archived,
};

#[derive(Archive)]
struct BasicExample {
    maybe_box: Option<Box<str>>,
    maybe_non_nan: Option<f32>,
}

#[derive(Archive)]
struct NichedExample {
    #[rkyv(with = NicheInto<Null>)]
    maybe_box: Option<Box<str>>,
    #[rkyv(with = NicheInto<NaN>)]
    maybe_non_nan: Option<f32>,
}

assert!(
    size_of::<Archived<BasicExample>>()
        > size_of::<Archived<NichedExample>>()
);
```

[`Niching`]: crate::niche::niching::Niching

```rust
pub struct NicheInto<N: ?Sized>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &Option<T>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &NichedOption<<T as >::Archived, N>, deserializer: &mut D) -> Result<Option<T>, <D as >::Error> { /* ... */ }
    ```

- **Send**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

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

- **Freeze**
- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &Option<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
#### Struct `MapNiche`

A wrapper that first applies another wrapper `W` to the value inside an
`Option` and then niches the result based on the [`Niching`] `N`.

# Example

```
use rkyv::{
    with::{AsBox, MapNiche},
    Archive, Serialize,
};

#[derive(Archive, Serialize)]
struct BasicExample {
    option: Option<HugeType>,
}

#[derive(Archive, Serialize)]
struct NichedExample {
    #[rkyv(with = MapNiche<AsBox>)]
    option: Option<HugeType>,
}

#[derive(Archive, Serialize)]
struct HugeType([u8; 1024]);

# fn main() -> Result<(), rkyv::rancor::Error> {
let basic_value = BasicExample { option: None };
let basic_bytes = rkyv::to_bytes(&basic_value)?;
assert_eq!(basic_bytes.len(), 1 + 1024);

let niched_value = NichedExample { option: None };
let niched_bytes = rkyv::to_bytes(&niched_value)?;
assert_eq!(niched_bytes.len(), 4); // size_of::<ArchivedBox<_>>()
# Ok(()) }
```

[`Niching`]: crate::niche::niching::Niching

```rust
pub struct MapNiche<W: ?Sized, N: ?Sized = DefaultNiche> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
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

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &Option<T>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &Option<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &NichedOption<<W as ArchiveWith<T>>::Archived, N>, deserializer: &mut D) -> Result<Option<T>, <D as >::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `AsUnixTime`

A wrapper that converts a [`SystemTime`](std::time::SystemTime) to a
[`Duration`](std::time::Duration) since
[`UNIX_EPOCH`](std::time::UNIX_EPOCH).

If the serialized time occurs before the UNIX epoch, serialization will
panic during `resolve`. The resulting archived time will be an
[`ArchivedDuration`](crate::time::ArchivedDuration) relative to the UNIX
epoch.

# Example

```
use rkyv::{Archive, with::AsUnixTime};
use std::time::SystemTime;

#[derive(Archive)]
struct Example {
    #[rkyv(with = AsUnixTime)]
    time: SystemTime,
}

```rust
pub struct AsUnixTime;
```

##### Implementations

###### Trait Implementations

- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &ArchivedDuration, _: &mut D) -> Result<SystemTime, <D as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &SystemTime, _: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Pointee**
- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &SystemTime, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

#### Struct `Unsafe`

A wrapper that allows serialize-unsafe types to be serialized.

Types like `Cell` and `UnsafeCell` may contain serializable types, but have
unsafe access semantics due to interior mutability. They may be safe to
serialize, but only under conditions that rkyv is unable to guarantee.

This wrapper enables serializing these types, and places the burden of
verifying that their access semantics are used safely on the user.

# Safety

Using this wrapper on types with interior mutability can create races
conditions or allow access to data in an invalid state if access semantics
are not followed properly. During serialization, the data must not be
modified.

# Example

```
use core::cell::{Cell, UnsafeCell};

use rkyv::{with::Unsafe, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = Unsafe)]
    cell: Cell<String>,
    #[rkyv(with = Unsafe)]
    unsafe_cell: UnsafeCell<String>,
}
```

```rust
pub struct Unsafe;
```

##### Implementations

###### Trait Implementations

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &UnsafeCell<F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(field: &Cell<F>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &UnsafeCell<F>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(field: &Cell<F>, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &<F as >::Archived, deserializer: &mut D) -> Result<UnsafeCell<F>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(field: &<F as >::Archived, deserializer: &mut D) -> Result<Cell<F>, <D as >::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Sync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Skip`

A wrapper that skips serializing a field.

Skipped fields must implement `Default` to be deserialized.

# Example

```
use rkyv::{with::Skip, Archive};

#[derive(Archive)]
struct Example {
    #[rkyv(with = Skip)]
    a: u32,
}
```

```rust
pub struct Skip;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(_: &F, _: &mut S) -> Result<(), <S as >::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DeserializeWith**
  - ```rust
    fn deserialize_with(_: &(), _: &mut D) -> Result<F, <D as >::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Pointee**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **ArchiveWith**
  - ```rust
    fn resolve_with(_: &F, _: <Self as >::Resolver, _: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Unshare`

A wrapper that clones the contents of `Arc` and `Rc` pointers.

```rust
pub struct Unshare;
```

##### Implementations

###### Trait Implementations

- **ArchiveWith**
  - ```rust
    fn resolve_with(x: &crate::alloc::sync::Arc<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

  - ```rust
    fn resolve_with(x: &Rc<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
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

- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DeserializeWith**
  - ```rust
    fn deserialize_with(x: &A, d: &mut D) -> Result<crate::alloc::sync::Arc<T>, <D as >::Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_with(x: &A, d: &mut D) -> Result<Rc<T>, <D as >::Error> { /* ... */ }
    ```

- **Sync**
- **SerializeWith**
  - ```rust
    fn serialize_with(x: &crate::alloc::sync::Arc<T>, s: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_with(x: &Rc<T>, s: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Unpin**
#### Struct `Identity`

A no-op wrapper which uses the default impls for the type.

This is most useful for wrappers like [`MapKV`] when you only want to apply
a wrapper to either the key or the value.

# Example

```
use std::collections::HashMap;

use rkyv::{
    with::{Identity, Inline, MapKV},
    Archive,
};

#[derive(Archive)]
struct Example<'a> {
    #[rkyv(with = MapKV<Identity, Inline>)]
    a: HashMap<u32, &'a u32>,
}
```

```rust
pub struct Identity;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **DeserializeWith**
  - ```rust
    fn deserialize_with(field: &F, deserializer: &mut D) -> Result<T, <D as Fallible>::Error> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **LayoutRaw**
  - ```rust
    fn layout_raw(_: <T as Pointee>::Metadata) -> Result<Layout, LayoutError> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
- **Unpin**
- **UnwindSafe**
- **Niching**
  - ```rust
    unsafe fn is_niched(niched: *const NichedOption<T, N1>) -> bool { /* ... */ }
    ```

  - ```rust
    fn resolve_niched(out: Place<NichedOption<T, N1>>) { /* ... */ }
    ```

- **ArchivePointee**
  - ```rust
    fn pointer_metadata(_: &<T as ArchivePointee>::ArchivedMetadata) -> <T as Pointee>::Metadata { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **ArchiveWith**
  - ```rust
    fn resolve_with(field: &F, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>) { /* ... */ }
    ```

- **SerializeWith**
  - ```rust
    fn serialize_with(field: &F, serializer: &mut S) -> Result<<Self as >::Resolver, <S as >::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Traits

#### Trait `ArchiveWith`

A variant of [`Archive`] that works with wrappers.

Creating a wrapper allows users to customize how fields are archived easily
without changing the unarchived type.

This trait allows wrapper types to transparently change the archive
behaviors for struct and enum fields. When a field is serialized, it may use
the implementations for the wrapper type and the given field instead of the
implementation for the type itself.

Only a single implementation of [`Archive`] may be written
for each type, but multiple implementations of ArchiveWith can be written
for the same type because it is parametric over the wrapper type. This is
used with the `#[rkyv(with = ..)]` macro attribute to provide a more
flexible interface for serialization.

# Example

```
use rkyv::{
    access_unchecked, deserialize,
    rancor::{Error, Fallible, Infallible, ResultExt as _},
    to_bytes,
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Archived, Deserialize, Place, Resolver, Serialize,
};

struct Incremented;

impl ArchiveWith<i32> for Incremented {
    type Archived = Archived<i32>;
    type Resolver = Resolver<i32>;

    fn resolve_with(field: &i32, _: (), out: Place<Self::Archived>) {
        let incremented = field + 1;
        incremented.resolve((), out);
    }
}

impl<S> SerializeWith<i32, S> for Incremented
where
    S: Fallible + ?Sized,
    i32: Serialize<S>,
{
    fn serialize_with(
        field: &i32,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        let incremented = field + 1;
        incremented.serialize(serializer)
    }
}

impl<D> DeserializeWith<Archived<i32>, i32, D> for Incremented
where
    D: Fallible + ?Sized,
    Archived<i32>: Deserialize<i32, D>,
{
    fn deserialize_with(
        field: &Archived<i32>,
        deserializer: &mut D,
    ) -> Result<i32, D::Error> {
        Ok(field.deserialize(deserializer)? - 1)
    }
}

#[derive(Archive, Deserialize, Serialize)]
struct Example {
    #[rkyv(with = Incremented)]
    a: i32,
    // Another i32 field, but not incremented this time
    b: i32,
}

let value = Example { a: 4, b: 9 };

let buf = to_bytes::<Error>(&value).unwrap();

let archived =
    unsafe { access_unchecked::<Archived<Example>>(buf.as_ref()) };
// The wrapped field has been incremented
assert_eq!(archived.a, 5);
// ... and the unwrapped field has not
assert_eq!(archived.b, 9);

let deserialized = deserialize::<Example, Infallible>(archived).always_ok();
// The wrapped field is back to normal
assert_eq!(deserialized.a, 4);
// ... and the unwrapped field is unchanged
assert_eq!(deserialized.b, 9);
```

```rust
pub trait ArchiveWith<F: ?Sized> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Archived`: The archived type of `Self` with `F`.
- `Resolver`: The resolver of a `Self` with `F`.

###### Required Methods

- `resolve_with`: Resolves the archived type using a reference to the field type `F`.

##### Implementations

This trait is implemented for the following types:

- `crate::with::MapKV<A, B>` with <A, B, K, V>
- `crate::with::Map<A>` with <A, O>
- `crate::with::AsOwned` with <''a, F: Archive + Clone>
- `crate::with::AsOwned` with <''a, T: Archive + Clone>
- `crate::with::AsOwned` with <''a>
- `crate::with::AsVec` with <K: Archive, V: Archive>
- `crate::with::AsVec` with <T: Archive>
- `crate::with::Niche` with <T>
- `crate::with::Unshare` with <T: Archive>
- `crate::with::Unshare` with <T: Archive>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `AtomicLoad<SO>` with <SO: LoadOrdering>
- `crate::with::InlineAsBox` with <F: ArchiveUnsized + ?Sized>
- `crate::with::AsBox` with <F: ArchiveUnsized + ?Sized>
- `crate::with::Map<A>` with <A, O>
- `Niche`
- `Niche`
- `Niche`
- `Niche`
- `Niche`
- `Niche`
- `Niche`
- `Niche`
- `Niche`
- `Niche`
- `crate::with::Niche`
- `crate::with::Niche`
- `crate::with::NicheInto<N>` with <T, N>
- `crate::with::MapNiche<W, N>` with <T, W, N>
- `crate::niche::niching::DefaultNiche` with <T>
- `crate::with::Inline` with <F: Archive>
- `crate::with::Unsafe` with <F: Archive>
- `crate::with::Unsafe` with <F: Archive>
- `crate::with::Skip` with <F>
- `crate::with::Identity` with <F: Archive>
- `crate::with::MapKV<A, B>` with <A, B, K, V, H>
- `crate::with::AsString`
- `crate::with::AsString`
- `crate::with::Lock` with <F: Archive>
- `crate::with::Lock` with <F: Archive>
- `crate::with::AsVec` with <K: Archive, V: Archive, H>
- `crate::with::AsVec` with <T: Archive, H>
- `crate::with::AsUnixTime`
- `crate::with::AsOwned` with <''a>
- `crate::with::AsVec` with <const A: usize>

#### Trait `SerializeWith`

A variant of `Serialize` for "with" types.

See [ArchiveWith] for more details.

```rust
pub trait SerializeWith<F: ?Sized, S: Fallible + ?Sized>: ArchiveWith<F> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `serialize_with`: Serializes the field type `F` using the given serializer.

##### Implementations

This trait is implemented for the following types:

- `crate::with::MapKV<A, B>` with <A, B, K, V, S>
- `crate::with::Map<A>` with <A, O, S>
- `crate::with::AsOwned` with <''a, F, S>
- `crate::with::AsOwned` with <''a, T, S>
- `crate::with::AsOwned` with <''a, S>
- `crate::with::AsVec` with <K, V, S>
- `crate::with::AsVec` with <T, S>
- `crate::with::Niche` with <T, S>
- `crate::with::Unshare` with <T, S>
- `crate::with::Unshare` with <T: Serialize<S>, S: Fallible + ?Sized>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `$crate::with::AtomicLoad<SO>` with <S, SO>
- `crate::with::InlineAsBox` with <F, S>
- `crate::with::AsBox` with <F, S>
- `crate::with::Map<A>` with <A, O, S>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `Niche` with <S: Fallible + ?Sized>
- `crate::with::Niche` with <S: Fallible + ?Sized>
- `crate::with::Niche` with <S: Fallible + ?Sized>
- `crate::with::NicheInto<N>` with <T, N, S>
- `crate::with::MapNiche<W, N>` with <T, W, N, S>
- `crate::niche::niching::DefaultNiche` with <T, S>
- `crate::with::Inline` with <F: Serialize<S>, S: Fallible + ?Sized>
- `crate::with::Unsafe` with <F, S>
- `crate::with::Unsafe` with <F, S>
- `crate::with::Skip` with <F, S: Fallible + ?Sized>
- `crate::with::Identity` with <F: Serialize<S>, S: Fallible + ?Sized>
- `crate::with::MapKV<A, B>` with <A, B, K, V, S, H>
- `crate::with::AsString` with <S>
- `crate::with::AsString` with <S>
- `crate::with::Lock` with <F, S>
- `crate::with::Lock` with <F, S>
- `crate::with::AsVec` with <K, V, H, S>
- `crate::with::AsVec` with <T, H, S>
- `crate::with::AsUnixTime` with <S>
- `crate::with::AsOwned` with <''a, S>
- `crate::with::AsVec` with <S, const A: usize>

#### Trait `DeserializeWith`

A variant of `Deserialize` for "with" types.

See [ArchiveWith] for more details.

```rust
pub trait DeserializeWith<F: ?Sized, T, D: Fallible + ?Sized> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `deserialize_with`: Deserializes the field type `F` using the given deserializer.

##### Implementations

This trait is implemented for the following types:

- `crate::with::MapKV<A, B>` with <A, B, K, V, D>
- `crate::with::Map<A>` with <A, O, D>
- `crate::with::AsOwned` with <T, D>
- `crate::with::AsOwned` with <''a, T, D>
- `crate::with::AsOwned` with <''a, D>
- `crate::with::AsVec` with <K, V, D>
- `crate::with::AsVec` with <T, D>
- `crate::with::Niche` with <T, D>
- `crate::with::Unshare` with <A, T, D>
- `crate::with::Unshare` with <A, T, D>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `AtomicLoad<SO>` with <D, SO>
- `crate::with::AsBox` with <F, D>
- `crate::with::Map<A>` with <A, O, D>
- `Niche` with <D>
- `Niche` with <D>
- `Niche` with <D>
- `Niche` with <D>
- `Niche` with <D>
- `Niche` with <D>
- `Niche` with <D>
- `Niche` with <D>
- `Niche` with <D>
- `Niche` with <D>
- `crate::with::Niche` with <D>
- `crate::with::Niche` with <D>
- `crate::with::NicheInto<N>` with <T, N, D>
- `crate::with::MapNiche<W, N>` with <T, W, N, D>
- `crate::niche::niching::DefaultNiche` with <T, D>
- `crate::with::Unsafe` with <F, D>
- `crate::with::Unsafe` with <F, D>
- `crate::with::Skip` with <F: Default, D: Fallible + ?Sized>
- `crate::with::Identity` with <F, T, D>
- `crate::with::MapKV<A, B>` with <A, B, K, V, D, S>
- `crate::with::AsString` with <D>
- `crate::with::AsString` with <D>
- `crate::with::Lock` with <F, T, D>
- `crate::with::Lock` with <F, T, D>
- `crate::with::AsVec` with <K, V, H, D>
- `crate::with::AsVec` with <T, H, D>
- `crate::with::AsUnixTime` with <D>
- `crate::with::AsOwned` with <''a, D>
- `crate::with::AsVec` with <D, const A: usize>

### Re-exports

#### Re-export `DefaultNiche`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::niche::niching::DefaultNiche;
```

## Re-exports

### Re-export `bytecheck`

**Attributes:**

- `#[<cfg>(feature = "bytecheck")]`

```rust
pub use ::bytecheck;
```

### Re-export `munge`

```rust
pub use ::munge;
```

### Re-export `ptr_meta`

```rust
pub use ::ptr_meta;
```

### Re-export `rancor`

```rust
pub use ::rancor;
```

### Re-export `rend`

```rust
pub use ::rend;
```

### Re-export `access`

**Attributes:**

- `#[<cfg>(all(feature = "bytecheck", feature = "alloc"))]`
- `#[doc(inline)]`

```rust
pub use api::high::access;
```

### Re-export `access_mut`

**Attributes:**

- `#[<cfg>(all(feature = "bytecheck", feature = "alloc"))]`
- `#[doc(inline)]`

```rust
pub use api::high::access_mut;
```

### Re-export `from_bytes`

**Attributes:**

- `#[<cfg>(all(feature = "bytecheck", feature = "alloc"))]`
- `#[doc(inline)]`

```rust
pub use api::high::from_bytes;
```

### Re-export `deserialize`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`
- `#[doc(inline)]`

```rust
pub use api::high::deserialize;
```

### Re-export `from_bytes_unchecked`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`
- `#[doc(inline)]`

```rust
pub use api::high::from_bytes_unchecked;
```

### Re-export `to_bytes`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`
- `#[doc(inline)]`

```rust
pub use api::high::to_bytes;
```

### Re-export `access_unchecked`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::api::access_unchecked;
```

### Re-export `access_unchecked_mut`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::api::access_unchecked_mut;
```

### Re-export `Place`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::place::Place;
```

### Re-export `Archive`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::Archive;
```

### Re-export `Archive`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::Archive;
```

### Re-export `ArchiveUnsized`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::ArchiveUnsized;
```

### Re-export `Deserialize`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::Deserialize;
```

### Re-export `Deserialize`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::Deserialize;
```

### Re-export `DeserializeUnsized`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::DeserializeUnsized;
```

### Re-export `Portable`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::Portable;
```

### Re-export `Portable`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::Portable;
```

### Re-export `Serialize`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::Serialize;
```

### Re-export `Serialize`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::Serialize;
```

### Re-export `SerializeUnsized`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::traits::SerializeUnsized;
```

### Re-export `crate::alias::*`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::alias::*;
```

