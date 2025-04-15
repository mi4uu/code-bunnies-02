# Crate Documentation

**Version:** 0.1.13

**Format Version:** 43

# Module `diff`

## Types

### Enum `Result`

A fragment of a computed diff.

```rust
pub enum Result<T> {
    Left(T),
    Both(T, T),
    Right(T),
}
```

#### Variants

##### `Left`

An element that only exists in the left input.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### `Both`

Elements that exist in both inputs.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |
| 1 | `T` |  |

##### `Right`

An element that only exists in the right input.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

#### Implementations

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Result<T> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Result<T>) -> bool { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Functions

### Function `slice`

Computes the diff between two slices.

```rust
pub fn slice<''a, T: PartialEq>(left: &''a [T], right: &''a [T]) -> Vec<Result<&''a T>> { /* ... */ }
```

### Function `lines`

Computes the diff between the lines of two strings.

```rust
pub fn lines<''a>(left: &''a str, right: &''a str) -> Vec<Result<&''a str>> { /* ... */ }
```

### Function `chars`

Computes the diff between the chars of two strings.

```rust
pub fn chars<''a>(left: &''a str, right: &''a str) -> Vec<Result<char>> { /* ... */ }
```

