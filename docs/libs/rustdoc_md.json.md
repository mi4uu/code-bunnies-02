# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `rustdoc_md`

## Modules

## Module `rustdoc_json_types`

Rustdoc's JSON output interface

These types are the public API exposed through the `--output-format json` flag. The [`Crate`]
struct is the root of the JSON blob and all other items are contained within.

We expose a `rustc-hash` feature that is disabled by default. This feature switches the
[`std::collections::HashMap`] for [`rustc_hash::FxHashMap`] to improve the performance of said
`HashMap` in specific situations.

`cargo-semver-checks` for example, saw a [-3% improvement][1] when benchmarking using the
`aws_sdk_ec2` JSON output (~500MB of JSON). As always, we recommend measuring the impact before
turning this feature on, as [`FxHashMap`][2] only concerns itself with hash speed, and may
increase the number of collisions.

[1]: https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/rustc-hash.20and.20performance.20of.20rustdoc-types/near/474855731
[2]: https://crates.io/crates/rustc-hash

```rust
pub mod rustdoc_json_types { /* ... */ }
```

### Types

#### Type Alias `FxHashMap`

```rust
pub type FxHashMap<K, V> = std::collections::HashMap<K, V>;
```

#### Struct `Crate`

The root of the emitted JSON blob.

It contains all type/documentation information
about the language items in the local crate, as well as info about external items to allow
tools to find or link to them.

```rust
pub struct Crate {
    pub root: Id,
    pub crate_version: Option<String>,
    pub includes_private: bool,
    pub index: std::collections::HashMap<Id, Item>,
    pub paths: std::collections::HashMap<Id, ItemSummary>,
    pub external_crates: std::collections::HashMap<u32, ExternalCrate>,
    pub format_version: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `root` | `Id` | The id of the root [`Module`] item of the local crate. |
| `crate_version` | `Option<String>` | The version string given to `--crate-version`, if any. |
| `includes_private` | `bool` | Whether or not the output includes private items. |
| `index` | `std::collections::HashMap<Id, Item>` | A collection of all items in the local crate as well as some external traits and their<br>items that are referenced locally. |
| `paths` | `std::collections::HashMap<Id, ItemSummary>` | Maps IDs to fully qualified paths and other info helpful for generating links. |
| `external_crates` | `std::collections::HashMap<u32, ExternalCrate>` | Maps `crate_id` of items to a crate name and html_root_url if it exists. |
| `format_version` | `u32` | A single version number to be used in the future when making backwards incompatible changes<br>to the JSON output. |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **Eq**
- **RefUnwindSafe**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
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

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Crate) -> bool { /* ... */ }
    ```

