# Crate Documentation

**Version:** 0.10.4

**Format Version:** 43

# Module `block_buffer`

Fixed size buffer for block processing of data.

## Types

### Type Alias `Block`

Block on which `BlockBuffer` operates.

```rust
pub type Block<BlockSize> = generic_array::GenericArray<u8, BlockSize>;
```

### Struct `Eager`

Eager block buffer kind, which guarantees that buffer position
always lies in the range of `0..BlockSize`.

```rust
pub struct Eager {
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|

#### Implementations

##### Trait Implementations

- **BufferKind**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Eager { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Eager { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `Lazy`

Lazy block buffer kind, which guarantees that buffer position
always lies in the range of `0..=BlockSize`.

```rust
pub struct Lazy {
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Lazy { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Same**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Lazy { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BufferKind**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **RefUnwindSafe**
### Type Alias `EagerBuffer`

Eager block buffer.

```rust
pub type EagerBuffer<B> = BlockBuffer<B, Eager>;
```

### Type Alias `LazyBuffer`

Lazy block buffer.

```rust
pub type LazyBuffer<B> = BlockBuffer<B, Lazy>;
```

### Struct `Error`

Block buffer error.

```rust
pub struct Error;
```

#### Implementations

##### Trait Implementations

- **Same**
- **Eq**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `BlockBuffer`

Buffer for block processing of data.

```rust
pub struct BlockBuffer<BlockSize, Kind> {
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
  pub fn new(buf: &[u8]) -> Self { /* ... */ }
  ```
  Create new buffer from slice.

- ```rust
  pub fn try_new(buf: &[u8]) -> Result<Self, Error> { /* ... */ }
  ```
  Create new buffer from slice.

- ```rust
  pub fn digest_blocks</* synthetic */ impl FnMut(&[Block<BlockSize>]): FnMut(&[Block<BlockSize>])>(self: &mut Self, input: &[u8], compress: impl FnMut(&[Block<BlockSize>])) { /* ... */ }
  ```
  Digest data in `input` in blocks of size `BlockSize` using

- ```rust
  pub fn reset(self: &mut Self) { /* ... */ }
  ```
  Reset buffer by setting cursor position to zero.

- ```rust
  pub fn pad_with_zeros(self: &mut Self) -> &mut Block<BlockSize> { /* ... */ }
  ```
  Pad remaining data with zeros and return resulting block.

- ```rust
  pub fn get_pos(self: &Self) -> usize { /* ... */ }
  ```
  Return current cursor position.

- ```rust
  pub fn get_data(self: &Self) -> &[u8] { /* ... */ }
  ```
  Return slice of data stored inside the buffer.

- ```rust
  pub fn set(self: &mut Self, buf: Block<BlockSize>, pos: usize) { /* ... */ }
  ```
  Set buffer content and cursor position.

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Return size of the internal buffer in bytes.

- ```rust
  pub fn remaining(self: &Self) -> usize { /* ... */ }
  ```
  Return number of remaining bytes in the internal buffer.

- ```rust
  pub fn set_data</* synthetic */ impl FnMut(&mut [Block<BlockSize>]): FnMut(&mut [Block<BlockSize>])>(self: &mut Self, data: &mut [u8], process_blocks: impl FnMut(&mut [Block<BlockSize>])) { /* ... */ }
  ```
  Set `data` to generated blocks.

- ```rust
  pub fn digest_pad</* synthetic */ impl FnMut(&Block<BlockSize>): FnMut(&Block<BlockSize>)>(self: &mut Self, delim: u8, suffix: &[u8], compress: impl FnMut(&Block<BlockSize>)) { /* ... */ }
  ```
  Compress remaining data after padding it with `delim`, zeros and

- ```rust
  pub fn len64_padding_be</* synthetic */ impl FnMut(&Block<BlockSize>): FnMut(&Block<BlockSize>)>(self: &mut Self, data_len: u64, compress: impl FnMut(&Block<BlockSize>)) { /* ... */ }
  ```
  Pad message with 0x80, zeros and 64-bit message length using

- ```rust
  pub fn len64_padding_le</* synthetic */ impl FnMut(&Block<BlockSize>): FnMut(&Block<BlockSize>)>(self: &mut Self, data_len: u64, compress: impl FnMut(&Block<BlockSize>)) { /* ... */ }
  ```
  Pad message with 0x80, zeros and 64-bit message length using

- ```rust
  pub fn len128_padding_be</* synthetic */ impl FnMut(&Block<BlockSize>): FnMut(&Block<BlockSize>)>(self: &mut Self, data_len: u128, compress: impl FnMut(&Block<BlockSize>)) { /* ... */ }
  ```
  Pad message with 0x80, zeros and 128-bit message length using

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **RefUnwindSafe**
- **Sync**
- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Freeze**
- **Send**
## Traits

### Trait `BufferKind`

Trait for buffer kinds.

```rust
pub trait BufferKind: sealed::Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Implementations

This trait is implemented for the following types:

- `Eager`
- `Lazy`

## Re-exports

### Re-export `generic_array`

```rust
pub use generic_array;
```

