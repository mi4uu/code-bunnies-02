# Crate Documentation

**Version:** 0.5.2

**Format Version:** 43

# Module `rend`

# rend

rend provides cross-platform, endian-aware primitives for Rust.

rend does not provide cross-platform alternatives for types that are
inherently cross-platform, such as `bool` and `u8`. It also does not provide
cross-platform alternatives for types that have an architecture-dependent
size, such as `isize` and `usize`. rend does not support custom types.

rend is intended to be used to build portable types that can be shared
between different architectures.

## Features

- `bytecheck`: Enables support for validating types using `bytecheck`.

## Crates

- `zerocopy-0_8`

## Example:
```rust
use core::mem::transmute;
use rend::*;

let little_int = i32_le::from_native(0x12345678);
// Internal representation is little-endian
assert_eq!(
    [0x78, 0x56, 0x34, 0x12],
    unsafe { transmute::<_, [u8; 4]>(little_int) }
);

// Can also be made with `.into()`
let little_int: i32_le = 0x12345678.into();
// Still formats correctly
assert_eq!("305419896", format!("{}", little_int));
assert_eq!("0x12345678", format!("0x{:x}", little_int));

let big_int = i32_be::from_native(0x12345678);
// Internal representation is big-endian
assert_eq!(
    [0x12, 0x34, 0x56, 0x78],
    unsafe { transmute::<_, [u8; 4]>(big_int) }
);

// Can also be made with `.into()`
let big_int: i32_be = 0x12345678.into();
// Still formats correctly
assert_eq!("305419896", format!("{}", big_int));
assert_eq!("0x12345678", format!("0x{:x}", big_int));
```

## Modules

## Module `unaligned`

Cross-platform primitives with unaligned representations.

```rust
pub mod unaligned { /* ... */ }
```

### Types

#### Struct `i16_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`.

```rust
pub struct i16_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: i16) -> Self { /* ... */ }
  ```
  Returns a `i16_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i16 { /* ... */ }
  ```
  Returns a `i16` with the same value as `self`.

###### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16_ule) -> bool { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Add**
  - ```rust
    fn add(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **Freeze**
- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **Shr**
  - ```rust
    fn shr(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **RefUnwindSafe**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **Pointee**
- **Unpin**
- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i16_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16_ule) -> Self { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i16_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i16_ule) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `i16_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`.

```rust
pub struct i16_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: i16) -> Self { /* ... */ }
  ```
  Returns a `i16_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i16 { /* ... */ }
  ```
  Returns a `i16` with the same value as `self`.

###### Trait Implementations

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16_ube) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i16_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16_ube) -> Self { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **Rem**
  - ```rust
    fn rem(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **Send**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Div**
  - ```rust
    fn div(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i16_ube) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i16_ube) { /* ... */ }
    ```

#### Struct `i32_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct i32_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: i32) -> Self { /* ... */ }
  ```
  Returns a `i32_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i32 { /* ... */ }
  ```
  Returns a `i32` with the same value as `self`.

###### Trait Implementations

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i32_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i32_ule) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Add**
  - ```rust
    fn add(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32_ule) -> bool { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **RefUnwindSafe**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i32_ule) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **Eq**
- **Shr**
  - ```rust
    fn shr(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i32_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i32_ule) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `i32_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct i32_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: i32) -> Self { /* ... */ }
  ```
  Returns a `i32_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i32 { /* ... */ }
  ```
  Returns a `i32` with the same value as `self`.

###### Trait Implementations

- **Sync**
- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Sub**
  - ```rust
    fn sub(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i32_ube) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i32_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i32_ube) -> Self { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32_ube) -> bool { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **Send**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i32_ube) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i32_ube) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_ube) -> <Self as >::Output { /* ... */ }
    ```

#### Struct `i64_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct i64_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: i64) -> Self { /* ... */ }
  ```
  Returns a `i64_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i64 { /* ... */ }
  ```
  Returns a `i64` with the same value as `self`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i64_ule) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Shr**
  - ```rust
    fn shr(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i64_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i64_ule) -> Self { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **Mul**
  - ```rust
    fn mul(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Pointee**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64_ule) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i64_ule) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i64_ule) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `i64_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct i64_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: i64) -> Self { /* ... */ }
  ```
  Returns a `i64_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i64 { /* ... */ }
  ```
  Returns a `i64` with the same value as `self`.

###### Trait Implementations

- **Sub**
  - ```rust
    fn sub(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i64_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i64_ube) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64_ube) -> bool { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **Freeze**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Copy**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **Pointee**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Eq**
- **Shr**
  - ```rust
    fn shr(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i64_ube) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i64_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i64_ube) { /* ... */ }
    ```

#### Struct `i128_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`.

```rust
pub struct i128_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: i128) -> Self { /* ... */ }
  ```
  Returns a `i128_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i128 { /* ... */ }
  ```
  Returns a `i128` with the same value as `self`.

###### Trait Implementations

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Pointee**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Add**
  - ```rust
    fn add(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128_ule) -> bool { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **Send**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i128_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i128_ule) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i128_ule) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i128_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i128_ule) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

#### Struct `i128_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`.

```rust
pub struct i128_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: i128) -> Self { /* ... */ }
  ```
  Returns a `i128_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i128 { /* ... */ }
  ```
  Returns a `i128` with the same value as `self`.

###### Trait Implementations

- **Mul**
  - ```rust
    fn mul(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i128_ube) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
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

  - ```rust
    fn from(value: i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i128_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i128_ube) -> Self { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128_ube) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sub**
  - ```rust
    fn sub(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **Freeze**
- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **Copy**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Pointee**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i128_ube) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i128_ube) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `u16_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`.

```rust
pub struct u16_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: u16) -> Self { /* ... */ }
  ```
  Returns a `u16_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u16 { /* ... */ }
  ```
  Returns a `u16` with the same value as `self`.

