# Crate Documentation

**Version:** 0.13.1

**Format Version:** 43

# Module `synstructure`

This crate provides helper types for matching against enum variants, and
extracting bindings to each of the fields in the deriving Struct or Enum in
a generic way.

If you are writing a `#[derive]` which needs to perform some operation on
every field, then you have come to the right place!

# Example: `WalkFields`
### Trait Implementation
```
pub trait WalkFields: std::any::Any {
    fn walk_fields(&self, walk: &mut FnMut(&WalkFields));
}
impl WalkFields for i32 {
    fn walk_fields(&self, _walk: &mut FnMut(&WalkFields)) {}
}
```

### Custom Derive
```
# use quote::quote;
fn walkfields_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.each(|bi| quote!{
        walk(#bi)
    });

    s.gen_impl(quote! {
        extern crate synstructure_test_traits;

        gen impl synstructure_test_traits::WalkFields for @Self {
            fn walk_fields(&self, walk: &mut FnMut(&synstructure_test_traits::WalkFields)) {
                match *self { #body }
            }
        }
    })
}
# const _IGNORE: &'static str = stringify!(
synstructure::decl_derive!([WalkFields] => walkfields_derive);
# );

/*
 * Test Case
 */
fn main() {
    synstructure::test_derive! {
        walkfields_derive {
            enum A<T> {
                B(i32, T),
                C(i32),
            }
        }
        expands to {
            const _: () = {
                extern crate synstructure_test_traits;
                impl<T> synstructure_test_traits::WalkFields for A<T>
                    where T: synstructure_test_traits::WalkFields
                {
                    fn walk_fields(&self, walk: &mut FnMut(&synstructure_test_traits::WalkFields)) {
                        match *self {
                            A::B(ref __binding_0, ref __binding_1,) => {
                                { walk(__binding_0) }
                                { walk(__binding_1) }
                            }
                            A::C(ref __binding_0,) => {
                                { walk(__binding_0) }
                            }
                        }
                    }
                }
            };
        }
    }
}
```

# Example: `Interest`
### Trait Implementation
```
pub trait Interest {
    fn interesting(&self) -> bool;
}
impl Interest for i32 {
    fn interesting(&self) -> bool { *self > 0 }
}
```

### Custom Derive
```
# use quote::quote;
fn interest_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.fold(false, |acc, bi| quote!{
        #acc || synstructure_test_traits::Interest::interesting(#bi)
    });

    s.gen_impl(quote! {
        extern crate synstructure_test_traits;
        gen impl synstructure_test_traits::Interest for @Self {
            fn interesting(&self) -> bool {
                match *self {
                    #body
                }
            }
        }
    })
}
# const _IGNORE: &'static str = stringify!(
synstructure::decl_derive!([Interest] => interest_derive);
# );

/*
 * Test Case
 */
fn main() {
    synstructure::test_derive!{
        interest_derive {
            enum A<T> {
                B(i32, T),
                C(i32),
            }
        }
        expands to {
            const _: () = {
                extern crate synstructure_test_traits;
                impl<T> synstructure_test_traits::Interest for A<T>
                    where T: synstructure_test_traits::Interest
                {
                    fn interesting(&self) -> bool {
                        match *self {
                            A::B(ref __binding_0, ref __binding_1,) => {
                                false ||
                                    synstructure_test_traits::Interest::interesting(__binding_0) ||
                                    synstructure_test_traits::Interest::interesting(__binding_1)
                            }
                            A::C(ref __binding_0,) => {
                                false ||
                                    synstructure_test_traits::Interest::interesting(__binding_0)
                            }
                        }
                    }
                }
            };
        }
    }
}
```

For more example usage, consider investigating the `abomonation_derive` crate,
which makes use of this crate, and is fairly simple.

## Types

### Enum `AddBounds`

**Attributes:**

- `#[allow(clippy::manual_non_exhaustive)]`

Changes how bounds are added

```rust
pub enum AddBounds {
    Both,
    Fields,
    Generics,
    None,
    // Some variants omitted
}
```

#### Variants

##### `Both`

Add for fields and generics

##### `Fields`

Fields only

##### `Generics`

Generics only

##### `None`

None

*Note: Some variants have been omitted because they are private or hidden.*

