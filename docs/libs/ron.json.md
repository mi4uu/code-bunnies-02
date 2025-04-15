# Crate Documentation

**Version:** 0.8.1

**Format Version:** 43

# Module `ron`

# Rusty Object Notation

[![CI](https://github.com/ron-rs/ron/actions/workflows/ci.yaml/badge.svg)](https://github.com/ron-rs/ron/actions/workflows/ci.yaml)
[![codecov](https://img.shields.io/codecov/c/github/ron-rs/ron/codecov?token=x4Q5KA51Ul)](https://codecov.io/gh/ron-rs/ron)
[![Crates.io](https://img.shields.io/crates/v/ron.svg)](https://crates.io/crates/ron)
[![MSRV](https://img.shields.io/badge/MSRV-1.64.0-orange)](https://github.com/ron-rs/ron)
[![Docs](https://docs.rs/ron/badge.svg)](https://docs.rs/ron)
[![Matrix](https://img.shields.io/matrix/ron-rs:matrix.org.svg)](https://matrix.to/#/#ron-rs:matrix.org)

RON is a simple readable data serialization format that looks similar to Rust syntax.
It's designed to support all of [Serde's data model](https://serde.rs/data-model.html), so
structs, enums, tuples, arrays, generic maps, and primitive values.

## Example

```rust,ignore
GameConfig( // optional struct name
    window_size: (800, 600),
    window_title: "PAC-MAN",
    fullscreen: false,

    mouse_sensitivity: 1.4,
    key_bindings: {
        "up": Up,
        "down": Down,
        "left": Left,
        "right": Right,

        // Uncomment to enable WASD controls
        /*
        "W": Up,
        "S": Down,
        "A": Left,
        "D": Right,
        */
    },

    difficulty_options: (
        start_difficulty: Easy,
        adaptive: false,
    ),
)
```

## Why RON?

### Example in JSON

```json
{
   "materials": {
        "metal": {
            "reflectivity": 1.0
        },
        "plastic": {
            "reflectivity": 0.5
        }
   },
   "entities": [
        {
            "name": "hero",
            "material": "metal"
        },
        {
            "name": "monster",
            "material": "plastic"
        }
   ]
}
```

### Same example in RON

```rust,ignore
Scene( // class name is optional
    materials: { // this is a map
        "metal": (
            reflectivity: 1.0,
        ),
        "plastic": (
            reflectivity: 0.5,
        ),
    },
    entities: [ // this is an array
        (
            name: "hero",
            material: "metal",
        ),
        (
            name: "monster",
            material: "plastic",
        ),
    ],
)
```

Note the following advantages of RON over JSON:

* trailing commas allowed
* single- and multi-line comments
* field names aren't quoted, so it's less verbose
* optional struct names improve readability
* enums are supported (and less verbose than their JSON representation)

## Limitations

RON is not designed to be a fully self-describing format (unlike JSON) and is thus not guaranteed to work when [`deserialize_any`](https://docs.rs/serde/latest/serde/trait.Deserializer.html#tymethod.deserialize_any) is used instead of its typed alternatives. In particular, the following Serde attributes are not yet supported:
- `#[serde(tag = "type")]`, i.e. internally tagged enums
- `#[serde(untagged)]`, i.e. untagged enums
- `#[serde(flatten)]`, i.e. flattening an inner struct into its outer container

## RON syntax overview

* Numbers: `42`, `3.14`, `0xFF`, `0b0110`
* Strings: `"Hello"`, `"with\\escapes\n"`, `r#"raw string, great for regex\."#`
* Booleans: `true`, `false`
* Chars: `'e'`, `'\n'`
* Optionals: `Some("string")`, `Some(Some(1.34))`, `None`
* Tuples: `("abc", 1.23, true)`, `()`
* Lists: `["abc", "def"]`
* Structs: `( foo: 1.0, bar: ( baz: "I'm nested" ) )`
* Maps: `{ "arbitrary": "keys", "are": "allowed" }`

> **Note:** Serde's data model represents fixed-size Rust arrays as tuple (instead of as list)

## Quickstart

### `Cargo.toml`

```toml
[dependencies]
ron = "0.8"
serde = { version = "1", features = ["derive"] }
```

### `main.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MyStruct {
    boolean: bool,
    float: f32,
}

fn main() {
    let x: MyStruct = ron::from_str("(boolean: true, float: 1.23)").unwrap();

    println!("RON: {}", ron::to_string(&x).unwrap());

    println!("Pretty RON: {}", ron::ser::to_string_pretty(
        &x, ron::ser::PrettyConfig::default()).unwrap(),
    );
}
```

## Tooling

| Editor       | Plugin                                                      |
| ------------ | ----------------------------------------------------------- |
| IntelliJ     | [intellij-ron](https://github.com/ron-rs/intellij-ron)      |
| VS Code      | [a5huynh/vscode-ron](https://github.com/a5huynh/vscode-ron) |
| Sublime Text | [RON](https://packagecontrol.io/packages/RON)               |
| Atom         | [language-ron](https://atom.io/packages/language-ron)       |
| Vim          | [ron-rs/ron.vim](https://github.com/ron-rs/ron.vim)         |
| EMACS        | [emacs-ron]                                                 |

[emacs-ron]: https://chiselapp.com/user/Hutzdog/repository/ron-mode/home

## Specification

There is a very basic, work in progress specification available on
[the wiki page](https://github.com/ron-rs/ron/wiki/Specification).
A more formal and complete grammar is available [here](docs/grammar.md).


## License

RON is dual-licensed under Apache-2.0 and MIT.

Any contribution intentionally submitted for inclusion in the work must be provided under the same dual-license terms.

## Modules

## Module `de`

```rust
pub mod de { /* ... */ }
```

### Types

#### Struct `Deserializer`

The RON deserializer.

If you just want to simply deserialize a value,
you can use the [`from_str`] convenience function.

```rust
pub struct Deserializer<''de> {
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
  pub fn from_str(input: &''de str) -> SpannedResult<Self> { /* ... */ }
  ```

- ```rust
  pub fn from_bytes(input: &''de [u8]) -> SpannedResult<Self> { /* ... */ }
  ```

- ```rust
  pub fn from_str_with_options(input: &''de str, options: Options) -> SpannedResult<Self> { /* ... */ }
  ```

- ```rust
  pub fn from_bytes_with_options(input: &''de [u8], options: Options) -> SpannedResult<Self> { /* ... */ }
  ```

- ```rust
  pub fn remainder(self: &Self) -> Cow<''_, str> { /* ... */ }
  ```

- ```rust
  pub fn span_error(self: &Self, code: Error) -> SpannedError { /* ... */ }
  ```

- ```rust
  pub fn end(self: &mut Self) -> Result<()> { /* ... */ }
  ```
  Check if the remaining bytes are whitespace only,

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, _fields: &''static [&''static str], visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, name: &''static str, _variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

### Functions

#### Function `from_reader`

A convenience function for building a deserializer
and deserializing a value of type `T` from a reader.

```rust
pub fn from_reader<R, T>(rdr: R) -> crate::error::SpannedResult<T>
where
    R: io::Read,
    T: de::DeserializeOwned { /* ... */ }
```

#### Function `from_str`

A convenience function for building a deserializer
and deserializing a value of type `T` from a string.

```rust
pub fn from_str<''a, T>(s: &''a str) -> crate::error::SpannedResult<T>
where
    T: de::Deserialize<''a> { /* ... */ }
```

#### Function `from_bytes`

A convenience function for building a deserializer
and deserializing a value of type `T` from bytes.

```rust
pub fn from_bytes<''a, T>(s: &''a [u8]) -> crate::error::SpannedResult<T>
where
    T: de::Deserialize<''a> { /* ... */ }
```

### Re-exports

#### Re-export `Error`

```rust
pub use crate::error::Error;
```

#### Re-export `Position`

```rust
pub use crate::error::Position;
```

#### Re-export `SpannedError`

```rust
pub use crate::error::SpannedError;
```

## Module `ser`

```rust
pub mod ser { /* ... */ }
```

### Types

#### Struct `PrettyConfig`

**Attributes:**

- `#[serde(default)]`
- `#[non_exhaustive]`

Pretty serializer configuration.

# Examples

```
use ron::ser::PrettyConfig;

let my_config = PrettyConfig::new()
    .depth_limit(4)
    // definitely superior (okay, just joking)
    .indentor("\t".to_owned());
```

```rust
pub struct PrettyConfig {
    pub depth_limit: usize,
    pub new_line: String,
    pub indentor: String,
    pub separator: String,
    pub struct_names: bool,
    pub separate_tuple_members: bool,
    pub enumerate_arrays: bool,
    pub extensions: crate::extensions::Extensions,
    pub compact_arrays: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `depth_limit` | `usize` | Limit the pretty-ness up to the given depth. |
| `new_line` | `String` | New line string |
| `indentor` | `String` | Indentation string |
| `separator` | `String` | Separator string |
| `struct_names` | `bool` |  |
| `separate_tuple_members` | `bool` | Separate tuple members with indentation |
| `enumerate_arrays` | `bool` | Enumerate array items in comments |
| `extensions` | `crate::extensions::Extensions` | Enable extensions. Only configures 'implicit_some',<br> 'unwrap_newtypes', and 'unwrap_variant_newtypes' for now. |
| `compact_arrays` | `bool` | Enable compact arrays |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a default [`PrettyConfig`].

- ```rust
  pub fn depth_limit(self: Self, depth_limit: usize) -> Self { /* ... */ }
  ```
  Limits the pretty-formatting based on the number of indentations.

- ```rust
  pub fn new_line(self: Self, new_line: String) -> Self { /* ... */ }
  ```
  Configures the newlines used for serialization.

- ```rust
  pub fn indentor(self: Self, indentor: String) -> Self { /* ... */ }
  ```
  Configures the string sequence used for indentation.

- ```rust
  pub fn separator(self: Self, separator: String) -> Self { /* ... */ }
  ```
  Configures the string sequence used to separate items inline.

- ```rust
  pub fn struct_names(self: Self, struct_names: bool) -> Self { /* ... */ }
  ```
  Configures whether to emit struct names.

- ```rust
  pub fn separate_tuple_members(self: Self, separate_tuple_members: bool) -> Self { /* ... */ }
  ```
  Configures whether tuples are single- or multi-line.

- ```rust
  pub fn enumerate_arrays(self: Self, enumerate_arrays: bool) -> Self { /* ... */ }
  ```
  Configures whether a comment shall be added to every array element,

- ```rust
  pub fn compact_arrays(self: Self, compact_arrays: bool) -> Self { /* ... */ }
  ```
  Configures whether every array should be a single line (`true`)

- ```rust
  pub fn extensions(self: Self, extensions: Extensions) -> Self { /* ... */ }
  ```
  Configures extensions

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PrettyConfig { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `Serializer`

The RON serializer.

You can just use [`to_string`] for deserializing a value.
If you want it pretty-printed, take a look at [`to_string_pretty`].

```rust
pub struct Serializer<W: io::Write> {
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
  pub fn new(writer: W, config: Option<PrettyConfig>) -> Result<Self> { /* ... */ }
  ```
  Creates a new [`Serializer`].

- ```rust
  pub fn with_options(writer: W, config: Option<PrettyConfig>, options: Options) -> Result<Self> { /* ... */ }
  ```
  Creates a new [`Serializer`].

###### Trait Implementations

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

- **Serializer**
  - ```rust
    fn serialize_bool(self: Self, v: bool) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i8(self: Self, v: i8) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i16(self: Self, v: i16) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i32(self: Self, v: i32) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i64(self: Self, v: i64) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u8(self: Self, v: u8) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u16(self: Self, v: u16) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u32(self: Self, v: u32) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u64(self: Self, v: u64) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_f32(self: Self, v: f32) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_f64(self: Self, v: f64) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_char(self: Self, v: char) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_str(self: Self, v: &str) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_bytes(self: Self, v: &[u8]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_none(self: Self) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_some<T>(self: Self, value: &T) -> Result<()>
where
    T: ?Sized + Serialize { /* ... */ }
    ```

  - ```rust
    fn serialize_unit(self: Self) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit_struct(self: Self, name: &''static str) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit_variant(self: Self, _: &''static str, _: u32, variant: &''static str) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_newtype_struct<T>(self: Self, name: &''static str, value: &T) -> Result<()>
where
    T: ?Sized + Serialize { /* ... */ }
    ```

  - ```rust
    fn serialize_newtype_variant<T>(self: Self, _: &''static str, _: u32, variant: &''static str, value: &T) -> Result<()>
where
    T: ?Sized + Serialize { /* ... */ }
    ```

  - ```rust
    fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple_struct(self: Self, name: &''static str, len: usize) -> Result<<Self as >::SerializeTupleStruct> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple_variant(self: Self, _: &''static str, _: u32, variant: &''static str, len: usize) -> Result<<Self as >::SerializeTupleVariant> { /* ... */ }
    ```

  - ```rust
    fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap> { /* ... */ }
    ```

  - ```rust
    fn serialize_struct(self: Self, name: &''static str, len: usize) -> Result<<Self as >::SerializeStruct> { /* ... */ }
    ```

  - ```rust
    fn serialize_struct_variant(self: Self, _: &''static str, _: u32, variant: &''static str, len: usize) -> Result<<Self as >::SerializeStructVariant> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `to_writer`

Serializes `value` into `writer`.

This function does not generate any newlines or nice formatting;
if you want that, you can use [`to_writer_pretty`] instead.

```rust
pub fn to_writer<W, T>(writer: W, value: &T) -> crate::error::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize { /* ... */ }
```

#### Function `to_writer_pretty`

Serializes `value` into `writer` in a pretty way.

```rust
pub fn to_writer_pretty<W, T>(writer: W, value: &T, config: PrettyConfig) -> crate::error::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize { /* ... */ }
```

#### Function `to_string`

Serializes `value` and returns it as string.

This function does not generate any newlines or nice formatting;
if you want that, you can use [`to_string_pretty`] instead.

```rust
pub fn to_string<T>(value: &T) -> crate::error::Result<String>
where
    T: ?Sized + Serialize { /* ... */ }
```

#### Function `to_string_pretty`

Serializes `value` in the recommended RON layout in a pretty way.

```rust
pub fn to_string_pretty<T>(value: &T, config: PrettyConfig) -> crate::error::Result<String>
where
    T: ?Sized + Serialize { /* ... */ }
```

## Module `error`

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `SpannedError`

This type represents all possible errors that can occur when
serializing or deserializing RON data.

```rust
pub struct SpannedError {
    pub code: Error,
    pub position: Position,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `code` | `Error` |  |
| `position` | `Position` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
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

  - ```rust
    fn from(e: io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: SpannedError) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SpannedError) -> bool { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SpannedError { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
- **Eq**
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

#### Type Alias `Result`

```rust
pub type Result<T, E = Error> = std::result::Result<T, E>;
```

#### Type Alias `SpannedResult`

```rust
pub type SpannedResult<T> = std::result::Result<T, SpannedError>;
```

#### Enum `Error`

**Attributes:**

- `#[non_exhaustive]`

```rust
pub enum Error {
    Io(String),
    Message(String),
    Base64Error(base64::DecodeError),
    Eof,
    ExpectedArray,
    ExpectedArrayEnd,
    ExpectedAttribute,
    ExpectedAttributeEnd,
    ExpectedBoolean,
    ExpectedComma,
    ExpectedChar,
    ExpectedFloat,
    FloatUnderscore,
    ExpectedInteger,
    ExpectedOption,
    ExpectedOptionEnd,
    ExpectedMap,
    ExpectedMapColon,
    ExpectedMapEnd,
    ExpectedDifferentStructName {
        expected: &''static str,
        found: String,
    },
    ExpectedStructLike,
    ExpectedNamedStructLike(&''static str),
    ExpectedStructLikeEnd,
    ExpectedUnit,
    ExpectedString,
    ExpectedStringEnd,
    ExpectedIdentifier,
    InvalidEscape(&''static str),
    IntegerOutOfBounds,
    NoSuchExtension(String),
    UnclosedBlockComment,
    UnderscoreAtBeginning,
    UnexpectedByte(char),
    Utf8Error(std::str::Utf8Error),
    TrailingCharacters,
    InvalidValueForType {
        expected: String,
        found: String,
    },
    ExpectedDifferentLength {
        expected: String,
        found: usize,
    },
    NoSuchEnumVariant {
        expected: &''static [&''static str],
        found: String,
        outer: Option<String>,
    },
    NoSuchStructField {
        expected: &''static [&''static str],
        found: String,
        outer: Option<String>,
    },
    MissingStructField {
        field: &''static str,
        outer: Option<String>,
    },
    DuplicateStructField {
        field: &''static str,
        outer: Option<String>,
    },
    InvalidIdentifier(String),
    SuggestRawIdentifier(String),
    ExceededRecursionLimit,
}
```

##### Variants

###### `Io`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Message`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Base64Error`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `base64::DecodeError` |  |

###### `Eof`

###### `ExpectedArray`

###### `ExpectedArrayEnd`

###### `ExpectedAttribute`

###### `ExpectedAttributeEnd`

###### `ExpectedBoolean`

###### `ExpectedComma`

###### `ExpectedChar`

###### `ExpectedFloat`

###### `FloatUnderscore`

###### `ExpectedInteger`

###### `ExpectedOption`

###### `ExpectedOptionEnd`

###### `ExpectedMap`

###### `ExpectedMapColon`

###### `ExpectedMapEnd`

###### `ExpectedDifferentStructName`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `expected` | `&''static str` |  |
| `found` | `String` |  |

###### `ExpectedStructLike`

###### `ExpectedNamedStructLike`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `ExpectedStructLikeEnd`

###### `ExpectedUnit`

###### `ExpectedString`

###### `ExpectedStringEnd`

###### `ExpectedIdentifier`

###### `InvalidEscape`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `IntegerOutOfBounds`

###### `NoSuchExtension`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `UnclosedBlockComment`

###### `UnderscoreAtBeginning`

###### `UnexpectedByte`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

###### `Utf8Error`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::str::Utf8Error` |  |

###### `TrailingCharacters`

###### `InvalidValueForType`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `expected` | `String` |  |
| `found` | `String` |  |

###### `ExpectedDifferentLength`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `expected` | `String` |  |
| `found` | `usize` |  |

###### `NoSuchEnumVariant`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `expected` | `&''static [&''static str]` |  |
| `found` | `String` |  |
| `outer` | `Option<String>` |  |

###### `NoSuchStructField`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `expected` | `&''static [&''static str]` |  |
| `found` | `String` |  |
| `outer` | `Option<String>` |  |

###### `MissingStructField`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `field` | `&''static str` |  |
| `outer` | `Option<String>` |  |

###### `DuplicateStructField`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `field` | `&''static str` |  |
| `outer` | `Option<String>` |  |

###### `InvalidIdentifier`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `SuggestRawIdentifier`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `ExceededRecursionLimit`

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

  - ```rust
    fn from(e: Utf8Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: FromUtf8Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: SpannedError) -> Self { /* ... */ }
    ```

- **Error**
  - ```rust
    fn custom<T: fmt::Display>(msg: T) -> Self { /* ... */ }
    ```

  - ```rust
    fn custom<T: fmt::Display>(msg: T) -> Self { /* ... */ }
    ```

  - ```rust
    fn invalid_type(unexp: de::Unexpected<''_>, exp: &dyn de::Expected) -> Self { /* ... */ }
    ```

  - ```rust
    fn invalid_value(unexp: de::Unexpected<''_>, exp: &dyn de::Expected) -> Self { /* ... */ }
    ```

  - ```rust
    fn invalid_length(len: usize, exp: &dyn de::Expected) -> Self { /* ... */ }
    ```

  - ```rust
    fn unknown_variant(variant: &str, expected: &''static [&''static str]) -> Self { /* ... */ }
    ```

  - ```rust
    fn unknown_field(field: &str, expected: &''static [&''static str]) -> Self { /* ... */ }
    ```

  - ```rust
    fn missing_field(field: &''static str) -> Self { /* ... */ }
    ```

  - ```rust
    fn duplicate_field(field: &''static str) -> Self { /* ... */ }
    ```

- **Send**
- **Sync**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Position`

```rust
pub struct Position {
    pub line: usize,
    pub col: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `line` | `usize` |  |
| `col` | `usize` |  |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Position) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Position { /* ... */ }
    ```

- **Sync**
- **Eq**
- **Send**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **RefUnwindSafe**
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `value`

Value module.

```rust
pub mod value { /* ... */ }
```

### Types

#### Struct `Map`

**Attributes:**

- `#[serde(transparent)]`

A [`Value`] to [`Value`] map.

This structure either uses a [BTreeMap](std::collections::BTreeMap) or the
[IndexMap](indexmap::IndexMap) internally.
The latter can be used by enabling the `indexmap` feature. This can be used
to preserve the order of the parsed map.

```rust
pub struct Map(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Map { /* ... */ }
  ```
  Creates a new, empty [`Map`].

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the map.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if `self.len() == 0`, `false` otherwise.

- ```rust
  pub fn insert(self: &mut Self, key: Value, value: Value) -> Option<Value> { /* ... */ }
  ```
  Inserts a new element, returning the previous element with this `key` if

- ```rust
  pub fn remove(self: &mut Self, key: &Value) -> Option<Value> { /* ... */ }
  ```
  Removes an element by its `key`.

- ```rust
  pub fn iter(self: &Self) -> impl Iterator<Item = (&Value, &Value)> + DoubleEndedIterator { /* ... */ }
  ```
  Iterate all key-value pairs.

- ```rust
  pub fn iter_mut(self: &mut Self) -> impl Iterator<Item = (&Value, &mut Value)> + DoubleEndedIterator { /* ... */ }
  ```
  Iterate all key-value pairs mutably.

- ```rust
  pub fn keys(self: &Self) -> impl Iterator<Item = &Value> + DoubleEndedIterator { /* ... */ }
  ```
  Iterate all keys.

- ```rust
  pub fn values(self: &Self) -> impl Iterator<Item = &Value> + DoubleEndedIterator { /* ... */ }
  ```
  Iterate all values.

- ```rust
  pub fn values_mut(self: &mut Self) -> impl Iterator<Item = &mut Value> + DoubleEndedIterator { /* ... */ }
  ```
  Iterate all values mutably.

- ```rust
  pub fn retain<F>(self: &mut Self, keep: F)
where
    F: FnMut(&Value, &mut Value) -> bool { /* ... */ }
  ```
  Retains only the elements specified by the `keep` predicate.

###### Trait Implementations

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

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: &Value) -> &mut <Self as >::Output { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Map { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Map) -> Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DeserializeOwned**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Map) -> bool { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: &Value) -> &<Self as >::Output { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = (Value, Value)>>(iter: T) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Map) -> Option<Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Map { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
#### Enum `Number`

A wrapper for a number, which can be either [`f64`] or [`i64`].

```rust
pub enum Number {
    Integer(i64),
    Float(Float),
}
```

##### Variants

###### `Integer`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `Float`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Float` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new</* synthetic */ impl Into<Number>: Into<Number>>(v: impl Into<Number>) -> Self { /* ... */ }
  ```
  Construct a new number.

- ```rust
  pub fn into_f64(self: Self) -> f64 { /* ... */ }
  ```
  Returns the [`f64`] representation of the [`Number`] regardless of

- ```rust
  pub fn as_f64(self: Self) -> Option<f64> { /* ... */ }
  ```
  If the [`Number`] is a float, return it. Otherwise return [`None`].

- ```rust
  pub fn as_i64(self: Self) -> Option<i64> { /* ... */ }
  ```
  If the [`Number`] is an integer, return it. Otherwise return [`None`].

- ```rust
  pub fn map_to<T, /* synthetic */ impl FnOnce(i64) -> T: FnOnce(i64) -> T, /* synthetic */ impl FnOnce(f64) -> T: FnOnce(f64) -> T>(self: Self, integer_fn: impl FnOnce(i64) -> T, float_fn: impl FnOnce(f64) -> T) -> T { /* ... */ }
  ```
  Map this number to a single type using the appropriate closure.

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **UnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(f: f64) -> Number { /* ... */ }
    ```

  - ```rust
    fn from(i: i64) -> Number { /* ... */ }
    ```

  - ```rust
    fn from(i: i32) -> Number { /* ... */ }
    ```

  - ```rust
    fn from(i: u64) -> Number { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Number) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Number) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Number) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Number { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Float`

A wrapper for [`f64`], which guarantees that the inner value
is finite and thus implements [`Eq`], [`Hash`] and [`Ord`].

```rust
pub struct Float(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(v: f64) -> Self { /* ... */ }
  ```
  Construct a new [`Float`].

- ```rust
  pub fn get(self: Self) -> f64 { /* ... */ }
  ```
  Returns the wrapped float.

###### Trait Implementations

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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
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

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Float { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

#### Enum `Value`

```rust
pub enum Value {
    Bool(bool),
    Char(char),
    Map(Map),
    Number(Number),
    Option(Option<Box<Value>>),
    String(String),
    Seq(Vec<Value>),
    Unit,
}
```

##### Variants

###### `Bool`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

###### `Char`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

###### `Map`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Map` |  |

###### `Number`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Number` |  |

###### `Option`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<Box<Value>>` |  |

###### `String`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Seq`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Value>` |  |

###### `Unit`

##### Implementations

###### Methods

- ```rust
  pub fn into_rust<T>(self: Self) -> Result<T>
where
    T: DeserializeOwned { /* ... */ }
  ```
  Tries to deserialize this [`Value`] into `T`.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Deserializer**
  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, name: &''static str, variants: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: Visitor<''de> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Value) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **DeserializeOwned**
- **StructuralPartialEq**
- **Eq**
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

- **UnwindSafe**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> SpannedResult<Self> { /* ... */ }
    ```
    Creates a value from a string reference.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **Index**
  - ```rust
    fn index(self: &Self, index: &Value) -> &<Self as >::Output { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: Deserializer<''de> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Value) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: &Value) -> &mut <Self as >::Output { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Value { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `extensions`

```rust
pub mod extensions { /* ... */ }
```

### Types

#### Struct `Extensions`

```rust
pub struct Extensions(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> usize { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: usize) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: usize) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: usize) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<Extensions> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<Extensions> { /* ... */ }
  ```
  Yield a set of contained named flags values.

- ```rust
  pub fn from_ident(ident: &[u8]) -> Option<Extensions> { /* ... */ }
  ```
  Creates an extension flag from an ident.

###### Trait Implementations

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **PublicFlags**
- **RefUnwindSafe**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Extensions) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Extensions) -> bool { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **DeserializeOwned**
- **Flags**
  - ```rust
    fn bits(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: usize) -> Extensions { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: Extensions) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Extensions { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Extensions) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

## Module `options`

Roundtrip serde Options module.

```rust
pub mod options { /* ... */ }
```

### Types

#### Struct `Options`

**Attributes:**

- `#[serde(default)]`
- `#[non_exhaustive]`

Roundtrip serde options.

# Examples

```
use ron::{Options, extensions::Extensions};

let ron = Options::default()
    .with_default_extension(Extensions::IMPLICIT_SOME);

let de: Option<i32> = ron.from_str("42").unwrap();
let ser = ron.to_string(&de).unwrap();

assert_eq!(ser, "42");
```

```rust
pub struct Options {
    pub default_extensions: crate::extensions::Extensions,
    pub recursion_limit: Option<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `default_extensions` | `crate::extensions::Extensions` | Extensions that are enabled by default during serialization and<br> deserialization.<br>During serialization, these extensions do NOT have to be explicitly<br> enabled in the parsed RON.<br>During deserialization, these extensions are used, but their explicit<br> activation is NOT included in the output RON.<br>No extensions are enabled by default. |
| `recursion_limit` | `Option<usize>` | Default recursion limit that is checked during serialization and<br> deserialization.<br>If set to `None`, infinite recursion is allowed and stack overflow<br> errors can crash the serialization or deserialization process.<br>Defaults to `Some(128)`, i.e. 128 recursive calls are allowed. |

##### Implementations

###### Methods

- ```rust
  pub fn with_default_extension(self: Self, default_extension: Extensions) -> Self { /* ... */ }
  ```
  Enable `default_extension` by default during serialization and deserialization.

- ```rust
  pub fn without_default_extension(self: Self, default_extension: Extensions) -> Self { /* ... */ }
  ```
  Do NOT enable `default_extension` by default during serialization and deserialization.

- ```rust
  pub fn with_recursion_limit(self: Self, recursion_limit: usize) -> Self { /* ... */ }
  ```
  Set a maximum recursion limit during serialization and deserialization.

- ```rust
  pub fn without_recursion_limit(self: Self) -> Self { /* ... */ }
  ```
  Disable the recursion limit during serialization and deserialization.

- ```rust
  pub fn from_reader<R, T>(self: &Self, rdr: R) -> SpannedResult<T>
where
    R: io::Read,
    T: de::DeserializeOwned { /* ... */ }
  ```
  A convenience function for building a deserializer

- ```rust
  pub fn from_str<''a, T>(self: &Self, s: &''a str) -> SpannedResult<T>
where
    T: de::Deserialize<''a> { /* ... */ }
  ```
  A convenience function for building a deserializer

- ```rust
  pub fn from_bytes<''a, T>(self: &Self, s: &''a [u8]) -> SpannedResult<T>
where
    T: de::Deserialize<''a> { /* ... */ }
  ```
  A convenience function for building a deserializer

- ```rust
  pub fn from_reader_seed<R, S, T>(self: &Self, rdr: R, seed: S) -> SpannedResult<T>
where
    R: io::Read,
    S: for<''a> de::DeserializeSeed<''a, Value = T> { /* ... */ }
  ```
  A convenience function for building a deserializer

- ```rust
  pub fn from_str_seed<''a, S, T>(self: &Self, s: &''a str, seed: S) -> SpannedResult<T>
where
    S: de::DeserializeSeed<''a, Value = T> { /* ... */ }
  ```
  A convenience function for building a deserializer

- ```rust
  pub fn from_bytes_seed<''a, S, T>(self: &Self, s: &''a [u8], seed: S) -> SpannedResult<T>
where
    S: de::DeserializeSeed<''a, Value = T> { /* ... */ }
  ```
  A convenience function for building a deserializer

- ```rust
  pub fn to_writer<W, T>(self: &Self, writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + ser::Serialize { /* ... */ }
  ```
  Serializes `value` into `writer`.

- ```rust
  pub fn to_writer_pretty<W, T>(self: &Self, writer: W, value: &T, config: PrettyConfig) -> Result<()>
where
    W: io::Write,
    T: ?Sized + ser::Serialize { /* ... */ }
  ```
  Serializes `value` into `writer` in a pretty way.

- ```rust
  pub fn to_string<T>(self: &Self, value: &T) -> Result<String>
where
    T: ?Sized + ser::Serialize { /* ... */ }
  ```
  Serializes `value` and returns it as string.

- ```rust
  pub fn to_string_pretty<T>(self: &Self, value: &T, config: PrettyConfig) -> Result<String>
where
    T: ?Sized + ser::Serialize { /* ... */ }
  ```
  Serializes `value` in the recommended RON layout in a pretty way.

###### Trait Implementations

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **DeserializeOwned**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
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

- **RefUnwindSafe**
- **Unpin**
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

- **Sync**
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
    fn clone(self: &Self) -> Options { /* ... */ }
    ```

## Re-exports

### Re-export `from_str`

```rust
pub use de::from_str;
```

### Re-export `Deserializer`

```rust
pub use de::Deserializer;
```

### Re-export `Error`

```rust
pub use error::Error;
```

### Re-export `Result`

```rust
pub use error::Result;
```

### Re-export `Options`

```rust
pub use options::Options;
```

### Re-export `to_string`

```rust
pub use ser::to_string;
```

### Re-export `Serializer`

```rust
pub use ser::Serializer;
```

### Re-export `Map`

```rust
pub use value::Map;
```

### Re-export `Number`

```rust
pub use value::Number;
```

### Re-export `Value`

```rust
pub use value::Value;
```

