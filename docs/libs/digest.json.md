# Crate Documentation

**Version:** 0.10.7

**Format Version:** 43

# Module `digest`

This crate provides traits which describe functionality of cryptographic hash
functions and Message Authentication algorithms.

Traits in this repository are organized into the following levels:

- **High-level convenience traits**: [`Digest`], [`DynDigest`], [`Mac`].
  Wrappers around lower-level traits for most common use-cases. Users should
  usually prefer using these traits.
- **Mid-level traits**: [`Update`], [`FixedOutput`], [`FixedOutputReset`],
  [`ExtendableOutput`], [`ExtendableOutputReset`], [`XofReader`],
  [`VariableOutput`], [`Reset`], [`KeyInit`], and [`InnerInit`]. These
  traits atomically describe available functionality of an algorithm.
- **Marker traits**: [`HashMarker`], [`MacMarker`]. Used to distinguish
  different algorithm classes.
- **Low-level traits** defined in the [`core_api`] module. These traits
  operate at a block-level and do not contain any built-in buffering.
  They are intended to be implemented by low-level algorithm providers only.
  Usually they should not be used in application-level code.

Additionally hash functions implement traits from the standard library:
[`Default`], [`Clone`], [`Write`][std::io::Write]. The latter is
feature-gated behind `std` feature, which is usually enabled by default
by hash implementation crates.

## Modules

## Module `core_api`

**Attributes:**

- `#[<cfg>(feature = "core-api")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "core-api")))]`

Low-level traits operating on blocks and wrappers around them.

Usage of traits in this module in user code is discouraged. Instead use
core algorithm wrapped by the wrapper types, which implement the
higher-level traits.

```rust
pub mod core_api { /* ... */ }
```

### Types

#### Type Alias `Buffer`

Buffer type used by type which implements [`BufferKindUser`].

```rust
pub type Buffer<S> = block_buffer::BlockBuffer<<S as BlockSizeUser>::BlockSize, <S as BufferKindUser>::BufferKind>;
```

#### Enum `TruncSide`

Type which used for defining truncation side in the [`VariableOutputCore`]
trait.

```rust
pub enum TruncSide {
    Left,
    Right,
}
```

##### Variants

###### `Left`

Truncate left side, i.e. `&out[..n]`.

###### `Right`

Truncate right side, i.e. `&out[m..]`.

##### Implementations

###### Trait Implementations

