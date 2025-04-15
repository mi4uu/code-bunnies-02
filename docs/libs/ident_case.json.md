# Crate Documentation

**Version:** 1.0.1

**Format Version:** 43

# Module `ident_case`

Crate for changing case of Rust identifiers.

# Features
* Supports `snake_case`, `lowercase`, `camelCase`, 
  `PascalCase`, `SCREAMING_SNAKE_CASE`, and `kebab-case`
* Rename variants, and fields
 
# Examples
```rust
use ident_case::RenameRule;

assert_eq!("helloWorld", RenameRule::CamelCase.apply_to_field("hello_world"));

assert_eq!("i_love_serde", RenameRule::SnakeCase.apply_to_variant("ILoveSerde"));
```

## Types

### Enum `RenameRule`

A casing rule for renaming Rust identifiers.

```rust
pub enum RenameRule {
    None,
    LowerCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
}
```

#### Variants

##### `None`

No-op rename rule.

##### `LowerCase`

Rename direct children to "lowercase" style.

##### `PascalCase`

Rename direct children to "PascalCase" style, as typically used for enum variants.

##### `CamelCase`

Rename direct children to "camelCase" style.

##### `SnakeCase`

Rename direct children to "snake_case" style, as commonly used for fields.

##### `ScreamingSnakeCase`

Rename direct children to "SCREAMING_SNAKE_CASE" style, as commonly used for constants.

##### `KebabCase`

Rename direct children to "kebab-case" style.

#### Implementations

##### Methods

- ```rust
  pub fn apply_to_variant<S: AsRef<str>>(self: &Self, variant: S) -> String { /* ... */ }
  ```
  Change case of a `PascalCase` variant.

- ```rust
  pub fn apply_to_field<S: AsRef<str>>(self: &Self, field: S) -> String { /* ... */ }
  ```
  Change case of a `snake_case` field.

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RenameRule) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **Freeze**
- **Copy**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromStr**
  - ```rust
    fn from_str(rename_all_str: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
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

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RenameRule { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

