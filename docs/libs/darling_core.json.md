# Crate Documentation

**Version:** 0.20.11

**Format Version:** 43

# Module `darling_core`

## Modules

## Module `ast`

Utility types for working with the AST.

```rust
pub mod ast { /* ... */ }
```

### Re-exports

#### Re-export `GenericParam`

```rust
pub use self::generics::GenericParam;
```

#### Re-export `GenericParamExt`

```rust
pub use self::generics::GenericParamExt;
```

#### Re-export `Generics`

```rust
pub use self::generics::Generics;
```

#### Re-export `self::data::*`

```rust
pub use self::data::*;
```

## Module `derive`

Functions to derive `darling`'s traits from well-formed input, without directly depending
on `proc_macro`.

```rust
pub mod derive { /* ... */ }
```

### Functions

#### Function `from_meta`

Create tokens for a `darling::FromMeta` impl from a `DeriveInput`. If
the input cannot produce a valid impl, the returned tokens will contain
compile errors instead.

```rust
pub fn from_meta(input: &syn::DeriveInput) -> proc_macro2::TokenStream { /* ... */ }
```

#### Function `from_attributes`

Create tokens for a `darling::FromAttributes` impl from a `DeriveInput`. If
the input cannot produce a valid impl, the returned tokens will contain
compile errors instead.

```rust
pub fn from_attributes(input: &syn::DeriveInput) -> proc_macro2::TokenStream { /* ... */ }
```

#### Function `from_derive_input`

Create tokens for a `darling::FromDeriveInput` impl from a `DeriveInput`. If
the input cannot produce a valid impl, the returned tokens will contain
compile errors instead.

```rust
pub fn from_derive_input(input: &syn::DeriveInput) -> proc_macro2::TokenStream { /* ... */ }
```

#### Function `from_field`

Create tokens for a `darling::FromField` impl from a `DeriveInput`. If
the input cannot produce a valid impl, the returned tokens will contain
compile errors instead.

```rust
pub fn from_field(input: &syn::DeriveInput) -> proc_macro2::TokenStream { /* ... */ }
```

#### Function `from_type_param`

Create tokens for a `darling::FromTypeParam` impl from a `DeriveInput`. If
the input cannot produce a valid impl, the returned tokens will contain
compile errors instead.

```rust
pub fn from_type_param(input: &syn::DeriveInput) -> proc_macro2::TokenStream { /* ... */ }
```

#### Function `from_variant`

Create tokens for a `darling::FromVariant` impl from a `DeriveInput`. If
the input cannot produce a valid impl, the returned tokens will contain
compile errors instead.

```rust
pub fn from_variant(input: &syn::DeriveInput) -> proc_macro2::TokenStream { /* ... */ }
```

## Module `error`

The `darling::Error` type, the multiple error `Accumulator`, and their internals.

Error handling is one of the core values of `darling`; creating great errors is hard and
never the reason that a proc-macro author started writing their crate. As a result, the
`Error` type in `darling` tries to make adding span information, suggestions, and other
help content easy when manually implementing `darling` traits, and automatic when deriving
them.

```rust
pub mod error { /* ... */ }
```

### Types

#### Type Alias `Result`

An alias of `Result` specific to attribute parsing.

```rust
pub type Result<T> = ::std::result::Result<T, Error>;
```

#### Struct `Error`

An error encountered during attribute parsing.

Given that most errors darling encounters represent code bugs in dependent crates,
the internal structure of the error is deliberately opaque.

# Usage
Proc-macro expansion happens very infrequently compared to runtime tasks such as
deserialization, and it happens in the context of an expensive compilation taks.
For that reason, darling prefers not to fail on the first error it encounters, instead
doing as much work as it can, accumulating errors into a single report.

As a result, `darling::Error` is more of guaranteed-non-empty error collection
than a single problem. These errors also have some notion of hierarchy, stemming from
the hierarchical nature of darling's input.

These characteristics make for great experiences when using darling-powered crates,
provided crates using darling adhere to some best practices:

1. Do not attempt to simplify a `darling::Error` into some other error type, such as
   `syn::Error`. To surface compile errors, instead use `darling::Error::write_errors`.
   This preserves all span information, suggestions, etc. Wrapping a `darling::Error` in
   a custom error enum works as-expected and does not force any loss of fidelity.