#### Implementations

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Send**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AddBounds { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AddBounds) -> bool { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Enum `BindStyle`

The type of binding to use when generating a pattern.

```rust
pub enum BindStyle {
    Move,
    MoveMut,
    Ref,
    RefMut,
}
```

#### Variants

##### `Move`

`x`

##### `MoveMut`

`mut x`

##### `Ref`

`ref x`

##### `RefMut`

`ref mut x`

#### Implementations

##### Trait Implementations

- **Copy**
- **Sync**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> BindStyle { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToTokens**
  - ```rust
    fn to_tokens(self: &Self, tokens: &mut TokenStream) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Freeze**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BindStyle) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **Spanned**
  - ```rust
    fn span(self: &Self) -> Span { /* ... */ }
    ```

### Struct `BindingInfo`

Information about a specific binding. This contains both an `Ident`
reference to the given field, and the syn `&'a Field` descriptor for that
field.

This type supports `quote::ToTokens`, so can be directly used within the
`quote!` macro. It expands to a reference to the matched field.

```rust
pub struct BindingInfo<''a> {
    pub binding: syn::Ident,
    pub style: BindStyle,
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `binding` | `syn::Ident` | The name which this BindingInfo will bind to. |
| `style` | `BindStyle` | The type of binding which this BindingInfo will create. |
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn ast(self: &Self) -> &''a Field { /* ... */ }
  ```
  Returns a reference to the underlying `syn` AST node which this

- ```rust
  pub fn pat(self: &Self) -> TokenStream { /* ... */ }
  ```
  Generates the pattern fragment for this field binding.

- ```rust
  pub fn referenced_ty_params(self: &Self) -> Vec<&''a Ident> { /* ... */ }
  ```
  Returns a list of the type parameters which are referenced in this

##### Trait Implementations

- **ToTokens**
  - ```rust
    fn to_tokens(self: &Self, tokens: &mut TokenStream) { /* ... */ }
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

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Eq**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BindingInfo<''a>) -> bool { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Spanned**
  - ```rust
    fn span(self: &Self) -> Span { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BindingInfo<''a> { /* ... */ }
    ```

### Struct `VariantAst`

This type is similar to `syn`'s `Variant` type, however each of the fields
are references rather than owned. When this is used as the AST for a real
variant, this struct simply borrows the fields of the `syn::Variant`,
however this type may also be used as the sole variant for a struct.

```rust
pub struct VariantAst<''a> {
    pub attrs: &''a [syn::Attribute],
    pub ident: &''a syn::Ident,
    pub fields: &''a syn::Fields,
    pub discriminant: &''a Option<(token::Eq, syn::Expr)>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `attrs` | `&''a [syn::Attribute]` |  |
| `ident` | `&''a syn::Ident` |  |
| `fields` | `&''a syn::Fields` |  |
| `discriminant` | `&''a Option<(token::Eq, syn::Expr)>` |  |

#### Implementations

##### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VariantAst<''a> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VariantAst<''a>) -> bool { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
### Struct `VariantInfo`

A wrapper around a `syn::DeriveInput`'s variant which provides utilities
for destructuring `Variant`s with `match` expressions.

```rust
pub struct VariantInfo<''a> {
    pub prefix: Option<&''a syn::Ident>,
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `prefix` | `Option<&''a syn::Ident>` |  |
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn bindings(self: &Self) -> &[BindingInfo<''a>] { /* ... */ }
  ```
  Returns a slice of the bindings in this Variant.

- ```rust
  pub fn bindings_mut(self: &mut Self) -> &mut [BindingInfo<''a>] { /* ... */ }
  ```
  Returns a mut slice of the bindings in this Variant.

- ```rust
  pub fn ast(self: &Self) -> VariantAst<''a> { /* ... */ }
  ```
  Returns a `VariantAst` object which contains references to the

- ```rust
  pub fn omitted_bindings(self: &Self) -> bool { /* ... */ }
  ```
  True if any bindings were omitted due to a `filter` call.

- ```rust
  pub fn pat(self: &Self) -> TokenStream { /* ... */ }
  ```
  Generates the match-arm pattern which could be used to match against this Variant.

- ```rust
  pub fn construct<F, T>(self: &Self, func: F) -> TokenStream
where
    F: FnMut(&Field, usize) -> T,
    T: ToTokens { /* ... */ }
  ```
  Generates the token stream required to construct the current variant.

- ```rust
  pub fn each<F, R>(self: &Self, f: F) -> TokenStream
where
    F: FnMut(&BindingInfo<''_>) -> R,
    R: ToTokens { /* ... */ }
  ```
  Runs the passed-in function once for each bound field, passing in a `BindingInfo`.

- ```rust
  pub fn fold<F, I, R>(self: &Self, init: I, f: F) -> TokenStream
where
    F: FnMut(TokenStream, &BindingInfo<''_>) -> R,
    I: ToTokens,
    R: ToTokens { /* ... */ }
  ```
  Runs the passed-in function once for each bound field, passing in the

- ```rust
  pub fn filter<F>(self: &mut Self, f: F) -> &mut Self
where
    F: FnMut(&BindingInfo<''_>) -> bool { /* ... */ }
  ```
  Filter the bindings created by this `Variant` object. This has 2 effects:

- ```rust
  pub fn drain_filter<F>(self: &mut Self, f: F) -> Self
where
    F: FnMut(&BindingInfo<''_>) -> bool { /* ... */ }
  ```
  Iterates all the bindings of this `Variant` object and uses a closure to determine if a

- ```rust
  pub fn remove_binding(self: &mut Self, idx: usize) -> &mut Self { /* ... */ }
  ```
  Remove the binding at the given index.

- ```rust
  pub fn bind_with<F>(self: &mut Self, f: F) -> &mut Self
where
    F: FnMut(&BindingInfo<''_>) -> BindStyle { /* ... */ }
  ```
  Updates the `BindStyle` for each of the passed-in fields by calling the

- ```rust
  pub fn binding_name<F>(self: &mut Self, f: F) -> &mut Self
where
    F: FnMut(&Field, usize) -> Ident { /* ... */ }
  ```
  Updates the binding name for each fo the passed-in fields by calling the

- ```rust
  pub fn referenced_ty_params(self: &Self) -> Vec<&''a Ident> { /* ... */ }
  ```
  Returns a list of the type parameters which are referenced in this

##### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> VariantInfo<''a> { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Eq**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VariantInfo<''a>) -> bool { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `Structure`

A wrapper around a `syn::DeriveInput` which provides utilities for creating
custom derive trait implementations.

```rust
pub struct Structure<''a> {
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
  pub fn new(ast: &''a DeriveInput) -> Self { /* ... */ }
  ```
  Create a new `Structure` with the variants and fields from the passed-in

- ```rust
  pub fn try_new(ast: &''a DeriveInput) -> Result<Self> { /* ... */ }
  ```
  Create a new `Structure` with the variants and fields from the passed-in

- ```rust
  pub fn variants(self: &Self) -> &[VariantInfo<''a>] { /* ... */ }
  ```
  Returns a slice of the variants in this Structure.

- ```rust
  pub fn variants_mut(self: &mut Self) -> &mut [VariantInfo<''a>] { /* ... */ }
  ```
  Returns a mut slice of the variants in this Structure.

- ```rust
  pub fn ast(self: &Self) -> &''a DeriveInput { /* ... */ }
  ```
  Returns a reference to the underlying `syn` AST node which this

- ```rust
  pub fn omitted_variants(self: &Self) -> bool { /* ... */ }
  ```
  True if any variants were omitted due to a `filter_variants` call.

- ```rust
  pub fn each<F, R>(self: &Self, f: F) -> TokenStream
where
    F: FnMut(&BindingInfo<''_>) -> R,
    R: ToTokens { /* ... */ }
  ```
  Runs the passed-in function once for each bound field, passing in a `BindingInfo`.

- ```rust
  pub fn fold<F, I, R>(self: &Self, init: I, f: F) -> TokenStream
where
    F: FnMut(TokenStream, &BindingInfo<''_>) -> R,
    I: ToTokens,
    R: ToTokens { /* ... */ }
  ```
  Runs the passed-in function once for each bound field, passing in the

- ```rust
  pub fn each_variant<F, R>(self: &Self, f: F) -> TokenStream
where
    F: FnMut(&VariantInfo<''_>) -> R,
    R: ToTokens { /* ... */ }
  ```
  Runs the passed-in function once for each variant, passing in a

- ```rust
  pub fn filter<F>(self: &mut Self, f: F) -> &mut Self
where
    F: FnMut(&BindingInfo<''_>) -> bool { /* ... */ }
  ```
  Filter the bindings created by this `Structure` object. This has 2 effects:

- ```rust
  pub fn drain_filter<F>(self: &mut Self, f: F) -> Self
where
    F: FnMut(&BindingInfo<''_>) -> bool { /* ... */ }
  ```
  Iterates all the bindings of this `Structure` object and uses a closure to determine if a

- ```rust
  pub fn add_where_predicate(self: &mut Self, pred: WherePredicate) -> &mut Self { /* ... */ }
  ```
  Specify additional where predicate bounds which should be generated by

- ```rust
  pub fn add_bounds(self: &mut Self, mode: AddBounds) -> &mut Self { /* ... */ }
  ```
  Specify which bounds should be generated by impl-generating functions

- ```rust
  pub fn filter_variants<F>(self: &mut Self, f: F) -> &mut Self
where
    F: FnMut(&VariantInfo<''_>) -> bool { /* ... */ }
  ```
  Filter the variants matched by this `Structure` object. This has 2 effects:

- ```rust
  pub fn drain_filter_variants<F>(self: &mut Self, f: F) -> Self
where
    F: FnMut(&VariantInfo<''_>) -> bool { /* ... */ }
  ```
  Iterates all the variants of this `Structure` object and uses a closure to determine if a

- ```rust
  pub fn remove_variant(self: &mut Self, idx: usize) -> &mut Self { /* ... */ }
  ```
  Remove the variant at the given index.

- ```rust
  pub fn bind_with<F>(self: &mut Self, f: F) -> &mut Self
where
    F: FnMut(&BindingInfo<''_>) -> BindStyle { /* ... */ }
  ```
  Updates the `BindStyle` for each of the passed-in fields by calling the

- ```rust
  pub fn binding_name<F>(self: &mut Self, f: F) -> &mut Self
where
    F: FnMut(&Field, usize) -> Ident { /* ... */ }
  ```
  Updates the binding name for each fo the passed-in fields by calling the

- ```rust
  pub fn referenced_ty_params(self: &Self) -> Vec<&''a Ident> { /* ... */ }
  ```
  Returns a list of the type parameters which are refrenced in the types

- ```rust
  pub fn add_impl_generic(self: &mut Self, param: GenericParam) -> &mut Self { /* ... */ }
  ```
  Adds an `impl<>` generic parameter.

- ```rust
  pub fn add_trait_bounds(self: &Self, bound: &TraitBound, where_clause: &mut Option<WhereClause>, mode: AddBounds) { /* ... */ }
  ```
  Add trait bounds for a trait with the given path for each type parmaeter

- ```rust
  pub fn underscore_const(self: &mut Self, _enabled: bool) -> &mut Self { /* ... */ }
  ```
  This method is a no-op, underscore consts are used by default now.

- ```rust
  pub fn bound_impl<P: ToTokens, B: ToTokens>(self: &Self, path: P, body: B) -> TokenStream { /* ... */ }
  ```
  > NOTE: This methods' features are superceded by `Structure::gen_impl`.

- ```rust
  pub fn unsafe_bound_impl<P: ToTokens, B: ToTokens>(self: &Self, path: P, body: B) -> TokenStream { /* ... */ }
  ```
  > NOTE: This methods' features are superceded by `Structure::gen_impl`.

- ```rust
  pub fn unbound_impl<P: ToTokens, B: ToTokens>(self: &Self, path: P, body: B) -> TokenStream { /* ... */ }
  ```
  > NOTE: This methods' features are superceded by `Structure::gen_impl`.

- ```rust
  pub fn unsafe_unbound_impl<P: ToTokens, B: ToTokens>(self: &Self, path: P, body: B) -> TokenStream { /* ... */ }
  ```
  > NOTE: This methods' features are superceded by `Structure::gen_impl`.

- ```rust
  pub fn gen_impl(self: &Self, cfg: TokenStream) -> TokenStream { /* ... */ }
  ```
  Generate an impl block for the given struct. This impl block will

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Structure<''a> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Structure<''a>) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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
## Traits

### Trait `MacroResult`

Helper trait describing values which may be returned by macro implementation
methods used by this crate's macros.

```rust
pub trait MacroResult {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `into_result`: Convert this result into a `Result` for further processing / validation.

#### Provided Methods

- ```rust
  fn into_stream(self: Self) -> proc_macro::TokenStream
where
    Self: Sized { /* ... */ }
  ```
  Convert this result into a `proc_macro::TokenStream`, ready to return

#### Implementations

This trait is implemented for the following types:

- `proc_macro::TokenStream`
- `proc_macro2::TokenStream`
- `syn::Result<T>` with <T: MacroResult>

## Functions

### Function `unpretty_print`

Dumps an unpretty version of a tokenstream. Takes any type which implements
`Display`.

This is mostly useful for visualizing the output of a procedural macro, as
it makes it marginally more readable. It is used in the implementation of
`test_derive!` to unprettily print the output.

# Stability

The stability of the output of this function is not guaranteed. Do not
assert that the output of this function does not change between minor
versions.

# Example

```
# use quote::quote;
assert_eq!(
    synstructure::unpretty_print(quote! {
        const _: () = {
            extern crate krate;
            impl<T, U> krate::Trait for A<T, U>
            where
                Option<U>: krate::Trait,
                U: krate::Trait
            {
                fn a() {}
            }
        };
    }),
    "const _ : (
    )
= {
    extern crate krate ;
    impl < T , U > krate :: Trait for A < T , U > where Option < U > : krate :: Trait , U : krate :: Trait {
        fn a (
            )
        {
            }
        }
    }
;
"
)
```

```rust
pub fn unpretty_print<T: std::fmt::Display>(ts: T) -> String { /* ... */ }
```

## Macros

### Macro `decl_derive`

**Attributes:**

- `#[<cfg>(all(not(all(target_arch = "wasm32",
any(target_os = "unknown", target_os = "wasi"))), feature = "proc-macro"))]`
- `#[macro_export]`

