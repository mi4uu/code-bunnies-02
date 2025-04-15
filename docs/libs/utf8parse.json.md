# Crate Documentation

**Version:** 0.2.2

**Format Version:** 43

# Module `utf8parse`

A table-driven UTF-8 Parser

This module implements a table-driven UTF-8 parser which should
theoretically contain the minimal number of branches (1). The only branch is
on the `Action` returned from unpacking a transition.

## Types

### Struct `Parser`

A parser for Utf8 Characters

Repeatedly call `advance` with bytes to emit Utf8 characters

```rust
pub struct Parser {
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
  pub fn new() -> Parser { /* ... */ }
  ```
  Create a new Parser

- ```rust
  pub fn advance<R>(self: &mut Self, receiver: &mut R, byte: u8)
where
    R: Receiver { /* ... */ }
  ```
  Advance the parser

##### Trait Implementations

- **UnwindSafe**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Parser) -> bool { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Parser { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Parser { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Freeze**
## Traits

### Trait `Receiver`

Handles codepoint and invalid sequence events from the parser.

```rust
pub trait Receiver {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `codepoint`: Called whenever a codepoint is parsed successfully
- `invalid_sequence`: Called when an invalid_sequence is detected

