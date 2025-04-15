# Crate Documentation

**Version:** 0.31.1

**Format Version:** 43

# Module `gimli`

`gimli` is a library for reading and writing the
[DWARF debugging format](https://dwarfstd.org/).

See the [read](./read/index.html) and [write](./write/index.html) modules
for examples and API documentation.

## Cargo Features

Cargo features that can be enabled with `gimli`:

* `std`: Enabled by default. Use the `std` library. Disabling this feature
  allows using `gimli` in embedded environments that do not have access to
  `std`. Note that even when `std` is disabled, `gimli` still requires an
  implementation of the `alloc` crate.

* `read`: Enabled by default. Enables the `read` module. Use of `std` is
  optional.

* `write`: Enabled by default. Enables the `write` module. Always uses
  the `std` library.

## Modules

## Module `constants`

**Attributes:**

- `#[allow(non_upper_case_globals)]`
- `#[allow(missing_docs)]`

Constant definitions.

The DWARF spec's `DW_AT_*` type is represented as `struct DwAt(u16)`,
`DW_FORM_*` as `DwForm(u16)`, etc.

There are also exported const definitions for each constant.

```rust
pub mod constants { /* ... */ }
```

### Types

#### Struct `DwSect`

The section type field in a `.dwp` unit index.

This is used for version 5 and later.

See Section 7.3.5.

```rust
pub struct DwSect(pub u32);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwSect) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwSect) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwSect) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwSect { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `DwSectV2`

The section type field in a `.dwp` unit index with version 2.

```rust
pub struct DwSectV2(pub u32);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwSectV2) -> bool { /* ... */ }
    ```

- **Eq**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwSectV2) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwSectV2) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwSectV2 { /* ... */ }
    ```

- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
#### Struct `DwUt`

The unit type field in a unit header.

See Section 7.5.1, Table 7.2.

```rust
pub struct DwUt(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwUt) -> bool { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwUt) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwUt { /* ... */ }
    ```

- **Copy**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwUt) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

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
#### Struct `DwCfa`

The opcode for a call frame instruction.

Section 7.24:
> Call frame instructions are encoded in one or more bytes. The primary
> opcode is encoded in the high order two bits of the first byte (that is,
> opcode = byte >> 6). An operand or extended opcode may be encoded in the
> low order 6 bits. Additional operands are encoded in subsequent bytes.

```rust
pub struct DwCfa(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwCfa { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwCfa) -> bool { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwCfa) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwCfa) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `DwChildren`

The child determination encodings for DIE attributes.

See Section 7.5.3, Table 7.4.

```rust
pub struct DwChildren(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwChildren) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwChildren) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Copy**
- **Freeze**
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwChildren { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwChildren) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `DwTag`

The tag encodings for DIE attributes.

See Section 7.5.3, Table 7.3.

```rust
pub struct DwTag(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwTag) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Eq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwTag) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwTag { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwTag) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `DwAt`

The attribute encodings for DIE attributes.

See Section 7.5.4, Table 7.5.

```rust
pub struct DwAt(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwAt) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwAt { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwAt) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwAt) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `DwForm`

The attribute form encodings for DIE attributes.

See Section 7.5.6, Table 7.6.

```rust
pub struct DwForm(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwForm) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwForm) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Unpin**
- **Send**
- **Sync**
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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwForm { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwForm) -> bool { /* ... */ }
    ```

- **Eq**
#### Struct `DwAte`

The encodings of the constants used in the `DW_AT_encoding` attribute.

See Section 7.8, Table 7.11.

```rust
pub struct DwAte(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwAte { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Unpin**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwAte) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwAte) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwAte) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

#### Struct `DwLle`

The encodings of the constants used in location list entries.

See Section 7.7.3, Table 7.10.

```rust
pub struct DwLle(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Sync**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Copy**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwLle) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwLle { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwLle) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwLle) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

#### Struct `DwDs`

The encodings of the constants used in the `DW_AT_decimal_sign` attribute.

See Section 7.8, Table 7.12.

```rust
pub struct DwDs(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwDs { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwDs) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwDs) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwDs) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

#### Struct `DwEnd`

The encodings of the constants used in the `DW_AT_endianity` attribute.

See Section 7.8, Table 7.13.

```rust
pub struct DwEnd(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwEnd) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwEnd) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **RefUnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwEnd) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwEnd { /* ... */ }
    ```

- **Send**
- **Freeze**
- **StructuralPartialEq**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `DwAccess`

The encodings of the constants used in the `DW_AT_accessibility` attribute.

See Section 7.9, Table 7.14.

```rust
pub struct DwAccess(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwAccess) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
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

- **StructuralPartialEq**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwAccess) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwAccess) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwAccess { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
#### Struct `DwVis`

The encodings of the constants used in the `DW_AT_visibility` attribute.

See Section 7.10, Table 7.15.

```rust
pub struct DwVis(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwVis) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwVis) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwVis) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwVis { /* ... */ }
    ```

#### Struct `DwVirtuality`

The encodings of the constants used in the `DW_AT_virtuality` attribute.

See Section 7.11, Table 7.16.

```rust
pub struct DwVirtuality(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwVirtuality { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwVirtuality) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwVirtuality) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwVirtuality) -> bool { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `DwLang`

The encodings of the constants used in the `DW_AT_language` attribute.

See Section 7.12, Table 7.17.

```rust
pub struct DwLang(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

- ```rust
  pub fn default_lower_bound(self: Self) -> Option<usize> { /* ... */ }
  ```
  Get the default DW_AT_lower_bound for this language.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwLang { /* ... */ }
    ```

- **StructuralPartialEq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwLang) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwLang) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwLang) -> bool { /* ... */ }
    ```

- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `DwAddr`

The encodings of the constants used in the `DW_AT_address_class` attribute.

There is only one value that is common to all target architectures.
See Section 7.13.

```rust
pub struct DwAddr(pub u64);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwAddr) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwAddr) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwAddr) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwAddr { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `DwId`

The encodings of the constants used in the `DW_AT_identifier_case` attribute.

See Section 7.14, Table 7.18.

```rust
pub struct DwId(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwId) -> bool { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwId) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwId) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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

#### Struct `DwCc`

The encodings of the constants used in the `DW_AT_calling_convention` attribute.

See Section 7.15, Table 7.19.

```rust
pub struct DwCc(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwCc) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwCc) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Copy**
- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **UnwindSafe**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwCc { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwCc) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

#### Struct `DwInl`

The encodings of the constants used in the `DW_AT_inline` attribute.

See Section 7.16, Table 7.20.

```rust
pub struct DwInl(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwInl) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwInl) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwInl { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwInl) -> bool { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
#### Struct `DwOrd`

The encodings of the constants used in the `DW_AT_ordering` attribute.

See Section 7.17, Table 7.17.

```rust
pub struct DwOrd(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwOrd { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Eq**
- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwOrd) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwOrd) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwOrd) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `DwDsc`

The encodings of the constants used in the `DW_AT_discr_list` attribute.

See Section 7.18, Table 7.22.

```rust
pub struct DwDsc(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwDsc { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwDsc) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwDsc) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwDsc) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Copy**
- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `DwIdx`

Name index attribute encodings.

See Section 7.19, Table 7.23.

```rust
pub struct DwIdx(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwIdx { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Copy**
- **Eq**
- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwIdx) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwIdx) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwIdx) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
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

#### Struct `DwDefaulted`

The encodings of the constants used in the `DW_AT_defaulted` attribute.

See Section 7.20, Table 7.24.

```rust
pub struct DwDefaulted(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwDefaulted { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwDefaulted) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwDefaulted) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwDefaulted) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
#### Struct `DwLns`

The encodings for the standard opcodes for line number information.

See Section 7.22, Table 7.25.

```rust
pub struct DwLns(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwLns) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwLns) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwLns) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwLns { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `DwLne`

The encodings for the extended opcodes for line number information.

See Section 7.22, Table 7.26.

```rust
pub struct DwLne(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwLne) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwLne) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwLne { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwLne) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `DwLnct`

The encodings for the line number header entry formats.

See Section 7.22, Table 7.27.

```rust
pub struct DwLnct(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Eq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwLnct) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwLnct) -> bool { /* ... */ }
    ```

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

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwLnct) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwLnct { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `DwMacro`

The encodings for macro information entry types.

See Section 7.23, Table 7.28.

```rust
pub struct DwMacro(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwMacro) -> bool { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwMacro { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwMacro) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwMacro) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `DwRle`

Range list entry encoding values.

See Section 7.25, Table 7.30.

```rust
pub struct DwRle(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwRle) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwRle { /* ... */ }
    ```

- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwRle) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwRle) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `DwOp`

The encodings for DWARF expression operations.

See Section 7.7.1, Table 7.9.

```rust
pub struct DwOp(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwOp) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwOp) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
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

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwOp) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwOp { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `DwEhPe`

Pointer encoding used by `.eh_frame`.

The four lower bits describe the
format of the pointer, the upper four bits describe how the encoding should
be applied.

Defined in `<https://refspecs.linuxfoundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html>`

```rust
pub struct DwEhPe(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn static_string(self: &Self) -> Option<&''static str> { /* ... */ }
  ```

- ```rust
  pub fn format(self: Self) -> DwEhPe { /* ... */ }
  ```
  Get the pointer encoding's format.

- ```rust
  pub fn application(self: Self) -> DwEhPe { /* ... */ }
  ```
  Get the pointer encoding's application.

- ```rust
  pub fn is_absent(self: Self) -> bool { /* ... */ }
  ```
  Is this encoding the absent pointer encoding?

- ```rust
  pub fn is_indirect(self: Self) -> bool { /* ... */ }
  ```
  Is this coding indirect? If so, its encoded value is the address of the

- ```rust
  pub fn is_valid_encoding(self: Self) -> bool { /* ... */ }
  ```
  Is this a known, valid pointer encoding?

###### Trait Implementations

- **StructuralPartialEq**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DwEhPe) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: DwEhPe) -> DwEhPe { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DwEhPe { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DwEhPe) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DwEhPe) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Constants and Statics

#### Constant `DW_SECT_INFO`

```rust
pub const DW_SECT_INFO: DwSect = _;
```

#### Constant `DW_SECT_ABBREV`

```rust
pub const DW_SECT_ABBREV: DwSect = _;
```

#### Constant `DW_SECT_LINE`

```rust
pub const DW_SECT_LINE: DwSect = _;
```

#### Constant `DW_SECT_LOCLISTS`

```rust
pub const DW_SECT_LOCLISTS: DwSect = _;
```

#### Constant `DW_SECT_STR_OFFSETS`

```rust
pub const DW_SECT_STR_OFFSETS: DwSect = _;
```

#### Constant `DW_SECT_MACRO`

```rust
pub const DW_SECT_MACRO: DwSect = _;
```

#### Constant `DW_SECT_RNGLISTS`

```rust
pub const DW_SECT_RNGLISTS: DwSect = _;
```

#### Constant `DW_SECT_V2_INFO`

```rust
pub const DW_SECT_V2_INFO: DwSectV2 = _;
```

#### Constant `DW_SECT_V2_TYPES`

```rust
pub const DW_SECT_V2_TYPES: DwSectV2 = _;
```

#### Constant `DW_SECT_V2_ABBREV`

```rust
pub const DW_SECT_V2_ABBREV: DwSectV2 = _;
```

#### Constant `DW_SECT_V2_LINE`

```rust
pub const DW_SECT_V2_LINE: DwSectV2 = _;
```

#### Constant `DW_SECT_V2_LOC`

```rust
pub const DW_SECT_V2_LOC: DwSectV2 = _;
```

#### Constant `DW_SECT_V2_STR_OFFSETS`

```rust
pub const DW_SECT_V2_STR_OFFSETS: DwSectV2 = _;
```

#### Constant `DW_SECT_V2_MACINFO`

```rust
pub const DW_SECT_V2_MACINFO: DwSectV2 = _;
```

#### Constant `DW_SECT_V2_MACRO`

```rust
pub const DW_SECT_V2_MACRO: DwSectV2 = _;
```

#### Constant `DW_UT_compile`

```rust
pub const DW_UT_compile: DwUt = _;
```

#### Constant `DW_UT_type`

```rust
pub const DW_UT_type: DwUt = _;
```

#### Constant `DW_UT_partial`

```rust
pub const DW_UT_partial: DwUt = _;
```

#### Constant `DW_UT_skeleton`

```rust
pub const DW_UT_skeleton: DwUt = _;
```

#### Constant `DW_UT_split_compile`

```rust
pub const DW_UT_split_compile: DwUt = _;
```

#### Constant `DW_UT_split_type`

```rust
pub const DW_UT_split_type: DwUt = _;
```

#### Constant `DW_UT_lo_user`

```rust
pub const DW_UT_lo_user: DwUt = _;
```

#### Constant `DW_UT_hi_user`

```rust
pub const DW_UT_hi_user: DwUt = _;
```

#### Constant `DW_CFA_advance_loc`

```rust
pub const DW_CFA_advance_loc: DwCfa = _;
```

#### Constant `DW_CFA_offset`

```rust
pub const DW_CFA_offset: DwCfa = _;
```

#### Constant `DW_CFA_restore`

```rust
pub const DW_CFA_restore: DwCfa = _;
```

#### Constant `DW_CFA_nop`

```rust
pub const DW_CFA_nop: DwCfa = _;
```

#### Constant `DW_CFA_set_loc`

```rust
pub const DW_CFA_set_loc: DwCfa = _;
```

#### Constant `DW_CFA_advance_loc1`

```rust
pub const DW_CFA_advance_loc1: DwCfa = _;
```

#### Constant `DW_CFA_advance_loc2`

```rust
pub const DW_CFA_advance_loc2: DwCfa = _;
```

#### Constant `DW_CFA_advance_loc4`

```rust
pub const DW_CFA_advance_loc4: DwCfa = _;
```

#### Constant `DW_CFA_offset_extended`

```rust
pub const DW_CFA_offset_extended: DwCfa = _;
```

#### Constant `DW_CFA_restore_extended`

```rust
pub const DW_CFA_restore_extended: DwCfa = _;
```

#### Constant `DW_CFA_undefined`

```rust
pub const DW_CFA_undefined: DwCfa = _;
```

#### Constant `DW_CFA_same_value`

```rust
pub const DW_CFA_same_value: DwCfa = _;
```

#### Constant `DW_CFA_register`

```rust
pub const DW_CFA_register: DwCfa = _;
```

#### Constant `DW_CFA_remember_state`

```rust
pub const DW_CFA_remember_state: DwCfa = _;
```

#### Constant `DW_CFA_restore_state`

```rust
pub const DW_CFA_restore_state: DwCfa = _;
```

#### Constant `DW_CFA_def_cfa`

```rust
pub const DW_CFA_def_cfa: DwCfa = _;
```

#### Constant `DW_CFA_def_cfa_register`

```rust
pub const DW_CFA_def_cfa_register: DwCfa = _;
```

#### Constant `DW_CFA_def_cfa_offset`

```rust
pub const DW_CFA_def_cfa_offset: DwCfa = _;
```

#### Constant `DW_CFA_def_cfa_expression`

```rust
pub const DW_CFA_def_cfa_expression: DwCfa = _;
```

#### Constant `DW_CFA_expression`

```rust
pub const DW_CFA_expression: DwCfa = _;
```

#### Constant `DW_CFA_offset_extended_sf`

```rust
pub const DW_CFA_offset_extended_sf: DwCfa = _;
```

#### Constant `DW_CFA_def_cfa_sf`

```rust
pub const DW_CFA_def_cfa_sf: DwCfa = _;
```

#### Constant `DW_CFA_def_cfa_offset_sf`

```rust
pub const DW_CFA_def_cfa_offset_sf: DwCfa = _;
```

#### Constant `DW_CFA_val_offset`

```rust
pub const DW_CFA_val_offset: DwCfa = _;
```

#### Constant `DW_CFA_val_offset_sf`

```rust
pub const DW_CFA_val_offset_sf: DwCfa = _;
```

#### Constant `DW_CFA_val_expression`

```rust
pub const DW_CFA_val_expression: DwCfa = _;
```

#### Constant `DW_CFA_lo_user`

```rust
pub const DW_CFA_lo_user: DwCfa = _;
```

#### Constant `DW_CFA_hi_user`

```rust
pub const DW_CFA_hi_user: DwCfa = _;
```

#### Constant `DW_CFA_MIPS_advance_loc8`

```rust
pub const DW_CFA_MIPS_advance_loc8: DwCfa = _;
```

#### Constant `DW_CFA_GNU_window_save`

```rust
pub const DW_CFA_GNU_window_save: DwCfa = _;
```

#### Constant `DW_CFA_GNU_args_size`

```rust
pub const DW_CFA_GNU_args_size: DwCfa = _;
```

#### Constant `DW_CFA_GNU_negative_offset_extended`

```rust
pub const DW_CFA_GNU_negative_offset_extended: DwCfa = _;
```

#### Constant `DW_CFA_AARCH64_negate_ra_state`

```rust
pub const DW_CFA_AARCH64_negate_ra_state: DwCfa = _;
```

#### Constant `DW_CHILDREN_no`

```rust
pub const DW_CHILDREN_no: DwChildren = _;
```

#### Constant `DW_CHILDREN_yes`

```rust
pub const DW_CHILDREN_yes: DwChildren = _;
```

#### Constant `DW_TAG_null`

```rust
pub const DW_TAG_null: DwTag = _;
```

#### Constant `DW_TAG_array_type`