The `decl_derive!` macro declares a custom derive wrapper. It will parse the
incoming `TokenStream` into a `synstructure::Structure` object, and pass it
into the inner function.

Your inner function should take a `synstructure::Structure` by value, and
return a type implementing `synstructure::MacroResult`, for example:

```
fn derive_simple(input: synstructure::Structure) -> proc_macro2::TokenStream {
    unimplemented!()
}

fn derive_result(input: synstructure::Structure)
    -> syn::Result<proc_macro2::TokenStream>
{
    unimplemented!()
}
```

# Usage

### Without Attributes
```
fn derive_interesting(_input: synstructure::Structure) -> proc_macro2::TokenStream {
    quote::quote! { ... }
}

# const _IGNORE: &'static str = stringify! {
decl_derive!([Interesting] => derive_interesting);
# };
```

### With Attributes
```
# fn main() {}
fn derive_interesting(_input: synstructure::Structure) -> proc_macro2::TokenStream {
    quote::quote! { ... }
}

# const _IGNORE: &'static str = stringify! {
decl_derive!([Interesting, attributes(interesting_ignore)] => derive_interesting);
# };
```

### Decl Attributes & Doc Comments
```
# fn main() {}
fn derive_interesting(_input: synstructure::Structure) -> proc_macro2::TokenStream {
    quote::quote! { ... }
}

# const _IGNORE: &'static str = stringify! {
decl_derive! {
    [Interesting] =>
    #[allow(some_lint)]
    /// Documentation Comments
    derive_interesting
}
# };
```

