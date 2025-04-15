# Crate Documentation

**Version:** 0.7.6

**Format Version:** 43

# Module `utf8`

## Types

### Enum `DecodeError`

```rust
pub enum DecodeError<''a> {
    Invalid {
        valid_prefix: &''a str,
        invalid_sequence: &''a [u8],
        remaining_input: &''a [u8],
    },
    Incomplete {
        valid_prefix: &''a str,
        incomplete_suffix: Incomplete,
    },
}
```

#### Variants

##### `Invalid`

In lossy decoding insert `valid_prefix`, then `"\u{FFFD}"`,
then call `decode()` again with `remaining_input`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `valid_prefix` | `&''a str` |  |
| `invalid_sequence` | `&''a [u8]` |  |
| `remaining_input` | `&''a [u8]` |  |

##### `Incomplete`

Call the `incomplete_suffix.try_complete` method with more input when available.
If no more input is available, this is an invalid byte sequence.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `valid_prefix` | `&''a str` |  |
| `incomplete_suffix` | `Incomplete` |  |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
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

- **Freeze**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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
    fn clone(self: &Self) -> DecodeError<''a> { /* ... */ }
    ```

- **Sync**
- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `Incomplete`

```rust
pub struct Incomplete {
    pub buffer: [u8; 4],
    pub buffer_len: u8,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buffer` | `[u8; 4]` |  |
| `buffer_len` | `u8` |  |

#### Implementations

##### Methods

- ```rust
  pub fn empty() -> Self { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn new(bytes: &[u8]) -> Self { /* ... */ }
  ```

- ```rust
  pub fn try_complete<''input>(self: &mut Self, input: &''input [u8]) -> Option<(Result<&str, &[u8]>, &''input [u8])> { /* ... */ }
  ```
  * `None`: still incomplete, call `try_complete` again with more input.

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Copy**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Incomplete { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Functions

### Function `decode`

```rust
pub fn decode(input: &[u8]) -> Result<&str, DecodeError<''_>> { /* ... */ }
```

## Constants and Statics

### Constant `REPLACEMENT_CHARACTER`

The replacement character, U+FFFD. In lossy decoding, insert it for every decoding error.

```rust
pub const REPLACEMENT_CHARACTER: &''static str = "\u{FFFD}";
```

## Re-exports

### Re-export `LossyDecoder`

```rust
pub use lossy::LossyDecoder;
```

### Re-export `BufReadDecoder`

```rust
pub use read::BufReadDecoder;
```

### Re-export `BufReadDecoderError`

```rust
pub use read::BufReadDecoderError;
```