2. Do not use early return (e.g. the `?` operator) for custom validations. Instead,
   create an [`error::Accumulator`](Accumulator) to collect errors as they are encountered.  Then use
   [`Accumulator::finish`] to return your validated result; it will give `Ok` if and only if
   no errors were encountered.  This can create very complex custom validation functions;
   in those cases, split independent "validation chains" out into their own functions to
   keep the main validator manageable.
3. Use `darling::Error::custom` to create additional errors as-needed, then call `with_span`
   to ensure those errors appear in the right place. Use `darling::util::SpannedValue` to keep
   span information around on parsed fields so that custom diagnostics can point to the correct
   parts of the input AST.

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
  pub fn custom<T: fmt::Display>(msg: T) -> Self { /* ... */ }
  ```
  Creates a new error with a custom message.

- ```rust
  pub fn duplicate_field(name: &str) -> Self { /* ... */ }
  ```
  Creates a new error for a field that appears twice in the input.

- ```rust
  pub fn duplicate_field_path(path: &Path) -> Self { /* ... */ }
  ```
  Creates a new error for a field that appears twice in the input. Helper to avoid repeating

- ```rust
  pub fn missing_field(name: &str) -> Self { /* ... */ }
  ```
  Creates a new error for a non-optional field that does not appear in the input.

- ```rust
  pub fn unknown_field(name: &str) -> Self { /* ... */ }
  ```
  Creates a new error for a field name that appears in the input but does not correspond

- ```rust
  pub fn unknown_field_path(path: &Path) -> Self { /* ... */ }
  ```
  Creates a new error for a field name that appears in the input but does not correspond

- ```rust
  pub fn unknown_field_with_alts<''a, T, I>(field: &str, alternates: I) -> Self
where
    T: AsRef<str> + ''a,
    I: IntoIterator<Item = &''a T> { /* ... */ }
  ```
  Creates a new error for a field name that appears in the input but does not correspond to

- ```rust
  pub fn unknown_field_path_with_alts<''a, T, I>(field: &Path, alternates: I) -> Self
where
    T: AsRef<str> + ''a,
    I: IntoIterator<Item = &''a T> { /* ... */ }
  ```
  Creates a new error for a field name that appears in the input but does not correspond to

- ```rust
  pub fn unsupported_shape(shape: &str) -> Self { /* ... */ }
  ```
  Creates a new error for a struct or variant that does not adhere to the supported shape.

- ```rust
  pub fn unsupported_shape_with_expected<T: fmt::Display>(shape: &str, expected: &T) -> Self { /* ... */ }
  ```

- ```rust
  pub fn unsupported_format(format: &str) -> Self { /* ... */ }
  ```

- ```rust
  pub fn unexpected_type(ty: &str) -> Self { /* ... */ }
  ```
  Creates a new error for a field which has an unexpected literal type.

- ```rust
  pub fn unexpected_expr_type(expr: &Expr) -> Self { /* ... */ }
  ```

- ```rust
  pub fn unexpected_lit_type(lit: &Lit) -> Self { /* ... */ }
  ```
  Creates a new error for a field which has an unexpected literal type. This will automatically

- ```rust
  pub fn unknown_value(value: &str) -> Self { /* ... */ }
  ```
  Creates a new error for a value which doesn't match a set of expected literals.

- ```rust
  pub fn too_few_items(min: usize) -> Self { /* ... */ }
  ```
  Creates a new error for a list which did not get enough items to proceed.

- ```rust
  pub fn too_many_items(max: usize) -> Self { /* ... */ }
  ```
  Creates a new error when a list got more items than it supports. The `max` argument

- ```rust
  pub fn multiple(errors: Vec<Error>) -> Self { /* ... */ }
  ```
  Bundle a set of multiple errors into a single `Error` instance.

- ```rust
  pub fn accumulator() -> Accumulator { /* ... */ }
  ```
  Creates an error collector, for aggregating multiple errors

- ```rust
  pub fn has_span(self: &Self) -> bool { /* ... */ }
  ```
  Check if this error is associated with a span in the token stream.

- ```rust
  pub fn with_span<T: Spanned>(self: Self, node: &T) -> Self { /* ... */ }
  ```
  Tie a span to the error if none is already present. This is used in `darling::FromMeta`

- ```rust
  pub fn span(self: &Self) -> Span { /* ... */ }
  ```
  Get a span for the error.

- ```rust
  pub fn explicit_span(self: &Self) -> Option<Span> { /* ... */ }
  ```
  Get the span for `self`, if one has been set.

- ```rust
  pub fn flatten(self: Self) -> Self { /* ... */ }
  ```
  Recursively converts a tree of errors to a flattened list.

- ```rust
  pub fn at<T: fmt::Display>(self: Self, location: T) -> Self { /* ... */ }
  ```
  Adds a location to the error, such as a field or variant.

- ```rust
  pub fn at_path(self: Self, path: &Path) -> Self { /* ... */ }
  ```
  Adds a location to the error, such as a field or variant.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Gets the number of individual errors in this error.

- ```rust
  pub fn add_sibling_alts_for_unknown_field<''a, T, I>(self: Self, alternates: I) -> Self
