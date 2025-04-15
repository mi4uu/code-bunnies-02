# Crate Documentation

**Version:** 0.8.24

**Format Version:** 43

# Module `zerocopy`

*<span style="font-size: 100%; color:grey;">Need more out of zerocopy?
Submit a [customer request issue][customer-request-issue]!</span>*

***<span style="font-size: 140%">Fast, safe, <span
style="color:red;">compile error</span>. Pick two.</span>***

Zerocopy makes zero-cost memory manipulation effortless. We write `unsafe`
so you don't have to.

*Thanks for using zerocopy 0.8! For an overview of what changes from 0.7,
check out our [release notes][release-notes], which include a step-by-step
guide for upgrading from 0.7.*

*Have questions? Need help? Ask the maintainers on [GitHub][github-q-a] or
on [Discord][discord]!*

[customer-request-issue]: https://github.com/google/zerocopy/issues/new/choose
[release-notes]: https://github.com/google/zerocopy/discussions/1680
[github-q-a]: https://github.com/google/zerocopy/discussions/categories/q-a
[discord]: https://discord.gg/MAvWH2R6zk

# Overview

##### Conversion Traits

Zerocopy provides four derivable traits for zero-cost conversions:
- [`TryFromBytes`] indicates that a type may safely be converted from
  certain byte sequences (conditional on runtime checks)
- [`FromZeros`] indicates that a sequence of zero bytes represents a valid
  instance of a type
- [`FromBytes`] indicates that a type may safely be converted from an
  arbitrary byte sequence
- [`IntoBytes`] indicates that a type may safely be converted *to* a byte
  sequence

These traits support sized types, slices, and [slice DSTs][slice-dsts].

[slice-dsts]: KnownLayout#dynamically-sized-types

##### Marker Traits

Zerocopy provides three derivable marker traits that do not provide any
functionality themselves, but are required to call certain methods provided
by the conversion traits:
- [`KnownLayout`] indicates that zerocopy can reason about certain layout
  qualities of a type
- [`Immutable`] indicates that a type is free from interior mutability,
  except by ownership or an exclusive (`&mut`) borrow
- [`Unaligned`] indicates that a type's alignment requirement is 1

You should generally derive these marker traits whenever possible.

##### Conversion Macros

Zerocopy provides six macros for safe casting between types:

- ([`try_`][try_transmute])[`transmute`] (conditionally) converts a value of
  one type to a value of another type of the same size
- ([`try_`][try_transmute_mut])[`transmute_mut`] (conditionally) converts a
  mutable reference of one type to a mutable reference of another type of
  the same size
- ([`try_`][try_transmute_ref])[`transmute_ref`] (conditionally) converts a
  mutable or immutable reference of one type to an immutable reference of
  another type of the same size

These macros perform *compile-time* size and alignment checks, meaning that
unconditional casts have zero cost at runtime. Conditional casts do not need
to validate size or alignment runtime, but do need to validate contents.

These macros cannot be used in generic contexts. For generic conversions,
use the methods defined by the [conversion traits](#conversion-traits).

##### Byteorder-Aware Numerics

Zerocopy provides byte-order aware integer types that support these
conversions; see the [`byteorder`] module. These types are especially useful
for network parsing.

# Cargo Features

- **`alloc`**
  By default, `zerocopy` is `no_std`. When the `alloc` feature is enabled,
  the `alloc` crate is added as a dependency, and some allocation-related
  functionality is added.

- **`std`**
  By default, `zerocopy` is `no_std`. When the `std` feature is enabled, the
  `std` crate is added as a dependency (ie, `no_std` is disabled), and
  support for some `std` types is added. `std` implies `alloc`.

- **`derive`**
  Provides derives for the core marker traits via the `zerocopy-derive`
  crate. These derives are re-exported from `zerocopy`, so it is not
  necessary to depend on `zerocopy-derive` directly.

  However, you may experience better compile times if you instead directly
  depend on both `zerocopy` and `zerocopy-derive` in your `Cargo.toml`,
  since doing so will allow Rust to compile these crates in parallel. To do
  so, do *not* enable the `derive` feature, and list both dependencies in
  your `Cargo.toml` with the same leading non-zero version number; e.g:

  ```toml
  [dependencies]
  zerocopy = "0.X"
  zerocopy-derive = "0.X"
  ```

  To avoid the risk of [duplicate import errors][duplicate-import-errors] if
  one of your dependencies enables zerocopy's `derive` feature, import
  derives as `use zerocopy_derive::*` rather than by name (e.g., `use
  zerocopy_derive::FromBytes`).

- **`simd`**
  When the `simd` feature is enabled, `FromZeros`, `FromBytes`, and
  `IntoBytes` impls are emitted for all stable SIMD types which exist on the
  target platform. Note that the layout of SIMD types is not yet stabilized,
  so these impls may be removed in the future if layout changes make them
  invalid. For more information, see the Unsafe Code Guidelines Reference
  page on the [layout of packed SIMD vectors][simd-layout].

- **`simd-nightly`**
  Enables the `simd` feature and adds support for SIMD types which are only
  available on nightly. Since these types are unstable, support for any type
  may be removed at any point in the future.

- **`float-nightly`**
  Adds support for the unstable `f16` and `f128` types. These types are
  not yet fully implemented and may not be supported on all platforms.

[duplicate-import-errors]: https://github.com/google/zerocopy/issues/1587
[simd-layout]: https://rust-lang.github.io/unsafe-code-guidelines/layout/packed-simd-vectors.html

# Security Ethos

Zerocopy is expressly designed for use in security-critical contexts. We
strive to ensure that that zerocopy code is sound under Rust's current
memory model, and *any future memory model*. We ensure this by:
- **...not 'guessing' about Rust's semantics.**
  We annotate `unsafe` code with a precise rationale for its soundness that
  cites a relevant section of Rust's official documentation. When Rust's
  documented semantics are unclear, we work with the Rust Operational
  Semantics Team to clarify Rust's documentation.
- **...rigorously testing our implementation.**
  We run tests using [Miri], ensuring that zerocopy is sound across a wide
  array of supported target platforms of varying endianness and pointer
  width, and across both current and experimental memory models of Rust.
- **...formally proving the correctness of our implementation.**
  We apply formal verification tools like [Kani][kani] to prove zerocopy's
  correctness.

For more information, see our full [soundness policy].

[Miri]: https://github.com/rust-lang/miri
[Kani]: https://github.com/model-checking/kani
[soundness policy]: https://github.com/google/zerocopy/blob/main/POLICIES.md#soundness

# Relationship to Project Safe Transmute

[Project Safe Transmute] is an official initiative of the Rust Project to
develop language-level support for safer transmutation. The Project consults
with crates like zerocopy to identify aspects of safer transmutation that
would benefit from compiler support, and has developed an [experimental,
compiler-supported analysis][mcp-transmutability] which determines whether,
for a given type, any value of that type may be soundly transmuted into
another type. Once this functionality is sufficiently mature, zerocopy
intends to replace its internal transmutability analysis (implemented by our
custom derives) with the compiler-supported one. This change will likely be
an implementation detail that is invisible to zerocopy's users.

Project Safe Transmute will not replace the need for most of zerocopy's
higher-level abstractions. The experimental compiler analysis is a tool for
checking the soundness of `unsafe` code, not a tool to avoid writing
`unsafe` code altogether. For the foreseeable future, crates like zerocopy
will still be required in order to provide higher-level abstractions on top
of the building block provided by Project Safe Transmute.

[Project Safe Transmute]: https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html
[mcp-transmutability]: https://github.com/rust-lang/compiler-team/issues/411

# MSRV

See our [MSRV policy].

[MSRV policy]: https://github.com/google/zerocopy/blob/main/POLICIES.md#msrv

# Changelog

Zerocopy uses [GitHub Releases].

[GitHub Releases]: https://github.com/google/zerocopy/releases

# Thanks

Zerocopy is maintained by engineers at Google and Amazon with help from
[many wonderful contributors][contributors]. Thank you to everyone who has
lent a hand in making Rust a little more secure!

[contributors]: https://github.com/google/zerocopy/graphs/contributors

## Modules

## Module `byte_slice`

Traits for types that encapsulate a `[u8]`.

These traits are used to bound the `B` parameter of [`Ref`].

```rust
pub mod byte_slice { /* ... */ }
```

### Traits

#### Trait `ByteSlice`

A mutable or immutable reference to a byte slice.

`ByteSlice` abstracts over the mutability of a byte slice reference, and is
implemented for various special reference types such as
[`Ref<[u8]>`](core::cell::Ref) and [`RefMut<[u8]>`](core::cell::RefMut).

# Safety

Implementations of `ByteSlice` must promise that their implementations of
[`Deref`] and [`DerefMut`] are "stable". In particular, given `B: ByteSlice`
and `b: B`, two calls, each to either `b.deref()` or `b.deref_mut()`, must
return a byte slice with the same address and length. This must hold even if
the two calls are separated by an arbitrary sequence of calls to methods on
`ByteSlice`, [`ByteSliceMut`], [`IntoByteSlice`], or [`IntoByteSliceMut`],
or on their super-traits. This does *not* need to hold if the two calls are
separated by any method calls, field accesses, or field modifications *other
than* those from these traits.

Note that this also implies that, given `b: B`, the address and length
cannot be modified via objects other than `b`, either on the same thread or
on another thread.

```rust
pub unsafe trait ByteSlice: Deref<Target = [u8]> + Sized {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `&[u8]`
- `&mut [u8]`
- `cell::Ref<''_, [u8]>`
- `cell::RefMut<''_, [u8]>`

#### Trait `ByteSliceMut`

A mutable reference to a byte slice.

`ByteSliceMut` abstracts over various ways of storing a mutable reference to
a byte slice, and is implemented for various special reference types such as
`RefMut<[u8]>`.

`ByteSliceMut` is a shorthand for [`ByteSlice`] and [`DerefMut`].

```rust
pub trait ByteSliceMut: ByteSlice + DerefMut {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `B` with <B: ByteSlice + DerefMut>

#### Trait `CopyableByteSlice`

A [`ByteSlice`] which can be copied without violating dereference stability.

# Safety

If `B: CopyableByteSlice`, then the dereference stability properties
required by [`ByteSlice`] (see that trait's safety documentation) do not
only hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also
hold regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by
copying `b`.

```rust
pub unsafe trait CopyableByteSlice: ByteSlice + Copy + CloneableByteSlice {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `&[u8]`

#### Trait `CloneableByteSlice`

A [`ByteSlice`] which can be cloned without violating dereference stability.

# Safety

If `B: CloneableByteSlice`, then the dereference stability properties
required by [`ByteSlice`] (see that trait's safety documentation) do not
only hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also
hold regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by
`b.clone()`, `b.clone().clone()`, etc.

```rust
pub unsafe trait CloneableByteSlice: ByteSlice + Clone {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `&[u8]`

#### Trait `SplitByteSlice`

A [`ByteSlice`] that can be split in two.

# Safety

Unsafe code may depend for its soundness on the assumption that `split_at`
and `split_at_unchecked` are implemented correctly. In particular, given `B:
SplitByteSlice` and `b: B`, if `b.deref()` returns a byte slice with address
`addr` and length `len`, then if `split <= len`, both of these
invocations:
- `b.split_at(split)`
- `b.split_at_unchecked(split)`

...will return `(first, second)` such that:
- `first`'s address is `addr` and its length is `split`
- `second`'s address is `addr + split` and its length is `len - split`

```rust
pub unsafe trait SplitByteSlice: ByteSlice {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `split_at_unchecked`: Splits the slice at the midpoint, possibly omitting bounds checks.

##### Provided Methods

- ```rust
  fn split_at(self: Self, mid: usize) -> Result<(Self, Self), Self> { /* ... */ }
  ```
  Attempts to split `self` at the midpoint.

##### Implementations

This trait is implemented for the following types:

- `&[u8]`
- `&mut [u8]`
- `cell::Ref<''_, [u8]>`
- `cell::RefMut<''_, [u8]>`

#### Trait `SplitByteSliceMut`

A shorthand for [`SplitByteSlice`] and [`ByteSliceMut`].

```rust
pub trait SplitByteSliceMut: SplitByteSlice + ByteSliceMut {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `B` with <B: SplitByteSlice + ByteSliceMut>

#### Trait `IntoByteSlice`

**Attributes:**

- `#[allow(clippy::missing_safety_doc)]`

A [`ByteSlice`] that conveys no ownership, and so can be converted into a
byte slice.

Some `ByteSlice` types (notably, the standard library's [`Ref`] type) convey
ownership, and so they cannot soundly be moved by-value into a byte slice
type (`&[u8]`). Some methods in this crate's API (such as [`Ref::into_ref`])
are only compatible with `ByteSlice` types without these ownership
semantics.

[`Ref`]: core::cell::Ref

```rust
pub unsafe trait IntoByteSlice<''a>: ByteSlice {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `into_byte_slice`: Coverts `self` into a `&[u8]`.

##### Implementations

This trait is implemented for the following types:

- `&''a [u8]` with <''a>
- `&''a mut [u8]` with <''a>

#### Trait `IntoByteSliceMut`

**Attributes:**

- `#[allow(clippy::missing_safety_doc)]`

A [`ByteSliceMut`] that conveys no ownership, and so can be converted into a
mutable byte slice.

Some `ByteSliceMut` types (notably, the standard library's [`RefMut`] type)
convey ownership, and so they cannot soundly be moved by-value into a byte
slice type (`&mut [u8]`). Some methods in this crate's API (such as
[`Ref::into_mut`]) are only compatible with `ByteSliceMut` types without
these ownership semantics.

[`RefMut`]: core::cell::RefMut

```rust
pub unsafe trait IntoByteSliceMut<''a>: IntoByteSlice<''a> + ByteSliceMut {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `into_byte_slice_mut`: Coverts `self` into a `&mut [u8]`.

##### Implementations

This trait is implemented for the following types:

- `&''a mut [u8]` with <''a>

## Module `byteorder`

Byte order-aware numeric primitives.

This module contains equivalents of the native multi-byte integer types with
no alignment requirement and supporting byte order conversions.

For each native multi-byte integer type - `u16`, `i16`, `u32`, etc - and
floating point type - `f32` and `f64` - an equivalent type is defined by
this module - [`U16`], [`I16`], [`U32`], [`F32`], [`F64`], etc. Unlike their
native counterparts, these types have alignment 1, and take a type parameter
specifying the byte order in which the bytes are stored in memory. Each type
implements this crate's relevant conversion and marker traits.

These two properties, taken together, make these types useful for defining
data structures whose memory layout matches a wire format such as that of a
network protocol or a file format. Such formats often have multi-byte values
at offsets that do not respect the alignment requirements of the equivalent
native types, and stored in a byte order not necessarily the same as that of
the target platform.

Type aliases are provided for common byte orders in the [`big_endian`],
[`little_endian`], [`network_endian`], and [`native_endian`] submodules.

# Example

One use of these types is for representing network packet formats, such as
UDP:

```rust
use zerocopy::{*, byteorder::network_endian::U16};
# use zerocopy_derive::*;

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
struct UdpHeader {
    src_port: U16,
    dst_port: U16,
    length: U16,
    checksum: U16,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct UdpPacket {
    header: UdpHeader,
    body: [u8],
}

impl UdpPacket {
    fn parse(bytes: &[u8]) -> Option<&UdpPacket> {
        UdpPacket::ref_from_bytes(bytes).ok()
    }
}
```

```rust
pub mod byteorder { /* ... */ }
```

### Modules

## Module `big_endian`

Numeric primitives stored in
big-endian
byte order.

```rust
pub mod big_endian { /* ... */ }
```

### Types

#### Type Alias `U16`

A
16-bit unsigned integer
stored in
big-endian
byte order.

```rust
pub type U16 = crate::byteorder::U16<super::BigEndian>;
```

#### Type Alias `U32`

A
32-bit unsigned integer
stored in
big-endian
byte order.

```rust
pub type U32 = crate::byteorder::U32<super::BigEndian>;
```

#### Type Alias `U64`

A
64-bit unsigned integer
stored in
big-endian
byte order.

```rust
pub type U64 = crate::byteorder::U64<super::BigEndian>;
```

#### Type Alias `U128`

A
128-bit unsigned integer
stored in
big-endian
byte order.

```rust
pub type U128 = crate::byteorder::U128<super::BigEndian>;
```

#### Type Alias `I16`

A
16-bit signed integer
stored in
big-endian
byte order.

```rust
pub type I16 = crate::byteorder::I16<super::BigEndian>;
```

#### Type Alias `I32`

A
32-bit signed integer
stored in
big-endian
byte order.

```rust
pub type I32 = crate::byteorder::I32<super::BigEndian>;
```

#### Type Alias `I64`

A
64-bit signed integer
stored in
big-endian
byte order.

```rust
pub type I64 = crate::byteorder::I64<super::BigEndian>;
```

#### Type Alias `I128`

A
128-bit signed integer
stored in
big-endian
byte order.

```rust
pub type I128 = crate::byteorder::I128<super::BigEndian>;
```

#### Type Alias `F32`

A
32-bit floating point number
stored in
big-endian
byte order.

```rust
pub type F32 = crate::byteorder::F32<super::BigEndian>;
```

#### Type Alias `F64`

A
64-bit floating point number
stored in
big-endian
byte order.

```rust
pub type F64 = crate::byteorder::F64<super::BigEndian>;
```

## Module `little_endian`

Numeric primitives stored in
little-endian
byte order.

```rust
pub mod little_endian { /* ... */ }
```

### Types

#### Type Alias `U16`

A
16-bit unsigned integer
stored in
little-endian
byte order.

```rust
pub type U16 = crate::byteorder::U16<super::LittleEndian>;
```

#### Type Alias `U32`

A
32-bit unsigned integer
stored in
little-endian
byte order.

```rust
pub type U32 = crate::byteorder::U32<super::LittleEndian>;
```

#### Type Alias `U64`

A
64-bit unsigned integer
stored in
little-endian
byte order.

```rust
pub type U64 = crate::byteorder::U64<super::LittleEndian>;
```

#### Type Alias `U128`

A
128-bit unsigned integer
stored in
little-endian
byte order.

```rust
pub type U128 = crate::byteorder::U128<super::LittleEndian>;
```

#### Type Alias `I16`

A
16-bit signed integer
stored in
little-endian
byte order.

```rust
pub type I16 = crate::byteorder::I16<super::LittleEndian>;
```

#### Type Alias `I32`

A
32-bit signed integer
stored in
little-endian
byte order.

```rust
pub type I32 = crate::byteorder::I32<super::LittleEndian>;
```

#### Type Alias `I64`

A
64-bit signed integer
stored in
little-endian
byte order.

```rust
pub type I64 = crate::byteorder::I64<super::LittleEndian>;
```

#### Type Alias `I128`

A
128-bit signed integer
stored in
little-endian
byte order.

```rust
pub type I128 = crate::byteorder::I128<super::LittleEndian>;
```

#### Type Alias `F32`

A
32-bit floating point number
stored in
little-endian
byte order.

```rust
pub type F32 = crate::byteorder::F32<super::LittleEndian>;
```

#### Type Alias `F64`

A
64-bit floating point number
stored in
little-endian
byte order.

```rust
pub type F64 = crate::byteorder::F64<super::LittleEndian>;
```

## Module `network_endian`

Numeric primitives stored in
network-endian
byte order.

```rust
pub mod network_endian { /* ... */ }
```

### Types

#### Type Alias `U16`

A
16-bit unsigned integer
stored in
network-endian
byte order.

```rust
pub type U16 = crate::byteorder::U16<super::NetworkEndian>;
```

#### Type Alias `U32`

A
32-bit unsigned integer
stored in
network-endian
byte order.

```rust
pub type U32 = crate::byteorder::U32<super::NetworkEndian>;
```

#### Type Alias `U64`

A
64-bit unsigned integer
stored in
network-endian
byte order.

```rust
pub type U64 = crate::byteorder::U64<super::NetworkEndian>;
```

#### Type Alias `U128`

A
128-bit unsigned integer
stored in
network-endian
byte order.

```rust
pub type U128 = crate::byteorder::U128<super::NetworkEndian>;
```

#### Type Alias `I16`

A
16-bit signed integer
stored in
network-endian
byte order.

```rust
pub type I16 = crate::byteorder::I16<super::NetworkEndian>;
```

#### Type Alias `I32`

A
32-bit signed integer
stored in
network-endian
byte order.

```rust
pub type I32 = crate::byteorder::I32<super::NetworkEndian>;
```

#### Type Alias `I64`

A
64-bit signed integer
stored in
network-endian
byte order.

```rust
pub type I64 = crate::byteorder::I64<super::NetworkEndian>;
```

#### Type Alias `I128`

A
128-bit signed integer
stored in
network-endian
byte order.

```rust
pub type I128 = crate::byteorder::I128<super::NetworkEndian>;
```

#### Type Alias `F32`

A
32-bit floating point number
stored in
network-endian
byte order.

```rust
pub type F32 = crate::byteorder::F32<super::NetworkEndian>;
```

#### Type Alias `F64`

A
64-bit floating point number
stored in
network-endian
byte order.

```rust
pub type F64 = crate::byteorder::F64<super::NetworkEndian>;
```

## Module `native_endian`

Numeric primitives stored in
native-endian
byte order.

```rust
pub mod native_endian { /* ... */ }
```

### Types

#### Type Alias `U16`

A
16-bit unsigned integer
stored in
native-endian
byte order.

```rust
pub type U16 = crate::byteorder::U16<super::NativeEndian>;
```

#### Type Alias `U32`

A
32-bit unsigned integer
stored in
native-endian
byte order.

```rust
pub type U32 = crate::byteorder::U32<super::NativeEndian>;
```

#### Type Alias `U64`

A
64-bit unsigned integer
stored in
native-endian
byte order.

```rust
pub type U64 = crate::byteorder::U64<super::NativeEndian>;
```

#### Type Alias `U128`

A
128-bit unsigned integer
stored in
native-endian
byte order.

```rust
pub type U128 = crate::byteorder::U128<super::NativeEndian>;
```

#### Type Alias `I16`

A
16-bit signed integer
stored in
native-endian
byte order.

```rust
pub type I16 = crate::byteorder::I16<super::NativeEndian>;
```

#### Type Alias `I32`

A
32-bit signed integer
stored in
native-endian
byte order.

```rust
pub type I32 = crate::byteorder::I32<super::NativeEndian>;
```

#### Type Alias `I64`

A
64-bit signed integer
stored in
native-endian
byte order.

```rust
pub type I64 = crate::byteorder::I64<super::NativeEndian>;
```

#### Type Alias `I128`

A
128-bit signed integer
stored in
native-endian
byte order.

```rust
pub type I128 = crate::byteorder::I128<super::NativeEndian>;
```

#### Type Alias `F32`

A
32-bit floating point number
stored in
native-endian
byte order.

```rust
pub type F32 = crate::byteorder::F32<super::NativeEndian>;
```

#### Type Alias `F64`

A
64-bit floating point number
stored in
native-endian
byte order.

```rust
pub type F64 = crate::byteorder::F64<super::NativeEndian>;
```

### Types

#### Enum `BigEndian`

Big-endian byte order.

See [`ByteOrder`] for more details.

```rust
pub enum BigEndian {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &BigEndian) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Eq**
- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, _: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &BigEndian) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BigEndian { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **Unpin**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ByteOrder**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BigEndian) -> bool { /* ... */ }
    ```

#### Enum `LittleEndian`

Little-endian byte order.

See [`ByteOrder`] for more details.

```rust
pub enum LittleEndian {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &LittleEndian) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LittleEndian) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LittleEndian { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ByteOrder**
- **Display**
  - ```rust
    fn fmt(self: &Self, _: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &LittleEndian) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **StructuralPartialEq**
#### Type Alias `NativeEndian`

**Attributes:**

- `#[<cfg>(target_endian = "little")]`

The endianness used by this platform.

This is a type alias for [`BigEndian`] or [`LittleEndian`] depending on the
endianness of the target platform.

```rust
pub type NativeEndian = LittleEndian;
```

#### Type Alias `NetworkEndian`

The endianness used in many network protocols.

This is a type alias for [`BigEndian`].

```rust
pub type NetworkEndian = BigEndian;
```

#### Type Alias `BE`

A type alias for [`BigEndian`].

```rust
pub type BE = BigEndian;
```

#### Type Alias `LE`

A type alias for [`LittleEndian`].

```rust
pub type LE = LittleEndian;
```

#### Struct `U16`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 16-bit unsigned integer stored in a given byte order.

`U16` is like the native `u16` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

A `U16` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`u16` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U16`
has endianness `O` and that, b) the layout of `u16` has
the platform's native endianness.

`U16` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::U16::new
[`get`]: crate::byteorder::U16::get
[`set`]: crate::byteorder::U16::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct U16<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 2]) -> U16<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 2] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: u16) -> U16<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> u16 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: u16) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Rem**
  - ```rust
    fn rem(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> U16<O> { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 2] { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Add**
  - ```rust
    fn add(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **KnownLayout**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **Unpin**
- **Sub**
  - ```rust
    fn sub(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **UnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Copy**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 2] { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: U16<O>) -> [u8; 2] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 2]) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> u16 { /* ... */ }
    ```

  - ```rust
    fn from(x: u16) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> u32 { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> u64 { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> u128 { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> usize { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> U32<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> U64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> U128<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U16<O>) -> Usize<P> { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **FromZeros**
- **RefUnwindSafe**
- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> U16<O> { /* ... */ }
    ```

- **TryFromBytes**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> U16<O> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &U16<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &U16<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 2]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **Immutable**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u16) -> Option<Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: u32) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: u64) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: u128) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: usize) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U32<P>) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U64<P>) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U128<P>) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: Usize<P>) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

- **Unaligned**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: U16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: u16) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

- **IntoBytes**
- **FromBytes**
- **Shl**
  - ```rust
    fn shl(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: U16<O>) -> U16<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: u16) -> U16<O> { /* ... */ }
    ```

#### Struct `U32`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 32-bit unsigned integer stored in a given byte order.

`U32` is like the native `u32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

A `U32` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`u32` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U32`
has endianness `O` and that, b) the layout of `u32` has
the platform's native endianness.

`U32` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::U32::new
[`get`]: crate::byteorder::U32::get
[`set`]: crate::byteorder::U32::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct U32<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 4]) -> U32<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 4] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: u32) -> U32<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> u32 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: u32) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **Div**
  - ```rust
    fn div(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> U32<O> { /* ... */ }
    ```

- **UnwindSafe**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 4] { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Mul**
  - ```rust
    fn mul(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U32<P>) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: u64) -> Result<U32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: u128) -> Result<U32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U64<P>) -> Result<U32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U128<P>) -> Result<U32<O>, TryFromIntError> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFromBytes**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> U32<O> { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Copy**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 4] { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **Freeze**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &U32<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &U32<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 4]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Immutable**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **KnownLayout**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: U16<O>) -> U32<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U32<O>) -> [u8; 4] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 4]) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: U32<O>) -> u32 { /* ... */ }
    ```

  - ```rust
    fn from(x: u32) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: U32<O>) -> u64 { /* ... */ }
    ```

  - ```rust
    fn from(x: U32<O>) -> u128 { /* ... */ }
    ```

  - ```rust
    fn from(x: U32<O>) -> U64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U32<O>) -> U128<P> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **Unaligned**
