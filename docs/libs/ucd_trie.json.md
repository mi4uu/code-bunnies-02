# Crate Documentation

**Version:** 0.1.7

**Format Version:** 43

# Module `ucd_trie`

The ucd-trie crate provides a compressed trie set specifically tailored for
Unicode codepoints. The principle use case for such a trie is to represent
properties defined by Unicode that correspond to sets of Unicode codepoints.
(These properties are formally called boolean properties or "single valued"
properties. See
[UTR#23 S3.3](https://www.unicode.org/reports/tr23/#PropertyTypeDefinitions)
for more details.)

This crate has two principle types: `TrieSetOwned` and `TrieSetSlice`,
corresponding to a similar split as there is between `Vec<T>` and `&[T]`.
`TrieSetOwned` is the only way to construct a trie from a set of Unicode
codepoints.

The intended use of this library is to embed a static instance of
`TrieSetSlice` into your source code, and then use its methods as defined in
this crate to test membership. (The `ucd-generate` tool can likely generate
this code for you.)

Finally, while this crate uses the standard library by default, it provides
`no_std` functionality by disabling the `std` feature. When `no_std` is
enabled, then `TrieSetOwned` is not provided. Instead, only `TrieSetSlice` is
provided, which means `no_std` crates can still embed tries into their code.

## Types

### Type Alias `TrieSet`

A type alias for `TrieSetSlice<'static>`.

```rust
pub type TrieSet = TrieSetSlice<''static>;
```

### Struct `TrieSetSlice`

A borrowed trie set.

```rust
pub struct TrieSetSlice<''a> {
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
  pub fn contains_char(self: &Self, c: char) -> bool { /* ... */ }
  ```
  Returns true if and only if the given Unicode scalar value is in this

- ```rust
  pub fn contains_u32(self: &Self, cp: u32) -> bool { /* ... */ }
  ```
  Returns true if and only if the given codepoint is in this set.

##### Trait Implementations

- **Send**
- **UnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TrieSetSlice<''a> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Re-exports

### Re-export `Error`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::owned::Error;
```

### Re-export `Result`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::owned::Result;
```

### Re-export `TrieSetOwned`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::owned::TrieSetOwned;
```