###### Trait Implementations

- **Rem**
  - ```rust
    fn rem(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **Pointee**
- **Add**
  - ```rust
    fn add(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Unpin**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **Sub**
  - ```rust
    fn sub(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u16_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16_ule) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Copy**
- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16_ule) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **UnwindSafe**
- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **Eq**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u16_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u16_ule) { /* ... */ }
    ```

#### Struct `u16_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`.

```rust
pub struct u16_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: u16) -> Self { /* ... */ }
  ```
  Returns a `u16_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u16 { /* ... */ }
  ```
  Returns a `u16` with the same value as `self`.

###### Trait Implementations

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16_ube) -> bool { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u16_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16_ube) -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Eq**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Send**
- **Rem**
  - ```rust
    fn rem(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u16_ube) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u16_ube) { /* ... */ }
    ```

#### Struct `u32_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct u32_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: u32) -> Self { /* ... */ }
  ```
  Returns a `u32_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u32 { /* ... */ }
  ```
  Returns a `u32` with the same value as `self`.

###### Trait Implementations

- **UnwindSafe**
- **Sub**
  - ```rust
    fn sub(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u32_ule) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Sync**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Pointee**
- **Div**
  - ```rust
    fn div(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u32_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u32_ule) -> Self { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32_ule) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u32_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u32_ule) { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

#### Struct `u32_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct u32_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: u32) -> Self { /* ... */ }
  ```
  Returns a `u32_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u32 { /* ... */ }
  ```
  Returns a `u32` with the same value as `self`.

###### Trait Implementations

- **Mul**
  - ```rust
    fn mul(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **Eq**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u32_ube) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u32_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u32_ube) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32_ube) -> bool { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u32_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u32_ube) { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
#### Struct `u64_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct u64_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: u64) -> Self { /* ... */ }
  ```
  Returns a `u64_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u64 { /* ... */ }
  ```
  Returns a `u64` with the same value as `self`.

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64_ule) -> bool { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
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
    fn from(value: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u64_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u64_ule) -> Self { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Eq**
- **Mul**
  - ```rust
    fn mul(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u64_ule) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Rem**
  - ```rust
    fn rem(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **Freeze**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **Pointee**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u64_ule) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u64_ule) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

#### Struct `u64_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct u64_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: u64) -> Self { /* ... */ }
  ```
  Returns a `u64_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u64 { /* ... */ }
  ```
  Returns a `u64` with the same value as `self`.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u64_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u64_ube) -> Self { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Sync**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Div**
  - ```rust
    fn div(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Send**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
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

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u64_ube) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64_ube) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **Pointee**
- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Unpin**
- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u64_ube) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u64_ube) { /* ... */ }
    ```

#### Struct `u128_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`.

```rust
pub struct u128_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: u128) -> Self { /* ... */ }
  ```
  Returns a `u128_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u128 { /* ... */ }
  ```
  Returns a `u128` with the same value as `self`.

###### Trait Implementations

- **Freeze**
- **Add**
  - ```rust
    fn add(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u128_ule) -> bool { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **Unpin**
- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Send**
- **Pointee**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u128_ule) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u128_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u128_ule) -> Self { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u128_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u128_ule) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Shr**
  - ```rust
    fn shr(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_ule) -> <Self as >::Output { /* ... */ }
    ```

#### Struct `u128_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`.

```rust
pub struct u128_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: u128) -> Self { /* ... */ }
  ```
  Returns a `u128_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u128 { /* ... */ }
  ```
  Returns a `u128` with the same value as `self`.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u128_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u128_ube) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u128_ube) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

- **UnwindSafe**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Eq**
- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
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

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
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

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Pointee**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u128_ube) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u128_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u128_ube) { /* ... */ }
    ```

#### Struct `f32_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct f32_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: f32) -> Self { /* ... */ }
  ```
  Returns a `f32_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> f32 { /* ... */ }
  ```
  Returns a `f32` with the same value as `self`.

###### Trait Implementations

- **Rem**
  - ```rust
    fn rem(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f32_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f32_ule) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Copy**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Pointee**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32_ule) -> bool { /* ... */ }
    ```

- **Eq**
- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: f32_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f32_ule) { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &f32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: f32_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f32_ule) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: f32_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f32_ule) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: f32_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f32_ule) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: f32_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f32_ule) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

#### Struct `f32_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct f32_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: f32) -> Self { /* ... */ }
  ```
  Returns a `f32_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> f32 { /* ... */ }
  ```
  Returns a `f32` with the same value as `self`.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32_ube) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: f32_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f32_ube) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Sub**
  - ```rust
    fn sub(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: f32_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f32_ube) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &f32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Add**
  - ```rust
    fn add(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: f32_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f32_ube) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **Pointee**
- **Send**
- **Copy**
- **Eq**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f32_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f32_ube) -> Self { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: f32_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f32_ube) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: f32_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f32_ube) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

#### Struct `f64_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct f64_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: f64) -> Self { /* ... */ }
  ```
  Returns a `f64_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> f64 { /* ... */ }
  ```
  Returns a `f64` with the same value as `self`.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: f64_ule) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f64_ule) { /* ... */ }
    ```

- **Send**
- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: f64_ule) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f64_ule) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64_ule) -> bool { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f64_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f64_ule) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Copy**
- **Rem**
  - ```rust
    fn rem(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &f64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: f64_ule) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f64_ule) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: f64_ule) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f64_ule) { /* ... */ }
    ```

- **Unpin**
- **Add**
  - ```rust
    fn add(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: f64_ule) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f64_ule) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `f64_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout, zerocopy_derive
:: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct f64_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: f64) -> Self { /* ... */ }
  ```
  Returns a `f64_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> f64 { /* ... */ }
  ```
  Returns a `f64` with the same value as `self`.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: f64_ube) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f64_ube) { /* ... */ }
    ```

- **Sync**
- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: f64_ube) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f64_ube) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: f64_ube) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f64_ube) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Add**
  - ```rust
    fn add(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **RefUnwindSafe**
- **Mul**
  - ```rust
    fn mul(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: f64_ube) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f64_ube) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64_ube) -> bool { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: f64_ube) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f64_ube) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &f64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f64_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f64_ube) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `char_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: IntoBytes, zerocopy_derive :: Immutable,
zerocopy_derive :: KnownLayout, zerocopy_derive :: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct char_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: char) -> Self { /* ... */ }
  ```
  Returns a `char_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> char { /* ... */ }
  ```
  Returns a `$prim` with the same value as `self`.

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &char) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Pointee**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Sync**
- **Unpin**
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

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: char_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a char_ule) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &char) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &char_ule) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

#### Struct `char_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: IntoBytes, zerocopy_derive :: Immutable,
zerocopy_derive :: KnownLayout, zerocopy_derive :: Unaligned,),)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct char_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_native(value: char) -> Self { /* ... */ }
  ```
  Returns a `char_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> char { /* ... */ }
  ```
  Returns a `$prim` with the same value as `self`.

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: char_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a char_ube) -> Self { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &char) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &char) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &char_ube) -> bool { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **Eq**
#### Struct `NonZeroI16_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`.

```rust
pub struct NonZeroI16_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: i16) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i16) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i16 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI16) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI16_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI16 { /* ... */ }
  ```
  Returns a `NonZeroI16` with the same value as `self`.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI16_ule) -> bool { /* ... */ }
    ```

- **Send**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI16_ule) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Pointee**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI16_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI16_ule) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

#### Struct `NonZeroI16_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`.

```rust
pub struct NonZeroI16_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: i16) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i16) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i16 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI16) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI16_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI16 { /* ... */ }
  ```
  Returns a `NonZeroI16` with the same value as `self`.

###### Trait Implementations

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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI16_ube) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI16_ube) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI16_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI16_ube) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Pointee**
- **Unpin**
#### Struct `NonZeroI32_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`.

```rust
pub struct NonZeroI32_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: i32) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i32) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i32 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI32) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI32_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI32 { /* ... */ }
  ```
  Returns a `NonZeroI32` with the same value as `self`.

###### Trait Implementations

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

- **Copy**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI32_ule) -> bool { /* ... */ }
    ```

- **Eq**
- **Pointee**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI32_ule) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI32_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI32_ule) -> Self { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `NonZeroI32_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`.

```rust
pub struct NonZeroI32_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: i32) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i32) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i32 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI32) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI32_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI32 { /* ... */ }
  ```
  Returns a `NonZeroI32` with the same value as `self`.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Eq**
- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI32_ube) -> bool { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI32_ube) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

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

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI32_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI32_ube) -> Self { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
#### Struct `NonZeroI64_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct NonZeroI64_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: i64) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i64) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i64 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI64) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI64_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI64 { /* ... */ }
  ```
  Returns a `NonZeroI64` with the same value as `self`.

###### Trait Implementations

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI64_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI64_ule) -> Self { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI64_ule) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Pointee**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI64_ule) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
#### Struct `NonZeroI64_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct NonZeroI64_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: i64) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i64) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i64 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI64) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI64_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI64 { /* ... */ }
  ```
  Returns a `NonZeroI64` with the same value as `self`.

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

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Eq**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI64_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI64_ube) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI64_ube) -> bool { /* ... */ }
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
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI64_ube) { /* ... */ }
    ```

#### Struct `NonZeroI128_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct NonZeroI128_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: i128) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i128) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i128 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI128) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI128_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI128 { /* ... */ }
  ```
  Returns a `NonZeroI128` with the same value as `self`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Copy**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Unpin**
- **Pointee**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI128_ule) -> bool { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Send**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI128_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI128_ule) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI128_ule) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `NonZeroI128_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`.

```rust
pub struct NonZeroI128_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: i128) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i128) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i128 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI128) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI128_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI128 { /* ... */ }
  ```
  Returns a `NonZeroI128` with the same value as `self`.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Pointee**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI128_ube) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI128_ube) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Send**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI128_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI128_ube) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
#### Struct `NonZeroU16_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct NonZeroU16_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: u16) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u16) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u16 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU16) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU16_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU16 { /* ... */ }
  ```
  Returns a `NonZeroU16` with the same value as `self`.

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Pointee**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU16_ule) -> bool { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU16_ule) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU16_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU16_ule) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
#### Struct `NonZeroU16_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct NonZeroU16_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: u16) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u16) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u16 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU16) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU16_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU16 { /* ... */ }
  ```
  Returns a `NonZeroU16` with the same value as `self`.

###### Trait Implementations

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_ube) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU16_ube) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU16_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU16_ube) -> Self { /* ... */ }
    ```

- **Unpin**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Eq**
- **Send**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU16_ube) { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `NonZeroU32_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct NonZeroU32_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: u32) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u32) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u32 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU32) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU32_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU32 { /* ... */ }
  ```
  Returns a `NonZeroU32` with the same value as `self`.

###### Trait Implementations

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU32_ule) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU32_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU32_ule) -> Self { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **Pointee**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_ule) -> <Self as >::Output { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU32_ule) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

#### Struct `NonZeroU32_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`.

```rust
pub struct NonZeroU32_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: u32) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u32) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u32 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU32) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU32_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU32 { /* ... */ }
  ```
  Returns a `NonZeroU32` with the same value as `self`.