- **RefUnwindSafe**
- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Shl**
  - ```rust
    fn shl(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: U32<O>) -> U32<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: u32) -> U32<O> { /* ... */ }
    ```

- **IntoBytes**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u32) -> Option<Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: U32<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: u32) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> U32<O> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **FromZeros**
- **FromBytes**
#### Struct `U64`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 64-bit unsigned integer stored in a given byte order.

`U64` is like the native `u64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

A `U64` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`u64` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U64`
has endianness `O` and that, b) the layout of `u64` has
the platform's native endianness.

`U64` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::U64::new
[`get`]: crate::byteorder::U64::get
[`set`]: crate::byteorder::U64::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct U64<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 8]) -> U64<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 8] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: u64) -> U64<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> u64 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: u64) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **Unaligned**
- **Not**
  - ```rust
    fn not(self: Self) -> U64<O> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **FromZeros**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &U64<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &U64<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 8]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> U64<O> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: U16<O>) -> U64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U32<O>) -> U64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U64<O>) -> [u8; 8] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 8]) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: U64<O>) -> u64 { /* ... */ }
    ```

  - ```rust
    fn from(x: u64) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: U64<O>) -> u128 { /* ... */ }
    ```

  - ```rust
    fn from(x: U64<O>) -> U128<P> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> U64<O> { /* ... */ }
    ```

- **Immutable**
- **KnownLayout**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **StructuralPartialEq**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Add**
  - ```rust
    fn add(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **IntoBytes**
- **Unpin**
- **FromBytes**
- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u64) -> Option<Ordering> { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 8] { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **TryFromBytes**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U64<P>) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U64<P>) -> Result<U32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: u128) -> Result<U64<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U128<P>) -> Result<U64<O>, TryFromIntError> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: U64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: u64) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: U64<O>) -> U64<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: u64) -> U64<O> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

#### Struct `U128`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 128-bit unsigned integer stored in a given byte order.

`U128` is like the native `u128` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

A `U128` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`u128` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U128`
has endianness `O` and that, b) the layout of `u128` has
the platform's native endianness.

`U128` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::U128::new
[`get`]: crate::byteorder::U128::get
[`set`]: crate::byteorder::U128::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct U128<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 16]) -> U128<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 16] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: u128) -> U128<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> u128 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: u128) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Copy**
- **Default**
  - ```rust
    fn default() -> U128<O> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Shr**
  - ```rust
    fn shr(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 16] { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **Eq**
- **Immutable**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 16] { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unaligned**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U128<P>) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U128<P>) -> Result<U32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: U128<P>) -> Result<U64<O>, TryFromIntError> { /* ... */ }
    ```

- **TryFromBytes**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> U128<O> { /* ... */ }
    ```

- **Freeze**
- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **Unpin**
- **KnownLayout**
- **FromBytes**
- **Rem**
  - ```rust
    fn rem(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **FromZeros**
- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **Sync**
- **Not**
  - ```rust
    fn not(self: Self) -> U128<O> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: U16<O>) -> U128<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U32<O>) -> U128<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U64<O>) -> U128<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: U128<O>) -> [u8; 16] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 16]) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: U128<O>) -> u128 { /* ... */ }
    ```

  - ```rust
    fn from(x: u128) -> U128<O> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u128) -> Option<Ordering> { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **IntoBytes**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &U128<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &U128<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 16]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u128) -> bool { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: U128<O>) -> U128<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: u128) -> U128<O> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: U128<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: u128) { /* ... */ }
    ```

#### Struct `Usize`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A word-sized unsigned integer stored in a given byte order.

`Usize` is like the native `usize` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

A `Usize` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`usize` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `Usize`
has endianness `O` and that, b) the layout of `usize` has
the platform's native endianness.

`Usize` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::Usize::new
[`get`]: crate::byteorder::Usize::get
[`set`]: crate::byteorder::Usize::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct Usize<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 8]) -> Usize<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 8] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: usize) -> Usize<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> usize { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: usize) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 8] { /* ... */ }
    ```

- **FromZeros**
- **Div**
  - ```rust
    fn div(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Usize<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Usize<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 8]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &usize) -> bool { /* ... */ }
    ```

- **Immutable**
- **UnwindSafe**
- **Shr**
  - ```rust
    fn shr(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoBytes**
- **Default**
  - ```rust
    fn default() -> Usize<O> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: U16<O>) -> Usize<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: Usize<O>) -> [u8; 8] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 8]) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: Usize<O>) -> usize { /* ... */ }
    ```

  - ```rust
    fn from(x: usize) -> Usize<O> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unaligned**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: Usize<P>) -> Result<U16<O>, TryFromIntError> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Not**
  - ```rust
    fn not(self: Self) -> Usize<O> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **TryFromBytes**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 8] { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Sync**
- **Add**
  - ```rust
    fn add(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **KnownLayout**
- **FromBytes**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Usize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: usize) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: Usize<O>) -> Usize<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: usize) -> Usize<O> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Usize<O> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &usize) -> Option<Ordering> { /* ... */ }
    ```

#### Struct `I16`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 16-bit signed integer stored in a given byte order.

`I16` is like the native `i16` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

An `I16` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`i16` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I16`
has endianness `O` and that, b) the layout of `i16` has
the platform's native endianness.

`I16` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::I16::new
[`get`]: crate::byteorder::I16::get
[`set`]: crate::byteorder::I16::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct I16<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 2]) -> I16<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 2] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: i16) -> I16<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> i16 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: i16) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **IntoBytes**
- **Freeze**
- **Not**
  - ```rust
    fn not(self: Self) -> I16<O> { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> I16<O> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &I16<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &I16<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 2]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **FromBytes**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 2] { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i16) -> Option<Ordering> { /* ... */ }
    ```

- **TryFromBytes**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **KnownLayout**
- **Default**
  - ```rust
    fn default() -> I16<O> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **Unaligned**
- **FromZeros**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: i32) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: i64) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: i128) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: isize) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I32<P>) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I64<P>) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I128<P>) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: Isize<P>) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Immutable**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> I16<O> { /* ... */ }
    ```