```rust
pub const DW_TAG_array_type: DwTag = _;
```

#### Constant `DW_TAG_class_type`

```rust
pub const DW_TAG_class_type: DwTag = _;
```

#### Constant `DW_TAG_entry_point`

```rust
pub const DW_TAG_entry_point: DwTag = _;
```

#### Constant `DW_TAG_enumeration_type`

```rust
pub const DW_TAG_enumeration_type: DwTag = _;
```

#### Constant `DW_TAG_formal_parameter`

```rust
pub const DW_TAG_formal_parameter: DwTag = _;
```

#### Constant `DW_TAG_imported_declaration`

```rust
pub const DW_TAG_imported_declaration: DwTag = _;
```

#### Constant `DW_TAG_label`

```rust
pub const DW_TAG_label: DwTag = _;
```

#### Constant `DW_TAG_lexical_block`

```rust
pub const DW_TAG_lexical_block: DwTag = _;
```

#### Constant `DW_TAG_member`

```rust
pub const DW_TAG_member: DwTag = _;
```

#### Constant `DW_TAG_pointer_type`

```rust
pub const DW_TAG_pointer_type: DwTag = _;
```

#### Constant `DW_TAG_reference_type`

```rust
pub const DW_TAG_reference_type: DwTag = _;
```

#### Constant `DW_TAG_compile_unit`

```rust
pub const DW_TAG_compile_unit: DwTag = _;
```

#### Constant `DW_TAG_string_type`

```rust
pub const DW_TAG_string_type: DwTag = _;
```

#### Constant `DW_TAG_structure_type`

```rust
pub const DW_TAG_structure_type: DwTag = _;
```

#### Constant `DW_TAG_subroutine_type`

```rust
pub const DW_TAG_subroutine_type: DwTag = _;
```

#### Constant `DW_TAG_typedef`

```rust
pub const DW_TAG_typedef: DwTag = _;
```

#### Constant `DW_TAG_union_type`

```rust
pub const DW_TAG_union_type: DwTag = _;
```

#### Constant `DW_TAG_unspecified_parameters`

```rust
pub const DW_TAG_unspecified_parameters: DwTag = _;
```

#### Constant `DW_TAG_variant`

```rust
pub const DW_TAG_variant: DwTag = _;
```

#### Constant `DW_TAG_common_block`

```rust
pub const DW_TAG_common_block: DwTag = _;
```

#### Constant `DW_TAG_common_inclusion`

```rust
pub const DW_TAG_common_inclusion: DwTag = _;
```

#### Constant `DW_TAG_inheritance`

```rust
pub const DW_TAG_inheritance: DwTag = _;
```

#### Constant `DW_TAG_inlined_subroutine`

```rust
pub const DW_TAG_inlined_subroutine: DwTag = _;
```

#### Constant `DW_TAG_module`

```rust
pub const DW_TAG_module: DwTag = _;
```

#### Constant `DW_TAG_ptr_to_member_type`

```rust
pub const DW_TAG_ptr_to_member_type: DwTag = _;
```

#### Constant `DW_TAG_set_type`

```rust
pub const DW_TAG_set_type: DwTag = _;
```

#### Constant `DW_TAG_subrange_type`

```rust
pub const DW_TAG_subrange_type: DwTag = _;
```

#### Constant `DW_TAG_with_stmt`

```rust
pub const DW_TAG_with_stmt: DwTag = _;
```

#### Constant `DW_TAG_access_declaration`

```rust
pub const DW_TAG_access_declaration: DwTag = _;
```

#### Constant `DW_TAG_base_type`

```rust
pub const DW_TAG_base_type: DwTag = _;
```

#### Constant `DW_TAG_catch_block`

```rust
pub const DW_TAG_catch_block: DwTag = _;
```

#### Constant `DW_TAG_const_type`

```rust
pub const DW_TAG_const_type: DwTag = _;
```

#### Constant `DW_TAG_constant`

```rust
pub const DW_TAG_constant: DwTag = _;
```

#### Constant `DW_TAG_enumerator`

```rust
pub const DW_TAG_enumerator: DwTag = _;
```

#### Constant `DW_TAG_file_type`

```rust
pub const DW_TAG_file_type: DwTag = _;
```

#### Constant `DW_TAG_friend`

```rust
pub const DW_TAG_friend: DwTag = _;
```

#### Constant `DW_TAG_namelist`

```rust
pub const DW_TAG_namelist: DwTag = _;
```

#### Constant `DW_TAG_namelist_item`

```rust
pub const DW_TAG_namelist_item: DwTag = _;
```

#### Constant `DW_TAG_packed_type`

```rust
pub const DW_TAG_packed_type: DwTag = _;
```

#### Constant `DW_TAG_subprogram`

```rust
pub const DW_TAG_subprogram: DwTag = _;
```

#### Constant `DW_TAG_template_type_parameter`

```rust
pub const DW_TAG_template_type_parameter: DwTag = _;
```

#### Constant `DW_TAG_template_value_parameter`

```rust
pub const DW_TAG_template_value_parameter: DwTag = _;
```

#### Constant `DW_TAG_thrown_type`

```rust
pub const DW_TAG_thrown_type: DwTag = _;
```

#### Constant `DW_TAG_try_block`

```rust
pub const DW_TAG_try_block: DwTag = _;
```

#### Constant `DW_TAG_variant_part`

```rust
pub const DW_TAG_variant_part: DwTag = _;
```

#### Constant `DW_TAG_variable`

```rust
pub const DW_TAG_variable: DwTag = _;
```

#### Constant `DW_TAG_volatile_type`

```rust
pub const DW_TAG_volatile_type: DwTag = _;
```

#### Constant `DW_TAG_dwarf_procedure`

```rust
pub const DW_TAG_dwarf_procedure: DwTag = _;
```

#### Constant `DW_TAG_restrict_type`

```rust
pub const DW_TAG_restrict_type: DwTag = _;
```

#### Constant `DW_TAG_interface_type`

```rust
pub const DW_TAG_interface_type: DwTag = _;
```

#### Constant `DW_TAG_namespace`

```rust
pub const DW_TAG_namespace: DwTag = _;
```

#### Constant `DW_TAG_imported_module`

```rust
pub const DW_TAG_imported_module: DwTag = _;
```

#### Constant `DW_TAG_unspecified_type`

```rust
pub const DW_TAG_unspecified_type: DwTag = _;
```

#### Constant `DW_TAG_partial_unit`

```rust
pub const DW_TAG_partial_unit: DwTag = _;
```

#### Constant `DW_TAG_imported_unit`

```rust
pub const DW_TAG_imported_unit: DwTag = _;
```

#### Constant `DW_TAG_condition`

```rust
pub const DW_TAG_condition: DwTag = _;
```

#### Constant `DW_TAG_shared_type`

```rust
pub const DW_TAG_shared_type: DwTag = _;
```

#### Constant `DW_TAG_type_unit`

```rust
pub const DW_TAG_type_unit: DwTag = _;
```

#### Constant `DW_TAG_rvalue_reference_type`

```rust
pub const DW_TAG_rvalue_reference_type: DwTag = _;
```

#### Constant `DW_TAG_template_alias`

```rust
pub const DW_TAG_template_alias: DwTag = _;
```

#### Constant `DW_TAG_coarray_type`

```rust
pub const DW_TAG_coarray_type: DwTag = _;
```

#### Constant `DW_TAG_generic_subrange`

```rust
pub const DW_TAG_generic_subrange: DwTag = _;
```

#### Constant `DW_TAG_dynamic_type`

```rust
pub const DW_TAG_dynamic_type: DwTag = _;
```

#### Constant `DW_TAG_atomic_type`

```rust
pub const DW_TAG_atomic_type: DwTag = _;
```

#### Constant `DW_TAG_call_site`

```rust
pub const DW_TAG_call_site: DwTag = _;
```

#### Constant `DW_TAG_call_site_parameter`

```rust
pub const DW_TAG_call_site_parameter: DwTag = _;
```

#### Constant `DW_TAG_skeleton_unit`

```rust
pub const DW_TAG_skeleton_unit: DwTag = _;
```

#### Constant `DW_TAG_immutable_type`

```rust
pub const DW_TAG_immutable_type: DwTag = _;
```

#### Constant `DW_TAG_lo_user`

```rust
pub const DW_TAG_lo_user: DwTag = _;
```

#### Constant `DW_TAG_hi_user`

```rust
pub const DW_TAG_hi_user: DwTag = _;
```

#### Constant `DW_TAG_MIPS_loop`

```rust
pub const DW_TAG_MIPS_loop: DwTag = _;
```

#### Constant `DW_TAG_HP_array_descriptor`

```rust
pub const DW_TAG_HP_array_descriptor: DwTag = _;
```

#### Constant `DW_TAG_HP_Bliss_field`

```rust
pub const DW_TAG_HP_Bliss_field: DwTag = _;
```

#### Constant `DW_TAG_HP_Bliss_field_set`

```rust
pub const DW_TAG_HP_Bliss_field_set: DwTag = _;
```

#### Constant `DW_TAG_format_label`

```rust
pub const DW_TAG_format_label: DwTag = _;
```

#### Constant `DW_TAG_function_template`

```rust
pub const DW_TAG_function_template: DwTag = _;
```

#### Constant `DW_TAG_class_template`

```rust
pub const DW_TAG_class_template: DwTag = _;
```

#### Constant `DW_TAG_GNU_BINCL`

```rust
pub const DW_TAG_GNU_BINCL: DwTag = _;
```

#### Constant `DW_TAG_GNU_EINCL`

```rust
pub const DW_TAG_GNU_EINCL: DwTag = _;
```

#### Constant `DW_TAG_GNU_template_template_param`

```rust
pub const DW_TAG_GNU_template_template_param: DwTag = _;
```

#### Constant `DW_TAG_GNU_template_parameter_pack`

```rust
pub const DW_TAG_GNU_template_parameter_pack: DwTag = _;
```

#### Constant `DW_TAG_GNU_formal_parameter_pack`

```rust
pub const DW_TAG_GNU_formal_parameter_pack: DwTag = _;
```

#### Constant `DW_TAG_GNU_call_site`

```rust
pub const DW_TAG_GNU_call_site: DwTag = _;
```

#### Constant `DW_TAG_GNU_call_site_parameter`

```rust
pub const DW_TAG_GNU_call_site_parameter: DwTag = _;
```

#### Constant `DW_TAG_APPLE_property`

```rust
pub const DW_TAG_APPLE_property: DwTag = _;
```

#### Constant `DW_TAG_SUN_function_template`

```rust
pub const DW_TAG_SUN_function_template: DwTag = _;
```

#### Constant `DW_TAG_SUN_class_template`

```rust
pub const DW_TAG_SUN_class_template: DwTag = _;
```

#### Constant `DW_TAG_SUN_struct_template`

```rust
pub const DW_TAG_SUN_struct_template: DwTag = _;
```

#### Constant `DW_TAG_SUN_union_template`

```rust
pub const DW_TAG_SUN_union_template: DwTag = _;
```

#### Constant `DW_TAG_SUN_indirect_inheritance`

```rust
pub const DW_TAG_SUN_indirect_inheritance: DwTag = _;
```

#### Constant `DW_TAG_SUN_codeflags`

```rust
pub const DW_TAG_SUN_codeflags: DwTag = _;
```

#### Constant `DW_TAG_SUN_memop_info`

```rust
pub const DW_TAG_SUN_memop_info: DwTag = _;
```

#### Constant `DW_TAG_SUN_omp_child_func`

```rust
pub const DW_TAG_SUN_omp_child_func: DwTag = _;
```

#### Constant `DW_TAG_SUN_rtti_descriptor`

```rust
pub const DW_TAG_SUN_rtti_descriptor: DwTag = _;
```

#### Constant `DW_TAG_SUN_dtor_info`

```rust
pub const DW_TAG_SUN_dtor_info: DwTag = _;
```

#### Constant `DW_TAG_SUN_dtor`

```rust
pub const DW_TAG_SUN_dtor: DwTag = _;
```

#### Constant `DW_TAG_SUN_f90_interface`

```rust
pub const DW_TAG_SUN_f90_interface: DwTag = _;
```

#### Constant `DW_TAG_SUN_fortran_vax_structure`

```rust
pub const DW_TAG_SUN_fortran_vax_structure: DwTag = _;
```

#### Constant `DW_TAG_ALTIUM_circ_type`

```rust
pub const DW_TAG_ALTIUM_circ_type: DwTag = _;
```

#### Constant `DW_TAG_ALTIUM_mwa_circ_type`

```rust
pub const DW_TAG_ALTIUM_mwa_circ_type: DwTag = _;
```

#### Constant `DW_TAG_ALTIUM_rev_carry_type`

```rust
pub const DW_TAG_ALTIUM_rev_carry_type: DwTag = _;
```

#### Constant `DW_TAG_ALTIUM_rom`

```rust
pub const DW_TAG_ALTIUM_rom: DwTag = _;
```

#### Constant `DW_TAG_upc_shared_type`

```rust
pub const DW_TAG_upc_shared_type: DwTag = _;
```

#### Constant `DW_TAG_upc_strict_type`

```rust
pub const DW_TAG_upc_strict_type: DwTag = _;
```

#### Constant `DW_TAG_upc_relaxed_type`

```rust
pub const DW_TAG_upc_relaxed_type: DwTag = _;
```

#### Constant `DW_TAG_PGI_kanji_type`

```rust
pub const DW_TAG_PGI_kanji_type: DwTag = _;
```

#### Constant `DW_TAG_PGI_interface_block`

```rust
pub const DW_TAG_PGI_interface_block: DwTag = _;
```

#### Constant `DW_TAG_BORLAND_property`

```rust
pub const DW_TAG_BORLAND_property: DwTag = _;
```

#### Constant `DW_TAG_BORLAND_Delphi_string`

```rust
pub const DW_TAG_BORLAND_Delphi_string: DwTag = _;
```

#### Constant `DW_TAG_BORLAND_Delphi_dynamic_array`

```rust
pub const DW_TAG_BORLAND_Delphi_dynamic_array: DwTag = _;
```

#### Constant `DW_TAG_BORLAND_Delphi_set`

```rust
pub const DW_TAG_BORLAND_Delphi_set: DwTag = _;
```

#### Constant `DW_TAG_BORLAND_Delphi_variant`

```rust
pub const DW_TAG_BORLAND_Delphi_variant: DwTag = _;
```

#### Constant `DW_AT_null`

```rust
pub const DW_AT_null: DwAt = _;
```

#### Constant `DW_AT_sibling`

```rust
pub const DW_AT_sibling: DwAt = _;
```

#### Constant `DW_AT_location`

```rust
pub const DW_AT_location: DwAt = _;
```

#### Constant `DW_AT_name`

```rust
pub const DW_AT_name: DwAt = _;
```

#### Constant `DW_AT_ordering`

```rust
pub const DW_AT_ordering: DwAt = _;
```

#### Constant `DW_AT_byte_size`

```rust
pub const DW_AT_byte_size: DwAt = _;
```

#### Constant `DW_AT_bit_offset`

```rust
pub const DW_AT_bit_offset: DwAt = _;
```

#### Constant `DW_AT_bit_size`

```rust
pub const DW_AT_bit_size: DwAt = _;
```

#### Constant `DW_AT_stmt_list`

```rust
pub const DW_AT_stmt_list: DwAt = _;
```

#### Constant `DW_AT_low_pc`

```rust
pub const DW_AT_low_pc: DwAt = _;
```

#### Constant `DW_AT_high_pc`

```rust
pub const DW_AT_high_pc: DwAt = _;
```

#### Constant `DW_AT_language`

```rust
pub const DW_AT_language: DwAt = _;
```

#### Constant `DW_AT_discr`

```rust
pub const DW_AT_discr: DwAt = _;
```

#### Constant `DW_AT_discr_value`

```rust
pub const DW_AT_discr_value: DwAt = _;
```

#### Constant `DW_AT_visibility`

```rust
pub const DW_AT_visibility: DwAt = _;
```

#### Constant `DW_AT_import`

```rust
pub const DW_AT_import: DwAt = _;
```

#### Constant `DW_AT_string_length`

```rust
pub const DW_AT_string_length: DwAt = _;
```

#### Constant `DW_AT_common_reference`

```rust
pub const DW_AT_common_reference: DwAt = _;
```

#### Constant `DW_AT_comp_dir`

```rust
pub const DW_AT_comp_dir: DwAt = _;
```

#### Constant `DW_AT_const_value`

```rust
pub const DW_AT_const_value: DwAt = _;
```

#### Constant `DW_AT_containing_type`

```rust
pub const DW_AT_containing_type: DwAt = _;
```

#### Constant `DW_AT_default_value`

```rust
pub const DW_AT_default_value: DwAt = _;
```

#### Constant `DW_AT_inline`

```rust
pub const DW_AT_inline: DwAt = _;
```

#### Constant `DW_AT_is_optional`

```rust
pub const DW_AT_is_optional: DwAt = _;
```

#### Constant `DW_AT_lower_bound`

```rust
pub const DW_AT_lower_bound: DwAt = _;
```

#### Constant `DW_AT_producer`

```rust
pub const DW_AT_producer: DwAt = _;
```

#### Constant `DW_AT_prototyped`

```rust
pub const DW_AT_prototyped: DwAt = _;
```

#### Constant `DW_AT_return_addr`

```rust
pub const DW_AT_return_addr: DwAt = _;
```

#### Constant `DW_AT_start_scope`

```rust
pub const DW_AT_start_scope: DwAt = _;
```