*This macro is available if `synstructure` is built with the `"proc-macro"`
feature.*

```rust
pub macro_rules! decl_derive {
    /* macro_rules! decl_derive {
    ([$derives:ident $($derive_t:tt)*] => $(#[$($attrs:tt)*])* $inner:path) => { ... };
} */
}
```

### Macro `decl_attribute`

**Attributes:**

- `#[<cfg>(all(not(all(target_arch = "wasm32",
any(target_os = "unknown", target_os = "wasi"))), feature = "proc-macro"))]`
- `#[macro_export]`

The `decl_attribute!` macro declares a custom attribute wrapper. It will
parse the incoming `TokenStream` into a `synstructure::Structure` object,
and pass it into the inner function.

Your inner function should have the following type:

```
fn attribute(
    attr: proc_macro2::TokenStream,
    structure: synstructure::Structure,
) -> proc_macro2::TokenStream {
    unimplemented!()
}
```

# Usage

```
fn attribute_interesting(
    _attr: proc_macro2::TokenStream,
    _structure: synstructure::Structure,
) -> proc_macro2::TokenStream {
    quote::quote! { ... }
}

# const _IGNORE: &'static str = stringify! {
decl_attribute!([interesting] => attribute_interesting);
# };
```

*This macro is available if `synstructure` is built with the `"proc-macro"`
feature.*