- **StructuralPartialEq**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 2] { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: I16<O>) -> [u8; 2] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 2]) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> i16 { /* ... */ }
    ```

  - ```rust
    fn from(x: i16) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> i32 { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> i64 { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> i128 { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> isize { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> I32<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> I64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> I128<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I16<O>) -> Isize<P> { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: I16<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: i16) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: I16<O>) -> I16<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: i16) -> I16<O> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `I32`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 32-bit signed integer stored in a given byte order.

`I32` is like the native `i32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

An `I32` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`i32` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I32`
has endianness `O` and that, b) the layout of `i32` has
the platform's native endianness.

`I32` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::I32::new
[`get`]: crate::byteorder::I32::get
[`set`]: crate::byteorder::I32::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct I32<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 4]) -> I32<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 4] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: i32) -> I32<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> i32 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: i32) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 4] { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **FromBytes**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> I32<O> { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I32<P>) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: i64) -> Result<I32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: i128) -> Result<I32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I64<P>) -> Result<I32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I128<P>) -> Result<I32<O>, TryFromIntError> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> I32<O> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> I32<O> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> I32<O> { /* ... */ }
    ```

- **Immutable**
- **IntoBytes**
- **KnownLayout**
- **FromZeros**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: I16<O>) -> I32<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I32<O>) -> [u8; 4] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 4]) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: I32<O>) -> i32 { /* ... */ }
    ```

  - ```rust
    fn from(x: i32) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: I32<O>) -> i64 { /* ... */ }
    ```

  - ```rust
    fn from(x: I32<O>) -> i128 { /* ... */ }
    ```

  - ```rust
    fn from(x: I32<O>) -> I64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I32<O>) -> I128<P> { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **Unpin**
- **Mul**
  - ```rust
    fn mul(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &I32<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &I32<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 4]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32) -> bool { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unaligned**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 4] { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: I32<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: i32) { /* ... */ }
    ```

- **TryFromBytes**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: I32<O>) -> I32<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: i32) -> I32<O> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i32) -> Option<Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `I64`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 64-bit signed integer stored in a given byte order.

`I64` is like the native `i64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

An `I64` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`i64` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I64`
has endianness `O` and that, b) the layout of `i64` has
the platform's native endianness.

`I64` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::I64::new
[`get`]: crate::byteorder::I64::get
[`set`]: crate::byteorder::I64::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct I64<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 8]) -> I64<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 8] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: i64) -> I64<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> i64 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: i64) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Sub**
  - ```rust
    fn sub(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> I64<O> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryFromBytes**
- **Div**
  - ```rust
    fn div(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> I64<O> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> I64<O> { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> I64<O> { /* ... */ }
    ```

- **Freeze**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **IntoBytes**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 8] { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **Immutable**
- **Add**
  - ```rust
    fn add(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **Unaligned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **FromBytes**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **Eq**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 8] { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i64) -> Option<Ordering> { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: I64<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: i64) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I64<P>) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I64<P>) -> Result<I32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: i128) -> Result<I64<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I128<P>) -> Result<I64<O>, TryFromIntError> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

- **Copy**
- **FromZeros**
- **KnownLayout**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &I64<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &I64<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 8]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: I16<O>) -> I64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I32<O>) -> I64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I64<O>) -> [u8; 8] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 8]) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: I64<O>) -> i64 { /* ... */ }
    ```

  - ```rust
    fn from(x: i64) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: I64<O>) -> i128 { /* ... */ }
    ```

  - ```rust
    fn from(x: I64<O>) -> I128<P> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: I64<O>) -> I64<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: i64) -> I64<O> { /* ... */ }
    ```

#### Struct `I128`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 128-bit signed integer stored in a given byte order.

`I128` is like the native `i128` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

An `I128` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`i128` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I128`
has endianness `O` and that, b) the layout of `i128` has
the platform's native endianness.

`I128` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::I128::new
[`get`]: crate::byteorder::I128::get
[`set`]: crate::byteorder::I128::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct I128<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 16]) -> I128<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 16] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: i128) -> I128<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> i128 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: i128) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **IntoBytes**
- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **KnownLayout**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> I128<O> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Eq**
- **Shr**
  - ```rust
    fn shr(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: I16<O>) -> I128<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I32<O>) -> I128<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I64<O>) -> I128<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: I128<O>) -> [u8; 16] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 16]) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: I128<O>) -> i128 { /* ... */ }
    ```

  - ```rust
    fn from(x: i128) -> I128<O> { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **FromZeros**
- **Copy**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 16] { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> I128<O> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **UnwindSafe**
- **Unaligned**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I128<P>) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I128<P>) -> Result<I32<O>, TryFromIntError> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: I128<P>) -> Result<I64<O>, TryFromIntError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> I128<O> { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &I128<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &I128<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 16]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128) -> bool { /* ... */ }
    ```

- **Immutable**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 16] { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> I128<O> { /* ... */ }
    ```

- **Unpin**
- **Div**
  - ```rust
    fn div(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **FromBytes**
- **RefUnwindSafe**
- **TryFromBytes**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: I128<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: i128) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: I128<O>) -> I128<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: i128) -> I128<O> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i128) -> Option<Ordering> { /* ... */ }
    ```

#### Struct `Isize`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A word-sized signed integer stored in a given byte order.

`Isize` is like the native `isize` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

An `Isize` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`isize` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `Isize`
has endianness `O` and that, b) the layout of `isize` has
the platform's native endianness.

`Isize` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::Isize::new
[`get`]: crate::byteorder::Isize::get
[`set`]: crate::byteorder::Isize::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct Isize<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 8]) -> Isize<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 8] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: isize) -> Isize<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> isize { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: isize) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **Unaligned**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 8] { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **FromBytes**
- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Isize<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Isize<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 8]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &isize) -> bool { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **Immutable**
- **FromZeros**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryFromBytes**
- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 8] { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: I16<O>) -> Isize<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: Isize<O>) -> [u8; 8] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 8]) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: Isize<O>) -> isize { /* ... */ }
    ```

  - ```rust
    fn from(x: isize) -> Isize<O> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Isize<O> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(x: Isize<P>) -> Result<I16<O>, TryFromIntError> { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> Isize<O> { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **KnownLayout**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Isize<O> { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Isize<O> { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **Copy**
- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **IntoBytes**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &isize) -> Option<Ordering> { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Isize<O>) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: isize) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: Isize<O>) -> Isize<O> { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, rhs: isize) -> Isize<O> { /* ... */ }
    ```

#### Struct `F32`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 32-bit floating point number stored in a given byte order.

`F32` is like the native `f32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

An `F32` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`f32` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `F32`
has endianness `O` and that, b) the layout of `f32` has
the platform's native endianness.

`F32` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::F32::new
[`get`]: crate::byteorder::F32::get
[`set`]: crate::byteorder::F32::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct F32<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 4]) -> F32<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 4] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: f32) -> F32<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> f32 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: f32) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **Unaligned**
- **Div**
  - ```rust
    fn div(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: f32) -> F32<O> { /* ... */ }
    ```

- **Freeze**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: f32) { /* ... */ }
    ```

- **Eq**
- **Add**
  - ```rust
    fn add(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: f32) -> F32<O> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Immutable**
- **Mul**
  - ```rust
    fn mul(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: f32) -> F32<O> { /* ... */ }
    ```

- **Copy**
- **Send**
- **Sync**
- **UnwindSafe**
- **TryFromBytes**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 4] { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: f32) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> F32<O> { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: f32) -> F32<O> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 4] { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **KnownLayout**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: F32<O>) -> [u8; 4] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 4]) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: F32<O>) -> f32 { /* ... */ }
    ```

  - ```rust
    fn from(x: f32) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: F32<O>) -> f64 { /* ... */ }
    ```

  - ```rust
    fn from(x: F32<O>) -> F64<P> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &F32<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &F32<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 4]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> F32<O> { /* ... */ }
    ```

- **FromBytes**
- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: f32) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: f32) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **FromZeros**
- **IntoBytes**
- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: F32<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: f32) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> F32<O> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: F32<O>) -> F32<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: f32) -> F32<O> { /* ... */ }
    ```

#### Struct `F64`

**Attributes:**

- `#[<cfg_attr>(any(feature = "derive", test),
derive(KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned))]`

A 64-bit floating point number stored in a given byte order.

`F64` is like the native `f64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`]. In particular, this refers
to [`BigEndian`], [`LittleEndian`], [`NativeEndian`], and [`NetworkEndian`].

An `F64` can be constructed using
the [`new`] method, and its contained value can be obtained as a native
`f64` using the [`get`] method, or updated in place with
the [`set`] method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `F64`
has endianness `O` and that, b) the layout of `f64` has
the platform's native endianness.

`F64` implements [`FromBytes`], [`IntoBytes`], and [`Unaligned`],
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.

[`new`]: crate::byteorder::F64::new
[`get`]: crate::byteorder::F64::get
[`set`]: crate::byteorder::F64::set
[`FromBytes`]: crate::FromBytes
[`IntoBytes`]: crate::IntoBytes
[`Unaligned`]: crate::Unaligned

```rust
pub struct F64<O>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_bytes(bytes: [u8; 8]) -> F64<O> { /* ... */ }
  ```
  Constructs a new value from bytes which are already in `O` byte

- ```rust
  pub const fn to_bytes(self: Self) -> [u8; 8] { /* ... */ }
  ```
  Extracts the bytes of `self` without swapping the byte order.

- ```rust
  pub const fn new(n: f64) -> F64<O> { /* ... */ }
  ```
  Constructs a new value, possibly performing an endianness

- ```rust
  pub const fn get(self: Self) -> f64 { /* ... */ }
  ```
  Returns the value as a primitive type, possibly performing

- ```rust
  pub fn set(self: &mut Self, n: f64) { /* ... */ }
  ```
  Updates the value in place as a primitive type, possibly

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> F64<O> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromBytes**
- **Copy**
- **StructuralPartialEq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, rhs: f64) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> F64<O> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, rhs: f64) -> F64<O> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> F64<O> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: F32<O>) -> F64<P> { /* ... */ }
    ```

  - ```rust
    fn from(x: F64<O>) -> [u8; 8] { /* ... */ }
    ```

  - ```rust
    fn from(bytes: [u8; 8]) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn from(x: F64<O>) -> f64 { /* ... */ }
    ```

  - ```rust
    fn from(x: f64) -> F64<O> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &F64<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &F64<O>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8; 8]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64) -> bool { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [u8; 8] { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, rhs: f64) { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, rhs: f64) { /* ... */ }
    ```

- **IntoBytes**
- **RefUnwindSafe**
- **Immutable**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 8] { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, rhs: f64) -> F64<O> { /* ... */ }
    ```

- **FromZeros**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, rhs: f64) { /* ... */ }
    ```

- **KnownLayout**
- **Sync**
- **TryFromBytes**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Div**
  - ```rust
    fn div(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, rhs: f64) -> F64<O> { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: F64<O>) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, rhs: f64) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, rhs: f64) -> F64<O> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: F64<O>) -> F64<O> { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, rhs: f64) -> F64<O> { /* ... */ }
    ```

- **Unaligned**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

### Traits

#### Trait `ByteOrder`

A type-level representation of byte order.

This type is implemented by [`BigEndian`] and [`LittleEndian`], which
represent big-endian and little-endian byte order respectively. This module
also provides a number of useful aliases for those types: [`NativeEndian`],
[`NetworkEndian`], [`BE`], and [`LE`].

`ByteOrder` types can be used to specify the byte order of the types in this
module - for example, [`U32<BigEndian>`] is a 32-bit integer stored in
big-endian byte order.

[`U32<BigEndian>`]: U32

```rust
pub trait ByteOrder: Copy + Clone + Debug + Display + Eq + PartialEq + Ord + PartialOrd + Hash + private::Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `BigEndian`
- `LittleEndian`

## Module `error`

Types related to error reporting.

## Single failure mode errors

Generally speaking, zerocopy's conversions may fail for one of up to three
reasons:
- [`AlignmentError`]: the conversion source was improperly aligned
- [`SizeError`]: the conversion source was of incorrect size
- [`ValidityError`]: the conversion source contained invalid data

Methods that only have one failure mode, like
[`FromBytes::read_from_bytes`], return that mode's corresponding error type
directly.

## Compound errors

Conversion methods that have either two or three possible failure modes
return one of these error types:
- [`CastError`]: the error type of reference conversions
- [`TryCastError`]: the error type of fallible reference conversions
- [`TryReadError`]: the error type of fallible read conversions

## [`Unaligned`] destination types

For [`Unaligned`] destination types, alignment errors are impossible. All
compound error types support infallibly discarding the alignment error via
[`From`] so long as `Dst: Unaligned`. For example, see [`<SizeError as
From<ConvertError>>::from`][size-error-from].

[size-error-from]: struct.SizeError.html#method.from-1

## Accessing the conversion source

All error types provide an `into_src` method that converts the error into
the source value underlying the failed conversion.

## Display formatting

All error types provide a `Display` implementation that produces a
human-readable error message. When `debug_assertions` are enabled, these
error messages are verbose and may include potentially sensitive
information, including:

- the names of the involved types
- the sizes of the involved types
- the addresses of the involved types
- the contents of the involved types

When `debug_assertions` are disabled (as is default for `release` builds),
such potentially sensitive information is excluded.

In the future, we may support manually configuring this behavior. If you are
interested in this feature, [let us know on GitHub][issue-1457] so we know
to prioritize it.

[issue-1457]: https://github.com/google/zerocopy/issues/1457

## Validation order

Our conversion methods typically check alignment, then size, then bit
validity. However, we do not guarantee that this is always the case, and
this behavior may change between releases.

## `Send`, `Sync`, and `'static`

Our error types are `Send`, `Sync`, and `'static` when their `Src` parameter
is `Send`, `Sync`, or `'static`, respectively. This can cause issues when an
error is sent or synchronized across threads; e.g.:

```compile_fail,E0515
use zerocopy::*;

let result: SizeError<&[u8], u32> = std::thread::spawn(|| {
    let source = &mut [0u8, 1, 2][..];
    // Try (and fail) to read a `u32` from `source`.
    u32::read_from_bytes(source).unwrap_err()
}).join().unwrap();
```

To work around this, use [`map_src`][CastError::map_src] to convert the
source parameter to an unproblematic type; e.g.:

```
use zerocopy::*;

let result: SizeError<(), u32> = std::thread::spawn(|| {
    let source = &mut [0u8, 1, 2][..];
    // Try (and fail) to read a `u32` from `source`.
    u32::read_from_bytes(source).unwrap_err()
        // Erase the error source.
        .map_src(drop)
}).join().unwrap();
```

Alternatively, use `.to_string()` to eagerly convert the error into a
human-readable message; e.g.:

```
use zerocopy::*;

let result: Result<u32, String> = std::thread::spawn(|| {
    let source = &mut [0u8, 1, 2][..];
    // Try (and fail) to read a `u32` from `source`.
    u32::read_from_bytes(source)
        // Eagerly render the error message.
        .map_err(|err| err.to_string())
}).join().unwrap();
```

```rust
pub mod error { /* ... */ }
```

### Types

#### Enum `ConvertError`

Zerocopy's generic error type.

Generally speaking, zerocopy's conversions may fail for one of up to three
reasons:
- [`AlignmentError`]: the conversion source was improperly aligned
- [`SizeError`]: the conversion source was of incorrect size
- [`ValidityError`]: the conversion source contained invalid data

However, not all conversions produce all errors. For instance,
[`FromBytes::ref_from_bytes`] may fail due to alignment or size issues, but
not validity issues. This generic error type captures these
(im)possibilities via parameterization: `A` is parameterized with
[`AlignmentError`], `S` is parameterized with [`SizeError`], and `V` is
parameterized with [`Infallible`].

Zerocopy never uses this type directly in its API. Rather, we provide three
pre-parameterized aliases:
- [`CastError`]: the error type of reference conversions
- [`TryCastError`]: the error type of fallible reference conversions
- [`TryReadError`]: the error type of fallible read conversions

```rust
pub enum ConvertError<A, S, V> {
    Alignment(A),
    Size(S),
    Validity(V),
}
```

##### Variants

###### `Alignment`

The conversion source was improperly aligned.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `A` |  |

###### `Size`

The conversion source was of incorrect size.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `S` |  |

###### `Validity`

The conversion source contained invalid data.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `V` |  |

##### Implementations

###### Methods

- ```rust
  pub fn into_src(self: Self) -> Src { /* ... */ }
  ```
  Produces the source underlying the failed conversion.

- ```rust
  pub fn map_src<NewSrc, /* synthetic */ impl FnOnce(Src) -> NewSrc: FnOnce(Src) -> NewSrc>(self: Self, f: impl FnOnce(Src) -> NewSrc) -> CastError<NewSrc, Dst> { /* ... */ }
  ```
  Maps the source value associated with the conversion error.

- ```rust
  pub fn into_src(self: Self) -> Src { /* ... */ }
  ```
  Produces the source underlying the failed conversion.