#### Constant `DW_AT_bit_stride`

```rust
pub const DW_AT_bit_stride: DwAt = _;
```

#### Constant `DW_AT_upper_bound`

```rust
pub const DW_AT_upper_bound: DwAt = _;
```

#### Constant `DW_AT_abstract_origin`

```rust
pub const DW_AT_abstract_origin: DwAt = _;
```

#### Constant `DW_AT_accessibility`

```rust
pub const DW_AT_accessibility: DwAt = _;
```

#### Constant `DW_AT_address_class`

```rust
pub const DW_AT_address_class: DwAt = _;
```

#### Constant `DW_AT_artificial`

```rust
pub const DW_AT_artificial: DwAt = _;
```

#### Constant `DW_AT_base_types`

```rust
pub const DW_AT_base_types: DwAt = _;
```

#### Constant `DW_AT_calling_convention`

```rust
pub const DW_AT_calling_convention: DwAt = _;
```

#### Constant `DW_AT_count`

```rust
pub const DW_AT_count: DwAt = _;
```

#### Constant `DW_AT_data_member_location`

```rust
pub const DW_AT_data_member_location: DwAt = _;
```

#### Constant `DW_AT_decl_column`

```rust
pub const DW_AT_decl_column: DwAt = _;
```

#### Constant `DW_AT_decl_file`

```rust
pub const DW_AT_decl_file: DwAt = _;
```

#### Constant `DW_AT_decl_line`

```rust
pub const DW_AT_decl_line: DwAt = _;
```

#### Constant `DW_AT_declaration`

```rust
pub const DW_AT_declaration: DwAt = _;
```

#### Constant `DW_AT_discr_list`

```rust
pub const DW_AT_discr_list: DwAt = _;
```

#### Constant `DW_AT_encoding`

```rust
pub const DW_AT_encoding: DwAt = _;
```

#### Constant `DW_AT_external`

```rust
pub const DW_AT_external: DwAt = _;
```

#### Constant `DW_AT_frame_base`

```rust
pub const DW_AT_frame_base: DwAt = _;
```

#### Constant `DW_AT_friend`

```rust
pub const DW_AT_friend: DwAt = _;
```

#### Constant `DW_AT_identifier_case`

```rust
pub const DW_AT_identifier_case: DwAt = _;
```

#### Constant `DW_AT_macro_info`

```rust
pub const DW_AT_macro_info: DwAt = _;
```

#### Constant `DW_AT_namelist_item`

```rust
pub const DW_AT_namelist_item: DwAt = _;
```

#### Constant `DW_AT_priority`

```rust
pub const DW_AT_priority: DwAt = _;
```

#### Constant `DW_AT_segment`

```rust
pub const DW_AT_segment: DwAt = _;
```

#### Constant `DW_AT_specification`

```rust
pub const DW_AT_specification: DwAt = _;
```

#### Constant `DW_AT_static_link`

```rust
pub const DW_AT_static_link: DwAt = _;
```

#### Constant `DW_AT_type`

```rust
pub const DW_AT_type: DwAt = _;
```

#### Constant `DW_AT_use_location`

```rust
pub const DW_AT_use_location: DwAt = _;
```

#### Constant `DW_AT_variable_parameter`

```rust
pub const DW_AT_variable_parameter: DwAt = _;
```

#### Constant `DW_AT_virtuality`

```rust
pub const DW_AT_virtuality: DwAt = _;
```

#### Constant `DW_AT_vtable_elem_location`

```rust
pub const DW_AT_vtable_elem_location: DwAt = _;
```

#### Constant `DW_AT_allocated`

```rust
pub const DW_AT_allocated: DwAt = _;
```

#### Constant `DW_AT_associated`

```rust
pub const DW_AT_associated: DwAt = _;
```

#### Constant `DW_AT_data_location`

```rust
pub const DW_AT_data_location: DwAt = _;
```

#### Constant `DW_AT_byte_stride`

```rust
pub const DW_AT_byte_stride: DwAt = _;
```

#### Constant `DW_AT_entry_pc`

```rust
pub const DW_AT_entry_pc: DwAt = _;
```

#### Constant `DW_AT_use_UTF8`

```rust
pub const DW_AT_use_UTF8: DwAt = _;
```

#### Constant `DW_AT_extension`

```rust
pub const DW_AT_extension: DwAt = _;
```

#### Constant `DW_AT_ranges`

```rust
pub const DW_AT_ranges: DwAt = _;
```

#### Constant `DW_AT_trampoline`

```rust
pub const DW_AT_trampoline: DwAt = _;
```

#### Constant `DW_AT_call_column`

```rust
pub const DW_AT_call_column: DwAt = _;
```

#### Constant `DW_AT_call_file`

```rust
pub const DW_AT_call_file: DwAt = _;
```

#### Constant `DW_AT_call_line`

```rust
pub const DW_AT_call_line: DwAt = _;
```

#### Constant `DW_AT_description`

```rust
pub const DW_AT_description: DwAt = _;
```

#### Constant `DW_AT_binary_scale`

```rust
pub const DW_AT_binary_scale: DwAt = _;
```

#### Constant `DW_AT_decimal_scale`

```rust
pub const DW_AT_decimal_scale: DwAt = _;
```

#### Constant `DW_AT_small`

```rust
pub const DW_AT_small: DwAt = _;
```

#### Constant `DW_AT_decimal_sign`

```rust
pub const DW_AT_decimal_sign: DwAt = _;
```

#### Constant `DW_AT_digit_count`

```rust
pub const DW_AT_digit_count: DwAt = _;
```

#### Constant `DW_AT_picture_string`

```rust
pub const DW_AT_picture_string: DwAt = _;
```

#### Constant `DW_AT_mutable`

```rust
pub const DW_AT_mutable: DwAt = _;
```

#### Constant `DW_AT_threads_scaled`

```rust
pub const DW_AT_threads_scaled: DwAt = _;
```

#### Constant `DW_AT_explicit`

```rust
pub const DW_AT_explicit: DwAt = _;
```

#### Constant `DW_AT_object_pointer`

```rust
pub const DW_AT_object_pointer: DwAt = _;
```

#### Constant `DW_AT_endianity`

```rust
pub const DW_AT_endianity: DwAt = _;
```

#### Constant `DW_AT_elemental`

```rust
pub const DW_AT_elemental: DwAt = _;
```

#### Constant `DW_AT_pure`

```rust
pub const DW_AT_pure: DwAt = _;
```

#### Constant `DW_AT_recursive`

```rust
pub const DW_AT_recursive: DwAt = _;
```

#### Constant `DW_AT_signature`

```rust
pub const DW_AT_signature: DwAt = _;
```

#### Constant `DW_AT_main_subprogram`

```rust
pub const DW_AT_main_subprogram: DwAt = _;
```

#### Constant `DW_AT_data_bit_offset`

```rust
pub const DW_AT_data_bit_offset: DwAt = _;
```

#### Constant `DW_AT_const_expr`

```rust
pub const DW_AT_const_expr: DwAt = _;
```

#### Constant `DW_AT_enum_class`

```rust
pub const DW_AT_enum_class: DwAt = _;
```

#### Constant `DW_AT_linkage_name`

```rust
pub const DW_AT_linkage_name: DwAt = _;
```

#### Constant `DW_AT_string_length_bit_size`

```rust
pub const DW_AT_string_length_bit_size: DwAt = _;
```

#### Constant `DW_AT_string_length_byte_size`

```rust
pub const DW_AT_string_length_byte_size: DwAt = _;
```

#### Constant `DW_AT_rank`

```rust
pub const DW_AT_rank: DwAt = _;
```

#### Constant `DW_AT_str_offsets_base`

```rust
pub const DW_AT_str_offsets_base: DwAt = _;
```

#### Constant `DW_AT_addr_base`

```rust
pub const DW_AT_addr_base: DwAt = _;
```

#### Constant `DW_AT_rnglists_base`

```rust
pub const DW_AT_rnglists_base: DwAt = _;
```

#### Constant `DW_AT_dwo_name`

```rust
pub const DW_AT_dwo_name: DwAt = _;
```

#### Constant `DW_AT_reference`

```rust
pub const DW_AT_reference: DwAt = _;
```

#### Constant `DW_AT_rvalue_reference`

```rust
pub const DW_AT_rvalue_reference: DwAt = _;
```

#### Constant `DW_AT_macros`

```rust
pub const DW_AT_macros: DwAt = _;
```

#### Constant `DW_AT_call_all_calls`

```rust
pub const DW_AT_call_all_calls: DwAt = _;
```

#### Constant `DW_AT_call_all_source_calls`

```rust
pub const DW_AT_call_all_source_calls: DwAt = _;
```

#### Constant `DW_AT_call_all_tail_calls`

```rust
pub const DW_AT_call_all_tail_calls: DwAt = _;
```

#### Constant `DW_AT_call_return_pc`

```rust
pub const DW_AT_call_return_pc: DwAt = _;
```

#### Constant `DW_AT_call_value`

```rust
pub const DW_AT_call_value: DwAt = _;
```

#### Constant `DW_AT_call_origin`

```rust
pub const DW_AT_call_origin: DwAt = _;
```

#### Constant `DW_AT_call_parameter`

```rust
pub const DW_AT_call_parameter: DwAt = _;
```

#### Constant `DW_AT_call_pc`

```rust
pub const DW_AT_call_pc: DwAt = _;
```

#### Constant `DW_AT_call_tail_call`

```rust
pub const DW_AT_call_tail_call: DwAt = _;
```

#### Constant `DW_AT_call_target`

```rust
pub const DW_AT_call_target: DwAt = _;
```

#### Constant `DW_AT_call_target_clobbered`

```rust
pub const DW_AT_call_target_clobbered: DwAt = _;
```

#### Constant `DW_AT_call_data_location`

```rust
pub const DW_AT_call_data_location: DwAt = _;
```

#### Constant `DW_AT_call_data_value`

```rust
pub const DW_AT_call_data_value: DwAt = _;
```

#### Constant `DW_AT_noreturn`

```rust
pub const DW_AT_noreturn: DwAt = _;
```

#### Constant `DW_AT_alignment`

```rust
pub const DW_AT_alignment: DwAt = _;
```

#### Constant `DW_AT_export_symbols`

```rust
pub const DW_AT_export_symbols: DwAt = _;
```

#### Constant `DW_AT_deleted`

```rust
pub const DW_AT_deleted: DwAt = _;
```

#### Constant `DW_AT_defaulted`

```rust
pub const DW_AT_defaulted: DwAt = _;
```

#### Constant `DW_AT_loclists_base`

```rust
pub const DW_AT_loclists_base: DwAt = _;
```

#### Constant `DW_AT_lo_user`

```rust
pub const DW_AT_lo_user: DwAt = _;
```

#### Constant `DW_AT_hi_user`

```rust
pub const DW_AT_hi_user: DwAt = _;
```

#### Constant `DW_AT_MIPS_fde`

```rust
pub const DW_AT_MIPS_fde: DwAt = _;
```

#### Constant `DW_AT_MIPS_loop_begin`

```rust
pub const DW_AT_MIPS_loop_begin: DwAt = _;
```

#### Constant `DW_AT_MIPS_tail_loop_begin`

```rust
pub const DW_AT_MIPS_tail_loop_begin: DwAt = _;
```

#### Constant `DW_AT_MIPS_epilog_begin`

```rust
pub const DW_AT_MIPS_epilog_begin: DwAt = _;
```

#### Constant `DW_AT_MIPS_loop_unroll_factor`

```rust
pub const DW_AT_MIPS_loop_unroll_factor: DwAt = _;
```

#### Constant `DW_AT_MIPS_software_pipeline_depth`

```rust
pub const DW_AT_MIPS_software_pipeline_depth: DwAt = _;
```

#### Constant `DW_AT_MIPS_linkage_name`

```rust
pub const DW_AT_MIPS_linkage_name: DwAt = _;
```

#### Constant `DW_AT_MIPS_stride`

```rust
pub const DW_AT_MIPS_stride: DwAt = _;
```

#### Constant `DW_AT_MIPS_abstract_name`

```rust
pub const DW_AT_MIPS_abstract_name: DwAt = _;
```

#### Constant `DW_AT_MIPS_clone_origin`

```rust
pub const DW_AT_MIPS_clone_origin: DwAt = _;
```

#### Constant `DW_AT_MIPS_has_inlines`

```rust
pub const DW_AT_MIPS_has_inlines: DwAt = _;
```

#### Constant `DW_AT_MIPS_stride_byte`

```rust
pub const DW_AT_MIPS_stride_byte: DwAt = _;
```

#### Constant `DW_AT_MIPS_stride_elem`

```rust
pub const DW_AT_MIPS_stride_elem: DwAt = _;
```

#### Constant `DW_AT_MIPS_ptr_dopetype`

```rust
pub const DW_AT_MIPS_ptr_dopetype: DwAt = _;
```

#### Constant `DW_AT_MIPS_allocatable_dopetype`

```rust
pub const DW_AT_MIPS_allocatable_dopetype: DwAt = _;
```

#### Constant `DW_AT_MIPS_assumed_shape_dopetype`

```rust
pub const DW_AT_MIPS_assumed_shape_dopetype: DwAt = _;
```

#### Constant `DW_AT_MIPS_assumed_size`

```rust
pub const DW_AT_MIPS_assumed_size: DwAt = _;
```

#### Constant `DW_AT_INTEL_other_endian`

```rust
pub const DW_AT_INTEL_other_endian: DwAt = _;
```

#### Constant `DW_AT_sf_names`

```rust
pub const DW_AT_sf_names: DwAt = _;
```

#### Constant `DW_AT_src_info`

```rust
pub const DW_AT_src_info: DwAt = _;
```

#### Constant `DW_AT_mac_info`

```rust
pub const DW_AT_mac_info: DwAt = _;
```

#### Constant `DW_AT_src_coords`

```rust
pub const DW_AT_src_coords: DwAt = _;
```

#### Constant `DW_AT_body_begin`

```rust
pub const DW_AT_body_begin: DwAt = _;
```

#### Constant `DW_AT_body_end`

```rust
pub const DW_AT_body_end: DwAt = _;
```

#### Constant `DW_AT_GNU_vector`

```rust
pub const DW_AT_GNU_vector: DwAt = _;
```

#### Constant `DW_AT_GNU_guarded_by`

```rust
pub const DW_AT_GNU_guarded_by: DwAt = _;
```

#### Constant `DW_AT_GNU_pt_guarded_by`

```rust
pub const DW_AT_GNU_pt_guarded_by: DwAt = _;
```

#### Constant `DW_AT_GNU_guarded`

```rust
pub const DW_AT_GNU_guarded: DwAt = _;
```

#### Constant `DW_AT_GNU_pt_guarded`

```rust
pub const DW_AT_GNU_pt_guarded: DwAt = _;
```

#### Constant `DW_AT_GNU_locks_excluded`

```rust
pub const DW_AT_GNU_locks_excluded: DwAt = _;
```

#### Constant `DW_AT_GNU_exclusive_locks_required`

```rust
pub const DW_AT_GNU_exclusive_locks_required: DwAt = _;
```

#### Constant `DW_AT_GNU_shared_locks_required`

```rust
pub const DW_AT_GNU_shared_locks_required: DwAt = _;
```

#### Constant `DW_AT_GNU_odr_signature`

```rust
pub const DW_AT_GNU_odr_signature: DwAt = _;
```

#### Constant `DW_AT_GNU_template_name`

```rust
pub const DW_AT_GNU_template_name: DwAt = _;
```

#### Constant `DW_AT_GNU_call_site_value`

```rust
pub const DW_AT_GNU_call_site_value: DwAt = _;
```

#### Constant `DW_AT_GNU_call_site_data_value`

```rust
pub const DW_AT_GNU_call_site_data_value: DwAt = _;
```

#### Constant `DW_AT_GNU_call_site_target`

```rust
pub const DW_AT_GNU_call_site_target: DwAt = _;
```

#### Constant `DW_AT_GNU_call_site_target_clobbered`

```rust
pub const DW_AT_GNU_call_site_target_clobbered: DwAt = _;
```

#### Constant `DW_AT_GNU_tail_call`

```rust
pub const DW_AT_GNU_tail_call: DwAt = _;
```

#### Constant `DW_AT_GNU_all_tail_call_sites`

```rust
pub const DW_AT_GNU_all_tail_call_sites: DwAt = _;
```

#### Constant `DW_AT_GNU_all_call_sites`

```rust
pub const DW_AT_GNU_all_call_sites: DwAt = _;
```

#### Constant `DW_AT_GNU_all_source_call_sites`

```rust
pub const DW_AT_GNU_all_source_call_sites: DwAt = _;
```

#### Constant `DW_AT_GNU_macros`

```rust
pub const DW_AT_GNU_macros: DwAt = _;
```

#### Constant `DW_AT_GNU_deleted`

```rust
pub const DW_AT_GNU_deleted: DwAt = _;
```

#### Constant `DW_AT_GNU_dwo_name`

```rust
pub const DW_AT_GNU_dwo_name: DwAt = _;
```

#### Constant `DW_AT_GNU_dwo_id`

```rust
pub const DW_AT_GNU_dwo_id: DwAt = _;
```

#### Constant `DW_AT_GNU_ranges_base`

```rust
pub const DW_AT_GNU_ranges_base: DwAt = _;
```

#### Constant `DW_AT_GNU_addr_base`

```rust
pub const DW_AT_GNU_addr_base: DwAt = _;
```