where
    T: AsRef<str> + ''a,
    I: IntoIterator<Item = &''a T> { /* ... */ }
  ```
  Consider additional field names as "did you mean" suggestions for

- ```rust
  pub fn write_errors(self: Self) -> TokenStream { /* ... */ }
  ```
  Write this error and any children as compile errors into a `TokenStream` to

###### Trait Implementations

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(e: syn::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: Error) -> Self { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = Error> { /* ... */ }
    ```

- **Sync**
- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn cause(self: &Self) -> Option<&dyn StdError> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **RefUnwindSafe**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `IntoIter`

An iterator that moves out of an `Error`.

```rust
pub struct IntoIter {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Accumulator`

**Attributes:**

- `#[must_use = "Accumulator will panic on drop if not defused."]`

Accumulator for errors, for helping call [`Error::multiple`].

See the docs for [`darling::Error`](Error) for more discussion of error handling with darling.

# Panics

`Accumulator` panics on drop unless [`finish`](Self::finish), [`finish_with`](Self::finish_with),
or [`into_inner`](Self::into_inner) has been called, **even if it contains no errors**.
If you want to discard an `Accumulator` that you know to be empty, use `accumulator.finish().unwrap()`.

# Example

```
# extern crate darling_core as darling;
# struct Thing;
# struct Output;
# impl Thing { fn validate(self) -> darling::Result<Output> { Ok(Output) } }
fn validate_things(inputs: Vec<Thing>) -> darling::Result<Vec<Output>> {
    let mut errors = darling::Error::accumulator();

    let outputs = inputs
        .into_iter()
        .filter_map(|thing| errors.handle_in(|| thing.validate()))
        .collect::<Vec<_>>();

    errors.finish()?;
    Ok(outputs)
}
```

```rust
pub struct Accumulator(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn handle_in<T, F: FnOnce() -> Result<T>>(self: &mut Self, f: F) -> Option<T> { /* ... */ }
  ```
  Runs a closure, returning the successful value as `Some`, or collecting the error

- ```rust
  pub fn handle<T>(self: &mut Self, result: Result<T>) -> Option<T> { /* ... */ }
  ```
  Handles a possible error.

- ```rust
  pub fn finish(self: Self) -> Result<()> { /* ... */ }
  ```
  Stop accumulating errors, producing `Ok` if there are no errors or producing

- ```rust
  pub fn finish_with<T>(self: Self, success: T) -> Result<T> { /* ... */ }
  ```
  Bundles the collected errors if there were any, or returns the success value

- ```rust
  pub fn into_inner(self: Self) -> Vec<Error> { /* ... */ }
  ```
  Returns the accumulated errors as a `Vec`.

- ```rust
  pub fn push(self: &mut Self, error: Error) { /* ... */ }
  ```
  Add one error to the collection.

- ```rust
  pub fn checkpoint(self: Self) -> Result<Accumulator> { /* ... */ }
  ```
  Finish the current accumulation, and if there are no errors create a new `Self` so processing may continue.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
## Module `usage`

Traits and types used for tracking the usage of generic parameters through a proc-macro input.

When generating trait impls, libraries often want to automatically figure out which type parameters
are used in which fields, and then emit bounds that will produce the most permissive compilable
code.

# Usage

## Example 1: Filtering
This example accepts a proc-macro input, then finds all lifetimes and type parameters used
by private fields.

