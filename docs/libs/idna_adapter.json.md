# Crate Documentation

**Version:** 1.2.0

**Format Version:** 43

# Module `idna_adapter`

This crate abstracts over a Unicode back end for the [`idna`][1]
crate.

To work around the lack of [`global-features`][2] in Cargo, this
crate allows the top level `Cargo.lock` to choose an alternative
Unicode back end for the `idna` crate by pinning a version of this
crate.

See the [README of the latest version][3] for more details.

[1]: https://docs.rs/crate/idna/latest
[2]: https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618
[3]: https://docs.rs/crate/idna_adapter/latest

## Types

### Struct `JoiningType`

Value for the Joining_Type Unicode property.

```rust
pub struct JoiningType(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn to_mask(self: Self) -> JoiningTypeMask { /* ... */ }
  ```
  Returns the corresponding `JoiningTypeMask`.

- ```rust
  pub fn is_transparent(self: Self) -> bool { /* ... */ }
  ```

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> JoiningType { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
### Struct `JoiningTypeMask`

A mask representing potentially multiple `JoiningType`
values.

```rust
pub struct JoiningTypeMask(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn intersects(self: Self, other: JoiningTypeMask) -> bool { /* ... */ }
  ```
  `true` iff both masks have at `JoiningType` in common.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> JoiningTypeMask { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **RefUnwindSafe**
- **UnwindSafe**
### Struct `BidiClass`

Value for the Bidi_Class Unicode property.

```rust
pub struct BidiClass(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn to_mask(self: Self) -> BidiClassMask { /* ... */ }
  ```
  Returns the corresponding `BidiClassMask`.

- ```rust
  pub fn is_ltr(self: Self) -> bool { /* ... */ }
  ```
  `true` iff this value is Left_To_Right

- ```rust
  pub fn is_nonspacing_mark(self: Self) -> bool { /* ... */ }
  ```
  `true` iff this value is Nonspacing_Mark

- ```rust
  pub fn is_european_number(self: Self) -> bool { /* ... */ }
  ```
  `true` iff this value is European_Number

- ```rust
  pub fn is_arabic_number(self: Self) -> bool { /* ... */ }
  ```
  `true` iff this value is Arabic_Number

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BidiClass { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Copy**
- **Sync**
- **ErasedDestructor**
- **Freeze**
- **Send**
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

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
### Struct `BidiClassMask`

A mask representing potentially multiple `BidiClass`
values.

```rust
pub struct BidiClassMask(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn intersects(self: Self, other: BidiClassMask) -> bool { /* ... */ }
  ```
  `true` iff both masks have at `BidiClass` in common.

##### Trait Implementations

- **Send**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> BidiClassMask { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `Adapter`

An adapter between a Unicode back end an the `idna` crate.

```rust
pub struct Adapter {
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
  pub const fn new() -> Self { /* ... */ }
  ```
  Constructor using data compiled into the binary.

- ```rust
  pub fn is_virama(self: &Self, c: char) -> bool { /* ... */ }
  ```
  `true` iff the Canonical_Combining_Class of `c` is Virama.

- ```rust
  pub fn is_mark(self: &Self, c: char) -> bool { /* ... */ }
  ```
  `true` iff the General_Category of `c` is Mark, i.e. any of Nonspacing_Mark,

- ```rust
  pub fn bidi_class(self: &Self, c: char) -> BidiClass { /* ... */ }
  ```
  Returns the Bidi_Class of `c`.

- ```rust
  pub fn joining_type(self: &Self, c: char) -> JoiningType { /* ... */ }
  ```
  Returns the Joining_Type of `c`.

- ```rust
  pub fn map_normalize<''delegate, I: Iterator<Item = char> + ''delegate>(self: &''delegate Self, iter: I) -> impl Iterator<Item = char> + ''delegate { /* ... */ }
  ```
  See the [method of the same name in `icu_normalizer`][1] for the

- ```rust
  pub fn normalize_validate<''delegate, I: Iterator<Item = char> + ''delegate>(self: &''delegate Self, iter: I) -> impl Iterator<Item = char> + ''delegate { /* ... */ }
  ```
  See the [method of the same name in `icu_normalizer`][1] for the

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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
- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
## Constants and Statics

### Constant `LEFT_OR_DUAL_JOINING_MASK`

Mask for checking for both left and dual joining.

```rust
pub const LEFT_OR_DUAL_JOINING_MASK: JoiningTypeMask = _;
```

### Constant `RIGHT_OR_DUAL_JOINING_MASK`

Mask for checking for both left and dual joining.

```rust
pub const RIGHT_OR_DUAL_JOINING_MASK: JoiningTypeMask = _;
```

### Constant `RTL_MASK`

Mask for checking if the domain is a bidi domain.

```rust
pub const RTL_MASK: BidiClassMask = _;
```

### Constant `FIRST_BC_MASK`

Mask for allowable bidi classes in the first character of a label
(either LTR or RTL) in a bidi domain.

```rust
pub const FIRST_BC_MASK: BidiClassMask = _;
```

### Constant `LAST_LTR_MASK`

```rust
pub const LAST_LTR_MASK: BidiClassMask = _;
```

### Constant `LAST_RTL_MASK`

```rust
pub const LAST_RTL_MASK: BidiClassMask = _;
```

### Constant `MIDDLE_LTR_MASK`

```rust
pub const MIDDLE_LTR_MASK: BidiClassMask = _;
```

### Constant `MIDDLE_RTL_MASK`

```rust
pub const MIDDLE_RTL_MASK: BidiClassMask = _;
```

