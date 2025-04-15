# Crate Documentation

**Version:** 1.0.4

**Format Version:** 43

# Module `utf8_iter`

Provides iteration by `char` over `&[u8]` containing potentially-invalid
UTF-8 such that errors are handled according to the [WHATWG Encoding
Standard](https://encoding.spec.whatwg.org/#utf-8-decoder) (i.e. the same
way as in `String::from_utf8_lossy`).

The trait `Utf8CharsEx` provides the convenience method `chars()` on
byte slices themselves instead of having to use the more verbose
`Utf8Chars::new(slice)`.

```rust
use utf8_iter::Utf8CharsEx;
let data = b"\xFF\xC2\xE2\xE2\x98\xF0\xF0\x9F\xF0\x9F\x92\xE2\x98\x83";
let from_iter: String = data.chars().collect();
let from_std = String::from_utf8_lossy(data);
assert_eq!(from_iter, from_std);
```

## Types

### Struct `Utf8Chars`

Iterator by `char` over `&[u8]` that contains
potentially-invalid UTF-8. See the crate documentation.

```rust
pub struct Utf8Chars<''a> {
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
  pub fn new(bytes: &''a [u8]) -> Self { /* ... */ }
  ```
  Creates the iterator from a byte slice.

- ```rust
  pub fn as_slice(self: &Self) -> &''a [u8] { /* ... */ }
  ```
  Views the current remaining data in the iterator as a subslice

##### Trait Implementations

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
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<char> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<char> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8Chars<''a> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FusedIterator**
- **RefUnwindSafe**
- **Unpin**
## Traits

### Trait `Utf8CharsEx`

Convenience trait that adds `chars()` and `char_indices()` methods
similar to the ones on string slices to byte slices.

```rust
pub trait Utf8CharsEx {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `chars`
- `char_indices`

#### Implementations

This trait is implemented for the following types:

- `[u8]`

## Re-exports

### Re-export `Utf8CharIndices`

```rust
pub use crate::indices::Utf8CharIndices;
```

### Re-export `ErrorReportingUtf8Chars`

```rust
pub use crate::report::ErrorReportingUtf8Chars;
```

### Re-export `Utf8CharsError`

```rust
pub use crate::report::Utf8CharsError;
```

