# Crate Documentation

**Version:** 0.7.1

**Format Version:** 43

# Module `serde_urlencoded`

`x-www-form-urlencoded` meets Serde

## Modules

## Module `de`

Deserialization support for the `application/x-www-form-urlencoded` format.

```rust
pub mod de { /* ... */ }
```

### Types

#### Struct `Deserializer`

A deserializer for the `application/x-www-form-urlencoded` format.

* Supported top-level outputs are structs, maps and sequences of pairs,
  with or without a given length.

* Main `deserialize` methods defers to `deserialize_map`.

* Everything else but `deserialize_seq` and `deserialize_seq_fixed_size`
  defers to `deserialize`.

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
  pub fn new(parser: UrlEncodedParse<''de>) -> Self { /* ... */ }
  ```
  Returns a new `Deserializer`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
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

- **Send**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: de::Visitor<''de> { /* ... */ }
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
    fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
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
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, name: &''static str, variants: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `from_bytes`

Deserializes a `application/x-www-form-urlencoded` value from a `&[u8]`.

```
let meal = vec![
    ("bread".to_owned(), "baguette".to_owned()),
    ("cheese".to_owned(), "comté".to_owned()),
    ("meat".to_owned(), "ham".to_owned()),
    ("fat".to_owned(), "butter".to_owned()),
];

assert_eq!(
    serde_urlencoded::from_bytes::<Vec<(String, String)>>(
        b"bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter"),
    Ok(meal));
```

```rust
pub fn from_bytes<''de, T>(input: &''de [u8]) -> Result<T, Error>
where
    T: de::Deserialize<''de> { /* ... */ }
```

#### Function `from_str`

Deserializes a `application/x-www-form-urlencoded` value from a `&str`.

```
let meal = vec![
    ("bread".to_owned(), "baguette".to_owned()),
    ("cheese".to_owned(), "comté".to_owned()),
    ("meat".to_owned(), "ham".to_owned()),
    ("fat".to_owned(), "butter".to_owned()),
];

assert_eq!(
    serde_urlencoded::from_str::<Vec<(String, String)>>(
        "bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter"),
    Ok(meal));
```

```rust
pub fn from_str<''de, T>(input: &''de str) -> Result<T, Error>
where
    T: de::Deserialize<''de> { /* ... */ }
```

#### Function `from_reader`

Convenience function that reads all bytes from `reader` and deserializes
them with `from_bytes`.

```rust
pub fn from_reader<T, R>(reader: R) -> Result<T, Error>
where
    T: de::DeserializeOwned,
    R: Read { /* ... */ }
```

### Re-exports

#### Re-export `Error`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use serde::de::value::Error;
```

## Module `ser`

Serialization support for the `application/x-www-form-urlencoded` format.

```rust
pub mod ser { /* ... */ }
```

### Types

#### Struct `Serializer`

A serializer for the `application/x-www-form-urlencoded` format.

* Supported top-level inputs are structs, maps and sequences of pairs,
  with or without a given length.

* Supported keys and values are integers, bytes (if convertible to strings),
  unit structs and unit variants.

* Newtype structs defer to their inner values.

```rust
pub struct Serializer<''input, ''output, Target: UrlEncodedTarget> {
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
  pub fn new(urlencoder: &''output mut UrlEncodedSerializer<''input, Target>) -> Self { /* ... */ }
  ```
  Returns a new `Serializer`.

###### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Sync**
- **Unpin**
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
- **Serializer**
  - ```rust
    fn serialize_bool(self: Self, _v: bool) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_i8(self: Self, _v: i8) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_i16(self: Self, _v: i16) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_i32(self: Self, _v: i32) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_i64(self: Self, _v: i64) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_u8(self: Self, _v: u8) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_u16(self: Self, _v: u16) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_u32(self: Self, _v: u32) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_u64(self: Self, _v: u64) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_f32(self: Self, _v: f32) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_f64(self: Self, _v: f64) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_char(self: Self, _v: char) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_str(self: Self, _value: &str) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_bytes(self: Self, _value: &[u8]) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_unit(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns `Ok`.

  - ```rust
    fn serialize_unit_struct(self: Self, _name: &''static str) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns `Ok`.

  - ```rust
    fn serialize_unit_variant(self: Self, _name: &''static str, _variant_index: u32, _variant: &''static str) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(self: Self, _name: &''static str, value: &T) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Serializes the inner value, ignoring the newtype name.

  - ```rust
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(self: Self, _name: &''static str, _variant_index: u32, _variant: &''static str, _value: &T) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_none(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Returns `Ok`.

  - ```rust
    fn serialize_some<T: ?Sized + ser::Serialize>(self: Self, value: &T) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```
    Serializes the given value.

  - ```rust
    fn serialize_seq(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeSeq, Error> { /* ... */ }
    ```
    Serialize a sequence, given length (if any) is ignored.

  - ```rust
    fn serialize_tuple(self: Self, _len: usize) -> Result<<Self as >::SerializeTuple, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_tuple_struct(self: Self, _name: &''static str, _len: usize) -> Result<<Self as >::SerializeTupleStruct, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_tuple_variant(self: Self, _name: &''static str, _variant_index: u32, _variant: &''static str, _len: usize) -> Result<<Self as >::SerializeTupleVariant, Error> { /* ... */ }
    ```
    Returns an error.

  - ```rust
    fn serialize_map(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeMap, Error> { /* ... */ }
    ```
    Serializes a map, given length is ignored.

  - ```rust
    fn serialize_struct(self: Self, _name: &''static str, _len: usize) -> Result<<Self as >::SerializeStruct, Error> { /* ... */ }
    ```
    Serializes a struct, given length is ignored.

  - ```rust
    fn serialize_struct_variant(self: Self, _name: &''static str, _variant_index: u32, _variant: &''static str, _len: usize) -> Result<<Self as >::SerializeStructVariant, Error> { /* ... */ }
    ```
    Returns an error.

#### Enum `Error`

Errors returned during serializing to `application/x-www-form-urlencoded`.

```rust
pub enum Error {
    Custom(std::borrow::Cow<''static, str>),
    Utf8(str::Utf8Error),
}
```

##### Variants

###### `Custom`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::borrow::Cow<''static, str>` |  |

###### `Utf8`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `str::Utf8Error` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn cause(self: &Self) -> Option<&dyn error::Error> { /* ... */ }
    ```
    The lower-level cause of this error, in the case of a `Utf8` error.

  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```
    The lower-level source of this error, in the case of a `Utf8` error.

  - ```rust
    fn custom<T: fmt::Display>(msg: T) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

#### Struct `SeqSerializer`

Sequence serializer.

```rust
pub struct SeqSerializer<''input, ''output, Target: UrlEncodedTarget> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **SerializeSeq**
  - ```rust
    fn serialize_element<T: ?Sized + ser::Serialize>(self: &mut Self, value: &T) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn end(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **RefUnwindSafe**
#### Struct `TupleSerializer`

Tuple serializer.

Mostly used for arrays.

```rust
pub struct TupleSerializer<''input, ''output, Target: UrlEncodedTarget> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Sync**
- **Freeze**
- **UnwindSafe**
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

- **Send**
- **Unpin**
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

- **SerializeTuple**
  - ```rust
    fn serialize_element<T: ?Sized + ser::Serialize>(self: &mut Self, value: &T) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn end(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `TupleStructSerializer`

Tuple struct serializer.

Never instantiated, tuple structs are not supported.

```rust
pub struct TupleStructSerializer<''input, ''output, T: UrlEncodedTarget> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **SerializeTupleStruct**
  - ```rust
    fn serialize_field<T: ?Sized + ser::Serialize>(self: &mut Self, value: &T) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn end(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `TupleVariantSerializer`

Tuple variant serializer.

Never instantiated, tuple variants are not supported.

```rust
pub struct TupleVariantSerializer<''input, ''output, T: UrlEncodedTarget> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
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

- **Unpin**
- **SerializeTupleVariant**
  - ```rust
    fn serialize_field<T: ?Sized + ser::Serialize>(self: &mut Self, value: &T) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn end(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```

- **Sync**
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

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **UnwindSafe**
#### Struct `MapSerializer`

Map serializer.

```rust
pub struct MapSerializer<''input, ''output, Target: UrlEncodedTarget> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **SerializeMap**
  - ```rust
    fn serialize_entry<K: ?Sized + ser::Serialize, V: ?Sized + ser::Serialize>(self: &mut Self, key: &K, value: &V) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_key<T: ?Sized + ser::Serialize>(self: &mut Self, key: &T) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn serialize_value<T: ?Sized + ser::Serialize>(self: &mut Self, value: &T) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn end(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Send**
- **Freeze**
- **Unpin**
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

#### Struct `StructSerializer`

Struct serializer.

```rust
pub struct StructSerializer<''input, ''output, Target: UrlEncodedTarget> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Unpin**
- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **SerializeStruct**
  - ```rust
    fn serialize_field<T: ?Sized + ser::Serialize>(self: &mut Self, key: &''static str, value: &T) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn end(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```

#### Struct `StructVariantSerializer`

Struct variant serializer.

Never instantiated, struct variants are not supported.

```rust
pub struct StructVariantSerializer<''input, ''output, T: UrlEncodedTarget> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **SerializeStructVariant**
  - ```rust
    fn serialize_field<T: ?Sized + ser::Serialize>(self: &mut Self, key: &''static str, value: &T) -> Result<(), Error> { /* ... */ }
    ```

  - ```rust
    fn end(self: Self) -> Result<<Self as >::Ok, Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
### Functions

#### Function `to_string`

Serializes a value into a `application/x-www-form-urlencoded` `String` buffer.

```
let meal = &[
    ("bread", "baguette"),
    ("cheese", "comté"),
    ("meat", "ham"),
    ("fat", "butter"),
];

assert_eq!(
    serde_urlencoded::to_string(meal),
    Ok("bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter".to_owned()));
```

```rust
pub fn to_string<T: ser::Serialize>(input: T) -> Result<String, Error> { /* ... */ }
```

## Re-exports

### Re-export `from_bytes`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::de::from_bytes;
```

### Re-export `from_reader`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::de::from_reader;
```

### Re-export `from_str`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::de::from_str;
```

### Re-export `Deserializer`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::de::Deserializer;
```

### Re-export `to_string`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::ser::to_string;
```

### Re-export `Serializer`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::ser::Serializer;
```