###### Trait Implementations

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU32_ube) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU32_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU32_ube) -> Self { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **Sync**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Pointee**
- **Copy**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU32_ube) -> bool { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

#### Struct `NonZeroU64_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`.

```rust
pub struct NonZeroU64_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: u64) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u64) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u64 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU64) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU64_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU64 { /* ... */ }
  ```
  Returns a `NonZeroU64` with the same value as `self`.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_ule) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Pointee**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU64_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU64_ule) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU64_ule) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU64_ule) -> bool { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

#### Struct `NonZeroU64_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`.

```rust
pub struct NonZeroU64_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: u64) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u64) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u64 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU64) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU64_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU64 { /* ... */ }
  ```
  Returns a `NonZeroU64` with the same value as `self`.

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU64_ube) -> bool { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU64_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU64_ube) -> Self { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU64_ube) { /* ... */ }
    ```

#### Struct `NonZeroU128_ule`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A little-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`.

```rust
pub struct NonZeroU128_ule(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: u128) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u128) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u128 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU128) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU128_ule` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU128 { /* ... */ }
  ```
  Returns a `NonZeroU128` with the same value as `self`.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU128_ule) { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

  - ```rust
    fn from(value: NonZeroU128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU128_ule) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU128_ule) -> Self { /* ... */ }
    ```

- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_ule) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_ule) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU128_ule) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `NonZeroU128_ube`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[repr(C, packed(1))]`

A big-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`.

```rust
pub struct NonZeroU128_ube(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(value: u128) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u128) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u128 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU128) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU128_ube` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU128 { /* ... */ }
  ```
  Returns a `NonZeroU128` with the same value as `self`.

###### Trait Implementations

- **Unpin**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Pointee**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_ube) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_ube) -> <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU128_ube) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU128_ube) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU128_ube) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU128_ube) -> Self { /* ... */ }
    ```

## Types

### Struct `i16_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A little-endian `i16` with a guaranteed size and alignment of `2`.

```rust
pub struct i16_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: i16) -> Self { /* ... */ }
  ```
  Returns a `i16_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i16 { /* ... */ }
  ```
  Returns a `i16` with the same value as `self`.

##### Trait Implementations

- **Sub**
  - ```rust
    fn sub(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16_le) -> bool { /* ... */ }
    ```

- **Copy**
- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Freeze**
- **Mul**
  - ```rust
    fn mul(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **Eq**
- **Div**
  - ```rust
    fn div(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i16_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16_le) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
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
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i16_le) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i16_le) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_le) -> <Self as >::Output { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
### Struct `i16_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A big-endian `i16` with a guaranteed size and alignment of `2`.

```rust
pub struct i16_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: i16) -> Self { /* ... */ }
  ```
  Returns a `i16_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i16 { /* ... */ }
  ```
  Returns a `i16` with the same value as `self`.

##### Trait Implementations

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i16_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16_be) -> Self { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Send**
- **Sync**
- **Div**
  - ```rust
    fn div(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **Add**
  - ```rust
    fn add(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i16_be) -> <Self as >::Output { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Pointee**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i16_be) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i16_be) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16_be) -> bool { /* ... */ }
    ```

### Struct `i32_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A little-endian `i32` with a guaranteed size and alignment of `4`.

```rust
pub struct i32_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: i32) -> Self { /* ... */ }
  ```
  Returns a `i32_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i32 { /* ... */ }
  ```
  Returns a `i32` with the same value as `self`.

##### Trait Implementations

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32_le) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i32_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i32_le) -> Self { /* ... */ }
    ```

- **Copy**
- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Mul**
  - ```rust
    fn mul(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i32_le) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_le) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i32_le) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i32_le) { /* ... */ }
    ```

- **Pointee**
- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

### Struct `i32_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A big-endian `i32` with a guaranteed size and alignment of `4`.

```rust
pub struct i32_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: i32) -> Self { /* ... */ }
  ```
  Returns a `i32_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i32 { /* ... */ }
  ```
  Returns a `i32` with the same value as `self`.

##### Trait Implementations

- **Copy**
- **Freeze**
- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i32_be) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **Eq**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **Mul**
  - ```rust
    fn mul(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i32_be) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32_be) -> bool { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i32_be) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i32_be) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i32_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i32_be) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Pointee**
### Struct `i64_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A little-endian `i64` with a guaranteed size and alignment of `8`.

```rust
pub struct i64_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: i64) -> Self { /* ... */ }
  ```
  Returns a `i64_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i64 { /* ... */ }
  ```
  Returns a `i64` with the same value as `self`.

##### Trait Implementations

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64_le) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i64_le) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **Pointee**
- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **Copy**
- **Eq**
- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Add**
  - ```rust
    fn add(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i64_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i64_le) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i64_le) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i64_le) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

### Struct `i64_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A big-endian `i64` with a guaranteed size and alignment of `8`.

```rust
pub struct i64_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: i64) -> Self { /* ... */ }
  ```
  Returns a `i64_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i64 { /* ... */ }
  ```
  Returns a `i64` with the same value as `self`.

##### Trait Implementations

- **Add**
  - ```rust
    fn add(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **Sync**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i64_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i64_be) -> Self { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64_be) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i64_be) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **Eq**
- **Sub**
  - ```rust
    fn sub(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i64_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i64_be) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
### Struct `i128_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(16))]`

A little-endian `i128` with a guaranteed size and alignment of `16`.

```rust
pub struct i128_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: i128) -> Self { /* ... */ }
  ```
  Returns a `i128_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i128 { /* ... */ }
  ```
  Returns a `i128` with the same value as `self`.

##### Trait Implementations

- **Sub**
  - ```rust
    fn sub(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i128_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i128_le) -> Self { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Copy**
- **Sync**
- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i128_le) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128_le) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Pointee**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_le) -> <Self as >::Output { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Eq**
- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i128_le) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i128_le) { /* ... */ }
    ```

### Struct `i128_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(16))]`

