# Crate Documentation

**Version:** 1.0.0

**Format Version:** 43

# Module `bipatch`

## Types

### Enum `DecodeError`

```rust
pub enum DecodeError {
    IO(io::Error),
    WrongMagic(u32),
    WrongVersion(u32),
}
```

#### Variants

##### `IO`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `io::Error` |  |

##### `WrongMagic`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### `WrongVersion`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: io::Error) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn StdError + ''static> { /* ... */ }
    ```

### Struct `Reader`

```rust
pub struct Reader<R, RS> {
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
  pub fn new(patch: R, old: RS) -> Result<Self, DecodeError> { /* ... */ }
  ```

##### Trait Implementations

- **Freeze**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **FixedIntReader**
  - ```rust
    fn read_fixedint<FI>(self: &mut Self) -> Result<FI, Error>
where
    FI: FixedInt { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

- **ReadBytesExt**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VarIntReader**
  - ```rust
    fn read_varint<VI>(self: &mut Self) -> Result<VI, Error>
where
    VI: VarInt { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Constants and Statics

### Constant `MAGIC`

```rust
pub const MAGIC: u32 = 0xB1DF;
```

### Constant `VERSION`

```rust
pub const VERSION: u32 = 0x1000;
```