#### Constant `DW_AT_GNU_pubnames`

```rust
pub const DW_AT_GNU_pubnames: DwAt = _;
```

#### Constant `DW_AT_GNU_pubtypes`

```rust
pub const DW_AT_GNU_pubtypes: DwAt = _;
```

#### Constant `DW_AT_GNU_discriminator`

```rust
pub const DW_AT_GNU_discriminator: DwAt = _;
```

#### Constant `DW_AT_GNU_locviews`

```rust
pub const DW_AT_GNU_locviews: DwAt = _;
```

#### Constant `DW_AT_GNU_entry_view`

```rust
pub const DW_AT_GNU_entry_view: DwAt = _;
```

#### Constant `DW_AT_SUN_template`

```rust
pub const DW_AT_SUN_template: DwAt = _;
```

#### Constant `DW_AT_SUN_alignment`

```rust
pub const DW_AT_SUN_alignment: DwAt = _;
```

#### Constant `DW_AT_SUN_vtable`

```rust
pub const DW_AT_SUN_vtable: DwAt = _;
```

#### Constant `DW_AT_SUN_count_guarantee`

```rust
pub const DW_AT_SUN_count_guarantee: DwAt = _;
```

#### Constant `DW_AT_SUN_command_line`

```rust
pub const DW_AT_SUN_command_line: DwAt = _;
```

#### Constant `DW_AT_SUN_vbase`

```rust
pub const DW_AT_SUN_vbase: DwAt = _;
```

#### Constant `DW_AT_SUN_compile_options`

```rust
pub const DW_AT_SUN_compile_options: DwAt = _;
```

#### Constant `DW_AT_SUN_language`

```rust
pub const DW_AT_SUN_language: DwAt = _;
```

#### Constant `DW_AT_SUN_browser_file`

```rust
pub const DW_AT_SUN_browser_file: DwAt = _;
```

#### Constant `DW_AT_SUN_vtable_abi`

```rust
pub const DW_AT_SUN_vtable_abi: DwAt = _;
```

#### Constant `DW_AT_SUN_func_offsets`

```rust
pub const DW_AT_SUN_func_offsets: DwAt = _;
```

#### Constant `DW_AT_SUN_cf_kind`

```rust
pub const DW_AT_SUN_cf_kind: DwAt = _;
```

#### Constant `DW_AT_SUN_vtable_index`

```rust
pub const DW_AT_SUN_vtable_index: DwAt = _;
```

#### Constant `DW_AT_SUN_omp_tpriv_addr`

```rust
pub const DW_AT_SUN_omp_tpriv_addr: DwAt = _;
```

#### Constant `DW_AT_SUN_omp_child_func`

```rust
pub const DW_AT_SUN_omp_child_func: DwAt = _;
```

#### Constant `DW_AT_SUN_func_offset`

```rust
pub const DW_AT_SUN_func_offset: DwAt = _;
```

#### Constant `DW_AT_SUN_memop_type_ref`

```rust
pub const DW_AT_SUN_memop_type_ref: DwAt = _;
```

#### Constant `DW_AT_SUN_profile_id`

```rust
pub const DW_AT_SUN_profile_id: DwAt = _;
```

#### Constant `DW_AT_SUN_memop_signature`

```rust
pub const DW_AT_SUN_memop_signature: DwAt = _;
```

#### Constant `DW_AT_SUN_obj_dir`

```rust
pub const DW_AT_SUN_obj_dir: DwAt = _;
```

#### Constant `DW_AT_SUN_obj_file`

```rust
pub const DW_AT_SUN_obj_file: DwAt = _;
```

#### Constant `DW_AT_SUN_original_name`

```rust
pub const DW_AT_SUN_original_name: DwAt = _;
```

#### Constant `DW_AT_SUN_hwcprof_signature`

```rust
pub const DW_AT_SUN_hwcprof_signature: DwAt = _;
```

#### Constant `DW_AT_SUN_amd64_parmdump`

```rust
pub const DW_AT_SUN_amd64_parmdump: DwAt = _;
```

#### Constant `DW_AT_SUN_part_link_name`

```rust
pub const DW_AT_SUN_part_link_name: DwAt = _;
```

#### Constant `DW_AT_SUN_link_name`

```rust
pub const DW_AT_SUN_link_name: DwAt = _;
```

#### Constant `DW_AT_SUN_pass_with_const`

```rust
pub const DW_AT_SUN_pass_with_const: DwAt = _;
```

#### Constant `DW_AT_SUN_return_with_const`

```rust
pub const DW_AT_SUN_return_with_const: DwAt = _;
```

#### Constant `DW_AT_SUN_import_by_name`

```rust
pub const DW_AT_SUN_import_by_name: DwAt = _;
```

#### Constant `DW_AT_SUN_f90_pointer`

```rust
pub const DW_AT_SUN_f90_pointer: DwAt = _;
```

#### Constant `DW_AT_SUN_pass_by_ref`

```rust
pub const DW_AT_SUN_pass_by_ref: DwAt = _;
```

#### Constant `DW_AT_SUN_f90_allocatable`

```rust
pub const DW_AT_SUN_f90_allocatable: DwAt = _;
```

#### Constant `DW_AT_SUN_f90_assumed_shape_array`

```rust
pub const DW_AT_SUN_f90_assumed_shape_array: DwAt = _;
```

#### Constant `DW_AT_SUN_c_vla`

```rust
pub const DW_AT_SUN_c_vla: DwAt = _;
```

#### Constant `DW_AT_SUN_return_value_ptr`

```rust
pub const DW_AT_SUN_return_value_ptr: DwAt = _;
```

#### Constant `DW_AT_SUN_dtor_start`

```rust
pub const DW_AT_SUN_dtor_start: DwAt = _;
```

#### Constant `DW_AT_SUN_dtor_length`

```rust
pub const DW_AT_SUN_dtor_length: DwAt = _;
```

#### Constant `DW_AT_SUN_dtor_state_initial`

```rust
pub const DW_AT_SUN_dtor_state_initial: DwAt = _;
```

#### Constant `DW_AT_SUN_dtor_state_final`

```rust
pub const DW_AT_SUN_dtor_state_final: DwAt = _;
```

#### Constant `DW_AT_SUN_dtor_state_deltas`

```rust
pub const DW_AT_SUN_dtor_state_deltas: DwAt = _;
```

#### Constant `DW_AT_SUN_import_by_lname`

```rust
pub const DW_AT_SUN_import_by_lname: DwAt = _;
```

#### Constant `DW_AT_SUN_f90_use_only`

```rust
pub const DW_AT_SUN_f90_use_only: DwAt = _;
```

#### Constant `DW_AT_SUN_namelist_spec`

```rust
pub const DW_AT_SUN_namelist_spec: DwAt = _;
```

#### Constant `DW_AT_SUN_is_omp_child_func`

```rust
pub const DW_AT_SUN_is_omp_child_func: DwAt = _;
```

#### Constant `DW_AT_SUN_fortran_main_alias`

```rust
pub const DW_AT_SUN_fortran_main_alias: DwAt = _;
```

#### Constant `DW_AT_SUN_fortran_based`

```rust
pub const DW_AT_SUN_fortran_based: DwAt = _;
```

#### Constant `DW_AT_ALTIUM_loclist`

```rust
pub const DW_AT_ALTIUM_loclist: DwAt = _;
```

#### Constant `DW_AT_use_GNAT_descriptive_type`

```rust
pub const DW_AT_use_GNAT_descriptive_type: DwAt = _;
```

#### Constant `DW_AT_GNAT_descriptive_type`

```rust
pub const DW_AT_GNAT_descriptive_type: DwAt = _;
```

#### Constant `DW_AT_GNU_numerator`

```rust
pub const DW_AT_GNU_numerator: DwAt = _;
```

#### Constant `DW_AT_GNU_denominator`

```rust
pub const DW_AT_GNU_denominator: DwAt = _;
```

#### Constant `DW_AT_GNU_bias`

```rust
pub const DW_AT_GNU_bias: DwAt = _;
```

#### Constant `DW_AT_upc_threads_scaled`

```rust
pub const DW_AT_upc_threads_scaled: DwAt = _;
```

#### Constant `DW_AT_PGI_lbase`

```rust
pub const DW_AT_PGI_lbase: DwAt = _;
```

#### Constant `DW_AT_PGI_soffset`

```rust
pub const DW_AT_PGI_soffset: DwAt = _;
```

#### Constant `DW_AT_PGI_lstride`

```rust
pub const DW_AT_PGI_lstride: DwAt = _;
```

#### Constant `DW_AT_BORLAND_property_read`

```rust
pub const DW_AT_BORLAND_property_read: DwAt = _;
```

#### Constant `DW_AT_BORLAND_property_write`

```rust
pub const DW_AT_BORLAND_property_write: DwAt = _;
```

#### Constant `DW_AT_BORLAND_property_implements`

```rust
pub const DW_AT_BORLAND_property_implements: DwAt = _;
```

#### Constant `DW_AT_BORLAND_property_index`

```rust
pub const DW_AT_BORLAND_property_index: DwAt = _;
```

#### Constant `DW_AT_BORLAND_property_default`

```rust
pub const DW_AT_BORLAND_property_default: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_unit`

```rust
pub const DW_AT_BORLAND_Delphi_unit: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_class`

```rust
pub const DW_AT_BORLAND_Delphi_class: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_record`

```rust
pub const DW_AT_BORLAND_Delphi_record: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_metaclass`

```rust
pub const DW_AT_BORLAND_Delphi_metaclass: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_constructor`

```rust
pub const DW_AT_BORLAND_Delphi_constructor: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_destructor`

```rust
pub const DW_AT_BORLAND_Delphi_destructor: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_anonymous_method`

```rust
pub const DW_AT_BORLAND_Delphi_anonymous_method: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_interface`

```rust
pub const DW_AT_BORLAND_Delphi_interface: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_ABI`

```rust
pub const DW_AT_BORLAND_Delphi_ABI: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_return`

```rust
pub const DW_AT_BORLAND_Delphi_return: DwAt = _;
```

#### Constant `DW_AT_BORLAND_Delphi_frameptr`

```rust
pub const DW_AT_BORLAND_Delphi_frameptr: DwAt = _;
```

#### Constant `DW_AT_BORLAND_closure`

```rust
pub const DW_AT_BORLAND_closure: DwAt = _;
```

#### Constant `DW_AT_LLVM_include_path`

```rust
pub const DW_AT_LLVM_include_path: DwAt = _;
```

#### Constant `DW_AT_LLVM_config_macros`

```rust
pub const DW_AT_LLVM_config_macros: DwAt = _;
```

#### Constant `DW_AT_LLVM_isysroot`

```rust
pub const DW_AT_LLVM_isysroot: DwAt = _;
```

#### Constant `DW_AT_APPLE_optimized`

```rust
pub const DW_AT_APPLE_optimized: DwAt = _;
```

#### Constant `DW_AT_APPLE_flags`

```rust
pub const DW_AT_APPLE_flags: DwAt = _;
```

#### Constant `DW_AT_APPLE_isa`

```rust
pub const DW_AT_APPLE_isa: DwAt = _;
```

#### Constant `DW_AT_APPLE_block`

```rust
pub const DW_AT_APPLE_block: DwAt = _;
```

#### Constant `DW_AT_APPLE_major_runtime_vers`

```rust
pub const DW_AT_APPLE_major_runtime_vers: DwAt = _;
```

#### Constant `DW_AT_APPLE_runtime_class`

```rust
pub const DW_AT_APPLE_runtime_class: DwAt = _;
```

#### Constant `DW_AT_APPLE_omit_frame_ptr`

```rust
pub const DW_AT_APPLE_omit_frame_ptr: DwAt = _;
```

#### Constant `DW_AT_APPLE_property_name`

```rust
pub const DW_AT_APPLE_property_name: DwAt = _;
```

#### Constant `DW_AT_APPLE_property_getter`

```rust
pub const DW_AT_APPLE_property_getter: DwAt = _;
```

#### Constant `DW_AT_APPLE_property_setter`

```rust
pub const DW_AT_APPLE_property_setter: DwAt = _;
```

#### Constant `DW_AT_APPLE_property_attribute`

```rust
pub const DW_AT_APPLE_property_attribute: DwAt = _;
```

#### Constant `DW_AT_APPLE_objc_complete_type`

```rust
pub const DW_AT_APPLE_objc_complete_type: DwAt = _;
```

#### Constant `DW_AT_APPLE_property`

```rust
pub const DW_AT_APPLE_property: DwAt = _;
```

#### Constant `DW_FORM_null`

```rust
pub const DW_FORM_null: DwForm = _;
```

#### Constant `DW_FORM_addr`

```rust
pub const DW_FORM_addr: DwForm = _;
```

#### Constant `DW_FORM_block2`

```rust
pub const DW_FORM_block2: DwForm = _;
```

#### Constant `DW_FORM_block4`

```rust
pub const DW_FORM_block4: DwForm = _;
```

#### Constant `DW_FORM_data2`

```rust
pub const DW_FORM_data2: DwForm = _;
```

#### Constant `DW_FORM_data4`

```rust
pub const DW_FORM_data4: DwForm = _;
```

#### Constant `DW_FORM_data8`

```rust
pub const DW_FORM_data8: DwForm = _;
```

#### Constant `DW_FORM_string`

```rust
pub const DW_FORM_string: DwForm = _;
```

#### Constant `DW_FORM_block`

```rust
pub const DW_FORM_block: DwForm = _;
```

#### Constant `DW_FORM_block1`

```rust
pub const DW_FORM_block1: DwForm = _;
```

#### Constant `DW_FORM_data1`

```rust
pub const DW_FORM_data1: DwForm = _;
```

#### Constant `DW_FORM_flag`

```rust
pub const DW_FORM_flag: DwForm = _;
```

#### Constant `DW_FORM_sdata`

```rust
pub const DW_FORM_sdata: DwForm = _;
```

#### Constant `DW_FORM_strp`

```rust
pub const DW_FORM_strp: DwForm = _;
```

#### Constant `DW_FORM_udata`

```rust
pub const DW_FORM_udata: DwForm = _;
```

#### Constant `DW_FORM_ref_addr`

```rust
pub const DW_FORM_ref_addr: DwForm = _;
```

#### Constant `DW_FORM_ref1`

```rust
pub const DW_FORM_ref1: DwForm = _;
```

#### Constant `DW_FORM_ref2`

```rust
pub const DW_FORM_ref2: DwForm = _;
```

#### Constant `DW_FORM_ref4`

```rust
pub const DW_FORM_ref4: DwForm = _;
```

#### Constant `DW_FORM_ref8`

```rust
pub const DW_FORM_ref8: DwForm = _;
```

#### Constant `DW_FORM_ref_udata`

```rust
pub const DW_FORM_ref_udata: DwForm = _;
```

#### Constant `DW_FORM_indirect`

```rust
pub const DW_FORM_indirect: DwForm = _;
```

#### Constant `DW_FORM_sec_offset`

```rust
pub const DW_FORM_sec_offset: DwForm = _;
```

#### Constant `DW_FORM_exprloc`

```rust
pub const DW_FORM_exprloc: DwForm = _;
```

#### Constant `DW_FORM_flag_present`

```rust
pub const DW_FORM_flag_present: DwForm = _;
```

#### Constant `DW_FORM_ref_sig8`

```rust
pub const DW_FORM_ref_sig8: DwForm = _;
```

#### Constant `DW_FORM_strx`

```rust
pub const DW_FORM_strx: DwForm = _;
```

#### Constant `DW_FORM_addrx`

```rust
pub const DW_FORM_addrx: DwForm = _;
```

#### Constant `DW_FORM_ref_sup4`

```rust
pub const DW_FORM_ref_sup4: DwForm = _;
```

#### Constant `DW_FORM_strp_sup`

```rust
pub const DW_FORM_strp_sup: DwForm = _;
```

#### Constant `DW_FORM_data16`

```rust
pub const DW_FORM_data16: DwForm = _;
```

#### Constant `DW_FORM_line_strp`

```rust
pub const DW_FORM_line_strp: DwForm = _;
```

#### Constant `DW_FORM_implicit_const`

```rust
pub const DW_FORM_implicit_const: DwForm = _;
```

#### Constant `DW_FORM_loclistx`

```rust
pub const DW_FORM_loclistx: DwForm = _;
```

#### Constant `DW_FORM_rnglistx`

```rust
pub const DW_FORM_rnglistx: DwForm = _;
```

#### Constant `DW_FORM_ref_sup8`

```rust
pub const DW_FORM_ref_sup8: DwForm = _;
```

#### Constant `DW_FORM_strx1`

```rust
pub const DW_FORM_strx1: DwForm = _;
```

#### Constant `DW_FORM_strx2`

```rust
pub const DW_FORM_strx2: DwForm = _;
```

#### Constant `DW_FORM_strx3`

```rust
pub const DW_FORM_strx3: DwForm = _;
```

#### Constant `DW_FORM_strx4`

```rust
pub const DW_FORM_strx4: DwForm = _;
```

#### Constant `DW_FORM_addrx1`

```rust
pub const DW_FORM_addrx1: DwForm = _;
```

#### Constant `DW_FORM_addrx2`

```rust
pub const DW_FORM_addrx2: DwForm = _;
```

#### Constant `DW_FORM_addrx3`

```rust
pub const DW_FORM_addrx3: DwForm = _;
```

#### Constant `DW_FORM_addrx4`

```rust
pub const DW_FORM_addrx4: DwForm = _;
```