A big-endian `i128` with a guaranteed size and alignment of `16`.

```rust
pub struct i128_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: i128) -> Self { /* ... */ }
  ```
  Returns a `i128_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> i128 { /* ... */ }
  ```
  Returns a `i128` with the same value as `self`.

##### Trait Implementations

- **Unpin**
- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Eq**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Pointee**
- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i128_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a i128_be) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i128_be) -> bool { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Div**
  - ```rust
    fn div(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &i128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: i128_be) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &i128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: i128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: i128_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &i128_be) { /* ... */ }
    ```

### Struct `u16_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A little-endian `u16` with a guaranteed size and alignment of `2`.

```rust
pub struct u16_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: u16) -> Self { /* ... */ }
  ```
  Returns a `u16_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u16 { /* ... */ }
  ```
  Returns a `u16` with the same value as `self`.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u16_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16_le) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16_le) -> bool { /* ... */ }
    ```

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u16_le) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u16_le) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_le) -> <Self as >::Output { /* ... */ }
    ```

### Struct `u16_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A big-endian `u16` with a guaranteed size and alignment of `2`.

```rust
pub struct u16_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: u16) -> Self { /* ... */ }
  ```
  Returns a `u16_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u16 { /* ... */ }
  ```
  Returns a `u16` with the same value as `self`.

##### Trait Implementations

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **UnwindSafe**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Mul**
  - ```rust
    fn mul(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **Pointee**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16_be) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u16_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16_be) -> Self { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u16_be) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u16) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u16_be) { /* ... */ }
    ```

### Struct `u32_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A little-endian `u32` with a guaranteed size and alignment of `4`.

```rust
pub struct u32_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: u32) -> Self { /* ... */ }
  ```
  Returns a `u32_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u32 { /* ... */ }
  ```
  Returns a `u32` with the same value as `self`.

##### Trait Implementations

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **Pointee**
- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

  - ```rust
    fn eq(self: &Self, other: &u32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32_le) -> bool { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u32_le) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Div**
  - ```rust
    fn div(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **RefUnwindSafe**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Copy**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u32_le) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u32_le) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u32_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u32_le) -> Self { /* ... */ }
    ```

### Struct `u32_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A big-endian `u32` with a guaranteed size and alignment of `4`.

```rust
pub struct u32_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: u32) -> Self { /* ... */ }
  ```
  Returns a `u32_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u32 { /* ... */ }
  ```
  Returns a `u32` with the same value as `self`.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u32_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u32_be) -> Self { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Pointee**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **Shl**
  - ```rust
    fn shl(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u32_be) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u32_be) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32_be) -> bool { /* ... */ }
    ```

- **Copy**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u32_be) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u32) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u32_be) { /* ... */ }
    ```

### Struct `u64_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A little-endian `u64` with a guaranteed size and alignment of `8`.

```rust
pub struct u64_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: u64) -> Self { /* ... */ }
  ```
  Returns a `u64_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u64 { /* ... */ }
  ```
  Returns a `u64` with the same value as `self`.

##### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Pointee**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u64_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u64_le) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u64_le) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64_le) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Shl**
  - ```rust
    fn shl(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_le) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u64_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u64_le) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
### Struct `u64_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A big-endian `u64` with a guaranteed size and alignment of `8`.

```rust
pub struct u64_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: u64) -> Self { /* ... */ }
  ```
  Returns a `u64_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u64 { /* ... */ }
  ```
  Returns a `u64` with the same value as `self`.

##### Trait Implementations

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Eq**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Add**
  - ```rust
    fn add(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **Shr**
  - ```rust
    fn shr(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64_be) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **UnwindSafe**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u64_be) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u64_be) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u64) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u64_be) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u64_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u64_be) -> Self { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

### Struct `u128_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(16))]`

A little-endian `u128` with a guaranteed size and alignment of `16`.

```rust
pub struct u128_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: u128) -> Self { /* ... */ }
  ```
  Returns a `u128_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u128 { /* ... */ }
  ```
  Returns a `u128` with the same value as `self`.

##### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u128_le) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **Eq**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Pointee**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u128_le) { /* ... */ }
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

  - ```rust
    fn eq(self: &Self, other: &u128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u128_le) -> bool { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Unpin**
- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u128_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u128_le) -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **Copy**
- **Div**
  - ```rust
    fn div(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **Mul**
  - ```rust
    fn mul(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u128_le) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u128_le) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_le) -> <Self as >::Output { /* ... */ }
    ```

### Struct `u128_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(16))]`

A big-endian `u128` with a guaranteed size and alignment of `16`.

```rust
pub struct u128_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: u128) -> Self { /* ... */ }
  ```
  Returns a `u128_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> u128 { /* ... */ }
  ```
  Returns a `u128` with the same value as `self`.

##### Trait Implementations

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **RefUnwindSafe**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u128_be) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u128_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a u128_be) -> Self { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u128_be) -> bool { /* ... */ }
    ```

- **Send**
- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &u128_be) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &u128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Eq**
- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: u128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: u128_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u128) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &u128_be) { /* ... */ }
    ```

- **Pointee**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

### Struct `f32_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A little-endian `f32` with a guaranteed size and alignment of `4`.

```rust
pub struct f32_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: f32) -> Self { /* ... */ }
  ```
  Returns a `f32_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> f32 { /* ... */ }
  ```
  Returns a `f32` with the same value as `self`.

##### Trait Implementations

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: f32_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f32_le) { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: f32_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f32_le) { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32_le) -> bool { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: f32_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f32_le) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Rem**
  - ```rust
    fn rem(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f32_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f32_le) -> Self { /* ... */ }
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
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &f32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: f32_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f32_le) { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: f32_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f32_le) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_le) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

### Struct `f32_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A big-endian `f32` with a guaranteed size and alignment of `4`.