- **Sync**
- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Crate { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
#### Struct `ExternalCrate`

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

```rust
pub struct ExternalCrate {
    pub name: String,
    pub html_root_url: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the crate.<br><br>Note: This is the [*crate* name][crate-name], which may not be the same as the<br>[*package* name][package-name]. For example, for <https://crates.io/crates/regex-syntax>,<br>this field will be `regex_syntax` (which uses an `_`, not a `-`).<br><br>[crate-name]: https://doc.rust-lang.org/stable/cargo/reference/cargo-targets.html#the-name-field<br>[package-name]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-name-field |
| `html_root_url` | `Option<String>` | The root URL at which the crate's documentation lives. |

##### Implementations

###### Trait Implementations

- **Unpin**
- **DeserializeOwned**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExternalCrate { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExternalCrate) -> bool { /* ... */ }
    ```

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

- **Sync**
- **UnwindSafe**
- **Eq**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `ItemSummary`

Information about an external (not defined in the local crate) [`Item`].

For external items, you don't get the same level of
information. This struct should contain enough to generate a link/reference to the item in
question, or can be used by a tool that takes the json output of multiple crates to find
the actual item definition with all the relevant info.

```rust
pub struct ItemSummary {
    pub crate_id: u32,
    pub path: Vec<String>,
    pub kind: ItemKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `crate_id` | `u32` | Can be used to look up the name and html_root_url of the crate this item came from in the<br>`external_crates` map. |
| `path` | `Vec<String>` | The list of path components for the fully qualified path of this item (e.g.<br>`["std", "io", "lazy", "Lazy"]` for `std::io::lazy::Lazy`).<br><br>Note that items can appear in multiple paths, and the one chosen is implementation<br>defined. Currently, this is the full path to where the item was defined. Eg<br>[`String`] is currently `["alloc", "string", "String"]` and [`HashMap`][`std::collections::HashMap`]<br>is `["std", "collections", "hash", "map", "HashMap"]`, but this is subject to change. |
| `kind` | `ItemKind` | Whether this item is a struct, trait, macro, etc. |

##### Implementations

###### Trait Implementations

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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ItemSummary { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ItemSummary) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeOwned**
- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Send**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Eq**
#### Struct `Item`

Anything that can hold documentation - modules, structs, enums, functions, traits, etc.

The `Item` data type holds fields that can apply to any of these,
and leaves kind-specific details (like function args or enum variants) to the `inner` field.

```rust
pub struct Item {
    pub id: Id,
    pub crate_id: u32,
    pub name: Option<String>,
    pub span: Option<Span>,
    pub visibility: Visibility,
    pub docs: Option<String>,
    pub links: std::collections::HashMap<String, Id>,
    pub attrs: Vec<String>,
    pub deprecation: Option<Deprecation>,
    pub inner: ItemEnum,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `Id` | The unique identifier of this item. Can be used to find this item in various mappings. |
| `crate_id` | `u32` | This can be used as a key to the `external_crates` map of [`Crate`] to see which crate<br>this item came from. |
| `name` | `Option<String>` | Some items such as impls don't have names. |
| `span` | `Option<Span>` | The source location of this item (absent if it came from a macro expansion or inline<br>assembly). |
| `visibility` | `Visibility` | By default all documented items are public, but you can tell rustdoc to output private items<br>so this field is needed to differentiate. |
| `docs` | `Option<String>` | The full markdown docstring of this item. Absent if there is no documentation at all,<br>Some("") if there is some documentation but it is empty (EG `#[doc = ""]`). |
| `links` | `std::collections::HashMap<String, Id>` | This mapping resolves [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md) from the docstring to their IDs |
| `attrs` | `Vec<String>` | Stringified versions of parsed attributes on this item.<br>Essentially debug printed (e.g. `#[inline]` becomes something similar to `#[attr="Inline(Hint)"]`).<br>Equivalent to the hir pretty-printing of attributes. |
| `deprecation` | `Option<Deprecation>` | Information about the item’s deprecation, if present. |
| `inner` | `ItemEnum` | The type-specific fields describing this item. |

##### Implementations

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Item { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DeserializeOwned**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Item) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `Span`

A range of source code.

```rust
pub struct Span {
    pub filename: std::path::PathBuf,
    pub begin: (usize, usize),
    pub end: (usize, usize),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `filename` | `std::path::PathBuf` | The path to the source file for this span relative to the path `rustdoc` was invoked with. |
| `begin` | `(usize, usize)` | Zero indexed Line and Column of the first character of the `Span` |
| `end` | `(usize, usize)` | Zero indexed Line and Column of the last character of the `Span` |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Span) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **Eq**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Span { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **DeserializeOwned**
- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `Deprecation`

Information about the deprecation of an [`Item`].

```rust
pub struct Deprecation {
    pub since: Option<String>,
    pub note: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `since` | `Option<String>` | Usually a version number when this [`Item`] first became deprecated. |
| `note` | `Option<String>` | The reason for deprecation and/or what alternatives to use. |

##### Implementations

###### Trait Implementations

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Deprecation { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Deprecation) -> bool { /* ... */ }
    ```

- **DeserializeOwned**
- **Eq**
#### Enum `Visibility`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

Visibility of an [`Item`].

```rust
pub enum Visibility {
    Public,
    Default,
    Crate,
    Restricted {
        parent: Id,
        path: String,
    },
}
```

##### Variants

###### `Public`

Explicitly public visibility set with `pub`.

###### `Default`

For the most part items are private by default. The exceptions are associated items of
public traits and variants of public enums.

###### `Crate`

Explicitly crate-wide visibility set with `pub(crate)`

###### `Restricted`

For `pub(in path)` visibility.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `parent` | `Id` | ID of the module to which this visibility restricts items. |
| `path` | `String` | The path with which [`parent`] was referenced<br>(like `super::super` or `crate::foo::bar`).<br><br>[`parent`]: Visibility::Restricted::parent |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Visibility) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DeserializeOwned**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
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
    fn clone(self: &Self) -> Visibility { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `DynTrait`

Dynamic trait object type (`dyn Trait`).

```rust
pub struct DynTrait {
    pub traits: Vec<PolyTrait>,
    pub lifetime: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `traits` | `Vec<PolyTrait>` | All the traits implemented. One of them is the vtable, and the rest must be auto traits. |
| `lifetime` | `Option<String>` | The lifetime of the whole dyn object<br>```text<br>dyn Debug + 'static<br>            ^^^^^^^<br>            |<br>            this part<br>``` |

##### Implementations

###### Trait Implementations

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeOwned**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DynTrait) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DynTrait { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
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

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
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

#### Struct `PolyTrait`

A trait and potential HRTBs

```rust
pub struct PolyTrait {
    pub trait_: Path,
    pub generic_params: Vec<GenericParamDef>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `trait_` | `Path` | The path to the trait. |
| `generic_params` | `Vec<GenericParamDef>` | Used for Higher-Rank Trait Bounds (HRTBs)<br>```text<br>dyn for<'a> Fn() -> &'a i32"<br>    ^^^^^^^<br>``` |

##### Implementations

###### Trait Implementations

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PolyTrait { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PolyTrait) -> bool { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **DeserializeOwned**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Send**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
#### Enum `GenericArgs`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

A set of generic arguments provided to a path segment, e.g.

```text
std::option::Option::<u32>::None
                     ^^^^^
```

```rust
pub enum GenericArgs {
    AngleBracketed {
        args: Vec<GenericArg>,
        constraints: Vec<AssocItemConstraint>,
    },
    Parenthesized {
        inputs: Vec<Type>,
        output: Option<Type>,
    },
    ReturnTypeNotation,
}
```

##### Variants

###### `AngleBracketed`

`<'a, 32, B: Copy, C = u32>`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `args` | `Vec<GenericArg>` | The list of each argument on this type.<br>```text<br><'a, 32, B: Copy, C = u32><br> ^^^^^^<br>``` |
| `constraints` | `Vec<AssocItemConstraint>` | Associated type or constant bindings (e.g. `Item=i32` or `Item: Clone`) for this type. |

###### `Parenthesized`

`Fn(A, B) -> C`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `inputs` | `Vec<Type>` | The input types, enclosed in parentheses. |
| `output` | `Option<Type>` | The output type provided after the `->`, if present. |

###### `ReturnTypeNotation`

`T::method(..)`

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GenericArgs) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **Send**
- **UnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeOwned**
- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenericArgs { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `GenericArg`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

One argument in a list of generic arguments to a path segment.

Part of [`GenericArgs`].

```rust
pub enum GenericArg {
    Lifetime(String),
    Type(Type),
    Const(Constant),
    Infer,
}
```

##### Variants

###### `Lifetime`

A lifetime argument.
```text
std::borrow::Cow<'static, str>
                 ^^^^^^^
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Type`

A type argument.
```text
std::borrow::Cow<'static, str>
                          ^^^
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Type` |  |

###### `Const`

A constant as a generic argument.
```text
core::array::IntoIter<u32, { 640 * 1024 }>
                           ^^^^^^^^^^^^^^
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Constant` |  |

###### `Infer`

A generic argument that's explicitly set to be inferred.
```text
std::vec::Vec::<_>::new()
                ^
```

##### Implementations

###### Trait Implementations

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

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeOwned**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GenericArg) -> bool { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Send**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenericArg { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **UnwindSafe**
- **Eq**
#### Struct `Constant`

A constant.

```rust
pub struct Constant {
    pub expr: String,
    pub value: Option<String>,
    pub is_literal: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `String` | The stringified expression of this constant. Note that its mapping to the original<br>source code is unstable and it's not guaranteed that it'll match the source code. |
| `value` | `Option<String>` | The value of the evaluated expression for this constant, which is only computed for numeric<br>types. |
| `is_literal` | `bool` | Whether this constant is a bool, numeric, string, or char literal. |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Constant) -> bool { /* ... */ }
    ```

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

- **Sync**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DeserializeOwned**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Constant { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
#### Struct `AssocItemConstraint`

Describes a bound applied to an associated type/constant.

Example:
```text
IntoIterator<Item = u32, IntoIter: Clone>
             ^^^^^^^^^^  ^^^^^^^^^^^^^^^
```

```rust
pub struct AssocItemConstraint {
    pub name: String,
    pub args: GenericArgs,
    pub binding: AssocItemConstraintKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the associated type/constant. |
| `args` | `GenericArgs` | Arguments provided to the associated type/constant. |
| `binding` | `AssocItemConstraintKind` | The kind of bound applied to the associated type/constant. |

##### Implementations

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AssocItemConstraint { /* ... */ }
    ```

- **Sync**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AssocItemConstraint) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeOwned**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Enum `AssocItemConstraintKind`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

The way in which an associate type/constant is bound.

```rust
pub enum AssocItemConstraintKind {
    Equality(Term),
    Constraint(Vec<GenericBound>),
}
```

##### Variants

###### `Equality`

The required value/type is specified exactly. e.g.
```text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
         ^^^^^^^^^^
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Term` |  |

###### `Constraint`

The type is required to satisfy a set of bounds.
```text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<GenericBound>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AssocItemConstraintKind) -> bool { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **DeserializeOwned**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AssocItemConstraintKind { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
#### Struct `Id`

An opaque identifier for an item.

It can be used to lookup in [`Crate::index`] or [`Crate::paths`] to resolve it
to an [`Item`].

Id's are only valid within a single JSON blob. They cannot be used to
resolve references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications
should treat them as opaque keys to lookup items, and avoid attempting
to parse them, or otherwise depend on any implementation details.

```rust
pub struct Id(pub u32);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **StructuralPartialEq**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Id { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Id) -> bool { /* ... */ }
    ```

- **Send**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Unpin**
- **DeserializeOwned**
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

#### Enum `ItemKind`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

The fundamental kind of an item. Unlike [`ItemEnum`], this does not carry any additional info.

Part of [`ItemSummary`].

```rust
pub enum ItemKind {
    Module,
    ExternCrate,
    Use,
    Struct,
    StructField,
    Union,
    Enum,
    Variant,
    Function,
    TypeAlias,
    Constant,
    Trait,
    TraitAlias,
    Impl,
    Static,
    ExternType,
    Macro,
    ProcAttribute,
    ProcDerive,
    AssocConst,
    AssocType,
    Primitive,
    Keyword,
}
```

##### Variants

###### `Module`

A module declaration, e.g. `mod foo;` or `mod foo {}`

###### `ExternCrate`

A crate imported via the `extern crate` syntax.

###### `Use`

An import of 1 or more items into scope, using the `use` keyword.

###### `Struct`

A `struct` declaration.

###### `StructField`

A field of a struct.

###### `Union`

A `union` declaration.

###### `Enum`

An `enum` declaration.

###### `Variant`

A variant of a enum.

###### `Function`

A function declaration, e.g. `fn f() {}`

###### `TypeAlias`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

###### `Constant`

The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

###### `Trait`

A `trait` declaration.

###### `TraitAlias`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

###### `Impl`

An `impl` block.

###### `Static`

A `static` declaration.

###### `ExternType`

`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

###### `Macro`

A macro declaration.

Corresponds to either `ItemEnum::Macro(_)`
or `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Bang })`

###### `ProcAttribute`

A procedural macro attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Attr })`

###### `ProcDerive`

A procedural macro usable in the `#[derive()]` attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Derive })`

###### `AssocConst`

An associated constant of a trait or a type.

###### `AssocType`

An associated type of a trait or a type.

###### `Primitive`

A primitive type, e.g. `u32`.

[`Item`]s of this kind only come from the core library.

###### `Keyword`

A keyword declaration.

[`Item`]s of this kind only come from the come library and exist solely
to carry documentation for the respective keywords.

##### Implementations

###### Trait Implementations

- **DeserializeOwned**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ItemKind) -> bool { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ItemKind { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Freeze**
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

- **StructuralPartialEq**
#### Enum `ItemEnum`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

Specific fields of an item.

Part of [`Item`].

```rust
pub enum ItemEnum {
    Module(Module),
    ExternCrate {
        name: String,
        rename: Option<String>,
    },
    Use(Use),
    Union(Union),
    Struct(Struct),
    StructField(Type),
    Enum(Enum),
    Variant(Variant),
    Function(Function),
    Trait(Trait),
    TraitAlias(TraitAlias),
    Impl(Impl),
    TypeAlias(TypeAlias),
    Constant {
        type_: Type,
        const_: Constant,
    },
    Static(Static),
    ExternType,
    Macro(String),
    ProcMacro(ProcMacro),
    Primitive(Primitive),
    AssocConst {
        type_: Type,
        value: Option<String>,
    },
    AssocType {
        generics: Generics,
        bounds: Vec<GenericBound>,
        type_: Option<Type>,
    },
}
```

##### Variants

###### `Module`

A module declaration, e.g. `mod foo;` or `mod foo {}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Module` |  |

###### `ExternCrate`

A crate imported via the `extern crate` syntax.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the imported crate. |
| `rename` | `Option<String>` | If the crate is renamed, this is its name in the crate. |

###### `Use`

An import of 1 or more items into scope, using the `use` keyword.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Use` |  |

###### `Union`

A `union` declaration.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Union` |  |

###### `Struct`

A `struct` declaration.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Struct` |  |

###### `StructField`

A field of a struct.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Type` |  |

###### `Enum`

An `enum` declaration.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Enum` |  |

###### `Variant`

A variant of a enum.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Variant` |  |

###### `Function`

A function declaration (including methods and other associated functions)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Function` |  |

###### `Trait`

A `trait` declaration.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Trait` |  |

###### `TraitAlias`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TraitAlias` |  |

###### `Impl`

An `impl` block.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Impl` |  |

###### `TypeAlias`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TypeAlias` |  |

###### `Constant`

The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `Type` | The type of the constant. |
| `const_` | `Constant` | The declared constant itself. |

###### `Static`

A declaration of a `static`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Static` |  |

###### `ExternType`

`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

###### `Macro`

A macro_rules! declarative macro. Contains a single string with the source
representation of the macro with the patterns stripped.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `ProcMacro`

A procedural macro.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ProcMacro` |  |

###### `Primitive`

A primitive type, e.g. `u32`.

[`Item`]s of this kind only come from the core library.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Primitive` |  |

###### `AssocConst`

An associated constant of a trait or a type.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `Type` | The type of the constant. |
| `value` | `Option<String>` | Inside a trait declaration, this is the default value for the associated constant,<br>if provided.<br>Inside an `impl` block, this is the value assigned to the associated constant,<br>and will always be present.<br><br>The representation is implementation-defined and not guaranteed to be representative of<br>either the resulting value or of the source code.<br><br>```rust<br>const X: usize = 640 * 1024;<br>//               ^^^^^^^^^^<br>``` |

###### `AssocType`

An associated type of a trait or a type.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `generics` | `Generics` | The generic parameters and where clauses on ahis associated type. |
| `bounds` | `Vec<GenericBound>` | The bounds for this associated type. e.g.<br>```rust<br>trait IntoIterator {<br>    type Item;<br>    type IntoIter: Iterator<Item = Self::Item>;<br>//                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^<br>}<br>``` |
| `type_` | `Option<Type>` | Inside a trait declaration, this is the default for the associated type, if provided.<br>Inside an impl block, this is the type assigned to the associated type, and will always<br>be present.<br><br>```rust<br>type X = usize;<br>//       ^^^^^<br>``` |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ItemEnum { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **RefUnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **DeserializeOwned**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ItemEnum) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Unpin**
- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
#### Struct `Module`

A module declaration, e.g. `mod foo;` or `mod foo {}`.

```rust
pub struct Module {
    pub is_crate: bool,
    pub items: Vec<Id>,
    pub is_stripped: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `is_crate` | `bool` | Whether this is the root item of a crate.<br><br>This item doesn't correspond to any construction in the source code and is generated by the<br>compiler. |
| `items` | `Vec<Id>` | [`Item`]s declared inside this module. |
| `is_stripped` | `bool` | If `true`, this module is not part of the public API, but it contains<br>items that are re-exported as public API. |

##### Implementations

###### Trait Implementations

- **DeserializeOwned**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Module) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Module { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **UnwindSafe**
#### Struct `Union`

A `union`.

```rust
pub struct Union {
    pub generics: Generics,
    pub has_stripped_fields: bool,
    pub fields: Vec<Id>,
    pub impls: Vec<Id>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `generics` | `Generics` | The generic parameters and where clauses on this union. |
| `has_stripped_fields` | `bool` | Whether any fields have been removed from the result, due to being private or hidden. |
| `fields` | `Vec<Id>` | The list of fields in the union.<br><br>All of the corresponding [`Item`]s are of kind [`ItemEnum::StructField`]. |
| `impls` | `Vec<Id>` | All impls (both of traits and inherent) for this union.<br><br>All of the corresponding [`Item`]s are of kind [`ItemEnum::Impl`]. |

##### Implementations

###### Trait Implementations

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Union) -> bool { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Union { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Eq**
- **DeserializeOwned**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `Struct`

A `struct`.

```rust
pub struct Struct {
    pub kind: StructKind,
    pub generics: Generics,
    pub impls: Vec<Id>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `kind` | `StructKind` | The kind of the struct (e.g. unit, tuple-like or struct-like) and the data specific to it,<br>i.e. fields. |
| `generics` | `Generics` | The generic parameters and where clauses on this struct. |
| `impls` | `Vec<Id>` | All impls (both of traits and inherent) for this struct.<br>All of the corresponding [`Item`]s are of kind [`ItemEnum::Impl`]. |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Struct) -> bool { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Struct { /* ... */ }
    ```

- **Eq**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Freeze**
- **DeserializeOwned**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Enum `StructKind`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

The kind of a [`Struct`] and the data specific to it, i.e. fields.

```rust
pub enum StructKind {
    Unit,
    Tuple(Vec<Option<Id>>),
    Plain {
        fields: Vec<Id>,
        has_stripped_fields: bool,
    },
}
```

##### Variants

###### `Unit`

A struct with no fields and no parentheses.

```rust
pub struct Unit;
```

###### `Tuple`

A struct with unnamed fields.

All [`Id`]'s will point to [`ItemEnum::StructField`].
Unlike most of JSON, private and `#[doc(hidden)]` fields will be given as `None`
instead of being omitted, because order matters.

```rust
pub struct TupleStruct(i32);
pub struct EmptyTupleStruct();
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Option<Id>>` |  |

###### `Plain`

A struct with named fields.

```rust
pub struct PlainStruct { x: i32 }
pub struct EmptyPlainStruct {}
```

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `fields` | `Vec<Id>` | The list of fields in the struct.<br><br>All of the corresponding [`Item`]s are of kind [`ItemEnum::StructField`]. |
| `has_stripped_fields` | `bool` | Whether any fields have been removed from the result, due to being private or hidden. |

##### Implementations

###### Trait Implementations

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeOwned**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StructKind { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StructKind) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `Enum`

An `enum`.

```rust
pub struct Enum {
    pub generics: Generics,
    pub has_stripped_variants: bool,
    pub variants: Vec<Id>,
    pub impls: Vec<Id>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `generics` | `Generics` | Information about the type parameters and `where` clauses of the enum. |
| `has_stripped_variants` | `bool` | Whether any variants have been removed from the result, due to being private or hidden. |
| `variants` | `Vec<Id>` | The list of variants in the enum.<br><br>All of the corresponding [`Item`]s are of kind [`ItemEnum::Variant`] |
| `impls` | `Vec<Id>` | `impl`s for the enum. |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Enum) -> bool { /* ... */ }
    ```

- **DeserializeOwned**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Enum { /* ... */ }
    ```

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
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

- **UnwindSafe**
- **Unpin**
- **Send**
#### Struct `Variant`

A variant of an enum.

```rust
pub struct Variant {
    pub kind: VariantKind,
    pub discriminant: Option<Discriminant>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `kind` | `VariantKind` | Whether the variant is plain, a tuple-like, or struct-like. Contains the fields. |
| `discriminant` | `Option<Discriminant>` | The discriminant, if explicitly specified. |

##### Implementations

###### Trait Implementations

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Variant) -> bool { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeOwned**
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

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Variant { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

#### Enum `VariantKind`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

The kind of an [`Enum`] [`Variant`] and the data specific to it, i.e. fields.

```rust
pub enum VariantKind {
    Plain,
    Tuple(Vec<Option<Id>>),
    Struct {
        fields: Vec<Id>,
        has_stripped_fields: bool,
    },
}
```

##### Variants

###### `Plain`

A variant with no parentheses

```rust
enum Demo {
    PlainVariant,
    PlainWithDiscriminant = 1,
}
```

###### `Tuple`

A variant with unnamed fields.

All [`Id`]'s will point to [`ItemEnum::StructField`].
Unlike most of JSON, `#[doc(hidden)]` fields will be given as `None`
instead of being omitted, because order matters.

```rust
enum Demo {
    TupleVariant(i32),
    EmptyTupleVariant(),
}
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Option<Id>>` |  |

###### `Struct`

A variant with named fields.

```rust
enum Demo {
    StructVariant { x: i32 },
    EmptyStructVariant {},
}
```

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `fields` | `Vec<Id>` | The list of variants in the enum.<br>All of the corresponding [`Item`]s are of kind [`ItemEnum::Variant`]. |
| `has_stripped_fields` | `bool` | Whether any variants have been removed from the result, due to being private or hidden. |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VariantKind) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Send**
- **DeserializeOwned**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> VariantKind { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Discriminant`

The value that distinguishes a variant in an [`Enum`] from other variants.

```rust
pub struct Discriminant {
    pub expr: String,
    pub value: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `String` | The expression that produced the discriminant.<br><br>Unlike `value`, this preserves the original formatting (eg suffixes,<br>hexadecimal, and underscores), making it unsuitable to be machine<br>interpreted.<br><br>In some cases, when the value is too complex, this may be `"{ _ }"`.<br>When this occurs is unstable, and may change without notice. |
| `value` | `String` | The numerical value of the discriminant. Stored as a string due to<br>JSON's poor support for large integers, and the fact that it would need<br>to store from [`i128::MIN`] to [`u128::MAX`]. |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Discriminant { /* ... */ }
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

- **StructuralPartialEq**
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

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DeserializeOwned**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Discriminant) -> bool { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `FunctionHeader`

A set of fundamental properties of a function.

```rust
pub struct FunctionHeader {
    pub is_const: bool,
    pub is_unsafe: bool,
    pub is_async: bool,
    pub abi: Abi,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `is_const` | `bool` | Is this function marked as `const`? |
| `is_unsafe` | `bool` | Is this function unsafe? |
| `is_async` | `bool` | Is this function async? |
| `abi` | `Abi` | The ABI used by the function. |

##### Implementations

###### Trait Implementations

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FunctionHeader { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FunctionHeader) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **DeserializeOwned**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **RefUnwindSafe**
- **StructuralPartialEq**
- **Send**
- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

#### Enum `Abi`

The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be specified in 2
ways: `extern "_"` and `extern "_-unwind"`, and a value of `true` for that field signifies the
latter variant.

See the [Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

```rust
pub enum Abi {
    Rust,
    C {
        unwind: bool,
    },
    Cdecl {
        unwind: bool,
    },
    Stdcall {
        unwind: bool,
    },
    Fastcall {
        unwind: bool,
    },
    Aapcs {
        unwind: bool,
    },
    Win64 {
        unwind: bool,
    },
    SysV64 {
        unwind: bool,
    },
    System {
        unwind: bool,
    },
    Other(String),
}
```

##### Variants

###### `Rust`

The default ABI, but that can also be written explicitly with `extern "Rust"`.

###### `C`

Can be specified as `extern "C"` or, as a shorthand, just `extern`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `unwind` | `bool` |  |

###### `Cdecl`

Can be specified as `extern "cdecl"`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `unwind` | `bool` |  |

###### `Stdcall`

Can be specified as `extern "stdcall"`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `unwind` | `bool` |  |

###### `Fastcall`

Can be specified as `extern "fastcall"`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `unwind` | `bool` |  |

###### `Aapcs`

Can be specified as `extern "aapcs"`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `unwind` | `bool` |  |

###### `Win64`

Can be specified as `extern "win64"`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `unwind` | `bool` |  |

###### `SysV64`

Can be specified as `extern "sysv64"`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `unwind` | `bool` |  |

###### `System`

Can be specified as `extern "system"`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `unwind` | `bool` |  |

###### `Other`

Any other ABI, including unstable ones.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **DeserializeOwned**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Abi { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Abi) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
#### Struct `Function`

A function declaration (including methods and other associated functions).

```rust
pub struct Function {
    pub sig: FunctionSignature,
    pub generics: Generics,
    pub header: FunctionHeader,
    pub has_body: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sig` | `FunctionSignature` | Information about the function signature, or declaration. |
| `generics` | `Generics` | Information about the function’s type parameters and `where` clauses. |
| `header` | `FunctionHeader` | Information about core properties of the function, e.g. whether it's `const`, its ABI, etc. |
| `has_body` | `bool` | Whether the function has a body, i.e. an implementation. |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Function) -> bool { /* ... */ }
    ```

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

- **Send**
- **Freeze**
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

- **RefUnwindSafe**
- **DeserializeOwned**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Function { /* ... */ }
    ```

- **Eq**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Generics`

Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

```rust
pub struct Generics {
    pub params: Vec<GenericParamDef>,
    pub where_predicates: Vec<WherePredicate>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `params` | `Vec<GenericParamDef>` | A list of generic parameter definitions (e.g. `<T: Clone + Hash, U: Copy>`). |
| `where_predicates` | `Vec<WherePredicate>` | A list of where predicates (e.g. `where T: Iterator, T::Item: Copy`). |

##### Implementations

###### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DeserializeOwned**
- **UnwindSafe**
- **RefUnwindSafe**
- **StructuralPartialEq**
- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Generics { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Generics) -> bool { /* ... */ }
    ```

#### Struct `GenericParamDef`

One generic parameter accepted by an item.

```rust
pub struct GenericParamDef {
    pub name: String,
    pub kind: GenericParamDefKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | Name of the parameter.<br>```rust<br>fn f<'resource, Resource>(x: &'resource Resource) {}<br>//    ^^^^^^^^  ^^^^^^^^<br>``` |
| `kind` | `GenericParamDefKind` | The kind of the parameter and data specific to a particular parameter kind, e.g. type<br>bounds. |

##### Implementations

###### Trait Implementations

- **DeserializeOwned**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GenericParamDef) -> bool { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenericParamDef { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Send**
- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
#### Enum `GenericParamDefKind`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

The kind of a [`GenericParamDef`].

```rust
pub enum GenericParamDefKind {
    Lifetime {
        outlives: Vec<String>,
    },
    Type {
        bounds: Vec<GenericBound>,
        default: Option<Type>,
        is_synthetic: bool,
    },
    Const {
        type_: Type,
        default: Option<String>,
    },
}
```

##### Variants

###### `Lifetime`

Denotes a lifetime parameter.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `outlives` | `Vec<String>` | Lifetimes that this lifetime parameter is required to outlive.<br><br>```rust<br>fn f<'a, 'b, 'resource: 'a + 'b>(a: &'a str, b: &'b str, res: &'resource str) {}<br>//                      ^^^^^^^<br>``` |

###### `Type`

Denotes a type parameter.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `bounds` | `Vec<GenericBound>` | Bounds applied directly to the type. Note that the bounds from `where` clauses<br>that constrain this parameter won't appear here.<br><br>```rust<br>fn default2<T: Default>() -> [T; 2] where T: Clone { todo!() }<br>//             ^^^^^^^<br>``` |
| `default` | `Option<Type>` | The default type for this parameter, if provided, e.g.<br><br>```rust<br>trait PartialEq<Rhs = Self> {}<br>//                    ^^^^<br>``` |
| `is_synthetic` | `bool` | This is normally `false`, which means that this generic parameter is<br>declared in the Rust source text.<br><br>If it is `true`, this generic parameter has been introduced by the<br>compiler behind the scenes.<br><br># Example<br><br>Consider<br><br>```ignore (pseudo-rust)<br>pub fn f(_: impl Trait) {}<br>```<br><br>The compiler will transform this behind the scenes to<br><br>```ignore (pseudo-rust)<br>pub fn f<impl Trait: Trait>(_: impl Trait) {}<br>```<br><br>In this example, the generic parameter named `impl Trait` (and which<br>is bound by `Trait`) is synthetic, because it was not originally in<br>the Rust source text. |

###### `Const`

Denotes a constant parameter.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `Type` | The type of the constant as declared. |
| `default` | `Option<String>` | The stringified expression for the default value, if provided. It's not guaranteed that<br>it'll match the actual source code for the default value. |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenericParamDefKind { /* ... */ }
    ```

- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **DeserializeOwned**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Freeze**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GenericParamDefKind) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `WherePredicate`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

One `where` clause.
```rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
```

```rust
pub enum WherePredicate {
    BoundPredicate {
        type_: Type,
        bounds: Vec<GenericBound>,
        generic_params: Vec<GenericParamDef>,
    },
    LifetimePredicate {
        lifetime: String,
        outlives: Vec<String>,
    },
    EqPredicate {
        lhs: Type,
        rhs: Term,
    },
}
```

##### Variants

###### `BoundPredicate`

A type is expected to comply with a set of bounds

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `Type` | The type that's being constrained.<br><br>```rust<br>fn f<T>(x: T) where for<'a> &'a T: Iterator {}<br>//                              ^<br>``` |
| `bounds` | `Vec<GenericBound>` | The set of bounds that constrain the type.<br><br>```rust<br>fn f<T>(x: T) where for<'a> &'a T: Iterator {}<br>//                                 ^^^^^^^^<br>``` |
| `generic_params` | `Vec<GenericParamDef>` | Used for Higher-Rank Trait Bounds (HRTBs)<br>```rust<br>fn f<T>(x: T) where for<'a> &'a T: Iterator {}<br>//                  ^^^^^^^<br>``` |

###### `LifetimePredicate`

A lifetime is expected to outlive other lifetimes.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `lifetime` | `String` | The name of the lifetime. |
| `outlives` | `Vec<String>` | The lifetimes that must be encompassed by the lifetime. |

###### `EqPredicate`

A type must exactly equal another type.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `lhs` | `Type` | The left side of the equation. |
| `rhs` | `Term` | The right side of the equation. |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Eq**
- **DeserializeOwned**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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
    fn clone(self: &Self) -> WherePredicate { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WherePredicate) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **UnwindSafe**
#### Enum `GenericBound`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

Either a trait bound or a lifetime bound.

```rust
pub enum GenericBound {
    TraitBound {
        trait_: Path,
        generic_params: Vec<GenericParamDef>,
        modifier: TraitBoundModifier,
    },
    Outlives(String),
    Use(Vec<PreciseCapturingArg>),
}
```

##### Variants

###### `TraitBound`

A trait bound.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `trait_` | `Path` | The full path to the trait. |
| `generic_params` | `Vec<GenericParamDef>` | Used for Higher-Rank Trait Bounds (HRTBs)<br>```text<br>where F: for<'a, 'b> Fn(&'a u8, &'b u8)<br>         ^^^^^^^^^^^<br>         |<br>         this part<br>``` |
| `modifier` | `TraitBoundModifier` | The context for which a trait is supposed to be used, e.g. `const |

###### `Outlives`

A lifetime bound, e.g.
```rust
fn f<'a, T>(x: &'a str, y: &T) where T: 'a {}
//                                     ^^^
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Use`

`use<'a, T>` precise-capturing bound syntax

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<PreciseCapturingArg>` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenericBound { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GenericBound) -> bool { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `TraitBoundModifier`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

A set of modifiers applied to a trait.

```rust
pub enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}
```

##### Variants

###### `None`

Marks the absence of a modifier.

###### `Maybe`

Indicates that the trait bound relaxes a trait bound applied to a parameter by default,
e.g. `T: Sized?`, the `Sized` trait is required for all generic type parameters by default
unless specified otherwise with this modifier.

###### `MaybeConst`

Indicates that the trait bound must be applicable in both a run-time and a compile-time
context.

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **DeserializeOwned**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TraitBoundModifier { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TraitBoundModifier) -> bool { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Enum `PreciseCapturingArg`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

One precise capturing argument. See [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

```rust
pub enum PreciseCapturingArg {
    Lifetime(String),
    Param(String),
}
```

##### Variants

###### `Lifetime`

A lifetime.
```rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                        ^^

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Param`

A type or constant parameter.
```rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                            ^  ^

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PreciseCapturingArg { /* ... */ }
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

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
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

- **DeserializeOwned**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PreciseCapturingArg) -> bool { /* ... */ }
    ```

#### Enum `Term`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

Either a type or a constant, usually stored as the right-hand side of an equation in places like
[`AssocItemConstraint`]

```rust
pub enum Term {
    Type(Type),
    Constant(Constant),
}
```

##### Variants

###### `Type`

A type.

```rust
fn f(x: impl IntoIterator<Item = u32>) {}
//                               ^^^
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Type` |  |

###### `Constant`

A constant.

```ignore (incomplete feature in the snippet)
trait Foo {
    const BAR: usize;
}

fn f(x: impl Foo<BAR = 42>) {}
//                     ^^
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Constant` |  |

##### Implementations

###### Trait Implementations

- **Sync**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **UnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Term { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Term) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
#### Enum `Type`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

A type.

```rust
pub enum Type {
    ResolvedPath(Path),
    DynTrait(DynTrait),
    Generic(String),
    Primitive(String),
    FunctionPointer(Box<FunctionPointer>),
    Tuple(Vec<Type>),
    Slice(Box<Type>),
    Array {
        type_: Box<Type>,
        len: String,
    },
    Pat {
        type_: Box<Type>,
        // Some fields omitted
    },
    ImplTrait(Vec<GenericBound>),
    Infer,
    RawPointer {
        is_mutable: bool,
        type_: Box<Type>,
    },
    BorrowedRef {
        lifetime: Option<String>,
        is_mutable: bool,
        type_: Box<Type>,
    },
    QualifiedPath {
        name: String,
        args: Box<GenericArgs>,
        self_type: Box<Type>,
        trait_: Option<Path>,
    },
}
```

##### Variants

###### `ResolvedPath`

Structs, enums, unions and type aliases, e.g. `std::option::Option<u32>`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Path` |  |

###### `DynTrait`

Dynamic trait object type (`dyn Trait`).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `DynTrait` |  |

###### `Generic`

Parameterized types. The contained string is the name of the parameter.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Primitive`

Built-in numeric types (e.g. `u32`, `f32`), `bool`, `char`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `FunctionPointer`

A function pointer type, e.g. `fn(u32) -> u32`, `extern "C" fn() -> *const u8`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<FunctionPointer>` |  |

###### `Tuple`

A tuple type, e.g. `(String, u32, Box<usize>)`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Type>` |  |

###### `Slice`

An unsized slice type, e.g. `[u32]`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Type>` |  |

###### `Array`

An array type, e.g. `[u32; 15]`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `Box<Type>` | The type of the contained element. |
| `len` | `String` | The stringified expression that is the length of the array.<br><br>Keep in mind that it's not guaranteed to match the actual source code of the expression. |

###### `Pat`

A pattern type, e.g. `u32 is 1..`

See [the tracking issue](https://github.com/rust-lang/rust/issues/123646)

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `Box<Type>` | The base type, e.g. the `u32` in `u32 is 1..` |
| *private fields* | ... | *Some fields have been omitted* |

###### `ImplTrait`

An opaque type that satisfies a set of bounds, `impl TraitA + TraitB + ...`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<GenericBound>` |  |

###### `Infer`

A type that's left to be inferred, `_`

###### `RawPointer`

A raw pointer type, e.g. `*mut u32`, `*const u8`, etc.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `is_mutable` | `bool` | This is `true` for `*mut _` and `false` for `*const _`. |
| `type_` | `Box<Type>` | The type of the pointee. |

###### `BorrowedRef`

`&'a mut String`, `&str`, etc.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `lifetime` | `Option<String>` | The name of the lifetime of the reference, if provided. |
| `is_mutable` | `bool` | This is `true` for `&mut i32` and `false` for `&i32` |
| `type_` | `Box<Type>` | The type of the pointee, e.g. the `i32` in `&'a mut i32` |

###### `QualifiedPath`

Associated types like `<Type as Trait>::Name` and `T::Item` where
`T: Iterator` or inherent associated types like `Struct::Name`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the associated type in the parent type.<br><br>```ignore (incomplete expression)<br><core::array::IntoIter<u32, 42> as Iterator>::Item<br>//                                            ^^^^<br>``` |
| `args` | `Box<GenericArgs>` | The generic arguments provided to the associated type.<br><br>```ignore (incomplete expression)<br><core::slice::IterMut<'static, u32> as BetterIterator>::Item<'static><br>//                                                          ^^^^^^^^^<br>``` |
| `self_type` | `Box<Type>` | The type with which this type is associated.<br><br>```ignore (incomplete expression)<br><core::array::IntoIter<u32, 42> as Iterator>::Item<br>// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^<br>``` |
| `trait_` | `Option<Path>` | `None` iff this is an *inherent* associated type. |

##### Implementations

###### Trait Implementations

- **Send**
- **StructuralPartialEq**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Type { /* ... */ }
    ```

- **Sync**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DeserializeOwned**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Type) -> bool { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `Path`

A type that has a simple path to it. This is the kind of type of structs, unions, enums, etc.

```rust
pub struct Path {
    pub path: String,
    pub id: Id,
    pub args: Option<Box<GenericArgs>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `path` | `String` | The path of the type.<br><br>This will be the path that is *used* (not where it is defined), so<br>multiple `Path`s may have different values for this field even if<br>they all refer to the same item. e.g.<br><br>```rust<br>pub type Vec1 = std::vec::Vec<i32>; // path: "std::vec::Vec"<br>pub type Vec2 = Vec<i32>; // path: "Vec"<br>pub type Vec3 = std::prelude::v1::Vec<i32>; // path: "std::prelude::v1::Vec"<br>``` |
| `id` | `Id` | The ID of the type. |
| `args` | `Option<Box<GenericArgs>>` | Generic arguments to the type.<br><br>```ignore (incomplete expression)<br>std::borrow::Cow<'static, str><br>//              ^^^^^^^^^^^^^^<br>``` |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Path { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
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

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DeserializeOwned**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Path) -> bool { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **Freeze**
#### Struct `FunctionPointer`

A type that is a function pointer.

```rust
pub struct FunctionPointer {
    pub sig: FunctionSignature,
    pub generic_params: Vec<GenericParamDef>,
    pub header: FunctionHeader,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sig` | `FunctionSignature` | The signature of the function. |
| `generic_params` | `Vec<GenericParamDef>` | Used for Higher-Rank Trait Bounds (HRTBs)<br><br>```ignore (incomplete expression)<br>   for<'c> fn(val: &'c i32) -> i32<br>// ^^^^^^^<br>``` |
| `header` | `FunctionHeader` | The core properties of the function, such as the ABI it conforms to, whether it's unsafe, etc. |

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

- **Freeze**
- **Sync**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FunctionPointer { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **DeserializeOwned**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FunctionPointer) -> bool { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **StructuralPartialEq**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
#### Struct `FunctionSignature`

The signature of a function.

```rust
pub struct FunctionSignature {
    pub inputs: Vec<(String, Type)>,
    pub output: Option<Type>,
    pub is_c_variadic: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inputs` | `Vec<(String, Type)>` | List of argument names and their type.<br><br>Note that not all names will be valid identifiers, as some of<br>them may be patterns. |
| `output` | `Option<Type>` | The output type, if specified. |
| `is_c_variadic` | `bool` | Whether the function accepts an arbitrary amount of trailing arguments the C way.<br><br>```ignore (incomplete code)<br>fn printf(fmt: &str, ...);<br>``` |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FunctionSignature { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DeserializeOwned**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FunctionSignature) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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

#### Struct `Trait`

A `trait` declaration.

```rust
pub struct Trait {
    pub is_auto: bool,
    pub is_unsafe: bool,
    pub is_dyn_compatible: bool,
    pub items: Vec<Id>,
    pub generics: Generics,
    pub bounds: Vec<GenericBound>,
    pub implementations: Vec<Id>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `is_auto` | `bool` | Whether the trait is marked `auto` and is thus implemented automatically<br>for all applicable types. |
| `is_unsafe` | `bool` | Whether the trait is marked as `unsafe`. |
| `is_dyn_compatible` | `bool` | Whether the trait is [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)[^1].<br><br>[^1]: Formerly known as "object safe". |
| `items` | `Vec<Id>` | Associated [`Item`]s that can/must be implemented by the `impl` blocks. |
| `generics` | `Generics` | Information about the type parameters and `where` clauses of the trait. |
| `bounds` | `Vec<GenericBound>` | Constraints that must be met by the implementor of the trait. |
| `implementations` | `Vec<Id>` | The implementations of the trait. |

##### Implementations

###### Trait Implementations

- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Trait { /* ... */ }
    ```

- **Eq**
- **Freeze**
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

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Trait) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
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

#### Struct `TraitAlias`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

```rust
pub struct TraitAlias {
    pub generics: Generics,
    pub params: Vec<GenericBound>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `generics` | `Generics` | Information about the type parameters and `where` clauses of the alias. |
| `params` | `Vec<GenericBound>` | The bounds that are associated with the alias. |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TraitAlias) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Send**
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

- **DeserializeOwned**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TraitAlias { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Impl`

An `impl` block.

```rust
pub struct Impl {
    pub is_unsafe: bool,
    pub generics: Generics,
    pub provided_trait_methods: Vec<String>,
    pub trait_: Option<Path>,
    pub for_: Type,
    pub items: Vec<Id>,
    pub is_negative: bool,
    pub is_synthetic: bool,
    pub blanket_impl: Option<Type>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `is_unsafe` | `bool` | Whether this impl is for an unsafe trait. |
| `generics` | `Generics` | Information about the impl’s type parameters and `where` clauses. |
| `provided_trait_methods` | `Vec<String>` | The list of the names of all the trait methods that weren't mentioned in this impl but<br>were provided by the trait itself.<br><br>For example, for this impl of the [`PartialEq`] trait:<br>```rust<br>struct Foo;<br><br>impl PartialEq for Foo {<br>    fn eq(&self, other: &Self) -> bool { todo!() }<br>}<br>```<br>This field will be `["ne"]`, as it has a default implementation defined for it. |
| `trait_` | `Option<Path>` | The trait being implemented or `None` if the impl is inherent, which means<br>`impl Struct {}` as opposed to `impl Trait for Struct {}`. |
| `for_` | `Type` | The type that the impl block is for. |
| `items` | `Vec<Id>` | The list of associated items contained in this impl block. |
| `is_negative` | `bool` | Whether this is a negative impl (e.g. `!Sized` or `!Send`). |
| `is_synthetic` | `bool` | Whether this is an impl that’s implied by the compiler<br>(for autotraits, e.g. `Send` or `Sync`). |
| `blanket_impl` | `Option<Type>` |  |

##### Implementations

###### Trait Implementations

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Sync**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Impl) -> bool { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Impl { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `Use`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

A `use` statement.

```rust
pub struct Use {
    pub source: String,
    pub name: String,
    pub id: Option<Id>,
    pub is_glob: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `source` | `String` | The full path being imported. |
| `name` | `String` | May be different from the last segment of `source` when renaming imports:<br>`use source as name;` |
| `id` | `Option<Id>` | The ID of the item being imported. Will be `None` in case of re-exports of primitives:<br>```rust<br>pub use i32 as my_i32;<br>``` |
| `is_glob` | `bool` | Whether this statement is a wildcard `use`, e.g. `use source::*;` |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Use { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **DeserializeOwned**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
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

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Use) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

#### Struct `ProcMacro`

A procedural macro.

```rust
pub struct ProcMacro {
    pub kind: MacroKind,
    pub helpers: Vec<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `kind` | `MacroKind` | How this macro is supposed to be called: `foo!()`, `#[foo]` or `#[derive(foo)]` |
| `helpers` | `Vec<String>` | Helper attributes defined by a macro to be used inside it.<br><br>Defined only for derive macros.<br><br>E.g. the [`Default`] derive macro defines a `#[default]` helper attribute so that one can<br>do:<br><br>```rust<br>#[derive(Default)]<br>enum Option<T> {<br>    #[default]<br>    None,<br>    Some(T),<br>}<br>``` |

##### Implementations

###### Trait Implementations

- **Sync**
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **UnwindSafe**
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

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ProcMacro) -> bool { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **DeserializeOwned**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProcMacro { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `MacroKind`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

The way a [`ProcMacro`] is declared to be used.

```rust
pub enum MacroKind {
    Bang,
    Attr,
    Derive,
}
```

##### Variants

###### `Bang`

A bang macro `foo!()`.

###### `Attr`

An attribute macro `#[foo]`.

###### `Derive`

A derive macro `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]`

##### Implementations

###### Trait Implementations

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

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MacroKind { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
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

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MacroKind) -> bool { /* ... */ }
    ```

- **DeserializeOwned**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `TypeAlias`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

```rust
pub struct TypeAlias {
    pub type_: Type,
    pub generics: Generics,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `Type` | The type referred to by this alias. |
| `generics` | `Generics` | Information about the type parameters and `where` clauses of the alias. |

##### Implementations

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DeserializeOwned**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TypeAlias { /* ... */ }
    ```

- **Sync**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TypeAlias) -> bool { /* ... */ }
    ```

#### Struct `Static`

A `static` declaration.

```rust
pub struct Static {
    pub type_: Type,
    pub is_mutable: bool,
    pub expr: String,
    pub is_unsafe: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `Type` | The type of the static. |
| `is_mutable` | `bool` | This is `true` for mutable statics, declared as `static mut X: T = f();` |
| `expr` | `String` | The stringified expression for the initial value.<br><br>It's not guaranteed that it'll match the actual source code for the initial value. |
| `is_unsafe` | `bool` | Is the static `unsafe`?<br><br>This is only true if it's in an `extern` block, and not explicity marked<br>as `safe`.<br><br>```rust<br>unsafe extern {<br>    static A: i32;      // unsafe<br>    safe static B: i32; // safe<br>}<br><br>static C: i32 = 0;     // safe<br>static mut D: i32 = 0; // safe<br>``` |

##### Implementations

###### Trait Implementations

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

- **Eq**
- **Sync**
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

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **DeserializeOwned**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **RefUnwindSafe**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Static) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Static { /* ... */ }
    ```

#### Struct `Primitive`

A primitive type declaration. Declarations of this kind can only come from the core library.

```rust
pub struct Primitive {
    pub name: String,
    pub impls: Vec<Id>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the type. |
| `impls` | `Vec<Id>` | The implementations, inherent and of traits, on the primitive type. |

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DeserializeOwned**
- **Eq**
- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Primitive) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Primitive { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

### Constants and Statics

#### Constant `FORMAT_VERSION`

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as [`Crate::format_version`].
Consuming code should assert that this value matches the format version(s) that it supports.

```rust
pub const FORMAT_VERSION: u32 = 42;
```

## Functions

### Function `rustdoc_json_to_markdown`

```rust
pub fn rustdoc_json_to_markdown(data: Crate) -> String { /* ... */ }
```

