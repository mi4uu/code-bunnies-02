# Crate Documentation

**Version:** 0.1.9

**Format Version:** 43

# Module `cargo_platform`

Platform definition used by Cargo.

This defines a [`Platform`] type which is used in Cargo to specify a target platform.
There are two kinds, a named target like `x86_64-apple-darwin`, and a "cfg expression"
like `cfg(any(target_os = "macos", target_os = "ios"))`.

See `examples/matches.rs` for an example of how to match against a `Platform`.

> This crate is maintained by the Cargo team for use by the wider
> ecosystem. This crate follows semver compatibility for its APIs.

[`Platform`]: enum.Platform.html

## Types

### Enum `Platform`

Platform definition.

```rust
pub enum Platform {
    Name(String),
    Cfg(CfgExpr),
}
```

#### Variants

##### `Name`

A named platform, like `x86_64-apple-darwin`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### `Cfg`

A cfg expression, like `cfg(windows)`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CfgExpr` |  |

#### Implementations

##### Methods

- ```rust
  pub fn matches(self: &Self, name: &str, cfg: &[Cfg]) -> bool { /* ... */ }
  ```
  Returns whether the Platform matches the given target and cfg.

- ```rust
  pub fn check_cfg_attributes(self: &Self, warnings: &mut Vec<String>) { /* ... */ }
  ```

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, s: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: serde::Serializer { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Send**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Platform, ParseError> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Platform) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: serde::Deserializer<''de> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Platform { /* ... */ }
    ```

- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DeserializeOwned**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Platform) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Platform) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

## Re-exports

### Re-export `Cfg`

```rust
pub use cfg::Cfg;
```

### Re-export `CfgExpr`

```rust
pub use cfg::CfgExpr;
```

### Re-export `ParseError`

```rust
pub use error::ParseError;
```

### Re-export `ParseErrorKind`

```rust
pub use error::ParseErrorKind;
```