```rust
pub struct f32_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: f32) -> Self { /* ... */ }
  ```
  Returns a `f32_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> f32 { /* ... */ }
  ```
  Returns a `f32` with the same value as `self`.

##### Trait Implementations

- **Sub**
  - ```rust
    fn sub(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

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

- **Eq**
- **Pointee**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: f32_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f32_be) { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: f32_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f32_be) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f32_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f32_be) -> Self { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &f32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f32_be) -> <Self as >::Output { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: f32_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f32_be) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32_be) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: f32_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f32_be) { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: f32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: f32_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f32) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f32_be) { /* ... */ }
    ```

### Struct `f64_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A little-endian `f64` with a guaranteed size and alignment of `8`.

```rust
pub struct f64_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: f64) -> Self { /* ... */ }
  ```
  Returns a `f64_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> f64 { /* ... */ }
  ```
  Returns a `f64` with the same value as `self`.

##### Trait Implementations

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: f64_le) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f64_le) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: f64_le) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f64_le) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f64_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f64_le) -> Self { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: f64_le) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f64_le) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &f64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Pointee**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: f64_le) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f64_le) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Div**
  - ```rust
    fn div(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: f64_le) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f64_le) { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_le) -> <Self as >::Output { /* ... */ }
    ```

- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64_le) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

### Struct `f64_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A big-endian `f64` with a guaranteed size and alignment of `8`.

```rust
pub struct f64_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: f64) -> Self { /* ... */ }
  ```
  Returns a `f64_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> f64 { /* ... */ }
  ```
  Returns a `f64` with the same value as `self`.

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sum**
  - ```rust
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Unpin**
- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: f64_be) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, other: &f64_be) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: f64_be) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, other: &f64_be) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: f64_be) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, other: &f64_be) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Rem**
  - ```rust
    fn rem(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f64_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a f64_be) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64_be) -> bool { /* ... */ }
    ```

- **Eq**
- **Send**
- **UpperExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Product**
  - ```rust
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { /* ... */ }
    ```

- **Pointee**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: f64_be) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, other: &f64_be) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Mul**
  - ```rust
    fn mul(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: f64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: f64_be) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f64) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: &f64_be) { /* ... */ }
    ```

- **Copy**
- **LowerExp**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &f64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: &f64_be) -> <Self as >::Output { /* ... */ }
    ```

### Struct `char_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: IntoBytes, zerocopy_derive :: Immutable,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A little-endian `u32` with a guaranteed size and alignment of `4`.

```rust
pub struct char_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: char) -> Self { /* ... */ }
  ```
  Returns a `char_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> char { /* ... */ }
  ```
  Returns a `$prim` with the same value as `self`.

##### Trait Implementations

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &char) -> Option<::core::cmp::Ordering> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: char_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a char_le) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &char) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &char_le) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
### Struct `char_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: IntoBytes, zerocopy_derive :: Immutable,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A big-endian `u32` with a guaranteed size and alignment of `4`.

```rust
pub struct char_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_native(value: char) -> Self { /* ... */ }
  ```
  Returns a `char_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> char { /* ... */ }
  ```
  Returns a `$prim` with the same value as `self`.

##### Trait Implementations

- **Freeze**
- **Pointee**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &char) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &char_be) -> bool { /* ... */ }
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

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &char) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: char_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a char_be) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

### Struct `NonZeroI16_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A little-endian `NonZeroI16` with a guaranteed size and alignment of `2`.

```rust
pub struct NonZeroI16_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i16) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i16) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i16 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI16) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI16_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI16 { /* ... */ }
  ```
  Returns a `NonZeroI16` with the same value as `self`.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI16_le) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI16_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI16_le) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_le) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI16_le) { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Pointee**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Eq**
- **Unpin**
- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
### Struct `NonZeroI16_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A big-endian `NonZeroI16` with a guaranteed size and alignment of `2`.

```rust
pub struct NonZeroI16_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i16) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i16) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i16 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI16) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI16_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI16 { /* ... */ }
  ```
  Returns a `NonZeroI16` with the same value as `self`.

##### Trait Implementations

- **Unpin**
- **Sync**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI16_be) -> <Self as >::Output { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI16_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI16_be) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Send**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI16_be) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI16_be) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

### Struct `NonZeroI32_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A little-endian `NonZeroI32` with a guaranteed size and alignment of `4`.

```rust
pub struct NonZeroI32_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i32) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i32) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i32 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI32) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI32_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI32 { /* ... */ }
  ```
  Returns a `NonZeroI32` with the same value as `self`.

##### Trait Implementations

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI32_le) -> bool { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI32_le) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Pointee**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Eq**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_le) -> <Self as >::Output { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI32_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI32_le) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `NonZeroI32_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A big-endian `NonZeroI32` with a guaranteed size and alignment of `4`.

```rust
pub struct NonZeroI32_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i32) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i32) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i32 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI32) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI32_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI32 { /* ... */ }
  ```
  Returns a `NonZeroI32` with the same value as `self`.

##### Trait Implementations

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Pointee**
- **Copy**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI32_be) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI32_be) -> bool { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

  - ```rust
    fn from(value: NonZeroI32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI32_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI32_be) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Eq**
### Struct `NonZeroI64_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A little-endian `NonZeroI64` with a guaranteed size and alignment of `8`.

```rust
pub struct NonZeroI64_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i64) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i64) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i64 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI64) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI64_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI64 { /* ... */ }
  ```
  Returns a `NonZeroI64` with the same value as `self`.

##### Trait Implementations

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI64_le) -> bool { /* ... */ }
    ```

- **Pointee**
- **Copy**
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI64_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI64_le) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI64_le) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_le) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `NonZeroI64_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A big-endian `NonZeroI64` with a guaranteed size and alignment of `8`.

