# Crate Documentation

**Version:** 0.1.1

**Format Version:** 43

# Module `foreign_types_shared`

Internal crate used by foreign-types

## Types

### Struct `Opaque`

An opaque type used to define `ForeignTypeRef` types.

A type implementing `ForeignTypeRef` should simply be a newtype wrapper around this type.

```rust
pub struct Opaque(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
## Traits

### Trait `ForeignType`

A type implemented by wrappers over foreign types.

```rust
pub trait ForeignType: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `CType`: The raw C type.
- `Ref`: The type representing a reference to this type.

##### Required Methods

- `from_ptr`: Constructs an instance of this type from its raw type.
- `as_ptr`: Returns a raw pointer to the wrapped value.

### Trait `ForeignTypeRef`

A trait implemented by types which reference borrowed foreign types.

```rust
pub trait ForeignTypeRef: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `CType`: The raw C type.

#### Provided Methods

- ```rust
  unsafe fn from_ptr<''a>(ptr: *mut <Self as >::CType) -> &''a Self { /* ... */ }
  ```
  Constructs a shared instance of this type from its raw type.

- ```rust
  unsafe fn from_ptr_mut<''a>(ptr: *mut <Self as >::CType) -> &''a mut Self { /* ... */ }
  ```
  Constructs a mutable reference of this type from its raw type.

- ```rust
  fn as_ptr(self: &Self) -> *mut <Self as >::CType { /* ... */ }
  ```
  Returns a raw pointer to the wrapped value.

