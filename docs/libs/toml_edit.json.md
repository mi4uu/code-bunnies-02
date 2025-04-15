# Crate Documentation

**Version:** 0.22.24

**Format Version:** 43

# Module `toml_edit`

# `toml_edit`

This crate allows you to parse and modify toml
documents, while preserving comments, spaces *and
relative order* or items.

If you also need the ease of a more traditional API, see the [`toml`] crate.

# Example

```rust
# #[cfg(feature = "parse")] {
# #[cfg(feature = "display")] {
use toml_edit::{DocumentMut, value};

let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
"#;
let mut doc = toml.parse::<DocumentMut>().expect("invalid doc");
assert_eq!(doc.to_string(), toml);
// let's add a new key/value pair inside a.b: c = {d = "hello"}
doc["a"]["b"]["c"]["d"] = value("hello");
// autoformat inline table a.b.c: { d = "hello" }
doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());
let expected = r#"
"hello" = 'toml!' # comment
['a'.b]
c = { d = "hello" }
"#;
assert_eq!(doc.to_string(), expected);
# }
# }
```

## Controlling formatting

By default, values are created with default formatting
```rust
# #[cfg(feature = "display")] {
# #[cfg(feature = "parse")] {
let mut doc = toml_edit::DocumentMut::new();
doc["foo"] = toml_edit::value("bar");
let expected = r#"foo = "bar"
"#;
assert_eq!(doc.to_string(), expected);
# }
# }
```

You can choose a custom TOML representation by parsing the value.
```rust
# #[cfg(feature = "display")] {
# #[cfg(feature = "parse")] {
let mut doc = toml_edit::DocumentMut::new();
doc["foo"] = "'bar'".parse::<toml_edit::Item>().unwrap();
let expected = r#"foo = 'bar'
"#;
assert_eq!(doc.to_string(), expected);
# }
# }
```

## Limitations

Things it does not preserve:

* Order of dotted keys, see [issue](https://github.com/toml-rs/toml/issues/163).

[`toml`]: https://docs.rs/toml/latest/toml/

## Modules

## Module `de`

**Attributes:**

- `#[<cfg>(feature = "serde")]`

Deserializing TOML into Rust structures.

This module contains all the Serde support for deserializing TOML documents into Rust structures.

```rust
pub mod de { /* ... */ }
```

### Types

#### Struct `Error`

Errors that can occur when deserializing a type.

```rust
pub struct Error {
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
  pub fn add_key(self: &mut Self, key: String) { /* ... */ }
  ```
  Add key while unwinding

- ```rust
  pub fn message(self: &Self) -> &str { /* ... */ }
  ```
  What went wrong

- ```rust
  pub fn span(self: &Self) -> Option<std::ops::Range<usize>> { /* ... */ }
  ```
  The start/end index into the original document where the error occurred

###### Trait Implementations

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(e: crate::TomlError) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(e: Error) -> crate::TomlError { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
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

- **IntoDeserializer**
  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

- **Error**
  - ```rust
    fn custom<T>(msg: T) -> Self
where
    T: std::fmt::Display { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Deserializer`

Deserialization for TOML [documents][crate::DocumentMut].

```rust
pub struct Deserializer<S = String> {
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
  pub fn new(input: crate::DocumentMut) -> Self { /* ... */ }
  ```
  Deserialization implementation for TOML.

- ```rust
  pub fn parse(raw: S) -> Result<Self, Error> { /* ... */ }
  ```
  Parse a TOML document

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```
    Parses a document from a &str

- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, name: &''static str, variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: serde::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
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
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
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
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(doc: crate::DocumentMut) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(doc: crate::ImDocument<S>) -> Self { /* ... */ }
    ```

- **Sync**
- **Send**
- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **IntoDeserializer**
  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

### Functions

#### Function `from_str`

**Attributes:**

- `#[<cfg>(feature = "parse")]`

Convert a TOML [documents][crate::DocumentMut] into `T`.

```rust
pub fn from_str<T>(s: &str) -> Result<T, Error>
where
    T: DeserializeOwned { /* ... */ }
```

#### Function `from_slice`

**Attributes:**

- `#[<cfg>(feature = "parse")]`

Convert a TOML [documents][crate::DocumentMut] into `T`.

```rust
pub fn from_slice<T>(s: &[u8]) -> Result<T, Error>
where
    T: DeserializeOwned { /* ... */ }
```

#### Function `from_document`

Convert a [`DocumentMut`][crate::DocumentMut] into `T`.

```rust
pub fn from_document<T, /* synthetic */ impl Into<Deserializer>: Into<Deserializer>>(d: impl Into<Deserializer>) -> Result<T, Error>
where
    T: DeserializeOwned { /* ... */ }
```

### Re-exports

#### Re-export `ValueDeserializer`

```rust
pub use value::ValueDeserializer;
```

## Module `ser`

**Attributes:**

- `#[<cfg>(feature = "serde")]`

Serializing Rust structures into TOML.

This module contains all the Serde support for serializing Rust structures into TOML.

```rust
pub mod ser { /* ... */ }
```

### Types

#### Enum `Error`

**Attributes:**

- `#[non_exhaustive]`

Errors that can occur when deserializing a type.

```rust
pub enum Error {
    UnsupportedType(Option<&''static str>),
    OutOfRange(Option<&''static str>),
    UnsupportedNone,
    KeyNotString,
    DateInvalid,
    Custom(String),
}
```

##### Variants

###### `UnsupportedType`

Type could not be serialized to TOML

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<&''static str>` |  |

###### `OutOfRange`

Value was out of range for the given type

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<&''static str>` |  |

###### `UnsupportedNone`

`None` could not be serialized to TOML

###### `KeyNotString`

Key was not convertible to `String` for serializing to TOML

###### `DateInvalid`

A serialized date was invalid

###### `Custom`

Other serialization error

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Error**
  - ```rust
    fn custom<T>(msg: T) -> Self
where
    T: std::fmt::Display { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(e: crate::TomlError) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(e: Error) -> crate::TomlError { /* ... */ }
    ```

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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
### Functions

#### Function `to_vec`

**Attributes:**

- `#[<cfg>(feature = "display")]`

Serialize the given data structure as a TOML byte vector.

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, if `T` contains a map with non-string keys, or if `T` attempts to
serialize an unsupported datatype such as an enum, tuple, or tuple struct.

```rust
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
```

#### Function `to_string`

**Attributes:**

- `#[<cfg>(feature = "display")]`

Serialize the given data structure as a String of TOML.

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, if `T` contains a map with non-string keys, or if `T` attempts to
serialize an unsupported datatype such as an enum, tuple, or tuple struct.

# Examples

```
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    database: Database,
}

#[derive(Serialize)]
struct Database {
    ip: String,
    port: Vec<u16>,
    connection_max: u32,
    enabled: bool,
}

let config = Config {
    database: Database {
        ip: "192.168.1.1".to_string(),
        port: vec![8001, 8002, 8003],
        connection_max: 5000,
        enabled: false,
    },
};

let toml = toml_edit::ser::to_string(&config).unwrap();
println!("{}", toml)
```

```rust
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
```

#### Function `to_string_pretty`

**Attributes:**

- `#[<cfg>(feature = "display")]`

Serialize the given data structure as a "pretty" String of TOML.

This is identical to `to_string` except the output string has a more
"pretty" output. See `ValueSerializer::pretty` for more details.

```rust
pub fn to_string_pretty<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
```

#### Function `to_document`

Serialize the given data structure into a TOML document.

This would allow custom formatting to be applied, mixing with format preserving edits, etc.

```rust
pub fn to_document<T>(value: &T) -> Result<crate::DocumentMut, Error>
where
    T: serde::ser::Serialize + ?Sized { /* ... */ }
```

### Re-exports

#### Re-export `ValueSerializer`

```rust
pub use value::ValueSerializer;
```

## Module `visit`

**Attributes:**

- `#[allow(missing_docs)]`

Document tree traversal to walk a shared borrow of a document tree.

Each method of the [`Visit`] trait is a hook that can be overridden
to customize the behavior when mutating the corresponding type of node.
By default, every method recursively visits the substructure of the
input by invoking the right visitor method of each of its fields.

```
# use toml_edit::{Item, ArrayOfTables, Table, Value};

pub trait Visit<'doc> {
    /* ... */

    fn visit_item(&mut self, i: &'doc Item) {
        visit_item(self, i);
    }

    /* ... */
    # fn visit_value(&mut self, i: &'doc Value);
    # fn visit_table(&mut self, i: &'doc Table);
    # fn visit_array_of_tables(&mut self, i: &'doc ArrayOfTables);
}

pub fn visit_item<'doc, V>(v: &mut V, node: &'doc Item)
where
    V: Visit<'doc> + ?Sized,
{
    match node {
        Item::None => {}
        Item::Value(value) => v.visit_value(value),
        Item::Table(table) => v.visit_table(table),
        Item::ArrayOfTables(array) => v.visit_array_of_tables(array),
    }
}
```

The API is modeled after [`syn::visit`](https://docs.rs/syn/1/syn/visit).

# Examples

This visitor stores every string in the document.

```
# #[cfg(feature = "parse")] {
# use toml_edit::*;
use toml_edit::visit::*;

#[derive(Default)]
struct StringCollector<'doc> {
    strings: Vec<&'doc str>,
}

impl<'doc> Visit<'doc> for StringCollector<'doc> {
    fn visit_string(&mut self, node: &'doc Formatted<String>) {
         self.strings.push(node.value().as_str());
    }
}

let input = r#"
laputa = "sky-castle"
the-force = { value = "surrounds-you" }
"#;

let mut document: DocumentMut = input.parse().unwrap();
let mut visitor = StringCollector::default();
visitor.visit_document(&document);

assert_eq!(visitor.strings, vec!["sky-castle", "surrounds-you"]);
# }
```

For a more complex example where the visitor has internal state, see `examples/visit.rs`
[on GitHub](https://github.com/toml-rs/toml/blob/main/crates/toml_edit/examples/visit.rs).

```rust
pub mod visit { /* ... */ }
```

### Traits

#### Trait `Visit`

Document tree traversal to mutate an exclusive borrow of a document tree in-place.

See the [module documentation](self) for details.

```rust
pub trait Visit<''doc> {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn visit_document(self: &mut Self, node: &''doc DocumentMut) { /* ... */ }
  ```

- ```rust
  fn visit_item(self: &mut Self, node: &''doc Item) { /* ... */ }
  ```

- ```rust
  fn visit_table(self: &mut Self, node: &''doc Table) { /* ... */ }
  ```

- ```rust
  fn visit_inline_table(self: &mut Self, node: &''doc InlineTable) { /* ... */ }
  ```

- ```rust
  fn visit_table_like(self: &mut Self, node: &''doc dyn TableLike) { /* ... */ }
  ```

- ```rust
  fn visit_table_like_kv(self: &mut Self, key: &''doc str, node: &''doc Item) { /* ... */ }
  ```

- ```rust
  fn visit_array(self: &mut Self, node: &''doc Array) { /* ... */ }
  ```

- ```rust
  fn visit_array_of_tables(self: &mut Self, node: &''doc ArrayOfTables) { /* ... */ }
  ```

- ```rust
  fn visit_value(self: &mut Self, node: &''doc Value) { /* ... */ }
  ```

- ```rust
  fn visit_boolean(self: &mut Self, node: &''doc Formatted<bool>) { /* ... */ }
  ```

- ```rust
  fn visit_datetime(self: &mut Self, node: &''doc Formatted<Datetime>) { /* ... */ }
  ```

- ```rust
  fn visit_float(self: &mut Self, node: &''doc Formatted<f64>) { /* ... */ }
  ```

- ```rust
  fn visit_integer(self: &mut Self, node: &''doc Formatted<i64>) { /* ... */ }
  ```

- ```rust
  fn visit_string(self: &mut Self, node: &''doc Formatted<String>) { /* ... */ }
  ```

### Functions

#### Function `visit_document`

```rust
pub fn visit_document<''doc, V>(v: &mut V, node: &''doc crate::DocumentMut)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

#### Function `visit_item`

```rust
pub fn visit_item<''doc, V>(v: &mut V, node: &''doc crate::Item)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

#### Function `visit_table`

```rust
pub fn visit_table<''doc, V>(v: &mut V, node: &''doc crate::Table)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

#### Function `visit_inline_table`

```rust
pub fn visit_inline_table<''doc, V>(v: &mut V, node: &''doc crate::InlineTable)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

#### Function `visit_table_like`

```rust
pub fn visit_table_like<''doc, V>(v: &mut V, node: &''doc dyn TableLike)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

#### Function `visit_table_like_kv`

```rust
pub fn visit_table_like_kv<''doc, V>(v: &mut V, _key: &''doc str, node: &''doc crate::Item)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

#### Function `visit_array`

```rust
pub fn visit_array<''doc, V>(v: &mut V, node: &''doc crate::Array)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

#### Function `visit_array_of_tables`

```rust
pub fn visit_array_of_tables<''doc, V>(v: &mut V, node: &''doc crate::ArrayOfTables)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

#### Function `visit_value`

```rust
pub fn visit_value<''doc, V>(v: &mut V, node: &''doc crate::Value)
where
    V: Visit<''doc> + ?Sized { /* ... */ }
```

## Module `visit_mut`

**Attributes:**

- `#[allow(missing_docs)]`

Document tree traversal to mutate an exclusive borrow of a document tree in place.


Each method of the [`VisitMut`] trait is a hook that can be overridden
to customize the behavior when mutating the corresponding type of node.
By default, every method recursively visits the substructure of the
input by invoking the right visitor method of each of its fields.

```
# use toml_edit::{Item, ArrayOfTables, Table, Value};

pub trait VisitMut {
    /* ... */

    fn visit_item_mut(&mut self, i: &mut Item) {
        visit_item_mut(self, i);
    }

    /* ... */
    # fn visit_value_mut(&mut self, i: &mut Value);
    # fn visit_table_mut(&mut self, i: &mut Table);
    # fn visit_array_of_tables_mut(&mut self, i: &mut ArrayOfTables);
}

pub fn visit_item_mut<V>(v: &mut V, node: &mut Item)
where
    V: VisitMut + ?Sized,
{
    match node {
        Item::None => {}
        Item::Value(value) => v.visit_value_mut(value),
        Item::Table(table) => v.visit_table_mut(table),
        Item::ArrayOfTables(array) => v.visit_array_of_tables_mut(array),
    }
}
```

The API is modeled after [`syn::visit_mut`](https://docs.rs/syn/1/syn/visit_mut).

# Examples

This visitor replaces every floating point value with its decimal string representation, to
2 decimal points.

```
# #[cfg(feature = "parse")] {
# #[cfg(feature = "display")] {
# use toml_edit::*;
use toml_edit::visit_mut::*;

struct FloatToString;

impl VisitMut for FloatToString {
    fn visit_value_mut(&mut self, node: &mut Value) {
        if let Value::Float(f) = node {
            // Convert the float to a string.
            let mut s = Formatted::new(format!("{:.2}", f.value()));
            // Copy over the formatting.
            std::mem::swap(s.decor_mut(), f.decor_mut());
            *node = Value::String(s);
        }
        // Most of the time, you will also need to call the default implementation to recurse
        // further down the document tree.
        visit_value_mut(self, node);
    }
}

let input = r#"
banana = 3.26
table = { apple = 4.5 }
"#;

let mut document: DocumentMut = input.parse().unwrap();
let mut visitor = FloatToString;
visitor.visit_document_mut(&mut document);

let output = r#"
banana = "3.26"
table = { apple = "4.50" }
"#;

assert_eq!(format!("{}", document), output);
# }
# }
```

For a more complex example where the visitor has internal state, see `examples/visit.rs`
[on GitHub](https://github.com/toml-rs/toml/blob/main/crates/toml_edit/examples/visit.rs).

```rust
pub mod visit_mut { /* ... */ }
```

### Traits

#### Trait `VisitMut`

Document tree traversal to mutate an exclusive borrow of a document tree in-place.

See the [module documentation](self) for details.

```rust
pub trait VisitMut {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn visit_document_mut(self: &mut Self, node: &mut DocumentMut) { /* ... */ }
  ```

- ```rust
  fn visit_item_mut(self: &mut Self, node: &mut Item) { /* ... */ }
  ```

- ```rust
  fn visit_table_mut(self: &mut Self, node: &mut Table) { /* ... */ }
  ```

- ```rust
  fn visit_inline_table_mut(self: &mut Self, node: &mut InlineTable) { /* ... */ }
  ```

- ```rust
  fn visit_table_like_mut(self: &mut Self, node: &mut dyn TableLike) { /* ... */ }
  ```
  [`visit_table_mut`](Self::visit_table_mut) and

- ```rust
  fn visit_table_like_kv_mut(self: &mut Self, key: KeyMut<''_>, node: &mut Item) { /* ... */ }
  ```

- ```rust
  fn visit_array_mut(self: &mut Self, node: &mut Array) { /* ... */ }
  ```

- ```rust
  fn visit_array_of_tables_mut(self: &mut Self, node: &mut ArrayOfTables) { /* ... */ }
  ```

- ```rust
  fn visit_value_mut(self: &mut Self, node: &mut Value) { /* ... */ }
  ```

- ```rust
  fn visit_boolean_mut(self: &mut Self, node: &mut Formatted<bool>) { /* ... */ }
  ```

- ```rust
  fn visit_datetime_mut(self: &mut Self, node: &mut Formatted<Datetime>) { /* ... */ }
  ```

- ```rust
  fn visit_float_mut(self: &mut Self, node: &mut Formatted<f64>) { /* ... */ }
  ```

- ```rust
  fn visit_integer_mut(self: &mut Self, node: &mut Formatted<i64>) { /* ... */ }
  ```

- ```rust
  fn visit_string_mut(self: &mut Self, node: &mut Formatted<String>) { /* ... */ }
  ```

### Functions

#### Function `visit_document_mut`

```rust
pub fn visit_document_mut<V>(v: &mut V, node: &mut crate::DocumentMut)
where
    V: VisitMut + ?Sized { /* ... */ }
```

#### Function `visit_item_mut`

```rust
pub fn visit_item_mut<V>(v: &mut V, node: &mut crate::Item)
where
    V: VisitMut + ?Sized { /* ... */ }
```

#### Function `visit_table_mut`

```rust
pub fn visit_table_mut<V>(v: &mut V, node: &mut crate::Table)
where
    V: VisitMut + ?Sized { /* ... */ }
```

#### Function `visit_inline_table_mut`

```rust
pub fn visit_inline_table_mut<V>(v: &mut V, node: &mut crate::InlineTable)
where
    V: VisitMut + ?Sized { /* ... */ }
```

#### Function `visit_table_like_mut`

```rust
pub fn visit_table_like_mut<V>(v: &mut V, node: &mut dyn TableLike)
where
    V: VisitMut + ?Sized { /* ... */ }
```

#### Function `visit_table_like_kv_mut`

```rust
pub fn visit_table_like_kv_mut<V>(v: &mut V, _key: crate::KeyMut<''_>, node: &mut crate::Item)
where
    V: VisitMut + ?Sized { /* ... */ }
```

#### Function `visit_array_mut`

```rust
pub fn visit_array_mut<V>(v: &mut V, node: &mut crate::Array)
where
    V: VisitMut + ?Sized { /* ... */ }
```

#### Function `visit_array_of_tables_mut`

```rust
pub fn visit_array_of_tables_mut<V>(v: &mut V, node: &mut crate::ArrayOfTables)
where
    V: VisitMut + ?Sized { /* ... */ }
```

#### Function `visit_value_mut`

```rust
pub fn visit_value_mut<V>(v: &mut V, node: &mut crate::Value)
where
    V: VisitMut + ?Sized { /* ... */ }
```

## Types

### Type Alias `Document`

**⚠️ Deprecated since 0.22.6**: Replaced with `DocumentMut`

Deprecated, replaced with [`DocumentMut`]

```rust
pub type Document = DocumentMut;
```

## Re-exports

### Re-export `Array`

```rust
pub use crate::array::Array;
```

### Re-export `ArrayIntoIter`

```rust
pub use crate::array::ArrayIntoIter;
```

### Re-export `ArrayIter`

```rust
pub use crate::array::ArrayIter;
```

### Re-export `ArrayIterMut`

```rust
pub use crate::array::ArrayIterMut;
```

### Re-export `ArrayOfTables`

```rust
pub use crate::array_of_tables::ArrayOfTables;
```

### Re-export `ArrayOfTablesIntoIter`

```rust
pub use crate::array_of_tables::ArrayOfTablesIntoIter;
```

### Re-export `ArrayOfTablesIter`

```rust
pub use crate::array_of_tables::ArrayOfTablesIter;
```

### Re-export `ArrayOfTablesIterMut`

```rust
pub use crate::array_of_tables::ArrayOfTablesIterMut;
```

### Re-export `DocumentMut`

```rust
pub use crate::document::DocumentMut;
```

### Re-export `ImDocument`

```rust
pub use crate::document::ImDocument;
```

### Re-export `TomlError`

```rust
pub use crate::error::TomlError;
```

### Re-export `InlineEntry`

```rust
pub use crate::inline_table::InlineEntry;
```

### Re-export `InlineOccupiedEntry`

```rust
pub use crate::inline_table::InlineOccupiedEntry;
```

### Re-export `InlineTable`

```rust
pub use crate::inline_table::InlineTable;
```

### Re-export `InlineTableIntoIter`

```rust
pub use crate::inline_table::InlineTableIntoIter;
```

### Re-export `InlineTableIter`

```rust
pub use crate::inline_table::InlineTableIter;
```

### Re-export `InlineTableIterMut`

```rust
pub use crate::inline_table::InlineTableIterMut;
```

### Re-export `InlineVacantEntry`

```rust
pub use crate::inline_table::InlineVacantEntry;
```

### Re-export `InternalString`

```rust
pub use crate::internal_string::InternalString;
```

### Re-export `array`

```rust
pub use crate::item::array;
```

### Re-export `table`

```rust
pub use crate::item::table;
```

### Re-export `value`

```rust
pub use crate::item::value;
```

### Re-export `Item`

```rust
pub use crate::item::Item;
```

### Re-export `Key`

```rust
pub use crate::key::Key;
```

### Re-export `KeyMut`

```rust
pub use crate::key::KeyMut;
```

### Re-export `RawString`

```rust
pub use crate::raw_string::RawString;
```

### Re-export `Decor`

```rust
pub use crate::repr::Decor;
```

### Re-export `Formatted`

```rust
pub use crate::repr::Formatted;
```

### Re-export `Repr`

```rust
pub use crate::repr::Repr;
```

### Re-export `Entry`

```rust
pub use crate::table::Entry;
```

### Re-export `IntoIter`

```rust
pub use crate::table::IntoIter;
```

### Re-export `Iter`

```rust
pub use crate::table::Iter;
```

### Re-export `IterMut`

```rust
pub use crate::table::IterMut;
```

### Re-export `OccupiedEntry`

```rust
pub use crate::table::OccupiedEntry;
```

### Re-export `Table`

```rust
pub use crate::table::Table;
```

### Re-export `TableLike`

```rust
pub use crate::table::TableLike;
```

### Re-export `VacantEntry`

```rust
pub use crate::table::VacantEntry;
```

### Re-export `Value`

```rust
pub use crate::value::Value;
```

### Re-export `toml_datetime::*`

```rust
pub use toml_datetime::*;
```