```rust
pub struct NonZeroI64_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i64) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i64) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i64 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI64) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI64_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI64 { /* ... */ }
  ```
  Returns a `NonZeroI64` with the same value as `self`.

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI64_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI64_be) -> Self { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Sync**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI64_be) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI64_be) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

### Struct `NonZeroI128_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(16))]`

A little-endian `NonZeroI128` with a guaranteed size and alignment of `16`.

```rust
pub struct NonZeroI128_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i128) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i128) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i128 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI128) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI128_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI128 { /* ... */ }
  ```
  Returns a `NonZeroI128` with the same value as `self`.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI128_le) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI128_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI128_le) -> Self { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Copy**
- **Unpin**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI128_le) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_le) -> <Self as >::Output { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Eq**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Sync**
- **UnwindSafe**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

### Struct `NonZeroI128_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(16))]`

A big-endian `NonZeroI128` with a guaranteed size and alignment of `16`.

```rust
pub struct NonZeroI128_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i128) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: i128) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> i128 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroI128) -> Self { /* ... */ }
  ```
  Returns a `NonZeroI128_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroI128 { /* ... */ }
  ```
  Returns a `NonZeroI128` with the same value as `self`.

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
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

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroI128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroI128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroI128_be) { /* ... */ }
    ```

- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Pointee**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroI128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroI128_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroI128_be) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroI128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroI128_be) -> bool { /* ... */ }
    ```

### Struct `NonZeroU16_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A little-endian `NonZeroU16` with a guaranteed size and alignment of `2`.

```rust
pub struct NonZeroU16_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u16) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u16) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u16 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU16) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU16_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU16 { /* ... */ }
  ```
  Returns a `NonZeroU16` with the same value as `self`.

##### Trait Implementations

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU16_le) { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU16_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU16_le) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_le) -> <Self as >::Output { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU16_le) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Copy**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU16) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

### Struct `NonZeroU16_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A big-endian `NonZeroU16` with a guaranteed size and alignment of `2`.

```rust
pub struct NonZeroU16_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u16) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u16) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u16 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU16) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU16_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU16 { /* ... */ }
  ```
  Returns a `NonZeroU16` with the same value as `self`.

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU16_be) -> <Self as >::Output { /* ... */ }
    ```

- **Pointee**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU16) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU16_be) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU16_be) -> bool { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU16) -> Option<::core::cmp::Ordering> { /* ... */ }
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

  - ```rust
    fn from(value: NonZeroU16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU16_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU16_be) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
### Struct `NonZeroU32_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A little-endian `NonZeroU32` with a guaranteed size and alignment of `4`.

```rust
pub struct NonZeroU32_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u32) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u32) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u32 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU32) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU32_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU32 { /* ... */ }
  ```
  Returns a `NonZeroU32` with the same value as `self`.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointee**
- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU32_le) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_le) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
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

  - ```rust
    fn from(value: NonZeroU32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU32_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU32_le) -> Self { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU32_le) -> bool { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

### Struct `NonZeroU32_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A big-endian `NonZeroU32` with a guaranteed size and alignment of `4`.

```rust
pub struct NonZeroU32_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u32) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u32) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u32 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU32) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU32_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU32 { /* ... */ }
  ```
  Returns a `NonZeroU32` with the same value as `self`.

##### Trait Implementations

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU32_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU32_be) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU32) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU32_be) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU32) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU32_be) -> <Self as >::Output { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU32_be) -> bool { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `NonZeroU64_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A little-endian `NonZeroU64` with a guaranteed size and alignment of `8`.

```rust
pub struct NonZeroU64_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u64) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u64) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u64 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU64) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU64_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU64 { /* ... */ }
  ```
  Returns a `NonZeroU64` with the same value as `self`.

##### Trait Implementations

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU64_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU64_le) -> Self { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU64_le) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_le) -> <Self as >::Output { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU64_le) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
### Struct `NonZeroU64_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A big-endian `NonZeroU64` with a guaranteed size and alignment of `8`.

```rust
pub struct NonZeroU64_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u64) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u64) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u64 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU64) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU64_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU64 { /* ... */ }
  ```
  Returns a `NonZeroU64` with the same value as `self`.

##### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU64_be) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU64) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU64_be) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU64_be) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU64) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Eq**
- **Pointee**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU64_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU64_be) -> Self { /* ... */ }
    ```

### Struct `NonZeroU128_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(16))]`

A little-endian `NonZeroU128` with a guaranteed size and alignment of `16`.

```rust
pub struct NonZeroU128_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u128) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u128) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u128 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU128) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU128_le` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU128 { /* ... */ }
  ```
  Returns a `NonZeroU128` with the same value as `self`.

##### Trait Implementations

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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Pointee**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU128_le) -> bool { /* ... */ }
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

  - ```rust
    fn from(value: NonZeroU128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU128_le) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU128_le) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU128_le) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Send**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_le) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_le) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

### Struct `NonZeroU128_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: TryFromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: Immutable, zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(16))]`

A big-endian `NonZeroU128` with a guaranteed size and alignment of `16`.

```rust
pub struct NonZeroU128_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u128) -> Option<Self> { /* ... */ }
  ```
  Creates a non-zero if the given value is not zero.

- ```rust
  pub const unsafe fn new_unchecked(value: u128) -> Self { /* ... */ }
  ```
  Creates a non-zero without checking whether it is non-zero. This

- ```rust
  pub const fn get(self: Self) -> u128 { /* ... */ }
  ```
  Returns the value as a primitive type.

- ```rust
  pub const fn from_native(value: NonZeroU128) -> Self { /* ... */ }
  ```
  Returns a `NonZeroU128_be` containing `value`.

- ```rust
  pub const fn to_native(self: Self) -> NonZeroU128 { /* ... */ }
  ```
  Returns a `NonZeroU128` with the same value as `self`.

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: NonZeroU128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: NonZeroU128_be) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a NonZeroU128_be) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Copy**
- **Pointee**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering { /* ... */ }
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

  - ```rust
    fn eq(self: &Self, other: &NonZeroU128) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &NonZeroU128_be) -> bool { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU128) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, other: NonZeroU128_be) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: NonZeroU128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_be) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, other: &NonZeroU128_be) -> <Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &NonZeroU128) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
