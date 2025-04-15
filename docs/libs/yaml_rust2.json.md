# Crate Documentation

**Version:** 0.10.1

**Format Version:** 43

# Module `yaml_rust2`

YAML 1.2 implementation in pure Rust.

# Usage

This crate is [on github](https://github.com/Ethiraric/yaml-rust2) and can be used by adding
`yaml-rust2` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
yaml-rust2 = "0.10.1"
```

# Examples
Parse a string into `Vec<Yaml>` and then serialize it as a YAML string.

```
use yaml_rust2::{YamlLoader, YamlEmitter};

let docs = YamlLoader::load_from_str("[1, 2, 3]").unwrap();
let doc = &docs[0]; // select the first YAML document
assert_eq!(doc[0].as_i64().unwrap(), 1); // access elements by index

let mut out_str = String::new();
let mut emitter = YamlEmitter::new(&mut out_str);
emitter.dump(doc).unwrap(); // dump the YAML object to a String

```

# Features
**Note:** With all features disabled, this crate's MSRV is `1.65.0`.

#### `encoding` (_enabled by default_)
Enables encoding-aware decoding of Yaml documents.

The MSRV for this feature is `1.70.0`.

#### `debug_prints`
Enables the `debug` module and usage of debug prints in the scanner and the parser. Do not
enable if you are consuming the crate rather than working on it as this can significantly
decrease performance.

The MSRV for this feature is `1.70.0`.

## Modules

## Module `emitter`

YAML serialization helpers.

```rust
pub mod emitter { /* ... */ }
```

### Types

#### Enum `EmitError`

An error when emitting YAML.

```rust
pub enum EmitError {
    FmtError(fmt::Error),
}
```

##### Variants

###### `FmtError`

A formatting error.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `fmt::Error` |  |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **UnwindSafe**
- **Copy**
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

  - ```rust
    fn from(f: fmt::Error) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EmitError { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Error**
  - ```rust
    fn cause(self: &Self) -> Option<&dyn Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `YamlEmitter`

**Attributes:**

- `#[allow(clippy::module_name_repetitions)]`

The YAML serializer.

```
# use yaml_rust2::{YamlLoader, YamlEmitter};
let input_string = "a: b\nc: d";
let yaml = YamlLoader::load_from_str(input_string).unwrap();

let mut output = String::new();
YamlEmitter::new(&mut output).dump(&yaml[0]).unwrap();

assert_eq!(output, r#"---
a: b
c: d"#);
```

```rust
pub struct YamlEmitter<''a> {
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
  pub fn new(writer: &''a mut dyn fmt::Write) -> YamlEmitter<''a> { /* ... */ }
  ```
  Create a new emitter serializing into `writer`.

- ```rust
  pub fn compact(self: &mut Self, compact: bool) { /* ... */ }
  ```
  Set 'compact inline notation' on or off, as described for block

- ```rust
  pub fn is_compact(self: &Self) -> bool { /* ... */ }
  ```
  Determine if this emitter is using 'compact inline notation'.

- ```rust
  pub fn multiline_strings(self: &mut Self, multiline_strings: bool) { /* ... */ }
  ```
  Render strings containing multiple lines in [literal style].

- ```rust
  pub fn is_multiline_strings(self: &Self) -> bool { /* ... */ }
  ```
  Determine if this emitter will emit multiline strings when appropriate.

- ```rust
  pub fn dump(self: &mut Self, doc: &Yaml) -> EmitResult { /* ... */ }
  ```
  Dump Yaml to an output stream.

###### Trait Implementations

- **Send**
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

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
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
#### Type Alias `EmitResult`

A convenience alias for emitter functions that may fail without returning a value.

```rust
pub type EmitResult = Result<(), EmitError>;
```

## Module `parser`

Home to the YAML Parser.

The parser takes input from the [`crate::scanner::Scanner`], performs final checks for YAML
compliance, and emits a stream of tokens that can be used by the [`crate::YamlLoader`] to
construct the [`crate::Yaml`] object.

```rust
pub mod parser { /* ... */ }
```

### Types

#### Enum `Event`

An event generated by the YAML parser.

Events are used in the low-level event-based API (push parser). The API entrypoint is the
[`EventReceiver`] trait.

```rust
pub enum Event {
    Nothing,
    StreamStart,
    StreamEnd,
    DocumentStart,
    DocumentEnd,
    Alias(usize),
    Scalar(String, crate::scanner::TScalarStyle, usize, Option<Tag>),
    SequenceStart(usize, Option<Tag>),
    SequenceEnd,
    MappingStart(usize, Option<Tag>),
    MappingEnd,
}
```

##### Variants

###### `Nothing`

Reserved for internal use.

###### `StreamStart`

Event generated at the very beginning of parsing.

###### `StreamEnd`

Last event that will be generated by the parser. Signals EOF.

###### `DocumentStart`

The YAML start document directive (`---`).

###### `DocumentEnd`

The YAML end document directive (`...`).

###### `Alias`

A YAML Alias.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` | The anchor ID the alias refers to. |

###### `Scalar`

Value, style, anchor id, tag

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |
| 1 | `crate::scanner::TScalarStyle` |  |
| 2 | `usize` |  |
| 3 | `Option<Tag>` |  |

###### `SequenceStart`

The start of a YAML sequence (array).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` | The anchor ID of the start of the sequence. |
| 1 | `Option<Tag>` | An optional tag |

###### `SequenceEnd`

The end of a YAML sequence (array).

###### `MappingStart`

The start of a YAML mapping (object, hash).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` | The anchor ID of the start of the mapping. |
| 1 | `Option<Tag>` | An optional tag |

###### `MappingEnd`

The end of a YAML mapping (object, hash).

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Event { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Event) -> bool { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **RefUnwindSafe**
#### Struct `Tag`

A YAML tag.

```rust
pub struct Tag {
    pub handle: String,
    pub suffix: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `handle` | `String` | Handle of the tag (`!` included). |
| `suffix` | `String` | The suffix of the tag. |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Tag { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Tag) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Unpin**
- **Sync**
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

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

#### Struct `Parser`

A YAML parser.

```rust
pub struct Parser<T> {
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
  pub fn new_from_str(value: &''a str) -> Self { /* ... */ }
  ```
  Create a new instance of a parser from a &str.

- ```rust
  pub fn new(src: T) -> Parser<T> { /* ... */ }
  ```
  Create a new instance of a parser from the given input of characters.

- ```rust
  pub fn keep_tags(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Whether to keep tags across multiple documents when parsing.

- ```rust
  pub fn peek(self: &mut Self) -> Result<&(Event, Marker), ScanError> { /* ... */ }
  ```
  Try to load the next event and return it, but do not consuming it from `self`.

- ```rust
  pub fn next_token(self: &mut Self) -> ParseResult { /* ... */ }
  ```
  Try to load the next event and return it, consuming it from `self`.

- ```rust
  pub fn load<R: MarkedEventReceiver>(self: &mut Self, recv: &mut R, multi: bool) -> Result<(), ScanError> { /* ... */ }
  ```
  Load the YAML from the stream in `self`, pushing events into `recv`.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Type Alias `ParseResult`

A convenience alias for a `Result` of a parser event.

```rust
pub type ParseResult = Result<(Event, crate::scanner::Marker), crate::scanner::ScanError>;
```

### Traits

#### Trait `EventReceiver`

Trait to be implemented in order to use the low-level parsing API.

The low-level parsing API is event-based (a push parser), calling [`EventReceiver::on_event`]
for each YAML [`Event`] that occurs.
The [`EventReceiver`] trait only receives events. In order to receive both events and their
location in the source, use [`MarkedEventReceiver`]. Note that [`EventReceiver`]s implement
[`MarkedEventReceiver`] automatically.

# Event hierarchy
The event stream starts with an [`Event::StreamStart`] event followed by an
[`Event::DocumentStart`] event. If the YAML document starts with a mapping (an object), an
[`Event::MappingStart`] event is emitted. If it starts with a sequence (an array), an
[`Event::SequenceStart`] event is emitted. Otherwise, an [`Event::Scalar`] event is emitted.

In a mapping, key-values are sent as consecutive events. The first event after an
[`Event::MappingStart`] will be the key, and following its value. If the mapping contains no
sub-mapping or sub-sequence, then even events (starting from 0) will always be keys and odd
ones will always be values. The mapping ends when an [`Event::MappingEnd`] event is received.

In a sequence, values are sent consecutively until the [`Event::SequenceEnd`] event.

If a value is a sub-mapping or a sub-sequence, an [`Event::MappingStart`] or
[`Event::SequenceStart`] event will be sent respectively. Following events until the associated
[`Event::MappingStart`] or [`Event::SequenceEnd`] (beware of nested mappings or sequences) will
be part of the value and not another key-value pair or element in the sequence.

For instance, the following yaml:
```yaml
a: b
c:
  d: e
f:
  - g
  - h
```
will emit (indented and commented for lisibility):
```text
StreamStart, DocumentStart, MappingStart,
  Scalar("a", ..), Scalar("b", ..)
  Scalar("c", ..), MappingStart, Scalar("d", ..), Scalar("e", ..), MappingEnd,
  Scalar("f", ..), SequenceStart, Scalar("g", ..), Scalar("h", ..), SequenceEnd,
MappingEnd, DocumentEnd, StreamEnd
```

# Example
```
# use yaml_rust2::parser::{Event, EventReceiver, Parser};
#
/// Sink of events. Collects them into an array.
struct EventSink {
    events: Vec<Event>,
}

/// Implement `on_event`, pushing into `self.events`.
impl EventReceiver for EventSink {
    fn on_event(&mut self, ev: Event) {
        self.events.push(ev);
    }
}

/// Load events from a yaml string.
fn str_to_events(yaml: &str) -> Vec<Event> {
    let mut sink = EventSink { events: Vec::new() };
    let mut parser = Parser::new_from_str(yaml);
    // Load events using our sink as the receiver.
    parser.load(&mut sink, true).unwrap();
    sink.events
}
```

```rust
pub trait EventReceiver {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `on_event`: Handler called for each YAML event that is emitted by the parser.

#### Trait `MarkedEventReceiver`

Trait to be implemented for using the low-level parsing API.

Functionally similar to [`EventReceiver`], but receives a [`Marker`] as well as the event.

```rust
pub trait MarkedEventReceiver {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `on_event`: Handler called for each event that occurs.

##### Implementations

This trait is implemented for the following types:

- `R` with <R: EventReceiver>
- `YamlLoader`

## Module `scanner`

**Attributes:**

- `#[allow(clippy::cast_possible_wrap)]`
- `#[allow(clippy::cast_sign_loss)]`

Home to the YAML Scanner.

The scanner is the lowest-level parsing utility. It is the lexer / tokenizer, reading input a
character at a time and emitting tokens that can later be interpreted by the [`crate::parser`]
to check for more context and validity.

Due to the grammar of YAML, the scanner has to have some context and is not error-free.

```rust
pub mod scanner { /* ... */ }
```

### Types

#### Enum `TEncoding`

The encoding of the input. Currently, only UTF-8 is supported.

```rust
pub enum TEncoding {
    Utf8,
}
```

##### Variants

###### `Utf8`

UTF-8 encoding.

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TEncoding { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TEncoding) -> bool { /* ... */ }
    ```

- **Sync**
- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
#### Enum `TScalarStyle`

The style as which the scalar was written in the YAML document.

```rust
pub enum TScalarStyle {
    Plain,
    SingleQuoted,
    DoubleQuoted,
    Literal,
    Folded,
}
```

##### Variants

###### `Plain`

A YAML plain scalar.

###### `SingleQuoted`

A YAML single quoted scalar.

###### `DoubleQuoted`

A YAML double quoted scalar.

###### `Literal`

A YAML literal block (`|` block).

###### `Folded`

A YAML folded block (`>` block).

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TScalarStyle { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
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

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TScalarStyle) -> bool { /* ... */ }
    ```

- **Freeze**
#### Struct `Marker`

A location in a yaml document.

```rust
pub struct Marker {
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
  pub fn index(self: &Self) -> usize { /* ... */ }
  ```
  Return the index (in bytes) of the marker in the source.

- ```rust
  pub fn line(self: &Self) -> usize { /* ... */ }
  ```
  Return the line of the marker in the source.

- ```rust
  pub fn col(self: &Self) -> usize { /* ... */ }
  ```
  Return the column of the marker in the source.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Copy**
- **Eq**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **UnwindSafe**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Marker) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Marker { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
#### Struct `ScanError`

An error that occurred while scanning.

```rust
pub struct ScanError {
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
  pub fn new(loc: Marker, info: &str) -> ScanError { /* ... */ }
  ```
  Create a new error from a location and an error string.

- ```rust
  pub fn new_string(loc: Marker, info: String) -> ScanError { /* ... */ }
  ```
  Create a new error from a location and an error string.

- ```rust
  pub fn marker(self: &Self) -> &Marker { /* ... */ }
  ```
  Return the marker pointing to the error in the source.

- ```rust
  pub fn info(self: &Self) -> &str { /* ... */ }
  ```
  Return the information string describing the error that happened.

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ScanError) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn cause(self: &Self) -> Option<&dyn Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ScanError { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
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

#### Enum `TokenType`

The contents of a scanner token.

```rust
pub enum TokenType {
    StreamStart(TEncoding),
    StreamEnd,
    VersionDirective(u32, u32),
    TagDirective(String, String),
    DocumentStart,
    DocumentEnd,
    BlockSequenceStart,
    BlockMappingStart,
    BlockEnd,
    FlowSequenceStart,
    FlowSequenceEnd,
    FlowMappingStart,
    FlowMappingEnd,
    BlockEntry,
    FlowEntry,
    Key,
    Value,
    Alias(String),
    Anchor(String),
    Tag(String, String),
    Scalar(TScalarStyle, String),
}
```

##### Variants

###### `StreamStart`

The start of the stream. Sent first, before even [`TokenType::DocumentStart`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TEncoding` |  |

###### `StreamEnd`

The end of the stream, EOF.

###### `VersionDirective`

A YAML version directive.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` | Major |
| 1 | `u32` | Minor |

###### `TagDirective`

A YAML tag directive (e.g.: `!!str`, `!foo!bar`, ...).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` | Handle |
| 1 | `String` | Prefix |

###### `DocumentStart`

The start of a YAML document (`---`).

###### `DocumentEnd`

The end of a YAML document (`...`).

###### `BlockSequenceStart`

The start of a sequence block.

Sequence blocks are arrays starting with a `-`.

###### `BlockMappingStart`

The start of a sequence mapping.

Sequence mappings are "dictionaries" with "key: value" entries.

###### `BlockEnd`

End of the corresponding `BlockSequenceStart` or `BlockMappingStart`.

###### `FlowSequenceStart`

Start of an inline array (`[ a, b ]`).

###### `FlowSequenceEnd`

End of an inline array.

###### `FlowMappingStart`

Start of an inline mapping (`{ a: b, c: d }`).

###### `FlowMappingEnd`

End of an inline mapping.

###### `BlockEntry`

An entry in a block sequence (c.f.: [`TokenType::BlockSequenceStart`]).

###### `FlowEntry`

An entry in a flow sequence (c.f.: [`TokenType::FlowSequenceStart`]).

###### `Key`

A key in a mapping.

###### `Value`

A value in a mapping.

###### `Alias`

A reference to an anchor.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Anchor`

A YAML anchor (`&`/`*`).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Tag`

A YAML tag (starting with bangs `!`).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` | The handle of the tag. |
| 1 | `String` | The suffix of the tag. |

###### `Scalar`

A regular YAML scalar.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TScalarStyle` |  |
| 1 | `String` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TokenType) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TokenType { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Freeze**
#### Struct `Token`

A scanner token.

```rust
pub struct Token(pub Marker, pub TokenType);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Marker` |  |
| 1 | `TokenType` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Token) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Token { /* ... */ }
    ```

#### Struct `Scanner`

**Attributes:**

- `#[allow(clippy::struct_excessive_bools)]`

The YAML scanner.

This corresponds to the low-level interface when reading YAML. The scanner emits token as they
are read (akin to a lexer), but it also holds sufficient context to be able to disambiguate
some of the constructs. It has understanding of indentation and whitespace and is able to
generate error messages for some invalid YAML constructs.

It is however not a full parser and needs [`crate::parser::Parser`] to fully detect invalid
YAML documents.

```rust
pub struct Scanner<T> {
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
  pub fn new(rdr: T) -> Scanner<T> { /* ... */ }
  ```
  Creates the YAML tokenizer.

- ```rust
  pub fn get_error(self: &Self) -> Option<ScanError> { /* ... */ }
  ```
  Get a copy of the last error that was encountered, if any.

- ```rust
  pub fn stream_started(self: &Self) -> bool { /* ... */ }
  ```
  Return whether the [`TokenType::StreamStart`] event has been emitted.

- ```rust
  pub fn stream_ended(self: &Self) -> bool { /* ... */ }
  ```
  Return whether the [`TokenType::StreamEnd`] event has been emitted.

- ```rust
  pub fn mark(self: &Self) -> Marker { /* ... */ }
  ```
  Get the current position in the input stream.

- ```rust
  pub fn fetch_next_token(self: &mut Self) -> ScanResult { /* ... */ }
  ```
  Fetch the next token in the stream.

- ```rust
  pub fn next_token(self: &mut Self) -> Result<Option<Token>, ScanError> { /* ... */ }
  ```
  Return the next token in the stream.

- ```rust
  pub fn fetch_more_tokens(self: &mut Self) -> ScanResult { /* ... */ }
  ```
  Fetch tokens from the token stream.

###### Trait Implementations

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Token> { /* ... */ }
    ```

#### Type Alias `ScanResult`

A convenience alias for scanner functions that may fail without returning a value.

```rust
pub type ScanResult = Result<(), ScanError>;
```

#### Enum `Chomping`

Chomping, how final line breaks and trailing empty lines are interpreted.

See YAML spec 8.1.1.2.

```rust
pub enum Chomping {
    Strip,
    Clip,
    Keep,
}
```

##### Variants

###### `Strip`

The final line break and any trailing empty lines are excluded.

###### `Clip`

The final line break is preserved, but trailing empty lines are excluded.

###### `Keep`

The final line break and trailing empty lines are included.

##### Implementations

###### Trait Implementations

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Chomping) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
## Module `yaml`

**Attributes:**

- `#[allow(clippy::module_name_repetitions)]`

YAML objects manipulation utilities.

```rust
pub mod yaml { /* ... */ }
```

### Types

#### Enum `Yaml`

A YAML node is stored as this `Yaml` enumeration, which provides an easy way to
access your YAML document.

# Examples

```
use yaml_rust2::Yaml;
let foo = Yaml::from_str("-123"); // convert the string to the appropriate YAML type
assert_eq!(foo.as_i64().unwrap(), -123);

// iterate over an Array
let vec = Yaml::Array(vec![Yaml::Integer(1), Yaml::Integer(2)]);
for v in vec.as_vec().unwrap() {
    assert!(v.as_i64().is_some());
}
```

```rust
pub enum Yaml {
    Real(String),
    Integer(i64),
    String(String),
    Boolean(bool),
    Array(Array),
    Hash(Hash),
    Alias(usize),
    Null,
    BadValue,
}
```

##### Variants

###### `Real`

Float types are stored as String and parsed on demand.
Note that `f64` does NOT implement Eq trait and can NOT be stored in `BTreeMap`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Integer`

YAML int is stored as i64.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `String`

YAML scalar.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Boolean`

YAML bool, e.g. `true` or `false`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

###### `Array`

YAML array, can be accessed as a [`Vec`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Array` |  |

###### `Hash`

YAML hash, can be accessed as a [`LinkedHashMap`].

Insertion order will match the order of insertion into the map.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Hash` |  |

###### `Alias`

Alias, not fully supported yet.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `Null`

YAML null, e.g. `null` or `~`.

###### `BadValue`

Accessing a nonexistent node via the Index trait returns `BadValue`. This
simplifies error handling in the calling code. Invalid type conversion also
returns `BadValue`.

##### Implementations

###### Methods

- ```rust
  pub fn as_bool(self: &Self) -> Option<bool> { /* ... */ }
  ```
  Get a copy of the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn as_i64(self: &Self) -> Option<i64> { /* ... */ }
  ```
  Get a copy of the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn as_str(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Get a reference to the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn as_hash(self: &Self) -> Option<&Hash> { /* ... */ }
  ```
  Get a reference to the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn as_vec(self: &Self) -> Option<&Array> { /* ... */ }
  ```
  Get a reference to the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn as_mut_hash(self: &mut Self) -> Option<&mut Hash> { /* ... */ }
  ```
  Get a mutable reference to the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn as_mut_vec(self: &mut Self) -> Option<&mut Array> { /* ... */ }
  ```
  Get a mutable reference to the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn into_bool(self: Self) -> Option<bool> { /* ... */ }
  ```
  Get the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn into_i64(self: Self) -> Option<i64> { /* ... */ }
  ```
  Get the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn into_string(self: Self) -> Option<String> { /* ... */ }
  ```
  Get the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn into_hash(self: Self) -> Option<Hash> { /* ... */ }
  ```
  Get the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn into_vec(self: Self) -> Option<Array> { /* ... */ }
  ```
  Get the inner object in the YAML enum if it is a `$t`.

- ```rust
  pub fn is_null(self: &Self) -> bool { /* ... */ }
  ```
  Return whether `self` is a [`Yaml::Null`] node.

- ```rust
  pub fn is_badvalue(self: &Self) -> bool { /* ... */ }
  ```
  Return whether `self` is a [`Yaml::BadValue`] node.

- ```rust
  pub fn is_array(self: &Self) -> bool { /* ... */ }
  ```
  Return whether `self` is a [`Yaml::Array`] node.

- ```rust
  pub fn is_hash(self: &Self) -> bool { /* ... */ }
  ```
  Return whether `self` is a [`Yaml::Hash`] node.

- ```rust
  pub fn as_f64(self: &Self) -> Option<f64> { /* ... */ }
  ```
  Return the `f64` value contained in this YAML node.

- ```rust
  pub fn into_f64(self: Self) -> Option<f64> { /* ... */ }
  ```
  Return the `f64` value contained in this YAML node.

- ```rust
  pub fn or(self: Self, other: Self) -> Self { /* ... */ }
  ```
  If a value is null or otherwise bad (see variants), consume it and

- ```rust
  pub fn borrowed_or<''a>(self: &''a Self, other: &''a Self) -> &''a Self { /* ... */ }
  ```
  See `or` for behavior. This performs the same operations, but with

- ```rust
  pub fn from_str(v: &str) -> Yaml { /* ... */ }
  ```
  Convert a string to a [`Yaml`] node.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Yaml) -> $crate::cmp::Ordering { /* ... */ }
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

- **Eq**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, idx: &''a str) -> &mut Yaml { /* ... */ }
    ```
    Perform indexing if `self` is a mapping.

  - ```rust
    fn index_mut(self: &mut Self, idx: usize) -> &mut Yaml { /* ... */ }
    ```
    Perform indexing if `self` is a sequence or a mapping.

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
    fn clone(self: &Self) -> Yaml { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Yaml) -> bool { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```
    Extract the [`Array`] from `self` and iterate over it.

- **UnwindSafe**
- **Send**
- **Index**
  - ```rust
    fn index(self: &Self, idx: &''a str) -> &Yaml { /* ... */ }
    ```
    Perform indexing if `self` is a mapping.

  - ```rust
    fn index(self: &Self, idx: usize) -> &Yaml { /* ... */ }
    ```
    Perform indexing if `self` is a sequence or a mapping.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Yaml) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **StructuralPartialEq**
#### Type Alias `Array`

The type contained in the `Yaml::Array` variant. This corresponds to YAML sequences.

```rust
pub type Array = Vec<Yaml>;
```

#### Type Alias `Hash`

The type contained in the `Yaml::Hash` variant. This corresponds to YAML mappings.

```rust
pub type Hash = hashlink::LinkedHashMap<Yaml, Yaml>;
```

#### Struct `YamlLoader`

Main structure for quickly parsing YAML.

See [`YamlLoader::load_from_str`].

```rust
pub struct YamlLoader {
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
  pub fn load_from_str(source: &str) -> Result<Vec<Yaml>, ScanError> { /* ... */ }
  ```
  Load the given string as a set of YAML documents.

- ```rust
  pub fn load_from_iter<I: Iterator<Item = char>>(source: I) -> Result<Vec<Yaml>, ScanError> { /* ... */ }
  ```
  Load the contents of the given iterator as a set of YAML documents.

- ```rust
  pub fn load_from_parser<I: Iterator<Item = char>>(parser: &mut Parser<I>) -> Result<Vec<Yaml>, ScanError> { /* ... */ }
  ```
  Load the contents from the specified Parser as a set of YAML documents.

- ```rust
  pub fn documents(self: &Self) -> &[Yaml] { /* ... */ }
  ```
  Return a reference to the parsed Yaml documents.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> YamlLoader { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **MarkedEventReceiver**
  - ```rust
    fn on_event(self: &mut Self, ev: Event, mark: Marker) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Enum `LoadError`

An error that happened when loading a YAML document.

```rust
pub enum LoadError {
    IO(std::io::Error),
    Scan(crate::scanner::ScanError),
    Decode(std::borrow::Cow<''static, str>),
}
```

##### Variants

###### `IO`

An I/O error.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::io::Error` |  |

###### `Scan`

An error within the scanner. This indicates a malformed YAML input.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::scanner::ScanError` |  |

###### `Decode`

A decoding error (e.g.: Invalid UTF-8).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::borrow::Cow<''static, str>` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(error: std::io::Error) -> Self { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Type Alias `YAMLDecodingTrapFn`

**Attributes:**

- `#[<cfg>(feature = "encoding")]`

The signature of the function to call when using [`YAMLDecodingTrap::Call`].

The arguments are as follows:
 * `malformation_length`: The length of the sequence the decoder failed to decode.
 * `bytes_read_after_malformation`: The number of lookahead bytes the decoder consumed after
   the malformation.
 * `input_at_malformation`: What the input buffer is at the malformation.
   This is the buffer starting at the malformation. The first `malformation_length` bytes are
   the problematic sequence. The following `bytes_read_after_malformation` are already stored
   in the decoder and will not be re-fed.
 * `output`: The output string.

The function must modify `output` as it feels is best. For instance, one could recreate the
behavior of [`YAMLDecodingTrap::Ignore`] with an empty function, [`YAMLDecodingTrap::Replace`]
by pushing a `\u{FFFD}` into `output` and [`YAMLDecodingTrap::Strict`] by returning
[`ControlFlow::Break`].

# Returns
The function must return [`ControlFlow::Continue`] if decoding may continue or
[`ControlFlow::Break`] if decoding must be aborted. An optional error string may be supplied.

```rust
pub type YAMLDecodingTrapFn = fn(u8, u8, &[u8], &mut String) -> std::ops::ControlFlow<std::borrow::Cow<''static, str>>;
```

#### Enum `YAMLDecodingTrap`

**Attributes:**

- `#[<cfg>(feature = "encoding")]`

The behavior [`YamlDecoder`] must have when an decoding error occurs.

```rust
pub enum YAMLDecodingTrap {
    Ignore,
    Strict,
    Replace,
    Call(YAMLDecodingTrapFn),
}
```

##### Variants

###### `Ignore`

Ignore the offending bytes, remove them from the output.

###### `Strict`

Error out.

###### `Replace`

Replace them with the Unicode REPLACEMENT CHARACTER.

###### `Call`

Call the user-supplied function upon decoding malformation.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `YAMLDecodingTrapFn` |  |

##### Implementations

###### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &YAMLDecodingTrap) -> bool { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> YAMLDecodingTrap { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Freeze**
#### Struct `YamlDecoder`

**Attributes:**

- `#[<cfg>(feature = "encoding")]`

`YamlDecoder` is a `YamlLoader` builder that allows you to supply your own encoding error trap.
For example, to read a YAML file while ignoring Unicode decoding errors you can set the
`encoding_trap` to `encoding::DecoderTrap::Ignore`.
```rust
use yaml_rust2::yaml::{YamlDecoder, YAMLDecodingTrap};

let string = b"---
a\xa9: 1
b: 2.2
c: [1, 2]
";
let out = YamlDecoder::read(string as &[u8])
    .encoding_trap(YAMLDecodingTrap::Ignore)
    .decode()
    .unwrap();
```

```rust
pub struct YamlDecoder<T: std::io::Read> {
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
  pub fn read(source: T) -> YamlDecoder<T> { /* ... */ }
  ```
  Create a `YamlDecoder` decoding the given source.

- ```rust
  pub fn encoding_trap(self: &mut Self, trap: YAMLDecodingTrap) -> &mut Self { /* ... */ }
  ```
  Set the behavior of the decoder when the encoding is invalid.

- ```rust
  pub fn decode(self: &mut Self) -> Result<Vec<Yaml>, LoadError> { /* ... */ }
  ```
  Run the decode operation with the source and trap the `YamlDecoder` was built with.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `YamlIter`

An iterator over a [`Yaml`] node.

```rust
pub struct YamlIter {
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
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Yaml> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Re-exports

### Re-export `EmitError`

```rust
pub use crate::emitter::EmitError;
```

### Re-export `YamlEmitter`

```rust
pub use crate::emitter::YamlEmitter;
```

### Re-export `Event`

```rust
pub use crate::parser::Event;
```

### Re-export `ScanError`

```rust
pub use crate::scanner::ScanError;
```

### Re-export `Yaml`

```rust
pub use crate::yaml::Yaml;
```

### Re-export `YamlLoader`

```rust
pub use crate::yaml::YamlLoader;
```