#### Constant `DW_FORM_GNU_addr_index`

```rust
pub const DW_FORM_GNU_addr_index: DwForm = _;
```

#### Constant `DW_FORM_GNU_str_index`

```rust
pub const DW_FORM_GNU_str_index: DwForm = _;
```

#### Constant `DW_FORM_GNU_ref_alt`

```rust
pub const DW_FORM_GNU_ref_alt: DwForm = _;
```

#### Constant `DW_FORM_GNU_strp_alt`

```rust
pub const DW_FORM_GNU_strp_alt: DwForm = _;
```

#### Constant `DW_ATE_address`

```rust
pub const DW_ATE_address: DwAte = _;
```

#### Constant `DW_ATE_boolean`

```rust
pub const DW_ATE_boolean: DwAte = _;
```

#### Constant `DW_ATE_complex_float`

```rust
pub const DW_ATE_complex_float: DwAte = _;
```

#### Constant `DW_ATE_float`

```rust
pub const DW_ATE_float: DwAte = _;
```

#### Constant `DW_ATE_signed`

```rust
pub const DW_ATE_signed: DwAte = _;
```

#### Constant `DW_ATE_signed_char`

```rust
pub const DW_ATE_signed_char: DwAte = _;
```

#### Constant `DW_ATE_unsigned`

```rust
pub const DW_ATE_unsigned: DwAte = _;
```

#### Constant `DW_ATE_unsigned_char`

```rust
pub const DW_ATE_unsigned_char: DwAte = _;
```

#### Constant `DW_ATE_imaginary_float`

```rust
pub const DW_ATE_imaginary_float: DwAte = _;
```

#### Constant `DW_ATE_packed_decimal`

```rust
pub const DW_ATE_packed_decimal: DwAte = _;
```

#### Constant `DW_ATE_numeric_string`

```rust
pub const DW_ATE_numeric_string: DwAte = _;
```

#### Constant `DW_ATE_edited`

```rust
pub const DW_ATE_edited: DwAte = _;
```

#### Constant `DW_ATE_signed_fixed`

```rust
pub const DW_ATE_signed_fixed: DwAte = _;
```

#### Constant `DW_ATE_unsigned_fixed`

```rust
pub const DW_ATE_unsigned_fixed: DwAte = _;
```

#### Constant `DW_ATE_decimal_float`

```rust
pub const DW_ATE_decimal_float: DwAte = _;
```

#### Constant `DW_ATE_UTF`

```rust
pub const DW_ATE_UTF: DwAte = _;
```

#### Constant `DW_ATE_UCS`

```rust
pub const DW_ATE_UCS: DwAte = _;
```

#### Constant `DW_ATE_ASCII`

```rust
pub const DW_ATE_ASCII: DwAte = _;
```

#### Constant `DW_ATE_lo_user`

```rust
pub const DW_ATE_lo_user: DwAte = _;
```

#### Constant `DW_ATE_hi_user`

```rust
pub const DW_ATE_hi_user: DwAte = _;
```

#### Constant `DW_LLE_end_of_list`

```rust
pub const DW_LLE_end_of_list: DwLle = _;
```

#### Constant `DW_LLE_base_addressx`

```rust
pub const DW_LLE_base_addressx: DwLle = _;
```

#### Constant `DW_LLE_startx_endx`

```rust
pub const DW_LLE_startx_endx: DwLle = _;
```

#### Constant `DW_LLE_startx_length`

```rust
pub const DW_LLE_startx_length: DwLle = _;
```

#### Constant `DW_LLE_offset_pair`

```rust
pub const DW_LLE_offset_pair: DwLle = _;
```

#### Constant `DW_LLE_default_location`

```rust
pub const DW_LLE_default_location: DwLle = _;
```

#### Constant `DW_LLE_base_address`

```rust
pub const DW_LLE_base_address: DwLle = _;
```

#### Constant `DW_LLE_start_end`

```rust
pub const DW_LLE_start_end: DwLle = _;
```

#### Constant `DW_LLE_start_length`

```rust
pub const DW_LLE_start_length: DwLle = _;
```

#### Constant `DW_LLE_GNU_view_pair`

```rust
pub const DW_LLE_GNU_view_pair: DwLle = _;
```

#### Constant `DW_DS_unsigned`

```rust
pub const DW_DS_unsigned: DwDs = _;
```

#### Constant `DW_DS_leading_overpunch`

```rust
pub const DW_DS_leading_overpunch: DwDs = _;
```

#### Constant `DW_DS_trailing_overpunch`

```rust
pub const DW_DS_trailing_overpunch: DwDs = _;
```

#### Constant `DW_DS_leading_separate`

```rust
pub const DW_DS_leading_separate: DwDs = _;
```

#### Constant `DW_DS_trailing_separate`

```rust
pub const DW_DS_trailing_separate: DwDs = _;
```

#### Constant `DW_END_default`

```rust
pub const DW_END_default: DwEnd = _;
```

#### Constant `DW_END_big`

```rust
pub const DW_END_big: DwEnd = _;
```

#### Constant `DW_END_little`

```rust
pub const DW_END_little: DwEnd = _;
```

#### Constant `DW_END_lo_user`

```rust
pub const DW_END_lo_user: DwEnd = _;
```

#### Constant `DW_END_hi_user`

```rust
pub const DW_END_hi_user: DwEnd = _;
```

#### Constant `DW_ACCESS_public`

```rust
pub const DW_ACCESS_public: DwAccess = _;
```

#### Constant `DW_ACCESS_protected`

```rust
pub const DW_ACCESS_protected: DwAccess = _;
```

#### Constant `DW_ACCESS_private`

```rust
pub const DW_ACCESS_private: DwAccess = _;
```

#### Constant `DW_VIS_local`

```rust
pub const DW_VIS_local: DwVis = _;
```

#### Constant `DW_VIS_exported`

```rust
pub const DW_VIS_exported: DwVis = _;
```

#### Constant `DW_VIS_qualified`

```rust
pub const DW_VIS_qualified: DwVis = _;
```

#### Constant `DW_VIRTUALITY_none`

```rust
pub const DW_VIRTUALITY_none: DwVirtuality = _;
```

#### Constant `DW_VIRTUALITY_virtual`

```rust
pub const DW_VIRTUALITY_virtual: DwVirtuality = _;
```

#### Constant `DW_VIRTUALITY_pure_virtual`

```rust
pub const DW_VIRTUALITY_pure_virtual: DwVirtuality = _;
```

#### Constant `DW_LANG_C89`

```rust
pub const DW_LANG_C89: DwLang = _;
```

#### Constant `DW_LANG_C`

```rust
pub const DW_LANG_C: DwLang = _;
```

#### Constant `DW_LANG_Ada83`

```rust
pub const DW_LANG_Ada83: DwLang = _;
```

#### Constant `DW_LANG_C_plus_plus`

```rust
pub const DW_LANG_C_plus_plus: DwLang = _;
```

#### Constant `DW_LANG_Cobol74`

```rust
pub const DW_LANG_Cobol74: DwLang = _;
```

#### Constant `DW_LANG_Cobol85`

```rust
pub const DW_LANG_Cobol85: DwLang = _;
```

#### Constant `DW_LANG_Fortran77`

```rust
pub const DW_LANG_Fortran77: DwLang = _;
```

#### Constant `DW_LANG_Fortran90`

```rust
pub const DW_LANG_Fortran90: DwLang = _;
```

#### Constant `DW_LANG_Pascal83`

```rust
pub const DW_LANG_Pascal83: DwLang = _;
```

#### Constant `DW_LANG_Modula2`

```rust
pub const DW_LANG_Modula2: DwLang = _;
```

#### Constant `DW_LANG_Java`

```rust
pub const DW_LANG_Java: DwLang = _;
```

#### Constant `DW_LANG_C99`

```rust
pub const DW_LANG_C99: DwLang = _;
```

#### Constant `DW_LANG_Ada95`

```rust
pub const DW_LANG_Ada95: DwLang = _;
```

#### Constant `DW_LANG_Fortran95`

```rust
pub const DW_LANG_Fortran95: DwLang = _;
```

#### Constant `DW_LANG_PLI`

```rust
pub const DW_LANG_PLI: DwLang = _;
```

#### Constant `DW_LANG_ObjC`

```rust
pub const DW_LANG_ObjC: DwLang = _;
```

#### Constant `DW_LANG_ObjC_plus_plus`

```rust
pub const DW_LANG_ObjC_plus_plus: DwLang = _;
```

#### Constant `DW_LANG_UPC`

```rust
pub const DW_LANG_UPC: DwLang = _;
```

#### Constant `DW_LANG_D`

```rust
pub const DW_LANG_D: DwLang = _;
```

#### Constant `DW_LANG_Python`

```rust
pub const DW_LANG_Python: DwLang = _;
```

#### Constant `DW_LANG_OpenCL`

```rust
pub const DW_LANG_OpenCL: DwLang = _;
```

#### Constant `DW_LANG_Go`

```rust
pub const DW_LANG_Go: DwLang = _;
```

#### Constant `DW_LANG_Modula3`

```rust
pub const DW_LANG_Modula3: DwLang = _;
```

#### Constant `DW_LANG_Haskell`

```rust
pub const DW_LANG_Haskell: DwLang = _;
```

#### Constant `DW_LANG_C_plus_plus_03`

```rust
pub const DW_LANG_C_plus_plus_03: DwLang = _;
```

#### Constant `DW_LANG_C_plus_plus_11`

```rust
pub const DW_LANG_C_plus_plus_11: DwLang = _;
```

#### Constant `DW_LANG_OCaml`

```rust
pub const DW_LANG_OCaml: DwLang = _;
```

#### Constant `DW_LANG_Rust`

```rust
pub const DW_LANG_Rust: DwLang = _;
```

#### Constant `DW_LANG_C11`

```rust
pub const DW_LANG_C11: DwLang = _;
```

#### Constant `DW_LANG_Swift`

```rust
pub const DW_LANG_Swift: DwLang = _;
```

#### Constant `DW_LANG_Julia`

```rust
pub const DW_LANG_Julia: DwLang = _;
```

#### Constant `DW_LANG_Dylan`

```rust
pub const DW_LANG_Dylan: DwLang = _;
```

#### Constant `DW_LANG_C_plus_plus_14`

```rust
pub const DW_LANG_C_plus_plus_14: DwLang = _;
```

#### Constant `DW_LANG_Fortran03`

```rust
pub const DW_LANG_Fortran03: DwLang = _;
```

#### Constant `DW_LANG_Fortran08`

```rust
pub const DW_LANG_Fortran08: DwLang = _;
```

#### Constant `DW_LANG_RenderScript`

```rust
pub const DW_LANG_RenderScript: DwLang = _;
```

#### Constant `DW_LANG_BLISS`

```rust
pub const DW_LANG_BLISS: DwLang = _;
```

#### Constant `DW_LANG_Kotlin`

```rust
pub const DW_LANG_Kotlin: DwLang = _;
```

#### Constant `DW_LANG_Zig`

```rust
pub const DW_LANG_Zig: DwLang = _;
```

#### Constant `DW_LANG_Crystal`

```rust
pub const DW_LANG_Crystal: DwLang = _;
```

#### Constant `DW_LANG_C_plus_plus_17`

```rust
pub const DW_LANG_C_plus_plus_17: DwLang = _;
```

#### Constant `DW_LANG_C_plus_plus_20`

```rust
pub const DW_LANG_C_plus_plus_20: DwLang = _;
```

#### Constant `DW_LANG_C17`

```rust
pub const DW_LANG_C17: DwLang = _;
```

#### Constant `DW_LANG_Fortran18`

```rust
pub const DW_LANG_Fortran18: DwLang = _;
```

#### Constant `DW_LANG_Ada2005`

```rust
pub const DW_LANG_Ada2005: DwLang = _;
```

#### Constant `DW_LANG_Ada2012`

```rust
pub const DW_LANG_Ada2012: DwLang = _;
```

#### Constant `DW_LANG_lo_user`

```rust
pub const DW_LANG_lo_user: DwLang = _;
```

#### Constant `DW_LANG_hi_user`

```rust
pub const DW_LANG_hi_user: DwLang = _;
```

#### Constant `DW_LANG_Mips_Assembler`

```rust
pub const DW_LANG_Mips_Assembler: DwLang = _;
```

#### Constant `DW_LANG_GOOGLE_RenderScript`

```rust
pub const DW_LANG_GOOGLE_RenderScript: DwLang = _;
```

#### Constant `DW_LANG_SUN_Assembler`

```rust
pub const DW_LANG_SUN_Assembler: DwLang = _;
```

#### Constant `DW_LANG_ALTIUM_Assembler`

```rust
pub const DW_LANG_ALTIUM_Assembler: DwLang = _;
```

#### Constant `DW_LANG_BORLAND_Delphi`

```rust
pub const DW_LANG_BORLAND_Delphi: DwLang = _;
```

#### Constant `DW_ADDR_none`

```rust
pub const DW_ADDR_none: DwAddr = _;
```

#### Constant `DW_ID_case_sensitive`

```rust
pub const DW_ID_case_sensitive: DwId = _;
```

#### Constant `DW_ID_up_case`

```rust
pub const DW_ID_up_case: DwId = _;
```

#### Constant `DW_ID_down_case`

```rust
pub const DW_ID_down_case: DwId = _;
```

#### Constant `DW_ID_case_insensitive`

```rust
pub const DW_ID_case_insensitive: DwId = _;
```

#### Constant `DW_CC_normal`

```rust
pub const DW_CC_normal: DwCc = _;
```

#### Constant `DW_CC_program`

```rust
pub const DW_CC_program: DwCc = _;
```

#### Constant `DW_CC_nocall`

```rust
pub const DW_CC_nocall: DwCc = _;
```

#### Constant `DW_CC_pass_by_reference`

```rust
pub const DW_CC_pass_by_reference: DwCc = _;
```

#### Constant `DW_CC_pass_by_value`

```rust
pub const DW_CC_pass_by_value: DwCc = _;
```

#### Constant `DW_CC_lo_user`

```rust
pub const DW_CC_lo_user: DwCc = _;
```

#### Constant `DW_CC_hi_user`

```rust
pub const DW_CC_hi_user: DwCc = _;
```

#### Constant `DW_INL_not_inlined`

```rust
pub const DW_INL_not_inlined: DwInl = _;
```

#### Constant `DW_INL_inlined`

```rust
pub const DW_INL_inlined: DwInl = _;
```

#### Constant `DW_INL_declared_not_inlined`

```rust
pub const DW_INL_declared_not_inlined: DwInl = _;
```

#### Constant `DW_INL_declared_inlined`

```rust
pub const DW_INL_declared_inlined: DwInl = _;
```

#### Constant `DW_ORD_row_major`

```rust
pub const DW_ORD_row_major: DwOrd = _;
```

#### Constant `DW_ORD_col_major`

```rust
pub const DW_ORD_col_major: DwOrd = _;
```

#### Constant `DW_DSC_label`

```rust
pub const DW_DSC_label: DwDsc = _;
```

#### Constant `DW_DSC_range`

```rust
pub const DW_DSC_range: DwDsc = _;
```

#### Constant `DW_IDX_compile_unit`

```rust
pub const DW_IDX_compile_unit: DwIdx = _;
```

#### Constant `DW_IDX_type_unit`

```rust
pub const DW_IDX_type_unit: DwIdx = _;
```

#### Constant `DW_IDX_die_offset`

```rust
pub const DW_IDX_die_offset: DwIdx = _;
```

#### Constant `DW_IDX_parent`

```rust
pub const DW_IDX_parent: DwIdx = _;
```

#### Constant `DW_IDX_type_hash`

```rust
pub const DW_IDX_type_hash: DwIdx = _;
```

#### Constant `DW_IDX_lo_user`

```rust
pub const DW_IDX_lo_user: DwIdx = _;
```

#### Constant `DW_IDX_hi_user`

```rust
pub const DW_IDX_hi_user: DwIdx = _;
```

#### Constant `DW_DEFAULTED_no`

```rust
pub const DW_DEFAULTED_no: DwDefaulted = _;
```

#### Constant `DW_DEFAULTED_in_class`

```rust
pub const DW_DEFAULTED_in_class: DwDefaulted = _;
```

#### Constant `DW_DEFAULTED_out_of_class`

```rust
pub const DW_DEFAULTED_out_of_class: DwDefaulted = _;
```

#### Constant `DW_LNS_copy`

```rust
pub const DW_LNS_copy: DwLns = _;
```

#### Constant `DW_LNS_advance_pc`

```rust
pub const DW_LNS_advance_pc: DwLns = _;
```

#### Constant `DW_LNS_advance_line`

```rust
pub const DW_LNS_advance_line: DwLns = _;
```

#### Constant `DW_LNS_set_file`

```rust
pub const DW_LNS_set_file: DwLns = _;
```

#### Constant `DW_LNS_set_column`

```rust
pub const DW_LNS_set_column: DwLns = _;
```

#### Constant `DW_LNS_negate_stmt`

```rust
pub const DW_LNS_negate_stmt: DwLns = _;
```

#### Constant `DW_LNS_set_basic_block`

```rust
pub const DW_LNS_set_basic_block: DwLns = _;
```

#### Constant `DW_LNS_const_add_pc`

```rust
pub const DW_LNS_const_add_pc: DwLns = _;
```

#### Constant `DW_LNS_fixed_advance_pc`

```rust
pub const DW_LNS_fixed_advance_pc: DwLns = _;
```