- ```rust
  pub fn map_src<NewSrc, /* synthetic */ impl FnOnce(Src) -> NewSrc: FnOnce(Src) -> NewSrc>(self: Self, f: impl FnOnce(Src) -> NewSrc) -> TryCastError<NewSrc, Dst> { /* ... */ }
  ```
  Maps the source value associated with the conversion error.

- ```rust
  pub fn into_src(self: Self) -> Src { /* ... */ }
  ```
  Produces the source underlying the failed conversion.

- ```rust
  pub fn map_src<NewSrc, /* synthetic */ impl FnOnce(Src) -> NewSrc: FnOnce(Src) -> NewSrc>(self: Self, f: impl FnOnce(Src) -> NewSrc) -> TryReadError<NewSrc, Dst> { /* ... */ }
  ```
  Maps the source value associated with the conversion error.

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: ConvertError<AlignmentError<Src, Dst>, S, V>) -> ConvertError<Infallible, S, V> { /* ... */ }
    ```
    Infallibly discards the alignment error from this `ConvertError` since

  - ```rust
    fn from(err: AlignmentError<Src, Dst>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: SizeError<Src, Dst>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: ValidityError<Src, Dst>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: CastError<Src, Dst>) -> SizeError<Src, Dst> { /* ... */ }
    ```
    Infallibly extracts the [`SizeError`] from this `CastError` since `Dst`

  - ```rust
    fn from(value: CastError<Src, Dst>) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Error**
- **Send**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ConvertError<A, S, V>) -> bool { /* ... */ }
    ```

- **Unpin**
#### Struct `AlignmentError`

The error emitted if the conversion source is improperly aligned.

```rust
pub struct AlignmentError<Src, Dst: ?Sized> {
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
  pub fn into_src(self: Self) -> Src { /* ... */ }
  ```
  Produces the source underlying the failed conversion.

- ```rust
  pub fn map_src<NewSrc, /* synthetic */ impl FnOnce(Src) -> NewSrc: FnOnce(Src) -> NewSrc>(self: Self, f: impl FnOnce(Src) -> NewSrc) -> AlignmentError<NewSrc, Dst> { /* ... */ }
  ```
  Maps the source value associated with the conversion error.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Error**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(_: AlignmentError<Src, Dst>) -> Infallible { /* ... */ }
    ```

  - ```rust
    fn from(err: AlignmentError<Src, Dst>) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AlignmentError<Src, Dst>) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `SizeError`

The error emitted if the conversion source is of incorrect size.

```rust
pub struct SizeError<Src, Dst: ?Sized> {
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
  pub fn into_src(self: Self) -> Src { /* ... */ }
  ```
  Produces the source underlying the failed conversion.

- ```rust
  pub fn map_src<NewSrc, /* synthetic */ impl FnOnce(Src) -> NewSrc: FnOnce(Src) -> NewSrc>(self: Self, f: impl FnOnce(Src) -> NewSrc) -> SizeError<NewSrc, Dst> { /* ... */ }
  ```
  Maps the source value associated with the conversion error.

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SizeError<Src, Dst>) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Error**
- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
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

  - ```rust
    fn from(err: SizeError<Src, Dst>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: CastError<Src, Dst>) -> SizeError<Src, Dst> { /* ... */ }
    ```
    Infallibly extracts the [`SizeError`] from this `CastError` since `Dst`

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ValidityError`

The error emitted if the conversion source contains invalid data.

```rust
pub struct ValidityError<Src, Dst: ?Sized + TryFromBytes> {
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
  pub fn into_src(self: Self) -> Src { /* ... */ }
  ```
  Produces the source underlying the failed conversion.

- ```rust
  pub fn map_src<NewSrc, /* synthetic */ impl FnOnce(Src) -> NewSrc: FnOnce(Src) -> NewSrc>(self: Self, f: impl FnOnce(Src) -> NewSrc) -> ValidityError<NewSrc, Dst> { /* ... */ }
  ```
  Maps the source value associated with the conversion error.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: ValidityError<Src, Dst>) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ValidityError<Src, Dst>) -> bool { /* ... */ }
    ```

#### Type Alias `CastError`

**Attributes:**

- `#[allow(type_alias_bounds)]`

The error type of reference conversions.

Reference conversions, like [`FromBytes::ref_from_bytes`] may emit
[alignment](AlignmentError) and [size](SizeError) errors.

```rust
pub type CastError<Src, Dst: ?Sized> = ConvertError<AlignmentError<Src, Dst>, SizeError<Src, Dst>, core::convert::Infallible>;
```

#### Type Alias `TryCastError`

**Attributes:**

- `#[allow(type_alias_bounds)]`

The error type of fallible reference conversions.

Fallible reference conversions, like [`TryFromBytes::try_ref_from_bytes`]
may emit [alignment](AlignmentError), [size](SizeError), and
[validity](ValidityError) errors.

```rust
pub type TryCastError<Src, Dst: ?Sized + TryFromBytes> = ConvertError<AlignmentError<Src, Dst>, SizeError<Src, Dst>, ValidityError<Src, Dst>>;
```

#### Type Alias `TryReadError`

**Attributes:**

- `#[allow(type_alias_bounds)]`

The error type of fallible read-conversions.

Fallible read-conversions, like [`TryFromBytes::try_read_from_bytes`] may emit
[size](SizeError) and [validity](ValidityError) errors, but not alignment errors.

```rust
pub type TryReadError<Src, Dst: ?Sized + TryFromBytes> = ConvertError<core::convert::Infallible, SizeError<Src, Dst>, ValidityError<Src, Dst>>;
```

#### Type Alias `AlignedTryCastError`

**Attributes:**

- `#[allow(type_alias_bounds)]`

The error type of well-aligned, fallible casts.

This is like [`TryCastError`], but for casts that are always well-aligned.
It is identical to `TryCastError`, except that its alignment error is
[`Infallible`].

As of this writing, none of zerocopy's API produces this error directly.
However, it is useful since it permits users to infallibly discard alignment
errors when they can prove statically that alignment errors are impossible.

# Examples

```
use core::convert::Infallible;
use zerocopy::*;
# use zerocopy_derive::*;

#[derive(TryFromBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C, packed)]
struct Bools {
    one: bool,
    two: bool,
    many: [bool],
}

impl Bools {
    fn parse(bytes: &[u8]) -> Result<&Bools, AlignedTryCastError<&[u8], Bools>> {
        // Since `Bools: Unaligned`, we can infallibly discard
        // the alignment error.
        Bools::try_ref_from_bytes(bytes).map_err(Into::into)
    }
}
```

```rust
pub type AlignedTryCastError<Src, Dst: ?Sized + TryFromBytes> = ConvertError<core::convert::Infallible, SizeError<Src, Dst>, ValidityError<Src, Dst>>;
```

#### Struct `AllocError`

The error type of a failed allocation.

This type is intended to be deprecated in favor of the standard library's
[`AllocError`] type once it is stabilized. When that happens, this type will
be replaced by a type alias to the standard library type. We do not intend
to treat this as a breaking change; users who wish to avoid breakage should
avoid writing code which assumes that this is *not* such an alias. For
example, implementing the same trait for both types will result in an impl
conflict once this type is an alias.

[`AllocError`]: https://doc.rust-lang.org/alloc/alloc/struct.AllocError.html

```rust
pub struct AllocError;
```

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AllocError { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AllocError) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
## Traits

### Trait `KnownLayout`

**Attributes:**

- `#[<cfg_attr>(feature = "derive", doc =
"[derive]: zerocopy_derive::KnownLayout")]`
- `#[<cfg_attr>(not(feature = "derive"), doc =
concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"),
"/zerocopy/derive.KnownLayout.html"),)]`
- `#[<cfg_attr>(zerocopy_diagnostic_on_unimplemented_1_78_0,
diagnostic::on_unimplemented(note =
"Consider adding `#[derive(KnownLayout)]` to `{Self}`"))]`
- `#[diagnostic::on_unimplemented(note =
"Consider adding `#[derive(KnownLayout)]` to `{Self}`")]`

Indicates that zerocopy can reason about certain aspects of a type's layout.

This trait is required by many of zerocopy's APIs. It supports sized types,
slices, and [slice DSTs](#dynamically-sized-types).

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(KnownLayout)]`][derive]; e.g.:

```
# use zerocopy_derive::KnownLayout;
#[derive(KnownLayout)]
struct MyStruct {
# /*
    ...
# */
}

#[derive(KnownLayout)]
enum MyEnum {
# /*
    ...
# */
}

#[derive(KnownLayout)]
union MyUnion {
#   variant: u8,
# /*
    ...
# */
}
```

This derive performs a sophisticated analysis to deduce the layout
characteristics of types. You **must** implement this trait via the derive.

# Dynamically-sized types

`KnownLayout` supports slice-based dynamically sized types ("slice DSTs").

A slice DST is a type whose trailing field is either a slice or another
slice DST, rather than a type with fixed size. For example:

```
#[repr(C)]
struct PacketHeader {
# /*
    ...
# */
}

#[repr(C)]
struct Packet {
    header: PacketHeader,
    body: [u8],
}
```

It can be useful to think of slice DSTs as a generalization of slices - in
other words, a normal slice is just the special case of a slice DST with
zero leading fields. In particular:
- Like slices, slice DSTs can have different lengths at runtime
- Like slices, slice DSTs cannot be passed by-value, but only by reference
  or via other indirection such as `Box`
- Like slices, a reference (or `Box`, or other pointer type) to a slice DST
  encodes the number of elements in the trailing slice field

## Slice DST layout

Just like other composite Rust types, the layout of a slice DST is not
well-defined unless it is specified using an explicit `#[repr(...)]`
attribute such as `#[repr(C)]`. [Other representations are
supported][reprs], but in this section, we'll use `#[repr(C)]` as our
example.

A `#[repr(C)]` slice DST is laid out [just like sized `#[repr(C)]`
types][repr-c-structs], but the presenence of a variable-length field
introduces the possibility of *dynamic padding*. In particular, it may be
necessary to add trailing padding *after* the trailing slice field in order
to satisfy the outer type's alignment, and the amount of padding required
may be a function of the length of the trailing slice field. This is just a
natural consequence of the normal `#[repr(C)]` rules applied to slice DSTs,
but it can result in surprising behavior. For example, consider the
following type:

```
#[repr(C)]
struct Foo {
    a: u32,
    b: u8,
    z: [u16],
}
```

Assuming that `u32` has alignment 4 (this is not true on all platforms),
then `Foo` has alignment 4 as well. Here is the smallest possible value for
`Foo`:

```text
byte offset | 01234567
      field | aaaab---
                   ><
```

In this value, `z` has length 0. Abiding by `#[repr(C)]`, the lowest offset
that we can place `z` at is 5, but since `z` has alignment 2, we need to
round up to offset 6. This means that there is one byte of padding between
`b` and `z`, then 0 bytes of `z` itself (denoted `><` in this diagram), and
then two bytes of padding after `z` in order to satisfy the overall
alignment of `Foo`. The size of this instance is 8 bytes.

What about if `z` has length 1?

```text
byte offset | 01234567
      field | aaaab-zz
```

In this instance, `z` has length 1, and thus takes up 2 bytes. That means
that we no longer need padding after `z` in order to satisfy `Foo`'s
alignment. We've now seen two different values of `Foo` with two different
lengths of `z`, but they both have the same size - 8 bytes.

What about if `z` has length 2?

```text
byte offset | 012345678901
      field | aaaab-zzzz--
```

Now `z` has length 2, and thus takes up 4 bytes. This brings our un-padded
size to 10, and so we now need another 2 bytes of padding after `z` to
satisfy `Foo`'s alignment.

Again, all of this is just a logical consequence of the `#[repr(C)]` rules
applied to slice DSTs, but it can be surprising that the amount of trailing
padding becomes a function of the trailing slice field's length, and thus
can only be computed at runtime.

[reprs]: https://doc.rust-lang.org/reference/type-layout.html#representations
[repr-c-structs]: https://doc.rust-lang.org/reference/type-layout.html#reprc-structs

## What is a valid size?

There are two places in zerocopy's API that we refer to "a valid size" of a
type. In normal casts or conversions, where the source is a byte slice, we
need to know whether the source byte slice is a valid size of the
destination type. In prefix or suffix casts, we need to know whether *there
exists* a valid size of the destination type which fits in the source byte
slice and, if so, what the largest such size is.

As outlined above, a slice DST's size is defined by the number of elements
in its trailing slice field. However, there is not necessarily a 1-to-1
mapping between trailing slice field length and overall size. As we saw in
the previous section with the type `Foo`, instances with both 0 and 1
elements in the trailing `z` field result in a `Foo` whose size is 8 bytes.

When we say "x is a valid size of `T`", we mean one of two things:
- If `T: Sized`, then we mean that `x == size_of::<T>()`
- If `T` is a slice DST, then we mean that there exists a `len` such that the instance of
  `T` with `len` trailing slice elements has size `x`

When we say "largest possible size of `T` that fits in a byte slice", we
mean one of two things:
- If `T: Sized`, then we mean `size_of::<T>()` if the byte slice is at least
  `size_of::<T>()` bytes long
- If `T` is a slice DST, then we mean to consider all values, `len`, such
  that the instance of `T` with `len` trailing slice elements fits in the
  byte slice, and to choose the largest such `len`, if any


# Safety

This trait does not convey any safety guarantees to code outside this crate.

You must not rely on the `#[doc(hidden)]` internals of `KnownLayout`. Future
releases of zerocopy may make backwards-breaking changes to these items,
including changes that only affect soundness, which may cause code which
uses those items to silently become unsound.

[derive]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.KnownLayout.html

```rust
pub unsafe trait KnownLayout {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `PointerMetadata`: The type of metadata stored in a pointer to `Self`.

#### Implementations

This trait is implemented for the following types:

- `U16<O>` with <O>
- `U32<O>` with <O>
- `U64<O>` with <O>
- `U128<O>` with <O>
- `Usize<O>` with <O>
- `I16<O>` with <O>
- `I32<O>` with <O>
- `I64<O>` with <O>
- `I128<O>` with <O>
- `Isize<O>` with <O>
- `F32<O>` with <O>
- `F64<O>` with <O>
- `AtomicU8`
- `AtomicI8`
- `AtomicBool`
- `AtomicU16`
- `AtomicI16`
- `AtomicU32`
- `AtomicI32`
- `AtomicU64`
- `AtomicI64`
- `AtomicUsize`
- `AtomicIsize`
- `AtomicPtr<T>` with <T>
- `float32x2_t`
- `float32x4_t`
- `float64x1_t`
- `float64x2_t`
- `int8x8_t`
- `int8x8x2_t`
- `int8x8x3_t`
- `int8x8x4_t`
- `int8x16_t`
- `int8x16x2_t`
- `int8x16x3_t`
- `int8x16x4_t`
- `int16x4_t`
- `int16x8_t`
- `int32x2_t`
- `int32x4_t`
- `int64x1_t`
- `int64x2_t`
- `poly8x8_t`
- `poly8x8x2_t`
- `poly8x8x3_t`
- `poly8x8x4_t`
- `poly8x16_t`
- `poly8x16x2_t`
- `poly8x16x3_t`
- `poly8x16x4_t`
- `poly16x4_t`
- `poly16x8_t`
- `poly64x1_t`
- `poly64x2_t`
- `uint8x8_t`
- `uint8x8x2_t`
- `uint8x8x3_t`
- `uint8x8x4_t`
- `uint8x16_t`
- `uint8x16x2_t`
- `uint8x16x3_t`
- `uint8x16x4_t`
- `uint16x4_t`
- `uint16x8_t`
- `uint32x2_t`
- `uint32x4_t`
- `uint64x1_t`
- `uint64x2_t`
- `Unalign<T>` with <T>
- `[T]` with <T>
- `()`
- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`
- `f32`
- `f64`
- `bool`
- `char`
- `NonZeroU8`
- `NonZeroI8`
- `NonZeroU16`
- `NonZeroI16`
- `NonZeroU32`
- `NonZeroI32`
- `NonZeroU64`
- `NonZeroI64`
- `NonZeroU128`
- `NonZeroI128`
- `NonZeroUsize`
- `NonZeroIsize`
- `Option<T>` with <T>
- `PhantomData<T>` with <T: ?Sized>
- `Wrapping<T>` with <T>
- `CoreMaybeUninit<T>` with <T>
- `*const T` with <T: ?Sized>
- `*mut T` with <T: ?Sized>
- `&T` with <T: ?Sized>
- `&mut T` with <T: ?Sized>
- `[T; N]` with <T, const N: usize>
- `str`
- `ManuallyDrop<T>` with <T: ?Sized + KnownLayout>
- `UnsafeCell<T>` with <T: ?Sized + KnownLayout>
- `Cell<T>` with <T: ?Sized + KnownLayout>

### Trait `Immutable`

**Attributes:**

