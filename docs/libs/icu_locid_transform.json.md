# Crate Documentation

**Version:** 1.5.0

**Format Version:** 43

# Module `icu_locid_transform`

Canonicalization of locale identifiers based on [`CLDR`] data.

This module is published as its own crate ([`icu_locid_transform`](https://docs.rs/icu_locid_transform/latest/icu_locid_transform/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

It currently supports locale canonicalization based upon the canonicalization
algorithm from [`UTS #35: Unicode LDML 3. LocaleId Canonicalization`],
as well as the minimize and maximize likely subtags algorithms
as described in [`UTS #35: Unicode LDML 3. Likely Subtags`].

The maximize method potentially updates a passed in locale in place
depending up the results of running the 'Add Likely Subtags' algorithm
from [`UTS #35: Unicode LDML 3. Likely Subtags`].

This minimize method returns a new Locale that is the result of running the
'Remove Likely Subtags' algorithm from [`UTS #35: Unicode LDML 3. Likely Subtags`].

# Examples

```
use icu::locid::Locale;
use icu::locid_transform::{LocaleCanonicalizer, TransformResult};

let lc = LocaleCanonicalizer::new();

let mut locale: Locale = "ja-Latn-fonipa-hepburn-heploc"
    .parse()
    .expect("parse failed");
assert_eq!(lc.canonicalize(&mut locale), TransformResult::Modified);
assert_eq!(locale, "ja-Latn-alalc97-fonipa".parse::<Locale>().unwrap());
```

```
use icu::locid::locale;
use icu::locid_transform::{LocaleExpander, TransformResult};

let lc = LocaleExpander::new();

let mut locale = locale!("zh-CN");
assert_eq!(lc.maximize(&mut locale), TransformResult::Modified);
assert_eq!(locale, locale!("zh-Hans-CN"));

let mut locale = locale!("zh-Hant-TW");
assert_eq!(lc.maximize(&mut locale), TransformResult::Unmodified);
assert_eq!(locale, locale!("zh-Hant-TW"));
```

```
use icu::locid::locale;
use icu::locid_transform::{LocaleExpander, TransformResult};
use writeable::assert_writeable_eq;

let lc = LocaleExpander::new();

let mut locale = locale!("zh-Hans-CN");
assert_eq!(lc.minimize(&mut locale), TransformResult::Modified);
assert_eq!(locale, locale!("zh"));

let mut locale = locale!("zh");
assert_eq!(lc.minimize(&mut locale), TransformResult::Unmodified);
assert_eq!(locale, locale!("zh"));
```

[`ICU4X`]: ../icu/index.html
[`CLDR`]: http://cldr.unicode.org/
[`UTS #35: Unicode LDML 3. Likely Subtags`]: https://www.unicode.org/reports/tr35/#Likely_Subtags.
[`UTS #35: Unicode LDML 3. LocaleId Canonicalization`]: http://unicode.org/reports/tr35/#LocaleId_Canonicalization,

## Modules

## Module `fallback`

Tools for locale fallback, enabling arbitrary input locales to be mapped into the nearest
locale with data.

```rust
pub mod fallback { /* ... */ }
```

### Types

#### Struct `LocaleFallbackerBorrowed`

Borrowed version of [`LocaleFallbacker`].

```rust
pub struct LocaleFallbackerBorrowed<''a> {
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
  pub const fn for_config(self: Self, config: LocaleFallbackConfig) -> LocaleFallbackerWithConfig<''a> { /* ... */ }
  ```
  Associates a configuration with this fallbacker.

- ```rust
  pub const fn static_to_owned(self: Self) -> LocaleFallbacker { /* ... */ }
  ```
  Cheaply converts a [`LocaleFallbackerBorrowed<'static>`] into a [`LocaleFallbacker`].

###### Trait Implementations

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LocaleFallbackerBorrowed<''a>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> LocaleFallbackerBorrowed<''a> { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `LocaleFallbackerWithConfig`

A [`LocaleFallbackerBorrowed`] with an associated [`LocaleFallbackConfig`].

```rust
pub struct LocaleFallbackerWithConfig<''a> {
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
  pub fn fallback_for(self: &Self, locale: DataLocale) -> LocaleFallbackIterator<''a, ''static> { /* ... */ }
  ```
  Creates an iterator based on a [`DataLocale`].

###### Trait Implementations

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LocaleFallbackerWithConfig<''a>) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LocaleFallbackerWithConfig<''a> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **UnwindSafe**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `LocaleFallbackIterator`

Iteration type for locale fallback operations.

Because the `Iterator` trait does not allow items to borrow from the iterator, this class does
not implement that trait. Instead, use `.step()` and `.get()`.

```rust
pub struct LocaleFallbackIterator<''a, ''b> {
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
  pub fn get(self: &Self) -> &DataLocale { /* ... */ }
  ```
  Borrows the current [`DataLocale`] under fallback.

- ```rust
  pub fn take(self: Self) -> DataLocale { /* ... */ }
  ```
  Takes the current [`DataLocale`] under fallback.

- ```rust
  pub fn step(self: &mut Self) -> &mut Self { /* ... */ }
  ```
  Performs one step of the locale fallback algorithm.

###### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: $crate::DataRequest<''_>) -> Result<$crate::DataResponse<$crate::NeverMarker<Y>>, $crate::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::LocaleFallbackLikelySubtagsV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::LocaleFallbackParentsV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::CollationFallbackSupplementV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::AliasesV2Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::LikelySubtagsExtendedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::LikelySubtagsForLanguageV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::LikelySubtagsForScriptRegionV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::locid_transform::provider::ScriptDirectionV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
#### Struct `StrStrPair`

A pair of strings with a EncodeAsVarULE implementation.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct StrStrPair<''a>(pub alloc::borrow::Cow<''a, str>, pub alloc::borrow::Cow<''a, str>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::borrow::Cow<''a, str>` |  |
| 1 | `alloc::borrow::Cow<''a, str>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **EncodeAsVarULE**
  - ```rust
    fn encode_var_ule_as_slices<R, /* synthetic */ impl FnOnce(&[&[u8]]) -> R: FnOnce(&[&[u8]]) -> R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_write(self: &Self, dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_as_slices<R, /* synthetic */ impl FnOnce(&[&[u8]]) -> R: FnOnce(&[&[u8]]) -> R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_write(self: &Self, dst: &mut [u8]) { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(other: &''a StrStrPairVarULE) -> Self { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &StrStrPair<''a>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StrStrPair<''a> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &StrStrPair<''a>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StrStrPair<''a>) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
#### Struct `StrStrPairVarULE`

**Attributes:**

- `#[allow(missing_docs)]`
- `#[repr(C, packed(1))]`

[`VarULE`](zerovec::ule::VarULE) type for [`StrStrPair`]. See [`StrStrPair`] for documentation.

```rust
pub struct StrStrPairVarULE(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn field_0<''a>(self: &''a Self) -> &''a str { /* ... */ }
  ```
  Access the VarULE type behind tuple struct field #0

- ```rust
  pub fn field_1<''a>(self: &''a Self) -> &''a str { /* ... */ }
  ```
  Access the VarULE type behind tuple struct field #1

###### Trait Implementations

- **ZeroMapKV**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **EncodeAsVarULE**
  - ```rust
    fn encode_var_ule_as_slices<R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_as_slices<R, /* synthetic */ impl FnOnce(&[&[u8]]) -> R: FnOnce(&[&[u8]]) -> R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_write(self: &Self, dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_as_slices<R, /* synthetic */ impl FnOnce(&[&[u8]]) -> R: FnOnce(&[&[u8]]) -> R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_write(self: &Self, dst: &mut [u8]) { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(other: &''a StrStrPairVarULE) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sized**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
    ```

- **Eq**
- **Send**
- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **VarULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> { /* ... */ }
    ```

  - ```rust
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `LanguageStrStrPair`

A triplet of strings with a EncodeAsVarULE implementation.

```rust
pub struct LanguageStrStrPair<''a>(pub icu_locid::subtags::Language, pub alloc::borrow::Cow<''a, str>, pub alloc::borrow::Cow<''a, str>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `icu_locid::subtags::Language` |  |
| 1 | `alloc::borrow::Cow<''a, str>` |  |
| 2 | `alloc::borrow::Cow<''a, str>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **ZeroFrom**
  - ```rust
    fn zero_from(other: &''a LanguageStrStrPairVarULE) -> Self { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LanguageStrStrPair<''a>) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &LanguageStrStrPair<''a>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **EncodeAsVarULE**
  - ```rust
    fn encode_var_ule_as_slices<R, /* synthetic */ impl FnOnce(&[&[u8]]) -> R: FnOnce(&[&[u8]]) -> R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_write(self: &Self, dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_as_slices<R, /* synthetic */ impl FnOnce(&[&[u8]]) -> R: FnOnce(&[&[u8]]) -> R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_write(self: &Self, dst: &mut [u8]) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &LanguageStrStrPair<''a>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LanguageStrStrPair<''a> { /* ... */ }
    ```

- **ErasedDestructor**
- **Eq**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
#### Struct `LanguageStrStrPairVarULE`

**Attributes:**

- `#[allow(missing_docs)]`
- `#[repr(C, packed(1))]`

[`VarULE`](zerovec::ule::VarULE) type for [`LanguageStrStrPair`]. See [`LanguageStrStrPair`] for documentation.

```rust
pub struct LanguageStrStrPairVarULE(pub <icu_locid::subtags::Language as zerovec::ule::AsULE>::ULE, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<icu_locid::subtags::Language as zerovec::ule::AsULE>::ULE` |  |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn field_1<''a>(self: &''a Self) -> &''a str { /* ... */ }
  ```
  Access the VarULE type behind tuple struct field #1

- ```rust
  pub fn field_2<''a>(self: &''a Self) -> &''a str { /* ... */ }
  ```
  Access the VarULE type behind tuple struct field #2

###### Trait Implementations

- **ZeroMapKV**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **VarULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> { /* ... */ }
    ```

  - ```rust
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(other: &''a LanguageStrStrPairVarULE) -> Self { /* ... */ }
    ```

- **Sized**
- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **EncodeAsVarULE**
  - ```rust
    fn encode_var_ule_as_slices<R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_as_slices<R, /* synthetic */ impl FnOnce(&[&[u8]]) -> R: FnOnce(&[&[u8]]) -> R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_write(self: &Self, dst: &mut [u8]) { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_as_slices<R, /* synthetic */ impl FnOnce(&[&[u8]]) -> R: FnOnce(&[&[u8]]) -> R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn encode_var_ule_write(self: &Self, dst: &mut [u8]) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Eq**
### Re-exports

#### Re-export `canonicalizer::*`

```rust
pub use canonicalizer::*;
```

#### Re-export `directionality::*`

```rust
pub use directionality::*;
```

#### Re-export `expander::*`

```rust
pub use expander::*;
```

#### Re-export `fallback::*`

```rust
pub use fallback::*;
```

## Types

### Enum `TransformResult`

**Attributes:**

- `#[allow(clippy::exhaustive_enums)]`

Used to track the result of a transformation operation that potentially modifies its argument in place.

```rust
pub enum TransformResult {
    Modified,
    Unmodified,
}
```

#### Variants

##### `Modified`

The canonicalization operation modified the locale.

##### `Unmodified`

The canonicalization operation did not modify the locale.

#### Implementations

##### Trait Implementations

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TransformResult) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
## Re-exports

### Re-export `LocaleCanonicalizer`

```rust
pub use canonicalizer::LocaleCanonicalizer;
```

### Re-export `Direction`

```rust
pub use directionality::Direction;
```

### Re-export `LocaleDirectionality`

```rust
pub use directionality::LocaleDirectionality;
```

### Re-export `LocaleTransformError`

```rust
pub use error::LocaleTransformError;
```

### Re-export `LocaleExpander`

```rust
pub use expander::LocaleExpander;
```

### Re-export `LocaleTransformError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use LocaleTransformError as Error;
```