#### Constant `DW_LNS_set_prologue_end`

```rust
pub const DW_LNS_set_prologue_end: DwLns = _;
```

#### Constant `DW_LNS_set_epilogue_begin`

```rust
pub const DW_LNS_set_epilogue_begin: DwLns = _;
```

#### Constant `DW_LNS_set_isa`

```rust
pub const DW_LNS_set_isa: DwLns = _;
```

#### Constant `DW_LNE_end_sequence`

```rust
pub const DW_LNE_end_sequence: DwLne = _;
```

#### Constant `DW_LNE_set_address`

```rust
pub const DW_LNE_set_address: DwLne = _;
```

#### Constant `DW_LNE_define_file`

```rust
pub const DW_LNE_define_file: DwLne = _;
```

#### Constant `DW_LNE_set_discriminator`

```rust
pub const DW_LNE_set_discriminator: DwLne = _;
```

#### Constant `DW_LNE_lo_user`

```rust
pub const DW_LNE_lo_user: DwLne = _;
```

#### Constant `DW_LNE_hi_user`

```rust
pub const DW_LNE_hi_user: DwLne = _;
```

#### Constant `DW_LNCT_path`

```rust
pub const DW_LNCT_path: DwLnct = _;
```

#### Constant `DW_LNCT_directory_index`

```rust
pub const DW_LNCT_directory_index: DwLnct = _;
```

#### Constant `DW_LNCT_timestamp`

```rust
pub const DW_LNCT_timestamp: DwLnct = _;
```

#### Constant `DW_LNCT_size`

```rust
pub const DW_LNCT_size: DwLnct = _;
```

#### Constant `DW_LNCT_MD5`

```rust
pub const DW_LNCT_MD5: DwLnct = _;
```

#### Constant `DW_LNCT_lo_user`

```rust
pub const DW_LNCT_lo_user: DwLnct = _;
```

#### Constant `DW_LNCT_LLVM_source`

```rust
pub const DW_LNCT_LLVM_source: DwLnct = _;
```

#### Constant `DW_LNCT_hi_user`

```rust
pub const DW_LNCT_hi_user: DwLnct = _;
```

#### Constant `DW_MACRO_define`

```rust
pub const DW_MACRO_define: DwMacro = _;
```

#### Constant `DW_MACRO_undef`

```rust
pub const DW_MACRO_undef: DwMacro = _;
```

#### Constant `DW_MACRO_start_file`

```rust
pub const DW_MACRO_start_file: DwMacro = _;
```

#### Constant `DW_MACRO_end_file`

```rust
pub const DW_MACRO_end_file: DwMacro = _;
```

#### Constant `DW_MACRO_define_strp`

```rust
pub const DW_MACRO_define_strp: DwMacro = _;
```

#### Constant `DW_MACRO_undef_strp`

```rust
pub const DW_MACRO_undef_strp: DwMacro = _;
```

#### Constant `DW_MACRO_import`

```rust
pub const DW_MACRO_import: DwMacro = _;
```

#### Constant `DW_MACRO_define_sup`

```rust
pub const DW_MACRO_define_sup: DwMacro = _;
```

#### Constant `DW_MACRO_undef_sup`

```rust
pub const DW_MACRO_undef_sup: DwMacro = _;
```

#### Constant `DW_MACRO_import_sup`

```rust
pub const DW_MACRO_import_sup: DwMacro = _;
```

#### Constant `DW_MACRO_define_strx`

```rust
pub const DW_MACRO_define_strx: DwMacro = _;
```

#### Constant `DW_MACRO_undef_strx`

```rust
pub const DW_MACRO_undef_strx: DwMacro = _;
```

#### Constant `DW_MACRO_lo_user`

```rust
pub const DW_MACRO_lo_user: DwMacro = _;
```

#### Constant `DW_MACRO_hi_user`

```rust
pub const DW_MACRO_hi_user: DwMacro = _;
```

#### Constant `DW_RLE_end_of_list`

```rust
pub const DW_RLE_end_of_list: DwRle = _;
```

#### Constant `DW_RLE_base_addressx`

```rust
pub const DW_RLE_base_addressx: DwRle = _;
```

#### Constant `DW_RLE_startx_endx`

```rust
pub const DW_RLE_startx_endx: DwRle = _;
```

#### Constant `DW_RLE_startx_length`

```rust
pub const DW_RLE_startx_length: DwRle = _;
```

#### Constant `DW_RLE_offset_pair`

```rust
pub const DW_RLE_offset_pair: DwRle = _;
```

#### Constant `DW_RLE_base_address`

```rust
pub const DW_RLE_base_address: DwRle = _;
```

#### Constant `DW_RLE_start_end`

```rust
pub const DW_RLE_start_end: DwRle = _;
```

#### Constant `DW_RLE_start_length`

```rust
pub const DW_RLE_start_length: DwRle = _;
```

#### Constant `DW_OP_addr`

```rust
pub const DW_OP_addr: DwOp = _;
```

#### Constant `DW_OP_deref`

```rust
pub const DW_OP_deref: DwOp = _;
```

#### Constant `DW_OP_const1u`

```rust
pub const DW_OP_const1u: DwOp = _;
```

#### Constant `DW_OP_const1s`

```rust
pub const DW_OP_const1s: DwOp = _;
```

#### Constant `DW_OP_const2u`

```rust
pub const DW_OP_const2u: DwOp = _;
```

#### Constant `DW_OP_const2s`

```rust
pub const DW_OP_const2s: DwOp = _;
```

#### Constant `DW_OP_const4u`

```rust
pub const DW_OP_const4u: DwOp = _;
```

#### Constant `DW_OP_const4s`

```rust
pub const DW_OP_const4s: DwOp = _;
```

#### Constant `DW_OP_const8u`

```rust
pub const DW_OP_const8u: DwOp = _;
```

#### Constant `DW_OP_const8s`

```rust
pub const DW_OP_const8s: DwOp = _;
```

#### Constant `DW_OP_constu`

```rust
pub const DW_OP_constu: DwOp = _;
```

#### Constant `DW_OP_consts`

```rust
pub const DW_OP_consts: DwOp = _;
```

#### Constant `DW_OP_dup`

```rust
pub const DW_OP_dup: DwOp = _;
```

#### Constant `DW_OP_drop`

```rust
pub const DW_OP_drop: DwOp = _;
```

#### Constant `DW_OP_over`

```rust
pub const DW_OP_over: DwOp = _;
```

#### Constant `DW_OP_pick`

```rust
pub const DW_OP_pick: DwOp = _;
```

#### Constant `DW_OP_swap`

```rust
pub const DW_OP_swap: DwOp = _;
```

#### Constant `DW_OP_rot`

```rust
pub const DW_OP_rot: DwOp = _;
```

#### Constant `DW_OP_xderef`

```rust
pub const DW_OP_xderef: DwOp = _;
```

#### Constant `DW_OP_abs`

```rust
pub const DW_OP_abs: DwOp = _;
```

#### Constant `DW_OP_and`

```rust
pub const DW_OP_and: DwOp = _;
```

#### Constant `DW_OP_div`

```rust
pub const DW_OP_div: DwOp = _;
```

#### Constant `DW_OP_minus`

```rust
pub const DW_OP_minus: DwOp = _;
```

#### Constant `DW_OP_mod`

```rust
pub const DW_OP_mod: DwOp = _;
```

#### Constant `DW_OP_mul`

```rust
pub const DW_OP_mul: DwOp = _;
```

#### Constant `DW_OP_neg`

```rust
pub const DW_OP_neg: DwOp = _;
```

#### Constant `DW_OP_not`

```rust
pub const DW_OP_not: DwOp = _;
```

#### Constant `DW_OP_or`

```rust
pub const DW_OP_or: DwOp = _;
```

#### Constant `DW_OP_plus`

```rust
pub const DW_OP_plus: DwOp = _;
```

#### Constant `DW_OP_plus_uconst`

```rust
pub const DW_OP_plus_uconst: DwOp = _;
```

#### Constant `DW_OP_shl`

```rust
pub const DW_OP_shl: DwOp = _;
```

#### Constant `DW_OP_shr`

```rust
pub const DW_OP_shr: DwOp = _;
```

#### Constant `DW_OP_shra`

```rust
pub const DW_OP_shra: DwOp = _;
```

#### Constant `DW_OP_xor`

```rust
pub const DW_OP_xor: DwOp = _;
```

#### Constant `DW_OP_bra`

```rust
pub const DW_OP_bra: DwOp = _;
```

#### Constant `DW_OP_eq`

```rust
pub const DW_OP_eq: DwOp = _;
```

#### Constant `DW_OP_ge`

```rust
pub const DW_OP_ge: DwOp = _;
```

#### Constant `DW_OP_gt`

```rust
pub const DW_OP_gt: DwOp = _;
```

#### Constant `DW_OP_le`

```rust
pub const DW_OP_le: DwOp = _;
```

#### Constant `DW_OP_lt`

```rust
pub const DW_OP_lt: DwOp = _;
```

#### Constant `DW_OP_ne`

```rust
pub const DW_OP_ne: DwOp = _;
```

#### Constant `DW_OP_skip`

```rust
pub const DW_OP_skip: DwOp = _;
```

#### Constant `DW_OP_lit0`

```rust
pub const DW_OP_lit0: DwOp = _;
```

#### Constant `DW_OP_lit1`

```rust
pub const DW_OP_lit1: DwOp = _;
```

#### Constant `DW_OP_lit2`

```rust
pub const DW_OP_lit2: DwOp = _;
```

#### Constant `DW_OP_lit3`

```rust
pub const DW_OP_lit3: DwOp = _;
```

#### Constant `DW_OP_lit4`

```rust
pub const DW_OP_lit4: DwOp = _;
```

#### Constant `DW_OP_lit5`

```rust
pub const DW_OP_lit5: DwOp = _;
```

#### Constant `DW_OP_lit6`

```rust
pub const DW_OP_lit6: DwOp = _;
```

#### Constant `DW_OP_lit7`

```rust
pub const DW_OP_lit7: DwOp = _;
```

#### Constant `DW_OP_lit8`

```rust
pub const DW_OP_lit8: DwOp = _;
```

#### Constant `DW_OP_lit9`

```rust
pub const DW_OP_lit9: DwOp = _;
```

#### Constant `DW_OP_lit10`

```rust
pub const DW_OP_lit10: DwOp = _;
```

#### Constant `DW_OP_lit11`

```rust
pub const DW_OP_lit11: DwOp = _;
```

#### Constant `DW_OP_lit12`

```rust
pub const DW_OP_lit12: DwOp = _;
```

#### Constant `DW_OP_lit13`

```rust
pub const DW_OP_lit13: DwOp = _;
```

#### Constant `DW_OP_lit14`

```rust
pub const DW_OP_lit14: DwOp = _;
```

#### Constant `DW_OP_lit15`

```rust
pub const DW_OP_lit15: DwOp = _;
```

#### Constant `DW_OP_lit16`

```rust
pub const DW_OP_lit16: DwOp = _;
```

#### Constant `DW_OP_lit17`

```rust
pub const DW_OP_lit17: DwOp = _;
```

#### Constant `DW_OP_lit18`

```rust
pub const DW_OP_lit18: DwOp = _;
```

#### Constant `DW_OP_lit19`

```rust
pub const DW_OP_lit19: DwOp = _;
```

#### Constant `DW_OP_lit20`

```rust
pub const DW_OP_lit20: DwOp = _;
```

#### Constant `DW_OP_lit21`

```rust
pub const DW_OP_lit21: DwOp = _;
```

#### Constant `DW_OP_lit22`

```rust
pub const DW_OP_lit22: DwOp = _;
```

#### Constant `DW_OP_lit23`

```rust
pub const DW_OP_lit23: DwOp = _;
```

#### Constant `DW_OP_lit24`

```rust
pub const DW_OP_lit24: DwOp = _;
```

#### Constant `DW_OP_lit25`

```rust
pub const DW_OP_lit25: DwOp = _;
```

#### Constant `DW_OP_lit26`

```rust
pub const DW_OP_lit26: DwOp = _;
```

#### Constant `DW_OP_lit27`

```rust
pub const DW_OP_lit27: DwOp = _;
```

#### Constant `DW_OP_lit28`

```rust
pub const DW_OP_lit28: DwOp = _;
```

#### Constant `DW_OP_lit29`

```rust
pub const DW_OP_lit29: DwOp = _;
```

#### Constant `DW_OP_lit30`

```rust
pub const DW_OP_lit30: DwOp = _;
```

#### Constant `DW_OP_lit31`

```rust
pub const DW_OP_lit31: DwOp = _;
```

#### Constant `DW_OP_reg0`

```rust
pub const DW_OP_reg0: DwOp = _;
```

#### Constant `DW_OP_reg1`

```rust
pub const DW_OP_reg1: DwOp = _;
```

#### Constant `DW_OP_reg2`

```rust
pub const DW_OP_reg2: DwOp = _;
```

#### Constant `DW_OP_reg3`

```rust
pub const DW_OP_reg3: DwOp = _;
```

#### Constant `DW_OP_reg4`

```rust
pub const DW_OP_reg4: DwOp = _;
```

#### Constant `DW_OP_reg5`

```rust
pub const DW_OP_reg5: DwOp = _;
```

#### Constant `DW_OP_reg6`

```rust
pub const DW_OP_reg6: DwOp = _;
```

#### Constant `DW_OP_reg7`

```rust
pub const DW_OP_reg7: DwOp = _;
```

#### Constant `DW_OP_reg8`

```rust
pub const DW_OP_reg8: DwOp = _;
```

#### Constant `DW_OP_reg9`

```rust
pub const DW_OP_reg9: DwOp = _;
```

#### Constant `DW_OP_reg10`

```rust
pub const DW_OP_reg10: DwOp = _;
```

#### Constant `DW_OP_reg11`

```rust
pub const DW_OP_reg11: DwOp = _;
```

#### Constant `DW_OP_reg12`

```rust
pub const DW_OP_reg12: DwOp = _;
```

#### Constant `DW_OP_reg13`

```rust
pub const DW_OP_reg13: DwOp = _;
```

#### Constant `DW_OP_reg14`

```rust
pub const DW_OP_reg14: DwOp = _;
```

#### Constant `DW_OP_reg15`

```rust
pub const DW_OP_reg15: DwOp = _;
```

#### Constant `DW_OP_reg16`

```rust
pub const DW_OP_reg16: DwOp = _;
```

#### Constant `DW_OP_reg17`

```rust
pub const DW_OP_reg17: DwOp = _;
```

#### Constant `DW_OP_reg18`

```rust
pub const DW_OP_reg18: DwOp = _;
```

#### Constant `DW_OP_reg19`

```rust
pub const DW_OP_reg19: DwOp = _;
```

#### Constant `DW_OP_reg20`

```rust
pub const DW_OP_reg20: DwOp = _;
```

#### Constant `DW_OP_reg21`

```rust
pub const DW_OP_reg21: DwOp = _;
```

#### Constant `DW_OP_reg22`

```rust
pub const DW_OP_reg22: DwOp = _;
```

#### Constant `DW_OP_reg23`

```rust
pub const DW_OP_reg23: DwOp = _;
```

#### Constant `DW_OP_reg24`

```rust
pub const DW_OP_reg24: DwOp = _;
```

#### Constant `DW_OP_reg25`

```rust
pub const DW_OP_reg25: DwOp = _;
```

#### Constant `DW_OP_reg26`

```rust
pub const DW_OP_reg26: DwOp = _;
```

#### Constant `DW_OP_reg27`

```rust
pub const DW_OP_reg27: DwOp = _;
```

#### Constant `DW_OP_reg28`

```rust
pub const DW_OP_reg28: DwOp = _;
```

#### Constant `DW_OP_reg29`

```rust
pub const DW_OP_reg29: DwOp = _;
```

#### Constant `DW_OP_reg30`

```rust
pub const DW_OP_reg30: DwOp = _;
```

#### Constant `DW_OP_reg31`

```rust
pub const DW_OP_reg31: DwOp = _;
```

#### Constant `DW_OP_breg0`

```rust
pub const DW_OP_breg0: DwOp = _;
```

#### Constant `DW_OP_breg1`

```rust
pub const DW_OP_breg1: DwOp = _;
```

#### Constant `DW_OP_breg2`

```rust
pub const DW_OP_breg2: DwOp = _;
```

#### Constant `DW_OP_breg3`

```rust
pub const DW_OP_breg3: DwOp = _;
```

#### Constant `DW_OP_breg4`

```rust
pub const DW_OP_breg4: DwOp = _;
```

#### Constant `DW_OP_breg5`

```rust
pub const DW_OP_breg5: DwOp = _;
```

#### Constant `DW_OP_breg6`

```rust
pub const DW_OP_breg6: DwOp = _;
```

#### Constant `DW_OP_breg7`

```rust
pub const DW_OP_breg7: DwOp = _;
```

#### Constant `DW_OP_breg8`

```rust
pub const DW_OP_breg8: DwOp = _;
```

#### Constant `DW_OP_breg9`

```rust
pub const DW_OP_breg9: DwOp = _;
```

#### Constant `DW_OP_breg10`

```rust
pub const DW_OP_breg10: DwOp = _;
```

#### Constant `DW_OP_breg11`

```rust
pub const DW_OP_breg11: DwOp = _;
```

#### Constant `DW_OP_breg12`

```rust
pub const DW_OP_breg12: DwOp = _;
```

#### Constant `DW_OP_breg13`

```rust
pub const DW_OP_breg13: DwOp = _;
```

