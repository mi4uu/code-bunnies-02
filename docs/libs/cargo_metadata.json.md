# Crate Documentation

**Version:** 0.19.2

**Format Version:** 43

# Module `cargo_metadata`

Structured access to the output of `cargo metadata` and `cargo --message-format=json`.
Usually used from within a `cargo-*` executable

See the [cargo book](https://doc.rust-lang.org/cargo/index.html) for
details on cargo itself.

## Examples

```rust
# extern crate cargo_metadata;
# use std::path::Path;
let mut args = std::env::args().skip_while(|val| !val.starts_with("--manifest-path"));

let mut cmd = cargo_metadata::MetadataCommand::new();
let manifest_path = match args.next() {
    Some(ref p) if p == "--manifest-path" => {
        cmd.manifest_path(args.next().unwrap());
    }
    Some(p) => {
        cmd.manifest_path(p.trim_start_matches("--manifest-path="));
    }
    None => {}
};

let _metadata = cmd.exec().unwrap();
```

Pass features flags

```rust
# // This should be kept in sync with the equivalent example in the readme.
# extern crate cargo_metadata;
# use std::path::Path;
# fn main() {
use cargo_metadata::{MetadataCommand, CargoOpt};

let _metadata = MetadataCommand::new()
    .manifest_path("./Cargo.toml")
    .features(CargoOpt::AllFeatures)
    .exec()
    .unwrap();
# }
```

Parse message-format output:

```
# extern crate cargo_metadata;
use std::process::{Stdio, Command};
use cargo_metadata::Message;

let mut command = Command::new("cargo")
    .args(&["build", "--message-format=json-render-diagnostics"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

let reader = std::io::BufReader::new(command.stdout.take().unwrap());
for message in cargo_metadata::Message::parse_stream(reader) {
    match message.unwrap() {
        Message::CompilerMessage(msg) => {
            println!("{:?}", msg);
        },
        Message::CompilerArtifact(artifact) => {
            println!("{:?}", artifact);
        },
        Message::BuildScriptExecuted(script) => {
            println!("{:?}", script);
        },
        Message::BuildFinished(finished) => {
            println!("{:?}", finished);
        },
        _ => () // Unknown message
    }
}

let output = command.wait().expect("Couldn't get cargo's exit status");
```

## Modules

## Module `diagnostic`

This module contains `Diagnostic` and the types/functions it uses for deserialization.

```rust
pub mod diagnostic { /* ... */ }
```

### Types

#### Struct `DiagnosticCode`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

The error code associated to this diagnostic.

```rust
pub struct DiagnosticCode {
    pub code: String,
    pub explanation: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `code` | `String` | The code itself. |
| `explanation` | `Option<String>` | An explanation for the code |

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiagnosticCode { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **Freeze**
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

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DiagnosticCode) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `DiagnosticSpanLine`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

A line of code associated with the Diagnostic

```rust
pub struct DiagnosticSpanLine {
    pub text: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `text` | `String` | The line of code associated with the error |
| `highlight_start` | `usize` | Start of the section of the line to highlight. 1-based, character offset in self.text |
| `highlight_end` | `usize` | End of the section of the line to highlight. 1-based, character offset in self.text |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiagnosticSpanLine { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DiagnosticSpanLine) -> bool { /* ... */ }
    ```

- **DeserializeOwned**
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

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `DiagnosticSpanMacroExpansion`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

Macro expansion information associated with a diagnostic.

```rust
pub struct DiagnosticSpanMacroExpansion {
    pub span: DiagnosticSpan,
    pub macro_decl_name: String,
    pub def_site_span: Option<DiagnosticSpan>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `DiagnosticSpan` | span where macro was applied to generate this code; note that<br>this may itself derive from a macro (if<br>`span.expansion.is_some()`) |
| `macro_decl_name` | `String` | name of macro that was applied (e.g., "foo!" or "#[derive(Eq)]") |
| `def_site_span` | `Option<DiagnosticSpan>` | span where macro was defined (if known) |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiagnosticSpanMacroExpansion { /* ... */ }
    ```

- **Send**
- **Sync**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DeserializeOwned**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DiagnosticSpanMacroExpansion) -> bool { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Eq**
#### Struct `DiagnosticSpan`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

A section of the source code associated with a Diagnostic

```rust
pub struct DiagnosticSpan {
    pub file_name: String,
    pub byte_start: u32,
    pub byte_end: u32,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub is_primary: bool,
    pub text: Vec<DiagnosticSpanLine>,
    pub label: Option<String>,
    pub suggested_replacement: Option<String>,
    pub suggestion_applicability: Option<Applicability>,
    pub expansion: Option<Box<DiagnosticSpanMacroExpansion>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `file_name` | `String` | The file name or the macro name this diagnostic comes from. |
| `byte_start` | `u32` | The byte offset in the file where this diagnostic starts from. |
| `byte_end` | `u32` | The byte offset in the file where this diagnostic ends. |
| `line_start` | `usize` | 1-based. The line in the file. |
| `line_end` | `usize` | 1-based. The line in the file. |
| `column_start` | `usize` | 1-based, character offset. |
| `column_end` | `usize` | 1-based, character offset. |
| `is_primary` | `bool` | Is this a "primary" span -- meaning the point, or one of the points,<br>where the error occurred?<br><br>There are rare cases where multiple spans are marked as primary,<br>e.g. "immutable borrow occurs here" and "mutable borrow ends here" can<br>be two separate spans both "primary". Top (parent) messages should<br>always have at least one primary span, unless it has 0 spans. Child<br>messages may have 0 or more primary spans. |
| `text` | `Vec<DiagnosticSpanLine>` | Source text from the start of line_start to the end of line_end. |
| `label` | `Option<String>` | Label that should be placed at this location (if any) |
| `suggested_replacement` | `Option<String>` | If we are suggesting a replacement, this will contain text<br>that should be sliced in atop this span. |
| `suggestion_applicability` | `Option<Applicability>` | If the suggestion is approximate |
| `expansion` | `Option<Box<DiagnosticSpanMacroExpansion>>` | Macro invocations that created the code at this span, if any. |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DiagnosticSpan) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **DeserializeOwned**
- **RefUnwindSafe**
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

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Sync**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Unpin**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiagnosticSpan { /* ... */ }
    ```

#### Enum `Applicability`

**Attributes:**

- `#[non_exhaustive]`

Whether a suggestion can be safely applied.

```rust
pub enum Applicability {
    MachineApplicable,
    HasPlaceholders,
    MaybeIncorrect,
    Unspecified,
}
```

##### Variants

###### `MachineApplicable`

The suggested replacement can be applied automatically safely

###### `HasPlaceholders`

The suggested replacement has placeholders that will need to be manually
replaced.

###### `MaybeIncorrect`

The suggested replacement may be incorrect in some circumstances. Needs
human review.

###### `Unspecified`

The suggested replacement will probably not work.

##### Implementations

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Applicability { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Applicability) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
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

- **DeserializeOwned**
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

- **Sync**
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

#### Enum `DiagnosticLevel`

**Attributes:**

- `#[non_exhaustive]`
- `#[serde(rename_all = "lowercase")]`

The diagnostic level

```rust
pub enum DiagnosticLevel {
    Ice,
    Error,
    Warning,
    FailureNote,
    Note,
    Help,
}
```

##### Variants

###### `Ice`

Internal compiler error

###### `Error`

Error

###### `Warning`

Warning

###### `FailureNote`

Failure note

###### `Note`

Note

###### `Help`

Help

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **UnwindSafe**
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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DiagnosticLevel) -> bool { /* ... */ }
    ```

- **Eq**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiagnosticLevel { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **DeserializeOwned**
#### Struct `Diagnostic`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

A diagnostic message generated by rustc

```rust
pub struct Diagnostic {
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub level: DiagnosticLevel,
    pub spans: Vec<DiagnosticSpan>,
    pub children: Vec<Diagnostic>,
    pub rendered: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `message` | `String` | The error message of this diagnostic. |
| `code` | `Option<DiagnosticCode>` | The associated error code for this diagnostic |
| `level` | `DiagnosticLevel` | "error: internal compiler error", "error", "warning", "note", "help" |
| `spans` | `Vec<DiagnosticSpan>` | A list of source code spans this diagnostic is associated with. |
| `children` | `Vec<Diagnostic>` | Associated diagnostic messages. |
| `rendered` | `Option<String>` | The message as rustc would render it |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DeserializeOwned**
- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Diagnostic { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Diagnostic) -> bool { /* ... */ }
    ```

- **Sync**
- **Unpin**
## Types

### Struct `PackageId`

**Attributes:**

- `#[serde(transparent)]`

An "opaque" identifier for a package.

It is possible to inspect the `repr` field, if the need arises, but its
precise format is an implementation detail and is subject to change.

`Metadata` can be indexed by `PackageId`.

```rust
pub struct PackageId {
    pub repr: String,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `repr` | `String` | The underlying string representation of id. |

#### Implementations

##### Trait Implementations

- **StructuralPartialEq**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PackageId) -> bool { /* ... */ }
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PackageId) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PackageId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &PackageId) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **DeserializeOwned**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Index**
  - ```rust
    fn index(self: &Self, idx: &''a PackageId) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, idx: &''a PackageId) -> &<Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
- **Eq**
### Struct `Metadata`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

Starting point for metadata returned by `cargo metadata`

```rust
pub struct Metadata {
    pub packages: Vec<Package>,
    pub workspace_members: Vec<PackageId>,
    pub workspace_default_members: WorkspaceDefaultMembers,
    pub resolve: Option<Resolve>,
    pub workspace_root: camino::Utf8PathBuf,
    pub target_directory: camino::Utf8PathBuf,
    pub workspace_metadata: serde_json::Value,
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packages` | `Vec<Package>` | A list of all crates referenced by this crate (and the crate itself) |
| `workspace_members` | `Vec<PackageId>` | A list of all workspace members |
| `workspace_default_members` | `WorkspaceDefaultMembers` | The list of default workspace members<br><br>This is not available if running with a version of Cargo older than 1.71.<br><br>You can check whether it is available or missing using respectively<br>[`WorkspaceDefaultMembers::is_available`] and [`WorkspaceDefaultMembers::is_missing`]. |
| `resolve` | `Option<Resolve>` | Dependencies graph |
| `workspace_root` | `camino::Utf8PathBuf` | Workspace root |
| `target_directory` | `camino::Utf8PathBuf` | Build directory |
| `workspace_metadata` | `serde_json::Value` | The workspace-level metadata object. Null if non-existent. |
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn root_package(self: &Self) -> Option<&Package> { /* ... */ }
  ```
  Get the workspace's root package of this metadata instance.

- ```rust
  pub fn workspace_packages(self: &Self) -> Vec<&Package> { /* ... */ }
  ```
  Get the workspace packages.

- ```rust
  pub fn workspace_default_packages(self: &Self) -> Vec<&Package> { /* ... */ }
  ```
  Get the workspace default packages.

##### Trait Implementations

- **Sync**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Metadata { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Index**
  - ```rust
    fn index(self: &Self, idx: &''a PackageId) -> &<Self as >::Output { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DeserializeOwned**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Metadata) -> bool { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

### Struct `WorkspaceDefaultMembers`

**Attributes:**

- `#[serde(transparent)]`

A list of default workspace members.

See [`Metadata::workspace_default_members`].

It is only available if running a version of Cargo of 1.71 or newer.

# Panics

Dereferencing when running an older version of Cargo will panic.

```rust
pub struct WorkspaceDefaultMembers(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn is_available(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if the list of workspace default members is supported by

- ```rust
  pub fn is_missing(self: &Self) -> bool { /* ... */ }
  ```
  Return `false` if the list of workspace default members is supported by

##### Trait Implementations

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Default**
  - ```rust
    fn default() -> WorkspaceDefaultMembers { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **DeserializeOwned**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> WorkspaceDefaultMembers { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WorkspaceDefaultMembers) -> bool { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **Freeze**
### Struct `Resolve`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

A dependency graph

```rust
pub struct Resolve {
    pub nodes: Vec<Node>,
    pub root: Option<PackageId>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `nodes` | `Vec<Node>` | Nodes in a dependencies graph |
| `root` | `Option<PackageId>` | The crate for which the metadata was read. |

#### Implementations

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Resolve { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, idx: &''a PackageId) -> &<Self as >::Output { /* ... */ }
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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Resolve) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
### Struct `Node`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

A node in a dependencies graph

```rust
pub struct Node {
    pub id: PackageId,
    pub deps: Vec<NodeDep>,
    pub dependencies: Vec<PackageId>,
    pub features: Vec<String>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `PackageId` | An opaque identifier for a package |
| `deps` | `Vec<NodeDep>` | Dependencies in a structured format.<br><br>`deps` handles renamed dependencies whereas `dependencies` does not. |
| `dependencies` | `Vec<PackageId>` | List of opaque identifiers for this node's dependencies.<br>It doesn't support renamed dependencies. See `deps`. |
| `features` | `Vec<String>` | Features enabled on the crate |

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Node { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Node) -> bool { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DeserializeOwned**
### Struct `NodeDep`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

A dependency in a node

```rust
pub struct NodeDep {
    pub name: String,
    pub pkg: PackageId,
    pub dep_kinds: Vec<DepKindInfo>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the dependency's library target.<br>If the crate was renamed, it is the new name. |
| `pkg` | `PackageId` | Package ID (opaque unique identifier) |
| `dep_kinds` | `Vec<DepKindInfo>` | The kinds of dependencies.<br><br>This field was added in Rust 1.41. |

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeOwned**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NodeDep) -> bool { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NodeDep { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `DepKindInfo`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

Information about a dependency kind.

```rust
pub struct DepKindInfo {
    pub kind: DependencyKind,
    pub target: Option<dependency::Platform>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `kind` | `DependencyKind` | The kind of dependency. |
| `target` | `Option<dependency::Platform>` | The target platform for the dependency.<br><br>This is `None` if it is not a target dependency.<br><br>Use the [`Display`] trait to access the contents.<br><br>By default all platform dependencies are included in the resolve<br>graph. Use Cargo's `--filter-platform` flag if you only want to<br>include dependencies for a specific platform.<br><br>[`Display`]: std::fmt::Display |

#### Implementations

##### Trait Implementations

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **UnwindSafe**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Freeze**
- **DeserializeOwned**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DepKindInfo { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DepKindInfo) -> bool { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Sync**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `Package`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`

One or more crates described by a single `Cargo.toml`

Each [`target`][Package::targets] of a `Package` will be built as a crate.
For more information, see <https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html>.

```rust
pub struct Package {
    pub name: String,
    pub version: semver::Version,
    pub authors: Vec<String>,
    pub id: PackageId,
    pub source: Option<Source>,
    pub description: Option<String>,
    pub dependencies: Vec<Dependency>,
    pub license: Option<String>,
    pub license_file: Option<camino::Utf8PathBuf>,
    pub targets: Vec<Target>,
    pub features: std::collections::BTreeMap<String, Vec<String>>,
    pub manifest_path: camino::Utf8PathBuf,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub readme: Option<camino::Utf8PathBuf>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub edition: Edition,
    pub metadata: serde_json::Value,
    pub links: Option<String>,
    pub publish: Option<Vec<String>>,
    pub default_run: Option<String>,
    pub rust_version: Option<semver::Version>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The [`name` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field) as given in the `Cargo.toml` |
| `version` | `semver::Version` | The [`version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field) as specified in the `Cargo.toml` |
| `authors` | `Vec<String>` | The [`authors` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-authors-field) as specified in the `Cargo.toml` |
| `id` | `PackageId` | An opaque identifier for a package |
| `source` | `Option<Source>` | The source of the package, e.g.<br>crates.io or `None` for local projects. |
| `description` | `Option<String>` | The [`description` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-description-field) as specified in the `Cargo.toml` |
| `dependencies` | `Vec<Dependency>` | List of dependencies of this particular package |
| `license` | `Option<String>` | The [`license` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) as specified in the `Cargo.toml` |
| `license_file` | `Option<camino::Utf8PathBuf>` | The [`license-file` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) as specified in the `Cargo.toml`.<br>If the package is using a nonstandard license, this key may be specified instead of<br>`license`, and must point to a file relative to the manifest. |
| `targets` | `Vec<Target>` | Targets provided by the crate (lib, bin, example, test, ...) |
| `features` | `std::collections::BTreeMap<String, Vec<String>>` | Features provided by the crate, mapped to the features required by that feature. |
| `manifest_path` | `camino::Utf8PathBuf` | Path containing the `Cargo.toml` |
| `categories` | `Vec<String>` | The [`categories` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-categories-field) as specified in the `Cargo.toml` |
| `keywords` | `Vec<String>` | The [`keywords` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-keywords-field) as specified in the `Cargo.toml` |
| `readme` | `Option<camino::Utf8PathBuf>` | The [`readme` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field) as specified in the `Cargo.toml` |
| `repository` | `Option<String>` | The [`repository` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field) as specified in the `Cargo.toml` |
| `homepage` | `Option<String>` | The [`homepage` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-homepage-field) as specified in the `Cargo.toml`.<br><br>On versions of cargo before 1.49, this will always be [`None`]. |
| `documentation` | `Option<String>` | The [`documentation` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-documentation-field) as specified in the `Cargo.toml`.<br><br>On versions of cargo before 1.49, this will always be [`None`]. |
| `edition` | `Edition` | The default Rust edition for the package (either what's specified in the [`edition` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field)<br>or defaulting to [`Edition::E2015`]).<br><br>Beware that individual targets may specify their own edition in<br>[`Target::edition`]. |
| `metadata` | `serde_json::Value` | Contents of the free form [`package.metadata` section](https://doc.rust-lang.org/cargo/reference/manifest.html#the-metadata-table).<br><br>This contents can be serialized to a struct using serde:<br><br>```rust<br>use serde::Deserialize;<br>use serde_json::json;<br><br>#[derive(Debug, Deserialize)]<br>struct SomePackageMetadata {<br>    some_value: i32,<br>}<br><br>let value = json!({<br>    "some_value": 42,<br>});<br><br>let package_metadata: SomePackageMetadata = serde_json::from_value(value).unwrap();<br>assert_eq!(package_metadata.some_value, 42);<br><br>``` |
| `links` | `Option<String>` | The name of a native library the package is linking to. |
| `publish` | `Option<Vec<String>>` | List of registries to which this package may be published (derived from the [`publish` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field)).<br><br>Publishing is unrestricted if `None`, and forbidden if the `Vec` is empty.<br><br>This is always `None` if running with a version of Cargo older than 1.39. |
| `default_run` | `Option<String>` | The [`default-run` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-default-run-field) as given in the `Cargo.toml`<br>The default binary to run by `cargo run`.<br><br>This is always `None` if running with a version of Cargo older than 1.55. |
| `rust_version` | `Option<semver::Version>` | The [`rust-version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field) as specified in the `Cargo.toml`.<br>The minimum supported Rust version of this package.<br><br>This is always `None` if running with a version of Cargo older than 1.58. |

#### Implementations

##### Methods

- ```rust
  pub fn license_file(self: &Self) -> Option<Utf8PathBuf> { /* ... */ }
  ```
  Full path to the license file if one is present in the manifest

- ```rust
  pub fn readme(self: &Self) -> Option<Utf8PathBuf> { /* ... */ }
  ```
  Full path to the readme file if one is present in the manifest

##### Trait Implementations

- **UnwindSafe**
- **StructuralPartialEq**
- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Package) -> bool { /* ... */ }
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
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

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

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeOwned**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Package { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `Source`

**Attributes:**

- `#[serde(transparent)]`

The source of a package such as crates.io.

It is possible to inspect the `repr` field, if the need arises, but its
precise format is an implementation detail and is subject to change.

```rust
pub struct Source {
    pub repr: String,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `repr` | `String` | The underlying string representation of a source. |

#### Implementations

##### Methods

- ```rust
  pub fn is_crates_io(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the source is crates.io.

##### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Source) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Source { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **DeserializeOwned**
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
### Struct `Target`

**Attributes:**

- `#[<cfg_attr>(feature = "builder", derive(Builder))]`
- `#[<cfg_attr>(feature = "builder", builder(pattern = "owned", setter(into)))]`
- `#[non_exhaustive]`

A single target (lib, bin, example, ...) provided by a crate

```rust
pub struct Target {
    pub name: String,
    pub kind: Vec<TargetKind>,
    pub crate_types: Vec<CrateType>,
    pub required_features: Vec<String>,
    pub src_path: camino::Utf8PathBuf,
    pub edition: Edition,
    pub doctest: bool,
    pub test: bool,
    pub doc: bool,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | Name as given in the `Cargo.toml` or generated from the file name |
| `kind` | `Vec<TargetKind>` | Kind of target.<br><br>The possible values are `example`, `test`, `bench`, `custom-build` and<br>[Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):<br>`bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.<br><br>Other possible values may be added in the future. |
| `crate_types` | `Vec<CrateType>` | Similar to `kind`, but only reports the<br>[Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):<br>`bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.<br>Everything that's not a proc macro or a library of some kind is reported as "bin".<br><br>Other possible values may be added in the future. |
| `required_features` | `Vec<String>` | This target is built only if these features are enabled.<br>It doesn't apply to `lib` targets. |
| `src_path` | `camino::Utf8PathBuf` | Path to the main source file of the target |
| `edition` | `Edition` | Rust edition for this target |
| `doctest` | `bool` | Whether or not this target has doc tests enabled, and the target is<br>compatible with doc testing.<br><br>This is always `true` if running with a version of Cargo older than 1.37. |
| `test` | `bool` | Whether or not this target is tested by default by `cargo test`.<br><br>This is always `true` if running with a version of Cargo older than 1.47. |
| `doc` | `bool` | Whether or not this target is documented by `cargo doc`.<br><br>This is always `true` if running with a version of Cargo older than 1.50. |

#### Implementations

##### Methods

- ```rust
  pub fn is_kind(self: &Self, name: TargetKind) -> bool { /* ... */ }
  ```
  Return true if this target is of the given kind.

- ```rust
  pub fn is_lib(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_bin(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_example(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_test(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_bench(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_custom_build(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_proc_macro(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_cdylib(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_dylib(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_rlib(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

- ```rust
  pub fn is_staticlib(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this target is of kind `$kind`.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Target) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Target { /* ... */ }
    ```

- **UnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeOwned**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
### Enum `TargetKind`

**Attributes:**

- `#[non_exhaustive]`

Kind of target.

The possible values are `example`, `test`, `bench`, `custom-build` and
[Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
`bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.

Other possible values may be added in the future.

```rust
pub enum TargetKind {
    Bench,
    Bin,
    CustomBuild,
    CDyLib,
    DyLib,
    Example,
    Lib,
    ProcMacro,
    RLib,
    StaticLib,
    Test,
    Unknown(String),
}
```

#### Variants

##### `Bench`

`cargo bench` target

##### `Bin`

Binary executable target

##### `CustomBuild`

Custom build target

##### `CDyLib`

Dynamic system library target

##### `DyLib`

Dynamic Rust library target

##### `Example`

Example target

##### `Lib`

Rust library

##### `ProcMacro`

Procedural Macro

##### `RLib`

Rust library for use as an intermediate artifact

##### `StaticLib`

Static system library

##### `Test`

Test target

##### `Unknown`

Unknown type

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

#### Implementations

##### Trait Implementations

- **Send**
- **Unpin**
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TargetKind { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> std::result::Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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

  - ```rust
    fn from(value: &str) -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &TargetKind) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TargetKind) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &TargetKind) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Enum `CrateType`

**Attributes:**

- `#[non_exhaustive]`

Similar to `kind`, but only reports the
[Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):

`bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
Everything that's not a proc macro or a library of some kind is reported as "bin".

Other possible values may be added in the future.

```rust
pub enum CrateType {
    Bin,
    CDyLib,
    DyLib,
    Lib,
    ProcMacro,
    RLib,
    StaticLib,
    Unknown(String),
}
```

#### Variants

##### `Bin`

Binary executable target

##### `CDyLib`

Dynamic system library target

##### `DyLib`

Dynamic Rust library target

##### `Lib`

Rust library

##### `ProcMacro`

Procedural Macro

##### `RLib`

Rust library for use as an intermediate artifact

##### `StaticLib`

Static system library

##### `Unknown`

Unkown type

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: &str) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **DeserializeOwned**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> std::result::Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &CrateType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CrateType { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CrateType) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &CrateType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
### Enum `Edition`

**Attributes:**

- `#[non_exhaustive]`

The Rust edition

As of writing this comment rust editions 2024, 2027 and 2030 are not actually a thing yet but are parsed nonetheless for future proofing.

```rust
pub enum Edition {
    E2015,
    E2018,
    E2021,
    E2024,
    // Some variants omitted
}
```

#### Variants

##### `E2015`

Edition 2015

##### `E2018`

Edition 2018

##### `E2021`

Edition 2021

##### `E2024`

Edition 2024

*Note: Some variants have been omitted because they are private or hidden.*

#### Implementations

##### Methods

- ```rust
  pub fn as_str(self: &Self) -> &''static str { /* ... */ }
  ```
  Return the string representation of the edition

##### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Edition) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Edition) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Sync**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Edition) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Edition { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **DeserializeOwned**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
### Enum `CargoOpt`

Cargo features flags

```rust
pub enum CargoOpt {
    AllFeatures,
    NoDefaultFeatures,
    SomeFeatures(Vec<String>),
}
```

#### Variants

##### `AllFeatures`

Run cargo with `--features-all`

##### `NoDefaultFeatures`

Run cargo with `--no-default-features`

##### `SomeFeatures`

Run cargo with `--features <FEATURES>`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<String>` |  |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CargoOpt { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `MetadataCommand`

A builder for configuring `cargo metadata` invocation.

```rust
pub struct MetadataCommand {
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
  pub fn new() -> MetadataCommand { /* ... */ }
  ```
  Creates a default `cargo metadata` command, which will look for

- ```rust
  pub fn cargo_path</* synthetic */ impl Into<PathBuf>: Into<PathBuf>>(self: &mut Self, path: impl Into<PathBuf>) -> &mut MetadataCommand { /* ... */ }
  ```
  Path to `cargo` executable.  If not set, this will use the

- ```rust
  pub fn manifest_path</* synthetic */ impl Into<PathBuf>: Into<PathBuf>>(self: &mut Self, path: impl Into<PathBuf>) -> &mut MetadataCommand { /* ... */ }
  ```
  Path to `Cargo.toml`

- ```rust
  pub fn current_dir</* synthetic */ impl Into<PathBuf>: Into<PathBuf>>(self: &mut Self, path: impl Into<PathBuf>) -> &mut MetadataCommand { /* ... */ }
  ```
  Current directory of the `cargo metadata` process.

- ```rust
  pub fn no_deps(self: &mut Self) -> &mut MetadataCommand { /* ... */ }
  ```
  Output information only about workspace members and don't fetch dependencies.

- ```rust
  pub fn features(self: &mut Self, features: CargoOpt) -> &mut MetadataCommand { /* ... */ }
  ```
  Which features to include.

- ```rust
  pub fn other_options</* synthetic */ impl Into<Vec<String>>: Into<Vec<String>>>(self: &mut Self, options: impl Into<Vec<String>>) -> &mut MetadataCommand { /* ... */ }
  ```
  Arbitrary command line flags to pass to `cargo`.  These will be added

- ```rust
  pub fn env<K: Into<OsString>, V: Into<OsString>>(self: &mut Self, key: K, val: V) -> &mut MetadataCommand { /* ... */ }
  ```
  Arbitrary environment variables to set when running `cargo`.  These will be merged into

- ```rust
  pub fn verbose(self: &mut Self, verbose: bool) -> &mut MetadataCommand { /* ... */ }
  ```
  Set whether to show stderr

- ```rust
  pub fn cargo_command(self: &Self) -> Command { /* ... */ }
  ```
  Builds a command for `cargo metadata`.  This is the first

- ```rust
  pub fn parse<T: AsRef<str>>(data: T) -> Result<Metadata> { /* ... */ }
  ```
  Parses `cargo metadata` output.  `data` must have been

- ```rust
  pub fn exec(self: &Self) -> Result<Metadata> { /* ... */ }
  ```
  Runs configured `cargo metadata` and returns parsed `Metadata`.

##### Trait Implementations

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> MetadataCommand { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MetadataCommand { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Re-exports

### Re-export `camino`

```rust
pub use camino;
```

### Re-export `semver`

```rust
pub use semver;
```

### Re-export `Dependency`

```rust
pub use dependency::Dependency;
```

### Re-export `DependencyKind`

```rust
pub use dependency::DependencyKind;
```

### Re-export `Error`

```rust
pub use errors::Error;
```

### Re-export `Result`

```rust
pub use errors::Result;
```

### Re-export `parse_messages`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use messages::parse_messages;
```

### Re-export `Artifact`

```rust
pub use messages::Artifact;
```

### Re-export `ArtifactDebuginfo`

```rust
pub use messages::ArtifactDebuginfo;
```

### Re-export `ArtifactProfile`

```rust
pub use messages::ArtifactProfile;
```

### Re-export `BuildFinished`

```rust
pub use messages::BuildFinished;
```

### Re-export `BuildScript`

```rust
pub use messages::BuildScript;
```

### Re-export `CompilerMessage`

```rust
pub use messages::CompilerMessage;
```

### Re-export `Message`

```rust
pub use messages::Message;
```

### Re-export `MessageIter`

```rust
pub use messages::MessageIter;
```

