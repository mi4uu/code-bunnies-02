# Crate Documentation

**Version:** 2.0.0

**Format Version:** 43

# Module `sacabase`

## Types

### Struct `LongestCommonSubstring`

```rust
pub struct LongestCommonSubstring<''a> {
    pub text: &''a [u8],
    pub start: usize,
    pub len: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `text` | `&''a [u8]` |  |
| `start` | `usize` |  |
| `len` | `usize` |  |

#### Implementations

##### Methods

- ```rust
  pub fn as_bytes(self: &Self) -> &[u8] { /* ... */ }
  ```

##### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `NotSorted`

Error returned by `verify` when a suffix array is not sorted.

```rust
pub struct NotSorted {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Error**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `SuffixArray`

A suffix array

```rust
pub struct SuffixArray<''a, Index> {
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
  pub fn new(text: &''a [u8], sa: Vec<Index>) -> Self { /* ... */ }
  ```
  Create an instance of SuffixArray, taking ownership of `sa`

- ```rust
  pub fn into_parts(self: Self) -> (&''a [u8], Vec<Index>) { /* ... */ }
  ```
  Return (text, sa), giving back ownership of `sa`

- ```rust
  pub fn verify(self: &Self) -> Result<(), NotSorted> { /* ... */ }
  ```
  Verifies that this suffix array is sorted.

- ```rust
  pub fn text(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns a reference to the text

##### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StringIndex**
  - ```rust
    fn longest_substring_match(self: &Self, needle: &[u8]) -> LongestCommonSubstring<''a> { /* ... */ }
    ```

- **Freeze**
## Traits

### Trait `StringIndex`

```rust
pub trait StringIndex<''a> {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `longest_substring_match`: Returns the longest substring that matches `needle` in text

#### Implementations

This trait is implemented for the following types:

- `SuffixArray<''a, Index>` with <''a, Index>

## Functions

### Function `common_prefix_len`

**Attributes:**

- `#[inline(always)]`

Returns the number of bytes `a` and `b` have in common.
Ex: `common_prefix_len("banana", "banter") = 3`

```rust
pub fn common_prefix_len(a: &[u8], b: &[u8]) -> usize { /* ... */ }
```

### Function `longest_substring_match`

Searches for the longest substring match for `needle`
in `input`, using its suffix array `sa`.

```rust
pub fn longest_substring_match<''a, Index>(text: &''a [u8], sa: &[Index], needle: &[u8]) -> LongestCommonSubstring<''a>
where
    Index: ToPrimitive { /* ... */ }
```

### Function `verify`

Returns an error if `sa` is not the suffix array of `input`,
Ok(()) otherwise.

```rust
pub fn verify<Index>(input: &[u8], sa: &[Index]) -> Result<(), NotSorted>
where
    Index: ToPrimitive { /* ... */ }
```