```rust
# extern crate darling_core;
# extern crate syn;
#
# // in real-world usage, import from `darling`
# use darling_core::usage::{self, CollectLifetimes, CollectTypeParams, GenericsExt, Purpose};
# use syn::{Data, DeriveInput, GenericParam, Generics, Visibility};
#
# #[allow(dead_code)]
fn process(input: &DeriveInput) -> Generics {
    let type_params = input.generics.declared_type_params();
    let lifetimes = input.generics.declared_lifetimes();

    let mut ret_generics = input.generics.clone();

    if let Data::Struct(ref body) = input.data {
        let internal_fields = body
            .fields
            .iter()
            .filter(|field| field.vis == Visibility::Inherited)
            .collect::<Vec<_>>();

        let int_type_params = internal_fields
            .collect_type_params(&Purpose::BoundImpl.into(), &type_params);

        // We could reuse the vec from above, but here we'll instead
        // directly consume the chained iterator.
        let int_lifetimes = body
            .fields
            .iter()
            .filter(|field| field.vis == Visibility::Inherited)
            .collect_lifetimes(&Purpose::BoundImpl.into(), &lifetimes);


        ret_generics.params = ret_generics
            .params
            .into_iter()
            .filter(|gp| {
                match *gp {
                    GenericParam::Type(ref ty) => int_type_params.contains(&ty.ident),
                    GenericParam::Lifetime(ref lt) => int_lifetimes.contains(&lt.lifetime),
                    _ => true,
                }
            })
            .collect();
    }

    ret_generics
}

# fn main() {}
```

## Example 2: Integrating with `FromDeriveInput`
It is possible to use `darling`'s magic fields feature in tandem with the `usage` feature set.
While there is no custom derive for `UsesTypeParams` or `UsesLifetimes`, there are macros to
generate impls.

```rust,ignore
#![allow(dead_code)]

#[derive(FromField)]
#[darling(attributes(speak))]
struct SpeakerField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    volume: Option<u32>,
}

uses_type_params!(SpeakerField, ty);
uses_lifetimes!(SpeakerField, ty);

#[derive(FromDeriveInput)]
struct SpeakerOptions {
    generics: syn::Generics,
    data: darling::ast::Data<darling::util::Ignored, SpeakerField>,
}
```

At this point, you are able to call `uses_type_params` on `SpeakerOptions.data`, or any filtered
view of it. `darling` internally uses this in conjunction with the `skip` meta-item to determine
which type parameters don't require the `FromMeta` bound in generated impls.

**Note:** If you are performing operations referencing generic params in meta-items parsed by `darling`,
you should determine if those impact the emitted code and wire up `UsesTypeParams` accordingly for
your field/variant.

```rust
pub mod usage { /* ... */ }
```

### Re-exports

#### Re-export `GenericsExt`

```rust
pub use self::generics_ext::GenericsExt;
```

#### Re-export `IdentRefSet`

```rust
pub use self::ident_set::IdentRefSet;
```

#### Re-export `IdentSet`

```rust
pub use self::ident_set::IdentSet;
```

#### Re-export `CollectLifetimes`

```rust
pub use self::lifetimes::CollectLifetimes;
```

#### Re-export `LifetimeRefSet`

```rust
pub use self::lifetimes::LifetimeRefSet;
```

#### Re-export `LifetimeSet`

```rust
pub use self::lifetimes::LifetimeSet;
```

#### Re-export `UsesLifetimes`

```rust
pub use self::lifetimes::UsesLifetimes;
```

#### Re-export `Options`

```rust
pub use self::options::Options;
```

#### Re-export `Purpose`

```rust
pub use self::options::Purpose;
```

#### Re-export `CollectTypeParams`

```rust
pub use self::type_params::CollectTypeParams;
```

#### Re-export `UsesTypeParams`

```rust
pub use self::type_params::UsesTypeParams;
```

## Module `util`

Utility types for attribute parsing.

```rust
pub mod util { /* ... */ }
```

### Modules

## Module `parse_expr`

Functions to use with `#[darling(with = "...")]` that control how quoted values
in [`Meta`] instances are parsed into [`Expr`] fields.

Version 1 of syn did not permit expressions on the right-hand side of the `=` in a
[`MetaNameValue`](syn::MetaNameValue), so darling accepted string literals and then
parsed their contents as expressions.
Passing a string literal in this version would have required the use of a raw string
to add quotation marks inside the literal.

Version 2 of syn removes the requirement that the right-hand side be a literal.
For most types, such as [`Path`](syn::Path), the [`FromMeta`] impl can accept the
version without quotation marks without causing ambiguity; a path cannot start and
end with quotation marks, so removal is automatic.

[`Expr`] is the one type where this ambiguity is new and unavoidable. To address this,
this module provides different functions for different expected behaviors.

```rust
pub mod parse_expr { /* ... */ }
```

