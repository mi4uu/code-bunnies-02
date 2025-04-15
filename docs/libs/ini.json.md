# Crate Documentation

**Version:** 0.21.1

**Format Version:** 43

# Module `ini`

Ini parser for Rust

```no_run
use ini::Ini;

let mut conf = Ini::new();
conf.with_section(Some("User"))
    .set("name", "Raspberry树莓")
    .set("value", "Pi");
conf.with_section(Some("Library"))
    .set("name", "Sun Yat-sen U")
    .set("location", "Guangzhou=world");
conf.write_to_file("conf.ini").unwrap();

let i = Ini::load_from_file("conf.ini").unwrap();
for (sec, prop) in i.iter() {
    println!("Section: {:?}", sec);
    for (k, v) in prop.iter() {
        println!("{}:{}", k, v);
    }
}
```

## Types

### Enum `EscapePolicy`

Policies for escaping logic

```rust
pub enum EscapePolicy {
    Nothing,
    Basics,
    BasicsUnicode,
    BasicsUnicodeExtended,
    Reserved,
    ReservedUnicode,
    ReservedUnicodeExtended,
    Everything,
}
```

#### Variants

##### `Nothing`

Escape absolutely nothing (dangerous)

##### `Basics`

Only escape the most necessary things.
This means backslashes, control characters (codepoints U+0000 to U+001F), and delete (U+007F).
Quotes (single or double) are not escaped.

##### `BasicsUnicode`

