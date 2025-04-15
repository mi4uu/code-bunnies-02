# Crate Documentation

**Version:** 1.0.3

**Format Version:** 43

# Module `colorchoice`

Global override of color control

## Types

### Enum `ColorChoice`

**Attributes:**

- `#[allow(clippy::exhaustive_enums)]`

Selection for overriding color output

```rust
pub enum ColorChoice {
    Auto,
    AlwaysAnsi,
    Always,
    Never,
}
```

#### Variants

##### `Auto`

Use colors if the output device appears to support them

##### `AlwaysAnsi`

Like `Always`, except it never tries to use anything other than emitting ANSI
color codes.

##### `Always`

Try very hard to emit colors.

This includes emitting ANSI colors on Windows if the console API is unavailable.

##### `Never`

Never emit colors.

#### Implementations

##### Methods

- ```rust
  pub fn global() -> Self { /* ... */ }
  ```
  Get the current [`ColorChoice`] state

- ```rust
  pub fn write_global(self: Self) { /* ... */ }
  ```
  Override the detected [`ColorChoice`]

##### Trait Implementations

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ColorChoice { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ColorChoice) -> bool { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