```rust
pub macro_rules! decl_attribute {
    /* macro_rules! decl_attribute {
    ([$attribute:ident] => $(#[$($attrs:tt)*])* $inner:path) => { ... };
} */
}
```

### Macro `test_derive`

**Attributes:**

- `#[macro_export]`

Run a test on a custom derive. This macro expands both the original struct
and the expansion to ensure that they compile correctly, and confirms that
feeding the original struct into the named derive will produce the written
output.

You can add `no_build` to the end of the macro invocation to disable
checking that the written code compiles. This is useful in contexts where
the procedural macro cannot depend on the crate where it is used during
tests.

# Usage

```
fn test_derive_example(_s: synstructure::Structure)
    -> Result<proc_macro2::TokenStream, syn::Error>
{
    Ok(quote::quote! { const YOUR_OUTPUT: &'static str = "here"; })
}

fn main() {
    synstructure::test_derive!{
        test_derive_example {
            struct A;
        }
        expands to {
            const YOUR_OUTPUT: &'static str = "here";
        }
    }
}
```

```rust
pub macro_rules! test_derive {
    /* macro_rules! test_derive {
    ($name:path { $($i:tt)* } expands to { $($o:tt)* }) => { ... };
    ($name:path { $($i:tt)* } expands to { $($o:tt)* } no_build) => { ... };
} */
}
```

