# Crate Documentation

**Version:** 1.5.0

**Format Version:** 43

# Module `icu_collections`

Efficient collections for Unicode data.

This module is published as its own crate ([`icu_collections`](https://docs.rs/icu_collections/latest/icu_collections/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

ICU4X [`CodePointTrie`](crate::codepointtrie::CodePointTrie) provides a read-only view of `CodePointTrie` data that is exported
from ICU4C. Detailed information about the design of the data structure can be found in the documentation
for the [`CodePointTrie`](crate::codepointtrie::CodePointTrie) struct.

ICU4X [`CodePointInversionList`](`crate::codepointinvlist::CodePointInversionList`) provides necessary functionality for highly efficient querying of sets of Unicode characters.
It is an implementation of the existing [ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).

ICU4X [`Char16Trie`](`crate::char16trie::Char16Trie`) provides a data structure for a space-efficient and time-efficient lookup of
sequences of 16-bit units (commonly but not necessarily UTF-16 code units)
which map to integer values.
It is an implementation of the existing [ICU4C UCharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UCharsTrie.html)
/ [ICU4J CharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/util/CharsTrie.html) API.

## Modules

## Module `char16trie`

This module provides a data structure for a space-efficient and time-efficient lookup of
sequences of 16-bit units (commonly but not necessarily UTF-16 code units)
which map to integer values.

It is an implementation of the existing [ICU4C UCharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UCharsTrie.html)
/ [ICU4J CharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/util/CharsTrie.html) API.

## Architecture

ICU4X [`Char16Trie`] is designed to provide a read-only view of `UCharsTrie` data that is exported from ICU4C.

## Examples

### Querying a `Char16Trie`

```rust
use icu::collections::char16trie::{Char16Trie, TrieResult};
use zerovec::ZeroVec;

// A Char16Trie containing the ASCII characters mapping 'a' to 1 and 'ab'
// to 100.
let trie_data = [48, 97, 176, 98, 32868];
let trie = Char16Trie::new(ZeroVec::from_slice_or_alloc(&trie_data));

let mut iter = trie.iter();
let res = iter.next('a');
assert_eq!(res, TrieResult::Intermediate(1));
let res = iter.next('b');
assert_eq!(res, TrieResult::FinalValue(100));
let res = iter.next('c');
assert_eq!(res, TrieResult::NoMatch);
```

[`ICU4X`]: ../icu/index.html

```rust
pub mod char16trie { /* ... */ }
```

### Re-exports

#### Re-export `Char16Trie`

```rust
pub use trie::Char16Trie;
```

#### Re-export `Char16TrieIterator`

```rust
pub use trie::Char16TrieIterator;
```

#### Re-export `TrieResult`

```rust
pub use trie::TrieResult;
```

## Module `codepointinvlist`

**Attributes:**

- `#[warn(missing_docs)]`

This module provides necessary functionality for highly efficient querying of sets of Unicode characters.

It is an implementation of the code point portion of the existing
[ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).

# Architecture
ICU4X [`CodePointInversionList`] is split up into independent levels, with [`CodePointInversionList`] representing the membership/query API,
and [`CodePointInversionListBuilder`] representing the builder API.

# Examples:

## Creating a `CodePointInversionList`

`CodePointSets` are created from either serialized [`CodePointSets`](CodePointInversionList),
represented by [inversion lists](http://userguide.icu-project.org/strings/properties),
the [`CodePointInversionListBuilder`], or from the Properties API.

```
use icu::collections::codepointinvlist::{
    CodePointInversionList, CodePointInversionListBuilder,
};

let mut builder = CodePointInversionListBuilder::new();
builder.add_range(&('A'..='Z'));
let set: CodePointInversionList = builder.build();

assert!(set.contains('A'));
```

## Querying a `CodePointInversionList`

Currently, you can check if a character/range of characters exists in the [`CodePointInversionList`], or iterate through the characters.

```
use icu::collections::codepointinvlist::{
    CodePointInversionList, CodePointInversionListBuilder,
};

let mut builder = CodePointInversionListBuilder::new();
builder.add_range(&('A'..='Z'));
let set: CodePointInversionList = builder.build();

assert!(set.contains('A'));
assert!(set.contains_range(&('A'..='C')));
assert_eq!(set.iter_chars().next(), Some('A'));
```

[`ICU4X`]: ../icu/index.html

```rust
pub mod codepointinvlist { /* ... */ }
```

### Types

#### Enum `CodePointInversionListError`

Custom Errors for [`CodePointInversionList`].

Re-exported as [`Error`].

```rust
pub enum CodePointInversionListError {
    InvalidSet(alloc::vec::Vec<u32>),
    InvalidRange(u32, u32),
}
```

##### Variants

###### `InvalidSet`

A CodePointInversionList was constructed with an invalid inversion list

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::vec::Vec<u32>` |  |

###### `InvalidRange`

A CodePointInversionList was constructed containing an invalid range

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |
| 1 | `u32` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
### Re-exports

#### Re-export `CodePointInversionListBuilder`

```rust
pub use builder::CodePointInversionListBuilder;
```

#### Re-export `CodePointInversionList`

```rust
pub use cpinvlist::CodePointInversionList;
```

#### Re-export `CodePointInversionListULE`

```rust
pub use cpinvlist::CodePointInversionListULE;
```

#### Re-export `CodePointInversionListError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use CodePointInversionListError as Error;
```

## Module `codepointinvliststringlist`

This module provides functionality for querying of sets of Unicode code points and strings.

It depends on [`CodePointInversionList`] to efficiently represent Unicode code points, while
it also maintains a list of strings in the set.

It is an implementation of the existing [ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).

```rust
pub mod codepointinvliststringlist { /* ... */ }
```

### Types

#### Struct `CodePointInversionListAndStringList`

A data structure providing a concrete implementation of a `UnicodeSet`
(which represents a set of code points and strings) using an inversion list for the code points and a simple
list-like structure to store and iterate over the strings.

```rust
pub struct CodePointInversionListAndStringList<''data> {
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
  pub fn try_from(cp_inv_list: CodePointInversionList<''data>, str_list: VarZeroVec<''data, str>) -> Result<Self, CodePointInversionListAndStringListError> { /* ... */ }
  ```
  Returns a new [`CodePointInversionListAndStringList`] from both a [`CodePointInversionList`] for the

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in this set (its cardinality).

- ```rust
  pub fn has_strings(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this set contains multi-code point strings or the empty string.

- ```rust
  pub fn contains(self: &Self, s: &str) -> bool { /* ... */ }
  ```

- ```rust
  pub fn contains32(self: &Self, cp: u32) -> bool { /* ... */ }
  ```

- ```rust
  pub fn contains_char(self: &Self, ch: char) -> bool { /* ... */ }
  ```

- ```rust
  pub fn code_points(self: &Self) -> &CodePointInversionList<''data> { /* ... */ }
  ```
  Access the underlying [`CodePointInversionList`].

- ```rust
  pub fn strings(self: &Self) -> &VarZeroSlice<str> { /* ... */ }
  ```
  Access the contained strings.

###### Trait Implementations

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **FromIterator**
  - ```rust
    fn from_iter<I>(it: I) -> Self
where
    I: IntoIterator<Item = &''a str> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CodePointInversionListAndStringList<''data>) -> bool { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf CodePointInversionListAndStringList<''zf_inner>) -> Self { /* ... */ }
    ```

  - ```rust
    fn zero_from(other: &''data CodePointInversionListAndStringListULE) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CodePointInversionListAndStringList<''data> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

#### Struct `CodePointInversionListAndStringListULE`

**Attributes:**

- `#[allow(missing_docs)]`
- `#[repr(C, packed(1))]`

[`VarULE`](zerovec::ule::VarULE) type for [`CodePointInversionListAndStringList`]. See [`CodePointInversionListAndStringList`] for documentation.

```rust
pub struct CodePointInversionListAndStringListULE {
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
  pub fn cp_inv_list<''a>(self: &''a Self) -> &''a CodePointInversionListULE { /* ... */ }
  ```
  Access the VarULE type behind the unsized `cp_inv_list` field

- ```rust
  pub fn str_list<''a>(self: &''a Self) -> &''a zerovec::VarZeroSlice<str> { /* ... */ }
  ```
  Access the VarULE type behind the unsized `str_list` field

###### Trait Implementations

- **ZeroFrom**
  - ```rust
    fn zero_from(other: &''data CodePointInversionListAndStringListULE) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **Sized**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VarULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> { /* ... */ }
    ```

  - ```rust
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self { /* ... */ }
    ```

- **ZeroMapKV**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

#### Enum `CodePointInversionListAndStringListError`

Custom Errors for [`CodePointInversionListAndStringList`].

Re-exported as [`Error`].

```rust
pub enum CodePointInversionListAndStringListError {
    InvalidCodePointInversionList(crate::codepointinvlist::CodePointInversionListError),
    InvalidStringLength(alloc::string::String),
    StringListNotUnique(alloc::string::String),
    StringListNotSorted(alloc::string::String, alloc::string::String),
}
```

##### Variants

###### `InvalidCodePointInversionList`

An invalid CodePointInversionList was constructed

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::codepointinvlist::CodePointInversionListError` |  |

###### `InvalidStringLength`

A string in the string list had an invalid length

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `StringListNotUnique`

A string in the string list appears more than once

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `StringListNotSorted`

Two strings in the string list compare to each other opposite of sorted order

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
### Re-exports

#### Re-export `CodePointInversionListAndStringListError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use CodePointInversionListAndStringListError as Error;
```

## Module `codepointtrie`

This module provides a data structure for an time-efficient lookup of values
associated to code points.

It is an implementation of the existing [ICU4C UCPTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/ucptrie_8h.html)
/ [ICU4J CodePointTrie](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4j/) API.

# Architecture

ICU4X [`CodePointTrie`] is designed to provide a read-only view of [`CodePointTrie`] data that is exported
from ICU4C. Detailed information about the design of the data structure can be found in the documentation
for the [`CodePointTrie`] struct.

# Examples

## Querying a `CodePointTrie`

```
use icu::collections::codepointtrie::planes;
let trie = planes::get_planes_trie();

assert_eq!(0, trie.get32(0x41)); // 'A' as u32
assert_eq!(0, trie.get32(0x13E0)); // 'Ꮰ' as u32
assert_eq!(1, trie.get32(0x10044)); // '𐁄' as u32
```

[`ICU4X`]: ../icu/index.html

```rust
pub mod codepointtrie { /* ... */ }
```

### Modules

## Module `planes`

Sample data for [`CodePointTrie`] that returns the code point's plane number.

```rust
pub mod planes { /* ... */ }
```

### Functions

#### Function `get_planes_trie`

Return a [`CodePointTrie`] that returns the Unicode plane number, an
integer from 0-16 inclusive, for each code point. This `CodePointTrie`
does not actually represent any Unicode property, but it is provided in
case it is useful to users of `CodePointTrie` for testing or other
purposes. See <https://www.unicode.org/glossary/#plane>.

```rust
pub fn get_planes_trie() -> CodePointTrie<''static, u8> { /* ... */ }
```

### Re-exports

#### Re-export `CodePointMapRange`

```rust
pub use cptrie::CodePointMapRange;
```

#### Re-export `CodePointMapRangeIterator`

```rust
pub use cptrie::CodePointMapRangeIterator;
```

#### Re-export `CodePointTrie`

```rust
pub use cptrie::CodePointTrie;
```

#### Re-export `CodePointTrieHeader`

```rust
pub use cptrie::CodePointTrieHeader;
```

#### Re-export `TrieType`

```rust
pub use cptrie::TrieType;
```

#### Re-export `TrieValue`

```rust
pub use cptrie::TrieValue;
```

#### Re-export `Error`

```rust
pub use error::Error as CodePointTrieError;
```

#### Re-export `CodePointTrieError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use CodePointTrieError as Error;
```

