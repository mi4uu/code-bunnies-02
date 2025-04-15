# Crate Documentation

**Version:** 0.27.1

**Format Version:** 43

# Module `strum`

# Strum

[![Build Status](https://travis-ci.org/Peternator7/strum.svg?branch=master)](https://travis-ci.org/Peternator7/strum)
[![Latest Version](https://img.shields.io/crates/v/strum.svg)](https://crates.io/crates/strum)
[![Rust Documentation](https://docs.rs/strum/badge.svg)](https://docs.rs/strum)

Strum is a set of macros and traits for working with
enums and strings easier in Rust.

The full version of the README can be found on [GitHub](https://github.com/Peternator7/strum).

# Including Strum in Your Project

Import strum and `strum_macros` into your project by adding the following lines to your
Cargo.toml. `strum_macros` contains the macros needed to derive all the traits in Strum.

```toml
[dependencies]
strum = "0.27"
strum_macros = "0.27"

# You can also access strum_macros exports directly through strum using the "derive" feature
strum = { version = "0.27", features = ["derive"] }
```


## Modules

## Module `additional_attributes`

# Documentation for Additional Attributes

## Attributes on Enums

Strum supports several custom attributes to modify the generated code. At the enum level, the following attributes are supported:

- `#[strum(serialize_all = "case_style")]` attribute can be used to change the case used when serializing to and deserializing
  from strings. This feature is enabled by [withoutboats/heck](https://github.com/withoutboats/heck) and supported case styles are:

  - `camelCase`
  - `PascalCase`
  - `kebab-case`
  - `snake_case`
  - `SCREAMING_SNAKE_CASE`
  - `SCREAMING-KEBAB-CASE`
  - `lowercase`
  - `UPPERCASE`
  - `title_case`
  - `mixed_case`
  - `Train-Case`

  ```rust
  use strum_macros;
   
  #[derive(Debug, Eq, PartialEq, strum_macros::Display)]
  #[strum(serialize_all = "snake_case")]
  enum Brightness {
      DarkBlack,
      Dim {
          glow: usize,
      },
      #[strum(serialize = "bright")]
      BrightWhite,
  }
   
  assert_eq!(
      String::from("dark_black"),
      Brightness::DarkBlack.to_string().as_ref()
  );
  assert_eq!(
      String::from("dim"),
      Brightness::Dim { glow: 0 }.to_string().as_ref()
  );
  assert_eq!(
      String::from("bright"),
      Brightness::BrightWhite.to_string().as_ref()
  );
  ```

- You can also apply the `#[strum(ascii_case_insensitive)]` attribute to the enum,
  and this has the same effect of applying it to every variant.

## Attributes on Variants

Custom attributes are applied to a variant by adding `#[strum(parameter="value")]` to the variant.

- `serialize="..."`: Changes the text that `FromStr()` looks for when parsing a string. This attribute can
   be applied multiple times to an element and the enum variant will be parsed if any of them match.

- `to_string="..."`: Similar to `serialize`. This value will be included when using `FromStr()`. More importantly,
   this specifies what text to use when calling `variant.to_string()` with the `Display` derivation, or when calling `variant.as_ref()` with `AsRefStr`.

- `default`: Applied to a single variant of an enum. The variant must be a Tuple-like
   variant with a single piece of data that can be create from a `&str` i.e. `T: From<&str>`.
   The generated code will now return the variant with the input string captured as shown below
   instead of failing.

    ```text
    // Replaces this:
    _ => Err(strum::ParseError::VariantNotFound)
    // With this in generated code:
    default => Ok(Variant(default.into()))
    ```
    The plugin will fail if the data doesn't implement From<&str>. You can only have one `default`
    on your enum.

- `transparent`: Signals that the inner field's implementation should be used, instead of generating
   one for this variant. Only applicable to enum variants with a single field. Compatible with the
   `AsRefStr`, `Display` and `IntoStaticStr` derive macros. Note that `IntoStaticStr` has a few restrictions,
   the value must be `'static` and `const_into_str` is not supported in combination with `transparent` b/c
   transparent relies on a call on `From::from(variant)`.

- `disabled`: removes variant from generated code.

- `ascii_case_insensitive`: makes the comparison to this variant case insensitive (ASCII only).
  If the whole enum is marked `ascii_case_insensitive`, you can specify `ascii_case_insensitive = false`
  to disable case insensitivity on this variant.

- `message=".."`: Adds a message to enum variant. This is used in conjunction with the `EnumMessage`
   trait to associate a message with a variant. If `detailed_message` is not provided,
   then `message` will also be returned when `get_detailed_message` is called.

- `detailed_message=".."`: Adds a more detailed message to a variant. If this value is omitted, then
   `message` will be used in it's place.

- Structured documentation, as in `/// ...`: If using `EnumMessage`, is accessible via get_documentation().

- `props(key="value")`: Enables associating additional information with a given variant.

```rust
pub mod additional_attributes { /* ... */ }
```

## Types

### Enum `ParseError`

The `ParseError` enum is a collection of all the possible reasons
an enum can fail to parse from a string.

```rust
pub enum ParseError {
    VariantNotFound,
}
```

#### Variants

##### `VariantNotFound`

#### Implementations

##### Trait Implementations

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
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

- **Send**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParseError { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParseError) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> Result<(), std::fmt::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
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

## Traits

### Trait `IntoEnumIterator`

This trait designates that an `Enum` can be iterated over. It can
be auto generated using the [`EnumIter`](derive.EnumIter.html) derive macro.

# Example

```rust
# use std::fmt::Debug;
// You need to bring the type into scope to use it!!!
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug)]
enum Color {
    Red,
    Green { range: usize },
    Blue(usize),
    Yellow,
}

// Iterate over the items in an enum and perform some function on them.
fn generic_iterator<E, F>(pred: F)
where
    E: IntoEnumIterator,
    F: Fn(E),
{
    for e in E::iter() {
        pred(e)
    }
}

generic_iterator::<Color, _>(|color| println!("{:?}", color));
```

```rust
pub trait IntoEnumIterator: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `Iterator`

##### Required Methods

- `iter`

### Trait `VariantIterator`

```rust
pub trait VariantIterator: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `Iterator`

##### Required Methods

- `iter`

### Trait `VariantMetadata`

```rust
pub trait VariantMetadata {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Constants

- `VARIANT_COUNT`
- `VARIANT_NAMES`

##### Required Methods

- `variant_name`

### Trait `EnumMessage`

Associates additional pieces of information with an Enum. This can be
autoimplemented by deriving `EnumMessage` and annotating your variants with
`#[strum(message="...")]`.

# Example

```rust
# use std::fmt::Debug;
// You need to bring the type into scope to use it!!!
use strum::EnumMessage;

#[derive(PartialEq, Eq, Debug, EnumMessage)]
enum Pet {
    #[strum(message="I have a dog")]
    #[strum(detailed_message="My dog's name is Spots")]
    Dog,
    /// I am documented.
    #[strum(message="I don't have a cat")]
    Cat,
}

let my_pet = Pet::Dog;
assert_eq!("I have a dog", my_pet.get_message().unwrap());
```

```rust
pub trait EnumMessage {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `get_message`
- `get_detailed_message`
- `get_documentation`: Get the doc comment associated with a variant if it exists.
- `get_serializations`

### Trait `EnumProperty`

`EnumProperty` is a trait that makes it possible to store additional information
with enum variants. This trait is designed to be used with the macro of the same
name in the `strum_macros` crate. Currently, the string, integer and bool literals
are supported in attributes.

# Example

```rust
# use std::fmt::Debug;
// You need to bring the type into scope to use it!!!
use strum::EnumProperty;

#[derive(PartialEq, Eq, Debug, EnumProperty)]
enum Class {
    #[strum(props(Teacher="Ms.Frizzle", Room="201", students=16, mandatory=true))]
    History,
    #[strum(props(Teacher="Mr.Smith"))]
    #[strum(props(Room="103", students=10))]
    Mathematics,
    #[strum(props(Time="2:30", mandatory=true))]
    Science,
}

let history = Class::History;
assert_eq!("Ms.Frizzle", history.get_str("Teacher").unwrap());
assert_eq!(16, history.get_int("students").unwrap());
assert!(history.get_bool("mandatory").unwrap());
```

```rust
pub trait EnumProperty {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `get_str`
- `get_int`
- `get_bool`

### Trait `AsStaticRef`

**⚠️ Deprecated since 0.22.0**: please use `#[derive(IntoStaticStr)]` instead

A cheap reference-to-reference conversion. Used to convert a value to a
reference value with `'static` lifetime within generic code.

```rust
pub trait AsStaticRef<T>
where
    T: ?Sized {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `as_static`

### Trait `EnumCount`

A trait for capturing the number of variants in Enum. This trait can be autoderived by
`strum_macros`.

```rust
pub trait EnumCount {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Constants

- `COUNT`

### Trait `VariantNames`

A trait for retrieving the names of each variant in Enum. This trait can
be autoderived by `strum_macros`.

```rust
pub trait VariantNames {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Constants

- `VARIANTS`: Names of the variants of this enum

### Trait `IntoDiscriminant`

A trait for retrieving the enum generated by [`EnumDiscriminants`] from an associated
Type on the original enumeration. This trait can be autoderived by `strum_macros`.

```rust
pub trait IntoDiscriminant {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Discriminant`: Enum listing the same variants as this enum but without any data fields

##### Required Methods

- `discriminant`

### Trait `VariantArray`

A trait for retrieving a static array containing all the variants in an Enum.
This trait can be autoderived by `strum_macros`. For derived usage, all the
variants in the enumerator need to be unit-types, which means you can't autoderive
enums with inner data in one or more variants. Consider using it alongside
[`EnumDiscriminants`] if you require inner data but still want to have an
static array of variants.

```rust
pub trait VariantArray: ::core::marker::Sized + ''static {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Constants

- `VARIANTS`

