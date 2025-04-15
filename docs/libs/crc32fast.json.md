# Crate Documentation

**Version:** 1.4.2

**Format Version:** 43

# Module `crc32fast`

Fast, SIMD-accelerated CRC32 (IEEE) checksum computation.

## Usage

### Simple usage

For simple use-cases, you can call the [`hash()`] convenience function to
directly compute the CRC32 checksum for a given byte slice:

```rust
let checksum = crc32fast::hash(b"foo bar baz");
```

### Advanced usage

For use-cases that require more flexibility or performance, for example when
processing large amounts of data, you can create and manipulate a [`Hasher`]:

```rust
use crc32fast::Hasher;

let mut hasher = Hasher::new();
hasher.update(b"foo bar baz");
let checksum = hasher.finalize();
```

## Performance

This crate contains multiple CRC32 implementations:

- A fast baseline implementation which processes up to 16 bytes per iteration
- An optimized implementation for modern `x86` using `sse` and `pclmulqdq` instructions

Calling the [`Hasher::new`] constructor at runtime will perform a feature detection to select the most
optimal implementation for the current CPU feature set.

## Types

### Struct `Hasher`

Represents an in-progress CRC32 computation.

```rust
pub struct Hasher {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new `Hasher`.

- ```rust
  pub fn new_with_initial(init: u32) -> Self { /* ... */ }
  ```
  Create a new `Hasher` with an initial CRC32 state.

- ```rust
  pub fn new_with_initial_len(init: u32, amount: u64) -> Self { /* ... */ }
  ```
  Create a new `Hasher` with an initial CRC32 state.

- ```rust
  pub fn update(self: &mut Self, buf: &[u8]) { /* ... */ }
  ```
  Process the given byte slice and update the hash state.

- ```rust
  pub fn finalize(self: Self) -> u32 { /* ... */ }
  ```
  Finalize the hash state and return the computed CRC32 value.

- ```rust
  pub fn reset(self: &mut Self) { /* ... */ }
  ```
  Reset the hash state.

- ```rust
  pub fn combine(self: &mut Self, other: &Self) { /* ... */ }
  ```
  Combine the hash state with the hash state for the subsequent block of bytes.

##### Trait Implementations

- **Sync**
- **RefUnwindSafe**
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

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Hasher { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hasher**
  - ```rust
    fn write(self: &mut Self, bytes: &[u8]) { /* ... */ }
    ```

  - ```rust
    fn finish(self: &Self) -> u64 { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
## Functions

### Function `hash`

Computes the CRC32 hash of a byte slice.

Check out [`Hasher`] for more advanced use-cases.

```rust
pub fn hash(buf: &[u8]) -> u32 { /* ... */ }
```