- **Send**
- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn clone(self: &Self) -> TruncSide { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Same**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
### Traits

#### Trait `UpdateCore`

Types which consume data in blocks.

```rust
pub trait UpdateCore: BlockSizeUser {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `update_blocks`: Update state using the provided data blocks.

##### Implementations

This trait is implemented for the following types:

- `CtVariableCoreWrapper<T, OutSize, O>` with <T, OutSize, O>

#### Trait `BufferKindUser`

Types which use [`BlockBuffer`] functionality.

```rust
pub trait BufferKindUser: BlockSizeUser {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `BufferKind`: Block buffer kind over which type operates.

##### Implementations

This trait is implemented for the following types:

- `CtVariableCoreWrapper<T, OutSize, O>` with <T, OutSize, O>

#### Trait `FixedOutputCore`

Core trait for hash functions with fixed output size.

```rust
pub trait FixedOutputCore: UpdateCore + BufferKindUser + OutputSizeUser
where
    <Self as >::BlockSize: IsLess<crypto_common::typenum::U256>,
    crypto_common::typenum::Le<<Self as >::BlockSize, crypto_common::typenum::U256>: NonZero {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `finalize_fixed_core`: Finalize state using remaining data stored in the provided block buffer,

##### Implementations

This trait is implemented for the following types:

- `CtVariableCoreWrapper<T, OutSize, O>` with <T, OutSize, O>

#### Trait `ExtendableOutputCore`

Core trait for hash functions with extendable (XOF) output size.

```rust
pub trait ExtendableOutputCore: UpdateCore + BufferKindUser
where
    <Self as >::BlockSize: IsLess<crypto_common::typenum::U256>,
    crypto_common::typenum::Le<<Self as >::BlockSize, crypto_common::typenum::U256>: NonZero {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `ReaderCore`: XOF reader core state.

###### Required Methods

- `finalize_xof_core`: Retrieve XOF reader using remaining data stored in the block buffer

#### Trait `XofReaderCore`

Core reader trait for extendable-output function (XOF) result.

```rust
pub trait XofReaderCore: BlockSizeUser {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `read_block`: Read next XOF block.

#### Trait `VariableOutputCore`

Core trait for hash functions with variable output size.

Maximum output size is equal to [`OutputSizeUser::OutputSize`].
Users are expected to truncate result returned by the
[`finalize_variable_core`] to `output_size` passed to the [`new`] method
during construction. Truncation side is defined by the [`TRUNC_SIDE`]
associated constant.

[`finalize_variable_core`]: VariableOutputCore::finalize_variable_core
[`new`]: VariableOutputCore::new
[`TRUNC_SIDE`]: VariableOutputCore::TRUNC_SIDE

```rust
pub trait VariableOutputCore: UpdateCore + OutputSizeUser + BufferKindUser + Sized
where
    <Self as >::BlockSize: IsLess<crypto_common::typenum::U256>,
    crypto_common::typenum::Le<<Self as >::BlockSize, crypto_common::typenum::U256>: NonZero {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `TRUNC_SIDE`: Side which should be used in a truncated result.

###### Required Methods

- `new`: Initialize hasher state for given output size.
- `finalize_variable_core`: Finalize hasher and write full hashing result into the `out` buffer.

### Re-exports

#### Re-export `AlgorithmName`

```rust
pub use crypto_common::AlgorithmName;
```

#### Re-export `Block`

```rust
pub use crypto_common::Block;
```

#### Re-export `BlockSizeUser`

```rust
pub use crypto_common::BlockSizeUser;
```

#### Re-export `OutputSizeUser`

```rust
pub use crypto_common::OutputSizeUser;
```

#### Re-export `Reset`

```rust
pub use crypto_common::Reset;
```

#### Re-export `CtVariableCoreWrapper`

```rust
pub use ct_variable::CtVariableCoreWrapper;
```

#### Re-export `RtVariableCoreWrapper`

```rust
pub use rt_variable::RtVariableCoreWrapper;
```

#### Re-export `CoreProxy`

```rust
pub use wrapper::CoreProxy;
```

#### Re-export `CoreWrapper`

```rust
pub use wrapper::CoreWrapper;
```

#### Re-export `XofReaderCoreWrapper`

```rust
pub use xof_reader::XofReaderCoreWrapper;
```

## Types

### Struct `InvalidOutputSize`

The error type used in variable hash traits.

```rust
pub struct InvalidOutputSize;
```

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **Error**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> InvalidOutputSize { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InvalidOutputSize { /* ... */ }
    ```

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `InvalidBufferSize`

Buffer length is not equal to hash output size.

```rust
pub struct InvalidBufferSize;
```

#### Implementations

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InvalidBufferSize { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> InvalidBufferSize { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Error**
- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InvalidBufferSize) -> bool { /* ... */ }
    ```

- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Same**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

## Traits

### Trait `Update`

Types which consume data with byte granularity.

```rust
pub trait Update {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `update`: Update state using the provided data.

#### Provided Methods

- ```rust
  fn chain</* synthetic */ impl AsRef<[u8]>: AsRef<[u8]>>(self: Self, data: impl AsRef<[u8]>) -> Self
where
    Self: Sized { /* ... */ }
  ```
  Digest input data in a chained manner.

#### Implementations

This trait is implemented for the following types:

- `RtVariableCoreWrapper<T>` with <T>
- `CoreWrapper<T>` with <T>

### Trait `FixedOutput`

Trait for hash functions with fixed-size output.

```rust
pub trait FixedOutput: Update + OutputSizeUser + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `finalize_into`: Consume value and write result into provided array.

#### Provided Methods

- ```rust
  fn finalize_fixed(self: Self) -> Output<Self> { /* ... */ }
  ```
  Retrieve result and consume the hasher instance.

#### Implementations

This trait is implemented for the following types:

- `CoreWrapper<T>` with <T>

### Trait `FixedOutputReset`

Trait for hash functions with fixed-size output able to reset themselves.

```rust
pub trait FixedOutputReset: FixedOutput + Reset {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `finalize_into_reset`: Write result into provided array and reset the hasher state.

#### Provided Methods

- ```rust
  fn finalize_fixed_reset(self: &mut Self) -> Output<Self> { /* ... */ }
  ```
  Retrieve result and reset the hasher state.

#### Implementations

This trait is implemented for the following types:

- `CoreWrapper<T>` with <T>

### Trait `XofReader`

Trait for reader types which are used to extract extendable output
from a XOF (extendable-output function) result.

```rust
pub trait XofReader {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `read`: Read output into the `buffer`. Can be called an unlimited number of times.

#### Provided Methods

- ```rust
  fn read_boxed(self: &mut Self, n: usize) -> Box<[u8]> { /* ... */ }
  ```
  Read output into a boxed slice of the specified size.

#### Implementations

This trait is implemented for the following types:

- `XofReaderCoreWrapper<T>` with <T>

### Trait `ExtendableOutput`

Trait for hash functions with extendable-output (XOF).

```rust
pub trait ExtendableOutput: Sized + Update {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `Reader`: Reader

##### Required Methods

- `finalize_xof`: Retrieve XOF reader and consume hasher instance.

#### Provided Methods

- ```rust
  fn finalize_xof_into(self: Self, out: &mut [u8]) { /* ... */ }
  ```
  Finalize XOF and write result into `out`.

- ```rust
  fn digest_xof</* synthetic */ impl AsRef<[u8]>: AsRef<[u8]>>(input: impl AsRef<[u8]>, output: &mut [u8])
where
    Self: Default { /* ... */ }
  ```
  Compute hash of `data` and write it into `output`.

- ```rust
  fn finalize_boxed(self: Self, output_size: usize) -> Box<[u8]> { /* ... */ }
  ```
  Retrieve result into a boxed slice of the specified size and consume

#### Implementations

This trait is implemented for the following types:

- `CoreWrapper<T>` with <T>

### Trait `ExtendableOutputReset`

Trait for hash functions with extendable-output (XOF) able to reset themselves.

```rust
pub trait ExtendableOutputReset: ExtendableOutput + Reset {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `finalize_xof_reset`: Retrieve XOF reader and reset hasher instance state.

#### Provided Methods

- ```rust
  fn finalize_xof_reset_into(self: &mut Self, out: &mut [u8]) { /* ... */ }
  ```
  Finalize XOF, write result into `out`, and reset the hasher state.

- ```rust
  fn finalize_boxed_reset(self: &mut Self, output_size: usize) -> Box<[u8]> { /* ... */ }
  ```
  Retrieve result into a boxed slice of the specified size and reset

#### Implementations

This trait is implemented for the following types:

- `CoreWrapper<T>` with <T>

### Trait `VariableOutput`

Trait for hash functions with variable-size output.

```rust
pub trait VariableOutput: Sized + Update {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Constants

- `MAX_OUTPUT_SIZE`: Maximum size of output hash.

##### Required Methods

- `new`: Create new hasher instance with the given output size.
- `output_size`: Get output size of the hasher instance provided to the `new` method
- `finalize_variable`: Write result into the output buffer.

#### Provided Methods

- ```rust
  fn digest_variable</* synthetic */ impl AsRef<[u8]>: AsRef<[u8]>>(input: impl AsRef<[u8]>, output: &mut [u8]) -> Result<(), InvalidOutputSize> { /* ... */ }
  ```
  Compute hash of `data` and write it to `output`.

- ```rust
  fn finalize_boxed(self: Self) -> Box<[u8]> { /* ... */ }
  ```
  Retrieve result into a boxed slice and consume hasher.

#### Implementations

This trait is implemented for the following types:

- `RtVariableCoreWrapper<T>` with <T>

### Trait `VariableOutputReset`

Trait for hash functions with variable-size output able to reset themselves.

```rust
pub trait VariableOutputReset: VariableOutput + Reset {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `finalize_variable_reset`: Write result into the output buffer and reset the hasher state.

#### Provided Methods

- ```rust
  fn finalize_boxed_reset(self: &mut Self) -> Box<[u8]> { /* ... */ }
  ```
  Retrieve result into a boxed slice and reset the hasher state.

#### Implementations

This trait is implemented for the following types:

- `RtVariableCoreWrapper<T>` with <T>

## Macros

### Macro `impl_oid_carrier`

**Attributes:**

- `#[macro_export]`

Implement dummy type with hidden docs which is used to "carry" hasher
OID for [`CtVariableCoreWrapper`].

```rust
pub macro_rules! impl_oid_carrier {
    /* macro_rules! impl_oid_carrier {
    ($name:ident, $oid:literal) => { ... };
} */
}
```

## Re-exports

### Re-export `block_buffer`

**Attributes:**

- `#[<cfg>(feature = "core-api")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "core-api")))]`

```rust
pub use block_buffer;
```

### Re-export `crypto_common`

```rust
pub use crypto_common;
```

### Re-export `Digest`

```rust
pub use crate::digest::Digest;
```

### Re-export `DynDigest`

```rust
pub use crate::digest::DynDigest;
```

### Re-export `HashMarker`

```rust
pub use crate::digest::HashMarker;
```

### Re-export `generic_array`

```rust
pub use crypto_common::generic_array;
```

### Re-export `typenum`

```rust
pub use crypto_common::typenum;
```

### Re-export `consts`

```rust
pub use crypto_common::typenum::consts;
```

### Re-export `Output`

```rust
pub use crypto_common::Output;
```

### Re-export `OutputSizeUser`

```rust
pub use crypto_common::OutputSizeUser;
```

### Re-export `Reset`

```rust
pub use crypto_common::Reset;
```

