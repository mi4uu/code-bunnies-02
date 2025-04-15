# Crate Documentation

**Version:** 2.0.15+zstd.1.5.7

**Format Version:** 43

# Module `zstd_sys`

Low-level bindings to the [zstd] library.

[zstd]: https://facebook.github.io/zstd/

## Types

### Enum `ZSTD_ErrorCode`

**Attributes:**

- `#[repr(u32)]`

```rust
pub enum ZSTD_ErrorCode {
    ZSTD_error_no_error = 0,
    ZSTD_error_GENERIC = 1,
    ZSTD_error_prefix_unknown = 10,
    ZSTD_error_version_unsupported = 12,
    ZSTD_error_frameParameter_unsupported = 14,
    ZSTD_error_frameParameter_windowTooLarge = 16,
    ZSTD_error_corruption_detected = 20,
    ZSTD_error_checksum_wrong = 22,
    ZSTD_error_literals_headerWrong = 24,
    ZSTD_error_dictionary_corrupted = 30,
    ZSTD_error_dictionary_wrong = 32,
    ZSTD_error_dictionaryCreation_failed = 34,
    ZSTD_error_parameter_unsupported = 40,
    ZSTD_error_parameter_combination_unsupported = 41,
    ZSTD_error_parameter_outOfBound = 42,
    ZSTD_error_tableLog_tooLarge = 44,
    ZSTD_error_maxSymbolValue_tooLarge = 46,
    ZSTD_error_maxSymbolValue_tooSmall = 48,
    ZSTD_error_cannotProduce_uncompressedBlock = 49,
    ZSTD_error_stabilityCondition_notRespected = 50,
    ZSTD_error_stage_wrong = 60,
    ZSTD_error_init_missing = 62,
    ZSTD_error_memory_allocation = 64,
    ZSTD_error_workSpace_tooSmall = 66,
    ZSTD_error_dstSize_tooSmall = 70,
    ZSTD_error_srcSize_wrong = 72,
    ZSTD_error_dstBuffer_null = 74,
    ZSTD_error_noForwardProgress_destFull = 80,
    ZSTD_error_noForwardProgress_inputEmpty = 82,
    ZSTD_error_frameIndex_tooLarge = 100,
    ZSTD_error_seekableIO = 102,
    ZSTD_error_dstBuffer_wrong = 104,
    ZSTD_error_srcBuffer_wrong = 105,
    ZSTD_error_sequenceProducer_failed = 106,
    ZSTD_error_externalSequences_invalid = 107,
    ZSTD_error_maxCode = 120,
}
```

#### Variants

##### `ZSTD_error_no_error`

Discriminant: `0`

Discriminant value: `0`

##### `ZSTD_error_GENERIC`

Discriminant: `1`

Discriminant value: `1`

##### `ZSTD_error_prefix_unknown`

Discriminant: `10`

Discriminant value: `10`

##### `ZSTD_error_version_unsupported`

Discriminant: `12`

Discriminant value: `12`

##### `ZSTD_error_frameParameter_unsupported`

Discriminant: `14`

Discriminant value: `14`

##### `ZSTD_error_frameParameter_windowTooLarge`

Discriminant: `16`

Discriminant value: `16`

##### `ZSTD_error_corruption_detected`

Discriminant: `20`

Discriminant value: `20`

##### `ZSTD_error_checksum_wrong`

Discriminant: `22`

Discriminant value: `22`

##### `ZSTD_error_literals_headerWrong`

Discriminant: `24`

Discriminant value: `24`

##### `ZSTD_error_dictionary_corrupted`

Discriminant: `30`

Discriminant value: `30`

##### `ZSTD_error_dictionary_wrong`

Discriminant: `32`

Discriminant value: `32`

##### `ZSTD_error_dictionaryCreation_failed`

Discriminant: `34`

Discriminant value: `34`

##### `ZSTD_error_parameter_unsupported`

Discriminant: `40`

Discriminant value: `40`

##### `ZSTD_error_parameter_combination_unsupported`

Discriminant: `41`

Discriminant value: `41`

##### `ZSTD_error_parameter_outOfBound`

Discriminant: `42`

Discriminant value: `42`

##### `ZSTD_error_tableLog_tooLarge`

Discriminant: `44`

Discriminant value: `44`

##### `ZSTD_error_maxSymbolValue_tooLarge`

Discriminant: `46`

Discriminant value: `46`

##### `ZSTD_error_maxSymbolValue_tooSmall`

Discriminant: `48`

Discriminant value: `48`

##### `ZSTD_error_cannotProduce_uncompressedBlock`

Discriminant: `49`

Discriminant value: `49`

##### `ZSTD_error_stabilityCondition_notRespected`

Discriminant: `50`

Discriminant value: `50`

##### `ZSTD_error_stage_wrong`

Discriminant: `60`

Discriminant value: `60`

##### `ZSTD_error_init_missing`

Discriminant: `62`

Discriminant value: `62`

##### `ZSTD_error_memory_allocation`

Discriminant: `64`

Discriminant value: `64`

##### `ZSTD_error_workSpace_tooSmall`

Discriminant: `66`

Discriminant value: `66`

##### `ZSTD_error_dstSize_tooSmall`

Discriminant: `70`

Discriminant value: `70`

##### `ZSTD_error_srcSize_wrong`

Discriminant: `72`

Discriminant value: `72`

##### `ZSTD_error_dstBuffer_null`

Discriminant: `74`

Discriminant value: `74`

##### `ZSTD_error_noForwardProgress_destFull`

Discriminant: `80`

Discriminant value: `80`

##### `ZSTD_error_noForwardProgress_inputEmpty`

Discriminant: `82`

Discriminant value: `82`

##### `ZSTD_error_frameIndex_tooLarge`

Discriminant: `100`

Discriminant value: `100`

##### `ZSTD_error_seekableIO`

Discriminant: `102`

Discriminant value: `102`

##### `ZSTD_error_dstBuffer_wrong`

Discriminant: `104`

Discriminant value: `104`

##### `ZSTD_error_srcBuffer_wrong`

Discriminant: `105`

Discriminant value: `105`

##### `ZSTD_error_sequenceProducer_failed`

Discriminant: `106`

Discriminant value: `106`

##### `ZSTD_error_externalSequences_invalid`

Discriminant: `107`

Discriminant value: `107`

##### `ZSTD_error_maxCode`

Discriminant: `120`

Discriminant value: `120`

#### Implementations

##### Trait Implementations

- **StructuralPartialEq**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_ErrorCode { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ZSTD_ErrorCode) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
### Struct `ZSTD_CCtx_s`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct ZSTD_CCtx_s {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_CCtx_s { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Type Alias `ZSTD_CCtx`

Explicit context

```rust
pub type ZSTD_CCtx = ZSTD_CCtx_s;
```

### Struct `ZSTD_DCtx_s`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct ZSTD_DCtx_s {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Copy**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_DCtx_s { /* ... */ }
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

- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Send**
### Type Alias `ZSTD_DCtx`

```rust
pub type ZSTD_DCtx = ZSTD_DCtx_s;
```

### Enum `ZSTD_strategy`

**Attributes:**

- `#[repr(u32)]`

Advanced compression API (Requires v1.4.0+)

```rust
pub enum ZSTD_strategy {
    ZSTD_fast = 1,
    ZSTD_dfast = 2,
    ZSTD_greedy = 3,
    ZSTD_lazy = 4,
    ZSTD_lazy2 = 5,
    ZSTD_btlazy2 = 6,
    ZSTD_btopt = 7,
    ZSTD_btultra = 8,
    ZSTD_btultra2 = 9,
}
```

#### Variants

##### `ZSTD_fast`

Discriminant: `1`

Discriminant value: `1`

##### `ZSTD_dfast`

Discriminant: `2`

Discriminant value: `2`

##### `ZSTD_greedy`

Discriminant: `3`

Discriminant value: `3`

##### `ZSTD_lazy`

