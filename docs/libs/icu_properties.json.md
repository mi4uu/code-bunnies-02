# Crate Documentation

**Version:** 1.5.1

**Format Version:** 43

# Module `icu_properties`

Definitions of [Unicode Properties] and APIs for
retrieving property data in an appropriate data structure.

This module is published as its own crate ([`icu_properties`](https://docs.rs/icu_properties/latest/icu_properties/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

APIs that return a [`CodePointSetData`] exist for binary properties and certain enumerated
properties. See the [`sets`] module for more details.

APIs that return a [`CodePointMapData`] exist for certain enumerated properties. See the
[`maps`] module for more details.

# Examples

## Property data as `CodePointSetData`s

```
use icu::properties::{maps, sets, GeneralCategory};

// A binary property as a `CodePointSetData`

assert!(sets::emoji().contains('🎃')); // U+1F383 JACK-O-LANTERN
assert!(!sets::emoji().contains('木')); // U+6728

// An individual enumerated property value as a `CodePointSetData`

let line_sep_data = maps::general_category()
    .get_set_for_value(GeneralCategory::LineSeparator);
let line_sep = line_sep_data.as_borrowed();

assert!(line_sep.contains32(0x2028));
assert!(!line_sep.contains32(0x2029));
```

## Property data as `CodePointMapData`s

```
use icu::properties::{maps, Script};

assert_eq!(maps::script().get('🎃'), Script::Common); // U+1F383 JACK-O-LANTERN
assert_eq!(maps::script().get('木'), Script::Han); // U+6728
```

[`ICU4X`]: ../icu/index.html
[Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
[`CodePointSetData`]: crate::sets::CodePointSetData
[`CodePointMapData`]: crate::maps::CodePointMapData
[`sets`]: crate::sets

## Modules

## Module `maps`

The functions in this module return a [`CodePointMapData`] representing, for
each code point in the entire range of code points, the property values
for a particular Unicode property.

The descriptions of most properties are taken from [`TR44`], the documentation for the
Unicode Character Database.

[`TR44`]: https://www.unicode.org/reports/tr44

```rust
pub mod maps { /* ... */ }
```

### Types

#### Struct `CodePointMapData`

A wrapper around code point map data. It is returned by APIs that return Unicode
property data in a map-like form, ex: enumerated property value data keyed
by code point. Access its data via the borrowed version,
[`CodePointMapDataBorrowed`].

```rust
pub struct CodePointMapData<T: TrieValue> {
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
  pub fn as_borrowed(self: &Self) -> CodePointMapDataBorrowed<''_, T> { /* ... */ }
  ```
  Construct a borrowed version of this type that can be queried.

- ```rust
  pub fn try_into_converted<P>(self: Self) -> Result<CodePointMapData<P>, ZeroVecError>
where
    P: TrieValue { /* ... */ }
  ```
  Convert this map to a map around another type

- ```rust
  pub fn from_data<M>(data: DataPayload<M>) -> Self
where
    M: DataMarker<Yokeable = PropertyCodePointMapV1<''static, T>> { /* ... */ }
  ```
  Construct a new one from loaded data

- ```rust
  pub fn from_code_point_trie(trie: CodePointTrie<''static, T>) -> Self { /* ... */ }
  ```
  Construct a new one an owned [`CodePointTrie`]

- ```rust
  pub fn as_code_point_trie(self: &Self) -> Option<&CodePointTrie<''_, T>> { /* ... */ }
  ```
  Convert this type to a [`CodePointTrie`] as a borrowed value.

- ```rust
  pub fn to_code_point_trie(self: &Self) -> CodePointTrie<''_, T> { /* ... */ }
  ```
  Convert this type to a [`CodePointTrie`], borrowing if possible,

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CodePointMapData<T> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **Unpin**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Send**
#### Struct `CodePointMapDataBorrowed`

A borrowed wrapper around code point set data, returned by
[`CodePointSetData::as_borrowed()`]. More efficient to query.

```rust
pub struct CodePointMapDataBorrowed<''a, T: TrieValue> {
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
  pub fn get(self: Self, ch: char) -> T { /* ... */ }
  ```
  Get the value this map has associated with code point `ch`

- ```rust
  pub fn get32(self: Self, ch: u32) -> T { /* ... */ }
  ```
  Get the value this map has associated with code point `ch`

- ```rust
  pub fn get_set_for_value(self: Self, value: T) -> CodePointSetData { /* ... */ }
  ```
  Get a [`CodePointSetData`] for all elements corresponding to a particular value

- ```rust
  pub fn iter_ranges(self: Self) -> impl Iterator<Item = CodePointMapRange<T>> + ''a { /* ... */ }
  ```
  Yields an [`Iterator`] returning ranges of consecutive code points that

- ```rust
  pub fn iter_ranges_for_value(self: Self, val: T) -> impl Iterator<Item = RangeInclusive<u32>> + ''a { /* ... */ }
  ```
  Yields an [`Iterator`] returning ranges of consecutive code points that

- ```rust
  pub fn iter_ranges_for_value_complemented(self: Self, val: T) -> impl Iterator<Item = RangeInclusive<u32>> + ''a { /* ... */ }
  ```
  Yields an [`Iterator`] returning ranges of consecutive code points that

- ```rust
  pub const fn static_to_owned(self: Self) -> CodePointMapData<T> { /* ... */ }
  ```
  Cheaply converts a [`CodePointMapDataBorrowed<'static>`] into a [`CodePointMapData`].

- ```rust
  pub fn iter_ranges_for_group(self: Self, group: crate::GeneralCategoryGroup) -> impl Iterator<Item = RangeInclusive<u32>> + ''a { /* ... */ }
  ```
  Yields an [`Iterator`] returning ranges of consecutive code points that

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **Freeze**
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

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CodePointMapDataBorrowed<''a, T> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
### Functions

#### Function `load_general_category`

A version of [`general_category()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_general_category</* synthetic */ impl DataProvider<GeneralCategoryV1Marker> + ?Sized: DataProvider<GeneralCategoryV1Marker> + ?Sized>(provider: &impl DataProvider<GeneralCategoryV1Marker> + ?Sized) -> Result<CodePointMapData<crate::GeneralCategory>, PropertiesError> { /* ... */ }
```

#### Function `general_category`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the General_Category Unicode enumerated property. See [`GeneralCategory`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::{maps, GeneralCategory};

assert_eq!(maps::general_category().get('木'), GeneralCategory::OtherLetter);  // U+6728
assert_eq!(maps::general_category().get('🎃'), GeneralCategory::OtherSymbol);  // U+1F383 JACK-O-LANTERN
```

```rust
pub const fn general_category() -> CodePointMapDataBorrowed<''static, crate::GeneralCategory> { /* ... */ }
```

#### Function `load_bidi_class`

A version of [`bidi_class()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_bidi_class</* synthetic */ impl DataProvider<BidiClassV1Marker> + ?Sized: DataProvider<BidiClassV1Marker> + ?Sized>(provider: &impl DataProvider<BidiClassV1Marker> + ?Sized) -> Result<CodePointMapData<crate::BidiClass>, PropertiesError> { /* ... */ }
```

#### Function `bidi_class`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the Bidi_Class Unicode enumerated property. See [`BidiClass`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::{maps, BidiClass};

assert_eq!(maps::bidi_class().get('y'), BidiClass::LeftToRight);  // U+0079
assert_eq!(maps::bidi_class().get('ع'), BidiClass::ArabicLetter);  // U+0639
```

```rust
pub const fn bidi_class() -> CodePointMapDataBorrowed<''static, crate::BidiClass> { /* ... */ }
```

#### Function `load_script`

A version of [`script()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_script</* synthetic */ impl DataProvider<ScriptV1Marker> + ?Sized: DataProvider<ScriptV1Marker> + ?Sized>(provider: &impl DataProvider<ScriptV1Marker> + ?Sized) -> Result<CodePointMapData<crate::Script>, PropertiesError> { /* ... */ }
```

#### Function `script`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the Script Unicode enumerated property. See [`Script`].

**Note:** Some code points are associated with multiple scripts. If you are trying to
determine whether a code point belongs to a certain script, you should use
[`load_script_with_extensions_unstable`] and [`ScriptWithExtensionsBorrowed::has_script`]
instead of this function.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::{maps, Script};

assert_eq!(maps::script().get('木'), Script::Han);  // U+6728
assert_eq!(maps::script().get('🎃'), Script::Common);  // U+1F383 JACK-O-LANTERN
```
[`load_script_with_extensions_unstable`]: crate::script::load_script_with_extensions_unstable
[`ScriptWithExtensionsBorrowed::has_script`]: crate::script::ScriptWithExtensionsBorrowed::has_script

```rust
pub const fn script() -> CodePointMapDataBorrowed<''static, crate::Script> { /* ... */ }
```

#### Function `load_hangul_syllable_type`

A version of [`hangul_syllable_type()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_hangul_syllable_type</* synthetic */ impl DataProvider<HangulSyllableTypeV1Marker> + ?Sized: DataProvider<HangulSyllableTypeV1Marker> + ?Sized>(provider: &impl DataProvider<HangulSyllableTypeV1Marker> + ?Sized) -> Result<CodePointMapData<crate::HangulSyllableType>, PropertiesError> { /* ... */ }
```

#### Function `hangul_syllable_type`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Returns a [`CodePointMapDataBorrowed`] for the Hangul_Syllable_Type
Unicode enumerated property. See [`HangulSyllableType`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::{maps, HangulSyllableType};

assert_eq!(maps::hangul_syllable_type().get('ᄀ'), HangulSyllableType::LeadingJamo);  // U+1100
assert_eq!(maps::hangul_syllable_type().get('가'), HangulSyllableType::LeadingVowelSyllable);  // U+AC00
```

```rust
pub const fn hangul_syllable_type() -> CodePointMapDataBorrowed<''static, crate::HangulSyllableType> { /* ... */ }
```

#### Function `load_east_asian_width`

A version of [`east_asian_width()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_east_asian_width</* synthetic */ impl DataProvider<EastAsianWidthV1Marker> + ?Sized: DataProvider<EastAsianWidthV1Marker> + ?Sized>(provider: &impl DataProvider<EastAsianWidthV1Marker> + ?Sized) -> Result<CodePointMapData<crate::EastAsianWidth>, PropertiesError> { /* ... */ }
```

#### Function `east_asian_width`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the East_Asian_Width Unicode enumerated
property. See [`EastAsianWidth`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::{maps, EastAsianWidth};

assert_eq!(maps::east_asian_width().get('ｱ'), EastAsianWidth::Halfwidth); // U+FF71: Halfwidth Katakana Letter A
assert_eq!(maps::east_asian_width().get('ア'), EastAsianWidth::Wide); //U+30A2: Katakana Letter A
```

```rust
pub const fn east_asian_width() -> CodePointMapDataBorrowed<''static, crate::EastAsianWidth> { /* ... */ }
```

#### Function `load_line_break`

A version of [`line_break()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_line_break</* synthetic */ impl DataProvider<LineBreakV1Marker> + ?Sized: DataProvider<LineBreakV1Marker> + ?Sized>(provider: &impl DataProvider<LineBreakV1Marker> + ?Sized) -> Result<CodePointMapData<crate::LineBreak>, PropertiesError> { /* ... */ }
```

#### Function `line_break`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the Line_Break Unicode enumerated
property. See [`LineBreak`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

**Note:** Use `icu::segmenter` for an all-in-one break iterator implementation.

# Example

```
use icu::properties::{maps, LineBreak};

assert_eq!(maps::line_break().get(')'), LineBreak::CloseParenthesis); // U+0029: Right Parenthesis
assert_eq!(maps::line_break().get('ぁ'), LineBreak::ConditionalJapaneseStarter); //U+3041: Hiragana Letter Small A
```

```rust
pub const fn line_break() -> CodePointMapDataBorrowed<''static, crate::LineBreak> { /* ... */ }
```

#### Function `load_grapheme_cluster_break`

A version of [`grapheme_cluster_break()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_grapheme_cluster_break</* synthetic */ impl DataProvider<GraphemeClusterBreakV1Marker> + ?Sized: DataProvider<GraphemeClusterBreakV1Marker> + ?Sized>(provider: &impl DataProvider<GraphemeClusterBreakV1Marker> + ?Sized) -> Result<CodePointMapData<crate::GraphemeClusterBreak>, PropertiesError> { /* ... */ }
```

#### Function `grapheme_cluster_break`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the Grapheme_Cluster_Break Unicode enumerated
property. See [`GraphemeClusterBreak`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

**Note:** Use `icu::segmenter` for an all-in-one break iterator implementation.

# Example

```
use icu::properties::{maps, GraphemeClusterBreak};

assert_eq!(maps::grapheme_cluster_break().get('🇦'), GraphemeClusterBreak::RegionalIndicator); // U+1F1E6: Regional Indicator Symbol Letter A
assert_eq!(maps::grapheme_cluster_break().get('ำ'), GraphemeClusterBreak::SpacingMark); //U+0E33: Thai Character Sara Am
```

```rust
pub const fn grapheme_cluster_break() -> CodePointMapDataBorrowed<''static, crate::GraphemeClusterBreak> { /* ... */ }
```

#### Function `load_word_break`

A version of [`word_break()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_word_break</* synthetic */ impl DataProvider<WordBreakV1Marker> + ?Sized: DataProvider<WordBreakV1Marker> + ?Sized>(provider: &impl DataProvider<WordBreakV1Marker> + ?Sized) -> Result<CodePointMapData<crate::WordBreak>, PropertiesError> { /* ... */ }
```

#### Function `word_break`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the Word_Break Unicode enumerated
property. See [`WordBreak`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

**Note:** Use `icu::segmenter` for an all-in-one break iterator implementation.

# Example

```
use icu::properties::{maps, WordBreak};

assert_eq!(maps::word_break().get('.'), WordBreak::MidNumLet); // U+002E: Full Stop
assert_eq!(maps::word_break().get('，'), WordBreak::MidNum); // U+FF0C: Fullwidth Comma
```

```rust
pub const fn word_break() -> CodePointMapDataBorrowed<''static, crate::WordBreak> { /* ... */ }
```

#### Function `load_sentence_break`

A version of [`sentence_break()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_sentence_break</* synthetic */ impl DataProvider<SentenceBreakV1Marker> + ?Sized: DataProvider<SentenceBreakV1Marker> + ?Sized>(provider: &impl DataProvider<SentenceBreakV1Marker> + ?Sized) -> Result<CodePointMapData<crate::SentenceBreak>, PropertiesError> { /* ... */ }
```

#### Function `sentence_break`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the Sentence_Break Unicode enumerated
property. See [`SentenceBreak`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

**Note:** Use `icu::segmenter` for an all-in-one break iterator implementation.

# Example

```
use icu::properties::{maps, SentenceBreak};

assert_eq!(maps::sentence_break().get('９'), SentenceBreak::Numeric); // U+FF19: Fullwidth Digit Nine
assert_eq!(maps::sentence_break().get(','), SentenceBreak::SContinue); // U+002C: Comma
```

```rust
pub const fn sentence_break() -> CodePointMapDataBorrowed<''static, crate::SentenceBreak> { /* ... */ }
```

#### Function `load_canonical_combining_class`

A version of [`canonical_combining_class()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_canonical_combining_class</* synthetic */ impl DataProvider<CanonicalCombiningClassV1Marker> + ?Sized: DataProvider<CanonicalCombiningClassV1Marker> + ?Sized>(provider: &impl DataProvider<CanonicalCombiningClassV1Marker> + ?Sized) -> Result<CodePointMapData<crate::CanonicalCombiningClass>, PropertiesError> { /* ... */ }
```

#### Function `canonical_combining_class`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapData`] for the Canonical_Combining_Class Unicode property. See
[`CanonicalCombiningClass`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

**Note:** See `icu::normalizer::CanonicalCombiningClassMap` for the preferred API
to look up the Canonical_Combining_Class property by scalar value.

# Example

```
use icu::properties::{maps, CanonicalCombiningClass};

assert_eq!(maps::canonical_combining_class().get('a'), CanonicalCombiningClass::NotReordered); // U+0061: LATIN SMALL LETTER A
assert_eq!(maps::canonical_combining_class().get32(0x0301), CanonicalCombiningClass::Above); // U+0301: COMBINING ACUTE ACCENT
```

```rust
pub const fn canonical_combining_class() -> CodePointMapDataBorrowed<''static, crate::CanonicalCombiningClass> { /* ... */ }
```

#### Function `load_indic_syllabic_category`

A version of [`indic_syllabic_category()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_indic_syllabic_category</* synthetic */ impl DataProvider<IndicSyllabicCategoryV1Marker> + ?Sized: DataProvider<IndicSyllabicCategoryV1Marker> + ?Sized>(provider: &impl DataProvider<IndicSyllabicCategoryV1Marker> + ?Sized) -> Result<CodePointMapData<crate::IndicSyllabicCategory>, PropertiesError> { /* ... */ }
```

#### Function `indic_syllabic_category`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapData`] for the Indic_Syllabic_Category Unicode property. See
[`IndicSyllabicCategory`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::{maps, IndicSyllabicCategory};

assert_eq!(maps::indic_syllabic_category().get('a'), IndicSyllabicCategory::Other);
assert_eq!(maps::indic_syllabic_category().get32(0x0900), IndicSyllabicCategory::Bindu); // U+0900: DEVANAGARI SIGN INVERTED CANDRABINDU
```

```rust
pub const fn indic_syllabic_category() -> CodePointMapDataBorrowed<''static, crate::IndicSyllabicCategory> { /* ... */ }
```

#### Function `load_joining_type`

A version of [`joining_type()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointMapData::as_borrowed`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_joining_type</* synthetic */ impl DataProvider<JoiningTypeV1Marker> + ?Sized: DataProvider<JoiningTypeV1Marker> + ?Sized>(provider: &impl DataProvider<JoiningTypeV1Marker> + ?Sized) -> Result<CodePointMapData<crate::JoiningType>, PropertiesError> { /* ... */ }
```

#### Function `joining_type`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointMapDataBorrowed`] for the Joining_Type Unicode enumerated
property. See [`JoiningType`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::{maps, JoiningType};

assert_eq!(maps::joining_type().get('ؠ'), JoiningType::DualJoining); // U+0620: Arabic Letter Kashmiri Yeh
assert_eq!(maps::joining_type().get('𐫍'), JoiningType::LeftJoining); // U+10ACD: Manichaean Letter Heth
```

```rust
pub const fn joining_type() -> CodePointMapDataBorrowed<''static, crate::JoiningType> { /* ... */ }
```

## Module `bidi_data`

Data and APIs for supporting specific Bidi properties data in an efficient structure.

Supported properties are:
- `Bidi_Paired_Bracket`
- `Bidi_Paired_Bracket_Type`
- `Bidi_Mirrored`
- `Bidi_Mirroring_Glyph`

```rust
pub mod bidi_data { /* ... */ }
```

### Types

#### Struct `BidiAuxiliaryProperties`

A wrapper around certain Bidi properties data. Can be obtained via [`bidi_auxiliary_properties()`] and
related getters.

Most useful methods are on [`BidiAuxiliaryPropertiesBorrowed`] obtained by calling [`BidiAuxiliaryProperties::as_borrowed()`]

```rust
pub struct BidiAuxiliaryProperties {
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
  pub fn as_borrowed(self: &Self) -> BidiAuxiliaryPropertiesBorrowed<''_> { /* ... */ }
  ```
  Construct a borrowed version of this type that can be queried.

- ```rust
  pub fn from_data(data: DataPayload<BidiAuxiliaryPropertiesV1Marker>) -> Self { /* ... */ }
  ```
  Construct a new one from loaded data

###### Trait Implementations

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
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
#### Struct `BidiMirroringProperties`

**Attributes:**

- `#[non_exhaustive]`

This struct represents the properties Bidi_Mirrored and Bidi_Mirroring_Glyph.
If Bidi_Mirroring_Glyph is not defined for a code point, then the value in the
struct is `None`.

```rust
pub struct BidiMirroringProperties {
    pub mirroring_glyph: Option<char>,
    pub mirrored: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `mirroring_glyph` | `Option<char>` | Represents the Bidi_Mirroring_Glyph property value |
| `mirrored` | `bool` | Represents the Bidi_Mirrored property value |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BidiMirroringProperties) -> bool { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **ErasedDestructor**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
#### Enum `BidiPairingProperties`

**Attributes:**

- `#[non_exhaustive]`

The enum represents Bidi_Paired_Bracket_Type, the char represents Bidi_Paired_Bracket.
Bidi_Paired_Bracket has a value of `None` when Bidi_Paired_Bracket_Type is `None`.

```rust
pub enum BidiPairingProperties {
    Open(char),
    Close(char),
    None,
}
```

##### Variants

###### `Open`

Represents Bidi_Paired_Bracket_Type=Open, and the Bidi_Paired_Bracket value for that code point.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

###### `Close`

Represents Bidi_Paired_Bracket_Type=Close, and the Bidi_Paired_Bracket value for that code point.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

###### `None`

Represents Bidi_Paired_Bracket_Type=None, which cooccurs with Bidi_Paired_Bracket
being undefined for that code point.

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BidiPairingProperties) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `BidiAuxiliaryPropertiesBorrowed`

A borrowed wrapper around Bidi properties data, returned by
[`BidiAuxiliaryProperties::as_borrowed()`]. More efficient to query.

```rust
pub struct BidiAuxiliaryPropertiesBorrowed<''a> {
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
  pub fn get32_mirroring_props(self: &Self, code_point: u32) -> BidiMirroringProperties { /* ... */ }
  ```
  Return a struct for the given code point representing Bidi mirroring-related

- ```rust
  pub fn get32_pairing_props(self: &Self, code_point: u32) -> BidiPairingProperties { /* ... */ }
  ```
  Return a struct for the given code point representing Bidi bracket

- ```rust
  pub const fn static_to_owned(self: Self) -> BidiAuxiliaryProperties { /* ... */ }
  ```
  Cheaply converts a [`BidiAuxiliaryPropertiesBorrowed<'static>`] into a [`BidiAuxiliaryProperties`].

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
### Functions

#### Function `bidi_auxiliary_properties`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Creates a [`BidiAuxiliaryPropertiesV1`] struct that represents the data for certain
Bidi properties.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Examples
```
use icu::properties::bidi_data;

let bidi_data = bidi_data::bidi_auxiliary_properties();

let open_paren = bidi_data.get32_mirroring_props('(' as u32);
assert_eq!(open_paren.mirroring_glyph, Some(')'));
assert_eq!(open_paren.mirrored, true);
```

```rust
pub const fn bidi_auxiliary_properties() -> BidiAuxiliaryPropertiesBorrowed<''static> { /* ... */ }
```

#### Function `load_bidi_auxiliary_properties_with_any_provider`

A version of [`bidi_auxiliary_properties`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_bidi_auxiliary_properties_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<BidiAuxiliaryProperties, crate::PropertiesError> { /* ... */ }
```

#### Function `load_bidi_auxiliary_properties_unstable`

A version of [`bidi_auxiliary_properties`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

[📚 Help choosing a constructor](icu_provider::constructors)

<div class="stab unstable">⚠️ The bounds on <tt>provider</tt> may change over time, including in SemVer minor releases.</div>

```rust
pub fn load_bidi_auxiliary_properties_unstable</* synthetic */ impl DataProvider<BidiAuxiliaryPropertiesV1Marker> + ?Sized: DataProvider<crate::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker> + ?Sized>(provider: &impl DataProvider<crate::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker> + ?Sized) -> Result<BidiAuxiliaryProperties, crate::PropertiesError> { /* ... */ }
```

## Module `exemplar_chars`

This module provides APIs for getting exemplar characters for a locale.

Exemplars are characters used by a language, separated into different sets.
The sets are: main, auxiliary, punctuation, numbers, and index.

The sets define, according to typical usage in the language,
which characters occur in which contexts with which frequency.
For more information, see the documentation in the
[Exemplars section in Unicode Technical Standard #35](https://unicode.org/reports/tr35/tr35-general.html#Exemplars)
of the LDML specification.

# Examples

```
use icu::locid::locale;
use icu::properties::exemplar_chars;

let locale = locale!("en-001").into();
let data = exemplar_chars::exemplars_main(&locale)
    .expect("locale should be present");
let exemplars_main = data.as_borrowed();

assert!(exemplars_main.contains_char('a'));
assert!(exemplars_main.contains_char('z'));
assert!(exemplars_main.contains("a"));
assert!(!exemplars_main.contains("ä"));
assert!(!exemplars_main.contains("ng"));
```

```rust
pub mod exemplar_chars { /* ... */ }
```

### Functions

#### Function `load_exemplars_main`

A version of [`exemplars_main()`] that uses custom data provided by a [`DataProvider`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_exemplars_main</* synthetic */ impl DataProvider<ExemplarCharactersMainV1Marker> + ?Sized: DataProvider<ExemplarCharactersMainV1Marker> + ?Sized>(provider: &impl DataProvider<ExemplarCharactersMainV1Marker> + ?Sized, locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `exemplars_main`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Get the "main" set of exemplar characters.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Examples

```
use icu::locid::locale;
use icu::properties::exemplar_chars;

let data = exemplar_chars::exemplars_main(&locale!("en").into())
    .expect("locale should be present");
let exemplars_main = data.as_borrowed();

assert!(exemplars_main.contains_char('a'));
assert!(exemplars_main.contains_char('z'));
assert!(exemplars_main.contains("a"));
assert!(!exemplars_main.contains("ä"));
assert!(!exemplars_main.contains("ng"));
assert!(!exemplars_main.contains("A"));
```

```rust
pub fn exemplars_main(locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `load_exemplars_auxiliary`

A version of [`exemplars_auxiliary()`] that uses custom data provided by a [`DataProvider`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_exemplars_auxiliary</* synthetic */ impl DataProvider<ExemplarCharactersAuxiliaryV1Marker> + ?Sized: DataProvider<ExemplarCharactersAuxiliaryV1Marker> + ?Sized>(provider: &impl DataProvider<ExemplarCharactersAuxiliaryV1Marker> + ?Sized, locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `exemplars_auxiliary`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Get the "auxiliary" set of exemplar characters.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Examples

```
use icu::locid::locale;
use icu::properties::exemplar_chars;

let data =
    exemplar_chars::exemplars_auxiliary(&locale!("en").into())
    .expect("locale should be present");
let exemplars_auxiliary = data.as_borrowed();

assert!(!exemplars_auxiliary.contains_char('a'));
assert!(!exemplars_auxiliary.contains_char('z'));
assert!(!exemplars_auxiliary.contains("a"));
assert!(exemplars_auxiliary.contains("ä"));
assert!(!exemplars_auxiliary.contains("ng"));
assert!(!exemplars_auxiliary.contains("A"));
```

```rust
pub fn exemplars_auxiliary(locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `load_exemplars_punctuation`

A version of [`exemplars_punctuation()`] that uses custom data provided by a [`DataProvider`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_exemplars_punctuation</* synthetic */ impl DataProvider<ExemplarCharactersPunctuationV1Marker> + ?Sized: DataProvider<ExemplarCharactersPunctuationV1Marker> + ?Sized>(provider: &impl DataProvider<ExemplarCharactersPunctuationV1Marker> + ?Sized, locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `exemplars_punctuation`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Get the "punctuation" set of exemplar characters.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Examples

```
use icu::locid::locale;
use icu::properties::exemplar_chars;

let data =
    exemplar_chars::exemplars_punctuation(&locale!("en").into())
    .expect("locale should be present");
let exemplars_punctuation = data.as_borrowed();

assert!(!exemplars_punctuation.contains_char('0'));
assert!(!exemplars_punctuation.contains_char('9'));
assert!(!exemplars_punctuation.contains_char('%'));
assert!(exemplars_punctuation.contains_char(','));
assert!(exemplars_punctuation.contains_char('.'));
assert!(exemplars_punctuation.contains_char('!'));
assert!(exemplars_punctuation.contains_char('?'));
```

```rust
pub fn exemplars_punctuation(locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `load_exemplars_numbers`

A version of [`exemplars_numbers()`] that uses custom data provided by a [`DataProvider`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_exemplars_numbers</* synthetic */ impl DataProvider<ExemplarCharactersNumbersV1Marker> + ?Sized: DataProvider<ExemplarCharactersNumbersV1Marker> + ?Sized>(provider: &impl DataProvider<ExemplarCharactersNumbersV1Marker> + ?Sized, locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `exemplars_numbers`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Get the "numbers" set of exemplar characters.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Examples

```
use icu::locid::locale;
use icu::properties::exemplar_chars;

let data =
    exemplar_chars::exemplars_numbers(&locale!("en").into())
    .expect("locale should be present");
let exemplars_numbers = data.as_borrowed();

assert!(exemplars_numbers.contains_char('0'));
assert!(exemplars_numbers.contains_char('9'));
assert!(exemplars_numbers.contains_char('%'));
assert!(exemplars_numbers.contains_char(','));
assert!(exemplars_numbers.contains_char('.'));
assert!(!exemplars_numbers.contains_char('!'));
assert!(!exemplars_numbers.contains_char('?'));
```

```rust
pub fn exemplars_numbers(locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `load_exemplars_index`

A version of [`exemplars_index()`] that uses custom data provided by a [`DataProvider`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_exemplars_index</* synthetic */ impl DataProvider<ExemplarCharactersIndexV1Marker> + ?Sized: DataProvider<ExemplarCharactersIndexV1Marker> + ?Sized>(provider: &impl DataProvider<ExemplarCharactersIndexV1Marker> + ?Sized, locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `exemplars_index`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Get the "index" set of exemplar characters.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Examples

```
use icu::locid::locale;
use icu::properties::exemplar_chars;

let data =
    exemplar_chars::exemplars_index(&locale!("en").into())
    .expect("locale should be present");
let exemplars_index = data.as_borrowed();

assert!(!exemplars_index.contains_char('a'));
assert!(!exemplars_index.contains_char('z'));
assert!(!exemplars_index.contains("a"));
assert!(!exemplars_index.contains("ä"));
assert!(!exemplars_index.contains("ng"));
assert!(exemplars_index.contains("A"));
```

```rust
pub fn exemplars_index(locale: &DataLocale) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

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

### Modules

## Module `names`

🚧 \[Unstable\] Property names-related data for this component

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

Read more about data providers: [`icu_provider`]

```rust
pub mod names { /* ... */ }
```

### Types

#### Struct `NormalizedPropertyNameStr`

**Attributes:**

- `#[<cfg_attr>(feature = "serde", derive(serde::Serialize))]`

This is a property name that can be "loose matched" as according to
[PropertyValueAliases.txt](https://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt)

(matched case-insensitively in ASCII, ignoring underscores, whitespace, and hyphens)

This is expected to be ASCII, but we do not rely on this invariant anywhere except during
datagen.

The Ord impl will sort things using strict equality, but in such a way that all loose-equal items
will sort into the same area, such that a map can be searched for both strict and loose equality.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

# Examples

Using a [`NormalizedPropertyNameStr`] as the key of a [`ZeroMap`]:

```
use icu::properties::provider::names::NormalizedPropertyNameStr;
use zerovec::ZeroMap;

let map: ZeroMap<NormalizedPropertyNameStr, usize> = [
    (NormalizedPropertyNameStr::from_str("A_BC"), 11),
    (NormalizedPropertyNameStr::from_str("dEf"), 22),
    (NormalizedPropertyNameStr::from_str("G_H-I"), 33),
]
.into_iter()
.collect();

let key_approx = NormalizedPropertyNameStr::from_str("AB-C");
let key_exact = NormalizedPropertyNameStr::from_str("A_BC");

// Strict lookup:
assert_eq!(None, map.get_copied(key_approx));
assert_eq!(Some(11), map.get_copied(key_exact));

// Loose lookup:
assert_eq!(Some(11), map.get_copied_by(|u| u.cmp_loose(key_approx)));
assert_eq!(Some(11), map.get_copied_by(|u| u.cmp_loose(key_exact)));
```

```rust
pub struct NormalizedPropertyNameStr(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn cmp_loose(self: &Self, other: &Self) -> Ordering { /* ... */ }
  ```
  Perform the loose comparison as defined in [`NormalizedPropertyNameStr`].

- ```rust
  pub const fn from_str(s: &str) -> &Self { /* ... */ }
  ```
  Convert a string reference to a [`NormalizedPropertyNameStr`].

- ```rust
  pub const fn cast_ref(value: &UnvalidatedStr) -> &Self { /* ... */ }
  ```
  Convert a [`UnvalidatedStr`] reference to a [`NormalizedPropertyNameStr`] reference.

- ```rust
  pub const fn cast_box(value: Box<UnvalidatedStr>) -> Box<Self> { /* ... */ }
  ```
  Convert a [`UnvalidatedStr`] box to a [`NormalizedPropertyNameStr`] box.

- ```rust
  pub fn boxed_from_bytes(b: &[u8]) -> Box<Self> { /* ... */ }
  ```
  Get a [`NormalizedPropertyNameStr`] box from a byte slice.

###### Trait Implementations

- **EncodeAsVarULE**
  - ```rust
    fn encode_var_ule_as_slices<R>(self: &Self, cb: impl FnOnce(&[&[u8]]) -> R) -> R { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **VarULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> { /* ... */ }
    ```

  - ```rust
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NormalizedPropertyNameStr) -> bool { /* ... */ }
    ```

- **Eq**
- **ZeroMapKV**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Sized**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `GeneralCategoryMaskNameToValueV1Marker`

Marker type for [`PropertyValueNameToEnumMapV1`]: "propnames/from/gcm@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct GeneralCategoryMaskNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **KeyedDataMarker**
- **UnwindSafe**
- **Unpin**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::names::GeneralCategoryMaskNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **DataMarker**
- **MaybeSendSync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
#### Struct `PropertyValueNameToEnumMapV1`

**Attributes:**

- `#[yoke(prove_covariance_manually)]`

A set of characters and strings which share a particular property value.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct PropertyValueNameToEnumMapV1<''data> {
    pub map: zerovec::ZeroMap<''data, NormalizedPropertyNameStr, u16>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `zerovec::ZeroMap<''data, NormalizedPropertyNameStr, u16>` | A map from names to their value discriminant |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **ErasedDestructor**
- **UnwindSafe**
- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf PropertyValueNameToEnumMapV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PropertyValueNameToEnumMapV1<''data> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PropertyValueNameToEnumMapV1<''data>) -> bool { /* ... */ }
    ```

#### Struct `PropertyEnumToValueNameSparseMapV1`

**Attributes:**

- `#[yoke(prove_covariance_manually)]`

A mapping of property values to their names. A single instance of this map will only cover
either long or short names, determined whilst loading data.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct PropertyEnumToValueNameSparseMapV1<''data> {
    pub map: zerovec::ZeroMap<''data, u16, str>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `zerovec::ZeroMap<''data, u16, str>` | A map from the value discriminant to the names |

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf PropertyEnumToValueNameSparseMapV1<''zf_inner>) -> Self { /* ... */ }
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PropertyEnumToValueNameSparseMapV1<''data> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PropertyEnumToValueNameSparseMapV1<''data>) -> bool { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
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

#### Struct `PropertyEnumToValueNameLinearMapV1`

**Attributes:**

- `#[yoke(prove_covariance_manually)]`

A mapping of property values to their names. A single instance of this map will only cover
either long or short names, determined whilst loading data.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct PropertyEnumToValueNameLinearMapV1<''data> {
    pub map: zerovec::VarZeroVec<''data, str>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `zerovec::VarZeroVec<''data, str>` | A map from the value discriminant (the index) to the names, for mostly<br>contiguous data. Empty strings count as missing. |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf PropertyEnumToValueNameLinearMapV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PropertyEnumToValueNameLinearMapV1<''data> { /* ... */ }
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
    fn eq(self: &Self, other: &PropertyEnumToValueNameLinearMapV1<''data>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
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

- **Send**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `PropertyEnumToValueNameLinearTiny4MapV1`

**Attributes:**

- `#[yoke(prove_covariance_manually)]`

A mapping of property values to their names. A single instance of this map will only cover
either long or short names, determined whilst loading data.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct PropertyEnumToValueNameLinearTiny4MapV1<''data> {
    pub map: zerovec::ZeroVec<''data, tinystr::TinyStr4>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `zerovec::ZeroVec<''data, tinystr::TinyStr4>` | A map from the value discriminant (the index) to the names, for mostly<br>contiguous data. Empty strings count as missing. |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PropertyEnumToValueNameLinearTiny4MapV1<''data>) -> bool { /* ... */ }
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PropertyEnumToValueNameLinearTiny4MapV1<''data> { /* ... */ }
    ```

- **MaybeSendSync**
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

- **Unpin**
- **Send**
- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf PropertyEnumToValueNameLinearTiny4MapV1<''zf_inner>) -> Self { /* ... */ }
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

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `bidi_data`

🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

Read more about data providers: [`icu_provider`]

This module provides an efficient storage of data serving the following
properties:
- `Bidi_Paired_Bracket`
- `Bidi_Paired_Bracket_Type`
- `Bidi_Mirrored`
- `Bidi_Mirroring_Glyph`

```rust
pub mod bidi_data { /* ... */ }
```

### Types

#### Struct `BidiAuxiliaryPropertiesV1Marker`

Marker type for [`BidiAuxiliaryPropertiesV1`]: "props/bidiauxiliaryprops@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct BidiAuxiliaryPropertiesV1Marker;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **KeyedDataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataMarker**
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

- **MaybeSendSync**
- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **ErasedDestructor**
#### Struct `BidiAuxiliaryPropertiesV1`

A data provider struct for properties related to Bidi algorithms, including
mirroring and bracket pairing.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct BidiAuxiliaryPropertiesV1<''data> {
    pub trie: icu_collections::codepointtrie::CodePointTrie<''data, MirroredPairedBracketData>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `trie` | `icu_collections::codepointtrie::CodePointTrie<''data, MirroredPairedBracketData>` | A `CodePointTrie` efficiently storing the data from which property values<br>can be extracted or derived for the supported Bidi properties. |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BidiAuxiliaryPropertiesV1<''data>) -> bool { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Freeze**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> BidiAuxiliaryPropertiesV1<''data> { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf BidiAuxiliaryPropertiesV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `MirroredPairedBracketDataTryFromError`

**Attributes:**

- `#[displaydoc("Invalid MirroredPairedBracketData serialized in int: {0}")]`

A `u32` serialized value of `MirroredPairedBracketData` did not encode either a valid Bidi_Mirroring_Glyph or a valid Bidi_Paired_Bracket_Type

```rust
pub struct MirroredPairedBracketDataTryFromError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Eq**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> MirroredPairedBracketDataTryFromError { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MirroredPairedBracketDataTryFromError) -> bool { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `CheckedBidiPairedBracketType`

**Attributes:**

- `#[allow(clippy::exhaustive_enums)]`
- `#[repr(u8)]`

A closed Rust enum representing a closed set of the incoming Bidi_Paired_Bracket_Type
property values necessary in the internal representation of `MirroredPairedBracketData`
to satisfy the ULE invariants on valid values.

```rust
pub enum CheckedBidiPairedBracketType {
    None = 0,
    Open = 1,
    Close = 2,
}
```

##### Variants

###### `None`

Not a paired bracket.

Discriminant: `0`

Discriminant value: `0`

###### `Open`

Open paired bracket.

Discriminant: `1`

Discriminant value: `1`

###### `Close`

Close paired bracket.

Discriminant: `2`

Discriminant value: `2`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **MaybeSendSync**
- **Eq**
- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &CheckedBidiPairedBracketType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **AsULE**
  - ```rust
    fn to_unaligned(self: Self) -> <Self as >::ULE { /* ... */ }
    ```

  - ```rust
    fn from_unaligned(other: <Self as >::ULE) -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ZeroMapKV**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &CheckedBidiPairedBracketType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TrieValue**
  - ```rust
    fn try_from_u32(i: u32) -> Result<Self, <Self as >::TryFromU32Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CheckedBidiPairedBracketType { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CheckedBidiPairedBracketType) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `CheckedBidiPairedBracketTypeULE`

[`ULE`](zerovec::ule::ULE) type for CheckedBidiPairedBracketType

```rust
pub struct CheckedBidiPairedBracketTypeULE(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CheckedBidiPairedBracketTypeULE { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &CheckedBidiPairedBracketTypeULE) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &CheckedBidiPairedBracketTypeULE) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CheckedBidiPairedBracketTypeULE) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **ULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Send**
- **MaybeSendSync**
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

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: $crate::DataRequest<''_>) -> Result<$crate::DataResponse<$crate::NeverMarker<Y>>, $crate::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EastAsianWidthNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GeneralCategoryNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::names::GeneralCategoryMaskNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HangulSyllableTypeNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IndicSyllabicCategoryNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoiningTypeNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EastAsianWidthValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GeneralCategoryValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HangulSyllableTypeValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IndicSyllabicCategoryValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoiningTypeValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EastAsianWidthValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GeneralCategoryValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HangulSyllableTypeValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IndicSyllabicCategoryValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoiningTypeValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::AsciiHexDigitV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::AlnumV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::AlphabeticV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BasicEmojiV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiControlV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiMirroredV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BlankV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CasedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CaseIgnorableV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::FullCompositionExclusionV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenCasefoldedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenCasemappedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenNfkcCasefoldedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenLowercasedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenTitlecasedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenUppercasedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::DashV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::DeprecatedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::DefaultIgnorableCodePointV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::DiacriticV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EastAsianWidthV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiModifierBaseV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiComponentV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiModifierV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiPresentationV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersIndexV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersMainV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersNumbersV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersPunctuationV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExtenderV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExtendedPictographicV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GeneralCategoryV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeBaseV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeExtendV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeLinkV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HexDigitV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HangulSyllableTypeV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HyphenV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdContinueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdeographicV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdStartV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdsBinaryOperatorV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdsTrinaryOperatorV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IndicSyllabicCategoryV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoinControlV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoiningTypeV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LogicalOrderExceptionV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LowercaseV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::MathV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NoncharacterCodePointV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NfcInertV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NfdInertV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NfkcInertV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NfkdInertV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::PatternSyntaxV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::PatternWhiteSpaceV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::PrependedConcatenationMarkV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::PrintV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::QuotationMarkV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::RadicalV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::RegionalIndicatorV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptWithExtensionsPropertyV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SoftDottedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SegmentStarterV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CaseSensitiveV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceTerminalV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::TerminalPunctuationV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::UnifiedIdeographV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::UppercaseV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::VariationSelectorV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WhiteSpaceV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::XdigitV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::XidContinueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::XidStartV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Enum `PropertyCodePointSetV1`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(serde::Serialize, databake::Bake),
databake(path = icu_properties::provider),)]`
- `#[<cfg_attr>(feature = "serde", derive(serde::Deserialize))]`
- `#[non_exhaustive]`

A set of characters which share a particular property value.

This data enum is extensible, more backends may be added in the future.
Old data can be used with newer code but not vice versa.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub enum PropertyCodePointSetV1<''data> {
    InversionList(icu_collections::codepointinvlist::CodePointInversionList<''data>),
}
```

##### Variants

###### `InversionList`

The set of characters, represented as an inversion list

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `icu_collections::codepointinvlist::CodePointInversionList<''data>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf PropertyCodePointSetV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PropertyCodePointSetV1<''data> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **MaybeSendSync**
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

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PropertyCodePointSetV1<''data>) -> bool { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
#### Enum `PropertyCodePointMapV1`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(serde::Serialize, databake::Bake),
databake(path = icu_properties::provider),)]`
- `#[<cfg_attr>(feature = "serde", derive(serde::Deserialize))]`
- `#[non_exhaustive]`

A map efficiently storing data about individual characters.

This data enum is extensible, more backends may be added in the future.
Old data can be used with newer code but not vice versa.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub enum PropertyCodePointMapV1<''data, T: TrieValue> {
    CodePointTrie(icu_collections::codepointtrie::CodePointTrie<''data, T>),
}
```

##### Variants

###### `CodePointTrie`

A codepoint trie storing the data

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `icu_collections::codepointtrie::CodePointTrie<''data, T>` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf PropertyCodePointMapV1<''zf_inner, T>) -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **Eq**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PropertyCodePointMapV1<''data, T> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Send**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PropertyCodePointMapV1<''data, T>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Enum `PropertyUnicodeSetV1`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(serde::Serialize, databake::Bake),
databake(path = icu_properties::provider),)]`
- `#[<cfg_attr>(feature = "serde", derive(serde::Deserialize))]`
- `#[non_exhaustive]`

A set of characters and strings which share a particular property value.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub enum PropertyUnicodeSetV1<''data> {
    CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList<''data>),
}
```

##### Variants

###### `CPInversionListStrList`

A set representing characters in an inversion list, and the strings in a list.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList<''data>` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PropertyUnicodeSetV1<''data> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PropertyUnicodeSetV1<''data>) -> bool { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf PropertyUnicodeSetV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
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
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
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

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ScriptWithExtensionsPropertyV1Marker`

Marker type for [`ScriptWithExtensionsPropertyV1`]: "props/scx@1"

- Fallback priority: language (default)
- Extension keyword: none (default)

```rust
pub struct ScriptWithExtensionsPropertyV1Marker;
```

##### Implementations

###### Trait Implementations

- **KeyedDataMarker**
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
- **ErasedDestructor**
- **Freeze**
- **DataMarker**
- **MaybeSendSync**
- **UnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptWithExtensionsPropertyV1Marker>, icu_provider::DataError> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `ScriptWithExtensionsPropertyV1`

A struct that efficiently stores `Script` and `Script_Extensions` property data.

<div class="stab unstable">
🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
including in SemVer minor releases. While the serde representation of data structs is guaranteed
to be stable, their Rust representation might not be. Use with caution.
</div>

```rust
pub struct ScriptWithExtensionsPropertyV1<''data> {
    pub trie: icu_collections::codepointtrie::CodePointTrie<''data, crate::script::ScriptWithExt>,
    pub extensions: zerovec::VarZeroVec<''data, zerovec::ZeroSlice<crate::Script>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `trie` | `icu_collections::codepointtrie::CodePointTrie<''data, crate::script::ScriptWithExt>` | Note: The `ScriptWithExt` values in this array will assume a 12-bit layout. The 2<br>higher order bits 11..10 will indicate how to deduce the Script value and<br>Script_Extensions value, nearly matching the representation<br>[in ICU](https://github.com/unicode-org/icu/blob/main/icu4c/source/common/uprops.h):<br><br>| High order 2 bits value | Script                                                 | Script_Extensions                                              |<br>|-------------------------|--------------------------------------------------------|----------------------------------------------------------------|<br>| 3                       | First value in sub-array, index given by lower 10 bits | Sub-array excluding first value, index given by lower 10 bits  |<br>| 2                       | Script=Inherited                                       | Entire sub-array, index given by lower 10 bits                 |<br>| 1                       | Script=Common                                          | Entire sub-array, index given by lower 10 bits                 |<br>| 0                       | Value in lower 10 bits                                 | `[ Script value ]` single-element array                        |<br><br>When the lower 10 bits of the value are used as an index, that index is<br>used for the outer-level vector of the nested `extensions` structure. |
| `extensions` | `zerovec::VarZeroVec<''data, zerovec::ZeroSlice<crate::Script>>` | This companion structure stores Script_Extensions values, which are<br>themselves arrays / vectors. This structure only stores the values for<br>cases in which `scx(cp) != [ sc(cp) ]`. Each sub-vector is distinct. The<br>sub-vector represents the Script_Extensions array value for a code point,<br>and may also indicate Script value, as described for the `trie` field. |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
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

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ScriptWithExtensionsPropertyV1<''data>) -> bool { /* ... */ }
    ```

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

- **ErasedDestructor**
- **MaybeSendSync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
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
    fn clone(self: &Self) -> ScriptWithExtensionsPropertyV1<''data> { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf ScriptWithExtensionsPropertyV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `AsciiHexDigitV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'AHex' Unicode property

```rust
pub struct AsciiHexDigitV1Marker;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **RefUnwindSafe**
- **MaybeSendSync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::AsciiHexDigitV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **KeyedDataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> AsciiHexDigitV1Marker { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `AlnumV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'alnum' Unicode property

```rust
pub struct AlnumV1Marker;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> AlnumV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataMarker**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::AlnumV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `AlphabeticV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Alpha' Unicode property

```rust
pub struct AlphabeticV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataMarker**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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
- **Sync**
- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> AlphabeticV1Marker { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::AlphabeticV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Struct `BidiControlV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Bidi_C' Unicode property

```rust
pub struct BidiControlV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **KeyedDataMarker**
- **Default**
  - ```rust
    fn default() -> BidiControlV1Marker { /* ... */ }
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

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiControlV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `BidiMirroredV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Bidi_M' Unicode property

```rust
pub struct BidiMirroredV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **KeyedDataMarker**
- **DataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiMirroredV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> BidiMirroredV1Marker { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
#### Struct `BlankV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'blank' Unicode property

```rust
pub struct BlankV1Marker;
```

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> BlankV1Marker { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **ErasedDestructor**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BlankV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataMarker**
- **UnwindSafe**
- **KeyedDataMarker**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `CasedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Cased' Unicode property

```rust
pub struct CasedV1Marker;
```

##### Implementations

###### Trait Implementations

- **ErasedDestructor**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CasedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **KeyedDataMarker**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
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

- **DataMarker**
- **Default**
  - ```rust
    fn default() -> CasedV1Marker { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `CaseIgnorableV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'CI' Unicode property

```rust
pub struct CaseIgnorableV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **DataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> CaseIgnorableV1Marker { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CaseIgnorableV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Unpin**
- **KeyedDataMarker**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `FullCompositionExclusionV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Comp_Ex' Unicode property

```rust
pub struct FullCompositionExclusionV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> FullCompositionExclusionV1Marker { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::FullCompositionExclusionV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataMarker**
- **ErasedDestructor**
- **MaybeSendSync**
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

- **UnwindSafe**
- **KeyedDataMarker**
#### Struct `ChangesWhenCasefoldedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'CWCF' Unicode property

```rust
pub struct ChangesWhenCasefoldedV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenCasefoldedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ChangesWhenCasefoldedV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Sync**
- **DataMarker**
- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `ChangesWhenCasemappedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'CWCM' Unicode property

```rust
pub struct ChangesWhenCasemappedV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenCasemappedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
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
    fn default() -> ChangesWhenCasemappedV1Marker { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **KeyedDataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `ChangesWhenNfkcCasefoldedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'CWKCF' Unicode property

```rust
pub struct ChangesWhenNfkcCasefoldedV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenNfkcCasefoldedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ChangesWhenNfkcCasefoldedV1Marker { /* ... */ }
    ```

- **RefUnwindSafe**
- **DataMarker**
- **KeyedDataMarker**
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

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Sync**
- **MaybeSendSync**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ChangesWhenLowercasedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'CWL' Unicode property

```rust
pub struct ChangesWhenLowercasedV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **DataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
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

- **MaybeSendSync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ChangesWhenLowercasedV1Marker { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **KeyedDataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenLowercasedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Struct `ChangesWhenTitlecasedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'CWT' Unicode property

```rust
pub struct ChangesWhenTitlecasedV1Marker;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **DataMarker**
- **MaybeSendSync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenTitlecasedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ChangesWhenTitlecasedV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **ErasedDestructor**
#### Struct `ChangesWhenUppercasedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'CWU' Unicode property

```rust
pub struct ChangesWhenUppercasedV1Marker;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **KeyedDataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ChangesWhenUppercasedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **ErasedDestructor**
- **DataMarker**
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ChangesWhenUppercasedV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `DashV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Dash' Unicode property

```rust
pub struct DashV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> DashV1Marker { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::DashV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **KeyedDataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Sync**
- **DataMarker**
#### Struct `DeprecatedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Dep' Unicode property

```rust
pub struct DeprecatedV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataMarker**
- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::DeprecatedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
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

- **Sync**
- **Send**
- **ErasedDestructor**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> DeprecatedV1Marker { /* ... */ }
    ```

#### Struct `DefaultIgnorableCodePointV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'DI' Unicode property

```rust
pub struct DefaultIgnorableCodePointV1Marker;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> DefaultIgnorableCodePointV1Marker { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **Send**
- **Sync**
- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::DefaultIgnorableCodePointV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
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

- **Freeze**
#### Struct `DiacriticV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Dia' Unicode property

```rust
pub struct DiacriticV1Marker;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Default**
  - ```rust
    fn default() -> DiacriticV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **Unpin**
- **DataMarker**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::DiacriticV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `EmojiModifierBaseV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'EBase' Unicode property

```rust
pub struct EmojiModifierBaseV1Marker;
```

##### Implementations

###### Trait Implementations

- **KeyedDataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Default**
  - ```rust
    fn default() -> EmojiModifierBaseV1Marker { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiModifierBaseV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **DataMarker**
- **RefUnwindSafe**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `EmojiComponentV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'EComp' Unicode property

```rust
pub struct EmojiComponentV1Marker;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> EmojiComponentV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiComponentV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **DataMarker**
- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `EmojiModifierV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'EMod' Unicode property

```rust
pub struct EmojiModifierV1Marker;
```

##### Implementations

###### Trait Implementations

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> EmojiModifierV1Marker { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **KeyedDataMarker**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiModifierV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataMarker**
#### Struct `EmojiV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Emoji' Unicode property

```rust
pub struct EmojiV1Marker;
```

##### Implementations

###### Trait Implementations

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

- **UnwindSafe**
- **ErasedDestructor**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **KeyedDataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EmojiV1Marker { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataMarker**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `EmojiPresentationV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'EPres' Unicode property

```rust
pub struct EmojiPresentationV1Marker;
```

##### Implementations

###### Trait Implementations

- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **Unpin**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EmojiPresentationV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EmojiPresentationV1Marker { /* ... */ }
    ```

#### Struct `ExtenderV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Ext' Unicode property

```rust
pub struct ExtenderV1Marker;
```

##### Implementations

###### Trait Implementations

- **Sync**
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

- **Default**
  - ```rust
    fn default() -> ExtenderV1Marker { /* ... */ }
    ```

- **DataMarker**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExtenderV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **KeyedDataMarker**
- **RefUnwindSafe**
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

- **Freeze**
#### Struct `ExtendedPictographicV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'ExtPict' Unicode property

```rust
pub struct ExtendedPictographicV1Marker;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ExtendedPictographicV1Marker { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **DataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
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

- **ErasedDestructor**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExtendedPictographicV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
#### Struct `GraphV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'graph' Unicode property

```rust
pub struct GraphV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> GraphV1Marker { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **UnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **KeyedDataMarker**
#### Struct `GraphemeBaseV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Gr_Base' Unicode property

```rust
pub struct GraphemeBaseV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> GraphemeBaseV1Marker { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **RefUnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeBaseV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **KeyedDataMarker**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataMarker**
#### Struct `GraphemeExtendV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Gr_Ext' Unicode property

```rust
pub struct GraphemeExtendV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
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
- **KeyedDataMarker**
- **Send**
- **Default**
  - ```rust
    fn default() -> GraphemeExtendV1Marker { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeExtendV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **RefUnwindSafe**
- **DataMarker**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `GraphemeLinkV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Gr_Link' Unicode property

```rust
pub struct GraphemeLinkV1Marker;
```

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **DataMarker**
- **Freeze**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeLinkV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **KeyedDataMarker**
- **Default**
  - ```rust
    fn default() -> GraphemeLinkV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `HexDigitV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Hex' Unicode property

```rust
pub struct HexDigitV1Marker;
```

##### Implementations

###### Trait Implementations

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

- **Default**
  - ```rust
    fn default() -> HexDigitV1Marker { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **MaybeSendSync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **DataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HexDigitV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Sync**
#### Struct `HyphenV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Hyphen' Unicode property

```rust
pub struct HyphenV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> HyphenV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **DataMarker**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HyphenV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **KeyedDataMarker**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `IdContinueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'IDC' Unicode property

```rust
pub struct IdContinueV1Marker;
```

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **Send**
- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **DataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> IdContinueV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
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

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdContinueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `IdeographicV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Ideo' Unicode property

```rust
pub struct IdeographicV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **KeyedDataMarker**
- **Freeze**
- **UnwindSafe**
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
- **Unpin**
- **RefUnwindSafe**
- **DataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> IdeographicV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdeographicV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `IdStartV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'IDS' Unicode property

```rust
pub struct IdStartV1Marker;
```

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdStartV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> IdStartV1Marker { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
#### Struct `IdsBinaryOperatorV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'IDSB' Unicode property

```rust
pub struct IdsBinaryOperatorV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **ErasedDestructor**
- **Freeze**
- **Unpin**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdsBinaryOperatorV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> IdsBinaryOperatorV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `IdsTrinaryOperatorV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'IDST' Unicode property

```rust
pub struct IdsTrinaryOperatorV1Marker;
```

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> IdsTrinaryOperatorV1Marker { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IdsTrinaryOperatorV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **DataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **KeyedDataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
- **UnwindSafe**
#### Struct `JoinControlV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Join_C' Unicode property

```rust
pub struct JoinControlV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> JoinControlV1Marker { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
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

- **Unpin**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoinControlV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **DataMarker**
- **Freeze**
- **MaybeSendSync**
- **Send**
- **KeyedDataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `LogicalOrderExceptionV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'LOE' Unicode property

```rust
pub struct LogicalOrderExceptionV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LogicalOrderExceptionV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **ErasedDestructor**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> LogicalOrderExceptionV1Marker { /* ... */ }
    ```

- **UnwindSafe**
- **DataMarker**
- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `LowercaseV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Lower' Unicode property

```rust
pub struct LowercaseV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LowercaseV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> LowercaseV1Marker { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **KeyedDataMarker**
- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataMarker**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
#### Struct `MathV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Math' Unicode property

```rust
pub struct MathV1Marker;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::MathV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> MathV1Marker { /* ... */ }
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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **KeyedDataMarker**
- **Freeze**
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
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
#### Struct `NoncharacterCodePointV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'NChar' Unicode property

```rust
pub struct NoncharacterCodePointV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> NoncharacterCodePointV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataMarker**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NoncharacterCodePointV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
#### Struct `NfcInertV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'nfcinert' Unicode property

```rust
pub struct NfcInertV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NfcInertV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **KeyedDataMarker**
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

- **Default**
  - ```rust
    fn default() -> NfcInertV1Marker { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **MaybeSendSync**
- **Unpin**
- **Freeze**
- **Send**
#### Struct `NfdInertV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'nfdinert' Unicode property

```rust
pub struct NfdInertV1Marker;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **KeyedDataMarker**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NfdInertV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> NfdInertV1Marker { /* ... */ }
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

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `NfkcInertV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'nfkcinert' Unicode property

```rust
pub struct NfkcInertV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NfkcInertV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> NfkcInertV1Marker { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
#### Struct `NfkdInertV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'nfkdinert' Unicode property

```rust
pub struct NfkdInertV1Marker;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::NfkdInertV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> NfkdInertV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **DataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `PatternSyntaxV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Pat_Syn' Unicode property

```rust
pub struct PatternSyntaxV1Marker;
```

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> PatternSyntaxV1Marker { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::PatternSyntaxV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Unpin**
- **ErasedDestructor**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **KeyedDataMarker**
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

#### Struct `PatternWhiteSpaceV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Pat_WS' Unicode property

```rust
pub struct PatternWhiteSpaceV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Freeze**
- **UnwindSafe**
- **DataMarker**
- **KeyedDataMarker**
- **Unpin**
- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::PatternWhiteSpaceV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> PatternWhiteSpaceV1Marker { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

#### Struct `PrependedConcatenationMarkV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'PCM' Unicode property

```rust
pub struct PrependedConcatenationMarkV1Marker;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Sync**
- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> PrependedConcatenationMarkV1Marker { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::PrependedConcatenationMarkV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `PrintV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'print' Unicode property

```rust
pub struct PrintV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> PrintV1Marker { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **KeyedDataMarker**
- **ErasedDestructor**
- **Sync**
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

- **UnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::PrintV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Send**
#### Struct `QuotationMarkV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'QMark' Unicode property

```rust
pub struct QuotationMarkV1Marker;
```

##### Implementations

###### Trait Implementations

- **KeyedDataMarker**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> QuotationMarkV1Marker { /* ... */ }
    ```

- **UnwindSafe**
- **DataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::QuotationMarkV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Struct `RadicalV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Radical' Unicode property

```rust
pub struct RadicalV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::RadicalV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> RadicalV1Marker { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **KeyedDataMarker**
- **ErasedDestructor**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataMarker**
- **Freeze**
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
#### Struct `RegionalIndicatorV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'RI' Unicode property

```rust
pub struct RegionalIndicatorV1Marker;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **MaybeSendSync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::RegionalIndicatorV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **KeyedDataMarker**
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
- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> RegionalIndicatorV1Marker { /* ... */ }
    ```

#### Struct `SoftDottedV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'SD' Unicode property

```rust
pub struct SoftDottedV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SoftDottedV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **Default**
  - ```rust
    fn default() -> SoftDottedV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **DataMarker**
- **KeyedDataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `SegmentStarterV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'segstart' Unicode property

```rust
pub struct SegmentStarterV1Marker;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SegmentStarterV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
- **Default**
  - ```rust
    fn default() -> SegmentStarterV1Marker { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
#### Struct `CaseSensitiveV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Sensitive' Unicode property

```rust
pub struct CaseSensitiveV1Marker;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CaseSensitiveV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> CaseSensitiveV1Marker { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **DataMarker**
- **KeyedDataMarker**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **MaybeSendSync**
#### Struct `SentenceTerminalV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'STerm' Unicode property

```rust
pub struct SentenceTerminalV1Marker;
```

##### Implementations

###### Trait Implementations

- **KeyedDataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SentenceTerminalV1Marker { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
- **Unpin**
- **Sync**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceTerminalV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Freeze**
#### Struct `TerminalPunctuationV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Term' Unicode property

```rust
pub struct TerminalPunctuationV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::TerminalPunctuationV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataMarker**
- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Send**
- **Freeze**
- **MaybeSendSync**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> TerminalPunctuationV1Marker { /* ... */ }
    ```

#### Struct `UnifiedIdeographV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'UIdeo' Unicode property

```rust
pub struct UnifiedIdeographV1Marker;
```

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> UnifiedIdeographV1Marker { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::UnifiedIdeographV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **DataMarker**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **KeyedDataMarker**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
#### Struct `UppercaseV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Upper' Unicode property

```rust
pub struct UppercaseV1Marker;
```

##### Implementations

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
- **Send**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **DataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::UppercaseV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **KeyedDataMarker**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> UppercaseV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **RefUnwindSafe**
#### Struct `VariationSelectorV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'VS' Unicode property

```rust
pub struct VariationSelectorV1Marker;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> VariationSelectorV1Marker { /* ... */ }
    ```

- **ErasedDestructor**
- **KeyedDataMarker**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::VariationSelectorV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
#### Struct `WhiteSpaceV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'WSpace' Unicode property

```rust
pub struct WhiteSpaceV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WhiteSpaceV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> WhiteSpaceV1Marker { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **DataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **KeyedDataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `XdigitV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'xdigit' Unicode property

```rust
pub struct XdigitV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::XdigitV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **KeyedDataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **MaybeSendSync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> XdigitV1Marker { /* ... */ }
    ```

#### Struct `XidContinueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'XIDC' Unicode property

```rust
pub struct XidContinueV1Marker;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::XidContinueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> XidContinueV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `XidStartV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'XIDS' Unicode property

```rust
pub struct XidStartV1Marker;
```

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **DataMarker**
- **KeyedDataMarker**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Default**
  - ```rust
    fn default() -> XidStartV1Marker { /* ... */ }
    ```

- **Freeze**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::XidStartV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Struct `BasicEmojiV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'Basic_Emoji' Unicode property

```rust
pub struct BasicEmojiV1Marker;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **KeyedDataMarker**
- **RefUnwindSafe**
- **DataMarker**
- **Freeze**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BasicEmojiV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> BasicEmojiV1Marker { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Sync**
#### Struct `ExemplarCharactersMainV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'exemplarchars/main' Unicode property

```rust
pub struct ExemplarCharactersMainV1Marker;
```

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersMainV1Marker>, icu_provider::DataError> { /* ... */ }
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

- **KeyedDataMarker**
- **Freeze**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> ExemplarCharactersMainV1Marker { /* ... */ }
    ```

- **DataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ExemplarCharactersAuxiliaryV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'exemplarchars/auxiliary' Unicode property

```rust
pub struct ExemplarCharactersAuxiliaryV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **KeyedDataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersAuxiliaryV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> ExemplarCharactersAuxiliaryV1Marker { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `ExemplarCharactersPunctuationV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'exemplarchars/punctuation' Unicode property

```rust
pub struct ExemplarCharactersPunctuationV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **KeyedDataMarker**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersPunctuationV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ExemplarCharactersPunctuationV1Marker { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `ExemplarCharactersNumbersV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'exemplarchars/numbers' Unicode property

```rust
pub struct ExemplarCharactersNumbersV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **KeyedDataMarker**
- **RefUnwindSafe**
- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersNumbersV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ExemplarCharactersNumbersV1Marker { /* ... */ }
    ```

#### Struct `ExemplarCharactersIndexV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'exemplarchars/index' Unicode property

```rust
pub struct ExemplarCharactersIndexV1Marker;
```

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> ExemplarCharactersIndexV1Marker { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataMarker**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **KeyedDataMarker**
- **Unpin**
- **RefUnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ExemplarCharactersIndexV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Struct `CanonicalCombiningClassV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'ccc' Unicode property

```rust
pub struct CanonicalCombiningClassV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> CanonicalCombiningClassV1Marker { /* ... */ }
    ```

- **DataMarker**
- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **KeyedDataMarker**
- **Freeze**
- **MaybeSendSync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `CanonicalCombiningClassNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'ccc' Unicode property

```rust
pub struct CanonicalCombiningClassNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> CanonicalCombiningClassNameToValueV1Marker { /* ... */ }
    ```

- **DataMarker**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **KeyedDataMarker**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `CanonicalCombiningClassValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'ccc' Unicode property

```rust
pub struct CanonicalCombiningClassValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **KeyedDataMarker**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **UnwindSafe**
- **Send**
- **Freeze**
- **Sync**
- **Unpin**
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

- **Default**
  - ```rust
    fn default() -> CanonicalCombiningClassValueToShortNameV1Marker { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `CanonicalCombiningClassValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'ccc' Unicode property

```rust
pub struct CanonicalCombiningClassValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **RefUnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> CanonicalCombiningClassValueToLongNameV1Marker { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::CanonicalCombiningClassValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **KeyedDataMarker**
#### Struct `GeneralCategoryV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'gc' Unicode property

```rust
pub struct GeneralCategoryV1Marker;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GeneralCategoryV1Marker>, icu_provider::DataError> { /* ... */ }
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **KeyedDataMarker**
- **Default**
  - ```rust
    fn default() -> GeneralCategoryV1Marker { /* ... */ }
    ```

- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `GeneralCategoryNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'gc' Unicode property

```rust
pub struct GeneralCategoryNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Default**
  - ```rust
    fn default() -> GeneralCategoryNameToValueV1Marker { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GeneralCategoryNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **DataMarker**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `GeneralCategoryValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'gc' Unicode property

```rust
pub struct GeneralCategoryValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **DataMarker**
- **KeyedDataMarker**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GeneralCategoryValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Default**
  - ```rust
    fn default() -> GeneralCategoryValueToShortNameV1Marker { /* ... */ }
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

#### Struct `GeneralCategoryValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'gc' Unicode property

```rust
pub struct GeneralCategoryValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> GeneralCategoryValueToLongNameV1Marker { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **KeyedDataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GeneralCategoryValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Struct `BidiClassV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'bc' Unicode property

```rust
pub struct BidiClassV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **RefUnwindSafe**
- **KeyedDataMarker**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> BidiClassV1Marker { /* ... */ }
    ```

#### Struct `BidiClassNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'bc' Unicode property

```rust
pub struct BidiClassNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> BidiClassNameToValueV1Marker { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **KeyedDataMarker**
- **Send**
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

#### Struct `BidiClassValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'bc' Unicode property

```rust
pub struct BidiClassValueToShortNameV1Marker;
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **KeyedDataMarker**
- **ErasedDestructor**
- **MaybeSendSync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> BidiClassValueToShortNameV1Marker { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `BidiClassValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'bc' Unicode property

```rust
pub struct BidiClassValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> BidiClassValueToLongNameV1Marker { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **DataMarker**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
#### Struct `ScriptV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'sc' Unicode property

```rust
pub struct ScriptV1Marker;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **Default**
  - ```rust
    fn default() -> ScriptV1Marker { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
#### Struct `ScriptNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'sc' Unicode property

```rust
pub struct ScriptNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **KeyedDataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **DataMarker**
- **Unpin**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> ScriptNameToValueV1Marker { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `ScriptValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'sc' Unicode property

```rust
pub struct ScriptValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
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

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **KeyedDataMarker**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ScriptValueToShortNameV1Marker { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataMarker**
- **Freeze**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ScriptValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'sc' Unicode property

```rust
pub struct ScriptValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::ScriptValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ScriptValueToLongNameV1Marker { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **KeyedDataMarker**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataMarker**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `HangulSyllableTypeV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'hst' Unicode property

```rust
pub struct HangulSyllableTypeV1Marker;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **Sync**
- **Unpin**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HangulSyllableTypeV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataMarker**
- **KeyedDataMarker**
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

- **MaybeSendSync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> HangulSyllableTypeV1Marker { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `HangulSyllableTypeNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'hst' Unicode property

```rust
pub struct HangulSyllableTypeNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> HangulSyllableTypeNameToValueV1Marker { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **KeyedDataMarker**
- **MaybeSendSync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HangulSyllableTypeNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **DataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `HangulSyllableTypeValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'hst' Unicode property

```rust
pub struct HangulSyllableTypeValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HangulSyllableTypeValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

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
- **KeyedDataMarker**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Send**
- **DataMarker**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> HangulSyllableTypeValueToShortNameV1Marker { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `HangulSyllableTypeValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'hst' Unicode property

```rust
pub struct HangulSyllableTypeValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> HangulSyllableTypeValueToLongNameV1Marker { /* ... */ }
    ```

- **Unpin**
- **DataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::HangulSyllableTypeValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
- **Sync**
#### Struct `EastAsianWidthV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'ea' Unicode property

```rust
pub struct EastAsianWidthV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataMarker**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EastAsianWidthV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **KeyedDataMarker**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Default**
  - ```rust
    fn default() -> EastAsianWidthV1Marker { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
#### Struct `EastAsianWidthNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'ea' Unicode property

```rust
pub struct EastAsianWidthNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EastAsianWidthNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> EastAsianWidthNameToValueV1Marker { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **DataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
- **MaybeSendSync**
- **Unpin**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `EastAsianWidthValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'ea' Unicode property

```rust
pub struct EastAsianWidthValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EastAsianWidthValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EastAsianWidthValueToShortNameV1Marker { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **DataMarker**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **KeyedDataMarker**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **UnwindSafe**
- **Sync**
- **Unpin**
#### Struct `EastAsianWidthValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'ea' Unicode property

```rust
pub struct EastAsianWidthValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataMarker**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::EastAsianWidthValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EastAsianWidthValueToLongNameV1Marker { /* ... */ }
    ```

- **ErasedDestructor**
- **KeyedDataMarker**
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

- **Send**
- **Freeze**
- **Unpin**
#### Struct `LineBreakV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'lb' Unicode property

```rust
pub struct LineBreakV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataMarker**
- **MaybeSendSync**
- **Send**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **KeyedDataMarker**
- **Default**
  - ```rust
    fn default() -> LineBreakV1Marker { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
#### Struct `LineBreakNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'lb' Unicode property

```rust
pub struct LineBreakNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **KeyedDataMarker**
- **Default**
  - ```rust
    fn default() -> LineBreakNameToValueV1Marker { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
#### Struct `LineBreakValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'lb' Unicode property

```rust
pub struct LineBreakValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataMarker**
- **Default**
  - ```rust
    fn default() -> LineBreakValueToShortNameV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **KeyedDataMarker**
- **Sync**
- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `LineBreakValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'lb' Unicode property

```rust
pub struct LineBreakValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> LineBreakValueToLongNameV1Marker { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **DataMarker**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
#### Struct `GraphemeClusterBreakV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'GCB' Unicode property

```rust
pub struct GraphemeClusterBreakV1Marker;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **DataMarker**
- **ErasedDestructor**
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **KeyedDataMarker**
- **Sync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> GraphemeClusterBreakV1Marker { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `GraphemeClusterBreakNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'GCB' Unicode property

```rust
pub struct GraphemeClusterBreakNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> GraphemeClusterBreakNameToValueV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataMarker**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `GraphemeClusterBreakValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'GCB' Unicode property

```rust
pub struct GraphemeClusterBreakValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **KeyedDataMarker**
- **Freeze**
- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataMarker**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> GraphemeClusterBreakValueToShortNameV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `GraphemeClusterBreakValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'GCB' Unicode property

```rust
pub struct GraphemeClusterBreakValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> GraphemeClusterBreakValueToLongNameV1Marker { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **KeyedDataMarker**
- **DataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
#### Struct `WordBreakV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'WB' Unicode property

```rust
pub struct WordBreakV1Marker;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **MaybeSendSync**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **KeyedDataMarker**
- **UnwindSafe**
- **Send**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> WordBreakV1Marker { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `WordBreakNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'WB' Unicode property

```rust
pub struct WordBreakNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **KeyedDataMarker**
- **Sync**
- **Unpin**
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

- **Default**
  - ```rust
    fn default() -> WordBreakNameToValueV1Marker { /* ... */ }
    ```

- **DataMarker**
- **MaybeSendSync**
- **UnwindSafe**
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

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

#### Struct `WordBreakValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'WB' Unicode property

```rust
pub struct WordBreakValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Send**
- **KeyedDataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> WordBreakValueToShortNameV1Marker { /* ... */ }
    ```

#### Struct `WordBreakValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'WB' Unicode property

```rust
pub struct WordBreakValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> WordBreakValueToLongNameV1Marker { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **ErasedDestructor**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **KeyedDataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Unpin**
- **Send**
- **MaybeSendSync**
#### Struct `SentenceBreakV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'SB' Unicode property

```rust
pub struct SentenceBreakV1Marker;
```

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> SentenceBreakV1Marker { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **KeyedDataMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **DataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `SentenceBreakNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'SB' Unicode property

```rust
pub struct SentenceBreakNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataMarker**
- **KeyedDataMarker**
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

- **MaybeSendSync**
- **ErasedDestructor**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
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

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SentenceBreakNameToValueV1Marker { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Freeze**
- **Send**
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

#### Struct `SentenceBreakValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'SB' Unicode property

```rust
pub struct SentenceBreakValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SentenceBreakValueToShortNameV1Marker { /* ... */ }
    ```

- **Sync**
- **DataMarker**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **ErasedDestructor**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **KeyedDataMarker**
- **Send**
#### Struct `SentenceBreakValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'SB' Unicode property

```rust
pub struct SentenceBreakValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **KeyedDataMarker**
- **Default**
  - ```rust
    fn default() -> SentenceBreakValueToLongNameV1Marker { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **DataMarker**
- **Unpin**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
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

#### Struct `IndicSyllabicCategoryV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'InSC' Unicode property

```rust
pub struct IndicSyllabicCategoryV1Marker;
```

##### Implementations

###### Trait Implementations

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IndicSyllabicCategoryV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> IndicSyllabicCategoryV1Marker { /* ... */ }
    ```

- **ErasedDestructor**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

- **DataMarker**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **KeyedDataMarker**
- **RefUnwindSafe**
#### Struct `IndicSyllabicCategoryNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'InSC' Unicode property

```rust
pub struct IndicSyllabicCategoryNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **Freeze**
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

- **Sync**
- **Send**
- **KeyedDataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> IndicSyllabicCategoryNameToValueV1Marker { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IndicSyllabicCategoryNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **DataMarker**
#### Struct `IndicSyllabicCategoryValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'InSC' Unicode property

```rust
pub struct IndicSyllabicCategoryValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IndicSyllabicCategoryValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
- **Send**
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

- **RefUnwindSafe**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **KeyedDataMarker**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> IndicSyllabicCategoryValueToShortNameV1Marker { /* ... */ }
    ```

- **MaybeSendSync**
#### Struct `IndicSyllabicCategoryValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'InSC' Unicode property

```rust
pub struct IndicSyllabicCategoryValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **Send**
- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **KeyedDataMarker**
- **Default**
  - ```rust
    fn default() -> IndicSyllabicCategoryValueToLongNameV1Marker { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::IndicSyllabicCategoryValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
#### Struct `JoiningTypeV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for the 'jt' Unicode property

```rust
pub struct JoiningTypeV1Marker;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **Default**
  - ```rust
    fn default() -> JoiningTypeV1Marker { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoiningTypeV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **MaybeSendSync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DataMarker**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Sync**
- **KeyedDataMarker**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
#### Struct `JoiningTypeNameToValueV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for parsing the names of the values of the 'jt' Unicode property

```rust
pub struct JoiningTypeNameToValueV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoiningTypeNameToValueV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> JoiningTypeNameToValueV1Marker { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **DataMarker**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **KeyedDataMarker**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
#### Struct `JoiningTypeValueToShortNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing short names of the values of the 'jt' Unicode property

```rust
pub struct JoiningTypeValueToShortNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
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

- **Send**
- **DataMarker**
- **RefUnwindSafe**
- **MaybeSendSync**
- **Freeze**
- **Default**
  - ```rust
    fn default() -> JoiningTypeValueToShortNameV1Marker { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
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
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoiningTypeValueToShortNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **KeyedDataMarker**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `JoiningTypeValueToLongNameV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(databake :: Bake),
databake(path = icu_properties :: provider),)]`

Data marker for producing long names of the values of the 'jt' Unicode property

```rust
pub struct JoiningTypeValueToLongNameV1Marker;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> JoiningTypeValueToLongNameV1Marker { /* ... */ }
    ```

- **KeyedDataMarker**
- **Sync**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: icu_provider::DataRequest<''_>) -> Result<icu_provider::DataResponse<icu::properties::provider::JoiningTypeValueToLongNameV1Marker>, icu_provider::DataError> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **DataMarker**
- **Freeze**
### Constants and Statics

#### Constant `KEYS`

All data keys in this module.

```rust
pub const KEYS: &[DataKey] = _;
```

### Re-exports

#### Re-export `GeneralCategoryMaskNameToValueV1Marker`

```rust
pub use self::names::GeneralCategoryMaskNameToValueV1Marker;
```

## Module `script`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

Data and APIs for supporting both Script and Script_Extensions property
values in an efficient structure.

```rust
pub mod script { /* ... */ }
```

### Types

#### Struct `ScriptExtensionsSet`

A struct that wraps a [`Script`] array, such as in the return value for
[`get_script_extensions_val()`](ScriptWithExtensionsBorrowed::get_script_extensions_val).

```rust
pub struct ScriptExtensionsSet<''a> {
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
  pub fn contains(self: &Self, x: &Script) -> bool { /* ... */ }
  ```
  Returns whether this set contains the given script.

- ```rust
  pub fn iter(self: &Self) -> impl DoubleEndedIterator<Item = Script> + ''a { /* ... */ }
  ```
  Gets an iterator over the elements.

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ScriptExtensionsSet<''a> { /* ... */ }
    ```

- **Sync**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ScriptExtensionsSet<''a>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
#### Struct `ScriptWithExtensions`

A wrapper around script extensions data. Can be obtained via [`load_script_with_extensions_unstable()`] and
related getters.

Most useful methods are on [`ScriptWithExtensionsBorrowed`] obtained by calling [`ScriptWithExtensions::as_borrowed()`]

```rust
pub struct ScriptWithExtensions {
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
  pub fn as_borrowed(self: &Self) -> ScriptWithExtensionsBorrowed<''_> { /* ... */ }
  ```
  Construct a borrowed version of this type that can be queried.

- ```rust
  pub fn from_data(data: DataPayload<ScriptWithExtensionsPropertyV1Marker>) -> Self { /* ... */ }
  ```
  Construct a new one from loaded data

###### Trait Implementations

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

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Sync**
- **Send**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
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
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `ScriptWithExtensionsBorrowed`

A borrowed wrapper around script extension data, returned by
[`ScriptWithExtensions::as_borrowed()`]. More efficient to query.

```rust
pub struct ScriptWithExtensionsBorrowed<''a> {
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
  pub fn get_script_val(self: Self, code_point: u32) -> Script { /* ... */ }
  ```
  Returns the `Script` property value for this code point.

- ```rust
  pub fn get_script_extensions_val(self: Self, code_point: u32) -> ScriptExtensionsSet<''a> { /* ... */ }
  ```
  Return the `Script_Extensions` property value for this code point.

- ```rust
  pub fn has_script(self: Self, code_point: u32, script: Script) -> bool { /* ... */ }
  ```
  Returns whether `script` is contained in the Script_Extensions

- ```rust
  pub fn get_script_extensions_ranges(self: Self, script: Script) -> impl Iterator<Item = RangeInclusive<u32>> + ''a { /* ... */ }
  ```
  Returns all of the matching `CodePointMapRange`s for the given [`Script`]

- ```rust
  pub fn get_script_extensions_set(self: Self, script: Script) -> CodePointInversionList<''a> { /* ... */ }
  ```
  Returns a [`CodePointInversionList`] for the given [`Script`] which represents all

- ```rust
  pub const fn static_to_owned(self: Self) -> ScriptWithExtensions { /* ... */ }
  ```
  Cheaply converts a [`ScriptWithExtensionsBorrowed<'static>`] into a [`ScriptWithExtensions`].

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ScriptWithExtensionsBorrowed<''a> { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
- **Unpin**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

### Functions

#### Function `script_with_extensions`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Returns a [`ScriptWithExtensionsBorrowed`] struct that represents the data for the Script
and Script_Extensions properties.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Examples

```
use icu::properties::{script, Script};
let swe = script::script_with_extensions();

// get the `Script` property value
assert_eq!(swe.get_script_val(0x0640), Script::Common); // U+0640 ARABIC TATWEEL
assert_eq!(swe.get_script_val(0x0650), Script::Inherited); // U+0650 ARABIC KASRA
assert_eq!(swe.get_script_val(0x0660), Script::Arabic); // // U+0660 ARABIC-INDIC DIGIT ZERO
assert_eq!(swe.get_script_val(0xFDF2), Script::Arabic); // U+FDF2 ARABIC LIGATURE ALLAH ISOLATED FORM

// get the `Script_Extensions` property value
assert_eq!(
    swe.get_script_extensions_val(0x0640) // U+0640 ARABIC TATWEEL
        .iter().collect::<Vec<Script>>(),
    vec![Script::Arabic, Script::Syriac, Script::Mandaic, Script::Manichaean,
         Script::PsalterPahlavi, Script::Adlam, Script::HanifiRohingya, Script::Sogdian,
         Script::OldUyghur]
);
assert_eq!(
    swe.get_script_extensions_val('🥳' as u32) // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
        .iter().collect::<Vec<Script>>(),
    vec![Script::Common]
);
assert_eq!(
    swe.get_script_extensions_val(0x200D) // ZERO WIDTH JOINER
        .iter().collect::<Vec<Script>>(),
    vec![Script::Inherited]
);
assert_eq!(
    swe.get_script_extensions_val('௫' as u32) // U+0BEB TAMIL DIGIT FIVE
        .iter().collect::<Vec<Script>>(),
    vec![Script::Tamil, Script::Grantha]
);

// check containment of a `Script` value in the `Script_Extensions` value
// U+0650 ARABIC KASRA
assert!(!swe.has_script(0x0650, Script::Inherited)); // main Script value
assert!(swe.has_script(0x0650, Script::Arabic));
assert!(swe.has_script(0x0650, Script::Syriac));
assert!(!swe.has_script(0x0650, Script::Thaana));

// get a `CodePointInversionList` for when `Script` value is contained in `Script_Extensions` value
let syriac = swe.get_script_extensions_set(Script::Syriac);
assert!(syriac.contains32(0x0650)); // ARABIC KASRA
assert!(!syriac.contains32(0x0660)); // ARABIC-INDIC DIGIT ZERO
assert!(!syriac.contains32(0xFDF2)); // ARABIC LIGATURE ALLAH ISOLATED FORM
assert!(syriac.contains32(0x0700)); // SYRIAC END OF PARAGRAPH
assert!(syriac.contains32(0x074A)); // SYRIAC BARREKH
```

```rust
pub const fn script_with_extensions() -> ScriptWithExtensionsBorrowed<''static> { /* ... */ }
```

#### Function `load_script_with_extensions_with_any_provider`

A version of [`script_with_extensions`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_script_with_extensions_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized) -> Result<ScriptWithExtensions, crate::error::PropertiesError> { /* ... */ }
```

#### Function `load_script_with_extensions_unstable`

A version of [`script_with_extensions`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

[📚 Help choosing a constructor](icu_provider::constructors)

<div class="stab unstable">⚠️ The bounds on <tt>provider</tt> may change over time, including in SemVer minor releases.</div>

```rust
pub fn load_script_with_extensions_unstable</* synthetic */ impl DataProvider<ScriptWithExtensionsPropertyV1Marker> + ?Sized: DataProvider<ScriptWithExtensionsPropertyV1Marker> + ?Sized>(provider: &impl DataProvider<ScriptWithExtensionsPropertyV1Marker> + ?Sized) -> Result<ScriptWithExtensions, crate::error::PropertiesError> { /* ... */ }
```

## Module `sets`

The functions in this module return a [`CodePointSetData`] containing
the set of characters with a particular Unicode property.

The descriptions of most properties are taken from [`TR44`], the documentation for the
Unicode Character Database.  Some properties are instead defined in [`TR18`], the
documentation for Unicode regular expressions. In particular, Annex C of this document
defines properties for POSIX compatibility.

[`CodePointSetData`]: crate::sets::CodePointSetData
[`TR44`]: https://www.unicode.org/reports/tr44
[`TR18`]: https://www.unicode.org/reports/tr18

```rust
pub mod sets { /* ... */ }
```

### Types

#### Struct `CodePointSetData`

A wrapper around code point set data. It is returned by APIs that return Unicode
property data in a set-like form, ex: a set of code points sharing the same
value for a Unicode property. Access its data via the borrowed version,
[`CodePointSetDataBorrowed`].

```rust
pub struct CodePointSetData {
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
  pub fn as_borrowed(self: &Self) -> CodePointSetDataBorrowed<''_> { /* ... */ }
  ```
  Construct a borrowed version of this type that can be queried.

- ```rust
  pub fn from_data<M>(data: DataPayload<M>) -> Self
where
    M: DataMarker<Yokeable = PropertyCodePointSetV1<''static>> { /* ... */ }
  ```
  Construct a new one from loaded data

- ```rust
  pub fn from_code_point_inversion_list(set: CodePointInversionList<''static>) -> Self { /* ... */ }
  ```
  Construct a new owned [`CodePointInversionList`]

- ```rust
  pub fn as_code_point_inversion_list(self: &Self) -> Option<&CodePointInversionList<''_>> { /* ... */ }
  ```
  Convert this type to a [`CodePointInversionList`] as a borrowed value.

- ```rust
  pub fn to_code_point_inversion_list(self: &Self) -> CodePointInversionList<''_> { /* ... */ }
  ```
  Convert this type to a [`CodePointInversionList`], borrowing if possible,

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **ErasedDestructor**
- **RefUnwindSafe**
#### Struct `CodePointSetDataBorrowed`

A borrowed wrapper around code point set data, returned by
[`CodePointSetData::as_borrowed()`]. More efficient to query.

```rust
pub struct CodePointSetDataBorrowed<''a> {
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
  pub const fn static_to_owned(self: Self) -> CodePointSetData { /* ... */ }
  ```
  Cheaply converts a [`CodePointSetDataBorrowed<'static>`] into a [`CodePointSetData`].

- ```rust
  pub fn contains(self: Self, ch: char) -> bool { /* ... */ }
  ```
  Check if the set contains a character

- ```rust
  pub fn contains32(self: Self, ch: u32) -> bool { /* ... */ }
  ```
  Check if the set contains a character as a UTF32 code unit

- ```rust
  pub fn iter_ranges(self: Self) -> impl Iterator<Item = RangeInclusive<u32>> + ''a { /* ... */ }
  ```
  included in the [`CodePointSetData`]

- ```rust
  pub fn iter_ranges_complemented(self: Self) -> impl Iterator<Item = RangeInclusive<u32>> + ''a { /* ... */ }
  ```
  *not* included in the [`CodePointSetData`]

###### Trait Implementations

- **Sync**
- **Freeze**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
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

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CodePointSetDataBorrowed<''a> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
#### Struct `UnicodeSetData`

A wrapper around `UnicodeSet` data (characters and strings)

```rust
pub struct UnicodeSetData {
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
  pub fn as_borrowed(self: &Self) -> UnicodeSetDataBorrowed<''_> { /* ... */ }
  ```
  Construct a borrowed version of this type that can be queried.

- ```rust
  pub fn from_data<M>(data: DataPayload<M>) -> Self
where
    M: DataMarker<Yokeable = PropertyUnicodeSetV1<''static>> { /* ... */ }
  ```
  Construct a new one from loaded data

- ```rust
  pub fn from_code_point_inversion_list_string_list(set: CodePointInversionListAndStringList<''static>) -> Self { /* ... */ }
  ```
  Construct a new owned [`CodePointInversionListAndStringList`]

- ```rust
  pub fn as_code_point_inversion_list_string_list(self: &Self) -> Option<&CodePointInversionListAndStringList<''_>> { /* ... */ }
  ```
  Convert this type to a [`CodePointInversionListAndStringList`] as a borrowed value.

- ```rust
  pub fn to_code_point_inversion_list_string_list(self: &Self) -> CodePointInversionListAndStringList<''_> { /* ... */ }
  ```
  Convert this type to a [`CodePointInversionListAndStringList`], borrowing if possible,

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
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

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
#### Struct `UnicodeSetDataBorrowed`

A borrowed wrapper around code point set data, returned by
[`UnicodeSetData::as_borrowed()`]. More efficient to query.

```rust
pub struct UnicodeSetDataBorrowed<''a> {
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
  pub fn contains(self: Self, s: &str) -> bool { /* ... */ }
  ```
  Check if the set contains the string. Strings consisting of one character

- ```rust
  pub fn contains32(self: &Self, cp: u32) -> bool { /* ... */ }
  ```
  Check if the set contains a character as a UTF32 code unit

- ```rust
  pub fn contains_char(self: &Self, ch: char) -> bool { /* ... */ }
  ```
  Check if the set contains the code point corresponding to the Rust character.

- ```rust
  pub const fn static_to_owned(self: Self) -> UnicodeSetData { /* ... */ }
  ```
  Cheaply converts a [`UnicodeSetDataBorrowed<'static>`] into a [`UnicodeSetData`].

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnicodeSetDataBorrowed<''a> { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `load_ascii_hex_digit`

A version of [`ascii_hex_digit()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_ascii_hex_digit</* synthetic */ impl DataProvider<AsciiHexDigitV1Marker> + ?Sized: DataProvider<AsciiHexDigitV1Marker> + ?Sized>(provider: &impl DataProvider<AsciiHexDigitV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `ascii_hex_digit`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

ASCII characters commonly used for the representation of hexadecimal numbers

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let ascii_hex_digit = sets::ascii_hex_digit();

assert!(ascii_hex_digit.contains('3'));
assert!(!ascii_hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
assert!(ascii_hex_digit.contains('A'));
assert!(!ascii_hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
```

```rust
pub const fn ascii_hex_digit() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_alnum`

A version of [`alnum()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_alnum</* synthetic */ impl DataProvider<AlnumV1Marker> + ?Sized: DataProvider<AlnumV1Marker> + ?Sized>(provider: &impl DataProvider<AlnumV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `alnum`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters with the Alphabetic or Decimal_Number property
This is defined for POSIX compatibility.

```rust
pub const fn alnum() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_alphabetic`

A version of [`alphabetic()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_alphabetic</* synthetic */ impl DataProvider<AlphabeticV1Marker> + ?Sized: DataProvider<AlphabeticV1Marker> + ?Sized>(provider: &impl DataProvider<AlphabeticV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `alphabetic`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Alphabetic characters

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let alphabetic = sets::alphabetic();

assert!(!alphabetic.contains('3'));
assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
assert!(alphabetic.contains('A'));
assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
```

```rust
pub const fn alphabetic() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_bidi_control`

A version of [`bidi_control()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_bidi_control</* synthetic */ impl DataProvider<BidiControlV1Marker> + ?Sized: DataProvider<BidiControlV1Marker> + ?Sized>(provider: &impl DataProvider<BidiControlV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `bidi_control`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Format control characters which have specific functions in the Unicode Bidirectional
Algorithm

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let bidi_control = sets::bidi_control();

assert!(bidi_control.contains32(0x200F));  // RIGHT-TO-LEFT MARK
assert!(!bidi_control.contains('ش'));  // U+0634 ARABIC LETTER SHEEN
```

```rust
pub const fn bidi_control() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_bidi_mirrored`

A version of [`bidi_mirrored()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_bidi_mirrored</* synthetic */ impl DataProvider<BidiMirroredV1Marker> + ?Sized: DataProvider<BidiMirroredV1Marker> + ?Sized>(provider: &impl DataProvider<BidiMirroredV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `bidi_mirrored`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are mirrored in bidirectional text

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let bidi_mirrored = sets::bidi_mirrored();

assert!(bidi_mirrored.contains('['));
assert!(bidi_mirrored.contains(']'));
assert!(bidi_mirrored.contains('∑'));  // U+2211 N-ARY SUMMATION
assert!(!bidi_mirrored.contains('ཉ'));  // U+0F49 TIBETAN LETTER NYA
```

```rust
pub const fn bidi_mirrored() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_blank`

A version of [`blank()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_blank</* synthetic */ impl DataProvider<BlankV1Marker> + ?Sized: DataProvider<BlankV1Marker> + ?Sized>(provider: &impl DataProvider<BlankV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `blank`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Horizontal whitespace characters

```rust
pub const fn blank() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_cased`

A version of [`cased()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_cased</* synthetic */ impl DataProvider<CasedV1Marker> + ?Sized: DataProvider<CasedV1Marker> + ?Sized>(provider: &impl DataProvider<CasedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `cased`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Uppercase, lowercase, and titlecase characters

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let cased = sets::cased();

assert!(cased.contains('Ꙡ'));  // U+A660 CYRILLIC CAPITAL LETTER REVERSED TSE
assert!(!cased.contains('ދ'));  // U+078B THAANA LETTER DHAALU
```

```rust
pub const fn cased() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_case_ignorable`

A version of [`case_ignorable()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_case_ignorable</* synthetic */ impl DataProvider<CaseIgnorableV1Marker> + ?Sized: DataProvider<CaseIgnorableV1Marker> + ?Sized>(provider: &impl DataProvider<CaseIgnorableV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `case_ignorable`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters which are ignored for casing purposes

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let case_ignorable = sets::case_ignorable();

assert!(case_ignorable.contains(':'));
assert!(!case_ignorable.contains('λ'));  // U+03BB GREEK SMALL LETTER LAMDA
```

```rust
pub const fn case_ignorable() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_full_composition_exclusion`

A version of [`full_composition_exclusion()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_full_composition_exclusion</* synthetic */ impl DataProvider<FullCompositionExclusionV1Marker> + ?Sized: DataProvider<FullCompositionExclusionV1Marker> + ?Sized>(provider: &impl DataProvider<FullCompositionExclusionV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `full_composition_exclusion`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are excluded from composition
See <https://unicode.org/Public/UNIDATA/CompositionExclusions.txt>

```rust
pub const fn full_composition_exclusion() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_changes_when_casefolded`

A version of [`changes_when_casefolded()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_changes_when_casefolded</* synthetic */ impl DataProvider<ChangesWhenCasefoldedV1Marker> + ?Sized: DataProvider<ChangesWhenCasefoldedV1Marker> + ?Sized>(provider: &impl DataProvider<ChangesWhenCasefoldedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `changes_when_casefolded`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters whose normalized forms are not stable under case folding

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let changes_when_casefolded = sets::changes_when_casefolded();

assert!(changes_when_casefolded.contains('ß'));  // U+00DF LATIN SMALL LETTER SHARP S
assert!(!changes_when_casefolded.contains('ᜉ'));  // U+1709 TAGALOG LETTER PA
```

```rust
pub const fn changes_when_casefolded() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_changes_when_casemapped`

A version of [`changes_when_casemapped()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_changes_when_casemapped</* synthetic */ impl DataProvider<ChangesWhenCasemappedV1Marker> + ?Sized: DataProvider<ChangesWhenCasemappedV1Marker> + ?Sized>(provider: &impl DataProvider<ChangesWhenCasemappedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `changes_when_casemapped`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters which may change when they undergo case mapping

```rust
pub const fn changes_when_casemapped() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_changes_when_nfkc_casefolded`

A version of [`changes_when_nfkc_casefolded()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_changes_when_nfkc_casefolded</* synthetic */ impl DataProvider<ChangesWhenNfkcCasefoldedV1Marker> + ?Sized: DataProvider<ChangesWhenNfkcCasefoldedV1Marker> + ?Sized>(provider: &impl DataProvider<ChangesWhenNfkcCasefoldedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `changes_when_nfkc_casefolded`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters which are not identical to their NFKC_Casefold mapping

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let changes_when_nfkc_casefolded = sets::changes_when_nfkc_casefolded();

assert!(changes_when_nfkc_casefolded.contains('🄵'));  // U+1F135 SQUARED LATIN CAPITAL LETTER F
assert!(!changes_when_nfkc_casefolded.contains('f'));
```

```rust
pub const fn changes_when_nfkc_casefolded() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_changes_when_lowercased`

A version of [`changes_when_lowercased()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_changes_when_lowercased</* synthetic */ impl DataProvider<ChangesWhenLowercasedV1Marker> + ?Sized: DataProvider<ChangesWhenLowercasedV1Marker> + ?Sized>(provider: &impl DataProvider<ChangesWhenLowercasedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `changes_when_lowercased`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters whose normalized forms are not stable under a toLowercase mapping

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let changes_when_lowercased = sets::changes_when_lowercased();

assert!(changes_when_lowercased.contains('Ⴔ'));  // U+10B4 GEORGIAN CAPITAL LETTER PHAR
assert!(!changes_when_lowercased.contains('ფ'));  // U+10E4 GEORGIAN LETTER PHAR
```

```rust
pub const fn changes_when_lowercased() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_changes_when_titlecased`

A version of [`changes_when_titlecased()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_changes_when_titlecased</* synthetic */ impl DataProvider<ChangesWhenTitlecasedV1Marker> + ?Sized: DataProvider<ChangesWhenTitlecasedV1Marker> + ?Sized>(provider: &impl DataProvider<ChangesWhenTitlecasedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `changes_when_titlecased`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters whose normalized forms are not stable under a toTitlecase mapping

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let changes_when_titlecased = sets::changes_when_titlecased();

assert!(changes_when_titlecased.contains('æ'));  // U+00E6 LATIN SMALL LETTER AE
assert!(!changes_when_titlecased.contains('Æ'));  // U+00E6 LATIN CAPITAL LETTER AE
```

```rust
pub const fn changes_when_titlecased() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_changes_when_uppercased`

A version of [`changes_when_uppercased()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_changes_when_uppercased</* synthetic */ impl DataProvider<ChangesWhenUppercasedV1Marker> + ?Sized: DataProvider<ChangesWhenUppercasedV1Marker> + ?Sized>(provider: &impl DataProvider<ChangesWhenUppercasedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `changes_when_uppercased`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters whose normalized forms are not stable under a toUppercase mapping

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let changes_when_uppercased = sets::changes_when_uppercased();

assert!(changes_when_uppercased.contains('ւ'));  // U+0582 ARMENIAN SMALL LETTER YIWN
assert!(!changes_when_uppercased.contains('Ւ'));  // U+0552 ARMENIAN CAPITAL LETTER YIWN
```

```rust
pub const fn changes_when_uppercased() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_dash`

A version of [`dash()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_dash</* synthetic */ impl DataProvider<DashV1Marker> + ?Sized: DataProvider<DashV1Marker> + ?Sized>(provider: &impl DataProvider<DashV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `dash`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Punctuation characters explicitly called out as dashes in the Unicode Standard, plus
their compatibility equivalents

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let dash = sets::dash();

assert!(dash.contains('⸺'));  // U+2E3A TWO-EM DASH
assert!(dash.contains('-'));  // U+002D
assert!(!dash.contains('='));  // U+003D
```

```rust
pub const fn dash() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_deprecated`

A version of [`deprecated()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_deprecated</* synthetic */ impl DataProvider<DeprecatedV1Marker> + ?Sized: DataProvider<DeprecatedV1Marker> + ?Sized>(provider: &impl DataProvider<DeprecatedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `deprecated`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Deprecated characters. No characters will ever be removed from the standard, but the
usage of deprecated characters is strongly discouraged.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let deprecated = sets::deprecated();

assert!(deprecated.contains('ឣ'));  // U+17A3 KHMER INDEPENDENT VOWEL QAQ
assert!(!deprecated.contains('A'));
```

```rust
pub const fn deprecated() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_default_ignorable_code_point`

A version of [`default_ignorable_code_point()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_default_ignorable_code_point</* synthetic */ impl DataProvider<DefaultIgnorableCodePointV1Marker> + ?Sized: DataProvider<DefaultIgnorableCodePointV1Marker> + ?Sized>(provider: &impl DataProvider<DefaultIgnorableCodePointV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `default_ignorable_code_point`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

For programmatic determination of default ignorable code points.  New characters that
should be ignored in rendering (unless explicitly supported) will be assigned in these
ranges, permitting programs to correctly handle the default rendering of such
characters when not otherwise supported.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let default_ignorable_code_point = sets::default_ignorable_code_point();

assert!(default_ignorable_code_point.contains32(0x180B));  // MONGOLIAN FREE VARIATION SELECTOR ONE
assert!(!default_ignorable_code_point.contains('E'));
```

```rust
pub const fn default_ignorable_code_point() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_diacritic`

A version of [`diacritic()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_diacritic</* synthetic */ impl DataProvider<DiacriticV1Marker> + ?Sized: DataProvider<DiacriticV1Marker> + ?Sized>(provider: &impl DataProvider<DiacriticV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `diacritic`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that linguistically modify the meaning of another character to which they apply

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let diacritic = sets::diacritic();

assert!(diacritic.contains('\u{05B3}'));  // HEBREW POINT HATAF QAMATS
assert!(!diacritic.contains('א'));  // U+05D0 HEBREW LETTER ALEF
```

```rust
pub const fn diacritic() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_emoji_modifier_base`

A version of [`emoji_modifier_base()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_emoji_modifier_base</* synthetic */ impl DataProvider<EmojiModifierBaseV1Marker> + ?Sized: DataProvider<EmojiModifierBaseV1Marker> + ?Sized>(provider: &impl DataProvider<EmojiModifierBaseV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `emoji_modifier_base`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that can serve as a base for emoji modifiers

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let emoji_modifier_base = sets::emoji_modifier_base();

assert!(emoji_modifier_base.contains('✊'));  // U+270A RAISED FIST
assert!(!emoji_modifier_base.contains('⛰'));  // U+26F0 MOUNTAIN
```

```rust
pub const fn emoji_modifier_base() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_emoji_component`

A version of [`emoji_component()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_emoji_component</* synthetic */ impl DataProvider<EmojiComponentV1Marker> + ?Sized: DataProvider<EmojiComponentV1Marker> + ?Sized>(provider: &impl DataProvider<EmojiComponentV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `emoji_component`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters used in emoji sequences that normally do not appear on emoji keyboards as
separate choices, such as base characters for emoji keycaps

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let emoji_component = sets::emoji_component();

assert!(emoji_component.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T
assert!(emoji_component.contains32(0x20E3));  // COMBINING ENCLOSING KEYCAP
assert!(emoji_component.contains('7'));
assert!(!emoji_component.contains('T'));
```

```rust
pub const fn emoji_component() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_emoji_modifier`

A version of [`emoji_modifier()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_emoji_modifier</* synthetic */ impl DataProvider<EmojiModifierV1Marker> + ?Sized: DataProvider<EmojiModifierV1Marker> + ?Sized>(provider: &impl DataProvider<EmojiModifierV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `emoji_modifier`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are emoji modifiers

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let emoji_modifier = sets::emoji_modifier();

assert!(emoji_modifier.contains32(0x1F3FD));  // EMOJI MODIFIER FITZPATRICK TYPE-4
assert!(!emoji_modifier.contains32(0x200C));  // ZERO WIDTH NON-JOINER
```

```rust
pub const fn emoji_modifier() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_emoji`

A version of [`emoji()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_emoji</* synthetic */ impl DataProvider<EmojiV1Marker> + ?Sized: DataProvider<EmojiV1Marker> + ?Sized>(provider: &impl DataProvider<EmojiV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `emoji`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are emoji

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let emoji = sets::emoji();

assert!(emoji.contains('🔥'));  // U+1F525 FIRE
assert!(!emoji.contains('V'));
```

```rust
pub const fn emoji() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_emoji_presentation`

A version of [`emoji_presentation()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_emoji_presentation</* synthetic */ impl DataProvider<EmojiPresentationV1Marker> + ?Sized: DataProvider<EmojiPresentationV1Marker> + ?Sized>(provider: &impl DataProvider<EmojiPresentationV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `emoji_presentation`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that have emoji presentation by default

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let emoji_presentation = sets::emoji_presentation();

assert!(emoji_presentation.contains('🦬')); // U+1F9AC BISON
assert!(!emoji_presentation.contains('♻'));  // U+267B BLACK UNIVERSAL RECYCLING SYMBOL
```

```rust
pub const fn emoji_presentation() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_extender`

A version of [`extender()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_extender</* synthetic */ impl DataProvider<ExtenderV1Marker> + ?Sized: DataProvider<ExtenderV1Marker> + ?Sized>(provider: &impl DataProvider<ExtenderV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `extender`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters whose principal function is to extend the value of a preceding alphabetic
character or to extend the shape of adjacent characters.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let extender = sets::extender();

assert!(extender.contains('ヾ'));  // U+30FE KATAKANA VOICED ITERATION MARK
assert!(extender.contains('ー'));  // U+30FC KATAKANA-HIRAGANA PROLONGED SOUND MARK
assert!(!extender.contains('・'));  // U+30FB KATAKANA MIDDLE DOT
```

```rust
pub const fn extender() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_extended_pictographic`

A version of [`extended_pictographic()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_extended_pictographic</* synthetic */ impl DataProvider<ExtendedPictographicV1Marker> + ?Sized: DataProvider<ExtendedPictographicV1Marker> + ?Sized>(provider: &impl DataProvider<ExtendedPictographicV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `extended_pictographic`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Pictographic symbols, as well as reserved ranges in blocks largely associated with
emoji characters

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let extended_pictographic = sets::extended_pictographic();

assert!(extended_pictographic.contains('🥳')); // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
assert!(!extended_pictographic.contains('🇪'));  // U+1F1EA REGIONAL INDICATOR SYMBOL LETTER E
```

```rust
pub const fn extended_pictographic() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_graph`

A version of [`graph()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_graph</* synthetic */ impl DataProvider<GraphV1Marker> + ?Sized: DataProvider<GraphV1Marker> + ?Sized>(provider: &impl DataProvider<GraphV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `graph`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Visible characters.
This is defined for POSIX compatibility.

```rust
pub const fn graph() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_grapheme_base`

A version of [`grapheme_base()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_grapheme_base</* synthetic */ impl DataProvider<GraphemeBaseV1Marker> + ?Sized: DataProvider<GraphemeBaseV1Marker> + ?Sized>(provider: &impl DataProvider<GraphemeBaseV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `grapheme_base`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Property used together with the definition of Standard Korean Syllable Block to define
"Grapheme base". See D58 in Chapter 3, Conformance in the Unicode Standard.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let grapheme_base = sets::grapheme_base();

assert!(grapheme_base.contains('ക'));  // U+0D15 MALAYALAM LETTER KA
assert!(grapheme_base.contains('\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I
assert!(!grapheme_base.contains('\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA
```

```rust
pub const fn grapheme_base() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_grapheme_extend`

A version of [`grapheme_extend()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_grapheme_extend</* synthetic */ impl DataProvider<GraphemeExtendV1Marker> + ?Sized: DataProvider<GraphemeExtendV1Marker> + ?Sized>(provider: &impl DataProvider<GraphemeExtendV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `grapheme_extend`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Property used to define "Grapheme extender". See D59 in Chapter 3, Conformance in the
Unicode Standard.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let grapheme_extend = sets::grapheme_extend();

assert!(!grapheme_extend.contains('ക'));  // U+0D15 MALAYALAM LETTER KA
assert!(!grapheme_extend.contains('\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I
assert!(grapheme_extend.contains('\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA
```

```rust
pub const fn grapheme_extend() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_grapheme_link`

A version of [`grapheme_link()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_grapheme_link</* synthetic */ impl DataProvider<GraphemeLinkV1Marker> + ?Sized: DataProvider<GraphemeLinkV1Marker> + ?Sized>(provider: &impl DataProvider<GraphemeLinkV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `grapheme_link`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Deprecated property. Formerly proposed for programmatic determination of grapheme
cluster boundaries.

```rust
pub const fn grapheme_link() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_hex_digit`

A version of [`hex_digit()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_hex_digit</* synthetic */ impl DataProvider<HexDigitV1Marker> + ?Sized: DataProvider<HexDigitV1Marker> + ?Sized>(provider: &impl DataProvider<HexDigitV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `hex_digit`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters commonly used for the representation of hexadecimal numbers, plus their
compatibility equivalents

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let hex_digit = sets::hex_digit();

assert!(hex_digit.contains('0'));
assert!(!hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
assert!(hex_digit.contains('f'));
assert!(hex_digit.contains('ｆ'));  // U+FF46 FULLWIDTH LATIN SMALL LETTER F
assert!(hex_digit.contains('Ｆ'));  // U+FF26 FULLWIDTH LATIN CAPITAL LETTER F
assert!(!hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
```

```rust
pub const fn hex_digit() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_hyphen`

A version of [`hyphen()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_hyphen</* synthetic */ impl DataProvider<HyphenV1Marker> + ?Sized: DataProvider<HyphenV1Marker> + ?Sized>(provider: &impl DataProvider<HyphenV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `hyphen`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Deprecated property. Dashes which are used to mark connections between pieces of
words, plus the Katakana middle dot.

```rust
pub const fn hyphen() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_id_continue`

A version of [`id_continue()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_id_continue</* synthetic */ impl DataProvider<IdContinueV1Marker> + ?Sized: DataProvider<IdContinueV1Marker> + ?Sized>(provider: &impl DataProvider<IdContinueV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `id_continue`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that can come after the first character in an identifier. If using NFKC to
fold differences between characters, use [`load_xid_continue`] instead.  See
[`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
more details.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let id_continue = sets::id_continue();

assert!(id_continue.contains('x'));
assert!(id_continue.contains('1'));
assert!(id_continue.contains('_'));
assert!(id_continue.contains('ߝ'));  // U+07DD NKO LETTER FA
assert!(!id_continue.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
assert!(id_continue.contains32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
```

```rust
pub const fn id_continue() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_ideographic`

A version of [`ideographic()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_ideographic</* synthetic */ impl DataProvider<IdeographicV1Marker> + ?Sized: DataProvider<IdeographicV1Marker> + ?Sized>(provider: &impl DataProvider<IdeographicV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `ideographic`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters considered to be CJKV (Chinese, Japanese, Korean, and Vietnamese)
ideographs, or related siniform ideographs

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let ideographic = sets::ideographic();

assert!(ideographic.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD
assert!(!ideographic.contains('밥'));  // U+BC25 HANGUL SYLLABLE BAB
```

```rust
pub const fn ideographic() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_id_start`

A version of [`id_start()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_id_start</* synthetic */ impl DataProvider<IdStartV1Marker> + ?Sized: DataProvider<IdStartV1Marker> + ?Sized>(provider: &impl DataProvider<IdStartV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `id_start`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that can begin an identifier. If using NFKC to fold differences between
characters, use [`load_xid_start`] instead.  See [`Unicode Standard Annex
#31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let id_start = sets::id_start();

assert!(id_start.contains('x'));
assert!(!id_start.contains('1'));
assert!(!id_start.contains('_'));
assert!(id_start.contains('ߝ'));  // U+07DD NKO LETTER FA
assert!(!id_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
assert!(id_start.contains32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
```

```rust
pub const fn id_start() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_ids_binary_operator`

A version of [`ids_binary_operator()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_ids_binary_operator</* synthetic */ impl DataProvider<IdsBinaryOperatorV1Marker> + ?Sized: DataProvider<IdsBinaryOperatorV1Marker> + ?Sized>(provider: &impl DataProvider<IdsBinaryOperatorV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `ids_binary_operator`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters used in Ideographic Description Sequences

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let ids_binary_operator = sets::ids_binary_operator();

assert!(ids_binary_operator.contains32(0x2FF5));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE
assert!(!ids_binary_operator.contains32(0x3006));  // IDEOGRAPHIC CLOSING MARK
```

```rust
pub const fn ids_binary_operator() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_ids_trinary_operator`

A version of [`ids_trinary_operator()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_ids_trinary_operator</* synthetic */ impl DataProvider<IdsTrinaryOperatorV1Marker> + ?Sized: DataProvider<IdsTrinaryOperatorV1Marker> + ?Sized>(provider: &impl DataProvider<IdsTrinaryOperatorV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `ids_trinary_operator`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters used in Ideographic Description Sequences

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let ids_trinary_operator = sets::ids_trinary_operator();

assert!(ids_trinary_operator.contains32(0x2FF2));  // IDEOGRAPHIC DESCRIPTION CHARACTER LEFT TO MIDDLE AND RIGHT
assert!(ids_trinary_operator.contains32(0x2FF3));  // IDEOGRAPHIC DESCRIPTION CHARACTER ABOVE TO MIDDLE AND BELOW
assert!(!ids_trinary_operator.contains32(0x2FF4));
assert!(!ids_trinary_operator.contains32(0x2FF5));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE
assert!(!ids_trinary_operator.contains32(0x3006));  // IDEOGRAPHIC CLOSING MARK
```

```rust
pub const fn ids_trinary_operator() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_join_control`

A version of [`join_control()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_join_control</* synthetic */ impl DataProvider<JoinControlV1Marker> + ?Sized: DataProvider<JoinControlV1Marker> + ?Sized>(provider: &impl DataProvider<JoinControlV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `join_control`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Format control characters which have specific functions for control of cursive joining
and ligation

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let join_control = sets::join_control();

assert!(join_control.contains32(0x200C));  // ZERO WIDTH NON-JOINER
assert!(join_control.contains32(0x200D));  // ZERO WIDTH JOINER
assert!(!join_control.contains32(0x200E));
```

```rust
pub const fn join_control() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_logical_order_exception`

A version of [`logical_order_exception()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_logical_order_exception</* synthetic */ impl DataProvider<LogicalOrderExceptionV1Marker> + ?Sized: DataProvider<LogicalOrderExceptionV1Marker> + ?Sized>(provider: &impl DataProvider<LogicalOrderExceptionV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `logical_order_exception`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

A small number of spacing vowel letters occurring in certain Southeast Asian scripts such as Thai and Lao

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let logical_order_exception = sets::logical_order_exception();

assert!(logical_order_exception.contains('ແ'));  // U+0EC1 LAO VOWEL SIGN EI
assert!(!logical_order_exception.contains('ະ'));  // U+0EB0 LAO VOWEL SIGN A
```

```rust
pub const fn logical_order_exception() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_lowercase`

A version of [`lowercase()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_lowercase</* synthetic */ impl DataProvider<LowercaseV1Marker> + ?Sized: DataProvider<LowercaseV1Marker> + ?Sized>(provider: &impl DataProvider<LowercaseV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `lowercase`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Lowercase characters

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let lowercase = sets::lowercase();

assert!(lowercase.contains('a'));
assert!(!lowercase.contains('A'));
```

```rust
pub const fn lowercase() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_math`

A version of [`math()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_math</* synthetic */ impl DataProvider<MathV1Marker> + ?Sized: DataProvider<MathV1Marker> + ?Sized>(provider: &impl DataProvider<MathV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `math`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters used in mathematical notation

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let math = sets::math();

assert!(math.contains('='));
assert!(math.contains('+'));
assert!(!math.contains('-'));
assert!(math.contains('−'));  // U+2212 MINUS SIGN
assert!(!math.contains('/'));
assert!(math.contains('∕'));  // U+2215 DIVISION SLASH
```

```rust
pub const fn math() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_noncharacter_code_point`

A version of [`noncharacter_code_point()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_noncharacter_code_point</* synthetic */ impl DataProvider<NoncharacterCodePointV1Marker> + ?Sized: DataProvider<NoncharacterCodePointV1Marker> + ?Sized>(provider: &impl DataProvider<NoncharacterCodePointV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `noncharacter_code_point`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Code points permanently reserved for internal use

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let noncharacter_code_point = sets::noncharacter_code_point();

assert!(noncharacter_code_point.contains32(0xFDD0));
assert!(noncharacter_code_point.contains32(0xFFFF));
assert!(!noncharacter_code_point.contains32(0x10000));
```

```rust
pub const fn noncharacter_code_point() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_nfc_inert`

A version of [`nfc_inert()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_nfc_inert</* synthetic */ impl DataProvider<NfcInertV1Marker> + ?Sized: DataProvider<NfcInertV1Marker> + ?Sized>(provider: &impl DataProvider<NfcInertV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `nfc_inert`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are inert under NFC, i.e., they do not interact with adjacent characters

```rust
pub const fn nfc_inert() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_nfd_inert`

A version of [`nfd_inert()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_nfd_inert</* synthetic */ impl DataProvider<NfdInertV1Marker> + ?Sized: DataProvider<NfdInertV1Marker> + ?Sized>(provider: &impl DataProvider<NfdInertV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `nfd_inert`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are inert under NFD, i.e., they do not interact with adjacent characters

```rust
pub const fn nfd_inert() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_nfkc_inert`

A version of [`nfkc_inert()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_nfkc_inert</* synthetic */ impl DataProvider<NfkcInertV1Marker> + ?Sized: DataProvider<NfkcInertV1Marker> + ?Sized>(provider: &impl DataProvider<NfkcInertV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `nfkc_inert`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are inert under NFKC, i.e., they do not interact with adjacent characters

```rust
pub const fn nfkc_inert() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_nfkd_inert`

A version of [`nfkd_inert()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_nfkd_inert</* synthetic */ impl DataProvider<NfkdInertV1Marker> + ?Sized: DataProvider<NfkdInertV1Marker> + ?Sized>(provider: &impl DataProvider<NfkdInertV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `nfkd_inert`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are inert under NFKD, i.e., they do not interact with adjacent characters

```rust
pub const fn nfkd_inert() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_pattern_syntax`

A version of [`pattern_syntax()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_pattern_syntax</* synthetic */ impl DataProvider<PatternSyntaxV1Marker> + ?Sized: DataProvider<PatternSyntaxV1Marker> + ?Sized>(provider: &impl DataProvider<PatternSyntaxV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `pattern_syntax`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters used as syntax in patterns (such as regular expressions). See [`Unicode
Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
details.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let pattern_syntax = sets::pattern_syntax();

assert!(pattern_syntax.contains('{'));
assert!(pattern_syntax.contains('⇒'));  // U+21D2 RIGHTWARDS DOUBLE ARROW
assert!(!pattern_syntax.contains('0'));
```

```rust
pub const fn pattern_syntax() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_pattern_white_space`

A version of [`pattern_white_space()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_pattern_white_space</* synthetic */ impl DataProvider<PatternWhiteSpaceV1Marker> + ?Sized: DataProvider<PatternWhiteSpaceV1Marker> + ?Sized>(provider: &impl DataProvider<PatternWhiteSpaceV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `pattern_white_space`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters used as whitespace in patterns (such as regular expressions).  See
[`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
more details.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let pattern_white_space = sets::pattern_white_space();

assert!(pattern_white_space.contains(' '));
assert!(pattern_white_space.contains32(0x2029));  // PARAGRAPH SEPARATOR
assert!(pattern_white_space.contains32(0x000A));  // NEW LINE
assert!(!pattern_white_space.contains32(0x00A0));  // NO-BREAK SPACE
```

```rust
pub const fn pattern_white_space() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_prepended_concatenation_mark`

A version of [`prepended_concatenation_mark()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_prepended_concatenation_mark</* synthetic */ impl DataProvider<PrependedConcatenationMarkV1Marker> + ?Sized: DataProvider<PrependedConcatenationMarkV1Marker> + ?Sized>(provider: &impl DataProvider<PrependedConcatenationMarkV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `prepended_concatenation_mark`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

A small class of visible format controls, which precede and then span a sequence of
other characters, usually digits.

```rust
pub const fn prepended_concatenation_mark() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_print`

A version of [`print()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_print</* synthetic */ impl DataProvider<PrintV1Marker> + ?Sized: DataProvider<PrintV1Marker> + ?Sized>(provider: &impl DataProvider<PrintV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `print`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Printable characters (visible characters and whitespace).
This is defined for POSIX compatibility.

```rust
pub const fn print() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_quotation_mark`

A version of [`quotation_mark()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_quotation_mark</* synthetic */ impl DataProvider<QuotationMarkV1Marker> + ?Sized: DataProvider<QuotationMarkV1Marker> + ?Sized>(provider: &impl DataProvider<QuotationMarkV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `quotation_mark`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Punctuation characters that function as quotation marks.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let quotation_mark = sets::quotation_mark();

assert!(quotation_mark.contains('\''));
assert!(quotation_mark.contains('„'));  // U+201E DOUBLE LOW-9 QUOTATION MARK
assert!(!quotation_mark.contains('<'));
```

```rust
pub const fn quotation_mark() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_radical`

A version of [`radical()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_radical</* synthetic */ impl DataProvider<RadicalV1Marker> + ?Sized: DataProvider<RadicalV1Marker> + ?Sized>(provider: &impl DataProvider<RadicalV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `radical`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters used in the definition of Ideographic Description Sequences

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let radical = sets::radical();

assert!(radical.contains('⺆'));  // U+2E86 CJK RADICAL BOX
assert!(!radical.contains('丹'));  // U+F95E CJK COMPATIBILITY IDEOGRAPH-F95E
```

```rust
pub const fn radical() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_regional_indicator`

A version of [`regional_indicator()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_regional_indicator</* synthetic */ impl DataProvider<RegionalIndicatorV1Marker> + ?Sized: DataProvider<RegionalIndicatorV1Marker> + ?Sized>(provider: &impl DataProvider<RegionalIndicatorV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `regional_indicator`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Regional indicator characters, U+1F1E6..U+1F1FF

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let regional_indicator = sets::regional_indicator();

assert!(regional_indicator.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T
assert!(!regional_indicator.contains('Ⓣ'));  // U+24C9 CIRCLED LATIN CAPITAL LETTER T
assert!(!regional_indicator.contains('T'));
```

```rust
pub const fn regional_indicator() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_soft_dotted`

A version of [`soft_dotted()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_soft_dotted</* synthetic */ impl DataProvider<SoftDottedV1Marker> + ?Sized: DataProvider<SoftDottedV1Marker> + ?Sized>(provider: &impl DataProvider<SoftDottedV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `soft_dotted`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters with a "soft dot", like i or j. An accent placed on these characters causes
the dot to disappear.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let soft_dotted = sets::soft_dotted();

assert!(soft_dotted.contains('і'));  //U+0456 CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
assert!(!soft_dotted.contains('ı'));  // U+0131 LATIN SMALL LETTER DOTLESS I
```

```rust
pub const fn soft_dotted() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_segment_starter`

A version of [`segment_starter()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_segment_starter</* synthetic */ impl DataProvider<SegmentStarterV1Marker> + ?Sized: DataProvider<SegmentStarterV1Marker> + ?Sized>(provider: &impl DataProvider<SegmentStarterV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `segment_starter`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are starters in terms of Unicode normalization and combining character
sequences

```rust
pub const fn segment_starter() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_case_sensitive`

A version of [`case_sensitive()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_case_sensitive</* synthetic */ impl DataProvider<CaseSensitiveV1Marker> + ?Sized: DataProvider<CaseSensitiveV1Marker> + ?Sized>(provider: &impl DataProvider<CaseSensitiveV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `case_sensitive`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are either the source of a case mapping or in the target of a case
mapping

```rust
pub const fn case_sensitive() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_sentence_terminal`

A version of [`sentence_terminal()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_sentence_terminal</* synthetic */ impl DataProvider<SentenceTerminalV1Marker> + ?Sized: DataProvider<SentenceTerminalV1Marker> + ?Sized>(provider: &impl DataProvider<SentenceTerminalV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `sentence_terminal`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Punctuation characters that generally mark the end of sentences

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let sentence_terminal = sets::sentence_terminal();

assert!(sentence_terminal.contains('.'));
assert!(sentence_terminal.contains('?'));
assert!(sentence_terminal.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN
assert!(!sentence_terminal.contains(','));
assert!(!sentence_terminal.contains('¿'));  // U+00BF INVERTED QUESTION MARK
```

```rust
pub const fn sentence_terminal() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_terminal_punctuation`

A version of [`terminal_punctuation()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_terminal_punctuation</* synthetic */ impl DataProvider<TerminalPunctuationV1Marker> + ?Sized: DataProvider<TerminalPunctuationV1Marker> + ?Sized>(provider: &impl DataProvider<TerminalPunctuationV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `terminal_punctuation`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Punctuation characters that generally mark the end of textual units

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let terminal_punctuation = sets::terminal_punctuation();

assert!(terminal_punctuation.contains('.'));
assert!(terminal_punctuation.contains('?'));
assert!(terminal_punctuation.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN
assert!(terminal_punctuation.contains(','));
assert!(!terminal_punctuation.contains('¿'));  // U+00BF INVERTED QUESTION MARK
```

```rust
pub const fn terminal_punctuation() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_unified_ideograph`

A version of [`unified_ideograph()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_unified_ideograph</* synthetic */ impl DataProvider<UnifiedIdeographV1Marker> + ?Sized: DataProvider<UnifiedIdeographV1Marker> + ?Sized>(provider: &impl DataProvider<UnifiedIdeographV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `unified_ideograph`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

A property which specifies the exact set of Unified CJK Ideographs in the standard

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let unified_ideograph = sets::unified_ideograph();

assert!(unified_ideograph.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD
assert!(unified_ideograph.contains('木'));  // U+6728 CJK UNIFIED IDEOGRAPH-6728
assert!(!unified_ideograph.contains('𛅸'));  // U+1B178 NUSHU CHARACTER-1B178
```

```rust
pub const fn unified_ideograph() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_uppercase`

A version of [`uppercase()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_uppercase</* synthetic */ impl DataProvider<UppercaseV1Marker> + ?Sized: DataProvider<UppercaseV1Marker> + ?Sized>(provider: &impl DataProvider<UppercaseV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `uppercase`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Uppercase characters

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let uppercase = sets::uppercase();

assert!(uppercase.contains('U'));
assert!(!uppercase.contains('u'));
```

```rust
pub const fn uppercase() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_variation_selector`

A version of [`variation_selector()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_variation_selector</* synthetic */ impl DataProvider<VariationSelectorV1Marker> + ?Sized: DataProvider<VariationSelectorV1Marker> + ?Sized>(provider: &impl DataProvider<VariationSelectorV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `variation_selector`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that are Variation Selectors.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let variation_selector = sets::variation_selector();

assert!(variation_selector.contains32(0x180D));  // MONGOLIAN FREE VARIATION SELECTOR THREE
assert!(!variation_selector.contains32(0x303E));  // IDEOGRAPHIC VARIATION INDICATOR
assert!(variation_selector.contains32(0xFE0F));  // VARIATION SELECTOR-16
assert!(!variation_selector.contains32(0xFE10));  // PRESENTATION FORM FOR VERTICAL COMMA
assert!(variation_selector.contains32(0xE01EF));  // VARIATION SELECTOR-256
```

```rust
pub const fn variation_selector() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_white_space`

A version of [`white_space()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_white_space</* synthetic */ impl DataProvider<WhiteSpaceV1Marker> + ?Sized: DataProvider<WhiteSpaceV1Marker> + ?Sized>(provider: &impl DataProvider<WhiteSpaceV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `white_space`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Spaces, separator characters and other control characters which should be treated by
programming languages as "white space" for the purpose of parsing elements

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let white_space = sets::white_space();

assert!(white_space.contains(' '));
assert!(white_space.contains32(0x000A));  // NEW LINE
assert!(white_space.contains32(0x00A0));  // NO-BREAK SPACE
assert!(!white_space.contains32(0x200B));  // ZERO WIDTH SPACE
```

```rust
pub const fn white_space() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_xdigit`

A version of [`xdigit()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_xdigit</* synthetic */ impl DataProvider<XdigitV1Marker> + ?Sized: DataProvider<XdigitV1Marker> + ?Sized>(provider: &impl DataProvider<XdigitV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `xdigit`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Hexadecimal digits
This is defined for POSIX compatibility.

```rust
pub const fn xdigit() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_xid_continue`

A version of [`xid_continue()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_xid_continue</* synthetic */ impl DataProvider<XidContinueV1Marker> + ?Sized: DataProvider<XidContinueV1Marker> + ?Sized>(provider: &impl DataProvider<XidContinueV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `xid_continue`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that can come after the first character in an identifier.  See [`Unicode Standard Annex
#31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let xid_continue = sets::xid_continue();

assert!(xid_continue.contains('x'));
assert!(xid_continue.contains('1'));
assert!(xid_continue.contains('_'));
assert!(xid_continue.contains('ߝ'));  // U+07DD NKO LETTER FA
assert!(!xid_continue.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
assert!(!xid_continue.contains32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
```

```rust
pub const fn xid_continue() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_xid_start`

A version of [`xid_start()`] that uses custom data provided by a [`DataProvider`].

Note that this will return an owned version of the data. Functionality is available on
the borrowed version, accessible through [`CodePointSetData::as_borrowed`].

```rust
pub fn load_xid_start</* synthetic */ impl DataProvider<XidStartV1Marker> + ?Sized: DataProvider<XidStartV1Marker> + ?Sized>(provider: &impl DataProvider<XidStartV1Marker> + ?Sized) -> Result<CodePointSetData, PropertiesError> { /* ... */ }
```

#### Function `xid_start`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters that can begin an identifier. See [`Unicode
Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
details.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let xid_start = sets::xid_start();

assert!(xid_start.contains('x'));
assert!(!xid_start.contains('1'));
assert!(!xid_start.contains('_'));
assert!(xid_start.contains('ߝ'));  // U+07DD NKO LETTER FA
assert!(!xid_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
assert!(!xid_start.contains32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
```

```rust
pub const fn xid_start() -> CodePointSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_basic_emoji`

A version of [`basic_emoji()`] that uses custom data provided by a [`DataProvider`].

```rust
pub fn load_basic_emoji</* synthetic */ impl DataProvider<BasicEmojiV1Marker> + ?Sized: DataProvider<BasicEmojiV1Marker> + ?Sized>(provider: &impl DataProvider<BasicEmojiV1Marker> + ?Sized) -> Result<UnicodeSetData, PropertiesError> { /* ... */ }
```

#### Function `basic_emoji`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Characters and character sequences intended for general-purpose, independent, direct input.
See [`Unicode Technical Standard #51`](https://unicode.org/reports/tr51/) for more
details.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

# Example

```
use icu::properties::sets;

let basic_emoji = sets::basic_emoji();

assert!(!basic_emoji.contains32(0x0020));
assert!(!basic_emoji.contains_char('\n'));
assert!(basic_emoji.contains_char('🦃')); // U+1F983 TURKEY
assert!(basic_emoji.contains("\u{1F983}"));
assert!(basic_emoji.contains("\u{1F6E4}\u{FE0F}")); // railway track
assert!(!basic_emoji.contains("\u{0033}\u{FE0F}\u{20E3}"));  // Emoji_Keycap_Sequence, keycap 3
```

```rust
pub const fn basic_emoji() -> UnicodeSetDataBorrowed<''static> { /* ... */ }
```

#### Function `load_for_general_category_group`

A version of [`for_general_category_group()`] that uses custom data provided by a [`DataProvider`].

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_for_general_category_group</* synthetic */ impl DataProvider<GeneralCategoryV1Marker> + ?Sized: DataProvider<GeneralCategoryV1Marker> + ?Sized>(provider: &impl DataProvider<GeneralCategoryV1Marker> + ?Sized, enum_val: GeneralCategoryGroup) -> Result<CodePointSetData, crate::error::PropertiesError> { /* ... */ }
```

#### Function `for_general_category_group`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Return a [`CodePointSetData`] for a value or a grouping of values of the General_Category property. See [`GeneralCategoryGroup`].

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn for_general_category_group(enum_val: GeneralCategoryGroup) -> CodePointSetData { /* ... */ }
```

#### Function `load_for_ecma262`

**Attributes:**

- `#[<cfg>(feature = "compiled_data")]`

Returns a type capable of looking up values for a property specified as a string, as long as it is a
[binary property listed in ECMA-262][ecma], using strict matching on the names in the spec.

This handles every property required by ECMA-262 `/u` regular expressions, except for:

- `Script` and `General_Category`: handle these directly with [`maps::load_general_category()`] and
   [`maps::load_script()`].
   using property values parsed via [`GeneralCategory::get_name_to_enum_mapper()`] and [`Script::get_name_to_enum_mapper()`]
   if necessary.
- `Script_Extensions`: handle this directly using APIs from [`crate::script`], like [`script::load_script_with_extensions_unstable()`]
- `General_Category` mask values: Handle this alongside `General_Category` using [`GeneralCategoryGroup`],
   using property values parsed via [`GeneralCategoryGroup::get_name_to_enum_mapper()`] if necessary
- `Assigned`, `All`, and `ASCII` pseudoproperties: Handle these using their equivalent sets:
   - `Any` can be expressed as the range `[\u{0}-\u{10FFFF}]`
   - `Assigned` can be expressed as the inverse of the set `gc=Cn` (i.e., `\P{gc=Cn}`).
   - `ASCII` can be expressed as the range `[\u{0}-\u{7F}]`
- `General_Category` property values can themselves be treated like properties using a shorthand in ECMA262,
   simply create the corresponding `GeneralCategory` set.

✨ *Enabled with the `compiled_data` Cargo feature.*

[📚 Help choosing a constructor](icu_provider::constructors)

```
use icu::properties::sets;

let emoji = sets::load_for_ecma262("Emoji").expect("loading data failed");

assert!(emoji.contains('🔥')); // U+1F525 FIRE
assert!(!emoji.contains('V'));
```

[ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties

```rust
pub fn load_for_ecma262(name: &str) -> Result<CodePointSetDataBorrowed<''static>, crate::error::PropertiesError> { /* ... */ }
```

#### Function `load_for_ecma262_with_any_provider`

A version of [`load_for_ecma262`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

[📚 Help choosing a constructor](icu_provider::constructors)

```rust
pub fn load_for_ecma262_with_any_provider</* synthetic */ impl ::icu_provider::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized, name: &str) -> Result<CodePointSetData, crate::error::PropertiesError> { /* ... */ }
```

#### Function `load_for_ecma262_unstable`

A version of [`load_for_ecma262`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

[📚 Help choosing a constructor](icu_provider::constructors)

<div class="stab unstable">⚠️ The bounds on <tt>provider</tt> may change over time, including in SemVer minor releases.</div>

```rust
pub fn load_for_ecma262_unstable<P>(provider: &P, name: &str) -> Result<CodePointSetData, crate::error::PropertiesError>
where
    P: ?Sized + DataProvider<AsciiHexDigitV1Marker> + DataProvider<AlphabeticV1Marker> + DataProvider<BidiControlV1Marker> + DataProvider<BidiMirroredV1Marker> + DataProvider<CaseIgnorableV1Marker> + DataProvider<CasedV1Marker> + DataProvider<ChangesWhenCasefoldedV1Marker> + DataProvider<ChangesWhenCasemappedV1Marker> + DataProvider<ChangesWhenLowercasedV1Marker> + DataProvider<ChangesWhenNfkcCasefoldedV1Marker> + DataProvider<ChangesWhenTitlecasedV1Marker> + DataProvider<ChangesWhenUppercasedV1Marker> + DataProvider<DashV1Marker> + DataProvider<DefaultIgnorableCodePointV1Marker> + DataProvider<DeprecatedV1Marker> + DataProvider<DiacriticV1Marker> + DataProvider<EmojiV1Marker> + DataProvider<EmojiComponentV1Marker> + DataProvider<EmojiModifierV1Marker> + DataProvider<EmojiModifierBaseV1Marker> + DataProvider<EmojiPresentationV1Marker> + DataProvider<ExtendedPictographicV1Marker> + DataProvider<ExtenderV1Marker> + DataProvider<GraphemeBaseV1Marker> + DataProvider<GraphemeExtendV1Marker> + DataProvider<HexDigitV1Marker> + DataProvider<IdsBinaryOperatorV1Marker> + DataProvider<IdsTrinaryOperatorV1Marker> + DataProvider<IdContinueV1Marker> + DataProvider<IdStartV1Marker> + DataProvider<IdeographicV1Marker> + DataProvider<JoinControlV1Marker> + DataProvider<LogicalOrderExceptionV1Marker> + DataProvider<LowercaseV1Marker> + DataProvider<MathV1Marker> + DataProvider<NoncharacterCodePointV1Marker> + DataProvider<PatternSyntaxV1Marker> + DataProvider<PatternWhiteSpaceV1Marker> + DataProvider<QuotationMarkV1Marker> + DataProvider<RadicalV1Marker> + DataProvider<RegionalIndicatorV1Marker> + DataProvider<SentenceTerminalV1Marker> + DataProvider<SoftDottedV1Marker> + DataProvider<TerminalPunctuationV1Marker> + DataProvider<UnifiedIdeographV1Marker> + DataProvider<UppercaseV1Marker> + DataProvider<VariationSelectorV1Marker> + DataProvider<WhiteSpaceV1Marker> + DataProvider<XidContinueV1Marker> + DataProvider<XidStartV1Marker> { /* ... */ }
```

## Module `names`

Module for working with the names of property values

```rust
pub mod names { /* ... */ }
```

### Re-exports

#### Re-export `PropertyEnumToValueNameLinearMapper`

```rust
pub use crate::props::PropertyEnumToValueNameLinearMapper;
```

#### Re-export `PropertyEnumToValueNameLinearMapperBorrowed`

```rust
pub use crate::props::PropertyEnumToValueNameLinearMapperBorrowed;
```

#### Re-export `PropertyEnumToValueNameLinearTiny4Mapper`

```rust
pub use crate::props::PropertyEnumToValueNameLinearTiny4Mapper;
```

#### Re-export `PropertyEnumToValueNameLinearTiny4MapperBorrowed`

```rust
pub use crate::props::PropertyEnumToValueNameLinearTiny4MapperBorrowed;
```

#### Re-export `PropertyEnumToValueNameSparseMapper`

```rust
pub use crate::props::PropertyEnumToValueNameSparseMapper;
```

#### Re-export `PropertyEnumToValueNameSparseMapperBorrowed`

```rust
pub use crate::props::PropertyEnumToValueNameSparseMapperBorrowed;
```

#### Re-export `PropertyValueNameToEnumMapper`

```rust
pub use crate::props::PropertyValueNameToEnumMapper;
```

#### Re-export `PropertyValueNameToEnumMapperBorrowed`

```rust
pub use crate::props::PropertyValueNameToEnumMapperBorrowed;
```

## Re-exports

### Re-export `BidiClass`

```rust
pub use props::BidiClass;
```

### Re-export `CanonicalCombiningClass`

```rust
pub use props::CanonicalCombiningClass;
```

### Re-export `EastAsianWidth`

```rust
pub use props::EastAsianWidth;
```

### Re-export `GeneralCategory`

```rust
pub use props::GeneralCategory;
```

### Re-export `GeneralCategoryGroup`

```rust
pub use props::GeneralCategoryGroup;
```

### Re-export `GraphemeClusterBreak`

```rust
pub use props::GraphemeClusterBreak;
```

### Re-export `HangulSyllableType`

```rust
pub use props::HangulSyllableType;
```

### Re-export `IndicSyllabicCategory`

```rust
pub use props::IndicSyllabicCategory;
```

### Re-export `JoiningType`

```rust
pub use props::JoiningType;
```

### Re-export `LineBreak`

```rust
pub use props::LineBreak;
```

### Re-export `Script`

```rust
pub use props::Script;
```

### Re-export `SentenceBreak`

```rust
pub use props::SentenceBreak;
```

### Re-export `WordBreak`

```rust
pub use props::WordBreak;
```

### Re-export `PropertiesError`

```rust
pub use error::PropertiesError;
```

### Re-export `PropertiesError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use PropertiesError as Error;
```