### Struct `AtomicI16_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A little-endian `AtomicI16` with a guaranteed size and alignment of `2`.

```rust
pub struct AtomicI16_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i16) -> Self { /* ... */ }
  ```
  Returns a `AtomicI16_le` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(i16) -> Option<i16>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i16, i16> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> i16 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i16 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i16, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
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
    fn from(value: i16) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **UnwindSafe**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

### Struct `AtomicI16_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A big-endian `AtomicI16` with a guaranteed size and alignment of `2`.

```rust
pub struct AtomicI16_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i16) -> Self { /* ... */ }
  ```
  Returns a `AtomicI16_be` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(i16) -> Option<i16>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i16, i16> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> i16 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i16 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i16, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i16) -> Self { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **Pointee**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

### Struct `AtomicU16_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A little-endian `AtomicU16` with a guaranteed size and alignment of `2`.

```rust
pub struct AtomicU16_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u16) -> Self { /* ... */ }
  ```
  Returns a `AtomicU16_le` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(u16) -> Option<u16>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u16, u16> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> u16 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u16 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u16, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u16) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `AtomicU16_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(2))]`

A big-endian `AtomicU16` with a guaranteed size and alignment of `2`.

```rust
pub struct AtomicU16_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u16) -> Self { /* ... */ }
  ```
  Returns a `AtomicU16_be` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(u16) -> Option<u16>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u16, u16> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> u16 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u16 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u16, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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

  - ```rust
    fn from(value: u16) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Pointee**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
### Struct `AtomicI32_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A little-endian `AtomicI32` with a guaranteed size and alignment of `4`.

```rust
pub struct AtomicI32_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i32) -> Self { /* ... */ }
  ```
  Returns a `AtomicI32_le` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(i32) -> Option<i32>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i32, i32> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> i32 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i32 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i32, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Pointee**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i32) -> Self { /* ... */ }
    ```

- **Sync**
### Struct `AtomicI32_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A big-endian `AtomicI32` with a guaranteed size and alignment of `4`.

```rust
pub struct AtomicI32_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i32) -> Self { /* ... */ }
  ```
  Returns a `AtomicI32_be` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(i32) -> Option<i32>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i32, i32> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> i32 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i32 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i32, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: i32) -> Self { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

### Struct `AtomicU32_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A little-endian `AtomicU32` with a guaranteed size and alignment of `4`.

```rust
pub struct AtomicU32_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u32) -> Self { /* ... */ }
  ```
  Returns a `AtomicU32_le` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(u32) -> Option<u32>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u32, u32> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> u32 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u32 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u32, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **Unpin**
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
    fn from(value: u32) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Pointee**
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

- **Send**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

### Struct `AtomicU32_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(4))]`

A big-endian `AtomicU32` with a guaranteed size and alignment of `4`.

```rust
pub struct AtomicU32_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u32) -> Self { /* ... */ }
  ```
  Returns a `AtomicU32_be` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(u32) -> Option<u32>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u32, u32> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> u32 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u32 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u32, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **Freeze**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

  - ```rust
    fn from(value: u32) -> Self { /* ... */ }
    ```

- **Pointee**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Send**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

### Struct `AtomicI64_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A little-endian `AtomicI64` with a guaranteed size and alignment of `8`.

```rust
pub struct AtomicI64_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i64) -> Self { /* ... */ }
  ```
  Returns a `AtomicI64_le` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(i64) -> Option<i64>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i64, i64> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> i64 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i64 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i64, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
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
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
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

  - ```rust
    fn from(value: i64) -> Self { /* ... */ }
    ```

- **Send**
- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `AtomicI64_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A big-endian `AtomicI64` with a guaranteed size and alignment of `8`.

```rust
pub struct AtomicI64_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: i64) -> Self { /* ... */ }
  ```
  Returns a `AtomicI64_be` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(i64) -> Option<i64>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i64, i64> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> i64 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i64 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i64, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

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
    fn from(value: i64) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Pointee**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `AtomicU64_le`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A little-endian `AtomicU64` with a guaranteed size and alignment of `8`.

```rust
pub struct AtomicU64_le(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u64) -> Self { /* ... */ }
  ```
  Returns a `AtomicU64_le` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(u64) -> Option<u64>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u64, u64> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> u64 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u64 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u64, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **Freeze**
- **Sync**
- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u64) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `AtomicU64_be`

**Attributes:**

- `#[allow(non_camel_case_types)]`
- `#[<cfg_attr>(feature = "zerocopy-0_8",
derive(zerocopy_derive :: FromBytes, zerocopy_derive :: IntoBytes,
zerocopy_derive :: KnownLayout,),)]`
- `#[repr(C, align(8))]`

A big-endian `AtomicU64` with a guaranteed size and alignment of `8`.

```rust
pub struct AtomicU64_be(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(value: u64) -> Self { /* ... */ }
  ```
  Returns a `AtomicU64_be` containing `value`.

- ```rust
  pub fn compare_exchange(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is

- ```rust
  pub fn fetch_add(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn fetch_and(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_max(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn fetch_update<F: FnMut(u64) -> Option<u64>>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u64, u64> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an

- ```rust
  pub fn fetch_xor(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn into_inner(self: Self) -> u64 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u64 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u64, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous

##### Trait Implementations

- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CheckBytes**
  - ```rust
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u64) -> Self { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

