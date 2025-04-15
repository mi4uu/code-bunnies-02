# Crate Documentation

**Version:** 0.8.1

**Format Version:** 43

# Module `bytecheck`

# bytecheck

bytecheck is a memory validation framework for Rust.

For some types, creating an invalid value immediately results in undefined
behavior. This can cause some issues when trying to validate potentially
invalid bytes, as just casting the bytes to your type can technically cause
errors. This makes it difficult to write validation routines, because until
you're certain that the bytes represent valid values you cannot cast them.

bytecheck provides a framework for performing these byte-level validations
and implements checks for basic types along with a derive macro to implement
validation for custom structs and enums.

## Design

[`CheckBytes`] is at the heart of bytecheck, and does the heavy lifting of
verifying that some bytes represent a valid type. Implementing it can be
done manually or automatically with the [derive macro](macro@CheckBytes).

## Layout stability

The layouts of types may change between compiler versions, or even different
compilations. To guarantee stable type layout between compilations, structs,
enums, and unions can be annotated with `#[repr(C)]`, and enums specifically
can be annotated with `#[repr(int)]` or `#[repr(C, int)]` as well. See
[the reference's page on type layout][reference] for more details.

[reference]: https://doc.rust-lang.org/reference/type-layout.html

## Features

- `derive`: Re-exports the macros from `bytecheck_derive`. Enabled by
  default.
- `simdutf8`: Uses the `simdutf8` crate to validate UTF-8 strings. Enabled
  by default.

### Crates

Bytecheck provides integrations for some common crates by default. In the
future, crates should depend on bytecheck and provide their own integration.

- [`uuid-1`](https://docs.rs/uuid/1)

## Example
```rust
use bytecheck::{CheckBytes, check_bytes, rancor::Failure};

#[derive(CheckBytes, Debug)]
#[repr(C)]
struct Test {
    a: u32,
    b: char,
    c: bool,
}

#[repr(C, align(4))]
struct Aligned<const N: usize>([u8; N]);

macro_rules! bytes {
    ($($byte:literal,)*) => {
        (&Aligned([$($byte,)*]).0 as &[u8]).as_ptr()
    };
    ($($byte:literal),*) => {
        bytes!($($byte,)*)
    };
}

// In this example, the architecture is assumed to be little-endian
#[cfg(target_endian = "little")]
unsafe {
    // These are valid bytes for a `Test`
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x78u8, 0u8, 0u8, 0u8,
            1u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap();

    // Changing the bytes for the u32 is OK, any bytes are a valid u32
    check_bytes::<Test, Failure>(
        bytes![
            42u8, 16u8, 20u8, 3u8,
            0x78u8, 0u8, 0u8, 0u8,
            1u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap();

    // Characters outside the valid ranges are invalid
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x00u8, 0xd8u8, 0u8, 0u8,
            1u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap_err();
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x00u8, 0x00u8, 0x11u8, 0u8,
            1u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap_err();

    // 0 is a valid boolean value (false) but 2 is not
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x78u8, 0u8, 0u8, 0u8,
            0u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap();
    check_bytes::<Test, Failure>(
        bytes![
            0u8, 0u8, 0u8, 0u8,
            0x78u8, 0u8, 0u8, 0u8,
            2u8, 255u8, 255u8, 255u8,
        ].cast()
    ).unwrap_err();
}
```

## Types

### Struct `StructCheckContext`

Context for errors resulting from invalid structs.

This context is used by the derive macro to trace which field of a struct
failed validation.

```rust
pub struct StructCheckContext {
    pub struct_name: &''static str,
    pub field_name: &''static str,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `struct_name` | `&''static str` | The name of the struct with an invalid field. |
| `field_name` | `&''static str` | The name of the struct field that was invalid. |

#### Implementations

##### Trait Implementations

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Unpin**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `TupleStructCheckContext`

Context for errors resulting from invalid tuple structs.

This context is used by the derive macro to trace which field of a tuple
struct failed validation.

```rust
pub struct TupleStructCheckContext {
    pub tuple_struct_name: &''static str,
    pub field_index: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tuple_struct_name` | `&''static str` | The name of the tuple struct with an invalid field. |
| `field_index` | `usize` | The index of the tuple struct field that was invalid. |

#### Implementations

##### Trait Implementations

- **Unpin**
- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `InvalidEnumDiscriminantError`

An error resulting from an invalid enum tag.

This context is used by the derive macro to trace what the invalid
discriminant for an enum is.

```rust
pub struct InvalidEnumDiscriminantError<T> {
    pub enum_name: &''static str,
    pub invalid_discriminant: T,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enum_name` | `&''static str` | The name of the enum with an invalid discriminant. |
| `invalid_discriminant` | `T` | The invalid value of the enum discriminant. |

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Error**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **UnwindSafe**
- **Pointee**
- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `NamedEnumVariantCheckContext`

Context for errors resulting from checking enum variants with named fields.

This context is used by the derive macro to trace which field of an enum
variant failed validation.

```rust
pub struct NamedEnumVariantCheckContext {
    pub enum_name: &''static str,
    pub variant_name: &''static str,
    pub field_name: &''static str,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enum_name` | `&''static str` | The name of the enum with an invalid variant. |
| `variant_name` | `&''static str` | The name of the variant that was invalid. |
| `field_name` | `&''static str` | The name of the field that was invalid. |

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Pointee**
- **Unpin**
- **Send**
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `UnnamedEnumVariantCheckContext`

Context for errors resulting from checking enum variants with unnamed
fields.

This context is used by the derive macro to trace which field of an enum
variant failed validation.

```rust
pub struct UnnamedEnumVariantCheckContext {
    pub enum_name: &''static str,
    pub variant_name: &''static str,
    pub field_index: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enum_name` | `&''static str` | The name of the enum with an invalid variant. |
| `variant_name` | `&''static str` | The name of the variant that was invalid. |
| `field_index` | `usize` | The name of the field that was invalid. |

#### Implementations

##### Trait Implementations

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **Pointee**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Traits

### Trait `CheckBytes`

A type that can check whether a pointer points to a valid value.

`CheckBytes` can be derived with [`CheckBytes`](macro@CheckBytes) or
implemented manually for custom behavior.

# Safety

`check_bytes` must only return `Ok` if `value` points to a valid instance of
`Self`. Because `value` must always be properly aligned for `Self` and point
to enough bytes to represent the type, this implies that `value` may be
dereferenced safely.

# Example

```
use core::{error::Error, fmt};

use bytecheck::CheckBytes;
use rancor::{fail, Fallible, Source};

#[repr(C, align(4))]
pub struct NonMaxU32(u32);

unsafe impl<C: Fallible + ?Sized> CheckBytes<C> for NonMaxU32
where
    C::Error: Source,
{
    unsafe fn check_bytes(
        value: *const Self,
        context: &mut C,
    ) -> Result<(), C::Error> {
        #[derive(Debug)]
        struct NonMaxCheckError;

        impl fmt::Display for NonMaxCheckError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "non-max u32 was set to u32::MAX")
            }
        }

        impl Error for NonMaxCheckError {}

        let value = unsafe { value.read() };
        if value.0 == u32::MAX {
            fail!(NonMaxCheckError);
        }

        Ok(())
    }
}
```

See [`Verify`] for an example which uses less unsafe.

```rust
pub unsafe trait CheckBytes<C: Fallible + ?Sized> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `check_bytes`: Checks whether the given pointer points to a valid value within the

#### Implementations

This trait is implemented for the following types:

- `()` with <C: Fallible + ?Sized>
- `i8` with <C: Fallible + ?Sized>
- `i16` with <C: Fallible + ?Sized>
- `i32` with <C: Fallible + ?Sized>
- `i64` with <C: Fallible + ?Sized>
- `i128` with <C: Fallible + ?Sized>
- `u8` with <C: Fallible + ?Sized>
- `u16` with <C: Fallible + ?Sized>
- `u32` with <C: Fallible + ?Sized>
- `u64` with <C: Fallible + ?Sized>
- `u128` with <C: Fallible + ?Sized>
- `f32` with <C: Fallible + ?Sized>
- `f64` with <C: Fallible + ?Sized>
- `core::sync::atomic::AtomicI8` with <C: Fallible + ?Sized>
- `core::sync::atomic::AtomicU8` with <C: Fallible + ?Sized>
- `core::sync::atomic::AtomicI16` with <C: Fallible + ?Sized>
- `core::sync::atomic::AtomicU16` with <C: Fallible + ?Sized>
- `core::sync::atomic::AtomicI32` with <C: Fallible + ?Sized>
- `core::sync::atomic::AtomicU32` with <C: Fallible + ?Sized>
- `core::sync::atomic::AtomicI64` with <C: Fallible + ?Sized>
- `core::sync::atomic::AtomicU64` with <C: Fallible + ?Sized>
- `core::marker::PhantomData<T>` with <T: ?Sized, C: Fallible + ?Sized>
- `core::marker::PhantomPinned` with <C: Fallible + ?Sized>
- `core::mem::ManuallyDrop<T>` with <T, C>
- `core::cell::UnsafeCell<T>` with <T, C>
- `core::cell::Cell<T>` with <T, C>
- `bool` with <C>
- `core::sync::atomic::AtomicBool` with <C>
- `char` with <C>
- `(T0)` with <T0, C>
- `(T0, T1)` with <T0, T1, C>
- `(T0, T1, T2)` with <T0, T1, T2, C>
- `(T0, T1, T2, T3)` with <T0, T1, T2, T3, C>
- `(T0, T1, T2, T3, T4)` with <T0, T1, T2, T3, T4, C>
- `(T0, T1, T2, T3, T4, T5)` with <T0, T1, T2, T3, T4, T5, C>
- `(T0, T1, T2, T3, T4, T5, T6)` with <T0, T1, T2, T3, T4, T5, T6, C>
- `(T0, T1, T2, T3, T4, T5, T6, T7)` with <T0, T1, T2, T3, T4, T5, T6, T7, C>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, C>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, C>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, C>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, C>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)` with <T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, C>
- `[T; N]` with <T, const N: usize, C>
- `[T]` with <T, C>
- `str` with <C>
- `core::ffi::CStr` with <C>
- `ops::Range<T>` with <T, C>
- `ops::RangeFrom<T>` with <T, C>
- `ops::RangeFull` with <C: Fallible + ?Sized>
- `ops::RangeTo<T>` with <T, C>
- `ops::RangeToInclusive<T>` with <T, C>
- `core::num::NonZeroI8` with <C>
- `core::num::NonZeroI16` with <C>
- `core::num::NonZeroI32` with <C>
- `core::num::NonZeroI64` with <C>
- `core::num::NonZeroI128` with <C>
- `core::num::NonZeroU8` with <C>
- `core::num::NonZeroU16` with <C>
- `core::num::NonZeroU32` with <C>
- `core::num::NonZeroU64` with <C>
- `core::num::NonZeroU128` with <C>

### Trait `Verify`

A type that can check whether its invariants are upheld.

When using [the derive](macro@CheckBytes), adding `#[bytecheck(verify)]`
allows implementing `Verify` for the derived type. [`Verify::verify`] will
be called after the type is checked and all fields are known to be valid.

# Safety

- `verify` must only return `Ok` if all of the invariants of this type are
  upheld by `self`.
- `verify` may not assume that its type invariants are upheld by the given
  `self` (the invariants of each field are guaranteed to be upheld).

# Example

```
use core::{error::Error, fmt};

use bytecheck::{CheckBytes, Verify};
use rancor::{fail, Fallible, Source};

#[derive(CheckBytes)]
#[bytecheck(verify)]
#[repr(C, align(4))]
pub struct NonMaxU32(u32);

unsafe impl<C: Fallible + ?Sized> Verify<C> for NonMaxU32
where
    C::Error: Source,
{
    fn verify(&self, context: &mut C) -> Result<(), C::Error> {
        #[derive(Debug)]
        struct NonMaxCheckError;

        impl fmt::Display for NonMaxCheckError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "non-max u32 was set to u32::MAX")
            }
        }

        impl Error for NonMaxCheckError {}

        if self.0 == u32::MAX {
            fail!(NonMaxCheckError);
        }

        Ok(())
    }
}
```

```rust
pub unsafe trait Verify<C: Fallible + ?Sized> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

#### Required Items

##### Required Methods

- `verify`: Checks whether the invariants of this type are upheld by `self`.

## Functions

### Function `check_bytes`

**Attributes:**

- `#[inline]`

Checks whether the given pointer points to a valid value.

# Safety

The passed pointer must be aligned and point to enough initialized bytes to
represent the type.

# Example

```
use bytecheck::check_bytes;
use rancor::Failure;

unsafe {
    // 0 and 1 are valid values for bools
    check_bytes::<bool, Failure>((&0u8 as *const u8).cast()).unwrap();
    check_bytes::<bool, Failure>((&1u8 as *const u8).cast()).unwrap();

    // 2 is not a valid value
    check_bytes::<bool, Failure>((&2u8 as *const u8).cast()).unwrap_err();
}
```

```rust
pub unsafe fn check_bytes<T, E>(value: *const T) -> Result<(), E>
where
    T: CheckBytes<rancor::Strategy<(), E>> + ?Sized { /* ... */ }
```

### Function `check_bytes_with_context`

Checks whether the given pointer points to a valid value within the given
context.

# Safety

The passed pointer must be aligned and point to enough initialized bytes to
represent the type.

# Example

```
use core::{error::Error, fmt};

use bytecheck::{check_bytes_with_context, CheckBytes, Verify};
use rancor::{fail, Failure, Fallible, Source, Strategy};

trait Context {
    fn is_allowed(&self, value: u8) -> bool;
}

impl<T: Context + ?Sized, E> Context for Strategy<T, E> {
    fn is_allowed(&self, value: u8) -> bool {
        T::is_allowed(self, value)
    }
}

struct Allowed(u8);

impl Context for Allowed {
    fn is_allowed(&self, value: u8) -> bool {
        value == self.0
    }
}

#[derive(CheckBytes)]
#[bytecheck(verify)]
#[repr(C)]
pub struct ContextualByte(u8);

unsafe impl<C: Context + Fallible + ?Sized> Verify<C> for ContextualByte
where
    C::Error: Source,
{
    fn verify(&self, context: &mut C) -> Result<(), C::Error> {
        #[derive(Debug)]
        struct InvalidByte(u8);

        impl fmt::Display for InvalidByte {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "invalid contextual byte: {}", self.0)
            }
        }

        impl Error for InvalidByte {}

        if !context.is_allowed(self.0) {
            fail!(InvalidByte(self.0));
        }

        Ok(())
    }
}

let value = 45u8;
unsafe {
    // Checking passes when the context allows byte 45
    check_bytes_with_context::<ContextualByte, _, Failure>(
        (&value as *const u8).cast(),
        &mut Allowed(45),
    )
    .unwrap();

    // Checking fails when the conteext does not allow byte 45
    check_bytes_with_context::<ContextualByte, _, Failure>(
        (&value as *const u8).cast(),
        &mut Allowed(0),
    )
    .unwrap_err();
}
```

```rust
pub unsafe fn check_bytes_with_context<T, C, E>(value: *const T, context: &mut C) -> Result<(), E>
where
    T: CheckBytes<rancor::Strategy<C, E>> + ?Sized { /* ... */ }
```

## Re-exports

### Re-export `CheckBytes`

```rust
pub use bytecheck_derive::CheckBytes;
```

### Re-export `rancor`

```rust
pub use rancor;
```