- `#[<cfg_attr>(feature = "derive", doc = "[derive]: zerocopy_derive::Immutable",
doc = "[derive-analysis]: zerocopy_derive::Immutable#analysis")]`
- `#[<cfg_attr>(not(feature = "derive"), doc =
concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"),
"/zerocopy/derive.Immutable.html"), doc =
concat!("[derive-analysis]: https://docs.rs/zerocopy/",
env!("CARGO_PKG_VERSION"), "/zerocopy/derive.Immutable.html#analysis"),)]`
- `#[<cfg_attr>(zerocopy_diagnostic_on_unimplemented_1_78_0,
diagnostic::on_unimplemented(note =
"Consider adding `#[derive(Immutable)]` to `{Self}`"))]`
- `#[diagnostic::on_unimplemented(note =
"Consider adding `#[derive(Immutable)]` to `{Self}`")]`

Types which are free from interior mutability.

`T: Immutable` indicates that `T` does not permit interior mutation, except
by ownership or an exclusive (`&mut`) borrow.

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(Immutable)]`][derive] (requires the `derive` Cargo feature);
e.g.:

```
# use zerocopy_derive::Immutable;
#[derive(Immutable)]
struct MyStruct {
# /*
    ...
# */
}

#[derive(Immutable)]
enum MyEnum {
# /*
    ...
# */
}

#[derive(Immutable)]
union MyUnion {
#   variant: u8,
# /*
    ...
# */
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `Immutable`.

# Safety

Unsafe code outside of this crate must not make any assumptions about `T`
based on `T: Immutable`. We reserve the right to relax the requirements for
`Immutable` in the future, and if unsafe code outside of this crate makes
assumptions based on `T: Immutable`, future relaxations may cause that code
to become unsound.

[derive]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.Immutable.html
[derive-analysis]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.Immutable.html#analysis

```rust
pub unsafe trait Immutable {
    /* Associated items */
}
```

> This trait is unsafe to implement.

#### Implementations

This trait is implemented for the following types:

- `U16<O>` with <O>
- `U32<O>` with <O>
- `U64<O>` with <O>
- `U128<O>` with <O>
- `Usize<O>` with <O>
- `I16<O>` with <O>
- `I32<O>` with <O>
- `I64<O>` with <O>
- `I128<O>` with <O>
- `Isize<O>` with <O>
- `F32<O>` with <O>
- `F64<O>` with <O>
- `()`
- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`
- `f32`
- `f64`
- `bool`
- `char`
- `str`
- `NonZeroU8`
- `NonZeroI8`
- `NonZeroU16`
- `NonZeroI16`
- `NonZeroU32`
- `NonZeroI32`
- `NonZeroU64`
- `NonZeroI64`
- `NonZeroU128`
- `NonZeroI128`
- `NonZeroUsize`
- `NonZeroIsize`
- `Option<fn() -> M>` with <M>
- `Option<fn(L) -> M>` with <L, M>
- `Option<fn(K, L) -> M>` with <K, L, M>
- `Option<fn(J, K, L) -> M>` with <J, K, L, M>
- `Option<fn(I, J, K, L) -> M>` with <I, J, K, L, M>
- `Option<fn(H, I, J, K, L) -> M>` with <H, I, J, K, L, M>
- `Option<fn(G, H, I, J, K, L) -> M>` with <G, H, I, J, K, L, M>
- `Option<fn(F, G, H, I, J, K, L) -> M>` with <F, G, H, I, J, K, L, M>
- `Option<fn(E, F, G, H, I, J, K, L) -> M>` with <E, F, G, H, I, J, K, L, M>
- `Option<fn(D, E, F, G, H, I, J, K, L) -> M>` with <D, E, F, G, H, I, J, K, L, M>
- `Option<fn(C, D, E, F, G, H, I, J, K, L) -> M>` with <C, D, E, F, G, H, I, J, K, L, M>
- `Option<fn(B, C, D, E, F, G, H, I, J, K, L) -> M>` with <B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>` with <A, B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn() -> M>` with <M>
- `Option<extern "C" fn(L) -> M>` with <L, M>
- `Option<extern "C" fn(K, L) -> M>` with <K, L, M>
- `Option<extern "C" fn(J, K, L) -> M>` with <J, K, L, M>
- `Option<extern "C" fn(I, J, K, L) -> M>` with <I, J, K, L, M>
- `Option<extern "C" fn(H, I, J, K, L) -> M>` with <H, I, J, K, L, M>
- `Option<extern "C" fn(G, H, I, J, K, L) -> M>` with <G, H, I, J, K, L, M>
- `Option<extern "C" fn(F, G, H, I, J, K, L) -> M>` with <F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(E, F, G, H, I, J, K, L) -> M>` with <E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(D, E, F, G, H, I, J, K, L) -> M>` with <D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(C, D, E, F, G, H, I, J, K, L) -> M>` with <C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(B, C, D, E, F, G, H, I, J, K, L) -> M>` with <B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>` with <A, B, C, D, E, F, G, H, I, J, K, L, M>
- `PhantomData<T>` with <T: ?Sized>
- `Wrapping<T>` with <T: Immutable>
- `CoreMaybeUninit<T>` with <T: Immutable>
- `ManuallyDrop<T>` with <T: ?Sized + Immutable>
- `[T; N]` with <T: Immutable, const N: usize>
- `[T]` with <T: Immutable>
- `*const T` with <T: ?Sized>
- `*mut T` with <T: ?Sized>
- `NonNull<T>` with <T: ?Sized>
- `&T` with <T: ?Sized>
- `&mut T` with <T: ?Sized>
- `Option<T>` with <T: Immutable>
- `float32x2_t`
- `float32x4_t`
- `float64x1_t`
- `float64x2_t`
- `int8x8_t`
- `int8x8x2_t`
- `int8x8x3_t`
- `int8x8x4_t`
- `int8x16_t`
- `int8x16x2_t`
- `int8x16x3_t`
- `int8x16x4_t`
- `int16x4_t`
- `int16x8_t`
- `int32x2_t`
- `int32x4_t`
- `int64x1_t`
- `int64x2_t`
- `poly8x8_t`
- `poly8x8x2_t`
- `poly8x8x3_t`
- `poly8x8x4_t`
- `poly8x16_t`
- `poly8x16x2_t`
- `poly8x16x3_t`
- `poly8x16x4_t`
- `poly16x4_t`
- `poly16x8_t`
- `poly64x1_t`
- `poly64x2_t`
- `uint8x8_t`
- `uint8x8x2_t`
- `uint8x8x3_t`
- `uint8x8x4_t`
- `uint8x16_t`
- `uint8x16x2_t`
- `uint8x16x3_t`
- `uint8x16x4_t`
- `uint16x4_t`
- `uint16x8_t`
- `uint32x2_t`
- `uint32x4_t`
- `uint64x1_t`
- `uint64x2_t`
- `Unalign<T>` with <T: Immutable>

### Trait `TryFromBytes`

**Attributes:**

- `#[<cfg_attr>(feature = "derive", doc =
"[derive]: zerocopy_derive::TryFromBytes")]`
- `#[<cfg_attr>(not(feature = "derive"), doc =
concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"),
"/zerocopy/derive.TryFromBytes.html"),)]`
- `#[<cfg_attr>(zerocopy_diagnostic_on_unimplemented_1_78_0,
diagnostic::on_unimplemented(note =
"Consider adding `#[derive(TryFromBytes)]` to `{Self}`"))]`
- `#[diagnostic::on_unimplemented(note =
"Consider adding `#[derive(TryFromBytes)]` to `{Self}`")]`

Types for which some bit patterns are valid.

A memory region of the appropriate length which contains initialized bytes
can be viewed as a `TryFromBytes` type so long as the runtime value of those
bytes corresponds to a [*valid instance*] of that type. For example,
[`bool`] is `TryFromBytes`, so zerocopy can transmute a [`u8`] into a
[`bool`] so long as it first checks that the value of the [`u8`] is `0` or
`1`.

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(TryFromBytes)]`][derive]; e.g.:

```
# use zerocopy_derive::{TryFromBytes, Immutable};
#[derive(TryFromBytes)]
struct MyStruct {
# /*
    ...
# */
}

#[derive(TryFromBytes)]
#[repr(u8)]
enum MyEnum {
#   V00,
# /*
    ...
# */
}

#[derive(TryFromBytes, Immutable)]
union MyUnion {
#   variant: u8,
# /*
    ...
# */
}
```

This derive ensures that the runtime check of whether bytes correspond to a
valid instance is sound. You **must** implement this trait via the derive.

# What is a "valid instance"?

In Rust, each type has *bit validity*, which refers to the set of bit
patterns which may appear in an instance of that type. It is impossible for
safe Rust code to produce values which violate bit validity (ie, values
outside of the "valid" set of bit patterns). If `unsafe` code produces an
invalid value, this is considered [undefined behavior].

Rust's bit validity rules are currently being decided, which means that some
types have three classes of bit patterns: those which are definitely valid,
and whose validity is documented in the language; those which may or may not
be considered valid at some point in the future; and those which are
definitely invalid.

Zerocopy takes a conservative approach, and only considers a bit pattern to
be valid if its validity is a documenteed guarantee provided by the
language.

For most use cases, Rust's current guarantees align with programmers'
intuitions about what ought to be valid. As a result, zerocopy's
conservatism should not affect most users.

If you are negatively affected by lack of support for a particular type,
we encourage you to let us know by [filing an issue][github-repo].

# `TryFromBytes` is not symmetrical with [`IntoBytes`]

There are some types which implement both `TryFromBytes` and [`IntoBytes`],
but for which `TryFromBytes` is not guaranteed to accept all byte sequences
produced by `IntoBytes`. In other words, for some `T: TryFromBytes +
IntoBytes`, there exist values of `t: T` such that
`TryFromBytes::try_ref_from_bytes(t.as_bytes()) == None`. Code should not
generally assume that values produced by `IntoBytes` will necessarily be
accepted as valid by `TryFromBytes`.

# Safety

On its own, `T: TryFromBytes` does not make any guarantees about the layout
or representation of `T`. It merely provides the ability to perform a
validity check at runtime via methods like [`try_ref_from_bytes`].

You must not rely on the `#[doc(hidden)]` internals of `TryFromBytes`.
Future releases of zerocopy may make backwards-breaking changes to these
items, including changes that only affect soundness, which may cause code
which uses those items to silently become unsound.

[undefined behavior]: https://raphlinus.github.io/programming/rust/2018/08/17/undefined-behavior.html
[github-repo]: https://github.com/google/zerocopy
[`try_ref_from_bytes`]: TryFromBytes::try_ref_from_bytes
[*valid instance*]: #what-is-a-valid-instance
[derive]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.TryFromBytes.html

```rust
pub unsafe trait TryFromBytes {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Provided Methods

- ```rust
  fn try_ref_from_bytes(source: &[u8]) -> Result<&Self, TryCastError<&[u8], Self>>
where
    Self: KnownLayout + Immutable { /* ... */ }
  ```
  Attempts to interpret the given `source` as a `&Self`.

- ```rust
  fn try_ref_from_prefix(source: &[u8]) -> Result<(&Self, &[u8]), TryCastError<&[u8], Self>>
where
    Self: KnownLayout + Immutable { /* ... */ }
  ```
  Attempts to interpret the prefix of the given `source` as a `&Self`.

- ```rust
  fn try_ref_from_suffix(source: &[u8]) -> Result<(&[u8], &Self), TryCastError<&[u8], Self>>
where
    Self: KnownLayout + Immutable { /* ... */ }
  ```
  Attempts to interpret the suffix of the given `source` as a `&Self`.

- ```rust
  fn try_mut_from_bytes(bytes: &mut [u8]) -> Result<&mut Self, TryCastError<&mut [u8], Self>>
where
    Self: KnownLayout + IntoBytes { /* ... */ }
  ```
  Attempts to interpret the given `source` as a `&mut Self` without

- ```rust
  fn try_mut_from_prefix(source: &mut [u8]) -> Result<(&mut Self, &mut [u8]), TryCastError<&mut [u8], Self>>
where
    Self: KnownLayout + IntoBytes { /* ... */ }
  ```
  Attempts to interpret the prefix of the given `source` as a `&mut

