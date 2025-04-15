# Crate Documentation

**Version:** 0.2.21

**Format Version:** 43

# Module `ppv_lite86`

## Modules

## Module `generic`

**Attributes:**

- `#[<cfg>(any(feature = "no_simd", miri, not(target_arch = "x86_64"),
all(target_arch = "x86_64", not(target_feature = "sse2"))))]`
- `#[allow(non_camel_case_types)]`

```rust
pub mod generic { /* ... */ }
```

### Types

#### Union `vec128_storage`

**Attributes:**

- `#[repr(C)]`

```rust
pub union vec128_storage {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Copy**
- **Unpin**
- **FromZeros**
- **VZip**
  - `vzip`: 
- **Freeze**
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **Any**
  - `type_id`: 
- **Into**
  - `into`: Calls `U::from(self)`.
- **CloneToUninit**
  - `clone_to_uninit`: 
- **FromBytes**
- **Eq**
- **PartialEq**
  - `eq`: 
- **Store**
  - `unpack`: 
  - `unpack`: 
  - `unpack`: 
- **From**
  - `from`: Returns the argument unchanged.
  - `from`: 
  - `from`: 
  - `from`: 
  - `from`: 
  - `from`: 
  - `from`: 
  - `from`: 
- **BorrowMut**
  - `borrow_mut`: 
- **Borrow**
  - `borrow`: 
- **Send**
- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Clone**
  - `clone`: 
- **IntoBytes**
- **Immutable**
- **Default**
  - `default`: 
- **TryFromBytes**

#### Struct `vec256_storage`

```rust
pub struct vec256_storage {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new128(v128: [vec128_storage; 2]) -> Self { /* ... */ }
  ```

- ```rust
  pub fn split128(self: Self) -> [vec128_storage; 2] { /* ... */ }
  ```

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> vec256_storage { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &vec256_storage) -> bool { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> vec256_storage { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(q: vec256_storage) -> Self { /* ... */ }
    ```