#### Constant `DW_OP_breg14`

```rust
pub const DW_OP_breg14: DwOp = _;
```

#### Constant `DW_OP_breg15`

```rust
pub const DW_OP_breg15: DwOp = _;
```

#### Constant `DW_OP_breg16`

```rust
pub const DW_OP_breg16: DwOp = _;
```

#### Constant `DW_OP_breg17`

```rust
pub const DW_OP_breg17: DwOp = _;
```

#### Constant `DW_OP_breg18`

```rust
pub const DW_OP_breg18: DwOp = _;
```

#### Constant `DW_OP_breg19`

```rust
pub const DW_OP_breg19: DwOp = _;
```

#### Constant `DW_OP_breg20`

```rust
pub const DW_OP_breg20: DwOp = _;
```

#### Constant `DW_OP_breg21`

```rust
pub const DW_OP_breg21: DwOp = _;
```

#### Constant `DW_OP_breg22`

```rust
pub const DW_OP_breg22: DwOp = _;
```

#### Constant `DW_OP_breg23`

```rust
pub const DW_OP_breg23: DwOp = _;
```

#### Constant `DW_OP_breg24`

```rust
pub const DW_OP_breg24: DwOp = _;
```

#### Constant `DW_OP_breg25`

```rust
pub const DW_OP_breg25: DwOp = _;
```

#### Constant `DW_OP_breg26`

```rust
pub const DW_OP_breg26: DwOp = _;
```

#### Constant `DW_OP_breg27`

```rust
pub const DW_OP_breg27: DwOp = _;
```

#### Constant `DW_OP_breg28`

```rust
pub const DW_OP_breg28: DwOp = _;
```

#### Constant `DW_OP_breg29`

```rust
pub const DW_OP_breg29: DwOp = _;
```

#### Constant `DW_OP_breg30`

```rust
pub const DW_OP_breg30: DwOp = _;
```

#### Constant `DW_OP_breg31`

```rust
pub const DW_OP_breg31: DwOp = _;
```

#### Constant `DW_OP_regx`

```rust
pub const DW_OP_regx: DwOp = _;
```

#### Constant `DW_OP_fbreg`

```rust
pub const DW_OP_fbreg: DwOp = _;
```

#### Constant `DW_OP_bregx`

```rust
pub const DW_OP_bregx: DwOp = _;
```

#### Constant `DW_OP_piece`

```rust
pub const DW_OP_piece: DwOp = _;
```

#### Constant `DW_OP_deref_size`

```rust
pub const DW_OP_deref_size: DwOp = _;
```

#### Constant `DW_OP_xderef_size`

```rust
pub const DW_OP_xderef_size: DwOp = _;
```

#### Constant `DW_OP_nop`

```rust
pub const DW_OP_nop: DwOp = _;
```

#### Constant `DW_OP_push_object_address`

```rust
pub const DW_OP_push_object_address: DwOp = _;
```

#### Constant `DW_OP_call2`

```rust
pub const DW_OP_call2: DwOp = _;
```

#### Constant `DW_OP_call4`

```rust
pub const DW_OP_call4: DwOp = _;
```

#### Constant `DW_OP_call_ref`

```rust
pub const DW_OP_call_ref: DwOp = _;
```

#### Constant `DW_OP_form_tls_address`

```rust
pub const DW_OP_form_tls_address: DwOp = _;
```

#### Constant `DW_OP_call_frame_cfa`

```rust
pub const DW_OP_call_frame_cfa: DwOp = _;
```

#### Constant `DW_OP_bit_piece`

```rust
pub const DW_OP_bit_piece: DwOp = _;
```

#### Constant `DW_OP_implicit_value`

```rust
pub const DW_OP_implicit_value: DwOp = _;
```

#### Constant `DW_OP_stack_value`

```rust
pub const DW_OP_stack_value: DwOp = _;
```

#### Constant `DW_OP_implicit_pointer`

```rust
pub const DW_OP_implicit_pointer: DwOp = _;
```

#### Constant `DW_OP_addrx`

```rust
pub const DW_OP_addrx: DwOp = _;
```

#### Constant `DW_OP_constx`

```rust
pub const DW_OP_constx: DwOp = _;
```

#### Constant `DW_OP_entry_value`

```rust
pub const DW_OP_entry_value: DwOp = _;
```

#### Constant `DW_OP_const_type`

```rust
pub const DW_OP_const_type: DwOp = _;
```

#### Constant `DW_OP_regval_type`

```rust
pub const DW_OP_regval_type: DwOp = _;
```

#### Constant `DW_OP_deref_type`

```rust
pub const DW_OP_deref_type: DwOp = _;
```

#### Constant `DW_OP_xderef_type`

```rust
pub const DW_OP_xderef_type: DwOp = _;
```

#### Constant `DW_OP_convert`

```rust
pub const DW_OP_convert: DwOp = _;
```

#### Constant `DW_OP_reinterpret`

```rust
pub const DW_OP_reinterpret: DwOp = _;
```

#### Constant `DW_OP_GNU_push_tls_address`

```rust
pub const DW_OP_GNU_push_tls_address: DwOp = _;
```

#### Constant `DW_OP_GNU_implicit_pointer`

```rust
pub const DW_OP_GNU_implicit_pointer: DwOp = _;
```

#### Constant `DW_OP_GNU_entry_value`

```rust
pub const DW_OP_GNU_entry_value: DwOp = _;
```

#### Constant `DW_OP_GNU_const_type`

```rust
pub const DW_OP_GNU_const_type: DwOp = _;
```

#### Constant `DW_OP_GNU_regval_type`

```rust
pub const DW_OP_GNU_regval_type: DwOp = _;
```

#### Constant `DW_OP_GNU_deref_type`

```rust
pub const DW_OP_GNU_deref_type: DwOp = _;
```

#### Constant `DW_OP_GNU_convert`

```rust
pub const DW_OP_GNU_convert: DwOp = _;
```

#### Constant `DW_OP_GNU_reinterpret`

```rust
pub const DW_OP_GNU_reinterpret: DwOp = _;
```

#### Constant `DW_OP_GNU_parameter_ref`

```rust
pub const DW_OP_GNU_parameter_ref: DwOp = _;
```

#### Constant `DW_OP_GNU_addr_index`

```rust
pub const DW_OP_GNU_addr_index: DwOp = _;
```

#### Constant `DW_OP_GNU_const_index`

```rust
pub const DW_OP_GNU_const_index: DwOp = _;
```

#### Constant `DW_OP_WASM_location`

```rust
pub const DW_OP_WASM_location: DwOp = _;
```

#### Constant `DW_EH_PE_uleb128`

```rust
pub const DW_EH_PE_uleb128: DwEhPe = _;
```

#### Constant `DW_EH_PE_udata2`

```rust
pub const DW_EH_PE_udata2: DwEhPe = _;
```

#### Constant `DW_EH_PE_udata4`

```rust
pub const DW_EH_PE_udata4: DwEhPe = _;
```

#### Constant `DW_EH_PE_udata8`

```rust
pub const DW_EH_PE_udata8: DwEhPe = _;
```

#### Constant `DW_EH_PE_sleb128`

```rust
pub const DW_EH_PE_sleb128: DwEhPe = _;
```

#### Constant `DW_EH_PE_sdata2`

```rust
pub const DW_EH_PE_sdata2: DwEhPe = _;
```

#### Constant `DW_EH_PE_sdata4`

```rust
pub const DW_EH_PE_sdata4: DwEhPe = _;
```

#### Constant `DW_EH_PE_sdata8`

```rust
pub const DW_EH_PE_sdata8: DwEhPe = _;
```

#### Constant `DW_EH_PE_pcrel`

```rust
pub const DW_EH_PE_pcrel: DwEhPe = _;
```

#### Constant `DW_EH_PE_textrel`

```rust
pub const DW_EH_PE_textrel: DwEhPe = _;
```

#### Constant `DW_EH_PE_datarel`

```rust
pub const DW_EH_PE_datarel: DwEhPe = _;
```

#### Constant `DW_EH_PE_funcrel`

```rust
pub const DW_EH_PE_funcrel: DwEhPe = _;
```

#### Constant `DW_EH_PE_aligned`

```rust
pub const DW_EH_PE_aligned: DwEhPe = _;
```

#### Constant `DW_EH_PE_indirect`

```rust
pub const DW_EH_PE_indirect: DwEhPe = _;
```

#### Constant `DW_EH_PE_absptr`

```rust
pub const DW_EH_PE_absptr: DwEhPe = _;
```

#### Constant `DW_EH_PE_omit`

```rust
pub const DW_EH_PE_omit: DwEhPe = _;
```

## Module `leb128`

Read and write DWARF's "Little Endian Base 128" (LEB128) variable length
integer encoding.

The implementation is a direct translation of the psuedocode in the DWARF 4
standard's appendix C.

Read and write signed integers:

```
# #[cfg(all(feature = "read", feature = "write"))] {
use gimli::{EndianSlice, NativeEndian, leb128};

let mut buf = [0; 1024];

// Write to anything that implements `std::io::Write`.
{
    let mut writable = &mut buf[..];
    leb128::write::signed(&mut writable, -12345).expect("Should write number");
}

// Read from anything that implements `gimli::Reader`.
let mut readable = EndianSlice::new(&buf[..], NativeEndian);
let val = leb128::read::signed(&mut readable).expect("Should read number");
assert_eq!(val, -12345);
# }
```

Or read and write unsigned integers:

```
# #[cfg(all(feature = "read", feature = "write"))] {
use gimli::{EndianSlice, NativeEndian, leb128};

let mut buf = [0; 1024];

{
    let mut writable = &mut buf[..];
    leb128::write::unsigned(&mut writable, 98765).expect("Should write number");
}

let mut readable = EndianSlice::new(&buf[..], NativeEndian);
let val = leb128::read::unsigned(&mut readable).expect("Should read number");
assert_eq!(val, 98765);
# }
```

```rust
pub mod leb128 { /* ... */ }
```

### Modules

## Module `read`

**Attributes:**

- `#[<cfg>(feature = "read-core")]`

A module for reading signed and unsigned integers that have been LEB128
encoded.

```rust
pub mod read { /* ... */ }
```

### Functions

#### Function `skip`

Read bytes until the LEB128 continuation bit is not set.

```rust
pub fn skip<R: Reader>(r: &mut R) -> crate::read::Result<()> { /* ... */ }
```

#### Function `unsigned`

Read an unsigned LEB128 number from the given `Reader` and
return it or an error if reading failed.

```rust
pub fn unsigned<R: Reader>(r: &mut R) -> crate::read::Result<u64> { /* ... */ }
```

#### Function `u16`

Read an LEB128 u16 from the given `Reader` and
return it or an error if reading failed.

```rust
pub fn u16<R: Reader>(r: &mut R) -> crate::read::Result<u16> { /* ... */ }
```

#### Function `signed`

Read a signed LEB128 number from the given `Reader` and
return it or an error if reading failed.

```rust
pub fn signed<R: Reader>(r: &mut R) -> crate::read::Result<i64> { /* ... */ }
```

## Module `read`

**Attributes:**

- `#[<cfg>(feature = "read-core")]`

Read DWARF debugging information.