Discriminant: `4`

Discriminant value: `4`

##### `ZSTD_lazy2`

Discriminant: `5`

Discriminant value: `5`

##### `ZSTD_btlazy2`

Discriminant: `6`

Discriminant value: `6`

##### `ZSTD_btopt`

Discriminant: `7`

Discriminant value: `7`

##### `ZSTD_btultra`

Discriminant: `8`

Discriminant value: `8`

##### `ZSTD_btultra2`

Discriminant: `9`

Discriminant value: `9`

#### Implementations

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ZSTD_strategy) -> bool { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_strategy { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
### Enum `ZSTD_cParameter`

**Attributes:**

- `#[repr(u32)]`

```rust
pub enum ZSTD_cParameter {
    ZSTD_c_compressionLevel = 100,
    ZSTD_c_windowLog = 101,
    ZSTD_c_hashLog = 102,
    ZSTD_c_chainLog = 103,
    ZSTD_c_searchLog = 104,
    ZSTD_c_minMatch = 105,
    ZSTD_c_targetLength = 106,
    ZSTD_c_strategy = 107,
    ZSTD_c_targetCBlockSize = 130,
    ZSTD_c_enableLongDistanceMatching = 160,
    ZSTD_c_ldmHashLog = 161,
    ZSTD_c_ldmMinMatch = 162,
    ZSTD_c_ldmBucketSizeLog = 163,
    ZSTD_c_ldmHashRateLog = 164,
    ZSTD_c_contentSizeFlag = 200,
    ZSTD_c_checksumFlag = 201,
    ZSTD_c_dictIDFlag = 202,
    ZSTD_c_nbWorkers = 400,
    ZSTD_c_jobSize = 401,
    ZSTD_c_overlapLog = 402,
    ZSTD_c_experimentalParam1 = 500,
    ZSTD_c_experimentalParam2 = 10,
    ZSTD_c_experimentalParam3 = 1000,
    ZSTD_c_experimentalParam4 = 1001,
    ZSTD_c_experimentalParam5 = 1002,
    ZSTD_c_experimentalParam7 = 1004,
    ZSTD_c_experimentalParam8 = 1005,
    ZSTD_c_experimentalParam9 = 1006,
    ZSTD_c_experimentalParam10 = 1007,
    ZSTD_c_experimentalParam11 = 1008,
    ZSTD_c_experimentalParam12 = 1009,
    ZSTD_c_experimentalParam13 = 1010,
    ZSTD_c_experimentalParam14 = 1011,
    ZSTD_c_experimentalParam15 = 1012,
    ZSTD_c_experimentalParam16 = 1013,
    ZSTD_c_experimentalParam17 = 1014,
    ZSTD_c_experimentalParam18 = 1015,
    ZSTD_c_experimentalParam19 = 1016,
    ZSTD_c_experimentalParam20 = 1017,
}
```

#### Variants

##### `ZSTD_c_compressionLevel`

Discriminant: `100`

Discriminant value: `100`

##### `ZSTD_c_windowLog`

Discriminant: `101`

Discriminant value: `101`

##### `ZSTD_c_hashLog`

Discriminant: `102`

Discriminant value: `102`

##### `ZSTD_c_chainLog`

Discriminant: `103`

Discriminant value: `103`

##### `ZSTD_c_searchLog`

Discriminant: `104`

Discriminant value: `104`

##### `ZSTD_c_minMatch`

Discriminant: `105`

Discriminant value: `105`

##### `ZSTD_c_targetLength`

Discriminant: `106`

Discriminant value: `106`

##### `ZSTD_c_strategy`

Discriminant: `107`

Discriminant value: `107`

##### `ZSTD_c_targetCBlockSize`

Discriminant: `130`

Discriminant value: `130`

##### `ZSTD_c_enableLongDistanceMatching`

Discriminant: `160`

Discriminant value: `160`

##### `ZSTD_c_ldmHashLog`

Discriminant: `161`

Discriminant value: `161`

##### `ZSTD_c_ldmMinMatch`

Discriminant: `162`

Discriminant value: `162`

##### `ZSTD_c_ldmBucketSizeLog`

Discriminant: `163`

Discriminant value: `163`

##### `ZSTD_c_ldmHashRateLog`

Discriminant: `164`

Discriminant value: `164`

##### `ZSTD_c_contentSizeFlag`

Discriminant: `200`

Discriminant value: `200`

##### `ZSTD_c_checksumFlag`

Discriminant: `201`

Discriminant value: `201`

##### `ZSTD_c_dictIDFlag`

Discriminant: `202`

Discriminant value: `202`

##### `ZSTD_c_nbWorkers`

Discriminant: `400`

Discriminant value: `400`

##### `ZSTD_c_jobSize`

Discriminant: `401`

Discriminant value: `401`

##### `ZSTD_c_overlapLog`

Discriminant: `402`

Discriminant value: `402`

##### `ZSTD_c_experimentalParam1`

Discriminant: `500`

Discriminant value: `500`

##### `ZSTD_c_experimentalParam2`

Discriminant: `10`

Discriminant value: `10`

##### `ZSTD_c_experimentalParam3`

Discriminant: `1000`

Discriminant value: `1000`

##### `ZSTD_c_experimentalParam4`

Discriminant: `1001`

Discriminant value: `1001`

##### `ZSTD_c_experimentalParam5`

Discriminant: `1002`

Discriminant value: `1002`

##### `ZSTD_c_experimentalParam7`

Discriminant: `1004`

Discriminant value: `1004`

##### `ZSTD_c_experimentalParam8`

Discriminant: `1005`

Discriminant value: `1005`

##### `ZSTD_c_experimentalParam9`

Discriminant: `1006`

Discriminant value: `1006`

##### `ZSTD_c_experimentalParam10`

Discriminant: `1007`

Discriminant value: `1007`

##### `ZSTD_c_experimentalParam11`

Discriminant: `1008`

Discriminant value: `1008`

##### `ZSTD_c_experimentalParam12`

Discriminant: `1009`

Discriminant value: `1009`

##### `ZSTD_c_experimentalParam13`

Discriminant: `1010`

Discriminant value: `1010`

##### `ZSTD_c_experimentalParam14`

Discriminant: `1011`

Discriminant value: `1011`

##### `ZSTD_c_experimentalParam15`

Discriminant: `1012`

Discriminant value: `1012`

##### `ZSTD_c_experimentalParam16`

Discriminant: `1013`

Discriminant value: `1013`

##### `ZSTD_c_experimentalParam17`

Discriminant: `1014`

Discriminant value: `1014`

##### `ZSTD_c_experimentalParam18`

Discriminant: `1015`

Discriminant value: `1015`

##### `ZSTD_c_experimentalParam19`

Discriminant: `1016`

Discriminant value: `1016`

##### `ZSTD_c_experimentalParam20`

Discriminant: `1017`

Discriminant value: `1017`

#### Implementations

##### Trait Implementations

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_cParameter { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ZSTD_cParameter) -> bool { /* ... */ }
    ```

- **Freeze**
- **Sync**
### Struct `ZSTD_bounds`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct ZSTD_bounds {
    pub error: usize,
    pub lowerBound: ::core::ffi::c_int,
    pub upperBound: ::core::ffi::c_int,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `error` | `usize` |  |
| `lowerBound` | `::core::ffi::c_int` |  |
| `upperBound` | `::core::ffi::c_int` |  |

#### Implementations

##### Trait Implementations

- **Freeze**
- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_bounds { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
### Enum `ZSTD_ResetDirective`

**Attributes:**

- `#[repr(u32)]`

```rust
pub enum ZSTD_ResetDirective {
    ZSTD_reset_session_only = 1,
    ZSTD_reset_parameters = 2,
    ZSTD_reset_session_and_parameters = 3,
}
```

#### Variants

##### `ZSTD_reset_session_only`

Discriminant: `1`

Discriminant value: `1`

##### `ZSTD_reset_parameters`

Discriminant: `2`

Discriminant value: `2`

##### `ZSTD_reset_session_and_parameters`

Discriminant: `3`

Discriminant value: `3`

#### Implementations

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_ResetDirective { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ZSTD_ResetDirective) -> bool { /* ... */ }
    ```

- **Sync**
- **Eq**
### Enum `ZSTD_dParameter`

**Attributes:**

- `#[repr(u32)]`

Advanced decompression API (Requires v1.4.0+)

```rust
pub enum ZSTD_dParameter {
    ZSTD_d_windowLogMax = 100,
    ZSTD_d_experimentalParam1 = 1000,
    ZSTD_d_experimentalParam2 = 1001,
    ZSTD_d_experimentalParam3 = 1002,
    ZSTD_d_experimentalParam4 = 1003,
    ZSTD_d_experimentalParam5 = 1004,
    ZSTD_d_experimentalParam6 = 1005,
}
```

#### Variants

##### `ZSTD_d_windowLogMax`

Discriminant: `100`

Discriminant value: `100`

##### `ZSTD_d_experimentalParam1`

Discriminant: `1000`

Discriminant value: `1000`

##### `ZSTD_d_experimentalParam2`

Discriminant: `1001`

Discriminant value: `1001`

##### `ZSTD_d_experimentalParam3`

Discriminant: `1002`

Discriminant value: `1002`

##### `ZSTD_d_experimentalParam4`

Discriminant: `1003`

Discriminant value: `1003`

##### `ZSTD_d_experimentalParam5`

Discriminant: `1004`

Discriminant value: `1004`

##### `ZSTD_d_experimentalParam6`

Discriminant: `1005`

Discriminant value: `1005`

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_dParameter { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ZSTD_dParameter) -> bool { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Copy**
### Struct `ZSTD_inBuffer_s`

**Attributes:**

- `#[repr(C)]`

Streaming

```rust
pub struct ZSTD_inBuffer_s {
    pub src: *const ::core::ffi::c_void,
    pub size: usize,
    pub pos: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `src` | `*const ::core::ffi::c_void` | < start of input buffer |
| `size` | `usize` | < size of input buffer |
| `pos` | `usize` | < position where reading stopped. Will be updated. Necessarily 0 <= pos <= size |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_inBuffer_s { /* ... */ }
    ```

### Type Alias `ZSTD_inBuffer`

Streaming

```rust
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
```

### Struct `ZSTD_outBuffer_s`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct ZSTD_outBuffer_s {
    pub dst: *mut ::core::ffi::c_void,
    pub size: usize,
    pub pos: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `dst` | `*mut ::core::ffi::c_void` | < start of output buffer |
| `size` | `usize` | < size of output buffer |
| `pos` | `usize` | < position where writing stopped. Will be updated. Necessarily 0 <= pos <= size |

#### Implementations

##### Trait Implementations

- **Copy**
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

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_outBuffer_s { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
### Type Alias `ZSTD_outBuffer`

```rust
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
```

### Type Alias `ZSTD_CStream`

```rust
pub type ZSTD_CStream = ZSTD_CCtx;
```

### Enum `ZSTD_EndDirective`

**Attributes:**

- `#[repr(u32)]`

```rust
pub enum ZSTD_EndDirective {
    ZSTD_e_continue = 0,
    ZSTD_e_flush = 1,
    ZSTD_e_end = 2,
}
```

#### Variants

##### `ZSTD_e_continue`

Discriminant: `0`

Discriminant value: `0`

##### `ZSTD_e_flush`

Discriminant: `1`

Discriminant value: `1`

##### `ZSTD_e_end`

Discriminant: `2`

Discriminant value: `2`

#### Implementations

##### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Copy**
- **Eq**
- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
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
    fn eq(self: &Self, other: &ZSTD_EndDirective) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_EndDirective { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
### Type Alias `ZSTD_DStream`

```rust
pub type ZSTD_DStream = ZSTD_DCtx;
```

### Struct `ZSTD_CDict_s`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct ZSTD_CDict_s {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_CDict_s { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **Copy**
### Type Alias `ZSTD_CDict`

Bulk processing dictionary API

```rust
pub type ZSTD_CDict = ZSTD_CDict_s;
```

### Struct `ZSTD_DDict_s`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct ZSTD_DDict_s {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZSTD_DDict_s { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
### Type Alias `ZSTD_DDict`

```rust
pub type ZSTD_DDict = ZSTD_DDict_s;
```

### Struct `ZDICT_params_t`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct ZDICT_params_t {
    pub compressionLevel: ::core::ffi::c_int,
    pub notificationLevel: ::core::ffi::c_uint,
    pub dictID: ::core::ffi::c_uint,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `compressionLevel` | `::core::ffi::c_int` | < optimize for a specific zstd compression level; 0 means default |
| `notificationLevel` | `::core::ffi::c_uint` | < Write log to stderr; 0 = none (default); 1 = errors; 2 = progression; 3 = details; 4 = debug; |
| `dictID` | `::core::ffi::c_uint` | < force dictID value; 0 means auto mode (32-bits random value)<br>   NOTE: The zstd format reserves some dictionary IDs for future use.<br>         You may use them in private settings, but be warned that they<br>         may be used by zstd in a public dictionary registry in the future.<br>         These dictionary IDs are:<br>           - low range  : <= 32767<br>           - high range : >= (2^31) |

#### Implementations

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZDICT_params_t { /* ... */ }
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

- **Unpin**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Freeze**
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

## Functions

### Function `ZSTD_getErrorString`

```rust
pub unsafe extern "C" fn ZSTD_getErrorString(code: ZSTD_ErrorCode) -> *const ::core::ffi::c_char;
```

### Function `ZSTD_versionNumber`

ZSTD_versionNumber() :
 Return runtime library version, the value is (MAJOR*100*100 + MINOR*100 + RELEASE).

```rust
pub unsafe extern "C" fn ZSTD_versionNumber() -> ::core::ffi::c_uint;
```

### Function `ZSTD_versionString`

ZSTD_versionString() :
 Return runtime library version, like "1.4.5". Requires v1.3.0+.

```rust
pub unsafe extern "C" fn ZSTD_versionString() -> *const ::core::ffi::c_char;
```

### Function `ZSTD_compress`

  Simple Core API
/
/*! ZSTD_compress() :
  Compresses `src` content as a single zstd compressed frame into already allocated `dst`.
  NOTE: Providing `dstCapacity >= ZSTD_compressBound(srcSize)` guarantees that zstd will have
        enough space to successfully compress the data.
  @return : compressed size written into `dst` (<= `dstCapacity),
            or an error code if it fails (which can be tested using ZSTD_isError()).

```rust
pub unsafe extern "C" fn ZSTD_compress(dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, srcSize: usize, compressionLevel: ::core::ffi::c_int) -> usize;
```

### Function `ZSTD_decompress`

ZSTD_decompress() :
`compressedSize` : must be the _exact_ size of some number of compressed and/or skippable frames.
 Multiple compressed frames can be decompressed at once with this method.
 The result will be the concatenation of all decompressed frames, back to back.
`dstCapacity` is an upper bound of originalSize to regenerate.
 First frame's decompressed size can be extracted using ZSTD_getFrameContentSize().
 If maximum upper bound isn't known, prefer using streaming mode to decompress data.
@return : the number of bytes decompressed into `dst` (<= `dstCapacity`),
          or an errorCode if it fails (which can be tested using ZSTD_isError()).

```rust
pub unsafe extern "C" fn ZSTD_decompress(dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, compressedSize: usize) -> usize;
```

### Function `ZSTD_getFrameContentSize`

```rust
pub unsafe extern "C" fn ZSTD_getFrameContentSize(src: *const ::core::ffi::c_void, srcSize: usize) -> ::core::ffi::c_ulonglong;
```

### Function `ZSTD_getDecompressedSize`

ZSTD_getDecompressedSize() (obsolete):
 This function is now obsolete, in favor of ZSTD_getFrameContentSize().
 Both functions work the same way, but ZSTD_getDecompressedSize() blends
 "empty", "unknown" and "error" results to the same return value (0),
 while ZSTD_getFrameContentSize() gives them separate return values.
@return : decompressed size of `src` frame content _if known and not empty_, 0 otherwise.

```rust
pub unsafe extern "C" fn ZSTD_getDecompressedSize(src: *const ::core::ffi::c_void, srcSize: usize) -> ::core::ffi::c_ulonglong;
```

### Function `ZSTD_findFrameCompressedSize`

ZSTD_findFrameCompressedSize() : Requires v1.4.0+
`src` should point to the start of a ZSTD frame or skippable frame.
`srcSize` must be >= first frame size
@return : the compressed size of the first frame starting at `src`,
          suitable to pass as `srcSize` to `ZSTD_decompress` or similar,
          or an error code if input is invalid
 Note 1: this method is called _find*() because it's not enough to read the header,
         it may have to scan through the frame's content, to reach its end.
 Note 2: this method also works with Skippable Frames. In which case,
         it returns the size of the complete skippable frame,
         which is always equal to its content size + 8 bytes for headers.

```rust
pub unsafe extern "C" fn ZSTD_findFrameCompressedSize(src: *const ::core::ffi::c_void, srcSize: usize) -> usize;
```

### Function `ZSTD_compressBound`

```rust
pub unsafe extern "C" fn ZSTD_compressBound(srcSize: usize) -> usize;
```

### Function `ZSTD_isError`

```rust
pub unsafe extern "C" fn ZSTD_isError(result: usize) -> ::core::ffi::c_uint;
```

### Function `ZSTD_getErrorCode`

```rust
pub unsafe extern "C" fn ZSTD_getErrorCode(functionResult: usize) -> ZSTD_ErrorCode;
```

### Function `ZSTD_getErrorName`

```rust
pub unsafe extern "C" fn ZSTD_getErrorName(result: usize) -> *const ::core::ffi::c_char;
```

### Function `ZSTD_minCLevel`

```rust
pub unsafe extern "C" fn ZSTD_minCLevel() -> ::core::ffi::c_int;
```

### Function `ZSTD_maxCLevel`

```rust
pub unsafe extern "C" fn ZSTD_maxCLevel() -> ::core::ffi::c_int;
```

### Function `ZSTD_defaultCLevel`

```rust
pub unsafe extern "C" fn ZSTD_defaultCLevel() -> ::core::ffi::c_int;
```

### Function `ZSTD_createCCtx`

```rust
pub unsafe extern "C" fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
```

### Function `ZSTD_freeCCtx`

```rust
pub unsafe extern "C" fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> usize;
```

### Function `ZSTD_compressCCtx`

ZSTD_compressCCtx() :
 Same as ZSTD_compress(), using an explicit ZSTD_CCtx.
 Important : in order to mirror `ZSTD_compress()` behavior,
 this function compresses at the requested compression level,
 __ignoring any other advanced parameter__ .
 If any advanced parameter was set using the advanced API,
 they will all be reset. Only @compressionLevel remains.

```rust
pub unsafe extern "C" fn ZSTD_compressCCtx(cctx: *mut ZSTD_CCtx, dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, srcSize: usize, compressionLevel: ::core::ffi::c_int) -> usize;
```

### Function `ZSTD_createDCtx`

```rust
pub unsafe extern "C" fn ZSTD_createDCtx() -> *mut ZSTD_DCtx;
```

### Function `ZSTD_freeDCtx`

```rust
pub unsafe extern "C" fn ZSTD_freeDCtx(dctx: *mut ZSTD_DCtx) -> usize;
```

### Function `ZSTD_decompressDCtx`

ZSTD_decompressDCtx() :
 Same as ZSTD_decompress(),
 requires an allocated ZSTD_DCtx.
 Compatible with sticky parameters (see below).

```rust
pub unsafe extern "C" fn ZSTD_decompressDCtx(dctx: *mut ZSTD_DCtx, dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, srcSize: usize) -> usize;
```

### Function `ZSTD_cParam_getBounds`

ZSTD_cParam_getBounds() :
 All parameters must belong to an interval with lower and upper bounds,
 otherwise they will either trigger an error or be automatically clamped.
@return : a structure, ZSTD_bounds, which contains
        - an error status field, which must be tested using ZSTD_isError()
        - lower and upper bounds, both inclusive

```rust
pub unsafe extern "C" fn ZSTD_cParam_getBounds(cParam: ZSTD_cParameter) -> ZSTD_bounds;
```

### Function `ZSTD_CCtx_setParameter`

ZSTD_CCtx_setParameter() :
 Set one compression parameter, selected by enum ZSTD_cParameter.
 All parameters have valid bounds. Bounds can be queried using ZSTD_cParam_getBounds().
 Providing a value beyond bound will either clamp it, or trigger an error (depending on parameter).
 Setting a parameter is generally only possible during frame initialization (before starting compression).
 Exception : when using multi-threading mode (nbWorkers >= 1),
             the following parameters can be updated _during_ compression (within same frame):
             => compressionLevel, hashLog, chainLog, searchLog, minMatch, targetLength and strategy.
             new parameters will be active for next job only (after a flush()).
@return : an error code (which can be tested using ZSTD_isError()).

```rust
pub unsafe extern "C" fn ZSTD_CCtx_setParameter(cctx: *mut ZSTD_CCtx, param: ZSTD_cParameter, value: ::core::ffi::c_int) -> usize;
```

### Function `ZSTD_CCtx_setPledgedSrcSize`

ZSTD_CCtx_setPledgedSrcSize() :
 Total input data size to be compressed as a single frame.
 Value will be written in frame header, unless if explicitly forbidden using ZSTD_c_contentSizeFlag.
 This value will also be controlled at end of frame, and trigger an error if not respected.
@result : 0, or an error code (which can be tested with ZSTD_isError()).
 Note 1 : pledgedSrcSize==0 actually means zero, aka an empty frame.
          In order to mean "unknown content size", pass constant ZSTD_CONTENTSIZE_UNKNOWN.
          ZSTD_CONTENTSIZE_UNKNOWN is default value for any new frame.
 Note 2 : pledgedSrcSize is only valid once, for the next frame.
          It's discarded at the end of the frame, and replaced by ZSTD_CONTENTSIZE_UNKNOWN.
 Note 3 : Whenever all input data is provided and consumed in a single round,
          for example with ZSTD_compress2(),
          or invoking immediately ZSTD_compressStream2(,,,ZSTD_e_end),
          this value is automatically overridden by srcSize instead.

```rust
pub unsafe extern "C" fn ZSTD_CCtx_setPledgedSrcSize(cctx: *mut ZSTD_CCtx, pledgedSrcSize: ::core::ffi::c_ulonglong) -> usize;
```

### Function `ZSTD_CCtx_reset`

ZSTD_CCtx_reset() :
 There are 2 different things that can be reset, independently or jointly :
 - The session : will stop compressing current frame, and make CCtx ready to start a new one.
                 Useful after an error, or to interrupt any ongoing compression.
                 Any internal data not yet flushed is cancelled.
                 Compression parameters and dictionary remain unchanged.
                 They will be used to compress next frame.
                 Resetting session never fails.
 - The parameters : changes all parameters back to "default".
                 This also removes any reference to any dictionary or external sequence producer.
                 Parameters can only be changed between 2 sessions (i.e. no compression is currently ongoing)
                 otherwise the reset fails, and function returns an error value (which can be tested using ZSTD_isError())
 - Both : similar to resetting the session, followed by resetting parameters.

```rust
pub unsafe extern "C" fn ZSTD_CCtx_reset(cctx: *mut ZSTD_CCtx, reset: ZSTD_ResetDirective) -> usize;
```

### Function `ZSTD_compress2`

ZSTD_compress2() :
 Behave the same as ZSTD_compressCCtx(), but compression parameters are set using the advanced API.
 (note that this entry point doesn't even expose a compression level parameter).
 ZSTD_compress2() always starts a new frame.
 Should cctx hold data from a previously unfinished frame, everything about it is forgotten.
 - Compression parameters are pushed into CCtx before starting compression, using ZSTD_CCtx_set*()
 - The function is always blocking, returns when compression is completed.
 NOTE: Providing `dstCapacity >= ZSTD_compressBound(srcSize)` guarantees that zstd will have
       enough space to successfully compress the data, though it is possible it fails for other reasons.
@return : compressed size written into `dst` (<= `dstCapacity),
          or an error code if it fails (which can be tested using ZSTD_isError()).

```rust
pub unsafe extern "C" fn ZSTD_compress2(cctx: *mut ZSTD_CCtx, dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, srcSize: usize) -> usize;
```

### Function `ZSTD_dParam_getBounds`

ZSTD_dParam_getBounds() :
 All parameters must belong to an interval with lower and upper bounds,
 otherwise they will either trigger an error or be automatically clamped.
@return : a structure, ZSTD_bounds, which contains
        - an error status field, which must be tested using ZSTD_isError()
        - both lower and upper bounds, inclusive

```rust
pub unsafe extern "C" fn ZSTD_dParam_getBounds(dParam: ZSTD_dParameter) -> ZSTD_bounds;
```

### Function `ZSTD_DCtx_setParameter`

ZSTD_DCtx_setParameter() :
 Set one compression parameter, selected by enum ZSTD_dParameter.
 All parameters have valid bounds. Bounds can be queried using ZSTD_dParam_getBounds().
 Providing a value beyond bound will either clamp it, or trigger an error (depending on parameter).
 Setting a parameter is only possible during frame initialization (before starting decompression).
@return : 0, or an error code (which can be tested using ZSTD_isError()).

```rust
pub unsafe extern "C" fn ZSTD_DCtx_setParameter(dctx: *mut ZSTD_DCtx, param: ZSTD_dParameter, value: ::core::ffi::c_int) -> usize;
```

### Function `ZSTD_DCtx_reset`

ZSTD_DCtx_reset() :
 Return a DCtx to clean state.
 Session and parameters can be reset jointly or separately.
 Parameters can only be reset when no active frame is being decompressed.
@return : 0, or an error code, which can be tested with ZSTD_isError()

```rust
pub unsafe extern "C" fn ZSTD_DCtx_reset(dctx: *mut ZSTD_DCtx, reset: ZSTD_ResetDirective) -> usize;
```

### Function `ZSTD_createCStream`

```rust
pub unsafe extern "C" fn ZSTD_createCStream() -> *mut ZSTD_CStream;
```

### Function `ZSTD_freeCStream`

```rust
pub unsafe extern "C" fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> usize;
```

### Function `ZSTD_compressStream2`

ZSTD_compressStream2() : Requires v1.4.0+
 Behaves about the same as ZSTD_compressStream, with additional control on end directive.
 - Compression parameters are pushed into CCtx before starting compression, using ZSTD_CCtx_set*()
 - Compression parameters cannot be changed once compression is started (save a list of exceptions in multi-threading mode)
 - output->pos must be <= dstCapacity, input->pos must be <= srcSize
 - output->pos and input->pos will be updated. They are guaranteed to remain below their respective limit.
 - endOp must be a valid directive
 - When nbWorkers==0 (default), function is blocking : it completes its job before returning to caller.
 - When nbWorkers>=1, function is non-blocking : it copies a portion of input, distributes jobs to internal worker threads, flush to output whatever is available,
                                                 and then immediately returns, just indicating that there is some data remaining to be flushed.
                                                 The function nonetheless guarantees forward progress : it will return only after it reads or write at least 1+ byte.
 - Exception : if the first call requests a ZSTD_e_end directive and provides enough dstCapacity, the function delegates to ZSTD_compress2() which is always blocking.
 - @return provides a minimum amount of data remaining to be flushed from internal buffers
           or an error code, which can be tested using ZSTD_isError().
           if @return != 0, flush is not fully completed, there is still some data left within internal buffers.
           This is useful for ZSTD_e_flush, since in this case more flushes are necessary to empty all buffers.
           For ZSTD_e_end, @return == 0 when internal buffers are fully flushed and frame is completed.
 - after a ZSTD_e_end directive, if internal buffer is not fully flushed (@return != 0),
           only ZSTD_e_end or ZSTD_e_flush operations are allowed.
           Before starting a new compression job, or changing compression parameters,
           it is required to fully flush internal buffers.
 - note: if an operation ends with an error, it may leave @cctx in an undefined state.
         Therefore, it's UB to invoke ZSTD_compressStream2() of ZSTD_compressStream() on such a state.
         In order to be re-employed after an error, a state must be reset,
         which can be done explicitly (ZSTD_CCtx_reset()),
         or is sometimes implied by methods starting a new compression job (ZSTD_initCStream(), ZSTD_compressCCtx())

```rust
pub unsafe extern "C" fn ZSTD_compressStream2(cctx: *mut ZSTD_CCtx, output: *mut ZSTD_outBuffer, input: *mut ZSTD_inBuffer, endOp: ZSTD_EndDirective) -> usize;
```

### Function `ZSTD_CStreamInSize`

```rust
pub unsafe extern "C" fn ZSTD_CStreamInSize() -> usize;
```

### Function `ZSTD_CStreamOutSize`

```rust
pub unsafe extern "C" fn ZSTD_CStreamOutSize() -> usize;
```

### Function `ZSTD_initCStream`

Equivalent to:

    ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
    ZSTD_CCtx_refCDict(zcs, NULL); // clear the dictionary (if any)
    ZSTD_CCtx_setParameter(zcs, ZSTD_c_compressionLevel, compressionLevel);

Note that ZSTD_initCStream() clears any previously set dictionary. Use the new API
to compress with a dictionary.

```rust
pub unsafe extern "C" fn ZSTD_initCStream(zcs: *mut ZSTD_CStream, compressionLevel: ::core::ffi::c_int) -> usize;
```

### Function `ZSTD_compressStream`

Alternative for ZSTD_compressStream2(zcs, output, input, ZSTD_e_continue).
NOTE: The return value is different. ZSTD_compressStream() returns a hint for
the next read size (if non-zero and not an error). ZSTD_compressStream2()
returns the minimum nb of bytes left to flush (if non-zero and not an error).

```rust
pub unsafe extern "C" fn ZSTD_compressStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer, input: *mut ZSTD_inBuffer) -> usize;
```

### Function `ZSTD_flushStream`

Equivalent to ZSTD_compressStream2(zcs, output, &emptyInput, ZSTD_e_flush).

```rust
pub unsafe extern "C" fn ZSTD_flushStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer) -> usize;
```

### Function `ZSTD_endStream`

Equivalent to ZSTD_compressStream2(zcs, output, &emptyInput, ZSTD_e_end).

```rust
pub unsafe extern "C" fn ZSTD_endStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer) -> usize;
```

### Function `ZSTD_createDStream`

```rust
pub unsafe extern "C" fn ZSTD_createDStream() -> *mut ZSTD_DStream;
```

### Function `ZSTD_freeDStream`

```rust
pub unsafe extern "C" fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> usize;
```

### Function `ZSTD_initDStream`

ZSTD_initDStream() :
Initialize/reset DStream state for new decompression operation.
Call before new decompression operation using same DStream.

Note : This function is redundant with the advanced API and equivalent to:
    ZSTD_DCtx_reset(zds, ZSTD_reset_session_only);
    ZSTD_DCtx_refDDict(zds, NULL);

```rust
pub unsafe extern "C" fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> usize;
```

### Function `ZSTD_decompressStream`

ZSTD_decompressStream() :
Streaming decompression function.
Call repetitively to consume full input updating it as necessary.
Function will update both input and output `pos` fields exposing current state via these fields:
- `input.pos < input.size`, some input remaining and caller should provide remaining input
  on the next call.
- `output.pos < output.size`, decoder flushed internal output buffer.
- `output.pos == output.size`, unflushed data potentially present in the internal buffers,
  check ZSTD_decompressStream() @return value,
  if > 0, invoke it again to flush remaining data to output.
Note : with no additional input, amount of data flushed <= ZSTD_BLOCKSIZE_MAX.

@return : 0 when a frame is completely decoded and fully flushed,
          or an error code, which can be tested using ZSTD_isError(),
          or any other value > 0, which means there is some decoding or flushing to do to complete current frame.

Note: when an operation returns with an error code, the @zds state may be left in undefined state.
      It's UB to invoke `ZSTD_decompressStream()` on such a state.
      In order to re-use such a state, it must be first reset,
      which can be done explicitly (`ZSTD_DCtx_reset()`),
      or is implied for operations starting some new decompression job (`ZSTD_initDStream`, `ZSTD_decompressDCtx()`, `ZSTD_decompress_usingDict()`)

```rust
pub unsafe extern "C" fn ZSTD_decompressStream(zds: *mut ZSTD_DStream, output: *mut ZSTD_outBuffer, input: *mut ZSTD_inBuffer) -> usize;
```

### Function `ZSTD_DStreamInSize`

```rust
pub unsafe extern "C" fn ZSTD_DStreamInSize() -> usize;
```

### Function `ZSTD_DStreamOutSize`

```rust
pub unsafe extern "C" fn ZSTD_DStreamOutSize() -> usize;
```

### Function `ZSTD_compress_usingDict`

  Simple dictionary API
/
/*! ZSTD_compress_usingDict() :
  Compression at an explicit compression level using a Dictionary.
  A dictionary can be any arbitrary data segment (also called a prefix),
  or a buffer with specified information (see zdict.h).
  Note : This function loads the dictionary, resulting in significant startup delay.
         It's intended for a dictionary used only once.
  Note 2 : When `dict == NULL || dictSize < 8` no dictionary is used.

```rust
pub unsafe extern "C" fn ZSTD_compress_usingDict(ctx: *mut ZSTD_CCtx, dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, srcSize: usize, dict: *const ::core::ffi::c_void, dictSize: usize, compressionLevel: ::core::ffi::c_int) -> usize;
```

### Function `ZSTD_decompress_usingDict`

ZSTD_decompress_usingDict() :
 Decompression using a known Dictionary.
 Dictionary must be identical to the one used during compression.
 Note : This function loads the dictionary, resulting in significant startup delay.
        It's intended for a dictionary used only once.
 Note : When `dict == NULL || dictSize < 8` no dictionary is used.

```rust
pub unsafe extern "C" fn ZSTD_decompress_usingDict(dctx: *mut ZSTD_DCtx, dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, srcSize: usize, dict: *const ::core::ffi::c_void, dictSize: usize) -> usize;
```

### Function `ZSTD_createCDict`

ZSTD_createCDict() :
 When compressing multiple messages or blocks using the same dictionary,
 it's recommended to digest the dictionary only once, since it's a costly operation.
 ZSTD_createCDict() will create a state from digesting a dictionary.
 The resulting state can be used for future compression operations with very limited startup cost.
 ZSTD_CDict can be created once and shared by multiple threads concurrently, since its usage is read-only.
@dictBuffer can be released after ZSTD_CDict creation, because its content is copied within CDict.
 Note 1 : Consider experimental function `ZSTD_createCDict_byReference()` if you prefer to not duplicate @dictBuffer content.
 Note 2 : A ZSTD_CDict can be created from an empty @dictBuffer,
     in which case the only thing that it transports is the @compressionLevel.
     This can be useful in a pipeline featuring ZSTD_compress_usingCDict() exclusively,
     expecting a ZSTD_CDict parameter with any data, including those without a known dictionary.

```rust
pub unsafe extern "C" fn ZSTD_createCDict(dictBuffer: *const ::core::ffi::c_void, dictSize: usize, compressionLevel: ::core::ffi::c_int) -> *mut ZSTD_CDict;
```

### Function `ZSTD_freeCDict`

ZSTD_freeCDict() :
 Function frees memory allocated by ZSTD_createCDict().
 If a NULL pointer is passed, no operation is performed.

```rust
pub unsafe extern "C" fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> usize;
```

### Function `ZSTD_compress_usingCDict`

ZSTD_compress_usingCDict() :
 Compression using a digested Dictionary.
 Recommended when same dictionary is used multiple times.
 Note : compression level is _decided at dictionary creation time_,
    and frame parameters are hardcoded (dictID=yes, contentSize=yes, checksum=no)

```rust
pub unsafe extern "C" fn ZSTD_compress_usingCDict(cctx: *mut ZSTD_CCtx, dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, srcSize: usize, cdict: *const ZSTD_CDict) -> usize;
```

### Function `ZSTD_createDDict`

ZSTD_createDDict() :
 Create a digested dictionary, ready to start decompression operation without startup delay.
 dictBuffer can be released after DDict creation, as its content is copied inside DDict.

```rust
pub unsafe extern "C" fn ZSTD_createDDict(dictBuffer: *const ::core::ffi::c_void, dictSize: usize) -> *mut ZSTD_DDict;
```

### Function `ZSTD_freeDDict`

ZSTD_freeDDict() :
 Function frees memory allocated with ZSTD_createDDict()
 If a NULL pointer is passed, no operation is performed.

```rust
pub unsafe extern "C" fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict) -> usize;
```

### Function `ZSTD_decompress_usingDDict`

ZSTD_decompress_usingDDict() :
 Decompression using a digested Dictionary.
 Recommended when same dictionary is used multiple times.

```rust
pub unsafe extern "C" fn ZSTD_decompress_usingDDict(dctx: *mut ZSTD_DCtx, dst: *mut ::core::ffi::c_void, dstCapacity: usize, src: *const ::core::ffi::c_void, srcSize: usize, ddict: *const ZSTD_DDict) -> usize;
```

### Function `ZSTD_getDictID_fromDict`

ZSTD_getDictID_fromDict() : Requires v1.4.0+
 Provides the dictID stored within dictionary.
 if @return == 0, the dictionary is not conformant with Zstandard specification.
 It can still be loaded, but as a content-only dictionary.

```rust
pub unsafe extern "C" fn ZSTD_getDictID_fromDict(dict: *const ::core::ffi::c_void, dictSize: usize) -> ::core::ffi::c_uint;
```

### Function `ZSTD_getDictID_fromCDict`

ZSTD_getDictID_fromCDict() : Requires v1.5.0+
 Provides the dictID of the dictionary loaded into `cdict`.
 If @return == 0, the dictionary is not conformant to Zstandard specification, or empty.
 Non-conformant dictionaries can still be loaded, but as content-only dictionaries.

```rust
pub unsafe extern "C" fn ZSTD_getDictID_fromCDict(cdict: *const ZSTD_CDict) -> ::core::ffi::c_uint;
```

### Function `ZSTD_getDictID_fromDDict`

ZSTD_getDictID_fromDDict() : Requires v1.4.0+
 Provides the dictID of the dictionary loaded into `ddict`.
 If @return == 0, the dictionary is not conformant to Zstandard specification, or empty.
 Non-conformant dictionaries can still be loaded, but as content-only dictionaries.

```rust
pub unsafe extern "C" fn ZSTD_getDictID_fromDDict(ddict: *const ZSTD_DDict) -> ::core::ffi::c_uint;
```

### Function `ZSTD_getDictID_fromFrame`

ZSTD_getDictID_fromFrame() : Requires v1.4.0+
 Provides the dictID required to decompressed the frame stored within `src`.
 If @return == 0, the dictID could not be decoded.
 This could for one of the following reasons :
 - The frame does not require a dictionary to be decoded (most common case).
 - The frame was built with dictID intentionally removed. Whatever dictionary is necessary is a hidden piece of information.
   Note : this use case also happens when using a non-conformant dictionary.
 - `srcSize` is too small, and as a result, the frame header could not be decoded (only possible if `srcSize < ZSTD_FRAMEHEADERSIZE_MAX`).
 - This is not a Zstandard frame.
 When identifying the exact failure cause, it's possible to use ZSTD_getFrameHeader(), which will provide a more precise error code.

```rust
pub unsafe extern "C" fn ZSTD_getDictID_fromFrame(src: *const ::core::ffi::c_void, srcSize: usize) -> ::core::ffi::c_uint;
```

### Function `ZSTD_CCtx_loadDictionary`

ZSTD_CCtx_loadDictionary() : Requires v1.4.0+
 Create an internal CDict from `dict` buffer.
 Decompression will have to use same dictionary.
@result : 0, or an error code (which can be tested with ZSTD_isError()).
 Special: Loading a NULL (or 0-size) dictionary invalidates previous dictionary,
          meaning "return to no-dictionary mode".
 Note 1 : Dictionary is sticky, it will be used for all future compressed frames,
          until parameters are reset, a new dictionary is loaded, or the dictionary
          is explicitly invalidated by loading a NULL dictionary.
 Note 2 : Loading a dictionary involves building tables.
          It's also a CPU consuming operation, with non-negligible impact on latency.
          Tables are dependent on compression parameters, and for this reason,
          compression parameters can no longer be changed after loading a dictionary.
 Note 3 :`dict` content will be copied internally.
          Use experimental ZSTD_CCtx_loadDictionary_byReference() to reference content instead.
          In such a case, dictionary buffer must outlive its users.
 Note 4 : Use ZSTD_CCtx_loadDictionary_advanced()
          to precisely select how dictionary content must be interpreted.
 Note 5 : This method does not benefit from LDM (long distance mode).
          If you want to employ LDM on some large dictionary content,
          prefer employing ZSTD_CCtx_refPrefix() described below.

```rust
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary(cctx: *mut ZSTD_CCtx, dict: *const ::core::ffi::c_void, dictSize: usize) -> usize;
```

### Function `ZSTD_CCtx_refCDict`

ZSTD_CCtx_refCDict() : Requires v1.4.0+
 Reference a prepared dictionary, to be used for all future compressed frames.
 Note that compression parameters are enforced from within CDict,
 and supersede any compression parameter previously set within CCtx.
 The parameters ignored are labelled as "superseded-by-cdict" in the ZSTD_cParameter enum docs.
 The ignored parameters will be used again if the CCtx is returned to no-dictionary mode.
 The dictionary will remain valid for future compressed frames using same CCtx.
@result : 0, or an error code (which can be tested with ZSTD_isError()).
 Special : Referencing a NULL CDict means "return to no-dictionary mode".
 Note 1 : Currently, only one dictionary can be managed.
          Referencing a new dictionary effectively "discards" any previous one.
 Note 2 : CDict is just referenced, its lifetime must outlive its usage within CCtx.

```rust
pub unsafe extern "C" fn ZSTD_CCtx_refCDict(cctx: *mut ZSTD_CCtx, cdict: *const ZSTD_CDict) -> usize;
```

### Function `ZSTD_CCtx_refPrefix`

ZSTD_CCtx_refPrefix() : Requires v1.4.0+
 Reference a prefix (single-usage dictionary) for next compressed frame.
 A prefix is **only used once**. Tables are discarded at end of frame (ZSTD_e_end).
 Decompression will need same prefix to properly regenerate data.
 Compressing with a prefix is similar in outcome as performing a diff and compressing it,
 but performs much faster, especially during decompression (compression speed is tunable with compression level).
 This method is compatible with LDM (long distance mode).
@result : 0, or an error code (which can be tested with ZSTD_isError()).
 Special: Adding any prefix (including NULL) invalidates any previous prefix or dictionary
 Note 1 : Prefix buffer is referenced. It **must** outlive compression.
          Its content must remain unmodified during compression.
 Note 2 : If the intention is to diff some large src data blob with some prior version of itself,
          ensure that the window size is large enough to contain the entire source.
          See ZSTD_c_windowLog.
 Note 3 : Referencing a prefix involves building tables, which are dependent on compression parameters.
          It's a CPU consuming operation, with non-negligible impact on latency.
          If there is a need to use the same prefix multiple times, consider loadDictionary instead.
 Note 4 : By default, the prefix is interpreted as raw content (ZSTD_dct_rawContent).
          Use experimental ZSTD_CCtx_refPrefix_advanced() to alter dictionary interpretation.

```rust
pub unsafe extern "C" fn ZSTD_CCtx_refPrefix(cctx: *mut ZSTD_CCtx, prefix: *const ::core::ffi::c_void, prefixSize: usize) -> usize;
```

### Function `ZSTD_DCtx_loadDictionary`

ZSTD_DCtx_loadDictionary() : Requires v1.4.0+
 Create an internal DDict from dict buffer, to be used to decompress all future frames.
 The dictionary remains valid for all future frames, until explicitly invalidated, or
 a new dictionary is loaded.
@result : 0, or an error code (which can be tested with ZSTD_isError()).
 Special : Adding a NULL (or 0-size) dictionary invalidates any previous dictionary,
           meaning "return to no-dictionary mode".
 Note 1 : Loading a dictionary involves building tables,
          which has a non-negligible impact on CPU usage and latency.
          It's recommended to "load once, use many times", to amortize the cost
 Note 2 :`dict` content will be copied internally, so `dict` can be released after loading.
          Use ZSTD_DCtx_loadDictionary_byReference() to reference dictionary content instead.
 Note 3 : Use ZSTD_DCtx_loadDictionary_advanced() to take control of
          how dictionary content is loaded and interpreted.

```rust
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary(dctx: *mut ZSTD_DCtx, dict: *const ::core::ffi::c_void, dictSize: usize) -> usize;
```

### Function `ZSTD_DCtx_refDDict`

ZSTD_DCtx_refDDict() : Requires v1.4.0+
 Reference a prepared dictionary, to be used to decompress next frames.
 The dictionary remains active for decompression of future frames using same DCtx.

 If called with ZSTD_d_refMultipleDDicts enabled, repeated calls of this function
 will store the DDict references in a table, and the DDict used for decompression
 will be determined at decompression time, as per the dict ID in the frame.
 The memory for the table is allocated on the first call to refDDict, and can be
 freed with ZSTD_freeDCtx().

 If called with ZSTD_d_refMultipleDDicts disabled (the default), only one dictionary
 will be managed, and referencing a dictionary effectively "discards" any previous one.

@result : 0, or an error code (which can be tested with ZSTD_isError()).
 Special: referencing a NULL DDict means "return to no-dictionary mode".
 Note 2 : DDict is just referenced, its lifetime must outlive its usage from DCtx.

```rust
pub unsafe extern "C" fn ZSTD_DCtx_refDDict(dctx: *mut ZSTD_DCtx, ddict: *const ZSTD_DDict) -> usize;
```

### Function `ZSTD_DCtx_refPrefix`

ZSTD_DCtx_refPrefix() : Requires v1.4.0+
 Reference a prefix (single-usage dictionary) to decompress next frame.
 This is the reverse operation of ZSTD_CCtx_refPrefix(),
 and must use the same prefix as the one used during compression.
 Prefix is **only used once**. Reference is discarded at end of frame.
 End of frame is reached when ZSTD_decompressStream() returns 0.
@result : 0, or an error code (which can be tested with ZSTD_isError()).
 Note 1 : Adding any prefix (including NULL) invalidates any previously set prefix or dictionary
 Note 2 : Prefix buffer is referenced. It **must** outlive decompression.
          Prefix buffer must remain unmodified up to the end of frame,
          reached when ZSTD_decompressStream() returns 0.
 Note 3 : By default, the prefix is treated as raw content (ZSTD_dct_rawContent).
          Use ZSTD_CCtx_refPrefix_advanced() to alter dictMode (Experimental section)
 Note 4 : Referencing a raw content prefix has almost no cpu nor memory cost.
          A full dictionary is more costly, as it requires building tables.

```rust
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix(dctx: *mut ZSTD_DCtx, prefix: *const ::core::ffi::c_void, prefixSize: usize) -> usize;
```

### Function `ZSTD_sizeof_CCtx`

ZSTD_sizeof_*() : Requires v1.4.0+
 These functions give the _current_ memory usage of selected object.
 Note that object memory usage can evolve (increase or decrease) over time.

```rust
pub unsafe extern "C" fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> usize;
```

### Function `ZSTD_sizeof_DCtx`

```rust
pub unsafe extern "C" fn ZSTD_sizeof_DCtx(dctx: *const ZSTD_DCtx) -> usize;
```

### Function `ZSTD_sizeof_CStream`

```rust
pub unsafe extern "C" fn ZSTD_sizeof_CStream(zcs: *const ZSTD_CStream) -> usize;
```

### Function `ZSTD_sizeof_DStream`

```rust
pub unsafe extern "C" fn ZSTD_sizeof_DStream(zds: *const ZSTD_DStream) -> usize;
```

### Function `ZSTD_sizeof_CDict`

```rust
pub unsafe extern "C" fn ZSTD_sizeof_CDict(cdict: *const ZSTD_CDict) -> usize;
```

### Function `ZSTD_sizeof_DDict`

```rust
pub unsafe extern "C" fn ZSTD_sizeof_DDict(ddict: *const ZSTD_DDict) -> usize;
```

### Function `ZDICT_trainFromBuffer`

ZDICT_trainFromBuffer():
 Train a dictionary from an array of samples.
 Redirect towards ZDICT_optimizeTrainFromBuffer_fastCover() single-threaded, with d=8, steps=4,
 f=20, and accel=1.
 Samples must be stored concatenated in a single flat buffer `samplesBuffer`,
 supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.
 The resulting dictionary will be saved into `dictBuffer`.
@return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
         or an error code, which can be tested with ZDICT_isError().
 Note:  Dictionary training will fail if there are not enough samples to construct a
        dictionary, or if most of the samples are too small (< 8 bytes being the lower limit).
        If dictionary training fails, you should use zstd without a dictionary, as the dictionary
        would've been ineffective anyways. If you believe your samples would benefit from a dictionary
        please open an issue with details, and we can look into it.
 Note: ZDICT_trainFromBuffer()'s memory usage is about 6 MB.
 Tips: In general, a reasonable dictionary has a size of ~ 100 KB.
       It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.
       In general, it's recommended to provide a few thousands samples, though this can vary a lot.
       It's recommended that total size of all samples be about ~x100 times the target size of dictionary.

```rust
pub unsafe extern "C" fn ZDICT_trainFromBuffer(dictBuffer: *mut ::core::ffi::c_void, dictBufferCapacity: usize, samplesBuffer: *const ::core::ffi::c_void, samplesSizes: *const usize, nbSamples: ::core::ffi::c_uint) -> usize;
```

### Function `ZDICT_finalizeDictionary`

ZDICT_finalizeDictionary():
Given a custom content as a basis for dictionary, and a set of samples,
finalize dictionary by adding headers and statistics according to the zstd
dictionary format.

Samples must be stored concatenated in a flat buffer `samplesBuffer`,
supplied with an array of sizes `samplesSizes`, providing the size of each
sample in order. The samples are used to construct the statistics, so they
should be representative of what you will compress with this dictionary.

The compression level can be set in `parameters`. You should pass the
compression level you expect to use in production. The statistics for each
compression level differ, so tuning the dictionary for the compression level
can help quite a bit.

You can set an explicit dictionary ID in `parameters`, or allow us to pick
a random dictionary ID for you, but we can't guarantee no collisions.

The dstDictBuffer and the dictContent may overlap, and the content will be
appended to the end of the header. If the header + the content doesn't fit in
maxDictSize the beginning of the content is truncated to make room, since it
is presumed that the most profitable content is at the end of the dictionary,
since that is the cheapest to reference.

`maxDictSize` must be >= max(dictContentSize, ZDICT_DICTSIZE_MIN).

@return: size of dictionary stored into `dstDictBuffer` (<= `maxDictSize`),
         or an error code, which can be tested by ZDICT_isError().
Note: ZDICT_finalizeDictionary() will push notifications into stderr if
      instructed to, using notificationLevel>0.
NOTE: This function currently may fail in several edge cases including:
        * Not enough samples
        * Samples are uncompressible
        * Samples are all exactly the same

```rust
pub unsafe extern "C" fn ZDICT_finalizeDictionary(dstDictBuffer: *mut ::core::ffi::c_void, maxDictSize: usize, dictContent: *const ::core::ffi::c_void, dictContentSize: usize, samplesBuffer: *const ::core::ffi::c_void, samplesSizes: *const usize, nbSamples: ::core::ffi::c_uint, parameters: ZDICT_params_t) -> usize;
```

### Function `ZDICT_getDictID`

```rust
pub unsafe extern "C" fn ZDICT_getDictID(dictBuffer: *const ::core::ffi::c_void, dictSize: usize) -> ::core::ffi::c_uint;
```

### Function `ZDICT_getDictHeaderSize`

```rust
pub unsafe extern "C" fn ZDICT_getDictHeaderSize(dictBuffer: *const ::core::ffi::c_void, dictSize: usize) -> usize;
```

### Function `ZDICT_isError`

```rust
pub unsafe extern "C" fn ZDICT_isError(errorCode: usize) -> ::core::ffi::c_uint;
```

### Function `ZDICT_getErrorName`

```rust
pub unsafe extern "C" fn ZDICT_getErrorName(errorCode: usize) -> *const ::core::ffi::c_char;
```

## Constants and Statics

### Constant `ZSTD_VERSION_MAJOR`

```rust
pub const ZSTD_VERSION_MAJOR: u32 = 1;
```

### Constant `ZSTD_VERSION_MINOR`

```rust
pub const ZSTD_VERSION_MINOR: u32 = 5;
```

### Constant `ZSTD_VERSION_RELEASE`

```rust
pub const ZSTD_VERSION_RELEASE: u32 = 7;
```

### Constant `ZSTD_VERSION_NUMBER`

```rust
pub const ZSTD_VERSION_NUMBER: u32 = 10507;
```

### Constant `ZSTD_CLEVEL_DEFAULT`

```rust
pub const ZSTD_CLEVEL_DEFAULT: u32 = 3;
```

### Constant `ZSTD_MAGICNUMBER`

```rust
pub const ZSTD_MAGICNUMBER: u32 = 4247762216;
```

### Constant `ZSTD_MAGIC_DICTIONARY`

```rust
pub const ZSTD_MAGIC_DICTIONARY: u32 = 3962610743;
```

### Constant `ZSTD_MAGIC_SKIPPABLE_START`

```rust
pub const ZSTD_MAGIC_SKIPPABLE_START: u32 = 407710288;
```

### Constant `ZSTD_MAGIC_SKIPPABLE_MASK`

```rust
pub const ZSTD_MAGIC_SKIPPABLE_MASK: u32 = 4294967280;
```

### Constant `ZSTD_BLOCKSIZELOG_MAX`

```rust
pub const ZSTD_BLOCKSIZELOG_MAX: u32 = 17;
```

### Constant `ZSTD_BLOCKSIZE_MAX`

```rust
pub const ZSTD_BLOCKSIZE_MAX: u32 = 131072;
```

### Constant `ZSTD_CONTENTSIZE_UNKNOWN`

```rust
pub const ZSTD_CONTENTSIZE_UNKNOWN: i32 = -1;
```

### Constant `ZSTD_CONTENTSIZE_ERROR`

```rust
pub const ZSTD_CONTENTSIZE_ERROR: i32 = -2;
```