### Functions

#### Function `preserve_str_literal`

Parse a [`Meta`] to an [`Expr`]; if the value is a string literal, the emitted
expression will be a string literal.

```rust
pub fn preserve_str_literal(meta: &syn::Meta) -> crate::Result<syn::Expr> { /* ... */ }
```

#### Function `parse_str_literal`

Parse a [`Meta`] to an [`Expr`]; if the value is a string literal, the string's
contents will be parsed as an expression and emitted.

```rust
pub fn parse_str_literal(meta: &syn::Meta) -> crate::Result<syn::Expr> { /* ... */ }
```

### Re-exports

#### Re-export `Callable`

```rust
pub use self::callable::Callable;
```

#### Re-export `Flag`

```rust
pub use self::flag::Flag;
```

#### Re-export `IdentString`

```rust
pub use self::ident_string::IdentString;
```

#### Re-export `Ignored`

```rust
pub use self::ignored::Ignored;
```

#### Re-export `Override`

```rust
pub use self::over_ride::Override;
```

#### Re-export `parse_attribute_to_meta_list`

```rust
pub use self::parse_attribute::parse_attribute_to_meta_list;
```

#### Re-export `PathList`

```rust
pub use self::path_list::PathList;
```

#### Re-export `path_to_string`

```rust
pub use self::path_to_string::path_to_string;
```

#### Re-export `AsShape`

```rust
pub use self::shape::AsShape;
```

#### Re-export `Shape`

```rust
pub use self::shape::Shape;
```

#### Re-export `ShapeSet`

```rust
pub use self::shape::ShapeSet;
```

#### Re-export `SpannedValue`

```rust
pub use self::spanned_value::SpannedValue;
```

#### Re-export `WithOriginal`

```rust
pub use self::with_original::WithOriginal;
```

## Macros

### Macro `uses_type_params`

**Attributes:**

- `#[macro_export]`

Generator for `UsesTypeParam` impls that unions the used type parameters of the selected fields.

# Usage
The macro takes the type implementing the trait as the first argument, then a comma-separated list of
fields for the rest of its arguments.

The type of each passed-in field must implement `UsesTypeParams`, or the resulting code won't compile.

```rust
# extern crate syn;
# use darling_core::uses_type_params;
#
struct MyField {
    ty: syn::Type,
}

uses_type_params!(MyField, ty);

fn main() {
    // no test run
}
```

`darling` cannot derive this trait automatically, as it doesn't know which information extracted from
proc-macro input is meant to constitute "using" the type parameter, but crate consumers should
implement it by hand or using the macro.

```rust
pub macro_rules! uses_type_params {
    /* macro_rules! uses_type_params {
    ($impl_type:ty, $accessor:ident) => { ... };
    ($impl_type:ty, $first:ident, $($field:ident),+) => { ... };
} */
}
```

### Macro `uses_lifetimes`

**Attributes:**

- `#[macro_export]`

Generator for `UsesLifetimes` impls that unions the used lifetimes of the selected fields.

# Usage
The macro takes the type implementing the trait as the first argument, then a comma-separated list of
fields for the rest of its arguments.

The type of each passed-in field must implement `UsesLifetimes`, or the resulting code won't compile.

```rust
pub macro_rules! uses_lifetimes {
    /* macro_rules! uses_lifetimes {
    ($impl_type:ty, $accessor:ident) => { ... };
    ($impl_type:ty, $first:ident, $($field:ident),+) => { ... };
} */
}
```

## Re-exports

### Re-export `Error`

```rust
pub use self::error::Error;
```

### Re-export `Result`

```rust
pub use self::error::Result;
```

### Re-export `FromAttributes`

```rust
pub use self::from_attributes::FromAttributes;
```

### Re-export `FromDeriveInput`

```rust
pub use self::from_derive_input::FromDeriveInput;
```

### Re-export `FromField`

```rust
pub use self::from_field::FromField;
```

### Re-export `FromGenericParam`

```rust
pub use self::from_generic_param::FromGenericParam;
```

### Re-export `FromGenerics`

```rust
pub use self::from_generics::FromGenerics;
```

### Re-export `FromMeta`

```rust
pub use self::from_meta::FromMeta;
```

### Re-export `FromTypeParam`

```rust
pub use self::from_type_param::FromTypeParam;
```

### Re-export `FromVariant`

```rust
pub use self::from_variant::FromVariant;
```