* [Example Usage](#example-usage)
* [API Structure](#api-structure)
* [Using with `FallibleIterator`](#using-with-fallibleiterator)

## Example Usage

Print out all of the functions in the debuggee program:

```rust,no_run
# fn example() -> Result<(), gimli::Error> {
# type R = gimli::EndianSlice<'static, gimli::LittleEndian>;
# let get_file_section_reader = |name| -> Result<R, gimli::Error> { unimplemented!() };
# let get_sup_file_section_reader = |name| -> Result<R, gimli::Error> { unimplemented!() };
// Read the DWARF sections with whatever object loader you're using.
// These closures should return a `Reader` instance (e.g. `EndianSlice`).
let loader = |section: gimli::SectionId| { get_file_section_reader(section.name()) };
let sup_loader = |section: gimli::SectionId| { get_sup_file_section_reader(section.name()) };
let mut dwarf = gimli::Dwarf::load(loader)?;
dwarf.load_sup(sup_loader)?;

// Iterate over all compilation units.
let mut iter = dwarf.units();
while let Some(header) = iter.next()? {
    // Parse the abbreviations and other information for this compilation unit.
    let unit = dwarf.unit(header)?;

    // Iterate over all of this compilation unit's entries.
    let mut entries = unit.entries();
    while let Some((_, entry)) = entries.next_dfs()? {
        // If we find an entry for a function, print it.
        if entry.tag() == gimli::DW_TAG_subprogram {
            println!("Found a function: {:?}", entry);
        }
    }
}
# unreachable!()
# }
```

Full example programs:

  * [A simple parser](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/simple.rs)

  * [A `dwarfdump`
    clone](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/dwarfdump.rs)

  * [An `addr2line` clone](https://github.com/gimli-rs/addr2line)

  * [`ddbug`](https://github.com/gimli-rs/ddbug), a utility giving insight into
    code generation by making debugging information readable

  * [`dwprod`](https://github.com/fitzgen/dwprod), a tiny utility to list the
    compilers used to create each compilation unit within a shared library or
    executable (via `DW_AT_producer`)

  * [`dwarf-validate`](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/dwarf-validate.rs),
    a program to validate the integrity of some DWARF and its references
    between sections and compilation units.

## API Structure

* Basic familiarity with DWARF is assumed.

* The [`Dwarf`](./struct.Dwarf.html) type contains the commonly used DWARF
  sections. It has methods that simplify access to debugging data that spans
  multiple sections. Use of this type is optional, but recommended.

* The [`DwarfPackage`](./struct.Dwarf.html) type contains the DWARF
  package (DWP) sections. It has methods to find a DWARF object (DWO)
  within the package.

* Each section gets its own type. Consider these types the entry points to
  the library:

  * [`DebugAbbrev`](./struct.DebugAbbrev.html): The `.debug_abbrev` section.

  * [`DebugAddr`](./struct.DebugAddr.html): The `.debug_addr` section.

  * [`DebugAranges`](./struct.DebugAranges.html): The `.debug_aranges`
    section.

  * [`DebugFrame`](./struct.DebugFrame.html): The `.debug_frame` section.

  * [`DebugInfo`](./struct.DebugInfo.html): The `.debug_info` section.

  * [`DebugLine`](./struct.DebugLine.html): The `.debug_line` section.

  * [`DebugLineStr`](./struct.DebugLineStr.html): The `.debug_line_str` section.

  * [`DebugLoc`](./struct.DebugLoc.html): The `.debug_loc` section.

  * [`DebugLocLists`](./struct.DebugLocLists.html): The `.debug_loclists` section.

  * [`DebugPubNames`](./struct.DebugPubNames.html): The `.debug_pubnames`
    section.

  * [`DebugPubTypes`](./struct.DebugPubTypes.html): The `.debug_pubtypes`
    section.

  * [`DebugRanges`](./struct.DebugRanges.html): The `.debug_ranges` section.

  * [`DebugRngLists`](./struct.DebugRngLists.html): The `.debug_rnglists` section.

  * [`DebugStr`](./struct.DebugStr.html): The `.debug_str` section.

  * [`DebugStrOffsets`](./struct.DebugStrOffsets.html): The `.debug_str_offsets` section.

  * [`DebugTypes`](./struct.DebugTypes.html): The `.debug_types` section.

  * [`DebugCuIndex`](./struct.DebugCuIndex.html): The `.debug_cu_index` section.

  * [`DebugTuIndex`](./struct.DebugTuIndex.html): The `.debug_tu_index` section.

  * [`EhFrame`](./struct.EhFrame.html): The `.eh_frame` section.

  * [`EhFrameHdr`](./struct.EhFrameHdr.html): The `.eh_frame_hdr` section.

* Each section type exposes methods for accessing the debugging data encoded
  in that section. For example, the [`DebugInfo`](./struct.DebugInfo.html)
  struct has the [`units`](./struct.DebugInfo.html#method.units) method for
  iterating over the compilation units defined within it.

* Offsets into a section are strongly typed: an offset into `.debug_info` is
  the [`DebugInfoOffset`](./struct.DebugInfoOffset.html) type. It cannot be
  used to index into the [`DebugLine`](./struct.DebugLine.html) type because
  `DebugLine` represents the `.debug_line` section. There are similar types
  for offsets relative to a compilation unit rather than a section.

## Using with `FallibleIterator`

The standard library's `Iterator` trait and related APIs do not play well
with iterators where the `next` operation is fallible. One can make the
`Iterator`'s associated `Item` type be a `Result<T, E>`, however the
provided methods cannot gracefully handle the case when an `Err` is
returned.

This situation led to the
[`fallible-iterator`](https://crates.io/crates/fallible-iterator) crate's
existence. You can read more of the rationale for its existence in its
docs. The crate provides the helpers you have come to expect (eg `map`,
`filter`, etc) for iterators that can fail.

`gimli`'s many lazy parsing iterators are a perfect match for the
`fallible-iterator` crate's `FallibleIterator` trait because parsing is not
done eagerly. Parse errors later in the input might only be discovered after
having iterated through many items.

To use `gimli` iterators with `FallibleIterator`, import the crate and trait
into your code:

```
# #[cfg(feature = "fallible-iterator")]
# fn foo() {
// Use the `FallibleIterator` trait so its methods are in scope!
use fallible_iterator::FallibleIterator;
use gimli::{DebugAranges, EndianSlice, LittleEndian};

fn find_sum_of_address_range_lengths(aranges: DebugAranges<EndianSlice<LittleEndian>>)
    -> gimli::Result<u64>
{
    // `DebugAranges::headers` returns a `FallibleIterator`!
    aranges.headers()
        // `flat_map` is provided by `FallibleIterator`!
        .flat_map(|header| Ok(header.entries()))
        // `map` is provided by `FallibleIterator`!
        .map(|arange| Ok(arange.length()))
        // `fold` is provided by `FallibleIterator`!
        .fold(0, |sum, len| Ok(sum + len))
}
# }
# fn main() {}
```

```rust
pub mod read { /* ... */ }
```

### Types

#### Struct `UnitOffset`

An offset into the current compilation or type unit.

```rust
pub struct UnitOffset<T = usize>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn to_unit_section_offset<R>(self: &Self, unit: &Unit<R>) -> UnitSectionOffset<T>
where
    R: Reader<Offset = T> { /* ... */ }
  ```
  Convert an offset to be relative to the start of the .debug_info section,

- ```rust
  pub fn to_debug_info_offset<R>(self: &Self, unit: &UnitHeader<R>) -> Option<DebugInfoOffset<T>>
where
    R: Reader<Offset = T> { /* ... */ }
  ```
  Convert an offset to be relative to the start of the .debug_info section,

- ```rust
  pub fn to_debug_types_offset<R>(self: &Self, unit: &UnitHeader<R>) -> Option<DebugTypesOffset<T>>
where
    R: Reader<Offset = T> { /* ... */ }
  ```
  Convert an offset to be relative to the start of the .debug_types section,

###### Trait Implementations

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UnitOffset<T>) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &UnitOffset<T>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &UnitOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnitOffset<T> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `StoreOnHeap`

Indicates that storage should be allocated on heap.

```rust
pub struct StoreOnHeap;
```

##### Implementations

###### Trait Implementations

- **EvaluationStorage**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindContextStorage**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StoreOnHeap) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StoreOnHeap { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Type Alias `EndianBuf`

**⚠️ Deprecated**: EndianBuf has been renamed to EndianSlice, use that instead.

`EndianBuf` has been renamed to `EndianSlice`. For ease of upgrading across
`gimli` versions, we export this type alias.

```rust
pub type EndianBuf<''input, Endian> = EndianSlice<''input, Endian>;
```

#### Enum `Error`

**Attributes:**

- `#[non_exhaustive]`

An error that occurred when parsing.

```rust
pub enum Error {
    Io,
    PcRelativePointerButSectionBaseIsUndefined,
    TextRelativePointerButTextBaseIsUndefined,
    DataRelativePointerButDataBaseIsUndefined,
    FuncRelativePointerInBadContext,
    CannotParseOmitPointerEncoding,
    BadUnsignedLeb128,
    BadSignedLeb128,
    AbbreviationTagZero,
    AttributeFormZero,
    BadHasChildren,
    BadLength,
    UnknownForm(constants::DwForm),
    ExpectedZero,
    DuplicateAbbreviationCode,
    DuplicateArange,
    UnknownReservedLength,
    UnknownVersion(u64),
    UnknownAbbreviation(u64),
    UnexpectedEof(ReaderOffsetId),
    UnexpectedNull,
    UnknownStandardOpcode(constants::DwLns),
    UnknownExtendedOpcode(constants::DwLne),
    UnknownLocListsEntry(constants::DwLle),
    UnknownRangeListsEntry(constants::DwRle),
    UnsupportedAddressSize(u8),
    UnsupportedOffsetSize(u8),
    UnsupportedFieldSize(u8),
    MinimumInstructionLengthZero,
    MaximumOperationsPerInstructionZero,
    LineRangeZero,
    OpcodeBaseZero,
    BadUtf8,
    NotCieId,
    NotCiePointer,
    NotFdePointer,
    BadBranchTarget(u64),
    InvalidPushObjectAddress,
    NotEnoughStackItems,
    TooManyIterations,
    InvalidExpression(constants::DwOp),
    UnsupportedEvaluation,
    InvalidPiece,
    InvalidExpressionTerminator(u64),
    DivisionByZero,
    TypeMismatch,
    IntegralTypeRequired,
    UnsupportedTypeOperation,
    InvalidShiftExpression,
    InvalidDerefSize(u8),
    UnknownCallFrameInstruction(constants::DwCfa),
    InvalidAddressRange,
    AddressOverflow,
    CfiInstructionInInvalidContext,
    PopWithEmptyStack,
    NoUnwindInfoForAddress,
    UnsupportedOffset,
    UnknownPointerEncoding(constants::DwEhPe),
    NoEntryAtGivenOffset,
    OffsetOutOfBounds,
    UnknownAugmentation,
    UnsupportedPointerEncoding,
    UnsupportedRegister(u64),
    TooManyRegisterRules,
    StackFull,
    VariableLengthSearchTable,
    UnsupportedUnitType,
    UnsupportedAddressIndex,
    UnsupportedSegmentSize,
    MissingUnitDie,
    UnsupportedAttributeForm,
    MissingFileEntryFormatPath,
    ExpectedStringAttributeValue,
    InvalidImplicitConst,
    InvalidIndexSectionCount,
    InvalidIndexSlotCount,
    InvalidIndexRow,
    UnknownIndexSection(constants::DwSect),
    UnknownIndexSectionV2(constants::DwSectV2),
}
```

##### Variants

###### `Io`

An I/O error occurred while reading.

###### `PcRelativePointerButSectionBaseIsUndefined`

Found a PC relative pointer, but the section base is undefined.

###### `TextRelativePointerButTextBaseIsUndefined`

Found a `.text` relative pointer, but the `.text` base is undefined.

###### `DataRelativePointerButDataBaseIsUndefined`

Found a data relative pointer, but the data base is undefined.

###### `FuncRelativePointerInBadContext`

Found a function relative pointer in a context that does not have a
function base.

###### `CannotParseOmitPointerEncoding`

Cannot parse a pointer with a `DW_EH_PE_omit` encoding.

###### `BadUnsignedLeb128`

An error parsing an unsigned LEB128 value.

###### `BadSignedLeb128`

An error parsing a signed LEB128 value.

###### `AbbreviationTagZero`

An abbreviation declared that its tag is zero, but zero is reserved for
null records.

###### `AttributeFormZero`

An attribute specification declared that its form is zero, but zero is
reserved for null records.

###### `BadHasChildren`

The abbreviation's has-children byte was not one of
`DW_CHILDREN_{yes,no}`.

###### `BadLength`

The specified length is impossible.

###### `UnknownForm`

Found an unknown `DW_FORM_*` type.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwForm` |  |

###### `ExpectedZero`

Expected a zero, found something else.

###### `DuplicateAbbreviationCode`

Found an abbreviation code that has already been used.

###### `DuplicateArange`

Found a duplicate arange.

###### `UnknownReservedLength`

Found an unknown reserved length value.

###### `UnknownVersion`

Found an unknown DWARF version.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

###### `UnknownAbbreviation`

Found a record with an unknown abbreviation code.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

###### `UnexpectedEof`

Hit the end of input before it was expected.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ReaderOffsetId` |  |

###### `UnexpectedNull`

Read a null entry before it was expected.

###### `UnknownStandardOpcode`

Found an unknown standard opcode.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwLns` |  |

###### `UnknownExtendedOpcode`

Found an unknown extended opcode.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwLne` |  |

###### `UnknownLocListsEntry`

Found an unknown location-lists format.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwLle` |  |

###### `UnknownRangeListsEntry`

Found an unknown range-lists format.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwRle` |  |

###### `UnsupportedAddressSize`

The specified address size is not supported.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `UnsupportedOffsetSize`

The specified offset size is not supported.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `UnsupportedFieldSize`

The specified field size is not supported.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `MinimumInstructionLengthZero`

The minimum instruction length must not be zero.

###### `MaximumOperationsPerInstructionZero`

The maximum operations per instruction must not be zero.

###### `LineRangeZero`

The line range must not be zero.

###### `OpcodeBaseZero`

The opcode base must not be zero.

###### `BadUtf8`

Found an invalid UTF-8 string.

###### `NotCieId`

Expected to find the CIE ID, but found something else.

###### `NotCiePointer`

Expected to find a pointer to a CIE, but found the CIE ID instead.

###### `NotFdePointer`

Expected to find a pointer to an FDE, but found a CIE instead.

###### `BadBranchTarget`

Invalid branch target for a DW_OP_bra or DW_OP_skip.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

###### `InvalidPushObjectAddress`

DW_OP_push_object_address used but no address passed in.

###### `NotEnoughStackItems`

Not enough items on the stack when evaluating an expression.

###### `TooManyIterations`

Too many iterations to compute the expression.

###### `InvalidExpression`

An unrecognized operation was found while parsing a DWARF
expression.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwOp` |  |

###### `UnsupportedEvaluation`

An unsupported operation was found while evaluating a DWARF expression.

###### `InvalidPiece`

The expression had a piece followed by an expression
terminator without a piece.

###### `InvalidExpressionTerminator`

An expression-terminating operation was followed by something
other than the end of the expression or a piece operation.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

###### `DivisionByZero`

Division or modulus by zero when evaluating an expression.

###### `TypeMismatch`

An expression operation used mismatching types.

###### `IntegralTypeRequired`

An expression operation required an integral type but saw a
floating point type.

###### `UnsupportedTypeOperation`

An expression operation used types that are not supported.

###### `InvalidShiftExpression`

The shift value in an expression must be a non-negative integer.

###### `InvalidDerefSize`

The size of a deref expression must not be larger than the size of an address.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `UnknownCallFrameInstruction`

An unknown DW_CFA_* instruction.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwCfa` |  |

###### `InvalidAddressRange`

The end of an address range was before the beginning.

###### `AddressOverflow`

An address calculation overflowed.

This is returned in cases where the address is expected to be
larger than a previous address, but the calculation overflowed.

###### `CfiInstructionInInvalidContext`

Encountered a call frame instruction in a context in which it is not
valid.

###### `PopWithEmptyStack`

When evaluating call frame instructions, found a `DW_CFA_restore_state`
stack pop instruction, but the stack was empty, and had nothing to pop.

###### `NoUnwindInfoForAddress`

Do not have unwind info for the given address.

###### `UnsupportedOffset`

An offset value was larger than the maximum supported value.

###### `UnknownPointerEncoding`

The given pointer encoding is either unknown or invalid.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwEhPe` |  |

###### `NoEntryAtGivenOffset`

Did not find an entry at the given offset.

###### `OffsetOutOfBounds`

The given offset is out of bounds.

###### `UnknownAugmentation`

Found an unknown CFI augmentation.

###### `UnsupportedPointerEncoding`

We do not support the given pointer encoding yet.

###### `UnsupportedRegister`

Registers larger than `u16` are not supported.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

###### `TooManyRegisterRules`

The CFI program defined more register rules than we have storage for.

###### `StackFull`

Attempted to push onto the CFI or evaluation stack, but it was already
at full capacity.

###### `VariableLengthSearchTable`

The `.eh_frame_hdr` binary search table claims to be variable-length encoded,
which makes binary search impossible.

###### `UnsupportedUnitType`

The `DW_UT_*` value for this unit is not supported yet.

###### `UnsupportedAddressIndex`

Ranges using AddressIndex are not supported yet.

###### `UnsupportedSegmentSize`

Nonzero segment selector sizes aren't supported yet.

###### `MissingUnitDie`

A compilation unit or type unit is missing its top level DIE.

###### `UnsupportedAttributeForm`

A DIE attribute used an unsupported form.

###### `MissingFileEntryFormatPath`

Missing DW_LNCT_path in file entry format.

###### `ExpectedStringAttributeValue`

Expected an attribute value to be a string form.

###### `InvalidImplicitConst`

`DW_FORM_implicit_const` used in an invalid context.

###### `InvalidIndexSectionCount`

Invalid section count in `.dwp` index.

###### `InvalidIndexSlotCount`

Invalid slot count in `.dwp` index.

###### `InvalidIndexRow`

Invalid hash row in `.dwp` index.

###### `UnknownIndexSection`

Unknown section type in `.dwp` index.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwSect` |  |

###### `UnknownIndexSectionV2`

Unknown section type in version 2 `.dwp` index.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `constants::DwSectV2` |  |

##### Implementations

###### Methods

- ```rust
  pub fn description(self: &Self) -> &str { /* ... */ }
  ```
  A short description of the error.

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> ::core::result::Result<(), fmt::Error> { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Type Alias `Result`

The result of a parse.

```rust
pub type Result<T> = result::Result<T, Error>;
```

### Traits

#### Trait `Section`

A convenience trait for loading DWARF sections from object files.  To be
used like:

```
use gimli::{DebugInfo, EndianSlice, LittleEndian, Reader, Section};

let buf = [0x00, 0x01, 0x02, 0x03];
let reader = EndianSlice::new(&buf, LittleEndian);
let loader = |name| -> Result<_, ()> { Ok(reader) };

let debug_info: DebugInfo<_> = Section::load(loader).unwrap();
```

```rust
pub trait Section<R>: From<R> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `id`: Returns the section id for this type.
- `reader`: Returns the `Reader` for this section.

##### Provided Methods

- ```rust
  fn section_name() -> &''static str { /* ... */ }
  ```
  Returns the ELF section name for this type.

- ```rust
  fn dwo_section_name() -> Option<&''static str> { /* ... */ }
  ```
  Returns the ELF section name (if any) for this type when used in a dwo

- ```rust
  fn xcoff_section_name() -> Option<&''static str> { /* ... */ }
  ```
  Returns the XCOFF section name (if any) for this type when used in a XCOFF

- ```rust
  fn load<F, E>(f: F) -> core::result::Result<Self, E>
where
    F: FnOnce(SectionId) -> core::result::Result<R, E> { /* ... */ }
  ```
  Try to load the section using the given loader function.

- ```rust
  fn dwp_range(self: &Self, offset: u32, size: u32) -> Result<Self>
where
    R: Reader { /* ... */ }
  ```
  Returns the subrange of the section that is the contribution of

- ```rust
  fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>
where
    R: Reader { /* ... */ }
  ```
  Returns the `Reader` for this section.

##### Implementations

This trait is implemented for the following types:

- `DebugAddr<R>` with <R>
- `DebugFrame<R>` with <R: Reader>
- `EhFrameHdr<R>` with <R: Reader>
- `EhFrame<R>` with <R: Reader>
- `DebugAbbrev<R>` with <R>
- `DebugAranges<R>` with <R>
- `DebugCuIndex<R>` with <R>
- `DebugTuIndex<R>` with <R>
- `DebugLine<R>` with <R>
- `DebugLoc<R>` with <R>
- `DebugLocLists<R>` with <R>
- `DebugPubNames<R>` with <R: Reader>
- `DebugPubTypes<R>` with <R: Reader>
- `DebugRanges<R>` with <R>
- `DebugRngLists<R>` with <R>
- `DebugStr<R>` with <R>
- `DebugStrOffsets<R>` with <R>
- `DebugLineStr<R>` with <R>
- `DebugInfo<R>` with <R>
- `DebugTypes<R>` with <R>

### Re-exports

#### Re-export `util::*`

```rust
pub use util::*;
```

#### Re-export `self::addr::*`

```rust
pub use self::addr::*;
```

#### Re-export `self::cfi::*`

```rust
pub use self::cfi::*;
```

#### Re-export `self::dwarf::*`

**Attributes:**

- `#[<cfg>(feature = "read")]`

```rust
pub use self::dwarf::*;
```

#### Re-export `self::endian_slice::*`

```rust
pub use self::endian_slice::*;
```

#### Re-export `self::reader::*`

```rust
pub use self::reader::*;
```

#### Re-export `self::relocate::*`

```rust
pub use self::relocate::*;
```

#### Re-export `self::abbrev::*`

**Attributes:**

- `#[<cfg>(feature = "read")]`

```rust
pub use self::abbrev::*;
```

#### Re-export `self::aranges::*`

```rust
pub use self::aranges::*;
```

#### Re-export `self::index::*`

```rust
pub use self::index::*;
```

#### Re-export `self::line::*`

**Attributes:**

- `#[<cfg>(feature = "read")]`

```rust
pub use self::line::*;
```

#### Re-export `self::loclists::*`

```rust
pub use self::loclists::*;
```

#### Re-export `self::op::*`

```rust
pub use self::op::*;
```

#### Re-export `self::pubnames::*`

**Attributes:**

- `#[<cfg>(feature = "read")]`

```rust
pub use self::pubnames::*;
```

#### Re-export `self::pubtypes::*`

**Attributes:**

- `#[<cfg>(feature = "read")]`

```rust
pub use self::pubtypes::*;
```

#### Re-export `self::rnglists::*`

```rust
pub use self::rnglists::*;
```

#### Re-export `self::str::*`

```rust
pub use self::str::*;
```

#### Re-export `self::unit::*`

**Attributes:**

- `#[<cfg>(feature = "read")]`

```rust
pub use self::unit::*;
```

#### Re-export `self::value::*`

```rust
pub use self::value::*;
```

## Re-exports

### Re-export `crate::common::*`

```rust
pub use crate::common::*;
```

### Re-export `crate::arch::*`

```rust
pub use crate::arch::*;
```

### Re-export `crate::constants::*`

```rust
pub use crate::constants::*;
```

### Re-export `crate::endianity::*`

```rust
pub use crate::endianity::*;
```

### Re-export `crate::read::*`

**Attributes:**

- `#[<cfg>(feature = "read-core")]`

```rust
pub use crate::read::*;
```