- ```rust
  fn try_mut_from_suffix(source: &mut [u8]) -> Result<(&mut [u8], &mut Self), TryCastError<&mut [u8], Self>>
where
    Self: KnownLayout + IntoBytes { /* ... */ }
  ```
  Attempts to interpret the suffix of the given `source` as a `&mut

- ```rust
  fn try_ref_from_bytes_with_elems(source: &[u8], count: usize) -> Result<&Self, TryCastError<&[u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + Immutable { /* ... */ }
  ```
  Attempts to interpret the given `source` as a `&Self` with a DST length

- ```rust
  fn try_ref_from_prefix_with_elems(source: &[u8], count: usize) -> Result<(&Self, &[u8]), TryCastError<&[u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + Immutable { /* ... */ }
  ```
  Attempts to interpret the prefix of the given `source` as a `&Self` with

- ```rust
  fn try_ref_from_suffix_with_elems(source: &[u8], count: usize) -> Result<(&[u8], &Self), TryCastError<&[u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + Immutable { /* ... */ }
  ```
  Attempts to interpret the suffix of the given `source` as a `&Self` with

- ```rust
  fn try_mut_from_bytes_with_elems(source: &mut [u8], count: usize) -> Result<&mut Self, TryCastError<&mut [u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + IntoBytes { /* ... */ }
  ```
  Attempts to interpret the given `source` as a `&mut Self` with a DST

- ```rust
  fn try_mut_from_prefix_with_elems(source: &mut [u8], count: usize) -> Result<(&mut Self, &mut [u8]), TryCastError<&mut [u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + IntoBytes { /* ... */ }
  ```
  Attempts to interpret the prefix of the given `source` as a `&mut Self`

- ```rust
  fn try_mut_from_suffix_with_elems(source: &mut [u8], count: usize) -> Result<(&mut [u8], &mut Self), TryCastError<&mut [u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + IntoBytes { /* ... */ }
  ```
  Attempts to interpret the suffix of the given `source` as a `&mut Self`

- ```rust
  fn try_read_from_bytes(source: &[u8]) -> Result<Self, TryReadError<&[u8], Self>>
where
    Self: Sized { /* ... */ }
  ```
  Attempts to read the given `source` as a `Self`.

- ```rust
  fn try_read_from_prefix(source: &[u8]) -> Result<(Self, &[u8]), TryReadError<&[u8], Self>>
where
    Self: Sized { /* ... */ }
  ```
  Attempts to read a `Self` from the prefix of the given `source`.

- ```rust
  fn try_read_from_suffix(source: &[u8]) -> Result<(&[u8], Self), TryReadError<&[u8], Self>>
where
    Self: Sized { /* ... */ }
  ```
  Attempts to read a `Self` from the suffix of the given `source`.

#### Implementations

This trait is implemented for the following types:

- `U16<O>` with <O>
- `U32<O>` with <O>
- `U64<O>` with <O>
- `U128<O>` with <O>
- `Usize<O>` with <O>
- `I16<O>` with <O>
- `I32<O>` with <O>
- `I64<O>` with <O>
- `I128<O>` with <O>
- `Isize<O>` with <O>
- `F32<O>` with <O>
- `F64<O>` with <O>
- `()`
- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`
- `f32`
- `f64`
- `bool`
- `char`
- `str`
- `NonZeroU8`
- `NonZeroI8`
- `NonZeroU16`
- `NonZeroI16`
- `NonZeroU32`
- `NonZeroI32`
- `NonZeroU64`
- `NonZeroI64`
- `NonZeroU128`
- `NonZeroI128`
- `NonZeroUsize`
- `NonZeroIsize`
- `Option<NonZeroU8>`
- `Option<NonZeroI8>`
- `Option<NonZeroU16>`
- `Option<NonZeroI16>`
- `Option<NonZeroU32>`
- `Option<NonZeroI32>`
- `Option<NonZeroU64>`
- `Option<NonZeroI64>`
- `Option<NonZeroU128>`
- `Option<NonZeroI128>`
- `Option<NonZeroUsize>`
- `Option<NonZeroIsize>`
- `Option<&T>` with <T>
- `Option<&mut T>` with <T>
- `Option<NonNull<T>>` with <T>
- `Option<fn() -> M>` with <M>
- `Option<fn(L) -> M>` with <L, M>
- `Option<fn(K, L) -> M>` with <K, L, M>
- `Option<fn(J, K, L) -> M>` with <J, K, L, M>
- `Option<fn(I, J, K, L) -> M>` with <I, J, K, L, M>
- `Option<fn(H, I, J, K, L) -> M>` with <H, I, J, K, L, M>
- `Option<fn(G, H, I, J, K, L) -> M>` with <G, H, I, J, K, L, M>
- `Option<fn(F, G, H, I, J, K, L) -> M>` with <F, G, H, I, J, K, L, M>
- `Option<fn(E, F, G, H, I, J, K, L) -> M>` with <E, F, G, H, I, J, K, L, M>
- `Option<fn(D, E, F, G, H, I, J, K, L) -> M>` with <D, E, F, G, H, I, J, K, L, M>
- `Option<fn(C, D, E, F, G, H, I, J, K, L) -> M>` with <C, D, E, F, G, H, I, J, K, L, M>
- `Option<fn(B, C, D, E, F, G, H, I, J, K, L) -> M>` with <B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>` with <A, B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn() -> M>` with <M>
- `Option<extern "C" fn(L) -> M>` with <L, M>
- `Option<extern "C" fn(K, L) -> M>` with <K, L, M>
- `Option<extern "C" fn(J, K, L) -> M>` with <J, K, L, M>
- `Option<extern "C" fn(I, J, K, L) -> M>` with <I, J, K, L, M>
- `Option<extern "C" fn(H, I, J, K, L) -> M>` with <H, I, J, K, L, M>
- `Option<extern "C" fn(G, H, I, J, K, L) -> M>` with <G, H, I, J, K, L, M>
- `Option<extern "C" fn(F, G, H, I, J, K, L) -> M>` with <F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(E, F, G, H, I, J, K, L) -> M>` with <E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(D, E, F, G, H, I, J, K, L) -> M>` with <D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(C, D, E, F, G, H, I, J, K, L) -> M>` with <C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(B, C, D, E, F, G, H, I, J, K, L) -> M>` with <B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>` with <A, B, C, D, E, F, G, H, I, J, K, L, M>
- `core::sync::atomic::AtomicU8`
- `core::sync::atomic::AtomicI8`
- `core::sync::atomic::AtomicBool`
- `core::sync::atomic::AtomicU16`
- `core::sync::atomic::AtomicI16`
- `core::sync::atomic::AtomicU32`
- `core::sync::atomic::AtomicI32`
- `core::sync::atomic::AtomicU64`
- `core::sync::atomic::AtomicI64`
- `core::sync::atomic::AtomicUsize`
- `core::sync::atomic::AtomicIsize`
- `core::sync::atomic::AtomicPtr<T>` with <T>
- `PhantomData<T>` with <T: ?Sized>
- `core::num::Wrapping<T>` with <T: TryFromBytes>
- `CoreMaybeUninit<T>` with <T>
- `core::mem::ManuallyDrop<T>` with <T: ?Sized + TryFromBytes>
- `core::cell::Cell<T>` with <T: ?Sized + TryFromBytes>
- `core::cell::UnsafeCell<T>` with <T: TryFromBytes + ?Sized>
- `[T; N]` with <T: TryFromBytes, const N: usize>
- `[T]` with <T: TryFromBytes>
- `*const T` with <T>
- `*mut T` with <T>
- `float32x2_t`
- `float32x4_t`
- `float64x1_t`
- `float64x2_t`
- `int8x8_t`
- `int8x8x2_t`
- `int8x8x3_t`
- `int8x8x4_t`
- `int8x16_t`
- `int8x16x2_t`
- `int8x16x3_t`
- `int8x16x4_t`
- `int16x4_t`
- `int16x8_t`
- `int32x2_t`
- `int32x4_t`
- `int64x1_t`
- `int64x2_t`
- `poly8x8_t`
- `poly8x8x2_t`
- `poly8x8x3_t`
- `poly8x8x4_t`
- `poly8x16_t`
- `poly8x16x2_t`
- `poly8x16x3_t`
- `poly8x16x4_t`
- `poly16x4_t`
- `poly16x8_t`
- `poly64x1_t`
- `poly64x2_t`
- `uint8x8_t`
- `uint8x8x2_t`
- `uint8x8x3_t`
- `uint8x8x4_t`
- `uint8x16_t`
- `uint8x16x2_t`
- `uint8x16x3_t`
- `uint8x16x4_t`
- `uint16x4_t`
- `uint16x8_t`
- `uint32x2_t`
- `uint32x4_t`
- `uint64x1_t`
- `uint64x2_t`
- `Unalign<T>` with <T: TryFromBytes>

### Trait `FromZeros`

**Attributes:**

- `#[<cfg_attr>(feature = "derive", doc = "[derive]: zerocopy_derive::FromZeros",
doc = "[derive-analysis]: zerocopy_derive::FromZeros#analysis")]`
- `#[<cfg_attr>(not(feature = "derive"), doc =
concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"),
"/zerocopy/derive.FromZeros.html"), doc =
concat!("[derive-analysis]: https://docs.rs/zerocopy/",
env!("CARGO_PKG_VERSION"), "/zerocopy/derive.FromZeros.html#analysis"),)]`
- `#[<cfg_attr>(zerocopy_diagnostic_on_unimplemented_1_78_0,
diagnostic::on_unimplemented(note =
"Consider adding `#[derive(FromZeros)]` to `{Self}`"))]`
- `#[diagnostic::on_unimplemented(note =
"Consider adding `#[derive(FromZeros)]` to `{Self}`")]`

Types for which a sequence of bytes all set to zero represents a valid
instance of the type.

Any memory region of the appropriate length which is guaranteed to contain
only zero bytes can be viewed as any `FromZeros` type with no runtime
overhead. This is useful whenever memory is known to be in a zeroed state,
such memory returned from some allocation routines.

# Warning: Padding bytes

Note that, when a value is moved or copied, only the non-padding bytes of
that value are guaranteed to be preserved. It is unsound to assume that
values written to padding bytes are preserved after a move or copy. For more
details, see the [`FromBytes` docs][frombytes-warning-padding-bytes].

[frombytes-warning-padding-bytes]: FromBytes#warning-padding-bytes

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(FromZeros)]`][derive]; e.g.:

```
# use zerocopy_derive::{FromZeros, Immutable};
#[derive(FromZeros)]
struct MyStruct {
# /*
    ...
# */
}

#[derive(FromZeros)]
#[repr(u8)]
enum MyEnum {
#   Variant0,
# /*
    ...
# */
}

#[derive(FromZeros, Immutable)]
union MyUnion {
#   variant: u8,
# /*
    ...
# */
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `FromZeros`.

# Safety

*This section describes what is required in order for `T: FromZeros`, and
what unsafe code may assume of such types. If you don't plan on implementing
`FromZeros` manually, and you don't plan on writing unsafe code that
operates on `FromZeros` types, then you don't need to read this section.*

If `T: FromZeros`, then unsafe code may assume that it is sound to produce a
`T` whose bytes are all initialized to zero. If a type is marked as
`FromZeros` which violates this contract, it may cause undefined behavior.

`#[derive(FromZeros)]` only permits [types which satisfy these
requirements][derive-analysis].

[derive]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.FromZeros.html
[derive-analysis]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.FromZeros.html#analysis

```rust
pub unsafe trait FromZeros: TryFromBytes {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Provided Methods

- ```rust
  fn zero(self: &mut Self) { /* ... */ }
  ```
  Overwrites `self` with zeros.

- ```rust
  fn new_zeroed() -> Self
where
    Self: Sized { /* ... */ }
  ```
  Creates an instance of `Self` from zeroed bytes.

#### Implementations

This trait is implemented for the following types:

- `U16<O>` with <O>
- `U32<O>` with <O>
- `U64<O>` with <O>
- `U128<O>` with <O>
- `Usize<O>` with <O>
- `I16<O>` with <O>
- `I32<O>` with <O>
- `I64<O>` with <O>
- `I128<O>` with <O>
- `Isize<O>` with <O>
- `F32<O>` with <O>
- `F64<O>` with <O>
- `()`
- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`
- `f32`
- `f64`
- `bool`
- `char`
- `str`
- `Option<NonZeroU8>`
- `Option<NonZeroI8>`
- `Option<NonZeroU16>`
- `Option<NonZeroI16>`
- `Option<NonZeroU32>`
- `Option<NonZeroI32>`
- `Option<NonZeroU64>`
- `Option<NonZeroI64>`
- `Option<NonZeroU128>`
- `Option<NonZeroI128>`
- `Option<NonZeroUsize>`
- `Option<NonZeroIsize>`
- `Option<&T>` with <T>
- `Option<&mut T>` with <T>
- `Option<NonNull<T>>` with <T>
- `Option<fn() -> M>` with <M>
- `Option<fn(L) -> M>` with <L, M>
- `Option<fn(K, L) -> M>` with <K, L, M>
- `Option<fn(J, K, L) -> M>` with <J, K, L, M>
- `Option<fn(I, J, K, L) -> M>` with <I, J, K, L, M>
- `Option<fn(H, I, J, K, L) -> M>` with <H, I, J, K, L, M>
- `Option<fn(G, H, I, J, K, L) -> M>` with <G, H, I, J, K, L, M>
- `Option<fn(F, G, H, I, J, K, L) -> M>` with <F, G, H, I, J, K, L, M>
- `Option<fn(E, F, G, H, I, J, K, L) -> M>` with <E, F, G, H, I, J, K, L, M>
- `Option<fn(D, E, F, G, H, I, J, K, L) -> M>` with <D, E, F, G, H, I, J, K, L, M>
- `Option<fn(C, D, E, F, G, H, I, J, K, L) -> M>` with <C, D, E, F, G, H, I, J, K, L, M>
- `Option<fn(B, C, D, E, F, G, H, I, J, K, L) -> M>` with <B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>` with <A, B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn() -> M>` with <M>
- `Option<extern "C" fn(L) -> M>` with <L, M>
- `Option<extern "C" fn(K, L) -> M>` with <K, L, M>
- `Option<extern "C" fn(J, K, L) -> M>` with <J, K, L, M>
- `Option<extern "C" fn(I, J, K, L) -> M>` with <I, J, K, L, M>
- `Option<extern "C" fn(H, I, J, K, L) -> M>` with <H, I, J, K, L, M>
- `Option<extern "C" fn(G, H, I, J, K, L) -> M>` with <G, H, I, J, K, L, M>
- `Option<extern "C" fn(F, G, H, I, J, K, L) -> M>` with <F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(E, F, G, H, I, J, K, L) -> M>` with <E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(D, E, F, G, H, I, J, K, L) -> M>` with <D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(C, D, E, F, G, H, I, J, K, L) -> M>` with <C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(B, C, D, E, F, G, H, I, J, K, L) -> M>` with <B, C, D, E, F, G, H, I, J, K, L, M>
- `Option<extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>` with <A, B, C, D, E, F, G, H, I, J, K, L, M>
- `core::sync::atomic::AtomicU8`
- `core::sync::atomic::AtomicI8`
- `core::sync::atomic::AtomicBool`
- `core::sync::atomic::AtomicU16`
- `core::sync::atomic::AtomicI16`
- `core::sync::atomic::AtomicU32`
- `core::sync::atomic::AtomicI32`
- `core::sync::atomic::AtomicU64`
- `core::sync::atomic::AtomicI64`
- `core::sync::atomic::AtomicUsize`
- `core::sync::atomic::AtomicIsize`
- `core::sync::atomic::AtomicPtr<T>` with <T>
- `PhantomData<T>` with <T: ?Sized>
- `core::num::Wrapping<T>` with <T: FromZeros>
- `CoreMaybeUninit<T>` with <T>
- `core::mem::ManuallyDrop<T>` with <T: ?Sized + FromZeros>
- `core::cell::Cell<T>` with <T: ?Sized + FromZeros>
- `core::cell::UnsafeCell<T>` with <T: ?Sized + FromZeros>
- `[T; N]` with <T: FromZeros, const N: usize>
- `[T]` with <T: FromZeros>
- `*const T` with <T>
- `*mut T` with <T>
- `float32x2_t`
- `float32x4_t`
- `float64x1_t`
- `float64x2_t`
- `int8x8_t`
- `int8x8x2_t`
- `int8x8x3_t`
- `int8x8x4_t`
- `int8x16_t`
- `int8x16x2_t`
- `int8x16x3_t`
- `int8x16x4_t`
- `int16x4_t`
- `int16x8_t`
- `int32x2_t`
- `int32x4_t`
- `int64x1_t`
- `int64x2_t`
- `poly8x8_t`
- `poly8x8x2_t`
- `poly8x8x3_t`
- `poly8x8x4_t`
- `poly8x16_t`
- `poly8x16x2_t`
- `poly8x16x3_t`
- `poly8x16x4_t`
- `poly16x4_t`
- `poly16x8_t`
- `poly64x1_t`
- `poly64x2_t`
- `uint8x8_t`
- `uint8x8x2_t`
- `uint8x8x3_t`
- `uint8x8x4_t`
- `uint8x16_t`
- `uint8x16x2_t`
- `uint8x16x3_t`
- `uint8x16x4_t`
- `uint16x4_t`
- `uint16x8_t`
- `uint32x2_t`
- `uint32x4_t`
- `uint64x1_t`
- `uint64x2_t`
- `Unalign<T>` with <T: FromZeros>

### Trait `FromBytes`

**Attributes:**

- `#[<cfg_attr>(feature = "derive", doc = "[derive]: zerocopy_derive::FromBytes",
doc = "[derive-analysis]: zerocopy_derive::FromBytes#analysis")]`
- `#[<cfg_attr>(not(feature = "derive"), doc =
concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"),
"/zerocopy/derive.FromBytes.html"), doc =
concat!("[derive-analysis]: https://docs.rs/zerocopy/",
env!("CARGO_PKG_VERSION"), "/zerocopy/derive.FromBytes.html#analysis"),)]`
- `#[<cfg_attr>(zerocopy_diagnostic_on_unimplemented_1_78_0,
diagnostic::on_unimplemented(note =
"Consider adding `#[derive(FromBytes)]` to `{Self}`"))]`
- `#[diagnostic::on_unimplemented(note =
"Consider adding `#[derive(FromBytes)]` to `{Self}`")]`

Types for which any bit pattern is valid.

Any memory region of the appropriate length which contains initialized bytes
can be viewed as any `FromBytes` type with no runtime overhead. This is
useful for efficiently parsing bytes as structured data.

# Warning: Padding bytes

Note that, when a value is moved or copied, only the non-padding bytes of
that value are guaranteed to be preserved. It is unsound to assume that
values written to padding bytes are preserved after a move or copy. For
example, the following is unsound:

```rust,no_run
use core::mem::{size_of, transmute};
use zerocopy::FromZeros;
# use zerocopy_derive::*;

// Assume `Foo` is a type with padding bytes.
#[derive(FromZeros, Default)]
struct Foo {
# /*
    ...
# */
}

let mut foo: Foo = Foo::default();
FromZeros::zero(&mut foo);
// UNSOUND: Although `FromZeros::zero` writes zeros to all bytes of `foo`,
// those writes are not guaranteed to be preserved in padding bytes when
// `foo` is moved, so this may expose padding bytes as `u8`s.
let foo_bytes: [u8; size_of::<Foo>()] = unsafe { transmute(foo) };
```

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(FromBytes)]`][derive]; e.g.:

```
# use zerocopy_derive::{FromBytes, Immutable};
#[derive(FromBytes)]
struct MyStruct {
# /*
    ...
# */
}

#[derive(FromBytes)]
#[repr(u8)]
enum MyEnum {
#   V00, V01, V02, V03, V04, V05, V06, V07, V08, V09, V0A, V0B, V0C, V0D, V0E,
#   V0F, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V1A, V1B, V1C, V1D,
#   V1E, V1F, V20, V21, V22, V23, V24, V25, V26, V27, V28, V29, V2A, V2B, V2C,
#   V2D, V2E, V2F, V30, V31, V32, V33, V34, V35, V36, V37, V38, V39, V3A, V3B,
#   V3C, V3D, V3E, V3F, V40, V41, V42, V43, V44, V45, V46, V47, V48, V49, V4A,
#   V4B, V4C, V4D, V4E, V4F, V50, V51, V52, V53, V54, V55, V56, V57, V58, V59,
#   V5A, V5B, V5C, V5D, V5E, V5F, V60, V61, V62, V63, V64, V65, V66, V67, V68,
#   V69, V6A, V6B, V6C, V6D, V6E, V6F, V70, V71, V72, V73, V74, V75, V76, V77,
#   V78, V79, V7A, V7B, V7C, V7D, V7E, V7F, V80, V81, V82, V83, V84, V85, V86,
#   V87, V88, V89, V8A, V8B, V8C, V8D, V8E, V8F, V90, V91, V92, V93, V94, V95,
#   V96, V97, V98, V99, V9A, V9B, V9C, V9D, V9E, V9F, VA0, VA1, VA2, VA3, VA4,
#   VA5, VA6, VA7, VA8, VA9, VAA, VAB, VAC, VAD, VAE, VAF, VB0, VB1, VB2, VB3,
#   VB4, VB5, VB6, VB7, VB8, VB9, VBA, VBB, VBC, VBD, VBE, VBF, VC0, VC1, VC2,
#   VC3, VC4, VC5, VC6, VC7, VC8, VC9, VCA, VCB, VCC, VCD, VCE, VCF, VD0, VD1,
#   VD2, VD3, VD4, VD5, VD6, VD7, VD8, VD9, VDA, VDB, VDC, VDD, VDE, VDF, VE0,
#   VE1, VE2, VE3, VE4, VE5, VE6, VE7, VE8, VE9, VEA, VEB, VEC, VED, VEE, VEF,
#   VF0, VF1, VF2, VF3, VF4, VF5, VF6, VF7, VF8, VF9, VFA, VFB, VFC, VFD, VFE,
#   VFF,
# /*
    ...
# */
}