Escape basics and non-ASCII characters in the [Basic Multilingual Plane](https://www.compart.com/en/unicode/plane)
(i.e. between U+007F - U+FFFF)
Codepoints above U+FFFF, e.g. '🐱' U+1F431 "CAT FACE" will *not* be escaped!

##### `BasicsUnicodeExtended`

Escape basics and all non-ASCII characters, including codepoints above U+FFFF.
This will escape emoji - if you want them to remain raw, use BasicsUnicode instead.

##### `Reserved`

Escape reserved symbols.
This includes everything in EscapePolicy::Basics, plus the comment characters ';' and '#' and the key/value-separating characters '=' and ':'.

##### `ReservedUnicode`

Escape reserved symbols and non-ASCII characters in the BMP.
Codepoints above U+FFFF, e.g. '🐱' U+1F431 "CAT FACE" will *not* be escaped!

##### `ReservedUnicodeExtended`

Escape reserved symbols and all non-ASCII characters, including codepoints above U+FFFF.

##### `Everything`

Escape everything that some INI implementations assume

#### Implementations

##### Methods

- ```rust
  pub fn should_escape(self: Self, c: char) -> bool { /* ... */ }
  ```
  Given a character this returns true if it should be escaped as

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EscapePolicy { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EscapePolicy) -> bool { /* ... */ }
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

- **Copy**
### Struct `ParseOption`

Parsing configuration

```rust
pub struct ParseOption {
    pub enabled_quote: bool,
    pub enabled_escape: bool,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enabled_quote` | `bool` | Allow quote (`"` or `'`) in value<br>For example<br>```ini<br>[Section]<br>Key1="Quoted value"<br>Key2='Single Quote' with extra value<br>```<br><br>In this example, Value of `Key1` is `Quoted value`,<br>and value of `Key2` is `Single Quote with extra value`<br>if `enabled_quote` is set to `true`. |
| `enabled_escape` | `bool` | Interpret `\` as an escape character<br>For example<br>```ini<br>[Section]<br>Key1=C:\Windows<br>```<br><br>If `enabled_escape` is true, then the value of `Key` will become `C:Windows` (`\W` equals to `W`). |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ParseOption { /* ... */ }
    ```

### Enum `LineSeparator`

Newline style

```rust
pub enum LineSeparator {
    SystemDefault,
    CR,
    CRLF,
}
```

#### Variants

##### `SystemDefault`

System-dependent line separator

On UNIX system, uses "\n"
On Windows system, uses "\r\n"

##### `CR`

Uses "\n" as new line separator

##### `CRLF`

Uses "\r\n" as new line separator

#### Implementations

##### Methods

- ```rust
  pub fn as_str(self: Self) -> &''static str { /* ... */ }
  ```
  String representation

##### Trait Implementations

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LineSeparator) -> bool { /* ... */ }
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

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LineSeparator { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **Freeze**
- **Unpin**
### Struct `WriteOption`

Writing configuration

```rust
pub struct WriteOption {
    pub escape_policy: EscapePolicy,
    pub line_separator: LineSeparator,
    pub kv_separator: &''static str,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `escape_policy` | `EscapePolicy` | Policies about how to escape characters |
| `line_separator` | `LineSeparator` | Newline style |
| `kv_separator` | `&''static str` | Key value separator |

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Sync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WriteOption { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> WriteOption { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
### Type Alias `SectionKey`

Internal storage of section's key

```rust
pub type SectionKey = Option<String>;
```

### Type Alias `PropertyKey`

Internal storage of property's key

```rust
pub type PropertyKey = String;
```

### Struct `SectionSetter`

A setter which could be used to set key-value pair in a specified section

```rust
pub struct SectionSetter<''a> {
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
  pub fn set<''b, K, V>(self: &''b mut Self, key: K, value: V) -> &''b mut SectionSetter<''a>
where
    K: Into<String>,
    V: Into<String>,
    ''a: ''b { /* ... */ }
  ```
  Set (replace) key-value pair in this section (all with the same name)

- ```rust
  pub fn add<''b, K, V>(self: &''b mut Self, key: K, value: V) -> &''b mut SectionSetter<''a>
where
    K: Into<String>,
    V: Into<String>,
    ''a: ''b { /* ... */ }
  ```
  Add (append) key-value pair in this section

- ```rust
  pub fn delete<''b, K>(self: &''b mut Self, key: &K) -> &''b mut SectionSetter<''a>
where
    K: AsRef<str>,
    ''a: ''b { /* ... */ }
  ```
  Delete the first entry in this section with `key`

- ```rust
  pub fn get<K: AsRef<str>>(self: &''a Self, key: K) -> Option<&''a str> { /* ... */ }
  ```
  Get the entry in this section with `key`

##### Trait Implementations

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

- **UnwindSafe**
- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `Properties`

Properties type (key-value pairs)

```rust
pub struct Properties {
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
  pub fn new() -> Properties { /* ... */ }
  ```
  Create an instance

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Get the number of the properties

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Check if properties has 0 elements

- ```rust
  pub fn iter(self: &Self) -> PropertyIter<''_> { /* ... */ }
  ```
  Get an iterator of the properties

- ```rust
  pub fn iter_mut(self: &mut Self) -> PropertyIterMut<''_> { /* ... */ }
  ```
  Get a mutable iterator of the properties

- ```rust
  pub fn contains_key<S: AsRef<str>>(self: &Self, s: S) -> bool { /* ... */ }
  ```
  Return true if property exist

- ```rust
  pub fn insert<K, V>(self: &mut Self, k: K, v: V)
where
    K: Into<String>,
    V: Into<String> { /* ... */ }
  ```
  Insert (key, value) pair by replace

- ```rust
  pub fn append<K, V>(self: &mut Self, k: K, v: V)
where
    K: Into<String>,
    V: Into<String> { /* ... */ }
  ```
  Append key with (key, value) pair

- ```rust
  pub fn get<S: AsRef<str>>(self: &Self, s: S) -> Option<&str> { /* ... */ }
  ```
  Get the first value associate with the key

- ```rust
  pub fn get_all<S: AsRef<str>>(self: &Self, s: S) -> impl DoubleEndedIterator<Item = &str> { /* ... */ }
  ```
  Get all values associate with the key

- ```rust
  pub fn remove<S: AsRef<str>>(self: &mut Self, s: S) -> Option<String> { /* ... */ }
  ```
  Remove the property with the first value of the key

- ```rust
  pub fn remove_all<S: AsRef<str>>(self: &mut Self, s: S) -> impl DoubleEndedIterator<Item = String> + ''_ { /* ... */ }
  ```
  Remove the property with all values with the same key

##### Trait Implementations

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

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Properties) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: S) -> &str { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Default**
  - ```rust
    fn default() -> Properties { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Properties { /* ... */ }
    ```

- **Freeze**
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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `PropertyIter`

```rust
pub struct PropertyIter<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
- **Send**
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

### Struct `PropertyIterMut`

Iterator for traversing sections

```rust
pub struct PropertyIterMut<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
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

### Struct `PropertiesIntoIter`

```rust
pub struct PropertiesIntoIter {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
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

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
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

- **Freeze**
- **Unpin**
### Struct `SectionVacantEntry`

A view into a vacant entry in a `Ini`

```rust
pub struct SectionVacantEntry<''a> {
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
  pub fn insert(self: Self, value: Properties) -> &''a mut Properties { /* ... */ }
  ```
  Insert one new section

##### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
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

- **Unpin**
- **Send**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `SectionOccupiedEntry`

A view into a occupied entry in a `Ini`

```rust
pub struct SectionOccupiedEntry<''a> {
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
  pub fn into_mut(self: Self) -> &''a mut Properties { /* ... */ }
  ```
  Into the first internal mutable properties

- ```rust
  pub fn append(self: &mut Self, prop: Properties) { /* ... */ }
  ```
  Append a new section

##### Trait Implementations

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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
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
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
### Enum `SectionEntry`

A view into an `Ini`, which may either be vacant or occupied.

```rust
pub enum SectionEntry<''a> {
    Vacant(SectionVacantEntry<''a>),
    Occupied(SectionOccupiedEntry<''a>),
}
```

#### Variants

##### `Vacant`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SectionVacantEntry<''a>` |  |

##### `Occupied`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SectionOccupiedEntry<''a>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn or_insert(self: Self, properties: Properties) -> &''a mut Properties { /* ... */ }
  ```
  Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.

- ```rust
  pub fn or_insert_with<F: FnOnce() -> Properties>(self: Self, default: F) -> &''a mut Properties { /* ... */ }
  ```
  Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable reference to the value in the entry.

##### Trait Implementations

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
- **RefUnwindSafe**
- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

  - ```rust
    fn from(e: Entry<''a, SectionKey, Properties>) -> SectionEntry<''a> { /* ... */ }
    ```

### Struct `Ini`

Ini struct

```rust
pub struct Ini {
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
  pub fn new() -> Ini { /* ... */ }
  ```
  Create an instance

- ```rust
  pub fn with_section<S>(self: &mut Self, section: Option<S>) -> SectionSetter<''_>
where
    S: Into<String> { /* ... */ }
  ```
  Set with a specified section, `None` is for the general section

- ```rust
  pub fn with_general_section(self: &mut Self) -> SectionSetter<''_> { /* ... */ }
  ```
  Set with general section, a simple wrapper of `with_section(None::<String>)`

- ```rust
  pub fn general_section(self: &Self) -> &Properties { /* ... */ }
  ```
  Get the immutable general section

- ```rust
  pub fn general_section_mut(self: &mut Self) -> &mut Properties { /* ... */ }
  ```
  Get the mutable general section

- ```rust
  pub fn section<S>(self: &Self, name: Option<S>) -> Option<&Properties>
where
    S: Into<String> { /* ... */ }
  ```
  Get a immutable section

- ```rust
  pub fn section_mut<S>(self: &mut Self, name: Option<S>) -> Option<&mut Properties>
where
    S: Into<String> { /* ... */ }
  ```
  Get a mutable section

- ```rust
  pub fn section_all<S>(self: &Self, name: Option<S>) -> impl DoubleEndedIterator<Item = &Properties>
where
    S: Into<String> { /* ... */ }
  ```
  Get all sections immutable with the same key

- ```rust
  pub fn section_all_mut<S>(self: &mut Self, name: Option<S>) -> impl DoubleEndedIterator<Item = &mut Properties>
where
    S: Into<String> { /* ... */ }
  ```
  Get all sections mutable with the same key

- ```rust
  pub fn entry(self: &mut Self, name: Option<String>) -> SectionEntry<''_> { /* ... */ }
  ```
  Get the entry

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clear all entries

- ```rust
  pub fn sections(self: &Self) -> impl DoubleEndedIterator<Item = Option<&str>> { /* ... */ }
  ```
  Iterate with sections

- ```rust
  pub fn set_to<S>(self: &mut Self, section: Option<S>, key: String, value: String)
where
    S: Into<String> { /* ... */ }
  ```
  Set key-value to a section

- ```rust
  pub fn get_from<''a, S>(self: &''a Self, section: Option<S>, key: &str) -> Option<&''a str>
where
    S: Into<String> { /* ... */ }
  ```
  Get the first value from the sections with key

- ```rust
  pub fn get_from_or<''a, S>(self: &''a Self, section: Option<S>, key: &str, default: &''a str) -> &''a str
where
    S: Into<String> { /* ... */ }
  ```
  Get the first value from the sections with key, return the default value if it does not exist

- ```rust
  pub fn get_from_mut<''a, S>(self: &''a mut Self, section: Option<S>, key: &str) -> Option<&''a mut str>
where
    S: Into<String> { /* ... */ }
  ```
  Get the first mutable value from the sections with key

- ```rust
  pub fn delete<S>(self: &mut Self, section: Option<S>) -> Option<Properties>
where
    S: Into<String> { /* ... */ }
  ```
  Delete the first section with key, return the properties if it exists

- ```rust
  pub fn delete_from<S>(self: &mut Self, section: Option<S>, key: &str) -> Option<String>
where
    S: Into<String> { /* ... */ }
  ```
  Delete the key from the section, return the value if key exists or None

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Total sections count

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Check if object contains no section

- ```rust
  pub fn write_to_file<P: AsRef<Path>>(self: &Self, filename: P) -> io::Result<()> { /* ... */ }
  ```
  Write to a file

- ```rust
  pub fn write_to_file_policy<P: AsRef<Path>>(self: &Self, filename: P, policy: EscapePolicy) -> io::Result<()> { /* ... */ }
  ```
  Write to a file

- ```rust
  pub fn write_to_file_opt<P: AsRef<Path>>(self: &Self, filename: P, opt: WriteOption) -> io::Result<()> { /* ... */ }
  ```
  Write to a file with options

- ```rust
  pub fn write_to<W: Write>(self: &Self, writer: &mut W) -> io::Result<()> { /* ... */ }
  ```
  Write to a writer

- ```rust
  pub fn write_to_policy<W: Write>(self: &Self, writer: &mut W, policy: EscapePolicy) -> io::Result<()> { /* ... */ }
  ```
  Write to a writer

- ```rust
  pub fn write_to_opt<W: Write>(self: &Self, writer: &mut W, opt: WriteOption) -> io::Result<()> { /* ... */ }
  ```
  Write to a writer with options

- ```rust
  pub fn load_from_str(buf: &str) -> Result<Ini, ParseError> { /* ... */ }
  ```
  Load from a string

- ```rust
  pub fn load_from_str_noescape(buf: &str) -> Result<Ini, ParseError> { /* ... */ }
  ```
  Load from a string, but do not interpret '\' as an escape character

- ```rust
  pub fn load_from_str_opt(buf: &str, opt: ParseOption) -> Result<Ini, ParseError> { /* ... */ }
  ```
  Load from a string with options

- ```rust
  pub fn read_from<R: Read>(reader: &mut R) -> Result<Ini, Error> { /* ... */ }
  ```
  Load from a reader

- ```rust
  pub fn read_from_noescape<R: Read>(reader: &mut R) -> Result<Ini, Error> { /* ... */ }
  ```
  Load from a reader, but do not interpret '\' as an escape character

- ```rust
  pub fn read_from_opt<R: Read>(reader: &mut R, opt: ParseOption) -> Result<Ini, Error> { /* ... */ }
  ```
  Load from a reader with options

- ```rust
  pub fn load_from_file<P: AsRef<Path>>(filename: P) -> Result<Ini, Error> { /* ... */ }
  ```
  Load from a file

- ```rust
  pub fn load_from_file_noescape<P: AsRef<Path>>(filename: P) -> Result<Ini, Error> { /* ... */ }
  ```
  Load from a file, but do not interpret '\' as an escape character

- ```rust
  pub fn load_from_file_opt<P: AsRef<Path>>(filename: P, opt: ParseOption) -> Result<Ini, Error> { /* ... */ }
  ```
  Load from a file with options

- ```rust
  pub fn iter(self: &''a Self) -> SectionIter<''a> { /* ... */ }
  ```
  Immutable iterate though sections

- ```rust
  pub fn mut_iter(self: &''a mut Self) -> SectionIterMut<''a> { /* ... */ }
  ```
  Mutable iterate though sections

- ```rust
  pub fn iter_mut(self: &''a mut Self) -> SectionIterMut<''a> { /* ... */ }
  ```
  Mutable iterate though sections

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: Option<S>) -> &Properties { /* ... */ }
    ```

  - ```rust
    fn index<''a>(self: &''a Self, index: &''q str) -> &''a Properties { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: Option<S>) -> &mut Properties { /* ... */ }
    ```

  - ```rust
    fn index_mut<''a>(self: &''a mut Self, index: &''q str) -> &''a mut Properties { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ini { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
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

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Creates an ini instance with an empty general section. This allows [Ini::general_section]

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `SectionIter`

Iterator for traversing sections

```rust
pub struct SectionIter<''a> {
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `SectionIterMut`

Iterator for traversing sections

```rust
pub struct SectionIterMut<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `SectionIntoIter`

Iterator for traversing sections

```rust
pub struct SectionIntoIter {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

### Struct `ParseError`

Parse error

```rust
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub msg: std::borrow::Cow<''static, str>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `line` | `usize` |  |
| `col` | `usize` |  |
| `msg` | `std::borrow::Cow<''static, str>` |  |

#### Implementations

##### Trait Implementations

- **Freeze**
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
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Error**
### Enum `Error`

Error while parsing an INI document

```rust
pub enum Error {
    Io(io::Error),
    Parse(ParseError),
}
```

#### Variants

##### `Io`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `io::Error` |  |

##### `Parse`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ParseError` |  |

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: io::Error) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

