# Crate Documentation

**Version:** 0.8.22

**Format Version:** 43

# Module `schemars`

# Schemars

> [!NOTE]
> Schemars 1.0 is in development on [the v1 branch](https://github.com/GREsau/schemars/tree/v1), see [draft PR 290](https://github.com/GREsau/schemars/pull/290) for updates

[![CI Build](https://img.shields.io/github/actions/workflow/status/GREsau/schemars/ci.yml?branch=master&logo=GitHub)](https://github.com/GREsau/schemars/actions)
[![Crates.io](https://img.shields.io/crates/v/schemars)](https://crates.io/crates/schemars)
[![Docs](https://docs.rs/schemars/badge.svg)](https://docs.rs/schemars)
[![MSRV 1.60+](https://img.shields.io/badge/schemars-rustc_1.60+-lightgray.svg)](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html)

Generate JSON Schema documents from Rust code

## Basic Usage

If you don't really care about the specifics, the easiest way to generate a JSON schema for your types is to `#[derive(JsonSchema)]` and use the `schema_for!` macro. All fields of the type must also implement `JsonSchema` - Schemars implements this for many standard library types.

```rust
use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
pub struct MyStruct {
    pub my_int: i32,
    pub my_bool: bool,
    pub my_nullable_enum: Option<MyEnum>,
}

#[derive(JsonSchema)]
pub enum MyEnum {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}

let schema = schema_for!(MyStruct);
println!("{}", serde_json::to_string_pretty(&schema).unwrap());
```

<details>
<summary>Click to see the output JSON schema...</summary>

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "MyStruct",
  "type": "object",
  "required": ["my_bool", "my_int"],
  "properties": {
    "my_bool": {
      "type": "boolean"
    },
    "my_int": {
      "type": "integer",
      "format": "int32"
    },
    "my_nullable_enum": {
      "anyOf": [
        {
          "$ref": "#/definitions/MyEnum"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "definitions": {
    "MyEnum": {
      "anyOf": [
        {
          "type": "object",
          "required": ["StringNewType"],
          "properties": {
            "StringNewType": {
              "type": "string"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": ["StructVariant"],
          "properties": {
            "StructVariant": {
              "type": "object",
              "required": ["floats"],
              "properties": {
                "floats": {
                  "type": "array",
                  "items": {
                    "type": "number",
                    "format": "float"
                  }
                }
              }
            }
          },
          "additionalProperties": false
        }
      ]
    }
  }
}
```

</details>

### Serde Compatibility

One of the main aims of this library is compatibility with [Serde](https://github.com/serde-rs/serde). Any generated schema _should_ match how [serde_json](https://github.com/serde-rs/json) would serialize/deserialize to/from JSON. To support this, Schemars will check for any `#[serde(...)]` attributes on types that derive `JsonSchema`, and adjust the generated schema accordingly.

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MyStruct {
    #[serde(rename = "myNumber")]
    pub my_int: i32,
    pub my_bool: bool,
    #[serde(default)]
    pub my_nullable_enum: Option<MyEnum>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum MyEnum {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}

let schema = schema_for!(MyStruct);
println!("{}", serde_json::to_string_pretty(&schema).unwrap());
```

<details>
<summary>Click to see the output JSON schema...</summary>

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "MyStruct",
  "type": "object",
  "required": ["myBool", "myNumber"],
  "properties": {
    "myBool": {
      "type": "boolean"
    },
    "myNullableEnum": {
      "default": null,
      "anyOf": [
        {
          "$ref": "#/definitions/MyEnum"
        },
        {
          "type": "null"
        }
      ]
    },
    "myNumber": {
      "type": "integer",
      "format": "int32"
    }
  },
  "additionalProperties": false,
  "definitions": {
    "MyEnum": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "object",
          "required": ["floats"],
          "properties": {
            "floats": {
              "type": "array",
              "items": {
                "type": "number",
                "format": "float"
              }
            }
          }
        }
      ]
    }
  }
}
```

</details>

`#[serde(...)]` attributes can be overriden using `#[schemars(...)]` attributes, which behave identically (e.g. `#[schemars(rename_all = "camelCase")]`). You may find this useful if you want to change the generated schema without affecting Serde's behaviour, or if you're just not using Serde.

### Schema from Example Value

If you want a schema for a type that can't/doesn't implement `JsonSchema`, but does implement `serde::Serialize`, then you can generate a JSON schema from a value of that type. However, this schema will generally be less precise than if the type implemented `JsonSchema` - particularly when it involves enums, since schemars will not make any assumptions about the structure of an enum based on a single variant.

```rust
use schemars::schema_for_value;
use serde::Serialize;

#[derive(Serialize)]
pub struct MyStruct {
    pub my_int: i32,
    pub my_bool: bool,
    pub my_nullable_enum: Option<MyEnum>,
}

#[derive(Serialize)]
pub enum MyEnum {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}

let schema = schema_for_value!(MyStruct {
    my_int: 123,
    my_bool: true,
    my_nullable_enum: Some(MyEnum::StringNewType("foo".to_string()))
});
println!("{}", serde_json::to_string_pretty(&schema).unwrap());
```

<details>
<summary>Click to see the output JSON schema...</summary>

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "MyStruct",
  "examples": [
    {
      "my_bool": true,
      "my_int": 123,
      "my_nullable_enum": {
        "StringNewType": "foo"
      }
    }
  ],
  "type": "object",
  "properties": {
    "my_bool": {
      "type": "boolean"
    },
    "my_int": {
      "type": "integer"
    },
    "my_nullable_enum": true
  }
}
```

</details>

## Feature Flags

- `derive` (enabled by default) - provides `#[derive(JsonSchema)]` macro
- `impl_json_schema` - implements `JsonSchema` for Schemars types themselves
- `preserve_order` - keep the order of struct fields in `Schema` and `SchemaObject`
- `raw_value` - implements `JsonSchema` for `serde_json::value::RawValue` (enables the serde_json `raw_value` feature)

Schemars can implement `JsonSchema` on types from several popular crates, enabled via feature flags (dependency versions are shown in brackets):

- `chrono` - [chrono](https://crates.io/crates/chrono) (^0.4)
- `indexmap1` - [indexmap](https://crates.io/crates/indexmap) (^1.2)
- `indexmap2` - [indexmap](https://crates.io/crates/indexmap) (^2.0)
- `either` - [either](https://crates.io/crates/either) (^1.3)
- `uuid08` - [uuid](https://crates.io/crates/uuid) (^0.8)
- `uuid1` - [uuid](https://crates.io/crates/uuid) (^1.0)
- `smallvec` - [smallvec](https://crates.io/crates/smallvec) (^1.0)
- `arrayvec05` - [arrayvec](https://crates.io/crates/arrayvec) (^0.5)
- `arrayvec07` - [arrayvec](https://crates.io/crates/arrayvec) (^0.7)
- `url` - [url](https://crates.io/crates/url) (^2.0)
- `bytes` - [bytes](https://crates.io/crates/bytes) (^1.0)
- `enumset` - [enumset](https://crates.io/crates/enumset) (^1.0)
- `rust_decimal` - [rust_decimal](https://crates.io/crates/rust_decimal) (^1.0)
- `bigdecimal03` - [bigdecimal](https://crates.io/crates/bigdecimal) (^0.3)
- `bigdecimal04` - [bigdecimal](https://crates.io/crates/bigdecimal) (^0.4)
- `smol_str` - [smol_str](https://crates.io/crates/smol_str) (^0.1.17)
- `semver` - [semver](https://crates.io/crates/semver) (^1.0.9)

For example, to implement `JsonSchema` on types from `chrono`, enable it as a feature in the `schemars` dependency in your `Cargo.toml` like so:

```toml
[dependencies]
schemars = { version = "0.8", features = ["chrono"] }
```

## Modules

## Module `gen`

JSON Schema generator and settings.

This module is useful if you want more control over how the schema generated than the [`schema_for!`] macro gives you.
There are two main types in this module:
* [`SchemaSettings`], which defines what JSON Schema features should be used when generating schemas (for example, how `Option`s should be represented).
* [`SchemaGenerator`], which manages the generation of a schema document.

```rust
pub mod gen { /* ... */ }
```

### Types

#### Struct `SchemaSettings`

**Attributes:**

- `#[non_exhaustive]`

Settings to customize how Schemas are generated.

The default settings currently conform to [JSON Schema Draft 7](https://json-schema.org/specification-links.html#draft-7), but this is liable to change in a future version of Schemars if support for other JSON Schema versions is added.
If you require your generated schemas to conform to draft 7, consider using the [`draft07`](#method.draft07) method.

```rust
pub struct SchemaSettings {
    pub option_nullable: bool,
    pub option_add_null_type: bool,
    pub definitions_path: String,
    pub meta_schema: Option<String>,
    pub visitors: Vec<Box<dyn GenVisitor>>,
    pub inline_subschemas: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `option_nullable` | `bool` | If `true`, schemas for [`Option<T>`](Option) will include a `nullable` property.<br><br>This is not part of the JSON Schema spec, but is used in Swagger/OpenAPI schemas.<br><br>Defaults to `false`. |
| `option_add_null_type` | `bool` | If `true`, schemas for [`Option<T>`](Option) will have `null` added to their [`type`](../schema/struct.SchemaObject.html#structfield.instance_type).<br><br>Defaults to `true`. |
| `definitions_path` | `String` | A JSON pointer to the expected location of referenceable subschemas within the resulting root schema.<br><br>Defaults to `"#/definitions/"`. |
| `meta_schema` | `Option<String>` | The URI of the meta-schema describing the structure of the generated schemas.<br><br>Defaults to `"http://json-schema.org/draft-07/schema#"`. |
| `visitors` | `Vec<Box<dyn GenVisitor>>` | A list of visitors that get applied to all generated root schemas. |
| `inline_subschemas` | `bool` | Inline all subschemas instead of using references.<br><br>Some references may still be generated in schemas for recursive types.<br><br>Defaults to `false`. |

##### Implementations

###### Methods

- ```rust
  pub fn draft07() -> SchemaSettings { /* ... */ }
  ```
  Creates `SchemaSettings` that conform to [JSON Schema Draft 7](https://json-schema.org/specification-links.html#draft-7).

- ```rust
  pub fn draft2019_09() -> SchemaSettings { /* ... */ }
  ```
  Creates `SchemaSettings` that conform to [JSON Schema 2019-09](https://json-schema.org/specification-links.html#2019-09-formerly-known-as-draft-8).

- ```rust
  pub fn openapi3() -> SchemaSettings { /* ... */ }
  ```
  Creates `SchemaSettings` that conform to [OpenAPI 3.0](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.0.md#schemaObject).

- ```rust
  pub fn with</* synthetic */ impl FnOnce(&mut Self): FnOnce(&mut Self)>(self: Self, configure_fn: impl FnOnce(&mut Self)) -> Self { /* ... */ }
  ```
  Modifies the `SchemaSettings` by calling the given function.

- ```rust
  pub fn with_visitor</* synthetic */ impl Visitor + Debug + Clone + 'static: Visitor + Debug + Clone + ''static>(self: Self, visitor: impl Visitor + Debug + Clone + ''static) -> Self { /* ... */ }
  ```
  Appends the given visitor to the list of [visitors](SchemaSettings::visitors) for these `SchemaSettings`.

- ```rust
  pub fn into_generator(self: Self) -> SchemaGenerator { /* ... */ }
  ```
  Creates a new [`SchemaGenerator`] using these settings.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(settings: SchemaSettings) -> Self { /* ... */ }
    ```

- **Send**
- **Sync**
- **Default**
  - ```rust
    fn default() -> SchemaSettings { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SchemaSettings { /* ... */ }
    ```

- **Freeze**
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

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

#### Struct `SchemaGenerator`

The main type used to generate JSON Schemas.

# Example
```
use schemars::{JsonSchema, r#gen::SchemaGenerator};

#[derive(JsonSchema)]
struct MyStruct {
    foo: i32,
}

let generator = SchemaGenerator::default();
let schema = generator.into_root_schema_for::<MyStruct>();
```

```rust
pub struct SchemaGenerator {
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
  pub fn new(settings: SchemaSettings) -> SchemaGenerator { /* ... */ }
  ```
  Creates a new `SchemaGenerator` using the given settings.

- ```rust
  pub fn settings(self: &Self) -> &SchemaSettings { /* ... */ }
  ```
  Borrows the [`SchemaSettings`] being used by this `SchemaGenerator`.

- ```rust
  pub fn make_extensible(self: &Self, _schema: &mut SchemaObject) { /* ... */ }
  ```

- ```rust
  pub fn schema_for_any(self: &Self) -> Schema { /* ... */ }
  ```

- ```rust
  pub fn schema_for_none(self: &Self) -> Schema { /* ... */ }
  ```

- ```rust
  pub fn subschema_for<T: ?Sized + JsonSchema>(self: &mut Self) -> Schema { /* ... */ }
  ```
  Generates a JSON Schema for the type `T`, and returns either the schema itself or a `$ref` schema referencing `T`'s schema.

- ```rust
  pub fn definitions(self: &Self) -> &Map<String, Schema> { /* ... */ }
  ```
  Borrows the collection of all [referenceable](JsonSchema::is_referenceable) schemas that have been generated.

- ```rust
  pub fn definitions_mut(self: &mut Self) -> &mut Map<String, Schema> { /* ... */ }
  ```
  Mutably borrows the collection of all [referenceable](JsonSchema::is_referenceable) schemas that have been generated.

- ```rust
  pub fn take_definitions(self: &mut Self) -> Map<String, Schema> { /* ... */ }
  ```
  Returns the collection of all [referenceable](JsonSchema::is_referenceable) schemas that have been generated,

- ```rust
  pub fn visitors_mut(self: &mut Self) -> impl Iterator<Item = &mut dyn GenVisitor> { /* ... */ }
  ```
  Returns an iterator over the [visitors](SchemaSettings::visitors) being used by this `SchemaGenerator`.

- ```rust
  pub fn root_schema_for<T: ?Sized + JsonSchema>(self: &mut Self) -> RootSchema { /* ... */ }
  ```
  Generates a root JSON Schema for the type `T`.

- ```rust
  pub fn into_root_schema_for<T: ?Sized + JsonSchema>(self: Self) -> RootSchema { /* ... */ }
  ```
  Consumes `self` and generates a root JSON Schema for the type `T`.

- ```rust
  pub fn root_schema_for_value<T: ?Sized + Serialize>(self: &mut Self, value: &T) -> Result<RootSchema, serde_json::Error> { /* ... */ }
  ```
  Generates a root JSON Schema for the given example value.

- ```rust
  pub fn into_root_schema_for_value<T: ?Sized + Serialize>(self: Self, value: &T) -> Result<RootSchema, serde_json::Error> { /* ... */ }
  ```
  Consumes `self` and generates a root JSON Schema for the given example value.

- ```rust
  pub fn dereference<''a>(self: &''a Self, schema: &Schema) -> Option<&''a Schema> { /* ... */ }
  ```
  Attemps to find the schema that the given `schema` is referencing.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(settings: SchemaSettings) -> Self { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> SchemaGenerator { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `GenVisitor`

A [Visitor](Visitor) which implements additional traits required to be included in a [SchemaSettings].

You will rarely need to use this trait directly as it is automatically implemented for any type which implements all of:
- [`Visitor`]
- [`std::fmt::Debug`]
- [`std::any::Any`] (implemented for all `'static` types)
- [`std::clone::Clone`]

# Example
```
use schemars::visit::Visitor;
use schemars::r#gen::GenVisitor;

#[derive(Debug, Clone)]
struct MyVisitor;

impl Visitor for MyVisitor { }

let v: &dyn GenVisitor = &MyVisitor;
assert!(v.as_any().is::<MyVisitor>());
```

```rust
pub trait GenVisitor: Visitor + Debug + DynClone + Any {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_any`: Upcasts this visitor into an `Any`, which can be used to inspect and manipulate it as its concrete type.

##### Implementations

This trait is implemented for the following types:

- `T` with <T>

## Module `schema`

JSON Schema types.

```rust
pub mod schema { /* ... */ }
```

### Types

#### Enum `Schema`

**Attributes:**

- `#[allow(clippy::large_enum_variant)]`
- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(untagged)]`

A JSON Schema.

```rust
pub enum Schema {
    Bool(bool),
    Object(SchemaObject),
}
```

##### Variants

###### `Bool`

A trivial boolean JSON Schema.

The schema `true` matches everything (always passes validation), whereas the schema `false`
matches nothing (always fails validation).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

###### `Object`

A JSON Schema object.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SchemaObject` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new_ref(reference: String) -> Self { /* ... */ }
  ```
  Creates a new `$ref` schema.

- ```rust
  pub fn is_ref(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if `self` is a `$ref` schema.

- ```rust
  pub fn into_object(self: Self) -> SchemaObject { /* ... */ }
  ```
  Converts the given schema (if it is a boolean schema) into an equivalent schema object.

###### Trait Implementations

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

  - ```rust
    fn from(o: SchemaObject) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(b: bool) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(schema: Schema) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeOwned**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
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

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Unpin**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Schema) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
    fn clone(self: &Self) -> Schema { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `RootSchema`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase", default)]`

The root object of a JSON Schema document.

```rust
pub struct RootSchema {
    pub meta_schema: Option<String>,
    pub schema: SchemaObject,
    pub definitions: crate::Map<String, Schema>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `meta_schema` | `Option<String>` | The `$schema` keyword.<br><br>See [JSON Schema 8.1.1. The "$schema" Keyword](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-8.1.1). |
| `schema` | `SchemaObject` | The root schema itself. |
| `definitions` | `crate::Map<String, Schema>` | The `definitions` keyword.<br><br>In JSON Schema draft 2019-09 this was replaced by $defs, but in Schemars this is still<br>serialized as `definitions` for backward-compatibility.<br><br>See [JSON Schema 8.2.5. Schema Re-Use With "$defs"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-8.2.5),<br>and [JSON Schema (draft 07) 9. Schema Re-Use With "definitions"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-01#section-9). |

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> RootSchema { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RootSchema) -> bool { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **DeserializeOwned**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Unpin**
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RootSchema { /* ... */ }
    ```

- **Sync**
#### Struct `SchemaObject`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase", default)]`

A JSON Schema object.

```rust
pub struct SchemaObject {
    pub metadata: Option<Box<Metadata>>,
    pub instance_type: Option<SingleOrVec<InstanceType>>,
    pub format: Option<String>,
    pub enum_values: Option<Vec<serde_json::Value>>,
    pub const_value: Option<serde_json::Value>,
    pub subschemas: Option<Box<SubschemaValidation>>,
    pub number: Option<Box<NumberValidation>>,
    pub string: Option<Box<StringValidation>>,
    pub array: Option<Box<ArrayValidation>>,
    pub object: Option<Box<ObjectValidation>>,
    pub reference: Option<String>,
    pub extensions: crate::Map<String, serde_json::Value>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `metadata` | `Option<Box<Metadata>>` | Properties which annotate the [`SchemaObject`] which typically have no effect when an object is being validated against the schema. |
| `instance_type` | `Option<SingleOrVec<InstanceType>>` | The `type` keyword.<br><br>See [JSON Schema Validation 6.1.1. "type"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.1.1)<br>and [JSON Schema 4.2.1. Instance Data Model](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-4.2.1). |
| `format` | `Option<String>` | The `format` keyword.<br><br>See [JSON Schema Validation 7. A Vocabulary for Semantic Content With "format"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-7). |
| `enum_values` | `Option<Vec<serde_json::Value>>` | The `enum` keyword.<br><br>See [JSON Schema Validation 6.1.2. "enum"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.1.2) |
| `const_value` | `Option<serde_json::Value>` | The `const` keyword.<br><br>See [JSON Schema Validation 6.1.3. "const"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.1.3) |
| `subschemas` | `Option<Box<SubschemaValidation>>` | Properties of the [`SchemaObject`] which define validation assertions in terms of other schemas. |
| `number` | `Option<Box<NumberValidation>>` | Properties of the [`SchemaObject`] which define validation assertions for numbers. |
| `string` | `Option<Box<StringValidation>>` | Properties of the [`SchemaObject`] which define validation assertions for strings. |
| `array` | `Option<Box<ArrayValidation>>` | Properties of the [`SchemaObject`] which define validation assertions for arrays. |
| `object` | `Option<Box<ObjectValidation>>` | Properties of the [`SchemaObject`] which define validation assertions for objects. |
| `reference` | `Option<String>` | The `$ref` keyword.<br><br>See [JSON Schema 8.2.4.1. Direct References with "$ref"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-8.2.4.1). |
| `extensions` | `crate::Map<String, serde_json::Value>` | Arbitrary extra properties which are not part of the JSON Schema specification, or which `schemars` does not support. |

##### Implementations

###### Methods

- ```rust
  pub fn new_ref(reference: String) -> Self { /* ... */ }
  ```
  Creates a new `$ref` schema.

- ```rust
  pub fn is_ref(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if `self` is a `$ref` schema.

- ```rust
  pub fn has_type(self: &Self, ty: InstanceType) -> bool { /* ... */ }
  ```
  Returns `true` if `self` accepts values of the given type, according to the [`instance_type`](struct.SchemaObject.html#structfield.instance_type) field.

- ```rust
  pub fn metadata(self: &mut Self) -> &mut Metadata { /* ... */ }
  ```
  Returns a mutable reference to this schema's [`Metadata`](#structfield.metadata), creating it if it was `None`.

- ```rust
  pub fn subschemas(self: &mut Self) -> &mut SubschemaValidation { /* ... */ }
  ```
  Returns a mutable reference to this schema's [`SubschemaValidation`](#structfield.subschemas), creating it if it was `None`.

- ```rust
  pub fn number(self: &mut Self) -> &mut NumberValidation { /* ... */ }
  ```
  Returns a mutable reference to this schema's [`NumberValidation`](#structfield.number), creating it if it was `None`.

- ```rust
  pub fn string(self: &mut Self) -> &mut StringValidation { /* ... */ }
  ```
  Returns a mutable reference to this schema's [`StringValidation`](#structfield.string), creating it if it was `None`.

- ```rust
  pub fn array(self: &mut Self) -> &mut ArrayValidation { /* ... */ }
  ```
  Returns a mutable reference to this schema's [`ArrayValidation`](#structfield.array), creating it if it was `None`.

- ```rust
  pub fn object(self: &mut Self) -> &mut ObjectValidation { /* ... */ }
  ```
  Returns a mutable reference to this schema's [`ObjectValidation`](#structfield.object), creating it if it was `None`.

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SchemaObject { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SchemaObject { /* ... */ }
    ```

- **Freeze**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **DeserializeOwned**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SchemaObject) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(o: SchemaObject) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(schema: Schema) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Metadata`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase", default)]`

Properties which annotate a [`SchemaObject`] which typically have no effect when an object is being validated against the schema.

```rust
pub struct Metadata {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<serde_json::Value>,
    pub deprecated: bool,
    pub read_only: bool,
    pub write_only: bool,
    pub examples: Vec<serde_json::Value>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `Option<String>` | The `$id` keyword.<br><br>See [JSON Schema 8.2.2. The "$id" Keyword](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-8.2.2). |
| `title` | `Option<String>` | The `title` keyword.<br><br>See [JSON Schema Validation 9.1. "title" and "description"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-9.1). |
| `description` | `Option<String>` | The `description` keyword.<br><br>See [JSON Schema Validation 9.1. "title" and "description"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-9.1). |
| `default` | `Option<serde_json::Value>` | The `default` keyword.<br><br>See [JSON Schema Validation 9.2. "default"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-9.2). |
| `deprecated` | `bool` | The `deprecated` keyword.<br><br>See [JSON Schema Validation 9.3. "deprecated"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-9.3). |
| `read_only` | `bool` | The `readOnly` keyword.<br><br>See [JSON Schema Validation 9.4. "readOnly" and "writeOnly"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-9.4). |
| `write_only` | `bool` | The `writeOnly` keyword.<br><br>See [JSON Schema Validation 9.4. "readOnly" and "writeOnly"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-9.4). |
| `examples` | `Vec<serde_json::Value>` | The `examples` keyword.<br><br>See [JSON Schema Validation 9.5. "examples"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-9.5). |

##### Implementations

###### Trait Implementations

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

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Metadata { /* ... */ }
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

- **RefUnwindSafe**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **DeserializeOwned**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Metadata) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Metadata { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
#### Struct `SubschemaValidation`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase", default)]`

Properties of a [`SchemaObject`] which define validation assertions in terms of other schemas.

```rust
pub struct SubschemaValidation {
    pub all_of: Option<Vec<Schema>>,
    pub any_of: Option<Vec<Schema>>,
    pub one_of: Option<Vec<Schema>>,
    pub not: Option<Box<Schema>>,
    pub if_schema: Option<Box<Schema>>,
    pub then_schema: Option<Box<Schema>>,
    pub else_schema: Option<Box<Schema>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `all_of` | `Option<Vec<Schema>>` | The `allOf` keyword.<br><br>See [JSON Schema 9.2.1.1. "allOf"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.2.1.1). |
| `any_of` | `Option<Vec<Schema>>` | The `anyOf` keyword.<br><br>See [JSON Schema 9.2.1.2. "anyOf"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.2.1.2). |
| `one_of` | `Option<Vec<Schema>>` | The `oneOf` keyword.<br><br>See [JSON Schema 9.2.1.3. "oneOf"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.2.1.3). |
| `not` | `Option<Box<Schema>>` | The `not` keyword.<br><br>See [JSON Schema 9.2.1.4. "not"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.2.1.4). |
| `if_schema` | `Option<Box<Schema>>` | The `if` keyword.<br><br>See [JSON Schema 9.2.2.1. "if"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.2.2.1). |
| `then_schema` | `Option<Box<Schema>>` | The `then` keyword.<br><br>See [JSON Schema 9.2.2.2. "then"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.2.2.2). |
| `else_schema` | `Option<Box<Schema>>` | The `else` keyword.<br><br>See [JSON Schema 9.2.2.3. "else"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.2.2.3). |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SubschemaValidation) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SubschemaValidation { /* ... */ }
    ```

- **Sync**
- **Unpin**
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

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DeserializeOwned**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **Default**
  - ```rust
    fn default() -> SubschemaValidation { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

#### Struct `NumberValidation`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase", default)]`

Properties of a [`SchemaObject`] which define validation assertions for numbers.

```rust
pub struct NumberValidation {
    pub multiple_of: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_maximum: Option<f64>,
    pub minimum: Option<f64>,
    pub exclusive_minimum: Option<f64>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `multiple_of` | `Option<f64>` | The `multipleOf` keyword.<br><br>See [JSON Schema Validation 6.2.1. "multipleOf"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.2.1). |
| `maximum` | `Option<f64>` | The `maximum` keyword.<br><br>See [JSON Schema Validation 6.2.2. "maximum"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.2.2). |
| `exclusive_maximum` | `Option<f64>` | The `exclusiveMaximum` keyword.<br><br>See [JSON Schema Validation 6.2.3. "exclusiveMaximum"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.2.3). |
| `minimum` | `Option<f64>` | The `minimum` keyword.<br><br>See [JSON Schema Validation 6.2.4. "minimum"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.2.4). |
| `exclusive_minimum` | `Option<f64>` | The `exclusiveMinimum` keyword.<br><br>See [JSON Schema Validation 6.2.5. "exclusiveMinimum"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.2.5). |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NumberValidation) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **UnwindSafe**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NumberValidation { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> NumberValidation { /* ... */ }
    ```

- **Unpin**
- **DeserializeOwned**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Freeze**
- **Sync**
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

#### Struct `StringValidation`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase", default)]`

Properties of a [`SchemaObject`] which define validation assertions for strings.

```rust
pub struct StringValidation {
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
    pub pattern: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `max_length` | `Option<u32>` | The `maxLength` keyword.<br><br>See [JSON Schema Validation 6.3.1. "maxLength"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.3.1). |
| `min_length` | `Option<u32>` | The `minLength` keyword.<br><br>See [JSON Schema Validation 6.3.2. "minLength"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.3.2). |
| `pattern` | `Option<String>` | The `pattern` keyword.<br><br>See [JSON Schema Validation 6.3.3. "pattern"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.3.3). |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StringValidation { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> StringValidation { /* ... */ }
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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StringValidation) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DeserializeOwned**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

#### Struct `ArrayValidation`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase", default)]`

Properties of a [`SchemaObject`] which define validation assertions for arrays.

```rust
pub struct ArrayValidation {
    pub items: Option<SingleOrVec<Schema>>,
    pub additional_items: Option<Box<Schema>>,
    pub max_items: Option<u32>,
    pub min_items: Option<u32>,
    pub unique_items: Option<bool>,
    pub contains: Option<Box<Schema>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `items` | `Option<SingleOrVec<Schema>>` | The `items` keyword.<br><br>See [JSON Schema 9.3.1.1. "items"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.3.1.1). |
| `additional_items` | `Option<Box<Schema>>` | The `additionalItems` keyword.<br><br>See [JSON Schema 9.3.1.2. "additionalItems"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.3.1.2). |
| `max_items` | `Option<u32>` | The `maxItems` keyword.<br><br>See [JSON Schema Validation 6.4.1. "maxItems"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.4.1). |
| `min_items` | `Option<u32>` | The `minItems` keyword.<br><br>See [JSON Schema Validation 6.4.2. "minItems"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.4.2). |
| `unique_items` | `Option<bool>` | The `uniqueItems` keyword.<br><br>See [JSON Schema Validation 6.4.3. "uniqueItems"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.4.3). |
| `contains` | `Option<Box<Schema>>` | The `contains` keyword.<br><br>See [JSON Schema 9.3.1.4. "contains"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.3.1.4). |

##### Implementations

###### Trait Implementations

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArrayValidation { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
- **UnwindSafe**
- **DeserializeOwned**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArrayValidation) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArrayValidation { /* ... */ }
    ```

#### Struct `ObjectValidation`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase", default)]`

Properties of a [`SchemaObject`] which define validation assertions for objects.

```rust
pub struct ObjectValidation {
    pub max_properties: Option<u32>,
    pub min_properties: Option<u32>,
    pub required: crate::Set<String>,
    pub properties: crate::Map<String, Schema>,
    pub pattern_properties: crate::Map<String, Schema>,
    pub additional_properties: Option<Box<Schema>>,
    pub property_names: Option<Box<Schema>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `max_properties` | `Option<u32>` | The `maxProperties` keyword.<br><br>See [JSON Schema Validation 6.5.1. "maxProperties"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.5.1). |
| `min_properties` | `Option<u32>` | The `minProperties` keyword.<br><br>See [JSON Schema Validation 6.5.2. "minProperties"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.5.2). |
| `required` | `crate::Set<String>` | The `required` keyword.<br><br>See [JSON Schema Validation 6.5.3. "required"](https://tools.ietf.org/html/draft-handrews-json-schema-validation-02#section-6.5.3). |
| `properties` | `crate::Map<String, Schema>` | The `properties` keyword.<br><br>See [JSON Schema 9.3.2.1. "properties"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.3.2.1). |
| `pattern_properties` | `crate::Map<String, Schema>` | The `patternProperties` keyword.<br><br>See [JSON Schema 9.3.2.2. "patternProperties"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.3.2.2). |
| `additional_properties` | `Option<Box<Schema>>` | The `additionalProperties` keyword.<br><br>See [JSON Schema 9.3.2.3. "additionalProperties"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.3.2.3). |
| `property_names` | `Option<Box<Schema>>` | The `propertyNames` keyword.<br><br>See [JSON Schema 9.3.2.5. "propertyNames"](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-9.3.2.5). |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
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

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Freeze**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ObjectValidation { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ObjectValidation) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ObjectValidation { /* ... */ }
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

- **DeserializeOwned**
- **Unpin**
- **RefUnwindSafe**
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

- **Sync**
#### Enum `InstanceType`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(rename_all = "camelCase")]`

The possible types of values in JSON Schema documents.

See [JSON Schema 4.2.1. Instance Data Model](https://tools.ietf.org/html/draft-handrews-json-schema-02#section-4.2.1).

```rust
pub enum InstanceType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}
```

##### Variants

###### `Null`

###### `Boolean`

###### `Object`

###### `Array`

###### `Number`

###### `String`

###### `Integer`

##### Implementations

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InstanceType) -> bool { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InstanceType { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &InstanceType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &InstanceType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **DeserializeOwned**
- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
#### Enum `SingleOrVec`

**Attributes:**

- `#[<cfg_attr>(feature = "impl_json_schema", derive(JsonSchema))]`
- `#[serde(untagged)]`

A type which can be serialized as a single item, or multiple items.

In some contexts, a `Single` may be semantically distinct from a `Vec` containing only item.

```rust
pub enum SingleOrVec<T> {
    Single(Box<T>),
    Vec(Vec<T>),
}
```

##### Variants

###### `Single`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<T>` |  |

###### `Vec`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<T>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn contains(self: &Self, x: &T) -> bool { /* ... */ }
  ```
  Returns `true` if `self` is either a `Single` equal to `x`, or a `Vec` containing `x`.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SingleOrVec<T>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SingleOrVec<T>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **DeserializeOwned**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SingleOrVec<T> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SingleOrVec<T>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: never) -> T { /* ... */ }
    ```

  - ```rust
    fn from(single: T) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<T>) -> Self { /* ... */ }
    ```

- **Send**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **UnwindSafe**
## Module `visit`

Contains the [`Visitor`] trait, used to recursively modify a constructed schema and its subschemas.

Sometimes you may want to apply a change to a schema, as well as all schemas contained within it.
The easiest way to achieve this is by defining a type that implements [`Visitor`].
All methods of `Visitor` have a default implementation that makes no change but recursively visits all subschemas.
When overriding one of these methods, you will *usually* want to still call this default implementation.

# Example
To add a custom property to all schemas:
```
use schemars::schema::SchemaObject;
use schemars::visit::{Visitor, visit_schema_object};

pub struct MyVisitor;

impl Visitor for MyVisitor {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        // First, make our change to this schema
        schema.extensions.insert("my_property".to_string(), serde_json::json!("hello world"));

        // Then delegate to default implementation to visit any subschemas
        visit_schema_object(self, schema);
    }
}
```

```rust
pub mod visit { /* ... */ }
```

### Types

#### Struct `ReplaceBoolSchemas`

This visitor will replace all boolean JSON Schemas with equivalent object schemas.

This is useful for dialects of JSON Schema (e.g. OpenAPI 3.0) that do not support booleans as schemas.

```rust
pub struct ReplaceBoolSchemas {
    pub skip_additional_properties: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `skip_additional_properties` | `bool` | When set to `true`, a schema's `additionalProperties` property will not be changed from a boolean. |

##### Implementations

###### Trait Implementations

- **Unpin**
- **GenVisitor**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any + ''static { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
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

- **Visitor**
  - ```rust
    fn visit_schema(self: &mut Self, schema: &mut Schema) { /* ... */ }
    ```

  - ```rust
    fn visit_schema_object(self: &mut Self, schema: &mut SchemaObject) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ReplaceBoolSchemas { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `RemoveRefSiblings`

This visitor will restructure JSON Schema objects so that the `$ref` property will never appear alongside any other properties.

This is useful for dialects of JSON Schema (e.g. Draft 7) that do not support other properties alongside `$ref`.

```rust
pub struct RemoveRefSiblings;
```

##### Implementations

###### Trait Implementations

- **Send**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RemoveRefSiblings { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **GenVisitor**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any + ''static { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Visitor**
  - ```rust
    fn visit_schema_object(self: &mut Self, schema: &mut SchemaObject) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `SetSingleExample`

This visitor will remove the `examples` schema property and (if present) set its first value as the `example` property.

This is useful for dialects of JSON Schema (e.g. OpenAPI 3.0) that do not support the `examples` property.

```rust
pub struct SetSingleExample {
    pub retain_examples: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `retain_examples` | `bool` | When set to `true`, the `examples` property will not be removed, but its first value will still be copied to `example`. |

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
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

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SetSingleExample { /* ... */ }
    ```

- **Visitor**
  - ```rust
    fn visit_schema_object(self: &mut Self, schema: &mut SchemaObject) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **GenVisitor**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any + ''static { /* ... */ }
    ```

- **Sync**
### Traits

#### Trait `Visitor`

Trait used to recursively modify a constructed schema and its subschemas.

```rust
pub trait Visitor {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn visit_root_schema(self: &mut Self, root: &mut RootSchema) { /* ... */ }
  ```
  Override this method to modify a [`RootSchema`] and (optionally) its subschemas.

- ```rust
  fn visit_schema(self: &mut Self, schema: &mut Schema) { /* ... */ }
  ```
  Override this method to modify a [`Schema`] and (optionally) its subschemas.

- ```rust
  fn visit_schema_object(self: &mut Self, schema: &mut SchemaObject) { /* ... */ }
  ```
  Override this method to modify a [`SchemaObject`] and (optionally) its subschemas.

##### Implementations

This trait is implemented for the following types:

- `ReplaceBoolSchemas`
- `RemoveRefSiblings`
- `SetSingleExample`

### Functions

#### Function `visit_root_schema`

Visits all subschemas of the [`RootSchema`].

```rust
pub fn visit_root_schema<V: Visitor + ?Sized>(v: &mut V, root: &mut crate::schema::RootSchema) { /* ... */ }
```

#### Function `visit_schema`

Visits all subschemas of the [`Schema`].

```rust
pub fn visit_schema<V: Visitor + ?Sized>(v: &mut V, schema: &mut crate::schema::Schema) { /* ... */ }
```

#### Function `visit_schema_object`

Visits all subschemas of the [`SchemaObject`].

```rust
pub fn visit_schema_object<V: Visitor + ?Sized>(v: &mut V, schema: &mut crate::schema::SchemaObject) { /* ... */ }
```

## Types

### Type Alias `Map`

**Attributes:**

- `#[<cfg>(not(feature = "preserve_order"))]`

The map type used by schemars types.

Currently a `BTreeMap` or `IndexMap` can be used, but this may change to a different implementation
with a similar interface in a future version of schemars.
The `IndexMap` will be used when the `preserve_order` feature flag is set.

```rust
pub type Map<K, V> = std::collections::BTreeMap<K, V>;
```

### Type Alias `Set`

The set type used by schemars types.

Currently a `BTreeSet`, but this may change to a different implementation
with a similar interface in a future version of schemars.

```rust
pub type Set<T> = std::collections::BTreeSet<T>;
```

### Type Alias `MapEntry`

**Attributes:**

- `#[<cfg>(not(feature = "preserve_order"))]`

A view into a single entry in a map, which may either be vacant or occupied.
This is constructed from the `entry` method on `BTreeMap` or `IndexMap`,
depending on whether the `preserve_order` feature flag is set.

```rust
pub type MapEntry<''a, K, V> = std::collections::btree_map::Entry<''a, K, V>;
```

## Traits

### Trait `JsonSchema`

A type which can be described as a JSON Schema document.

This is implemented for many Rust primitive and standard library types.

This can also be automatically derived on most custom types with `#[derive(JsonSchema)]`.

# Examples
Deriving an implementation:
```
use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
struct MyStruct {
    foo: i32,
}

let my_schema = schema_for!(MyStruct);
```

When manually implementing `JsonSchema`, as well as determining an appropriate schema,
you will need to determine an appropriate name and ID for the type.
For non-generic types, the type name/path are suitable for this:
```
use schemars::{r#gen::SchemaGenerator, schema::Schema, JsonSchema};
use std::borrow::Cow;

struct NonGenericType;

impl JsonSchema for NonGenericType {
    fn schema_name() -> String {
        // Exclude the module path to make the name in generated schemas clearer.
        "NonGenericType".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        // Include the module, in case a type with the same name is in another module/crate
        Cow::Borrowed(concat!(module_path!(), "::NonGenericType"))
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        todo!()
    }
}

assert_eq!(NonGenericType::schema_id(), <&mut NonGenericType>::schema_id());
```

But generic type parameters which may affect the generated schema should typically be included in the name/ID:
```
use schemars::{r#gen::SchemaGenerator, schema::Schema, JsonSchema};
use std::{borrow::Cow, marker::PhantomData};

struct GenericType<T>(PhantomData<T>);

impl<T: JsonSchema> JsonSchema for GenericType<T> {
    fn schema_name() -> String {
        format!("GenericType_{}", T::schema_name())
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Owned(format!(
            "{}::GenericType<{}>",
            module_path!(),
            T::schema_id()
        ))
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        todo!()
    }
}

assert_eq!(<GenericType<i32>>::schema_id(), <&mut GenericType<&i32>>::schema_id());
```


```rust
pub trait JsonSchema {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `schema_name`: The name of the generated JSON Schema.
- `json_schema`: Generates a JSON Schema for this type.

#### Provided Methods

- ```rust
  fn is_referenceable() -> bool { /* ... */ }
  ```
  Whether JSON Schemas generated for this type should be re-used where possible using the `$ref` keyword.

- ```rust
  fn schema_id() -> Cow<''static, str> { /* ... */ }
  ```
  Returns a string that uniquely identifies the schema produced by this type.

#### Implementations

This trait is implemented for the following types:

- `[T; 0]` with <T>
- `[T; 1]` with <T: JsonSchema>
- `[T; 2]` with <T: JsonSchema>
- `[T; 3]` with <T: JsonSchema>
- `[T; 4]` with <T: JsonSchema>
- `[T; 5]` with <T: JsonSchema>
- `[T; 6]` with <T: JsonSchema>
- `[T; 7]` with <T: JsonSchema>
- `[T; 8]` with <T: JsonSchema>
- `[T; 9]` with <T: JsonSchema>
- `[T; 10]` with <T: JsonSchema>
- `[T; 11]` with <T: JsonSchema>
- `[T; 12]` with <T: JsonSchema>
- `[T; 13]` with <T: JsonSchema>
- `[T; 14]` with <T: JsonSchema>
- `[T; 15]` with <T: JsonSchema>
- `[T; 16]` with <T: JsonSchema>
- `[T; 17]` with <T: JsonSchema>
- `[T; 18]` with <T: JsonSchema>
- `[T; 19]` with <T: JsonSchema>
- `[T; 20]` with <T: JsonSchema>
- `[T; 21]` with <T: JsonSchema>
- `[T; 22]` with <T: JsonSchema>
- `[T; 23]` with <T: JsonSchema>
- `[T; 24]` with <T: JsonSchema>
- `[T; 25]` with <T: JsonSchema>
- `[T; 26]` with <T: JsonSchema>
- `[T; 27]` with <T: JsonSchema>
- `[T; 28]` with <T: JsonSchema>
- `[T; 29]` with <T: JsonSchema>
- `[T; 30]` with <T: JsonSchema>
- `[T; 31]` with <T: JsonSchema>
- `[T; 32]` with <T: JsonSchema>
- `AtomicBool`
- `AtomicI8`
- `AtomicI16`
- `AtomicI32`
- `AtomicI64`
- `AtomicIsize`
- `AtomicU8`
- `AtomicU16`
- `AtomicU32`
- `AtomicU64`
- `AtomicUsize`
- `Weekday`
- `NaiveDate`
- `NaiveDateTime`
- `NaiveTime`
- `DateTime<Tz>` with <Tz: TimeZone>
- `Option<T>` with <T: JsonSchema>
- `Result<T, E>` with <T: JsonSchema, E: JsonSchema>
- `std::ops::Bound<T>` with <T: JsonSchema>
- `std::ops::Range<T>` with <T: JsonSchema>
- `std::ops::RangeInclusive<T>` with <T: JsonSchema>
- `std::marker::PhantomData<T>` with <T: ?Sized>
- `std::fmt::Arguments<''a>` with <''a>
- `std::ffi::OsString`
- `std::ffi::OsStr`
- `std::ffi::CString`
- `std::ffi::CStr`
- `std::collections::BTreeMap<K, V>` with <K, V>
- `std::collections::HashMap<K, V, H>` with <K, V, H>
- `NonZeroI8`
- `NonZeroI16`
- `NonZeroI32`
- `NonZeroI64`
- `NonZeroI128`
- `NonZeroIsize`
- `NonZeroU8`
- `NonZeroU16`
- `NonZeroU32`
- `NonZeroU64`
- `NonZeroU128`
- `NonZeroUsize`
- `str`
- `String`
- `bool`
- `f32`
- `f64`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`
- `()`
- `std::path::Path`
- `std::path::PathBuf`
- `std::net::Ipv4Addr`
- `std::net::Ipv6Addr`
- `std::net::IpAddr`
- `std::net::SocketAddr`
- `std::net::SocketAddrV4`
- `std::net::SocketAddrV6`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`
- `char`
- `std::collections::BinaryHeap<T>` with <T>
- `std::collections::LinkedList<T>` with <T>
- `[T]` with <T>
- `Vec<T>` with <T>
- `std::collections::VecDeque<T>` with <T>
- `std::collections::BTreeSet<T>` with <T>
- `std::collections::HashSet<T, H>` with <T, H>
- `serde_json::Value`
- `serde_json::Map<String, serde_json::Value>`
- `serde_json::Number`
- `serde_json::value::RawValue`
- `std::time::Duration`
- `std::time::SystemTime`
- `(T0)` with <T0: JsonSchema>
- `(T0, T1)` with <T0: JsonSchema, T1: JsonSchema>
- `(T0, T1, T2)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema>
- `(T0, T1, T2, T3)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema>
- `(T0, T1, T2, T3, T4)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema>
- `(T0, T1, T2, T3, T4, T5)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema, T8: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema, T8: JsonSchema, T9: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema, T8: JsonSchema, T9: JsonSchema, T10: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema, T8: JsonSchema, T9: JsonSchema, T10: JsonSchema, T11: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema, T8: JsonSchema, T9: JsonSchema, T10: JsonSchema, T11: JsonSchema, T12: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema, T8: JsonSchema, T9: JsonSchema, T10: JsonSchema, T11: JsonSchema, T12: JsonSchema, T13: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema, T8: JsonSchema, T9: JsonSchema, T10: JsonSchema, T11: JsonSchema, T12: JsonSchema, T13: JsonSchema, T14: JsonSchema>
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)` with <T0: JsonSchema, T1: JsonSchema, T2: JsonSchema, T3: JsonSchema, T4: JsonSchema, T5: JsonSchema, T6: JsonSchema, T7: JsonSchema, T8: JsonSchema, T9: JsonSchema, T10: JsonSchema, T11: JsonSchema, T12: JsonSchema, T13: JsonSchema, T14: JsonSchema, T15: JsonSchema>
- `&''a T` with <''a, T>
- `&''a mut T` with <''a, T>
- `Box<T>` with <T>
- `std::rc::Rc<T>` with <T>
- `std::rc::Weak<T>` with <T>
- `std::sync::Arc<T>` with <T>
- `std::sync::Weak<T>` with <T>
- `std::sync::Mutex<T>` with <T>
- `std::sync::RwLock<T>` with <T>
- `std::cell::Cell<T>` with <T>
- `std::cell::RefCell<T>` with <T>
- `std::borrow::Cow<''a, T>` with <''a, T>
- `std::num::Wrapping<T>` with <T>
- `std::cmp::Reverse<T>` with <T>

## Macros

### Macro `schema_for`

**Attributes:**

- `#[<cfg>(doc)]`
- `#[macro_export]`

Generates a [`RootSchema`](crate::schema::RootSchema) for the given type using default settings.

The type must implement [`JsonSchema`](crate::JsonSchema).

# Example
```
use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
struct MyStruct {
    foo: i32,
}

let my_schema = schema_for!(MyStruct);
```

```rust
pub macro_rules! schema_for {
    /* macro_rules! schema_for {
    ($type:ty) => { ... };
} */
}
```

### Macro `schema_for_value`

**Attributes:**

- `#[macro_export]`

Generates a [`RootSchema`](crate::schema::RootSchema) for the given example value using default settings.

The value must implement [`Serialize`](serde::Serialize). If the value also implements [`JsonSchema`](crate::JsonSchema),
then prefer using the [`schema_for!`](schema_for) macro which will generally produce a more precise schema,
particularly when the value contains any enums.

If the `Serialize` implementation of the value decides to fail, this macro will panic.
For a non-panicking alternative, create a [`SchemaGenerator`](crate::r#gen::SchemaGenerator) and use
its [`into_root_schema_for_value`](crate::r#gen::SchemaGenerator::into_root_schema_for_value) method.

# Example
```
use schemars::schema_for_value;

#[derive(serde::Serialize)]
struct MyStruct {
    foo: i32,
}

let my_schema = schema_for_value!(MyStruct { foo: 123 });
```

```rust
pub macro_rules! schema_for_value {
    /* macro_rules! schema_for_value {
    ($value:expr) => { ... };
} */
}
```

## Re-exports

### Re-export `SchemaGenerator`

```rust
pub use gen::SchemaGenerator;
```

### Re-export `schemars_derive::*`

**Attributes:**

- `#[<cfg>(feature = "schemars_derive")]`

```rust
pub use schemars_derive::*;
```