#[derive(FromBytes, Immutable)]
union MyUnion {
#   variant: u8,
# /*
    ...
# */
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `FromBytes`.

# Safety

*This section describes what is required in order for `T: FromBytes`, and
what unsafe code may assume of such types. If you don't plan on implementing
`FromBytes` manually, and you don't plan on writing unsafe code that
operates on `FromBytes` types, then you don't need to read this section.*

If `T: FromBytes`, then unsafe code may assume that it is sound to produce a
`T` whose bytes are initialized to any sequence of valid `u8`s (in other
words, any byte value which is not uninitialized). If a type is marked as
`FromBytes` which violates this contract, it may cause undefined behavior.

`#[derive(FromBytes)]` only permits [types which satisfy these
requirements][derive-analysis].

[derive]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.FromBytes.html
[derive-analysis]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.FromBytes.html#analysis

```rust
pub unsafe trait FromBytes: FromZeros {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Provided Methods

- ```rust
  fn ref_from_bytes(source: &[u8]) -> Result<&Self, CastError<&[u8], Self>>
where
    Self: KnownLayout + Immutable { /* ... */ }
  ```
  Interprets the given `source` as a `&Self`.

- ```rust
  fn ref_from_prefix(source: &[u8]) -> Result<(&Self, &[u8]), CastError<&[u8], Self>>
where
    Self: KnownLayout + Immutable { /* ... */ }
  ```
  Interprets the prefix of the given `source` as a `&Self` without

- ```rust
  fn ref_from_suffix(source: &[u8]) -> Result<(&[u8], &Self), CastError<&[u8], Self>>
where
    Self: Immutable + KnownLayout { /* ... */ }
  ```
  Interprets the suffix of the given bytes as a `&Self`.

- ```rust
  fn mut_from_bytes(source: &mut [u8]) -> Result<&mut Self, CastError<&mut [u8], Self>>
where
    Self: IntoBytes + KnownLayout { /* ... */ }
  ```
  Interprets the given `source` as a `&mut Self`.

- ```rust
  fn mut_from_prefix(source: &mut [u8]) -> Result<(&mut Self, &mut [u8]), CastError<&mut [u8], Self>>
where
    Self: IntoBytes + KnownLayout { /* ... */ }
  ```
  Interprets the prefix of the given `source` as a `&mut Self` without

- ```rust
  fn mut_from_suffix(source: &mut [u8]) -> Result<(&mut [u8], &mut Self), CastError<&mut [u8], Self>>
where
    Self: IntoBytes + KnownLayout { /* ... */ }
  ```
  Interprets the suffix of the given `source` as a `&mut Self` without

- ```rust
  fn ref_from_bytes_with_elems(source: &[u8], count: usize) -> Result<&Self, CastError<&[u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + Immutable { /* ... */ }
  ```
  Interprets the given `source` as a `&Self` with a DST length equal to

- ```rust
  fn ref_from_prefix_with_elems(source: &[u8], count: usize) -> Result<(&Self, &[u8]), CastError<&[u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + Immutable { /* ... */ }
  ```
  Interprets the prefix of the given `source` as a DST `&Self` with length

- ```rust
  fn ref_from_suffix_with_elems(source: &[u8], count: usize) -> Result<(&[u8], &Self), CastError<&[u8], Self>>
where
    Self: KnownLayout<PointerMetadata = usize> + Immutable { /* ... */ }
  ```
  Interprets the suffix of the given `source` as a DST `&Self` with length

- ```rust
  fn mut_from_bytes_with_elems(source: &mut [u8], count: usize) -> Result<&mut Self, CastError<&mut [u8], Self>>
where
    Self: IntoBytes + KnownLayout<PointerMetadata = usize> + Immutable { /* ... */ }
  ```
  Interprets the given `source` as a `&mut Self` with a DST length equal

- ```rust
  fn mut_from_prefix_with_elems(source: &mut [u8], count: usize) -> Result<(&mut Self, &mut [u8]), CastError<&mut [u8], Self>>
where
    Self: IntoBytes + KnownLayout<PointerMetadata = usize> { /* ... */ }
  ```
  Interprets the prefix of the given `source` as a `&mut Self` with DST

- ```rust
  fn mut_from_suffix_with_elems(source: &mut [u8], count: usize) -> Result<(&mut [u8], &mut Self), CastError<&mut [u8], Self>>
where
    Self: IntoBytes + KnownLayout<PointerMetadata = usize> { /* ... */ }
  ```
  Interprets the suffix of the given `source` as a `&mut Self` with DST

- ```rust
  fn read_from_bytes(source: &[u8]) -> Result<Self, SizeError<&[u8], Self>>
where
    Self: Sized { /* ... */ }
  ```
  Reads a copy of `Self` from the given `source`.

- ```rust
  fn read_from_prefix(source: &[u8]) -> Result<(Self, &[u8]), SizeError<&[u8], Self>>
where
    Self: Sized { /* ... */ }
  ```
  Reads a copy of `Self` from the prefix of the given `source`.

- ```rust
  fn read_from_suffix(source: &[u8]) -> Result<(&[u8], Self), SizeError<&[u8], Self>>
where
    Self: Sized { /* ... */ }
  ```
  Reads a copy of `Self` from the suffix of the given `source`.

#### Implementations

This trait is implemented for the following types:

- `U16<O>` with <O>
- `U32<O>` with <O>
- `U64<O>` with <O>
- `U128<O>` with <O>
- `Usize<O>` with <O>
- `I16<O>` with <O>
- `I32<O>` with <O>
- `I64<O>` with <O>
- `I128<O>` with <O>
- `Isize<O>` with <O>
- `F32<O>` with <O>
- `F64<O>` with <O>
- `()`
- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`
- `f32`
- `f64`
- `Option<NonZeroU8>`
- `Option<NonZeroI8>`
- `Option<NonZeroU16>`
- `Option<NonZeroI16>`
- `Option<NonZeroU32>`
- `Option<NonZeroI32>`
- `Option<NonZeroU64>`
- `Option<NonZeroI64>`
- `Option<NonZeroU128>`
- `Option<NonZeroI128>`
- `Option<NonZeroUsize>`
- `Option<NonZeroIsize>`
- `core::sync::atomic::AtomicU8`
- `core::sync::atomic::AtomicI8`
- `core::sync::atomic::AtomicU16`
- `core::sync::atomic::AtomicI16`
- `core::sync::atomic::AtomicU32`
- `core::sync::atomic::AtomicI32`
- `core::sync::atomic::AtomicU64`
- `core::sync::atomic::AtomicI64`
- `core::sync::atomic::AtomicUsize`
- `core::sync::atomic::AtomicIsize`
- `PhantomData<T>` with <T: ?Sized>
- `core::num::Wrapping<T>` with <T: FromBytes>
- `CoreMaybeUninit<T>` with <T>
- `core::mem::ManuallyDrop<T>` with <T: ?Sized + FromBytes>
- `core::cell::Cell<T>` with <T: ?Sized + FromBytes>
- `core::cell::UnsafeCell<T>` with <T: ?Sized + FromBytes>
- `[T; N]` with <T: FromBytes, const N: usize>
- `[T]` with <T: FromBytes>
- `float32x2_t`
- `float32x4_t`
- `float64x1_t`
- `float64x2_t`
- `int8x8_t`
- `int8x8x2_t`
- `int8x8x3_t`
- `int8x8x4_t`
- `int8x16_t`
- `int8x16x2_t`
- `int8x16x3_t`
- `int8x16x4_t`
- `int16x4_t`
- `int16x8_t`
- `int32x2_t`
- `int32x4_t`
- `int64x1_t`
- `int64x2_t`
- `poly8x8_t`
- `poly8x8x2_t`
- `poly8x8x3_t`
- `poly8x8x4_t`
- `poly8x16_t`
- `poly8x16x2_t`
- `poly8x16x3_t`
- `poly8x16x4_t`
- `poly16x4_t`
- `poly16x8_t`
- `poly64x1_t`
- `poly64x2_t`
- `uint8x8_t`
- `uint8x8x2_t`
- `uint8x8x3_t`
- `uint8x8x4_t`
- `uint8x16_t`
- `uint8x16x2_t`
- `uint8x16x3_t`
- `uint8x16x4_t`
- `uint16x4_t`
- `uint16x8_t`
- `uint32x2_t`
- `uint32x4_t`
- `uint64x1_t`
- `uint64x2_t`
- `Unalign<T>` with <T: FromBytes>

### Trait `IntoBytes`

**Attributes:**

- `#[<cfg_attr>(feature = "derive", doc = "[derive]: zerocopy_derive::IntoBytes",
doc = "[derive-analysis]: zerocopy_derive::IntoBytes#analysis")]`
- `#[<cfg_attr>(not(feature = "derive"), doc =
concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"),
"/zerocopy/derive.IntoBytes.html"), doc =
concat!("[derive-analysis]: https://docs.rs/zerocopy/",
env!("CARGO_PKG_VERSION"), "/zerocopy/derive.IntoBytes.html#analysis"),)]`
- `#[<cfg_attr>(zerocopy_diagnostic_on_unimplemented_1_78_0,
diagnostic::on_unimplemented(note =
"Consider adding `#[derive(IntoBytes)]` to `{Self}`"))]`
- `#[diagnostic::on_unimplemented(note =
"Consider adding `#[derive(IntoBytes)]` to `{Self}`")]`

Types that can be converted to an immutable slice of initialized bytes.

Any `IntoBytes` type can be converted to a slice of initialized bytes of the
same size. This is useful for efficiently serializing structured data as raw
bytes.

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(IntoBytes)]`][derive]; e.g.:

```
# use zerocopy_derive::IntoBytes;
#[derive(IntoBytes)]
#[repr(C)]
struct MyStruct {
# /*
    ...
# */
}

#[derive(IntoBytes)]
#[repr(u8)]
enum MyEnum {
#   Variant0,
# /*
    ...
# */
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `IntoBytes`. See the [derive
documentation][derive] for guidance on how to interpret error messages
produced by the derive's analysis.

# Safety

*This section describes what is required in order for `T: IntoBytes`, and
what unsafe code may assume of such types. If you don't plan on implementing
`IntoBytes` manually, and you don't plan on writing unsafe code that
operates on `IntoBytes` types, then you don't need to read this section.*

If `T: IntoBytes`, then unsafe code may assume that it is sound to treat any
`t: T` as an immutable `[u8]` of length `size_of_val(t)`. If a type is
marked as `IntoBytes` which violates this contract, it may cause undefined
behavior.

`#[derive(IntoBytes)]` only permits [types which satisfy these
requirements][derive-analysis].

[derive]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.IntoBytes.html
[derive-analysis]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.IntoBytes.html#analysis

```rust
pub unsafe trait IntoBytes {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Provided Methods

- ```rust
  fn as_bytes(self: &Self) -> &[u8]
where
    Self: Immutable { /* ... */ }
  ```
  Gets the bytes of this value.

- ```rust
  fn as_mut_bytes(self: &mut Self) -> &mut [u8]
where
    Self: FromBytes { /* ... */ }
  ```
  Gets the bytes of this value mutably.

- ```rust
  fn write_to(self: &Self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
where
    Self: Immutable { /* ... */ }
  ```
  Writes a copy of `self` to `dst`.

- ```rust
  fn write_to_prefix(self: &Self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
where
    Self: Immutable { /* ... */ }
  ```
  Writes a copy of `self` to the prefix of `dst`.

- ```rust
  fn write_to_suffix(self: &Self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
where
    Self: Immutable { /* ... */ }
  ```
  Writes a copy of `self` to the suffix of `dst`.

#### Implementations

This trait is implemented for the following types:

- `U16<O>` with <O>
- `U32<O>` with <O>
- `U64<O>` with <O>
- `U128<O>` with <O>
- `Usize<O>` with <O>
- `I16<O>` with <O>
- `I32<O>` with <O>
- `I64<O>` with <O>
- `I128<O>` with <O>
- `Isize<O>` with <O>
- `F32<O>` with <O>
- `F64<O>` with <O>
- `()`
- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`
- `f32`
- `f64`
- `bool`
- `char`
- `str`
- `NonZeroU8`
- `NonZeroI8`
- `NonZeroU16`
- `NonZeroI16`
- `NonZeroU32`
- `NonZeroI32`
- `NonZeroU64`
- `NonZeroI64`
- `NonZeroU128`
- `NonZeroI128`
- `NonZeroUsize`
- `NonZeroIsize`
- `Option<NonZeroU8>`
- `Option<NonZeroI8>`
- `Option<NonZeroU16>`
- `Option<NonZeroI16>`
- `Option<NonZeroU32>`
- `Option<NonZeroI32>`
- `Option<NonZeroU64>`
- `Option<NonZeroI64>`
- `Option<NonZeroU128>`
- `Option<NonZeroI128>`
- `Option<NonZeroUsize>`
- `Option<NonZeroIsize>`
- `core::sync::atomic::AtomicU8`
- `core::sync::atomic::AtomicI8`
- `core::sync::atomic::AtomicBool`
- `core::sync::atomic::AtomicU16`
- `core::sync::atomic::AtomicI16`
- `core::sync::atomic::AtomicU32`
- `core::sync::atomic::AtomicI32`
- `core::sync::atomic::AtomicU64`
- `core::sync::atomic::AtomicI64`
- `core::sync::atomic::AtomicUsize`
- `core::sync::atomic::AtomicIsize`
- `PhantomData<T>` with <T: ?Sized>
- `core::num::Wrapping<T>` with <T: IntoBytes>
- `core::mem::ManuallyDrop<T>` with <T: ?Sized + IntoBytes>
- `core::cell::Cell<T>` with <T: ?Sized + IntoBytes>
- `core::cell::UnsafeCell<T>` with <T: ?Sized + IntoBytes>
- `[T; N]` with <T: IntoBytes, const N: usize>
- `[T]` with <T: IntoBytes>
- `float32x2_t`
- `float32x4_t`
- `float64x1_t`
- `float64x2_t`
- `int8x8_t`
- `int8x8x2_t`
- `int8x8x3_t`
- `int8x8x4_t`
- `int8x16_t`
- `int8x16x2_t`
- `int8x16x3_t`
- `int8x16x4_t`
- `int16x4_t`
- `int16x8_t`
- `int32x2_t`
- `int32x4_t`
- `int64x1_t`
- `int64x2_t`
- `poly8x8_t`
- `poly8x8x2_t`
- `poly8x8x3_t`
- `poly8x8x4_t`
- `poly8x16_t`
- `poly8x16x2_t`
- `poly8x16x3_t`
- `poly8x16x4_t`
- `poly16x4_t`
- `poly16x8_t`
- `poly64x1_t`
- `poly64x2_t`
- `uint8x8_t`
- `uint8x8x2_t`
- `uint8x8x3_t`
- `uint8x8x4_t`
- `uint8x16_t`
- `uint8x16x2_t`
- `uint8x16x3_t`
- `uint8x16x4_t`
- `uint16x4_t`
- `uint16x8_t`
- `uint32x2_t`
- `uint32x4_t`
- `uint64x1_t`
- `uint64x2_t`
- `Unalign<T>` with <T: IntoBytes>

### Trait `Unaligned`

**Attributes:**

- `#[<cfg_attr>(feature = "derive", doc = "[derive]: zerocopy_derive::Unaligned",
doc = "[derive-analysis]: zerocopy_derive::Unaligned#analysis")]`
- `#[<cfg_attr>(not(feature = "derive"), doc =
concat!("[derive]: https://docs.rs/zerocopy/", env!("CARGO_PKG_VERSION"),
"/zerocopy/derive.Unaligned.html"), doc =
concat!("[derive-analysis]: https://docs.rs/zerocopy/",
env!("CARGO_PKG_VERSION"), "/zerocopy/derive.Unaligned.html#analysis"),)]`
- `#[<cfg_attr>(zerocopy_diagnostic_on_unimplemented_1_78_0,
diagnostic::on_unimplemented(note =
"Consider adding `#[derive(Unaligned)]` to `{Self}`"))]`
- `#[diagnostic::on_unimplemented(note =
"Consider adding `#[derive(Unaligned)]` to `{Self}`")]`

Types with no alignment requirement.

If `T: Unaligned`, then `align_of::<T>() == 1`.

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(Unaligned)]`][derive]; e.g.:

```
# use zerocopy_derive::Unaligned;
#[derive(Unaligned)]
#[repr(C)]
struct MyStruct {
# /*
    ...
# */
}

#[derive(Unaligned)]
#[repr(u8)]
enum MyEnum {
#   Variant0,
# /*
    ...
# */
}

#[derive(Unaligned)]
#[repr(packed)]
union MyUnion {
#   variant: u8,
# /*
    ...
# */
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `Unaligned`.

# Safety

*This section describes what is required in order for `T: Unaligned`, and
what unsafe code may assume of such types. If you don't plan on implementing
`Unaligned` manually, and you don't plan on writing unsafe code that
operates on `Unaligned` types, then you don't need to read this section.*

If `T: Unaligned`, then unsafe code may assume that it is sound to produce a
reference to `T` at any memory location regardless of alignment. If a type
is marked as `Unaligned` which violates this contract, it may cause
undefined behavior.

`#[derive(Unaligned)]` only permits [types which satisfy these
requirements][derive-analysis].

[derive]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.Unaligned.html
[derive-analysis]: https://docs.rs/zerocopy/0.8.24/zerocopy/derive.Unaligned.html#analysis

```rust
pub unsafe trait Unaligned {
    /* Associated items */
}
```

> This trait is unsafe to implement.

#### Implementations

This trait is implemented for the following types:

- `U16<O>` with <O>
- `U32<O>` with <O>
- `U64<O>` with <O>
- `U128<O>` with <O>
- `Usize<O>` with <O>
- `I16<O>` with <O>
- `I32<O>` with <O>
- `I64<O>` with <O>
- `I128<O>` with <O>
- `Isize<O>` with <O>
- `F32<O>` with <O>
- `F64<O>` with <O>
- `()`
- `u8`
- `i8`
- `bool`
- `str`
- `NonZeroU8`
- `NonZeroI8`
- `Option<NonZeroU8>`
- `Option<NonZeroI8>`
- `AtomicBool`
- `AtomicU8`
- `AtomicI8`
- `PhantomData<T>` with <T: ?Sized>
- `Wrapping<T>` with <T: Unaligned>
- `CoreMaybeUninit<T>` with <T: Unaligned>
- `ManuallyDrop<T>` with <T: ?Sized + Unaligned>
- `Cell<T>` with <T: ?Sized + Unaligned>
- `UnsafeCell<T>` with <T: ?Sized + Unaligned>
- `[T; N]` with <T: Unaligned, const N: usize>
- `[T]` with <T: Unaligned>
- `Unalign<T>` with <T>

## Macros

### Macro `transmute`

**Attributes:**

- `#[macro_export]`

Safely transmutes a value of one type to a value of another type of the same
size.

This macro behaves like an invocation of this function:

```ignore
const fn transmute<Src, Dst>(src: Src) -> Dst
where
    Src: IntoBytes,
    Dst: FromBytes,
    size_of::<Src>() == size_of::<Dst>(),
{
# /*
    ...
# */
}
```

However, unlike a function, this macro can only be invoked when the types of
`Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
inferred from the calling context; they cannot be explicitly specified in
the macro invocation.

Note that the `Src` produced by the expression `$e` will *not* be dropped.
Semantically, its bits will be copied into a new value of type `Dst`, the
original `Src` will be forgotten, and the value of type `Dst` will be
returned.

# Examples

```
# use zerocopy::transmute;
let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

let two_dimensional: [[u8; 4]; 2] = transmute!(one_dimensional);

assert_eq!(two_dimensional, [[0, 1, 2, 3], [4, 5, 6, 7]]);
```

# Use in `const` contexts

This macro can be invoked in `const` contexts.

```rust
pub macro_rules! transmute {
    /* macro_rules! transmute {
    ($e:expr) => { ... };
} */
}
```

### Macro `transmute_ref`

**Attributes:**

- `#[macro_export]`

Safely transmutes a mutable or immutable reference of one type to an
immutable reference of another type of the same size and compatible
alignment.

This macro behaves like an invocation of this function:

```ignore
const fn transmute_ref<'src, 'dst, Src, Dst>(src: &'src Src) -> &'dst Dst
where
    'src: 'dst,
    Src: IntoBytes + Immutable,
    Dst: FromBytes + Immutable,
    size_of::<Src>() == size_of::<Dst>(),
    align_of::<Src>() >= align_of::<Dst>(),
{
# /*
    ...
# */
}
```

However, unlike a function, this macro can only be invoked when the types of
`Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
inferred from the calling context; they cannot be explicitly specified in
the macro invocation.

# Examples

```
# use zerocopy::transmute_ref;
let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

let two_dimensional: &[[u8; 4]; 2] = transmute_ref!(&one_dimensional);

assert_eq!(two_dimensional, &[[0, 1, 2, 3], [4, 5, 6, 7]]);
```

# Use in `const` contexts

This macro can be invoked in `const` contexts.

# Alignment increase error message

Because of limitations on macros, the error message generated when
`transmute_ref!` is used to transmute from a type of lower alignment to a
type of higher alignment is somewhat confusing. For example, the following
code:

```compile_fail
const INCREASE_ALIGNMENT: &u16 = zerocopy::transmute_ref!(&[0u8; 2]);
```

...generates the following error:

```text
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
 --> src/lib.rs:1524:34
  |
5 | const INCREASE_ALIGNMENT: &u16 = zerocopy::transmute_ref!(&[0u8; 2]);
  |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: source type: `AlignOf<[u8; 2]>` (8 bits)
  = note: target type: `MaxAlignsOf<[u8; 2], u16>` (16 bits)
  = note: this error originates in the macro `$crate::assert_align_gt_eq` which comes from the expansion of the macro `transmute_ref` (in Nightly builds, run with -Z macro-backtrace for more info)
```

This is saying that `max(align_of::<T>(), align_of::<U>()) !=
align_of::<T>()`, which is equivalent to `align_of::<T>() <
align_of::<U>()`.

```rust
pub macro_rules! transmute_ref {
    /* macro_rules! transmute_ref {
    ($e:expr) => { ... };
} */
}
```

### Macro `transmute_mut`

**Attributes:**

- `#[macro_export]`

Safely transmutes a mutable reference of one type to a mutable reference of
another type of the same size and compatible alignment.

This macro behaves like an invocation of this function:

```ignore
const fn transmute_mut<'src, 'dst, Src, Dst>(src: &'src mut Src) -> &'dst mut Dst
where
    'src: 'dst,
    Src: FromBytes + IntoBytes,
    Dst: FromBytes + IntoBytes,
    size_of::<Src>() == size_of::<Dst>(),
    align_of::<Src>() >= align_of::<Dst>(),
{
# /*
    ...
# */
}
```

However, unlike a function, this macro can only be invoked when the types of
`Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
inferred from the calling context; they cannot be explicitly specified in
the macro invocation.

# Examples

```
# use zerocopy::transmute_mut;
let mut one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

let two_dimensional: &mut [[u8; 4]; 2] = transmute_mut!(&mut one_dimensional);

assert_eq!(two_dimensional, &[[0, 1, 2, 3], [4, 5, 6, 7]]);

two_dimensional.reverse();

assert_eq!(one_dimensional, [4, 5, 6, 7, 0, 1, 2, 3]);
```

# Use in `const` contexts

This macro can be invoked in `const` contexts.

# Alignment increase error message

Because of limitations on macros, the error message generated when
`transmute_mut!` is used to transmute from a type of lower alignment to a
type of higher alignment is somewhat confusing. For example, the following
code:

```compile_fail
const INCREASE_ALIGNMENT: &mut u16 = zerocopy::transmute_mut!(&mut [0u8; 2]);
```

...generates the following error:

```text
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
 --> src/lib.rs:1524:34
  |
5 | const INCREASE_ALIGNMENT: &mut u16 = zerocopy::transmute_mut!(&mut [0u8; 2]);
  |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: source type: `AlignOf<[u8; 2]>` (8 bits)
  = note: target type: `MaxAlignsOf<[u8; 2], u16>` (16 bits)
  = note: this error originates in the macro `$crate::assert_align_gt_eq` which comes from the expansion of the macro `transmute_mut` (in Nightly builds, run with -Z macro-backtrace for more info)
```

This is saying that `max(align_of::<T>(), align_of::<U>()) !=
align_of::<T>()`, which is equivalent to `align_of::<T>() <
align_of::<U>()`.

```rust
pub macro_rules! transmute_mut {
    /* macro_rules! transmute_mut {
    ($e:expr) => { ... };
} */
}
```

### Macro `try_transmute`

**Attributes:**

- `#[macro_export]`

Conditionally transmutes a value of one type to a value of another type of
the same size.

This macro behaves like an invocation of this function:

```ignore
fn try_transmute<Src, Dst>(src: Src) -> Result<Dst, ValidityError<Src, Dst>>
where
    Src: IntoBytes,
    Dst: TryFromBytes,
    size_of::<Src>() == size_of::<Dst>(),
{
# /*
    ...
# */
}
```

However, unlike a function, this macro can only be invoked when the types of
`Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
inferred from the calling context; they cannot be explicitly specified in
the macro invocation.

Note that the `Src` produced by the expression `$e` will *not* be dropped.
Semantically, its bits will be copied into a new value of type `Dst`, the
original `Src` will be forgotten, and the value of type `Dst` will be
returned.

# Examples

```
# use zerocopy::*;
// 0u8 → bool = false
assert_eq!(try_transmute!(0u8), Ok(false));

// 1u8 → bool = true
 assert_eq!(try_transmute!(1u8), Ok(true));

// 2u8 → bool = error
assert!(matches!(
    try_transmute!(2u8),
    Result::<bool, _>::Err(ValidityError { .. })
));
```

```rust
pub macro_rules! try_transmute {
    /* macro_rules! try_transmute {
    ($e:expr) => { ... };
} */
}
```

### Macro `try_transmute_ref`

**Attributes:**

- `#[macro_export]`

Conditionally transmutes a mutable or immutable reference of one type to an
immutable reference of another type of the same size and compatible
alignment.

This macro behaves like an invocation of this function:

```ignore
fn try_transmute_ref<Src, Dst>(src: &Src) -> Result<&Dst, ValidityError<&Src, Dst>>
where
    Src: IntoBytes + Immutable,
    Dst: TryFromBytes + Immutable,
    size_of::<Src>() == size_of::<Dst>(),
    align_of::<Src>() >= align_of::<Dst>(),
{
# /*
    ...
# */
}
```

However, unlike a function, this macro can only be invoked when the types of
`Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
inferred from the calling context; they cannot be explicitly specified in
the macro invocation.

# Examples

```
# use zerocopy::*;
// 0u8 → bool = false
assert_eq!(try_transmute_ref!(&0u8), Ok(&false));

// 1u8 → bool = true
 assert_eq!(try_transmute_ref!(&1u8), Ok(&true));

// 2u8 → bool = error
assert!(matches!(
    try_transmute_ref!(&2u8),
    Result::<&bool, _>::Err(ValidityError { .. })
));
```

# Alignment increase error message

Because of limitations on macros, the error message generated when
`try_transmute_ref!` is used to transmute from a type of lower alignment to
a type of higher alignment is somewhat confusing. For example, the following
code:

```compile_fail
let increase_alignment: Result<&u16, _> = zerocopy::try_transmute_ref!(&[0u8; 2]);
```

...generates the following error:

```text
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
 --> example.rs:1:47
  |
1 |     let increase_alignment: Result<&u16, _> = zerocopy::try_transmute_ref!(&[0u8; 2]);
  |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: source type: `AlignOf<[u8; 2]>` (8 bits)
  = note: target type: `MaxAlignsOf<[u8; 2], u16>` (16 bits)
  = note: this error originates in the macro `$crate::assert_align_gt_eq` which comes from the expansion of the macro `zerocopy::try_transmute_ref` (in Nightly builds, run with -Z macro-backtrace for more info)/// ```
```

This is saying that `max(align_of::<T>(), align_of::<U>()) !=
align_of::<T>()`, which is equivalent to `align_of::<T>() <
align_of::<U>()`.

```rust
pub macro_rules! try_transmute_ref {
    /* macro_rules! try_transmute_ref {
    ($e:expr) => { ... };
} */
}
```

### Macro `try_transmute_mut`

**Attributes:**

- `#[macro_export]`

Conditionally transmutes a mutable reference of one type to a mutable
reference of another type of the same size and compatible alignment.

This macro behaves like an invocation of this function:

```ignore
fn try_transmute_mut<Src, Dst>(src: &mut Src) -> Result<&mut Dst, ValidityError<&mut Src, Dst>>
where
    Src: FromBytes + IntoBytes,
    Dst: TryFromBytes + IntoBytes,
    size_of::<Src>() == size_of::<Dst>(),
    align_of::<Src>() >= align_of::<Dst>(),
{
# /*
    ...
# */
}
```

However, unlike a function, this macro can only be invoked when the types of
`Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
inferred from the calling context; they cannot be explicitly specified in
the macro invocation.

# Examples

```
# use zerocopy::*;
// 0u8 → bool = false
let src = &mut 0u8;
assert_eq!(try_transmute_mut!(src), Ok(&mut false));

// 1u8 → bool = true
let src = &mut 1u8;
 assert_eq!(try_transmute_mut!(src), Ok(&mut true));

// 2u8 → bool = error
let src = &mut 2u8;
assert!(matches!(
    try_transmute_mut!(src),
    Result::<&mut bool, _>::Err(ValidityError { .. })
));
```

# Alignment increase error message

Because of limitations on macros, the error message generated when
`try_transmute_ref!` is used to transmute from a type of lower alignment to
a type of higher alignment is somewhat confusing. For example, the following
code:

```compile_fail
let src = &mut [0u8; 2];
let increase_alignment: Result<&mut u16, _> = zerocopy::try_transmute_mut!(src);
```

...generates the following error:

```text
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
 --> example.rs:2:51
  |
2 |     let increase_alignment: Result<&mut u16, _> = zerocopy::try_transmute_mut!(src);
  |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: source type: `AlignOf<[u8; 2]>` (8 bits)
  = note: target type: `MaxAlignsOf<[u8; 2], u16>` (16 bits)
  = note: this error originates in the macro `$crate::assert_align_gt_eq` which comes from the expansion of the macro `zerocopy::try_transmute_mut` (in Nightly builds, run with -Z macro-backtrace for more info)
```

This is saying that `max(align_of::<T>(), align_of::<U>()) !=
align_of::<T>()`, which is equivalent to `align_of::<T>() <
align_of::<U>()`.

```rust
pub macro_rules! try_transmute_mut {
    /* macro_rules! try_transmute_mut {
    ($e:expr) => { ... };
} */
}
```

### Macro `include_value`

**Attributes:**

- `#[doc(alias("include_bytes", "include_data", "include_type"))]`
- `#[macro_export]`

Includes a file and safely transmutes it to a value of an arbitrary type.

The file will be included as a byte array, `[u8; N]`, which will be
transmuted to another type, `T`. `T` is inferred from the calling context,
and must implement [`FromBytes`].

The file is located relative to the current file (similarly to how modules
are found). The provided path is interpreted in a platform-specific way at
compile time. So, for instance, an invocation with a Windows path containing
backslashes `\` would not compile correctly on Unix.

`include_value!` is ignorant of byte order. For byte order-aware types, see
the [`byteorder`] module.

[`FromBytes`]: crate::FromBytes
[`byteorder`]: crate::byteorder

# Examples

Assume there are two files in the same directory with the following
contents:

File `data` (no trailing newline):

```text
abcd
```

File `main.rs`:

```rust
use zerocopy::include_value;
# macro_rules! include_value {
# ($file:expr) => { zerocopy::include_value!(concat!("../testdata/include_value/", $file)) };
# }

fn main() {
    let as_u32: u32 = include_value!("data");
    assert_eq!(as_u32, u32::from_ne_bytes([b'a', b'b', b'c', b'd']));
    let as_i32: i32 = include_value!("data");
    assert_eq!(as_i32, i32::from_ne_bytes([b'a', b'b', b'c', b'd']));
}
```

# Use in `const` contexts

This macro can be invoked in `const` contexts.

```rust
pub macro_rules! include_value {
    /* macro_rules! include_value {
    ($file:expr $(,)?) => { ... };
} */
}
```

## Re-exports

### Re-export `crate::byte_slice::*`

```rust
pub use crate::byte_slice::*;
```

### Re-export `crate::byteorder::*`

```rust
pub use crate::byteorder::*;
```

### Re-export `crate::error::*`

```rust
pub use crate::error::*;
```

### Re-export `crate::ref::*`

```rust
pub use crate::ref::*;
```

### Re-export `crate::wrappers::*`

```rust
pub use crate::wrappers::*;
```

