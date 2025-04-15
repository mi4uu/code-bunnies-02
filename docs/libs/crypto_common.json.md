# Crate Documentation

**Version:** 0.1.6

**Format Version:** 43

# Module `crypto_common`

Common cryptographic traits.

## Types

### Type Alias `Block`

Block on which [`BlockSizeUser`] implementors operate.

```rust
pub type Block<B> = generic_array::GenericArray<u8, <B as BlockSizeUser>::BlockSize>;
```

### Type Alias `ParBlocks`

Parallel blocks on which [`ParBlocksSizeUser`] implementors operate.

```rust
pub type ParBlocks<T> = generic_array::GenericArray<Block<T>, <T as ParBlocksSizeUser>::ParBlocksSize>;
```

### Type Alias `Output`

Output array of [`OutputSizeUser`] implementors.

```rust
pub type Output<T> = generic_array::GenericArray<u8, <T as OutputSizeUser>::OutputSize>;
```

### Type Alias `Key`

Key used by [`KeySizeUser`] implementors.

```rust
pub type Key<B> = generic_array::GenericArray<u8, <B as KeySizeUser>::KeySize>;
```

### Type Alias `Iv`

Initialization vector (nonce) used by [`IvSizeUser`] implementors.

```rust
pub type Iv<B> = generic_array::GenericArray<u8, <B as IvSizeUser>::IvSize>;
```

### Struct `InvalidLength`

The error type returned when key and/or IV used in the [`KeyInit`],
[`KeyIvInit`], and [`InnerIvInit`] slice-based methods had
an invalid length.

```rust
pub struct InvalidLength;
```

#### Implementations

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InvalidLength { /* ... */ }
    ```

- **StructuralPartialEq**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Error**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Send**
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
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InvalidLength) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Same**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Traits

### Trait `BlockSizeUser`

Types which process data in blocks.

```rust
pub trait BlockSizeUser {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `BlockSize`: Size of the block in bytes.

#### Provided Methods

- ```rust
  fn block_size() -> usize { /* ... */ }
  ```
  Return block size in bytes.

#### Implementations

This trait is implemented for the following types:

- `&T` with <T: BlockSizeUser>
- `&mut T` with <T: BlockSizeUser>

### Trait `ParBlocksSizeUser`

Types which can process blocks in parallel.

```rust
pub trait ParBlocksSizeUser: BlockSizeUser {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `ParBlocksSize`: Number of blocks which can be processed in parallel.

### Trait `OutputSizeUser`

Types which return data with the given size.

```rust
pub trait OutputSizeUser {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `OutputSize`: Size of the output in bytes.

#### Provided Methods

- ```rust
  fn output_size() -> usize { /* ... */ }
  ```
  Return output size in bytes.

### Trait `KeySizeUser`

Types which use key for initialization.

Generally it's used indirectly via [`KeyInit`] or [`KeyIvInit`].

```rust
pub trait KeySizeUser {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `KeySize`: Key size in bytes.

#### Provided Methods

- ```rust
  fn key_size() -> usize { /* ... */ }
  ```
  Return key size in bytes.

#### Implementations

This trait is implemented for the following types:

- `T` with <T>

### Trait `IvSizeUser`

Types which use initialization vector (nonce) for initialization.

Generally it's used indirectly via [`KeyIvInit`] or [`InnerIvInit`].

```rust
pub trait IvSizeUser {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `IvSize`: Initialization vector size in bytes.

#### Provided Methods

- ```rust
  fn iv_size() -> usize { /* ... */ }
  ```
  Return IV size in bytes.

### Trait `InnerUser`

Types which use another type for initialization.

Generally it's used indirectly via [`InnerInit`] or [`InnerIvInit`].

```rust
pub trait InnerUser {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Inner`: Inner type.

### Trait `Reset`

Resettable types.

```rust
pub trait Reset {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `reset`: Reset state to its initial value.

### Trait `AlgorithmName`

Trait which stores algorithm name constant, used in `Debug` implementations.

```rust
pub trait AlgorithmName {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `write_alg_name`: Write algorithm name into `f`.

### Trait `KeyInit`

Types which can be initialized from key.

```rust
pub trait KeyInit: KeySizeUser + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `new`: Create new value from fixed size key.

#### Provided Methods

- ```rust
  fn new_from_slice(key: &[u8]) -> Result<Self, InvalidLength> { /* ... */ }
  ```
  Create new value from variable size key.

#### Implementations

This trait is implemented for the following types:

- `T` with <T>

### Trait `KeyIvInit`

Types which can be initialized from key and initialization vector (nonce).

```rust
pub trait KeyIvInit: KeySizeUser + IvSizeUser + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `new`: Create new value from fixed length key and nonce.

#### Provided Methods

- ```rust
  fn new_from_slices(key: &[u8], iv: &[u8]) -> Result<Self, InvalidLength> { /* ... */ }
  ```
  Create new value from variable length key and nonce.

#### Implementations

This trait is implemented for the following types:

- `T` with <T>

### Trait `InnerInit`

Types which can be initialized from another type (usually block ciphers).

Usually used for initializing types from block ciphers.

```rust
pub trait InnerInit: InnerUser + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `inner_init`: Initialize value from the `inner`.

### Trait `InnerIvInit`

Types which can be initialized from another type and additional initialization
vector/nonce.

Usually used for initializing types from block ciphers.

```rust
pub trait InnerIvInit: InnerUser + IvSizeUser + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `inner_iv_init`: Initialize value using `inner` and `iv` array.

#### Provided Methods

- ```rust
  fn inner_iv_slice_init(inner: <Self as >::Inner, iv: &[u8]) -> Result<Self, InvalidLength> { /* ... */ }
  ```
  Initialize value using `inner` and `iv` slice.

## Re-exports

### Re-export `generic_array`

```rust
pub use generic_array;
```

### Re-export `typenum`

```rust
pub use generic_array::typenum;
```

