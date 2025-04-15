# Crate Documentation

**Version:** 1.5.0

**Format Version:** 43

# Module `byteorder`

This crate provides convenience methods for encoding and decoding numbers in
either [big-endian or little-endian order].

The organization of the crate is pretty simple. A trait, [`ByteOrder`], specifies
byte conversion methods for each type of number in Rust (sans numbers that have
a platform dependent size like `usize` and `isize`). Two types, [`BigEndian`]
and [`LittleEndian`] implement these methods. Finally, [`ReadBytesExt`] and
[`WriteBytesExt`] provide convenience methods available to all types that
implement [`Read`] and [`Write`].

An alias, [`NetworkEndian`], for [`BigEndian`] is provided to help improve
code clarity.

An additional alias, [`NativeEndian`], is provided for the endianness of the
local platform. This is convenient when serializing data for use and
conversions are not desired.

# Examples

Read unsigned 16 bit big-endian integers from a [`Read`] type:

```rust
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
// Note that we use type parameters to indicate which kind of byte order
// we want!
assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
```

Write unsigned 16 bit little-endian integers to a [`Write`] type:

```rust
use byteorder::{LittleEndian, WriteBytesExt};

let mut wtr = vec![];
wtr.write_u16::<LittleEndian>(517).unwrap();
wtr.write_u16::<LittleEndian>(768).unwrap();
assert_eq!(wtr, vec![5, 2, 0, 3]);
```

# Optional Features

This crate optionally provides support for 128 bit values (`i128` and `u128`)
when built with the `i128` feature enabled.

This crate can also be used without the standard library.

# Alternatives

Note that as of Rust 1.32, the standard numeric types provide built-in methods
like `to_le_bytes` and `from_le_bytes`, which support some of the same use
cases.

[big-endian or little-endian order]: https://en.wikipedia.org/wiki/Endianness
[`ByteOrder`]: trait.ByteOrder.html
[`BigEndian`]: enum.BigEndian.html
[`LittleEndian`]: enum.LittleEndian.html
[`ReadBytesExt`]: trait.ReadBytesExt.html
[`WriteBytesExt`]: trait.WriteBytesExt.html
[`NetworkEndian`]: type.NetworkEndian.html
[`NativeEndian`]: type.NativeEndian.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

## Types

### Enum `BigEndian`

Defines big-endian serialization.

Note that this type has no value constructor. It is used purely at the
type level.

# Examples

Write and read `u32` numbers in big endian order:

```rust
use byteorder::{ByteOrder, BigEndian};

let mut buf = [0; 4];
BigEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, BigEndian::read_u32(&buf));
```

```rust
pub enum BigEndian {
}
```

#### Variants

#### Implementations

##### Trait Implementations