  - ```rust
    fn from([a, b, c, d]: [u64; 4]) -> Self { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `vec512_storage`

```rust
pub struct vec512_storage {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new128(v128: [vec128_storage; 4]) -> Self { /* ... */ }
  ```

- ```rust
  pub fn split128(self: Self) -> [vec128_storage; 4] { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> vec512_storage { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> vec512_storage { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &vec512_storage) -> bool { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **RefUnwindSafe**
#### Struct `GenericMachine`

```rust
pub struct GenericMachine;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **u128x4**
- **Machine**
  - ```rust
    unsafe fn instance() -> Self { /* ... */ }
    ```

- **u128x2**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **u128x1**
- **u64x2x2**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **u64x2x4**
- **Freeze**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **u32x4x2**
- **u64x4**
- **u32x4x4**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenericMachine { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **u32x4**
- **RefUnwindSafe**
- **u64x2**
#### Struct `u32x4_generic`

```rust
pub struct u32x4_generic(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **FromZeros**
- **Freeze**
- **TryFromBytes**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(d: u32x4_generic) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Immutable**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> u32x4_generic { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Store**
  - ```rust
    unsafe fn unpack(s: vec128_storage) -> Self { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &u32x4_generic) -> bool { /* ... */ }
    ```

- **ArithOps**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BitOps0**
- **Add**
  - ```rust
    fn add(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **StoreBytes**
  - ```rust
    unsafe fn unsafe_read_le(input: &[u8]) -> Self { /* ... */ }
    ```

  - ```rust
    unsafe fn unsafe_read_be(input: &[u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn write_le(self: Self, out: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn write_be(self: Self, out: &mut [u8]) { /* ... */ }
    ```

- **MultiLane**
  - ```rust
    fn to_lanes(self: Self) -> [u32; 4] { /* ... */ }
    ```

  - ```rust
    fn from_lanes(xs: [u32; 4]) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Vec4**
  - ```rust
    fn extract(self: Self, i: u32) -> u32 { /* ... */ }
    ```

  - ```rust
    fn insert(self: Self, v: u32, i: u32) -> Self { /* ... */ }
    ```

- **LaneWords4**
  - ```rust
    fn shuffle_lane_words2301(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn shuffle_lane_words1230(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn shuffle_lane_words3012(self: Self) -> Self { /* ... */ }
    ```

- **BSwap**
  - ```rust
    fn bswap(self: Self) -> Self { /* ... */ }
    ```

- **u32x4**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **AndNot**
  - ```rust
    fn andnot(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **IntoBytes**
- **Unpin**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **StructuralPartialEq**
- **Words4**
  - ```rust
    fn shuffle2301(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn shuffle1230(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn shuffle3012(self: Self) -> Self { /* ... */ }
    ```

- **RotateEachWord32**
  - ```rust
    fn rotate_each_word_right7(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right8(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right11(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right12(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right16(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right20(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right24(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right25(self: Self) -> Self { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **FromBytes**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **Swap64**
  - ```rust
    fn swap1(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap2(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap4(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap8(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap16(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap32(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap64(self: Self) -> Self { /* ... */ }
    ```

- **Copy**
- **BitOps32**
#### Struct `u64x2_generic`

```rust
pub struct u64x2_generic(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **AndNot**
  - ```rust
    fn andnot(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
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

- **Copy**
- **RotateEachWord64**
  - ```rust
    fn rotate_each_word_right32(self: Self) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> u64x2_generic { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &u64x2_generic) -> bool { /* ... */ }
    ```

- **Freeze**
- **BitOps64**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **FromZeros**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FromBytes**
- **IntoBytes**
- **ArithOps**
- **BSwap**
  - ```rust
    fn bswap(self: Self) -> Self { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **Vec2**
  - ```rust
    fn extract(self: Self, i: u32) -> u64 { /* ... */ }
    ```

  - ```rust
    fn insert(self: Self, v: u64, i: u32) -> Self { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **RotateEachWord32**
  - ```rust
    fn rotate_each_word_right7(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right8(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right11(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right12(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right16(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right20(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right24(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right25(self: Self) -> Self { /* ... */ }
    ```

- **Send**
- **u64x2**
- **MultiLane**
  - ```rust
    fn to_lanes(self: Self) -> [u64; 2] { /* ... */ }
    ```

  - ```rust
    fn from_lanes(xs: [u64; 2]) -> Self { /* ... */ }
    ```

- **Immutable**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **UnwindSafe**
- **BitOps32**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Store**
  - ```rust
    unsafe fn unpack(s: vec128_storage) -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryFromBytes**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Swap64**
  - ```rust
    fn swap1(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap2(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap4(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap8(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap16(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap32(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap64(self: Self) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **BitOps0**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **StoreBytes**
  - ```rust
    unsafe fn unsafe_read_le(input: &[u8]) -> Self { /* ... */ }
    ```

  - ```rust
    unsafe fn unsafe_read_be(input: &[u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn write_le(self: Self, out: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn write_be(self: Self, out: &mut [u8]) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(q: u64x2_generic) -> Self { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

#### Struct `u128x1_generic`

```rust
pub struct u128x1_generic(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **u128x1**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Store**
  - ```rust
    unsafe fn unpack(s: vec128_storage) -> Self { /* ... */ }
    ```

- **RotateEachWord64**
  - ```rust
    fn rotate_each_word_right32(self: Self) -> Self { /* ... */ }
    ```

- **FromBytes**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &u128x1_generic) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **FromZeros**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BitOps0**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **IntoBytes**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> u128x1_generic { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Swap64**
  - ```rust
    fn swap1(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap2(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap4(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap8(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap16(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap32(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn swap64(self: Self) -> Self { /* ... */ }
    ```

- **Copy**
- **TryFromBytes**
- **BitOps64**
- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **RotateEachWord128**
- **BitOps32**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RotateEachWord32**
  - ```rust
    fn rotate_each_word_right7(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right8(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right11(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right12(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right16(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right20(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right24(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn rotate_each_word_right25(self: Self) -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **Immutable**
- **Add**
  - ```rust
    fn add(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **AndNot**
  - ```rust
    fn andnot(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(o: u128x1_generic) -> Self { /* ... */ }
    ```

- **BitOps128**
- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **ArithOps**
- **MultiLane**
  - ```rust
    fn to_lanes(self: Self) -> [u128; 1] { /* ... */ }
    ```

  - ```rust
    fn from_lanes(xs: [u128; 1]) -> Self { /* ... */ }
    ```

- **Sync**
- **BSwap**
  - ```rust
    fn bswap(self: Self) -> Self { /* ... */ }
    ```

#### Struct `G0`

```rust
pub struct G0;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> G0 { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `G1`

```rust
pub struct G1;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Copy**
- **Freeze**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> G1 { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

#### Type Alias `u32x4x2_generic`

```rust
pub type u32x4x2_generic = crate::soft::x2<u32x4_generic, G0>;
```

#### Type Alias `u64x2x2_generic`

```rust
pub type u64x2x2_generic = crate::soft::x2<u64x2_generic, G0>;
```

#### Type Alias `u64x4_generic`

```rust
pub type u64x4_generic = crate::soft::x2<u64x2_generic, G1>;
```

#### Type Alias `u128x2_generic`

```rust
pub type u128x2_generic = crate::soft::x2<u128x1_generic, G0>;
```

#### Type Alias `u32x4x4_generic`

```rust
pub type u32x4x4_generic = crate::soft::x4<u32x4_generic>;
```

#### Type Alias `u64x2x4_generic`

```rust
pub type u64x2x4_generic = crate::soft::x4<u64x2_generic>;
```

#### Type Alias `u128x4_generic`

```rust
pub type u128x4_generic = crate::soft::x4<u128x1_generic>;
```

## Macros

### Macro `dispatch`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! dispatch {
    /* macro_rules! dispatch {
    ($mach:ident, $MTy:ident, { $([$pub:tt$(($krate:tt))*])* fn $name:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty $body:block }) => { ... };
    ($mach:ident, $MTy:ident, { $([$pub:tt $(($krate:tt))*])* fn $name:ident($($arg:ident: $argty:ty),* $(,)*) $body:block }) => { ... };
} */
}
```

### Macro `dispatch_light128`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! dispatch_light128 {
    /* macro_rules! dispatch_light128 {
    ($mach:ident, $MTy:ident, { $([$pub:tt$(($krate:tt))*])* fn $name:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty $body:block }) => { ... };
    ($mach:ident, $MTy:ident, { $([$pub:tt $(($krate:tt))*])* fn $name:ident($($arg:ident: $argty:ty),* $(,)*) $body:block }) => { ... };
} */
}
```

### Macro `dispatch_light256`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! dispatch_light256 {
    /* macro_rules! dispatch_light256 {
    ($mach:ident, $MTy:ident, { $([$pub:tt$(($krate:tt))*])* fn $name:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty $body:block }) => { ... };
    ($mach:ident, $MTy:ident, { $([$pub:tt $(($krate:tt))*])* fn $name:ident($($arg:ident: $argty:ty),* $(,)*) $body:block }) => { ... };
} */
}
```

### Macro `dispatch_light512`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! dispatch_light512 {
    /* macro_rules! dispatch_light512 {
    ($mach:ident, $MTy:ident, { $([$pub:tt$(($krate:tt))*])* fn $name:ident($($arg:ident: $argty:ty),* $(,)*) -> $ret:ty $body:block }) => { ... };
    ($mach:ident, $MTy:ident, { $([$pub:tt $(($krate:tt))*])* fn $name:ident($($arg:ident: $argty:ty),* $(,)*) $body:block }) => { ... };
} */
}
```

## Re-exports

### Re-export `vec128_storage`

```rust
pub use self::arch::vec128_storage;
```

### Re-export `vec256_storage`

```rust
pub use self::arch::vec256_storage;
```

### Re-export `vec512_storage`

```rust
pub use self::arch::vec512_storage;
```

### Re-export `self::types::*`

```rust
pub use self::types::*;
```

