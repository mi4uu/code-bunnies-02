# Crate Documentation

**Version:** 1.5.0

**Format Version:** 43

# Module `icu_locid`

Parsing, manipulating, and serializing Unicode Language and Locale Identifiers.

This module is published as its own crate ([`icu_locid`](https://docs.rs/icu_locid/latest/icu_locid/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

The module provides algorithms for parsing a string into a well-formed language or locale identifier
as defined by [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`].

[`Locale`] is the most common structure to use for storing information about a language,
script, region, variants and extensions. In almost all cases, this struct should be used as the
base unit for all locale management operations.

[`LanguageIdentifier`] is a strict subset of [`Locale`] which can be useful in a narrow range of
cases where [`Unicode Extensions`] are not relevant.

If in doubt, use [`Locale`].

# Examples

```
use icu::locid::Locale;
use icu::locid::{
    locale,
    subtags::{language, region},
};

let mut loc: Locale = locale!("en-US");

assert_eq!(loc.id.language, language!("en"));
assert_eq!(loc.id.script, None);
assert_eq!(loc.id.region, Some(region!("US")));
assert_eq!(loc.id.variants.len(), 0);

loc.id.region = Some(region!("GB"));

assert_eq!(loc, locale!("en-GB"));
```

For more details, see [`Locale`] and [`LanguageIdentifier`].

[`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]: https://unicode.org/reports/tr35/tr35.html#Unicode_Language_and_Locale_Identifiers
[`ICU4X`]: ../icu/index.html
[`Unicode Extensions`]: extensions

## Modules

## Module `extensions`

Unicode Extensions provide a mechanism to extend the [`LanguageIdentifier`] with
additional bits of information - a combination of a [`LanguageIdentifier`] and [`Extensions`]
is called [`Locale`].

There are four types of extensions:

 * [`Unicode Extensions`] - marked as `u`.
 * [`Transform Extensions`] - marked as `t`.
 * [`Private Use Extensions`] - marked as `x`.
 * [`Other Extensions`] - marked as any `a-z` except of `u`, `t` and `x`.

One can think of extensions as a bag of extra information on top of basic 4 [`subtags`].

Notice: `Other` extension type is currently not supported.

# Examples

```
use icu::locid::extensions::unicode::{Key, Value};
use icu::locid::Locale;

let loc: Locale = "en-US-u-ca-buddhist-t-en-us-h0-hybrid-x-foo"
    .parse()
    .expect("Failed to parse.");

assert_eq!(loc.id.language, "en".parse().unwrap());
assert_eq!(loc.id.script, None);
assert_eq!(loc.id.region, Some("US".parse().unwrap()));
assert_eq!(loc.id.variants.len(), 0);

let key: Key = "ca".parse().expect("Parsing key failed.");
let value: Value = "buddhist".parse().expect("Parsing value failed.");
assert_eq!(loc.extensions.unicode.keywords.get(&key), Some(&value));
```

[`LanguageIdentifier`]: super::LanguageIdentifier
[`Locale`]: super::Locale
[`subtags`]: super::subtags
[`Other Extensions`]: other
[`Private Use Extensions`]: private
[`Transform Extensions`]: transform
[`Unicode Extensions`]: unicode

```rust
pub mod extensions { /* ... */ }
```

### Modules

## Module `other`

Other Use Extensions is a list of extensions other than unicode,
transform or private.

Those extensions are treated as a pass-through, and no Unicode related
behavior depends on them.

The main struct for this extension is [`Other`] which is a list of [`Subtag`]s.

# Examples

```
use icu::locid::extensions::other::Other;
use icu::locid::Locale;

let mut loc: Locale = "en-US-a-foo-faa".parse().expect("Parsing failed.");
```

```rust
pub mod other { /* ... */ }
```

### Types

#### Struct `Other`

A list of [`Other Use Extensions`] as defined in [`Unicode Locale
Identifier`] specification.

Those extensions are treated as a pass-through, and no Unicode related
behavior depends on them.

# Examples

```
use icu::locid::extensions::other::{Other, Subtag};

let subtag1: Subtag = "foo".parse().expect("Failed to parse a Subtag.");
let subtag2: Subtag = "bar".parse().expect("Failed to parse a Subtag.");

let other = Other::from_vec_unchecked(b'a', vec![subtag1, subtag2]);
assert_eq!(&other.to_string(), "a-foo-bar");
```

[`Other Use Extensions`]: https://unicode.org/reports/tr35/#other_extensions
[`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier

```rust
pub struct Other {
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
  pub fn from_vec_unchecked(ext: u8, keys: Vec<Subtag>) -> Self { /* ... */ }
  ```
  A constructor which takes a pre-sorted list of [`Subtag`].

- ```rust
  pub fn get_ext_str(self: &Self) -> &str { /* ... */ }
  ```
  Gets the tag character for this extension as a &str.

- ```rust
  pub fn get_ext(self: &Self) -> char { /* ... */ }
  ```
  Gets the tag character for this extension as a char.

- ```rust
  pub fn get_ext_byte(self: &Self) -> u8 { /* ... */ }
  ```
  Gets the tag character for this extension as a byte.

###### Trait Implementations

- **Send**
- **Freeze**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Other { /* ... */ }
    ```

- **StructuralPartialEq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
- **Writeable**
  - ```rust
    fn write_to<W: core::fmt::Write + ?Sized>(self: &Self, sink: &mut W) -> core::fmt::Result { /* ... */ }
    ```

  - ```rust
    fn writeable_length_hint(self: &Self) -> writeable::LengthHint { /* ... */ }
    ```

  - ```rust
    fn write_to_string(self: &Self) -> alloc::borrow::Cow<''_, str> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Other { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Other) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Other) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Other) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

### Re-exports

#### Re-export `Subtag`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use subtag::Subtag;
```

## Module `private`

Private Use Extensions is a list of extensions intended for
private use.

Those extensions are treated as a pass-through, and no Unicode related
behavior depends on them.

The main struct for this extension is [`Private`] which is a list of [`Subtag`]s.

# Examples

```
use icu::locid::extensions::private::subtag;
use icu::locid::{locale, Locale};

let mut loc: Locale = "en-US-x-foo-faa".parse().expect("Parsing failed.");

assert!(loc.extensions.private.contains(&subtag!("foo")));
assert_eq!(loc.extensions.private.iter().next(), Some(&subtag!("foo")));

loc.extensions.private.clear();

assert!(loc.extensions.private.is_empty());
assert_eq!(loc, locale!("en-US"));
```

```rust
pub mod private { /* ... */ }
```

### Types

#### Struct `Private`

A list of [`Private Use Extensions`] as defined in [`Unicode Locale
Identifier`] specification.

Those extensions are treated as a pass-through, and no Unicode related
behavior depends on them.

# Examples

```
use icu::locid::extensions::private::{Private, Subtag};

let subtag1: Subtag = "foo".parse().expect("Failed to parse a Subtag.");
let subtag2: Subtag = "bar".parse().expect("Failed to parse a Subtag.");

let private = Private::from_vec_unchecked(vec![subtag1, subtag2]);
assert_eq!(&private.to_string(), "x-foo-bar");
```

[`Private Use Extensions`]: https://unicode.org/reports/tr35/#pu_extensions
[`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier

```rust
pub struct Private(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Returns a new empty list of private-use extensions. Same as [`default()`](Default::default()), but is `const`.

- ```rust
  pub fn from_vec_unchecked(input: Vec<Subtag>) -> Self { /* ... */ }
  ```
  A constructor which takes a pre-sorted list of [`Subtag`].

- ```rust
  pub const fn new_single(input: Subtag) -> Self { /* ... */ }
  ```
  A constructor which takes a single [`Subtag`].

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Empties the [`Private`] list.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Private { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Private { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Writeable**
  - ```rust
    fn write_to<W: core::fmt::Write + ?Sized>(self: &Self, sink: &mut W) -> core::fmt::Result { /* ... */ }
    ```

  - ```rust
    fn writeable_length_hint(self: &Self) -> writeable::LengthHint { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Sync**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Private) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Receiver**
- **Unpin**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Private) -> bool { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Private) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Re-exports

#### Re-export `Subtag`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use other::Subtag;
```

## Module `transform`

Transform Extensions provide information on content transformations in a given locale.

The main struct for this extension is [`Transform`] which contains [`Fields`] and an
optional [`LanguageIdentifier`].

[`LanguageIdentifier`]: super::super::LanguageIdentifier

# Examples

```
use icu::locid::extensions::transform::{Fields, Key, Transform, Value};
use icu::locid::{LanguageIdentifier, Locale};

let mut loc: Locale =
    "en-US-t-es-ar-h0-hybrid".parse().expect("Parsing failed.");

let lang: LanguageIdentifier =
    "es-AR".parse().expect("Parsing LanguageIdentifier failed.");

let key: Key = "h0".parse().expect("Parsing key failed.");
let value: Value = "hybrid".parse().expect("Parsing value failed.");

assert_eq!(loc.extensions.transform.lang, Some(lang));
assert!(loc.extensions.transform.fields.contains_key(&key));
assert_eq!(loc.extensions.transform.fields.get(&key), Some(&value));

assert_eq!(&loc.extensions.transform.to_string(), "t-es-ar-h0-hybrid");
```

```rust
pub mod transform { /* ... */ }
```

### Types

#### Struct `Transform`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

A list of [`Unicode BCP47 T Extensions`] as defined in [`Unicode Locale
Identifier`] specification.

Transform extension carries information about source language or script of
transformed content, including content that has been transliterated, transcribed,
or translated, or in some other way influenced by the source (See [`RFC 6497`] for details).

# Examples

```
use icu::locid::extensions::transform::{Key, Value};
use icu::locid::{LanguageIdentifier, Locale};

let mut loc: Locale =
    "de-t-en-us-h0-hybrid".parse().expect("Parsing failed.");

let en_us: LanguageIdentifier = "en-US".parse().expect("Parsing failed.");

assert_eq!(loc.extensions.transform.lang, Some(en_us));
let key: Key = "h0".parse().expect("Parsing key failed.");
let value: Value = "hybrid".parse().expect("Parsing value failed.");
assert_eq!(loc.extensions.transform.fields.get(&key), Some(&value));
```
[`Unicode BCP47 T Extensions`]: https://unicode.org/reports/tr35/#t_Extension
[`RFC 6497`]: https://www.ietf.org/rfc/rfc6497.txt
[`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier

```rust
pub struct Transform {
    pub lang: Option<crate::LanguageIdentifier>,
    pub fields: Fields,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `lang` | `Option<crate::LanguageIdentifier>` | The [`LanguageIdentifier`] specified with this locale extension, or `None` if not present. |
| `fields` | `Fields` | The key-value pairs present in this locale extension, with each extension key subtag<br>associated to its provided value subtag. |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Returns a new empty map of Transform extensions. Same as [`default()`](Default::default()), but is `const`.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if there are no tfields and no tlang in the `TransformExtensionList`.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the transform extension, effectively removing it from the locale.

- ```rust
  pub fn total_cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
  ```
  Returns an ordering suitable for use in [`BTreeSet`].

###### Trait Implementations

- **Send**
- **ErasedDestructor**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Transform { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Transform) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Transform { /* ... */ }
    ```

- **StructuralPartialEq**
- **Writeable**
  - ```rust
    fn write_to<W: core::fmt::Write + ?Sized>(self: &Self, sink: &mut W) -> core::fmt::Result { /* ... */ }
    ```

  - ```rust
    fn writeable_length_hint(self: &Self) -> writeable::LengthHint { /* ... */ }
    ```

- **Eq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **UnwindSafe**
### Re-exports

#### Re-export `Fields`

```rust
pub use fields::Fields;
```

#### Re-export `Key`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use key::Key;
```

#### Re-export `Value`

```rust
pub use value::Value;
```

## Module `unicode`

Unicode Extensions provide information about user preferences in a given locale.

The main struct for this extension is [`Unicode`] which contains [`Keywords`] and
[`Attributes`].


# Examples

```
use icu::locid::extensions::unicode::{attribute, key, value, Unicode};
use icu::locid::Locale;

let loc: Locale = "en-US-u-foobar-hc-h12".parse().expect("Parsing failed.");

assert_eq!(
    loc.extensions.unicode.keywords.get(&key!("hc")),
    Some(&value!("h12"))
);
assert!(loc
    .extensions
    .unicode
    .attributes
    .contains(&attribute!("foobar")));
```

```rust
pub mod unicode { /* ... */ }
```

### Types

#### Struct `Unicode`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

Unicode Extensions provide information about user preferences in a given locale.

A list of [`Unicode BCP47 U Extensions`] as defined in [`Unicode Locale
Identifier`] specification.

Unicode extensions provide subtags that specify language and/or locale-based behavior
or refinements to language tags, according to work done by the Unicode Consortium.
(See [`RFC 6067`] for details).

[`Unicode BCP47 U Extensions`]: https://unicode.org/reports/tr35/#u_Extension
[`RFC 6067`]: https://www.ietf.org/rfc/rfc6067.txt
[`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier

# Examples

```
use icu::locid::extensions::unicode::{key, value};
use icu::locid::Locale;

let loc: Locale =
    "de-u-hc-h12-ca-buddhist".parse().expect("Parsing failed.");

assert_eq!(
    loc.extensions.unicode.keywords.get(&key!("ca")),
    Some(&value!("buddhist"))
);
```

```rust
pub struct Unicode {
    pub keywords: Keywords,
    pub attributes: Attributes,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `keywords` | `Keywords` | The key-value pairs present in this locale extension, with each extension key subtag<br>associated to its provided value subtag. |
| `attributes` | `Attributes` | A canonically ordered sequence of single standalone subtags for this locale extension. |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Returns a new empty map of Unicode extensions. Same as [`default()`](Default::default()), but is `const`.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns [`true`] if there list of keywords and attributes is empty.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears all Unicode extension keywords and attributes, effectively removing

- ```rust
  pub fn total_cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
  ```
  Returns an ordering suitable for use in [`BTreeSet`].

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Unicode) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Unicode { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Unicode) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Writeable**
  - ```rust
    fn write_to<W: core::fmt::Write + ?Sized>(self: &Self, sink: &mut W) -> core::fmt::Result { /* ... */ }
    ```

  - ```rust
    fn writeable_length_hint(self: &Self) -> writeable::LengthHint { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Unicode { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Unicode) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Re-exports

#### Re-export `Attribute`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use attribute::Attribute;
```

#### Re-export `Attributes`

```rust
pub use attributes::Attributes;
```

#### Re-export `Key`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use key::Key;
```

#### Re-export `Keywords`

```rust
pub use keywords::Keywords;
```

#### Re-export `Value`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use value::Value;
```

### Types

#### Enum `ExtensionType`

**Attributes:**

- `#[non_exhaustive]`

Defines the type of extension.

```rust
pub enum ExtensionType {
    Transform,
    Unicode,
    Private,
    Other(u8),
}
```

##### Variants

###### `Transform`

Transform Extension Type marked as `t`.

###### `Unicode`

Unicode Extension Type marked as `u`.

###### `Private`

Private Extension Type marked as `x`.

###### `Other`

All other extension types.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExtensionType { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ExtensionType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Copy**
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **Unpin**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ExtensionType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExtensionType) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
#### Struct `Extensions`

**Attributes:**

- `#[non_exhaustive]`

A map of extensions associated with a given [`Locale`](crate::Locale).

```rust
pub struct Extensions {
    pub unicode: unicode::Unicode,
    pub transform: transform::Transform,
    pub private: private::Private,
    pub other: alloc::vec::Vec<other::Other>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `unicode` | `unicode::Unicode` | A representation of the data for a Unicode extension, when present in the locale identifier. |
| `transform` | `transform::Transform` | A representation of the data for a transform extension, when present in the locale identifier. |
| `private` | `private::Private` | A representation of the data for a private-use extension, when present in the locale identifier. |
| `other` | `alloc::vec::Vec<other::Other>` | A sequence of any other extensions that are present in the locale identifier but are not formally<br>[defined](https://unicode.org/reports/tr35/) and represented explicitly as [`Unicode`], [`Transform`],<br>and [`Private`] are. |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Returns a new empty map of extensions. Same as [`default()`](Default::default()), but is `const`.

- ```rust
  pub const fn from_unicode(unicode: Unicode) -> Self { /* ... */ }
  ```
  Function to create a new map of extensions containing exactly one unicode extension, callable in `const`

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether there are no extensions present.

- ```rust
  pub fn total_cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
  ```
  Returns an ordering suitable for use in [`BTreeSet`].

- ```rust
  pub fn retain_by_type<F>(self: &mut Self, predicate: F)
where
    F: FnMut(ExtensionType) -> bool { /* ... */ }
  ```
  Retains the specified extension types, clearing all others.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Extensions { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
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

- **ErasedDestructor**
- **Writeable**
  - ```rust
    fn write_to<W: core::fmt::Write + ?Sized>(self: &Self, sink: &mut W) -> core::fmt::Result { /* ... */ }
    ```

  - ```rust
    fn writeable_length_hint(self: &Self) -> writeable::LengthHint { /* ... */ }
    ```

- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Extensions) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Extensions { /* ... */ }
    ```

## Module `subtags`

**Attributes:**

- `#[macro_use]`

Language Identifier and Locale contains a set of subtags
which represent different fields of the structure.

* [`Language`] is the only mandatory field, which when empty,
takes the value `und`.
* [`Script`] is an optional field representing the written script used by the locale.
* [`Region`] is the region used by the locale.
* [`Variants`] is a list of optional [`Variant`] subtags containing information about the
               variant adjustments used by the locale.

Subtags can be used in isolation, and all basic operations such as parsing, syntax canonicalization
and serialization are supported on each individual subtag, but most commonly
they are used to construct a [`LanguageIdentifier`] instance.

[`Variants`] is a special structure which contains a list of [`Variant`] subtags.
It is wrapped around to allow for sorting and deduplication of variants, which
is one of the required steps of language identifier and locale syntax canonicalization.

# Examples

```
use icu::locid::subtags::{Language, Region, Script, Variant};

let language: Language =
    "en".parse().expect("Failed to parse a language subtag.");
let script: Script =
    "arab".parse().expect("Failed to parse a script subtag.");
let region: Region =
    "cn".parse().expect("Failed to parse a region subtag.");
let variant: Variant =
    "MacOS".parse().expect("Failed to parse a variant subtag.");

assert_eq!(language.as_str(), "en");
assert_eq!(script.as_str(), "Arab");
assert_eq!(region.as_str(), "CN");
assert_eq!(variant.as_str(), "macos");
```

`Notice`: The subtags are canonicalized on parsing. That means
that all operations work on a canonicalized version of the subtag
and serialization is very cheap.

[`LanguageIdentifier`]: super::LanguageIdentifier

```rust
pub mod subtags { /* ... */ }
```

### Re-exports

#### Re-export `Language`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use language::Language;
```

#### Re-export `Region`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use region::Region;
```

#### Re-export `Script`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use script::Script;
```

#### Re-export `Variant`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use variant::Variant;
```

#### Re-export `Variants`

```rust
pub use variants::Variants;
```

## Module `zerovec`

Documentation on zero-copy deserialization of locale types.

[`Locale`] and [`LanguageIdentifier`] are highly structured types that cannot be directly
stored in a zero-copy data structure, such as those provided by the [`zerovec`] crate.
This page explains how to indirectly store these types in a [`zerovec`].

There are two main use cases, which have different solutions:

1. **Lookup:** You need to locate a locale in a zero-copy vector, such as when querying a map.
2. **Obtain:** You have a locale stored in a zero-copy vector, and you need to obtain a proper
   [`Locale`] or [`LanguageIdentifier`] for use elsewhere in your program.

# Lookup

To perform lookup, store the stringified locale in a canonical BCP-47 form as a byte array,
and then use [`Locale::strict_cmp()`] to perform an efficient, zero-allocation lookup.

To produce more human-readable serialized output, you can use [`UnvalidatedStr`].

```
use icu::locid::Locale;
use zerovec::ule::UnvalidatedStr;
use zerovec::ZeroMap;

// ZeroMap from locales to integers
let data: &[(&UnvalidatedStr, u32)] = &[
    ("de-DE-u-hc-h12".into(), 5),
    ("en-US-u-ca-buddhist".into(), 10),
    ("my-MM".into(), 15),
    ("sr-Cyrl-ME".into(), 20),
    ("zh-TW".into(), 25),
];
let zm: ZeroMap<UnvalidatedStr, u32> = data.iter().copied().collect();

// Get the value associated with a locale
let loc: Locale = "en-US-u-ca-buddhist".parse().unwrap();
let value = zm.get_copied_by(|uvstr| loc.strict_cmp(uvstr).reverse());
assert_eq!(value, Some(10));
```

# Obtain

Obtaining a [`Locale`] or [`LanguageIdentifier`] is not generally a zero-copy operation, since
both of these types may require memory allocation. If possible, architect your code such that
you do not need to obtain a structured type.

If you need the structured type, such as if you need to manipulate it in some way, there are two
options: storing subtags, and storing a string for parsing.

## Storing Subtags

If the data being stored only contains a limited number of subtags, you can store them as a
tuple, and then construct the [`LanguageIdentifier`] externally.

```
use icu::locid::subtags::{Language, Region, Script};
use icu::locid::LanguageIdentifier;
use icu::locid::{
    langid,
    subtags::{language, region, script},
};
use zerovec::ZeroMap;

// ZeroMap from integer to LSR (language-script-region)
let zm: ZeroMap<u32, (Language, Option<Script>, Option<Region>)> = [
    (5, (language!("de"), None, Some(region!("DE")))),
    (10, (language!("en"), None, Some(region!("US")))),
    (15, (language!("my"), None, Some(region!("MM")))),
    (
        20,
        (language!("sr"), Some(script!("Cyrl")), Some(region!("ME"))),
    ),
    (25, (language!("zh"), None, Some(region!("TW")))),
]
.into_iter()
.collect();

// Construct a LanguageIdentifier from a tuple entry
let lid: LanguageIdentifier =
    zm.get_copied(&25).expect("element is present").into();

assert_eq!(lid, langid!("zh-TW"));
```

## Storing Strings

If it is necessary to store and obtain an arbitrary locale, it is currently recommended to
store a BCP-47 string and parse it when needed.

Since the string is stored in an unparsed state, it is not safe to `unwrap` the result from
`Locale::try_from_bytes()`. See [icu4x#831](https://github.com/unicode-org/icu4x/issues/831)
for a discussion on potential data models that could ensure that the locale is valid during
deserialization.

As above, to produce more human-readable serialized output, you can use [`UnvalidatedStr`].

```
use icu::locid::langid;
use icu::locid::Locale;
use zerovec::ule::UnvalidatedStr;
use zerovec::ZeroMap;

// ZeroMap from integer to locale string
let data: &[(u32, &UnvalidatedStr)] = &[
    (5, "de-DE-u-hc-h12".into()),
    (10, "en-US-u-ca-buddhist".into()),
    (15, "my-MM".into()),
    (20, "sr-Cyrl-ME".into()),
    (25, "zh-TW".into()),
    (30, "INVALID".into()),
];
let zm: ZeroMap<u32, UnvalidatedStr> = data.iter().copied().collect();

// Construct a Locale by parsing the string.
let value = zm.get(&25).expect("element is present");
let loc = Locale::try_from_bytes(value);
assert_eq!(loc, Ok(langid!("zh-TW").into()));

// Invalid entries are fallible
let err_value = zm.get(&30).expect("element is present");
let err_loc = Locale::try_from_bytes(err_value);
assert!(matches!(err_loc, Err(_)));
```

[`Locale`]: crate::Locale
[`Locale::strict_cmp()`]: crate::Locale::strict_cmp()
[`LanguageIdentifier`]: crate::LanguageIdentifier
[`UnvalidatedStr`]: zerovec::ule::UnvalidatedStr

```rust
pub mod zerovec { /* ... */ }
```

## Macros

### Macro `langid`

**Attributes:**

- `#[macro_export]`

A macro allowing for compile-time construction of valid [`LanguageIdentifier`]s.

The macro will perform syntax canonicalization of the tag.

# Examples

```
use icu::locid::{langid, LanguageIdentifier};

const DE_AT: LanguageIdentifier = langid!("de_at");

let de_at: LanguageIdentifier = "de_at".parse().unwrap();

assert_eq!(DE_AT, de_at);
```

*Note*: The macro cannot produce language identifiers with more than one variants due to const
limitations (see [`Heap Allocations in Constants`]):

```compile_fail,E0080
icu::locid::langid!("und-variant1-variant2");
```

Use runtime parsing instead:
```
"und-variant1-variant2"
    .parse::<icu::locid::LanguageIdentifier>()
    .unwrap();
```

[`LanguageIdentifier`]: crate::LanguageIdentifier
[`Heap Allocations in Constants`]: https://github.com/rust-lang/const-eval/issues/20

```rust
pub macro_rules! langid {
    /* macro_rules! langid {
    ($langid:literal) => { ... };
} */
}
```

### Macro `locale`

**Attributes:**

- `#[macro_export]`

A macro allowing for compile-time construction of valid [`Locale`]s.

The macro will perform syntax canonicalization of the tag.

# Examples

```
use icu::locid::{locale, Locale};

const DE_AT: Locale = locale!("de_at");

let de_at: Locale = "de_at".parse().unwrap();

assert_eq!(DE_AT, de_at);
```

*Note*: The macro cannot produce locales with more than one variant or multiple extensions
(only single keyword unicode extension is supported) due to const
limitations (see [`Heap Allocations in Constants`]):

```compile_fail,E0080
icu::locid::locale!("sl-IT-rozaj-biske-1994");
```
Use runtime parsing instead:
```
"sl-IT-rozaj-biske-1994"
    .parse::<icu::locid::Locale>()
    .unwrap();
```

Locales with multiple keys are not supported
```compile_fail,E0080
icu::locid::locale!("th-TH-u-ca-buddhist-nu-thai");
```
Use runtime parsing instead:
```
"th-TH-u-ca-buddhist-nu-thai"
    .parse::<icu::locid::Locale>()
    .unwrap();
```

Locales with attributes are not supported
```compile_fail,E0080
icu::locid::locale!("en-US-u-foobar-ca-buddhist");
```
Use runtime parsing instead:
```
"en-US-u-foobar-ca-buddhist"
    .parse::<icu::locid::Locale>()
    .unwrap();
```

Locales with single key but multiple types are not supported
```compile_fail,E0080
icu::locid::locale!("en-US-u-ca-islamic-umalqura");
```
Use runtime parsing instead:
```
"en-US-u-ca-islamic-umalqura"
    .parse::<icu::locid::Locale>()
    .unwrap();
```
[`Locale`]: crate::Locale
[`Heap Allocations in Constants`]: https://github.com/rust-lang/const-eval/issues/20

```rust
pub macro_rules! locale {
    /* macro_rules! locale {
    ($locale:literal) => { ... };
} */
}
```

## Re-exports

### Re-export `LanguageIdentifier`

```rust
pub use langid::LanguageIdentifier;
```

### Re-export `Locale`

```rust
pub use locale::Locale;
```

### Re-export `SubtagOrderingResult`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use ordering::SubtagOrderingResult;
```

### Re-export `ParserError`

```rust
pub use parser::errors::ParserError;
```

### Re-export `ParserError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use ParserError as Error;
```