- **Sync**
- **Copy**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BigEndian) -> bool { /* ... */ }
    ```

- **Send**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &BigEndian) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ByteOrder**
  - ```rust
    fn read_u16(buf: &[u8]) -> u16 { /* ... */ }
    ```

  - ```rust
    fn read_u32(buf: &[u8]) -> u32 { /* ... */ }
    ```

  - ```rust
    fn read_u64(buf: &[u8]) -> u64 { /* ... */ }
    ```

  - ```rust
    fn read_u128(buf: &[u8]) -> u128 { /* ... */ }
    ```

  - ```rust
    fn read_uint(buf: &[u8], nbytes: usize) -> u64 { /* ... */ }
    ```

  - ```rust
    fn read_uint128(buf: &[u8], nbytes: usize) -> u128 { /* ... */ }
    ```

  - ```rust
    fn write_u16(buf: &mut [u8], n: u16) { /* ... */ }
    ```

  - ```rust
    fn write_u32(buf: &mut [u8], n: u32) { /* ... */ }
    ```

  - ```rust
    fn write_u64(buf: &mut [u8], n: u64) { /* ... */ }
    ```

  - ```rust
    fn write_u128(buf: &mut [u8], n: u128) { /* ... */ }
    ```

  - ```rust
    fn write_uint(buf: &mut [u8], n: u64, nbytes: usize) { /* ... */ }
    ```

  - ```rust
    fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize) { /* ... */ }
    ```

  - ```rust
    fn read_u16_into(src: &[u8], dst: &mut [u16]) { /* ... */ }
    ```

  - ```rust
    fn read_u32_into(src: &[u8], dst: &mut [u32]) { /* ... */ }
    ```

  - ```rust
    fn read_u64_into(src: &[u8], dst: &mut [u64]) { /* ... */ }
    ```

  - ```rust
    fn read_u128_into(src: &[u8], dst: &mut [u128]) { /* ... */ }
    ```

  - ```rust
    fn write_u16_into(src: &[u16], dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn write_u32_into(src: &[u32], dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn write_u64_into(src: &[u64], dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn write_u128_into(src: &[u128], dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_u16(numbers: &mut [u16]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_u32(numbers: &mut [u32]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_u64(numbers: &mut [u64]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_u128(numbers: &mut [u128]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_f32(numbers: &mut [f32]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_f64(numbers: &mut [f64]) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BigEndian { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> BigEndian { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &BigEndian) -> $crate::cmp::Ordering { /* ... */ }
    ```

### Type Alias `BE`

A type alias for [`BigEndian`].

[`BigEndian`]: enum.BigEndian.html

```rust
pub type BE = BigEndian;
```

### Enum `LittleEndian`

Defines little-endian serialization.

Note that this type has no value constructor. It is used purely at the
type level.

# Examples

Write and read `u32` numbers in little endian order:

```rust
use byteorder::{ByteOrder, LittleEndian};

let mut buf = [0; 4];
LittleEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
```

```rust
pub enum LittleEndian {
}
```

#### Variants

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> LittleEndian { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &LittleEndian) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &LittleEndian) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> LittleEndian { /* ... */ }
    ```

- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ByteOrder**
  - ```rust
    fn read_u16(buf: &[u8]) -> u16 { /* ... */ }
    ```

  - ```rust
    fn read_u32(buf: &[u8]) -> u32 { /* ... */ }
    ```

  - ```rust
    fn read_u64(buf: &[u8]) -> u64 { /* ... */ }
    ```

  - ```rust
    fn read_u128(buf: &[u8]) -> u128 { /* ... */ }
    ```

  - ```rust
    fn read_uint(buf: &[u8], nbytes: usize) -> u64 { /* ... */ }
    ```

  - ```rust
    fn read_uint128(buf: &[u8], nbytes: usize) -> u128 { /* ... */ }
    ```

  - ```rust
    fn write_u16(buf: &mut [u8], n: u16) { /* ... */ }
    ```

  - ```rust
    fn write_u32(buf: &mut [u8], n: u32) { /* ... */ }
    ```

  - ```rust
    fn write_u64(buf: &mut [u8], n: u64) { /* ... */ }
    ```

  - ```rust
    fn write_u128(buf: &mut [u8], n: u128) { /* ... */ }
    ```

  - ```rust
    fn write_uint(buf: &mut [u8], n: u64, nbytes: usize) { /* ... */ }
    ```

  - ```rust
    fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize) { /* ... */ }
    ```

  - ```rust
    fn read_u16_into(src: &[u8], dst: &mut [u16]) { /* ... */ }
    ```

  - ```rust
    fn read_u32_into(src: &[u8], dst: &mut [u32]) { /* ... */ }
    ```

  - ```rust
    fn read_u64_into(src: &[u8], dst: &mut [u64]) { /* ... */ }
    ```

  - ```rust
    fn read_u128_into(src: &[u8], dst: &mut [u128]) { /* ... */ }
    ```

  - ```rust
    fn write_u16_into(src: &[u16], dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn write_u32_into(src: &[u32], dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn write_u64_into(src: &[u64], dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn write_u128_into(src: &[u128], dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_u16(numbers: &mut [u16]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_u32(numbers: &mut [u32]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_u64(numbers: &mut [u64]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_u128(numbers: &mut [u128]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_f32(numbers: &mut [f32]) { /* ... */ }
    ```

  - ```rust
    fn from_slice_f64(numbers: &mut [f64]) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LittleEndian) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
### Type Alias `LE`

A type alias for [`LittleEndian`].

[`LittleEndian`]: enum.LittleEndian.html

```rust
pub type LE = LittleEndian;
```

### Type Alias `NetworkEndian`

Defines network byte order serialization.

Network byte order is defined by [RFC 1700][1] to be big-endian, and is
referred to in several protocol specifications.  This type is an alias of
[`BigEndian`].

[1]: https://tools.ietf.org/html/rfc1700

Note that this type has no value constructor. It is used purely at the
type level.

# Examples

Write and read `i16` numbers in big endian order:

```rust
use byteorder::{ByteOrder, NetworkEndian, BigEndian};

let mut buf = [0; 2];
BigEndian::write_i16(&mut buf, -5_000);
assert_eq!(-5_000, NetworkEndian::read_i16(&buf));
```

[`BigEndian`]: enum.BigEndian.html

```rust
pub type NetworkEndian = BigEndian;
```

### Type Alias `NativeEndian`

**Attributes:**

- `#[<cfg>(target_endian = "little")]`

Defines system native-endian serialization.

Note that this type has no value constructor. It is used purely at the
type level.

On this platform, this is an alias for [`LittleEndian`].

[`LittleEndian`]: enum.LittleEndian.html

```rust
pub type NativeEndian = LittleEndian;
```

## Traits

### Trait `ByteOrder`

`ByteOrder` describes types that can serialize integers as bytes.

Note that `Self` does not appear anywhere in this trait's definition!
Therefore, in order to use it, you'll need to use syntax like
`T::read_u16(&[0, 1])` where `T` implements `ByteOrder`.

This crate provides two types that implement `ByteOrder`: [`BigEndian`]
and [`LittleEndian`].
This trait is sealed and cannot be implemented for callers to avoid
breaking backwards compatibility when adding new derived traits.

# Examples

Write and read `u32` numbers in little endian order:

```rust
use byteorder::{ByteOrder, LittleEndian};

let mut buf = [0; 4];
LittleEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
```

Write and read `i16` numbers in big endian order:

```rust
use byteorder::{ByteOrder, BigEndian};

let mut buf = [0; 2];
BigEndian::write_i16(&mut buf, -5_000);
assert_eq!(-5_000, BigEndian::read_i16(&buf));
```

[`BigEndian`]: enum.BigEndian.html
[`LittleEndian`]: enum.LittleEndian.html

```rust
pub trait ByteOrder: Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + private::Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `read_u16`: Reads an unsigned 16 bit integer from `buf`.
- `read_u32`: Reads an unsigned 32 bit integer from `buf`.
- `read_u64`: Reads an unsigned 64 bit integer from `buf`.
- `read_u128`: Reads an unsigned 128 bit integer from `buf`.
- `read_uint`: Reads an unsigned n-bytes integer from `buf`.
- `read_uint128`: Reads an unsigned n-bytes integer from `buf`.
- `write_u16`: Writes an unsigned 16 bit integer `n` to `buf`.
- `write_u32`: Writes an unsigned 32 bit integer `n` to `buf`.
- `write_u64`: Writes an unsigned 64 bit integer `n` to `buf`.
- `write_u128`: Writes an unsigned 128 bit integer `n` to `buf`.
- `write_uint`: Writes an unsigned integer `n` to `buf` using only `nbytes`.
- `write_uint128`: Writes an unsigned integer `n` to `buf` using only `nbytes`.
- `read_u16_into`: Reads unsigned 16 bit integers from `src` into `dst`.
- `read_u32_into`: Reads unsigned 32 bit integers from `src` into `dst`.
- `read_u64_into`: Reads unsigned 64 bit integers from `src` into `dst`.
- `read_u128_into`: Reads unsigned 128 bit integers from `src` into `dst`.
- `write_u16_into`: Writes unsigned 16 bit integers from `src` into `dst`.
- `write_u32_into`: Writes unsigned 32 bit integers from `src` into `dst`.
- `write_u64_into`: Writes unsigned 64 bit integers from `src` into `dst`.
- `write_u128_into`: Writes unsigned 128 bit integers from `src` into `dst`.
- `from_slice_u16`: Converts the given slice of unsigned 16 bit integers to a particular
- `from_slice_u32`: Converts the given slice of unsigned 32 bit integers to a particular
- `from_slice_u64`: Converts the given slice of unsigned 64 bit integers to a particular
- `from_slice_u128`: Converts the given slice of unsigned 128 bit integers to a particular
- `from_slice_f32`: Converts the given slice of IEEE754 single-precision (4 bytes) floating
- `from_slice_f64`: Converts the given slice of IEEE754 double-precision (8 bytes) floating

#### Provided Methods

- ```rust
  fn read_u24(buf: &[u8]) -> u32 { /* ... */ }
  ```
  Reads an unsigned 24 bit integer from `buf`, stored in u32.

- ```rust
  fn read_u48(buf: &[u8]) -> u64 { /* ... */ }
  ```
  Reads an unsigned 48 bit integer from `buf`, stored in u64.

- ```rust
  fn write_u24(buf: &mut [u8], n: u32) { /* ... */ }
  ```
  Writes an unsigned 24 bit integer `n` to `buf`, stored in u32.

- ```rust
  fn write_u48(buf: &mut [u8], n: u64) { /* ... */ }
  ```
  Writes an unsigned 48 bit integer `n` to `buf`, stored in u64.

- ```rust
  fn read_i16(buf: &[u8]) -> i16 { /* ... */ }
  ```
  Reads a signed 16 bit integer from `buf`.

- ```rust
  fn read_i24(buf: &[u8]) -> i32 { /* ... */ }
  ```
  Reads a signed 24 bit integer from `buf`, stored in i32.

- ```rust
  fn read_i32(buf: &[u8]) -> i32 { /* ... */ }
  ```
  Reads a signed 32 bit integer from `buf`.

- ```rust
  fn read_i48(buf: &[u8]) -> i64 { /* ... */ }
  ```
  Reads a signed 48 bit integer from `buf`, stored in i64.

- ```rust
  fn read_i64(buf: &[u8]) -> i64 { /* ... */ }
  ```
  Reads a signed 64 bit integer from `buf`.

- ```rust
  fn read_i128(buf: &[u8]) -> i128 { /* ... */ }
  ```
  Reads a signed 128 bit integer from `buf`.

- ```rust
  fn read_int(buf: &[u8], nbytes: usize) -> i64 { /* ... */ }
  ```
  Reads a signed n-bytes integer from `buf`.

- ```rust
  fn read_int128(buf: &[u8], nbytes: usize) -> i128 { /* ... */ }
  ```
  Reads a signed n-bytes integer from `buf`.

- ```rust
  fn read_f32(buf: &[u8]) -> f32 { /* ... */ }
  ```
  Reads a IEEE754 single-precision (4 bytes) floating point number.

- ```rust
  fn read_f64(buf: &[u8]) -> f64 { /* ... */ }
  ```
  Reads a IEEE754 double-precision (8 bytes) floating point number.

- ```rust
  fn write_i16(buf: &mut [u8], n: i16) { /* ... */ }
  ```
  Writes a signed 16 bit integer `n` to `buf`.

- ```rust
  fn write_i24(buf: &mut [u8], n: i32) { /* ... */ }
  ```
  Writes a signed 24 bit integer `n` to `buf`, stored in i32.

- ```rust
  fn write_i32(buf: &mut [u8], n: i32) { /* ... */ }
  ```
  Writes a signed 32 bit integer `n` to `buf`.

- ```rust
  fn write_i48(buf: &mut [u8], n: i64) { /* ... */ }
  ```
  Writes a signed 48 bit integer `n` to `buf`, stored in i64.

- ```rust
  fn write_i64(buf: &mut [u8], n: i64) { /* ... */ }
  ```
  Writes a signed 64 bit integer `n` to `buf`.

- ```rust
  fn write_i128(buf: &mut [u8], n: i128) { /* ... */ }
  ```
  Writes a signed 128 bit integer `n` to `buf`.

- ```rust
  fn write_int(buf: &mut [u8], n: i64, nbytes: usize) { /* ... */ }
  ```
  Writes a signed integer `n` to `buf` using only `nbytes`.

- ```rust
  fn write_int128(buf: &mut [u8], n: i128, nbytes: usize) { /* ... */ }
  ```
  Writes a signed integer `n` to `buf` using only `nbytes`.

- ```rust
  fn write_f32(buf: &mut [u8], n: f32) { /* ... */ }
  ```
  Writes a IEEE754 single-precision (4 bytes) floating point number.

- ```rust
  fn write_f64(buf: &mut [u8], n: f64) { /* ... */ }
  ```
  Writes a IEEE754 double-precision (8 bytes) floating point number.

- ```rust
  fn read_i16_into(src: &[u8], dst: &mut [i16]) { /* ... */ }
  ```
  Reads signed 16 bit integers from `src` to `dst`.

- ```rust
  fn read_i32_into(src: &[u8], dst: &mut [i32]) { /* ... */ }
  ```
  Reads signed 32 bit integers from `src` into `dst`.

- ```rust
  fn read_i64_into(src: &[u8], dst: &mut [i64]) { /* ... */ }
  ```
  Reads signed 64 bit integers from `src` into `dst`.

- ```rust
  fn read_i128_into(src: &[u8], dst: &mut [i128]) { /* ... */ }
  ```
  Reads signed 128 bit integers from `src` into `dst`.

- ```rust
  fn read_f32_into(src: &[u8], dst: &mut [f32]) { /* ... */ }
  ```
  Reads IEEE754 single-precision (4 bytes) floating point numbers from

- ```rust
  fn read_f32_into_unchecked(src: &[u8], dst: &mut [f32]) { /* ... */ }
  ```
  **DEPRECATED**.

- ```rust
  fn read_f64_into(src: &[u8], dst: &mut [f64]) { /* ... */ }
  ```
  Reads IEEE754 single-precision (4 bytes) floating point numbers from

- ```rust
  fn read_f64_into_unchecked(src: &[u8], dst: &mut [f64]) { /* ... */ }
  ```
  **DEPRECATED**.

- ```rust
  fn write_i8_into(src: &[i8], dst: &mut [u8]) { /* ... */ }
  ```
  Writes signed 8 bit integers from `src` into `dst`.

- ```rust
  fn write_i16_into(src: &[i16], dst: &mut [u8]) { /* ... */ }
  ```
  Writes signed 16 bit integers from `src` into `dst`.

- ```rust
  fn write_i32_into(src: &[i32], dst: &mut [u8]) { /* ... */ }
  ```
  Writes signed 32 bit integers from `src` into `dst`.

- ```rust
  fn write_i64_into(src: &[i64], dst: &mut [u8]) { /* ... */ }
  ```
  Writes signed 64 bit integers from `src` into `dst`.

- ```rust
  fn write_i128_into(src: &[i128], dst: &mut [u8]) { /* ... */ }
  ```
  Writes signed 128 bit integers from `src` into `dst`.

- ```rust
  fn write_f32_into(src: &[f32], dst: &mut [u8]) { /* ... */ }
  ```
  Writes IEEE754 single-precision (4 bytes) floating point numbers from

- ```rust
  fn write_f64_into(src: &[f64], dst: &mut [u8]) { /* ... */ }
  ```
  Writes IEEE754 double-precision (8 bytes) floating point numbers from

- ```rust
  fn from_slice_i16(src: &mut [i16]) { /* ... */ }
  ```
  Converts the given slice of signed 16 bit integers to a particular

- ```rust
  fn from_slice_i32(src: &mut [i32]) { /* ... */ }
  ```
  Converts the given slice of signed 32 bit integers to a particular

- ```rust
  fn from_slice_i64(src: &mut [i64]) { /* ... */ }
  ```
  Converts the given slice of signed 64 bit integers to a particular

- ```rust
  fn from_slice_i128(src: &mut [i128]) { /* ... */ }
  ```
  Converts the given slice of signed 128 bit integers to a particular

#### Implementations

This trait is implemented for the following types:

- `BigEndian`
- `LittleEndian`

## Re-exports

### Re-export `ReadBytesExt`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::io::ReadBytesExt;
```

### Re-export `WriteBytesExt`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::io::WriteBytesExt;
```

