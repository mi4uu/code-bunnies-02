# Crate Documentation

**Version:** 1.5.0

**Format Version:** 43

# Module `icu_normalizer`

Normalizing text into Unicode Normalization Forms.

This module is published as its own crate ([`icu_normalizer`](https://docs.rs/icu_normalizer/latest/icu_normalizer/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

# Implementation notes

The normalizer operates on a lazy iterator over Unicode scalar values (Rust `char`) internally
and iterating over guaranteed-valid UTF-8, potentially-invalid UTF-8, and potentially-invalid
UTF-16 is a step that doesn’t leak into the normalizer internals. Ill-formed byte sequences are
treated as U+FFFD.

The normalizer data layout is not based on the ICU4C design at all. Instead, the normalization
data layout is a clean-slate design optimized for the concept of fusing the NFD decomposition
into the collator. That is, the decomposing normalizer is a by-product of the collator-motivated
data layout.

Notably, the decomposition data structure is optimized for a starter decomposing to itself,
which is the most common case, and for a starter decomposing to a starter and a non-starter
on the Basic Multilingual Plane. Notably, in this case, the collator makes use of the
knowledge that the second character of such a decomposition is a non-starter. Therefore,
decomposition into two starters is handled by generic fallback path that looks the
decomposition from an array by offset and length instead of baking a BMP starter pair directly
into a trie value.

The decompositions into non-starters are hard-coded. At present in Unicode, these appear
to be special cases falling into three categories:

1. Deprecated combining marks.
2. Particular Tibetan vowel sings.
3. NFKD only: half-width kana voicing marks.

Hopefully Unicode never adds more decompositions into non-starters (other than a character
decomposing to itself), but if it does, a code update is needed instead of a mere data update.

The composing normalizer builds on the decomposing normalizer by performing the canonical
composition post-processing per spec. As an optimization, though, the composing normalizer
attempts to pass through already-normalized text consisting of starters that never combine
backwards and that map to themselves if followed by a character whose decomposition starts
with a starter that never combines backwards.

As a difference with ICU4C, the composing normalizer has only the simplest possible
passthrough (only one inversion list lookup per character in the best case) and the full
decompose-then-canonically-compose behavior, whereas ICU4C has other paths between these
extremes. The ICU4X collator doesn't make use of the FCD concept at all in order to avoid
doing the work of checking whether the FCD condition holds.

## Modules

## Module `properties`

Access to the Unicode properties or property-based operations that
are required for NFC and NFD.

Applications should generally use the full normalizers that are
provided at the top level of this crate. However, the APIs in this
module are provided for callers such as HarfBuzz that specifically
want access to the raw canonical composition operation e.g. for use in a
glyph-availability-guided custom normalizer.

```rust
pub mod properties { /* ... */ }
```

### Types

#### Struct `CanonicalComposition`

The raw canonical composition operation.

Callers should generally use `ComposingNormalizer` instead of this API.
However, this API is provided for callers such as HarfBuzz that specifically
want access to the raw canonical composition operation e.g. for use in a
glyph-availability-guided custom normalizer.

```rust
pub struct CanonicalComposition {
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
  pub fn compose(self: &Self, starter: char, second: char) -> Option<char> { /* ... */ }
  ```
  Performs canonical composition (including Hangul) on a pair of

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Constructs a new `CanonicalComposition` using compiled data.

- ```rust
  pub fn try_new_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<Self, NormalizerError> { /* ... */ }
  ```
  A version of [`Self :: new`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

- ```rust
  pub fn try_new_unstable<D>(provider: &D) -> Result<Self, NormalizerError>
where
    D: DataProvider<CanonicalCompositionsV1Marker> + ?Sized { /* ... */ }
  ```
  A version of [`Self::new`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Freeze**
- **MaybeSendSync**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
#### Enum `Decomposed`

**Attributes:**

- `#[allow(clippy::exhaustive_enums)]`

The outcome of non-recursive canonical decomposition of a character.

```rust
pub enum Decomposed {
    Default,
    Singleton(char),
    Expansion(char, char),
}
```

##### Variants

###### `Default`

The character is its own canonical decomposition.

###### `Singleton`

The character decomposes to a single different character.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

###### `Expansion`

The character decomposes to two characters.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |
| 1 | `char` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Decomposed) -> bool { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **ErasedDestructor**
- **Eq**
#### Struct `CanonicalDecomposition`

The raw (non-recursive) canonical decomposition operation.

Callers should generally use `DecomposingNormalizer` instead of this API.
However, this API is provided for callers such as HarfBuzz that specifically
want access to non-recursive canonical decomposition e.g. for use in a
glyph-availability-guided custom normalizer.

```rust
pub struct CanonicalDecomposition {
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
  pub fn decompose(self: &Self, c: char) -> Decomposed { /* ... */ }
  ```
  Performs non-recursive canonical decomposition (including for Hangul).

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Construct from compiled data.

- ```rust
  pub fn try_new_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<Self, NormalizerError> { /* ... */ }
  ```
  A version of [`Self :: new`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

- ```rust
  pub fn try_new_unstable<D>(provider: &D) -> Result<Self, NormalizerError>
where
    D: DataProvider<CanonicalDecompositionDataV1Marker> + DataProvider<CanonicalDecompositionTablesV1Marker> + DataProvider<NonRecursiveDecompositionSupplementV1Marker> + ?Sized { /* ... */ }
  ```
  A version of [`Self::new`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `CanonicalCombiningClassMap`

Lookup of the Canonical_Combining_Class Unicode property.

# Example

```
use icu::properties::CanonicalCombiningClass;
use icu::normalizer::properties::CanonicalCombiningClassMap;

let map = CanonicalCombiningClassMap::new();
assert_eq!(map.get('a'), CanonicalCombiningClass::NotReordered); // U+0061: LATIN SMALL LETTER A
assert_eq!(map.get32(0x0301), CanonicalCombiningClass::Above); // U+0301: COMBINING ACUTE ACCENT
```

```rust
pub struct CanonicalCombiningClassMap {
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
  pub fn get(self: &Self, c: char) -> CanonicalCombiningClass { /* ... */ }
  ```
  Look up the canonical combining class for a scalar value

- ```rust
  pub fn get32(self: &Self, c: u32) -> CanonicalCombiningClass { /* ... */ }
  ```
  Look up the canonical combining class for a scalar value

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Construct from compiled data.

- ```rust
  pub fn try_new_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<Self, NormalizerError> { /* ... */ }
  ```
  A version of [`Self :: new`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

- ```rust
  pub fn try_new_unstable<D>(provider: &D) -> Result<Self, NormalizerError>
where
    D: DataProvider<CanonicalDecompositionDataV1Marker> + ?Sized { /* ... */ }
  ```
  A version of [`Self::new`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

###### Trait Implementations

- **RefUnwindSafe**
- **UnwindSafe**
- **ErasedDestructor**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Freeze**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **MaybeSendSync**
## Module `provider`

**Attributes:**

- `#[allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]`

🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

Read more about data providers: [`icu_provider`]

```rust
pub mod provider { /* ... */ }
```

### Types

#### Struct `Baked`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Baked data

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. In particular, the `DataProvider` implementations are only
guaranteed to match with this version's `*_unstable` providers. Use with caution.
</div>

```rust
pub struct Baked;
```

##### Implementations

###### Trait Implementations

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: $crate::DataRequest<''_>) -> Result<$crate::DataResponse<$crate::NeverMarker<Y>>, $crate::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CanonicalCompositionsV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::NonRecursiveDecompositionSupplementV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CanonicalDecompositionDataV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CanonicalDecompositionTablesV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CompatibilityDecompositionSupplementV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::Uts46DecompositionSupplementV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Send**
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

- **Sync**
#### Struct `CanonicalDecompositionDataV1Marker`

Marker type for [`DecompositionDataV1`]: "normalizer/nfd@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct CanonicalDecompositionDataV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **KeyedDataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CanonicalDecompositionDataV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

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

- **MaybeSendSync**
- **RefUnwindSafe**
- **DataMarker**
- **Sync**
- **UnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `DecompositionDataV1`

Main data for NFD

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct DecompositionDataV1<''data> {
    pub trie: icu_collections::codepointtrie::CodePointTrie<''data, u32>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `trie` | `icu_collections::codepointtrie::CodePointTrie<''data, u32>` | Trie for NFD decomposition. |

##### Implementations

###### Trait Implementations

- **Yokeable**
  - ```rust
    fn transform(self: &''a Self) -> &''a <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn transform_owned(self: Self) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    unsafe fn make(this: <Self as >::Output) -> Self { /* ... */ }
    ```

  - ```rust
    fn transform_mut<F>(self: &''a mut Self, f: F)
where
    F: ''static + for<''b> FnOnce(&''b mut <Self as >::Output) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
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

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **StructuralPartialEq**
- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf DecompositionDataV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecompositionDataV1<''data>) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DecompositionDataV1<''data> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

#### Struct `CompatibilityDecompositionSupplementV1Marker`

Marker type for [`DecompositionSupplementV1`]: "normalizer/nfkd@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct CompatibilityDecompositionSupplementV1Marker;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CompatibilityDecompositionSupplementV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **KeyedDataMarker**
#### Struct `Uts46DecompositionSupplementV1Marker`

Marker type for [`DecompositionSupplementV1`]: "normalizer/uts46d@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct Uts46DecompositionSupplementV1Marker;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::Uts46DecompositionSupplementV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **DataMarker**
- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **KeyedDataMarker**
- **MaybeSendSync**
#### Struct `DecompositionSupplementV1`

Data that either NFKD or the decomposed form of UTS 46 needs
_in addition to_ the NFD data.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct DecompositionSupplementV1<''data> {
    pub trie: icu_collections::codepointtrie::CodePointTrie<''data, u32>,
    pub flags: u8,
    pub passthrough_cap: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `trie` | `icu_collections::codepointtrie::CodePointTrie<''data, u32>` | Trie for the decompositions that differ from NFD.<br>Getting a zero from this trie means that you need<br>to make another lookup from `DecompositionDataV1::trie`. |
| `flags` | `u8` | Flags that indicate how the set of characters whose<br>decompositions starts with a non-starter differs from<br>the set for NFD.<br><br>Bit 0: Whether half-width kana voicing marks decompose<br>       into non-starters (their full-width combining<br>       counterparts).<br>Bit 1: Whether U+0345 COMBINING GREEK YPOGEGRAMMENI<br>       decomposes into a starter (U+03B9 GREEK SMALL<br>       LETTER IOTA).<br>(Other bits unused.) |
| `passthrough_cap` | `u16` | The passthrough bounds of NFD/NFC are lowered to this<br>maximum instead. (16-bit, because cannot be higher<br>than 0x0300, which is the bound for NFC.) |

##### Implementations

###### Methods

- ```rust
  pub fn half_width_voicing_marks_become_non_starters(self: &Self) -> bool { /* ... */ }
  ```
  Whether half-width kana voicing marks decompose into non-starters

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DecompositionSupplementV1<''data> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Send**
- **ErasedDestructor**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecompositionSupplementV1<''data>) -> bool { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf DecompositionSupplementV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Freeze**
- **Yokeable**
  - ```rust
    fn transform(self: &''a Self) -> &''a <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn transform_owned(self: Self) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    unsafe fn make(this: <Self as >::Output) -> Self { /* ... */ }
    ```

  - ```rust
    fn transform_mut<F>(self: &''a mut Self, f: F)
where
    F: ''static + for<''b> FnOnce(&''b mut <Self as >::Output) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `CanonicalDecompositionTablesV1Marker`

Marker type for [`DecompositionTablesV1`]: "normalizer/nfdex@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct CanonicalDecompositionTablesV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CanonicalDecompositionTablesV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
#### Struct `CompatibilityDecompositionTablesV1Marker`

Marker type for [`DecompositionTablesV1`]: "normalizer/nfkdex@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct CompatibilityDecompositionTablesV1Marker;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Struct `DecompositionTablesV1`

The expansion tables for cases where the decomposition isn't
contained in the trie value

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct DecompositionTablesV1<''data> {
    pub scalars16: zerovec::ZeroVec<''data, u16>,
    pub scalars24: zerovec::ZeroVec<''data, char>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `scalars16` | `zerovec::ZeroVec<''data, u16>` | Decompositions that are fully within the BMP |
| `scalars24` | `zerovec::ZeroVec<''data, char>` | Decompositions with at least one character outside<br>the BMP |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecompositionTablesV1<''data>) -> bool { /* ... */ }
    ```

- **Yokeable**
  - ```rust
    fn transform(self: &''a Self) -> &''a <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn transform_owned(self: Self) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    unsafe fn make(this: <Self as >::Output) -> Self { /* ... */ }
    ```

  - ```rust
    fn transform_mut<F>(self: &''a mut Self, f: F)
where
    F: ''static + for<''b> FnOnce(&''b mut <Self as >::Output) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf DecompositionTablesV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **RefUnwindSafe**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DecompositionTablesV1<''data> { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

#### Struct `CanonicalCompositionsV1Marker`

Marker type for [`CanonicalCompositionsV1`]: "normalizer/comp@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct CanonicalCompositionsV1Marker;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CanonicalCompositionsV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Send**
- **KeyedDataMarker**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataMarker**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `CanonicalCompositionsV1`

Non-Hangul canonical compositions

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct CanonicalCompositionsV1<''data> {
    pub canonical_compositions: icu_collections::char16trie::Char16Trie<''data>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `canonical_compositions` | `icu_collections::char16trie::Char16Trie<''data>` | Trie keys are two-`char` strings with the second<br>character coming first. The value, if any, is the<br>(non-Hangul) canonical composition. |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf CanonicalCompositionsV1<''zf_inner>) -> Self { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **ErasedDestructor**
- **RefUnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CanonicalCompositionsV1<''data> { /* ... */ }
    ```

- **Yokeable**
  - ```rust
    fn transform(self: &''a Self) -> &''a <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn transform_owned(self: Self) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    unsafe fn make(this: <Self as >::Output) -> Self { /* ... */ }
    ```

  - ```rust
    fn transform_mut<F>(self: &''a mut Self, f: F)
where
    F: ''static + for<''b> FnOnce(&''b mut <Self as >::Output) { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CanonicalCompositionsV1<''data>) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `NonRecursiveDecompositionSupplementV1Marker`

Marker type for [`NonRecursiveDecompositionSupplementV1`]: "normalizer/decomp@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct NonRecursiveDecompositionSupplementV1Marker;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
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

- **Freeze**
- **MaybeSendSync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::normalizer::provider::NonRecursiveDecompositionSupplementV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **DataMarker**
- **ErasedDestructor**
- **KeyedDataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `NonRecursiveDecompositionSupplementV1`

Non-recursive canonical decompositions that differ from
`DecompositionDataV1`.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct NonRecursiveDecompositionSupplementV1<''data> {
    pub trie: icu_collections::codepointtrie::CodePointTrie<''data, u32>,
    pub scalars24: zerovec::ZeroVec<''data, char>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `trie` | `icu_collections::codepointtrie::CodePointTrie<''data, u32>` | Trie for the supplementary non-recursive decompositions |
| `scalars24` | `zerovec::ZeroVec<''data, char>` | Decompositions with at least one character outside<br>the BMP |

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NonRecursiveDecompositionSupplementV1<''data> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **Yokeable**
  - ```rust
    fn transform(self: &''a Self) -> &''a <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn transform_owned(self: Self) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    unsafe fn make(this: <Self as >::Output) -> Self { /* ... */ }
    ```

  - ```rust
    fn transform_mut<F>(self: &''a mut Self, f: F)
where
    F: ''static + for<''b> FnOnce(&''b mut <Self as >::Output) { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf NonRecursiveDecompositionSupplementV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NonRecursiveDecompositionSupplementV1<''data>) -> bool { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `uts46`

Bundles the part of UTS 46 that makes sense to implement as a
normalization.

This is meant to be used as a building block of an UTS 46
implementation, such as the `idna` crate.

```rust
pub mod uts46 { /* ... */ }
```

### Types

#### Struct `Uts46Mapper`

A mapper that knows how to performs the subsets of UTS 46 processing
documented on the methods.

```rust
pub struct Uts46Mapper {
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
  pub const fn new() -> Self { /* ... */ }
  ```
  Construct with compiled data.

- ```rust
  pub fn try_new<D>(provider: &D) -> Result<Self, NormalizerError>
where
    D: DataProvider<CanonicalDecompositionDataV1Marker> + DataProvider<Uts46DecompositionSupplementV1Marker> + DataProvider<CanonicalDecompositionTablesV1Marker> + DataProvider<CompatibilityDecompositionTablesV1Marker> + DataProvider<CanonicalCompositionsV1Marker> + ?Sized { /* ... */ }
  ```
  Construct with provider.

- ```rust
  pub fn map_normalize<''delegate, I: Iterator<Item = char> + ''delegate>(self: &''delegate Self, iter: I) -> impl Iterator<Item = char> + ''delegate { /* ... */ }
  ```
  Returns an iterator adaptor that turns an `Iterator` over `char`

- ```rust
  pub fn normalize_validate<''delegate, I: Iterator<Item = char> + ''delegate>(self: &''delegate Self, iter: I) -> impl Iterator<Item = char> + ''delegate { /* ... */ }
  ```
  Returns an iterator adaptor that turns an `Iterator` over `char`

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
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
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
## Types

### Struct `Decomposition`

An iterator adaptor that turns an `Iterator` over `char` into
a lazily-decomposed `char` sequence.

```rust
pub struct Decomposition<''data, I> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **ErasedDestructor**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<char> { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `Composition`

An iterator adaptor that turns an `Iterator` over `char` into
a lazily-decomposed and then canonically composed `char` sequence.

```rust
pub struct Composition<''data, I> {
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
  pub fn compose(self: &Self, starter: char, second: char) -> Option<char> { /* ... */ }
  ```
  Performs canonical composition (including Hangul) on a pair of

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<char> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Sync**
- **ErasedDestructor**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
### Struct `DecomposingNormalizer`

A normalizer for performing decomposing normalization.

```rust
pub struct DecomposingNormalizer {
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
  pub const fn new_nfd() -> Self { /* ... */ }
  ```
  NFD constructor using compiled data.

- ```rust
  pub fn try_new_nfd_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<Self, NormalizerError> { /* ... */ }
  ```
  A version of [`Self :: new_nfd`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

- ```rust
  pub fn try_new_nfd_unstable<D>(provider: &D) -> Result<Self, NormalizerError>
where
    D: DataProvider<CanonicalDecompositionDataV1Marker> + DataProvider<CanonicalDecompositionTablesV1Marker> + ?Sized { /* ... */ }
  ```
  A version of [`Self::new_nfd`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

- ```rust
  pub const fn new_nfkd() -> Self { /* ... */ }
  ```
  NFKD constructor using compiled data.

- ```rust
  pub fn try_new_nfkd_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<Self, NormalizerError> { /* ... */ }
  ```
  A version of [`Self :: new_nfkd`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

- ```rust
  pub fn try_new_nfkd_unstable<D>(provider: &D) -> Result<Self, NormalizerError>
where
    D: DataProvider<CanonicalDecompositionDataV1Marker> + DataProvider<CompatibilityDecompositionSupplementV1Marker> + DataProvider<CanonicalDecompositionTablesV1Marker> + DataProvider<CompatibilityDecompositionTablesV1Marker> + ?Sized { /* ... */ }
  ```
  A version of [`Self::new_nfkd`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

- ```rust
  pub fn normalize_iter<I: Iterator<Item = char>>(self: &Self, iter: I) -> Decomposition<''_, I> { /* ... */ }
  ```
  Wraps a delegate iterator into a decomposing iterator

- ```rust
  pub fn normalize(self: &Self, text: &str) -> String { /* ... */ }
  ```
  Normalize a string slice into a `String`.

- ```rust
  pub fn is_normalized(self: &Self, text: &str) -> bool { /* ... */ }
  ```
  Check whether a string slice is normalized.

- ```rust
  pub fn normalize_utf16(self: &Self, text: &[u16]) -> Vec<u16> { /* ... */ }
  ```
  Normalize a slice of potentially-invalid UTF-16 into a `Vec`.

- ```rust
  pub fn is_normalized_utf16(self: &Self, text: &[u16]) -> bool { /* ... */ }
  ```
  Checks whether a slice of potentially-invalid UTF-16 is normalized.

- ```rust
  pub fn normalize_utf8(self: &Self, text: &[u8]) -> String { /* ... */ }
  ```
  Normalize a slice of potentially-invalid UTF-8 into a `String`.

- ```rust
  pub fn is_normalized_utf8(self: &Self, text: &[u8]) -> bool { /* ... */ }
  ```
  Check if a slice of potentially-invalid UTF-8 is normalized.

- ```rust
  pub fn normalize_to<W: core::fmt::Write + ?Sized>(self: &Self, text: &str, sink: &mut W) -> core::fmt::Result { /* ... */ }
  ```
  Normalize a string slice into a `Write` sink.

- ```rust
  pub fn normalize_utf8_to<W: core::fmt::Write + ?Sized>(self: &Self, text: &[u8], sink: &mut W) -> core::fmt::Result { /* ... */ }
  ```
  Normalize a slice of potentially-invalid UTF-8 into a `Write` sink.

- ```rust
  pub fn normalize_utf16_to<W: write16::Write16 + ?Sized>(self: &Self, text: &[u16], sink: &mut W) -> core::fmt::Result { /* ... */ }
  ```
  Normalize a slice of potentially-invalid UTF-16 into a `Write16` sink.

##### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
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

- **RefUnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
### Struct `ComposingNormalizer`

A normalizer for performing composing normalization.

```rust
pub struct ComposingNormalizer {
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
  pub const fn new_nfc() -> Self { /* ... */ }
  ```
  NFC constructor using compiled data.

- ```rust
  pub fn try_new_nfc_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<Self, NormalizerError> { /* ... */ }
  ```
  A version of [`Self :: new_nfc`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

- ```rust
  pub fn try_new_nfc_unstable<D>(provider: &D) -> Result<Self, NormalizerError>
where
    D: DataProvider<CanonicalDecompositionDataV1Marker> + DataProvider<CanonicalDecompositionTablesV1Marker> + DataProvider<CanonicalCompositionsV1Marker> + ?Sized { /* ... */ }
  ```
  A version of [`Self::new_nfc`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

- ```rust
  pub const fn new_nfkc() -> Self { /* ... */ }
  ```
  NFKC constructor using compiled data.

- ```rust
  pub fn try_new_nfkc_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<Self, NormalizerError> { /* ... */ }
  ```
  A version of [`Self :: new_nfkc`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

- ```rust
  pub fn try_new_nfkc_unstable<D>(provider: &D) -> Result<Self, NormalizerError>
where
    D: DataProvider<CanonicalDecompositionDataV1Marker> + DataProvider<CompatibilityDecompositionSupplementV1Marker> + DataProvider<CanonicalDecompositionTablesV1Marker> + DataProvider<CompatibilityDecompositionTablesV1Marker> + DataProvider<CanonicalCompositionsV1Marker> + ?Sized { /* ... */ }
  ```
  A version of [`Self::new_nfkc`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

- ```rust
  pub fn normalize_iter<I: Iterator<Item = char>>(self: &Self, iter: I) -> Composition<''_, I> { /* ... */ }
  ```
  Wraps a delegate iterator into a composing iterator

- ```rust
  pub fn normalize(self: &Self, text: &str) -> String { /* ... */ }
  ```
  Normalize a string slice into a `String`.

- ```rust
  pub fn is_normalized(self: &Self, text: &str) -> bool { /* ... */ }
  ```
  Check whether a string slice is normalized.

- ```rust
  pub fn normalize_utf16(self: &Self, text: &[u16]) -> Vec<u16> { /* ... */ }
  ```
  Normalize a slice of potentially-invalid UTF-16 into a `Vec`.

- ```rust
  pub fn is_normalized_utf16(self: &Self, text: &[u16]) -> bool { /* ... */ }
  ```
  Checks whether a slice of potentially-invalid UTF-16 is normalized.

- ```rust
  pub fn normalize_utf8(self: &Self, text: &[u8]) -> String { /* ... */ }
  ```
  Normalize a slice of potentially-invalid UTF-8 into a `String`.

- ```rust
  pub fn is_normalized_utf8(self: &Self, text: &[u8]) -> bool { /* ... */ }
  ```
  Check if a slice of potentially-invalid UTF-8 is normalized.

- ```rust
  pub fn normalize_to<W: core::fmt::Write + ?Sized>(self: &Self, text: &str, sink: &mut W) -> core::fmt::Result { /* ... */ }
  ```
  Normalize a string slice into a `Write` sink.

- ```rust
  pub fn normalize_utf8_to<W: core::fmt::Write + ?Sized>(self: &Self, text: &[u8], sink: &mut W) -> core::fmt::Result { /* ... */ }
  ```
  Normalize a slice of potentially-invalid UTF-8 into a `Write` sink.

- ```rust
  pub fn normalize_utf16_to<W: write16::Write16 + ?Sized>(self: &Self, text: &[u16], sink: &mut W) -> core::fmt::Result { /* ... */ }
  ```
  Normalize a slice of potentially-invalid UTF-16 into a `Write16` sink.

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **MaybeSendSync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
## Re-exports

### Re-export `NormalizerError`

```rust
pub use crate::error::NormalizerError;
```

### Re-export `NormalizerError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use NormalizerError as Error;
```

