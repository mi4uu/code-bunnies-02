# Crate Documentation

**Version:** 0.7.6

**Format Version:** 43

# Module `winnow`

> winnow, making parsing a breeze

`winnow` is a parser combinator library

Quick links:
- [List of combinators][crate::combinator]
- [Tutorial][_tutorial::chapter_0]
- [Special Topics][_topic]
- [Discussions](https://github.com/winnow-rs/winnow/discussions)
- [CHANGELOG](https://github.com/winnow-rs/winnow/blob/v0.7.6/CHANGELOG.md) (includes major version migration
  guides)

## Aspirations

`winnow` aims to be your "do everything" parser, much like people treat regular expressions.

In roughly priority order:
1. Support writing parser declaratively while not getting in the way of imperative-style
   parsing when needed, working as an open-ended toolbox rather than a close-ended framework.
2. Flexible enough to be used for any application, including parsing binary data, strings, or
   separate lexing and parsing phases
3. Zero-cost abstractions, making it easy to write high performance parsers
4. Easy to use, making it trivial for one-off uses

In addition:
- Resilient maintainership, including
  - Willing to break compatibility rather than batching up breaking changes in large releases
  - Leverage feature flags to keep one active branch
- We will support the last 6 months of rust releases (MSRV, currently 1.64.0)

See also [Special Topic: Why winnow?][crate::_topic::why]

## Example

Run
```console
$ cargo add winnow
```

Then use it to parse:
```rust
# #[cfg(feature = "alloc")] {
use winnow::combinator::seq;
use winnow::prelude::*;
use winnow::token::take_while;
use winnow::Result;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl std::str::FromStr for Color {
    // The error must be owned
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex_color.parse(s).map_err(|e| e.to_string())
    }
}

pub(crate) fn hex_color(input: &mut &str) -> Result<Color> {
    seq!(Color {
        _: '#',
        red: hex_primary,
        green: hex_primary,
        blue: hex_primary
    })
    .parse_next(input)
}

fn hex_primary(input: &mut &str) -> Result<u8> {
    take_while(2, |c: char| c.is_ascii_hexdigit())
        .try_map(|input| u8::from_str_radix(input, 16))
        .parse_next(input)
}
# }
```

See also the [Tutorial][_tutorial::chapter_0] and [Special Topics][_topic]

## Modules

## Module `error`

**Attributes:**

- `#[macro_use]`

# Error management

Errors are designed with multiple needs in mind:
- Accumulate more [context][Parser::context] as the error goes up the parser chain
- Distinguish between [recoverable errors,
  unrecoverable errors, and more data is needed][ErrMode]
- Have a very low overhead, as errors are often discarded by the calling parser (examples: `repeat`, `alt`)
- Can be modified according to the user's needs, because some languages need a lot more information
- Help thread-through the [stream][crate::stream]

To abstract these needs away from the user, generally `winnow` parsers use the [`ModalResult`]
alias, rather than [`Result`].  [`Parser::parse`] is a top-level operation
that can help convert to a `Result` for integrating with your application's error reporting.

Error types include:
- [`EmptyError`] when the reason for failure doesn't matter
- [`ContextError`]
- [`InputError`] (mostly for testing)
- [`TreeError`] (mostly for testing)
- [Custom errors][crate::_topic::error]

```rust
pub mod error { /* ... */ }
```

### Types

#### Type Alias `Result`

By default, the error type (`E`) is [`ContextError`].

When integrating into the result of the application, see
- [`Parser::parse`]
- [`ParserError::into_inner`]

```rust
pub type Result<O, E = ContextError> = core::result::Result<O, E>;
```

#### Type Alias `ModalResult`

[Modal error reporting][ErrMode] for [`Parser::parse_next`]

- `Ok(O)` is the parsed value
- [`Err(ErrMode<E>)`][ErrMode] is the error along with how to respond to it

By default, the error type (`E`) is [`ContextError`].

When integrating into the result of the application, see
- [`Parser::parse`]
- [`ParserError::into_inner`]

```rust
pub type ModalResult<O, E = ContextError> = Result<O, ErrMode<E>>;
```

#### Enum `Needed`

Contains information on needed data if a parser returned `Incomplete`

<div class="warning">

**Note:** This is only possible for `Stream` that are [partial][`crate::stream::StreamIsPartial`],
like [`Partial`][crate::Partial].

</div>

```rust
pub enum Needed {
    Unknown,
    Size(core::num::NonZeroUsize),
}
```

##### Variants

###### `Unknown`

Needs more data, but we do not know how much

###### `Size`

Contains a lower bound on the buffer offset needed to finish parsing

For byte/`&str` streams, this translates to bytes

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `core::num::NonZeroUsize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(s: usize) -> Self { /* ... */ }
  ```
  Creates `Needed` instance, returns `Needed::Unknown` if the argument is zero

- ```rust
  pub fn is_known(self: &Self) -> bool { /* ... */ }
  ```
  Indicates if we know how many bytes we need

- ```rust
  pub fn map<F: Fn(NonZeroUsize) -> usize>(self: Self, f: F) -> Needed { /* ... */ }
  ```
  Maps a `Needed` to `Needed` by applying a function to a contained `Size` value.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
- **RefUnwindSafe**
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Needed) -> bool { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Needed { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
#### Enum `ErrMode`

Add parse error state to [`ParserError`]s

Needed for
- [`Partial`][crate::stream::Partial] to track whether the [`Stream`] is [`ErrMode::Incomplete`].
  See also [`_topic/partial`]
- Marking errors as unrecoverable ([`ErrMode::Cut`]) and not retrying alternative parsers.
  See also [`_tutorial/chapter_7#error-cuts`]

```rust
pub enum ErrMode<E> {
    Incomplete(Needed),
    Backtrack(E),
    Cut(E),
}
```

##### Variants

###### `Incomplete`

There was not enough data to determine the appropriate action

More data needs to be buffered before retrying the parse.

This must only be set when the [`Stream`] is [partial][`crate::stream::StreamIsPartial`], like with
[`Partial`][crate::Partial]

Convert this into an `Backtrack` with [`Parser::complete_err`]

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Needed` |  |

###### `Backtrack`

The parser failed with a recoverable error (the default).

For example, a parser for json values might include a
[`dec_uint`][crate::ascii::dec_uint] as one case in an [`alt`][crate::combinator::alt]
combinator. If it fails, the next case should be tried.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `E` |  |

###### `Cut`

The parser had an unrecoverable error.

The parser was on the right branch, so directly report it to the user rather than trying
other branches. You can use [`cut_err()`][crate::combinator::cut_err] combinator to switch
from `ErrMode::Backtrack` to `ErrMode::Cut`.

For example, one case in an [`alt`][crate::combinator::alt] combinator found a unique prefix
and you want any further errors parsing the case to be reported to the user.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `E` |  |

##### Implementations

###### Methods

- ```rust
  pub fn is_incomplete(self: &Self) -> bool { /* ... */ }
  ```
  Tests if the result is Incomplete

- ```rust
  pub fn cut(self: Self) -> Self { /* ... */ }
  ```
  Prevent backtracking, bubbling the error up to the top

- ```rust
  pub fn backtrack(self: Self) -> Self { /* ... */ }
  ```
  Enable backtracking support

- ```rust
  pub fn map<E2, F>(self: Self, f: F) -> ErrMode<E2>
where
    F: FnOnce(E) -> E2 { /* ... */ }
  ```
  Applies the given function to the inner error

- ```rust
  pub fn convert<F>(self: Self) -> ErrMode<F>
where
    E: ErrorConvert<F> { /* ... */ }
  ```
  Automatically converts between errors if the underlying type supports it

- ```rust
  pub fn into_inner(self: Self) -> Result<E, Self> { /* ... */ }
  ```
  Unwrap the mode, returning the underlying error

- ```rust
  pub fn map_input<U: Clone, F>(self: Self, f: F) -> ErrMode<InputError<U>>
where
    F: FnOnce(T) -> U { /* ... */ }
  ```
  Maps `ErrMode<InputError<T>>` to `ErrMode<InputError<U>>` with the given `F: T -> U`

###### Trait Implementations

- **Eq**
- **StructuralPartialEq**
- **Freeze**
- **ErrorConvert**
  - ```rust
    fn convert(self: Self) -> ErrMode<E2> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ErrMode<E> { /* ... */ }
    ```

- **ParserError**
  - ```rust
    fn from_input(input: &I) -> Self { /* ... */ }
    ```

  - ```rust
    fn assert(input: &I, message: &''static str) -> Self
where
    I: crate::lib::std::fmt::Debug { /* ... */ }
    ```

  - ```rust
    fn incomplete(_input: &I, needed: Needed) -> Self { /* ... */ }
    ```

  - ```rust
    fn append(self: Self, input: &I, token_start: &<I as Stream>::Checkpoint) -> Self { /* ... */ }
    ```

  - ```rust
    fn or(self: Self, other: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn is_backtrack(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn into_inner(self: Self) -> Result<<Self as >::Inner, Self> { /* ... */ }
    ```

  - ```rust
    fn is_incomplete(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn needed(self: &Self) -> Option<Needed> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ModalError**
  - ```rust
    fn cut(self: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn backtrack(self: Self) -> Self { /* ... */ }
    ```

- **Unpin**
- **Send**
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
    fn eq(self: &Self, other: &ErrMode<E>) -> bool { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromExternalError**
  - ```rust
    fn from_external_error(input: &I, e: EXT) -> Self { /* ... */ }
    ```

- **AddContext**
  - ```rust
    fn add_context(self: Self, input: &I, token_start: &<I as Stream>::Checkpoint, context: C) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `InputError`

Capture input on error

This is useful for testing of generic parsers to ensure the error happens at the right
location.

<div class="warning">

**Note:** [context][Parser::context] and inner errors (like from [`Parser::try_map`]) will be
dropped.

</div>

```rust
pub struct InputError<I: Clone> {
    pub input: I,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `input` | `I` | The input stream, pointing to the location where the error occurred |

##### Implementations

###### Methods

- ```rust
  pub fn at(input: I) -> Self { /* ... */ }
  ```
  Creates a new basic error

- ```rust
  pub fn map_input<I2: Clone, O: Fn(I) -> I2>(self: Self, op: O) -> InputError<I2> { /* ... */ }
  ```
  Translate the input type

- ```rust
  pub fn into_owned(self: Self) -> InputError<<I as ToOwned>::Owned> { /* ... */ }
  ```
  Obtaining ownership

###### Trait Implementations

- **AddContext**
- **Send**
- **Copy**
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

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErrorConvert**
  - ```rust
    fn convert(self: Self) -> InputError<(I, usize)> { /* ... */ }
    ```

  - ```rust
    fn convert(self: Self) -> InputError<I> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Error**
- **Eq**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InputError<I> { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InputError<I>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **FromExternalError**
  - ```rust
    fn from_external_error(input: &I, _e: E) -> Self { /* ... */ }
    ```
    Create a new error from an input position and an external error

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ParserError**
  - ```rust
    fn from_input(input: &I) -> Self { /* ... */ }
    ```

  - ```rust
    fn into_inner(self: Self) -> Result<<Self as >::Inner, Self> { /* ... */ }
    ```

#### Struct `EmptyError`

Track an error occurred without any other [`StrContext`]

```rust
pub struct EmptyError;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ParserError**
  - ```rust
    fn from_input(_: &I) -> Self { /* ... */ }
    ```

  - ```rust
    fn into_inner(self: Self) -> Result<<Self as >::Inner, Self> { /* ... */ }
    ```

- **AddContext**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut crate::lib::std::fmt::Formatter<''_>) -> crate::lib::std::fmt::Result { /* ... */ }
    ```

- **FromExternalError**
  - ```rust
    fn from_external_error(_input: &I, _e: E) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **ErrorConvert**
  - ```rust
    fn convert(self: Self) -> EmptyError { /* ... */ }
    ```

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

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EmptyError { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EmptyError) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `ContextError`

Accumulate context while backtracking errors

See the [tutorial][crate::_tutorial::chapter_7#error-adaptation-and-rendering]
for an example of how to adapt this to an application error with custom rendering.

```rust
pub struct ContextError<C = StrContext> {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Create an empty error

- ```rust
  pub fn context(self: &Self) -> impl Iterator<Item = &C> { /* ... */ }
  ```
  Access context from [`Parser::context`]

- ```rust
  pub fn cause(self: &Self) -> Option<&dyn std::error::Error + Send + Sync + ''static> { /* ... */ }
  ```
  Originating [`std::error::Error`]

###### Trait Implementations

- **AddContext**
  - ```rust
    fn add_context(self: Self, _input: &I, _token_start: &<I as Stream>::Checkpoint, context: C) -> Self { /* ... */ }
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut crate::lib::std::fmt::Formatter<''_>) -> crate::lib::std::fmt::Result { /* ... */ }
    ```

- **ErrorConvert**
  - ```rust
    fn convert(self: Self) -> ContextError<C> { /* ... */ }
    ```

- **Sync**
- **Send**
- **ParserError**
  - ```rust
    fn from_input(_input: &I) -> Self { /* ... */ }
    ```

  - ```rust
    fn into_inner(self: Self) -> Result<<Self as >::Inner, Self> { /* ... */ }
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

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FromExternalError**
  - ```rust
    fn from_external_error(_input: &I, e: E) -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Enum `StrContext`

**Attributes:**

- `#[non_exhaustive]`

Additional parse context for [`ContextError`] added via [`Parser::context`]

```rust
pub enum StrContext {
    Label(&''static str),
    Expected(StrContextValue),
}
```

##### Variants

###### `Label`

Description of what is currently being parsed

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `Expected`

Grammar item that was expected

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `StrContextValue` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut crate::lib::std::fmt::Formatter<''_>) -> crate::lib::std::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StrContext) -> bool { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **StructuralPartialEq**
- **Eq**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StrContext { /* ... */ }
    ```

- **Send**
- **Sync**
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

#### Enum `StrContextValue`

**Attributes:**

- `#[non_exhaustive]`

See [`StrContext`]

```rust
pub enum StrContextValue {
    CharLiteral(char),
    StringLiteral(&''static str),
    Description(&''static str),
}
```

##### Variants

###### `CharLiteral`

A [`char`] token

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

###### `StringLiteral`

A [`&str`] token

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `Description`

A description of what was being parsed

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
    fn eq(self: &Self, other: &StrContextValue) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(inner: char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(inner: &''static str) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Eq**
- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut crate::lib::std::fmt::Formatter<''_>) -> crate::lib::std::fmt::Result { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StrContextValue { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `TreeError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Trace all error paths, particularly for tests

```rust
pub enum TreeError<I, C = StrContext> {
    Base(TreeErrorBase<I>),
    Stack {
        base: Box<Self>,
        stack: Vec<TreeErrorFrame<I, C>>,
    },
    Alt(Vec<Self>),
}
```

##### Variants

###### `Base`

Initial error that kicked things off

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TreeErrorBase<I>` |  |

###### `Stack`

Traces added to the error while walking back up the stack

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `base` | `Box<Self>` | Initial error that kicked things off |
| `stack` | `Vec<TreeErrorFrame<I, C>>` | Traces added to the error while walking back up the stack |

###### `Alt`

All failed branches of an `alt`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Self>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn into_owned(self: Self) -> TreeError<<I as ToOwned>::Owned, C> { /* ... */ }
  ```
  Obtaining ownership

- ```rust
  pub fn map_input<I2, O: Clone + Fn(I) -> I2>(self: Self, op: O) -> TreeError<I2, C> { /* ... */ }
  ```
  Translate the input type

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AddContext**
  - ```rust
    fn add_context(self: Self, input: &I, token_start: &<I as Stream>::Checkpoint, context: C) -> Self { /* ... */ }
    ```

- **Error**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ParserError**
  - ```rust
    fn from_input(input: &I) -> Self { /* ... */ }
    ```

  - ```rust
    fn append(self: Self, input: &I, token_start: &<I as Stream>::Checkpoint) -> Self { /* ... */ }
    ```

  - ```rust
    fn or(self: Self, other: Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn into_inner(self: Self) -> Result<<Self as >::Inner, Self> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **ErrorConvert**
  - ```rust
    fn convert(self: Self) -> TreeError<(I, usize), C> { /* ... */ }
    ```

  - ```rust
    fn convert(self: Self) -> TreeError<I, C> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **FromExternalError**
  - ```rust
    fn from_external_error(input: &I, e: E) -> Self { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Enum `TreeErrorFrame`

**Attributes:**

- `#[<cfg>(feature = "std")]`

See [`TreeError::Stack`]

```rust
pub enum TreeErrorFrame<I, C = StrContext> {
    Kind(TreeErrorBase<I>),
    Context(TreeErrorContext<I, C>),
}
```

##### Variants

###### `Kind`

See [`ParserError::append`]

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TreeErrorBase<I>` |  |

###### `Context`

See [`AddContext::add_context`]

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TreeErrorContext<I, C>` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `TreeErrorBase`

**Attributes:**

- `#[<cfg>(feature = "std")]`

See [`TreeErrorFrame::Kind`], [`ParserError::append`]

```rust
pub struct TreeErrorBase<I> {
    pub input: I,
    pub cause: Option<Box<dyn std::error::Error + Send + Sync + ''static>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `input` | `I` | Parsed input, at the location where the error occurred |
| `cause` | `Option<Box<dyn std::error::Error + Send + Sync + ''static>>` | See [`FromExternalError::from_external_error`] |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `TreeErrorContext`

**Attributes:**

- `#[<cfg>(feature = "std")]`

See [`TreeErrorFrame::Context`], [`AddContext::add_context`]

```rust
pub struct TreeErrorContext<I, C = StrContext> {
    pub input: I,
    pub context: C,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `input` | `I` | Parsed input, at the location where the error occurred |
| `context` | `C` | See [`AddContext::add_context`] |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
#### Struct `ParseError`

See [`Parser::parse`]

```rust
pub struct ParseError<I, E> {
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
  pub fn input(self: &Self) -> &I { /* ... */ }
  ```
  The [`Stream`] at the initial location when parsing started

- ```rust
  pub fn offset(self: &Self) -> usize { /* ... */ }
  ```
  The location in [`ParseError::input`] where parsing failed

- ```rust
  pub fn inner(self: &Self) -> &E { /* ... */ }
  ```
  The original [`ParserError`]

- ```rust
  pub fn into_inner(self: Self) -> E { /* ... */ }
  ```
  The original [`ParserError`]

- ```rust
  pub fn char_span(self: &Self) -> crate::lib::std::ops::Range<usize> { /* ... */ }
  ```
  The byte indices for the `char` at [`ParseError::offset`]

###### Trait Implementations

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParseError<I, E>) -> bool { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParseError<I, E> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
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
### Traits

#### Trait `ParserError`

The basic [`Parser`] trait for errors

It provides methods to create an error from some combinators,
and combine existing errors in combinators like `alt`.

```rust
pub trait ParserError<I: Stream>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Inner`: Generally, `Self`

###### Required Methods

- `from_input`: Creates an error from the input position
- `into_inner`: Unwrap the mode, returning the underlying error, if present

##### Provided Methods

- ```rust
  fn assert(input: &I, _message: &''static str) -> Self
where
    I: crate::lib::std::fmt::Debug { /* ... */ }
  ```
  Process a parser assertion

- ```rust
  fn incomplete(input: &I, _needed: Needed) -> Self { /* ... */ }
  ```
  There was not enough data to determine the appropriate action

- ```rust
  fn append(self: Self, _input: &I, _token_start: &<I as Stream>::Checkpoint) -> Self { /* ... */ }
  ```
  Like [`ParserError::from_input`] but merges it with the existing error.

- ```rust
  fn or(self: Self, other: Self) -> Self { /* ... */ }
  ```
  Combines errors from two different parse branches.

- ```rust
  fn is_backtrack(self: &Self) -> bool { /* ... */ }
  ```
  Is backtracking and trying new parse branches allowed?

- ```rust
  fn is_incomplete(self: &Self) -> bool { /* ... */ }
  ```
  Is more data [`Needed`]

- ```rust
  fn needed(self: &Self) -> Option<Needed> { /* ... */ }
  ```
  Extract the [`Needed`] data, if present

##### Implementations

This trait is implemented for the following types:

- `ErrMode<E>` with <I: Stream, E: ParserError<I>>
- `InputError<I>` with <I: Stream + Clone>
- `EmptyError` with <I: Stream>
- `()` with <I: Stream>
- `ContextError<C>` with <I: Stream, C>
- `TreeError<I, C>` with <I, C>

#### Trait `ModalError`

Manipulate the how parsers respond to this error

```rust
pub trait ModalError {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `cut`: Prevent backtracking, bubbling the error up to the top
- `backtrack`: Enable backtracking support

##### Implementations

This trait is implemented for the following types:

- `ErrMode<E>` with <E>

#### Trait `AddContext`

Used by [`Parser::context`] to add custom data to error while backtracking

May be implemented multiple times for different kinds of context.

```rust
pub trait AddContext<I: Stream, C = &''static str>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn add_context(self: Self, _input: &I, _token_start: &<I as Stream>::Checkpoint, _context: C) -> Self { /* ... */ }
  ```
  Append to an existing error custom data

##### Implementations

This trait is implemented for the following types:

- `ErrMode<E>` with <I: Stream, C, E: AddContext<I, C>>
- `InputError<I>` with <I: Stream + Clone, C>
- `EmptyError` with <I: Stream, C>
- `()` with <I: Stream, C>
- `ContextError<C>` with <C, I: Stream>
- `TreeError<I, C>` with <I, C>

#### Trait `FromExternalError`

Create a new error with an external error, from [`std::str::FromStr`]

This trait is required by the [`Parser::try_map`] combinator.

```rust
pub trait FromExternalError<I, E> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `from_external_error`: Like [`ParserError::from_input`] but also include an external error.

##### Implementations

This trait is implemented for the following types:

- `ErrMode<E>` with <I, EXT, E>
- `InputError<I>` with <I: Clone, E>
- `EmptyError` with <I, E>
- `()` with <I, E>
- `ContextError<C>` with <C, I, E: std::error::Error + Send + Sync + ''static>
- `TreeError<I, C>` with <I, C, E: std::error::Error + Send + Sync + ''static>

#### Trait `ErrorConvert`

Equivalent of `From` implementation to avoid orphan rules in bits parsers

```rust
pub trait ErrorConvert<E> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `convert`: Transform to another error type

##### Implementations

This trait is implemented for the following types:

- `ErrMode<E1>` with <E1, E2>
- `InputError<I>` with <I: Clone>
- `InputError<(I, usize)>` with <I: Clone>
- `EmptyError`
- `()`
- `ContextError<C>` with <C>
- `TreeError<I, C>` with <I, C>
- `TreeError<(I, usize), C>` with <I, C>

## Module `stream`

Stream capability for combinators to parse

Stream types include:
- `&[u8]` and [`Bytes`] for binary data
- `&str` (aliased as [`Str`]) and [`BStr`] for UTF-8 data
- [`LocatingSlice`] can track the location within the original buffer to report
  [spans][crate::Parser::with_span]
- [`Stateful`] to thread global state through your parsers
- [`Partial`] can mark an input as partial buffer that is being streamed into
- [Custom stream types][crate::_topic::stream]

```rust
pub mod stream { /* ... */ }
```

### Types

#### Type Alias `Str`

UTF-8 Stream

```rust
pub type Str<''i> = &''i str;
```

#### Struct `BitOffsets`

Iterator for [bit][crate::binary::bits] stream (`(I, usize)`)

```rust
pub struct BitOffsets<I> {
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Enum `CompareResult`

Result of [`Compare::compare`]

```rust
pub enum CompareResult {
    Ok(usize),
    Incomplete,
    Error,
}
```

##### Variants

###### `Ok`

Comparison was successful

`usize` is the end of the successful match within the buffer.
This is most relevant for caseless UTF-8 where `Compare::compare`'s parameter might be a different
length than the match within the buffer.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `Incomplete`

We need more data to be sure

###### `Error`

Comparison failed

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CompareResult) -> bool { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Checkpoint`

Ensure checkpoint details are kept private

```rust
pub struct Checkpoint<T, S> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

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

- **Copy**
- **Freeze**
- **Unpin**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut crate::lib::std::fmt::Formatter<''_>) -> crate::lib::std::fmt::Result { /* ... */ }
    ```

- **Offset**
  - ```rust
    fn offset_from(self: &Self, start: &Self) -> usize { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Traits

#### Trait `SliceLen`

Abstract method to calculate the input length

```rust
pub trait SliceLen {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `slice_len`: Calculates the input length, as indicated by its name,

##### Implementations

This trait is implemented for the following types:

- `&BStr`
- `&Bytes`
- `LocatingSlice<I>` with <I>
- `Partial<I>` with <I>
- `Stateful<I, S>` with <I, S>
- `TokenSlice<''_, T>` with <T>
- `crate::ascii::Caseless<S>` with <S: SliceLen>
- `&[T]` with <T>
- `[T; LEN]` with <T, const LEN: usize>
- `&[T; LEN]` with <T, const LEN: usize>
- `&str`
- `u8`
- `char`
- `(I, usize, usize)` with <I>

#### Trait `Stream`

Core definition for parser input state

```rust
pub trait Stream: Offset<<Self as Stream>::Checkpoint> + crate::lib::std::fmt::Debug {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Token`: The smallest unit being parsed
- `Slice`: Sequence of `Token`s
- `IterOffsets`: Iterate with the offset from the current location
- `Checkpoint`: A parse location within the stream

###### Required Methods

- `iter_offsets`: Iterate with the offset from the current location
- `eof_offset`: Returns the offset to the end of the input
- `next_token`: Split off the next token from the input
- `peek_token`: Split off the next token from the input
- `offset_for`: Finds the offset of the next matching token
- `offset_at`: Get the offset for the number of `tokens` into the stream
- `next_slice`: Split off a slice of tokens from the input
- `peek_slice`: Split off a slice of tokens from the input
- `checkpoint`: Save the current parse location within the stream
- `reset`: Revert the stream to a prior [`Self::Checkpoint`]
- `raw`: Return the inner-most stream

##### Provided Methods

- ```rust
  unsafe fn next_slice_unchecked(self: &mut Self, offset: usize) -> <Self as >::Slice { /* ... */ }
  ```
  Split off a slice of tokens from the input

- ```rust
  unsafe fn peek_slice_unchecked(self: &Self, offset: usize) -> <Self as >::Slice { /* ... */ }
  ```
  Split off a slice of tokens from the input

- ```rust
  fn finish(self: &mut Self) -> <Self as >::Slice { /* ... */ }
  ```
  Advance to the end of the stream

- ```rust
  fn peek_finish(self: &Self) -> <Self as >::Slice
where
    Self: Clone { /* ... */ }
  ```
  Advance to the end of the stream

##### Implementations

This trait is implemented for the following types:

- `&''i BStr` with <''i>
- `&''i Bytes` with <''i>
- `LocatingSlice<I>` with <I: Stream>
- `Partial<I>` with <I: Stream>
- `Stateful<I, S>` with <I: Stream, S: crate::lib::std::fmt::Debug>
- `TokenSlice<''t, T>` with <''t, T>
- `&''i [T]` with <''i, T>
- `&''i str` with <''i>
- `(I, usize)` with <I>

#### Trait `Location`

Current parse locations offset

See [`LocatingSlice`] for adding location tracking to your [`Stream`]

```rust
pub trait Location {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `previous_token_end`: Previous token's end offset
- `current_token_start`: Current token's start offset

##### Implementations

This trait is implemented for the following types:

- `LocatingSlice<I>` with <I>
- `Partial<I>` with <I>
- `Stateful<I, S>` with <I, S>
- `TokenSlice<''_, T>` with <T>

#### Trait `StreamIsPartial`

Marks the input as being the complete buffer or a partial buffer for streaming input

See [`Partial`] for marking a presumed complete buffer type as a streaming buffer.

```rust
pub trait StreamIsPartial: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `PartialState`: Whether the stream is currently partial or complete

###### Required Methods

- `complete`: Mark the stream is complete
- `restore_partial`: Restore the stream back to its previous state
- `is_partial_supported`: Report whether the [`Stream`] is can ever be incomplete

##### Provided Methods

- ```rust
  fn is_partial(self: &Self) -> bool { /* ... */ }
  ```
  Report whether the [`Stream`] is currently incomplete

##### Implementations

This trait is implemented for the following types:

- `&BStr`
- `&Bytes`
- `LocatingSlice<I>` with <I>
- `Partial<I>` with <I>
- `Stateful<I, S>` with <I, S>
- `TokenSlice<''t, T>` with <''t, T>
- `&[T]` with <T>
- `&str`
- `(I, usize)` with <I>

#### Trait `Offset`

Useful functions to calculate the offset between slices and show a hexdump of a slice

```rust
pub trait Offset<Start = Self> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `offset_from`: Offset between the first byte of `start` and the first byte of `self`a

##### Implementations

This trait is implemented for the following types:

- `&BStr`
- `&''a BStr` with <''a>
- `&Bytes`
- `&''a Bytes` with <''a>
- `LocatingSlice<I>` with <I>
- `LocatingSlice<I>` with <I>
- `Partial<I>` with <I>
- `Partial<I>` with <I>
- `Stateful<I, S>` with <I, S>
- `Stateful<I, S>` with <I, S>
- `TokenSlice<''_, T>` with <T>
- `TokenSlice<''_, T>` with <T>
- `&[T]` with <T>
- `&''a [T]` with <''a, T>
- `&str`
- `&''a str` with <''a>
- `(I, usize)` with <I>
- `(I, usize)` with <I>
- `Checkpoint<I, S>` with <I, S>

#### Trait `AsBytes`

Helper trait for types that can be viewed as a byte slice

```rust
pub trait AsBytes {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_bytes`: Casts the input type to a byte slice

##### Implementations

This trait is implemented for the following types:

- `&Bytes`
- `LocatingSlice<I>` with <I>
- `Partial<I>` with <I>
- `Stateful<I, S>` with <I, S>
- `&[u8]`

#### Trait `AsBStr`

Helper trait for types that can be viewed as a byte slice

```rust
pub trait AsBStr {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_bstr`: Casts the input type to a byte slice

##### Implementations

This trait is implemented for the following types:

- `&BStr`
- `LocatingSlice<I>` with <I>
- `Partial<I>` with <I>
- `Stateful<I, S>` with <I, S>
- `&[u8]`
- `&str`

#### Trait `Compare`

Abstracts comparison operations

```rust
pub trait Compare<T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `compare`: Compares self to another value for equality

##### Implementations

This trait is implemented for the following types:

- `&''a BStr` with <''a, T>
- `&''a Bytes` with <''a, T>
- `LocatingSlice<I>` with <I, U>
- `Partial<I>` with <I, T>
- `Stateful<I, S>` with <I, S, U>
- `TokenSlice<''_, T>` with <T, O>
- `&[u8]` with <''b>
- `&[u8]` with <''b>
- `&[u8]` with <const LEN: usize>
- `&[u8]` with <const LEN: usize>
- `&[u8]` with <''b, const LEN: usize>
- `&[u8]` with <''b, const LEN: usize>
- `&[u8]` with <''b>
- `&[u8]` with <''b>
- `&[u8]`
- `&[u8]`
- `&[u8]`
- `&[u8]`
- `&str` with <''b>
- `&str` with <''b>
- `&str`
- `&str`

#### Trait `FindSlice`

Look for a slice in self

```rust
pub trait FindSlice<T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `find_slice`: Returns the offset of the slice if it is found

##### Implementations

This trait is implemented for the following types:

- `&''i BStr` with <''i, S>
- `&''i Bytes` with <''i, S>
- `LocatingSlice<I>` with <I, T>
- `Partial<I>` with <I, T>
- `Stateful<I, S>` with <I, S, T>
- `&[u8]` with <''s>
- `&[u8]` with <''s>
- `&[u8]` with <''s>
- `&[u8]` with <''s>
- `&[u8]`
- `&[u8]`
- `&[u8]`
- `&[u8]`
- `&[u8]`
- `&[u8]`
- `&[u8]`
- `&[u8]`
- `&[u8]` with <''s>
- `&[u8]` with <''s>
- `&[u8]` with <''s>
- `&[u8]` with <''s>
- `&str` with <''s>
- `&str` with <''s>
- `&str` with <''s>
- `&str` with <''s>
- `&str`
- `&str`
- `&str`
- `&str`

#### Trait `ParseSlice`

Used to integrate `str`'s `parse()` method

```rust
pub trait ParseSlice<R> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `parse_slice`: Succeeds if `parse()` succeeded

##### Implementations

This trait is implemented for the following types:

- `&[u8]` with <R: FromStr>
- `&str` with <R: FromStr>

#### Trait `UpdateSlice`

Convert a `Stream` into an appropriate `Output` type

```rust
pub trait UpdateSlice: Stream {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `update_slice`: Convert an `Output` type to be used as `Stream`

##### Implementations

This trait is implemented for the following types:

- `&BStr`
- `&Bytes`
- `LocatingSlice<I>` with <I>
- `Partial<I>` with <I>
- `Stateful<I, S>` with <I, S>
- `TokenSlice<''_, T>` with <T>
- `&[T]` with <T>
- `&str`

#### Trait `Accumulate`

Abstracts something which can extend an `Extend`.
Used to build modified input slices in `escaped_transform`

```rust
pub trait Accumulate<T>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `initial`: Create a new `Extend` of the correct type
- `accumulate`: Accumulate the input into an accumulator

##### Implementations

This trait is implemented for the following types:

- `()` with <T>
- `usize` with <T>
- `crate::lib::std::vec::Vec<T>` with <T>
- `crate::lib::std::vec::Vec<T>` with <''i, T: Clone>
- `crate::lib::std::string::String`
- `crate::lib::std::string::String` with <''i>
- `crate::lib::std::collections::BTreeMap<K, V>` with <K, V>
- `crate::lib::std::collections::HashMap<K, V, S>` with <K, V, S>
- `crate::lib::std::collections::BTreeSet<K>` with <K>
- `crate::lib::std::collections::HashSet<K, S>` with <K, S>

#### Trait `ToUsize`

Helper trait to convert numbers to usize.

By default, usize implements `From<u8>` and `From<u16>` but not
`From<u32>` and `From<u64>` because that would be invalid on some
platforms. This trait implements the conversion for platforms
with 32 and 64 bits pointer platforms

```rust
pub trait ToUsize {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `to_usize`: converts self to usize

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `usize`
- `u32`
- `u64`

#### Trait `AsChar`

**Attributes:**

- `#[allow(clippy::len_without_is_empty)]`
- `#[allow(clippy::wrong_self_convention)]`

Transforms a token into a char for basic string parsing

```rust
pub trait AsChar {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_char`: Makes a char from self
- `is_alpha`: Tests that self is an alphabetic character
- `is_alphanum`: Tests that self is an alphabetic character
- `is_dec_digit`: Tests that self is a decimal digit
- `is_hex_digit`: Tests that self is an hex digit
- `is_oct_digit`: Tests that self is an octal digit
- `len`: Gets the len in bytes for self
- `is_space`: Tests that self is ASCII space or tab
- `is_newline`: Tests if byte is ASCII newline: \n

##### Implementations

This trait is implemented for the following types:

- `u8`
- `&u8`
- `char`
- `&char`

#### Trait `ContainsToken`

Check if a token is in a set of possible tokens

While this can be implemented manually, you can also build up sets using:
- `b'c'` and `'c'`
- `b""`
- `|c| true`
- `b'a'..=b'z'`, `'a'..='z'` (etc for each [range type][std::ops])
- `(set1, set2, ...)`

# Example

For example, you could implement `hex_digit0` as:
```
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError};
# use winnow::token::take_while;
fn hex_digit1<'s>(input: &mut &'s str) -> ModalResult<&'s str, ContextError> {
    take_while(1.., ('a'..='f', 'A'..='F', '0'..='9')).parse_next(input)
}

assert_eq!(hex_digit1.parse_peek("21cZ"), Ok(("Z", "21c")));
assert!(hex_digit1.parse_peek("H2").is_err());
assert!(hex_digit1.parse_peek("").is_err());
```

```rust
pub trait ContainsToken<T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `contains_token`: Returns true if self contains the token

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u8`
- `u8`
- `u8`
- `char` with <C: AsChar>
- `F` with <C, F: Fn(C) -> bool>
- `crate::lib::std::ops::Range<C2>` with <C1: AsChar, C2: AsChar + Clone>
- `crate::lib::std::ops::RangeInclusive<C2>` with <C1: AsChar, C2: AsChar + Clone>
- `crate::lib::std::ops::RangeFrom<C2>` with <C1: AsChar, C2: AsChar + Clone>
- `crate::lib::std::ops::RangeTo<C2>` with <C1: AsChar, C2: AsChar + Clone>
- `crate::lib::std::ops::RangeToInclusive<C2>` with <C1: AsChar, C2: AsChar + Clone>
- `crate::lib::std::ops::RangeFull` with <C1: AsChar>
- `&[u8]` with <C: AsChar>
- `&[char]` with <C: AsChar>
- `&[u8; LEN]` with <const LEN: usize, C: AsChar>
- `&[char; LEN]` with <const LEN: usize, C: AsChar>
- `[u8; LEN]` with <const LEN: usize, C: AsChar>
- `[char; LEN]` with <const LEN: usize, C: AsChar>
- `()` with <T>
- `(F1)` with <T, F1>
- `(F1, F2)` with <T, F1, F2>
- `(F1, F2, F3)` with <T, F1, F2, F3>
- `(F1, F2, F3, F4)` with <T, F1, F2, F3, F4>
- `(F1, F2, F3, F4, F5)` with <T, F1, F2, F3, F4, F5>
- `(F1, F2, F3, F4, F5, F6)` with <T, F1, F2, F3, F4, F5, F6>
- `(F1, F2, F3, F4, F5, F6, F7)` with <T, F1, F2, F3, F4, F5, F6, F7>
- `(F1, F2, F3, F4, F5, F6, F7, F8)` with <T, F1, F2, F3, F4, F5, F6, F7, F8>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20>
- `(F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21)` with <T, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21>

### Re-exports

#### Re-export `BStr`

```rust
pub use bstr::BStr;
```

#### Re-export `Bytes`

```rust
pub use bytes::Bytes;
```

#### Re-export `LocatingSlice`

```rust
pub use locating::LocatingSlice;
```

#### Re-export `Partial`

```rust
pub use partial::Partial;
```

#### Re-export `Range`

```rust
pub use range::Range;
```

#### Re-export `Stateful`

```rust
pub use stateful::Stateful;
```

#### Re-export `TokenSlice`

```rust
pub use token::TokenSlice;
```

## Module `ascii`

Character specific parsers and combinators

Functions recognizing specific characters

```rust
pub mod ascii { /* ... */ }
```

### Types

#### Struct `Caseless`

Mark a value as case-insensitive for ASCII characters

# Example
```rust
# use winnow::prelude::*;
# use winnow::ascii::Caseless;

fn parser<'s>(s: &mut &'s str) -> ModalResult<&'s str> {
  Caseless("hello").parse_next(s)
}

assert_eq!(parser.parse_peek("Hello, World!"), Ok((", World!", "Hello")));
assert_eq!(parser.parse_peek("hello, World!"), Ok((", World!", "hello")));
assert_eq!(parser.parse_peek("HeLlo, World!"), Ok((", World!", "HeLlo")));
assert!(parser.parse_peek("Some").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
pub struct Caseless<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn as_bytes(self: &Self) -> Caseless<&[u8]> { /* ... */ }
  ```
  Get the byte-representation of this case-insensitive value

###### Trait Implementations

- **SliceLen**
  - ```rust
    fn slice_len(self: &Self) -> usize { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Compare**
  - ```rust
    fn compare(self: &Self, t: AsciiCaseless<&''b [u8]>) -> CompareResult { /* ... */ }
    ```

  - ```rust
    fn compare(self: &Self, t: AsciiCaseless<[u8; LEN]>) -> CompareResult { /* ... */ }
    ```

  - ```rust
    fn compare(self: &Self, t: AsciiCaseless<&''b [u8; LEN]>) -> CompareResult { /* ... */ }
    ```

  - ```rust
    fn compare(self: &Self, t: AsciiCaseless<&''b str>) -> CompareResult { /* ... */ }
    ```

  - ```rust
    fn compare(self: &Self, t: AsciiCaseless<u8>) -> CompareResult { /* ... */ }
    ```

  - ```rust
    fn compare(self: &Self, t: AsciiCaseless<char>) -> CompareResult { /* ... */ }
    ```

  - ```rust
    fn compare(self: &Self, t: AsciiCaseless<&''b str>) -> CompareResult { /* ... */ }
    ```

  - ```rust
    fn compare(self: &Self, t: AsciiCaseless<char>) -> CompareResult { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Caseless<T> { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ModalParser**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
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

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<<I as Stream>::Slice, E> { /* ... */ }
    ```

  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<<I as Stream>::Slice, E> { /* ... */ }
    ```

  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<<I as Stream>::Slice, E> { /* ... */ }
    ```

### Traits

#### Trait `Uint`

Metadata for parsing unsigned integers, see [`dec_uint`]

```rust
pub trait Uint: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`

#### Trait `Int`

Metadata for parsing signed integers, see [`dec_int`]

```rust
pub trait Int: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`

#### Trait `HexUint`

Metadata for parsing hex numbers, see [`hex_uint`]

```rust
pub trait HexUint: Default + Shl<Self, Output = Self> + Add<Self, Output = Self> + From<u8> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`

### Functions

#### Function `crlf`

**Attributes:**

- `#[inline(always)]`

Recognizes the string `"\r\n"`.

*Complete version*: Will return an error if there's not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn crlf<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::crlf.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::crlf;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    crlf.parse_next(input)
}

assert_eq!(parser.parse_peek("\r\nc"), Ok(("c", "\r\n")));
assert!(parser.parse_peek("ab\r\nc").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::crlf;
assert_eq!(crlf::<_, ErrMode<ContextError>>.parse_peek(Partial::new("\r\nc")), Ok((Partial::new("c"), "\r\n")));
assert!(crlf::<_, ErrMode<ContextError>>.parse_peek(Partial::new("ab\r\nc")).is_err());
assert_eq!(crlf::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::Unknown)));
```

```rust
pub fn crlf<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream + Compare<&''static str>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `till_line_ending`

**Attributes:**

- `#[inline(always)]`

Recognizes a string of 0+ characters until `"\r\n"`, `"\n"`, or eof.

*Complete version*: Will return an error if there's not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn till_line_ending<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::till_line_ending.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::till_line_ending;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    till_line_ending.parse_next(input)
}

assert_eq!(parser.parse_peek("ab\r\nc"), Ok(("\r\nc", "ab")));
assert_eq!(parser.parse_peek("ab\nc"), Ok(("\nc", "ab")));
assert_eq!(parser.parse_peek("abc"), Ok(("", "abc")));
assert_eq!(parser.parse_peek(""), Ok(("", "")));
assert!(parser.parse_peek("a\rb\nc").is_err());
assert!(parser.parse_peek("a\rbc").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::till_line_ending;
assert_eq!(till_line_ending::<_, ErrMode<ContextError>>.parse_peek(Partial::new("ab\r\nc")), Ok((Partial::new("\r\nc"), "ab")));
assert_eq!(till_line_ending::<_, ErrMode<ContextError>>.parse_peek(Partial::new("abc")), Err(ErrMode::Incomplete(Needed::Unknown)));
assert_eq!(till_line_ending::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::Unknown)));
assert!(till_line_ending::<_, ErrMode<ContextError>>.parse_peek(Partial::new("a\rb\nc")).is_err());
assert!(till_line_ending::<_, ErrMode<ContextError>>.parse_peek(Partial::new("a\rbc")).is_err());
```

```rust
pub fn till_line_ending<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream + Compare<&''static str> + FindSlice<(char, char)>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `line_ending`

**Attributes:**

- `#[inline(always)]`

Recognizes an end of line (both `"\n"` and `"\r\n"`).

*Complete version*: Will return an error if there's not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn line_ending<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::line_ending.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::line_ending;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    line_ending.parse_next(input)
}

assert_eq!(parser.parse_peek("\r\nc"), Ok(("c", "\r\n")));
assert!(parser.parse_peek("ab\r\nc").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::line_ending;
assert_eq!(line_ending::<_, ErrMode<ContextError>>.parse_peek(Partial::new("\r\nc")), Ok((Partial::new("c"), "\r\n")));
assert!(line_ending::<_, ErrMode<ContextError>>.parse_peek(Partial::new("ab\r\nc")).is_err());
assert_eq!(line_ending::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::Unknown)));
```

```rust
pub fn line_ending<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream + Compare<&''static str>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `newline`

**Attributes:**

- `#[inline(always)]`

Matches a newline character `'\n'`.

*Complete version*: Will return an error if there's not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn newline(input: &mut &str) -> ModalResult<char>
# {
#     winnow::ascii::newline.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::newline;
fn parser<'s>(input: &mut &'s str) -> ModalResult<char> {
    newline.parse_next(input)
}

assert_eq!(parser.parse_peek("\nc"), Ok(("c", '\n')));
assert!(parser.parse_peek("\r\nc").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::newline;
assert_eq!(newline::<_, ErrMode<ContextError>>.parse_peek(Partial::new("\nc")), Ok((Partial::new("c"), '\n')));
assert!(newline::<_, ErrMode<ContextError>>.parse_peek(Partial::new("\r\nc")).is_err());
assert_eq!(newline::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::Unknown)));
```

```rust
pub fn newline<I, Error: ParserError<I>>(input: &mut I) -> crate::Result<char, Error>
where
    I: StreamIsPartial + Stream + Compare<char> { /* ... */ }
```

#### Function `tab`

**Attributes:**

- `#[inline(always)]`

Matches a tab character `'\t'`.

*Complete version*: Will return an error if there's not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn tab(input: &mut &str) -> ModalResult<char>
# {
#     winnow::ascii::tab.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::tab;
fn parser<'s>(input: &mut &'s str) -> ModalResult<char> {
    tab.parse_next(input)
}

assert_eq!(parser.parse_peek("\tc"), Ok(("c", '\t')));
assert!(parser.parse_peek("\r\nc").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::tab;
assert_eq!(tab::<_, ErrMode<ContextError>>.parse_peek(Partial::new("\tc")), Ok((Partial::new("c"), '\t')));
assert!(tab::<_, ErrMode<ContextError>>.parse_peek(Partial::new("\r\nc")).is_err());
assert_eq!(tab::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::Unknown)));
```

```rust
pub fn tab<Input, Error>(input: &mut Input) -> crate::Result<char, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `alpha0`

**Attributes:**

- `#[inline(always)]`

Recognizes zero or more lowercase and uppercase ASCII alphabetic characters: `'a'..='z'`, `'A'..='Z'`

*Complete version*: Will return the whole input if no terminating token is found (a non
alphabetic character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non alphabetic character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn alpha0<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::alpha0.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::alpha0;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    alpha0.parse_next(input)
}

assert_eq!(parser.parse_peek("ab1c"), Ok(("1c", "ab")));
assert_eq!(parser.parse_peek("1c"), Ok(("1c", "")));
assert_eq!(parser.parse_peek(""), Ok(("", "")));
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::alpha0;
assert_eq!(alpha0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("ab1c")), Ok((Partial::new("1c"), "ab")));
assert_eq!(alpha0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("1c")), Ok((Partial::new("1c"), "")));
assert_eq!(alpha0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn alpha0<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `alpha1`

**Attributes:**

- `#[inline(always)]`

Recognizes one or more lowercase and uppercase ASCII alphabetic characters: `'a'..='z'`, `'A'..='Z'`

*Complete version*: Will return an error if there's not enough input data,
or the whole input if no terminating token is found  (a non alphabetic character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non alphabetic character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn alpha1<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::alpha1.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::alpha1;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    alpha1.parse_next(input)
}

assert_eq!(parser.parse_peek("aB1c"), Ok(("1c", "aB")));
assert!(parser.parse_peek("1c").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::alpha1;
assert_eq!(alpha1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("aB1c")), Ok((Partial::new("1c"), "aB")));
assert!(alpha1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("1c")).is_err());
assert_eq!(alpha1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn alpha1<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `digit0`

**Attributes:**

- `#[inline(always)]`

Recognizes zero or more ASCII numerical characters: `'0'..='9'`

*Complete version*: Will return an error if there's not enough input data,
or the whole input if no terminating token is found (a non digit character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non digit character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn digit0<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::digit0.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::digit0;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    digit0.parse_next(input)
}

assert_eq!(parser.parse_peek("21c"), Ok(("c", "21")));
assert_eq!(parser.parse_peek("21"), Ok(("", "21")));
assert_eq!(parser.parse_peek("a21c"), Ok(("a21c", "")));
assert_eq!(parser.parse_peek(""), Ok(("", "")));
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::digit0;
assert_eq!(digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("21c")), Ok((Partial::new("c"), "21")));
assert_eq!(digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("a21c")), Ok((Partial::new("a21c"), "")));
assert_eq!(digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn digit0<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `digit1`

**Attributes:**

- `#[inline(always)]`

Recognizes one or more ASCII numerical characters: `'0'..='9'`

*Complete version*: Will return an error if there's not enough input data,
or the whole input if no terminating token is found (a non digit character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non digit character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn digit1<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::digit1.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::digit1;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    digit1.parse_next(input)
}

assert_eq!(parser.parse_peek("21c"), Ok(("c", "21")));
assert!(parser.parse_peek("c1").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::digit1;
assert_eq!(digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("21c")), Ok((Partial::new("c"), "21")));
assert!(digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("c1")).is_err());
assert_eq!(digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

## Parsing an integer

You can use `digit1` in combination with [`Parser::try_map`] to parse an integer:

```rust
# use winnow::prelude::*;
# use winnow::ascii::digit1;
fn parser<'s>(input: &mut &'s str) -> ModalResult<u32> {
  digit1.try_map(str::parse).parse_next(input)
}

assert_eq!(parser.parse_peek("416"), Ok(("", 416)));
assert_eq!(parser.parse_peek("12b"), Ok(("b", 12)));
assert!(parser.parse_peek("b").is_err());
```

```rust
pub fn digit1<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `hex_digit0`

**Attributes:**

- `#[inline(always)]`

Recognizes zero or more ASCII hexadecimal numerical characters: `'0'..='9'`, `'A'..='F'`,
`'a'..='f'`

*Complete version*: Will return the whole input if no terminating token is found (a non hexadecimal digit character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non hexadecimal digit character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn hex_digit0<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::hex_digit0.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::hex_digit0;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    hex_digit0.parse_next(input)
}

assert_eq!(parser.parse_peek("21cZ"), Ok(("Z", "21c")));
assert_eq!(parser.parse_peek("Z21c"), Ok(("Z21c", "")));
assert_eq!(parser.parse_peek(""), Ok(("", "")));
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::hex_digit0;
assert_eq!(hex_digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("21cZ")), Ok((Partial::new("Z"), "21c")));
assert_eq!(hex_digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("Z21c")), Ok((Partial::new("Z21c"), "")));
assert_eq!(hex_digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn hex_digit0<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `hex_digit1`

**Attributes:**

- `#[inline(always)]`

Recognizes one or more ASCII hexadecimal numerical characters: `'0'..='9'`, `'A'..='F'`,
`'a'..='f'`

*Complete version*: Will return an error if there's not enough input data,
or the whole input if no terminating token is found (a non hexadecimal digit character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non hexadecimal digit character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn hex_digit1<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::hex_digit1.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::hex_digit1;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    hex_digit1.parse_next(input)
}

assert_eq!(parser.parse_peek("21cZ"), Ok(("Z", "21c")));
assert!(parser.parse_peek("H2").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::hex_digit1;
assert_eq!(hex_digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("21cZ")), Ok((Partial::new("Z"), "21c")));
assert!(hex_digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("H2")).is_err());
assert_eq!(hex_digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn hex_digit1<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `oct_digit0`

**Attributes:**

- `#[inline(always)]`

Recognizes zero or more octal characters: `'0'..='7'`

*Complete version*: Will return the whole input if no terminating token is found (a non octal
digit character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non octal digit character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn oct_digit0<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::oct_digit0.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::oct_digit0;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    oct_digit0.parse_next(input)
}

assert_eq!(parser.parse_peek("21cZ"), Ok(("cZ", "21")));
assert_eq!(parser.parse_peek("Z21c"), Ok(("Z21c", "")));
assert_eq!(parser.parse_peek(""), Ok(("", "")));
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::oct_digit0;
assert_eq!(oct_digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("21cZ")), Ok((Partial::new("cZ"), "21")));
assert_eq!(oct_digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("Z21c")), Ok((Partial::new("Z21c"), "")));
assert_eq!(oct_digit0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn oct_digit0<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `oct_digit1`

**Attributes:**

- `#[inline(always)]`

Recognizes one or more octal characters: `'0'..='7'`

*Complete version*: Will return an error if there's not enough input data,
or the whole input if no terminating token is found (a non octal digit character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non octal digit character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn oct_digit1<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::oct_digit1.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::oct_digit1;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    oct_digit1.parse_next(input)
}

assert_eq!(parser.parse_peek("21cZ"), Ok(("cZ", "21")));
assert!(parser.parse_peek("H2").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::oct_digit1;
assert_eq!(oct_digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("21cZ")), Ok((Partial::new("cZ"), "21")));
assert!(oct_digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("H2")).is_err());
assert_eq!(oct_digit1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn oct_digit1<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `alphanumeric0`

**Attributes:**

- `#[inline(always)]`

Recognizes zero or more ASCII numerical and alphabetic characters: `'a'..='z'`, `'A'..='Z'`, `'0'..='9'`

*Complete version*: Will return the whole input if no terminating token is found (a non
alphanumerical character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non alphanumerical character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn alphanumeric0<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::alphanumeric0.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::alphanumeric0;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    alphanumeric0.parse_next(input)
}

assert_eq!(parser.parse_peek("21cZ%1"), Ok(("%1", "21cZ")));
assert_eq!(parser.parse_peek("&Z21c"), Ok(("&Z21c", "")));
assert_eq!(parser.parse_peek(""), Ok(("", "")));
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::alphanumeric0;
assert_eq!(alphanumeric0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("21cZ%1")), Ok((Partial::new("%1"), "21cZ")));
assert_eq!(alphanumeric0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("&Z21c")), Ok((Partial::new("&Z21c"), "")));
assert_eq!(alphanumeric0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn alphanumeric0<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `alphanumeric1`

**Attributes:**

- `#[inline(always)]`

Recognizes one or more ASCII numerical and alphabetic characters: `'a'..='z'`, `'A'..='Z'`, `'0'..='9'`

*Complete version*: Will return an error if there's not enough input data,
or the whole input if no terminating token is found (a non alphanumerical character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non alphanumerical character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn alphanumeric1<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::alphanumeric1.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::alphanumeric1;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    alphanumeric1.parse_next(input)
}

assert_eq!(parser.parse_peek("21cZ%1"), Ok(("%1", "21cZ")));
assert!(parser.parse_peek("&H2").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::alphanumeric1;
assert_eq!(alphanumeric1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("21cZ%1")), Ok((Partial::new("%1"), "21cZ")));
assert!(alphanumeric1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("&H2")).is_err());
assert_eq!(alphanumeric1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn alphanumeric1<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `space0`

**Attributes:**

- `#[inline(always)]`

Recognizes zero or more spaces and tabs.

*Complete version*: Will return the whole input if no terminating token is found (a non space
character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non space character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn space0<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::space0.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::space0;
assert_eq!(space0::<_, ErrMode<ContextError>>.parse_peek(Partial::new(" \t21c")), Ok((Partial::new("21c"), " \t")));
assert_eq!(space0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("Z21c")), Ok((Partial::new("Z21c"), "")));
assert_eq!(space0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn space0<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `space1`

**Attributes:**

- `#[inline(always)]`

Recognizes one or more spaces and tabs.

*Complete version*: Will return the whole input if no terminating token is found (a non space
character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non space character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn space1<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::space1.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::space1;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    space1.parse_next(input)
}

assert_eq!(parser.parse_peek(" \t21c"), Ok(("21c", " \t")));
assert!(parser.parse_peek("H2").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::space1;
assert_eq!(space1::<_, ErrMode<ContextError>>.parse_peek(Partial::new(" \t21c")), Ok((Partial::new("21c"), " \t")));
assert!(space1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("H2")).is_err());
assert_eq!(space1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn space1<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `multispace0`

**Attributes:**

- `#[inline(always)]`

Recognizes zero or more spaces, tabs, carriage returns and line feeds.

*Complete version*: will return the whole input if no terminating token is found (a non space
character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non space character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn multispace0<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::multispace0.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::multispace0;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    multispace0.parse_next(input)
}

assert_eq!(parser.parse_peek(" \t\n\r21c"), Ok(("21c", " \t\n\r")));
assert_eq!(parser.parse_peek("Z21c"), Ok(("Z21c", "")));
assert_eq!(parser.parse_peek(""), Ok(("", "")));
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::multispace0;
assert_eq!(multispace0::<_, ErrMode<ContextError>>.parse_peek(Partial::new(" \t\n\r21c")), Ok((Partial::new("21c"), " \t\n\r")));
assert_eq!(multispace0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("Z21c")), Ok((Partial::new("Z21c"), "")));
assert_eq!(multispace0::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn multispace0<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `multispace1`

**Attributes:**

- `#[inline(always)]`

Recognizes one or more spaces, tabs, carriage returns and line feeds.

*Complete version*: will return an error if there's not enough input data,
or the whole input if no terminating token is found (a non space character).

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data,
or if no terminating token is found (a non space character).

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn multispace1<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::ascii::multispace1.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::multispace1;
fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    multispace1.parse_next(input)
}

assert_eq!(parser.parse_peek(" \t\n\r21c"), Ok(("21c", " \t\n\r")));
assert!(parser.parse_peek("H2").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::ascii::multispace1;
assert_eq!(multispace1::<_, ErrMode<ContextError>>.parse_peek(Partial::new(" \t\n\r21c")), Ok((Partial::new("21c"), " \t\n\r")));
assert!(multispace1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("H2")).is_err());
assert_eq!(multispace1::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn multispace1<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `dec_uint`

**Attributes:**

- `#[doc(alias = "u8")]`
- `#[doc(alias = "u16")]`
- `#[doc(alias = "u32")]`
- `#[doc(alias = "u64")]`
- `#[doc(alias = "u128")]`

Decode a decimal unsigned integer (e.g. [`u32`])

*Complete version*: can parse until the end of input.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream] into a `u32`:
```rust
# use winnow::prelude::*;;
pub fn dec_uint(input: &mut &str) -> ModalResult<u32>
# {
#     winnow::ascii::dec_uint.parse_next(input)
# }
```

```rust
pub fn dec_uint<Input, Output, Error>(input: &mut Input) -> crate::Result<Output, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Output: Uint,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `dec_int`

**Attributes:**

- `#[doc(alias = "i8")]`
- `#[doc(alias = "i16")]`
- `#[doc(alias = "i32")]`
- `#[doc(alias = "i64")]`
- `#[doc(alias = "i128")]`

Decode a decimal signed integer (e.g. [`i32`])

*Complete version*: can parse until the end of input.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream] into an `i32`:
```rust
# use winnow::prelude::*;;
pub fn dec_int(input: &mut &str) -> ModalResult<i32>
# {
#     winnow::ascii::dec_int.parse_next(input)
# }
```

```rust
pub fn dec_int<Input, Output, Error>(input: &mut Input) -> crate::Result<Output, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Output: Int,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `hex_uint`

**Attributes:**

- `#[inline]`

Decode a variable-width hexadecimal integer (e.g. [`u32`])

*Complete version*: Will parse until the end of input if it has fewer characters than the type
supports.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if end-of-input
is hit before a hard boundary (non-hex character, more characters than supported).

# Effective Signature

Assuming you are parsing a `&str` [Stream] into a `u32`:
```rust
# use winnow::prelude::*;;
pub fn hex_uint(input: &mut &str) -> ModalResult<u32>
# {
#     winnow::ascii::hex_uint.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
use winnow::ascii::hex_uint;

fn parser<'s>(s: &mut &'s [u8]) -> ModalResult<u32> {
  hex_uint(s)
}

assert_eq!(parser.parse_peek(&b"01AE"[..]), Ok((&b""[..], 0x01AE)));
assert_eq!(parser.parse_peek(&b"abc"[..]), Ok((&b""[..], 0x0ABC)));
assert!(parser.parse_peek(&b"ggg"[..]).is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::Needed};
# use winnow::Partial;
use winnow::ascii::hex_uint;

fn parser<'s>(s: &mut Partial<&'s [u8]>) -> ModalResult<u32> {
  hex_uint(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"01AE;"[..])), Ok((Partial::new(&b";"[..]), 0x01AE)));
assert_eq!(parser.parse_peek(Partial::new(&b"abc"[..])), Err(ErrMode::Incomplete(Needed::new(1))));
assert!(parser.parse_peek(Partial::new(&b"ggg"[..])).is_err());
```

```rust
pub fn hex_uint<Input, Output, Error>(input: &mut Input) -> crate::Result<Output, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    <Input as Stream>::Slice: AsBStr,
    Output: HexUint,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `float`

**Attributes:**

- `#[inline(always)]`
- `#[doc(alias = "f32")]`
- `#[doc(alias = "double")]`
- `#[allow(clippy::trait_duplication_in_bounds)]`

Recognizes floating point number in text format and returns a [`f32`] or [`f64`].

*Complete version*: Can parse until the end of input.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Effective Signature

Assuming you are parsing a `&str` [Stream] into an `f64`:
```rust
# use winnow::prelude::*;;
pub fn float(input: &mut &str) -> ModalResult<f64>
# {
#     winnow::ascii::float.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::ascii::float;

fn parser<'s>(s: &mut &'s str) -> ModalResult<f64> {
  float(s)
}

assert_eq!(parser.parse_peek("11e-1"), Ok(("", 1.1)));
assert_eq!(parser.parse_peek("123E-02"), Ok(("", 1.23)));
assert_eq!(parser.parse_peek("123K-01"), Ok(("K-01", 123.0)));
assert!(parser.parse_peek("abc").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::Needed};
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::ascii::float;

fn parser<'s>(s: &mut Partial<&'s str>) -> ModalResult<f64> {
  float(s)
}

assert_eq!(parser.parse_peek(Partial::new("11e-1 ")), Ok((Partial::new(" "), 1.1)));
assert_eq!(parser.parse_peek(Partial::new("11e-1")), Err(ErrMode::Incomplete(Needed::new(1))));
assert_eq!(parser.parse_peek(Partial::new("123E-02")), Err(ErrMode::Incomplete(Needed::new(1))));
assert_eq!(parser.parse_peek(Partial::new("123K-01")), Ok((Partial::new("K-01"), 123.0)));
assert!(parser.parse_peek(Partial::new("abc")).is_err());
```

```rust
pub fn float<Input, Output, Error>(input: &mut Input) -> crate::Result<Output, Error>
where
    Input: StreamIsPartial + Stream + Compare<Caseless<&''static str>> + Compare<char> + AsBStr,
    <Input as Stream>::Slice: ParseSlice<Output>,
    <Input as Stream>::Token: AsChar + Clone,
    <Input as Stream>::IterOffsets: Clone,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `take_escaped`

**Attributes:**

- `#[inline(always)]`

Recognize the input slice with escaped characters.

Arguments:
- `normal`: unescapeable characters
  - Must not include `control`
- `control_char`: e.g. `\` for strings in most languages
- `escape`: parse and transform the escaped character

Parsing ends when:
- `alt(normal, control._char)` [`Backtrack`s][crate::error::ErrMode::Backtrack]
- `normal` doesn't advance the input stream
- *(complete)* input stream is exhausted

See also [`escaped_transform`]

<div class="warning">

**Warning:** If the `normal` parser passed to `take_escaped` accepts empty inputs
(like `alpha0` or `digit0`), `take_escaped` will return an error,
to prevent going into an infinite loop.

</div>


# Example

```rust
# use winnow::prelude::*;
# use winnow::ascii::digit1;
# use winnow::prelude::*;
use winnow::ascii::take_escaped;
use winnow::token::one_of;

fn esc<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
  take_escaped(digit1, '\\', one_of(['"', 'n', '\\'])).parse_next(input)
}

assert_eq!(esc.parse_peek("123;"), Ok((";", "123")));
assert_eq!(esc.parse_peek(r#"12\"34;"#), Ok((";", r#"12\"34"#)));
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::Needed};
# use winnow::ascii::digit1;
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::ascii::take_escaped;
use winnow::token::one_of;

fn esc<'i>(input: &mut Partial<&'i str>) -> ModalResult<&'i str> {
  take_escaped(digit1, '\\', one_of(['"', 'n', '\\'])).parse_next(input)
}

assert_eq!(esc.parse_peek(Partial::new("123;")), Ok((Partial::new(";"), "123")));
assert_eq!(esc.parse_peek(Partial::new("12\\\"34;")), Ok((Partial::new(";"), "12\\\"34")));
```

```rust
pub fn take_escaped<Input, Error, Normal, Escapable, NormalOutput, EscapableOutput>(normal: Normal, control_char: char, escapable: Escapable) -> impl Parser<Input, <Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    Normal: Parser<Input, NormalOutput, Error>,
    Escapable: Parser<Input, EscapableOutput, Error>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `escaped_transform`

**Attributes:**

- `#[inline(always)]`

**⚠️ Deprecated since 7.0.0**: replaced with `escaped`

Deprecated, replaed with [`escaped`]

```rust
pub fn escaped_transform<Input, Error, Normal, NormalOutput, Escape, EscapeOutput, Output>(normal: Normal, control_char: char, escape: Escape) -> impl Parser<Input, Output, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    Normal: Parser<Input, NormalOutput, Error>,
    Escape: Parser<Input, EscapeOutput, Error>,
    Output: crate::stream::Accumulate<NormalOutput> + crate::stream::Accumulate<EscapeOutput>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `escaped`

**Attributes:**

- `#[inline(always)]`

Parse escaped characters, unescaping them

Arguments:
- `normal`: unescapeable characters
  - Must not include `control`
- `control_char`: e.g. `\` for strings in most languages
- `escape`: parse and transform the escaped character

Parsing ends when:
- `alt(normal, control._char)` [`Backtrack`s][crate::error::ErrMode::Backtrack]
- `normal` doesn't advance the input stream
- *(complete)* input stream is exhausted

<div class="warning">

**Warning:** If the `normal` parser passed to `escaped_transform` accepts empty inputs
(like `alpha0` or `digit0`), `escaped_transform` will return an error,
to prevent going into an infinite loop.

</div>

# Example

```rust
# #[cfg(feature = "std")] {
# use winnow::prelude::*;
# use std::str::from_utf8;
use winnow::token::literal;
use winnow::ascii::escaped_transform;
use winnow::ascii::alpha1;
use winnow::combinator::alt;

fn parser<'s>(input: &mut &'s str) -> ModalResult<String> {
  escaped_transform(
    alpha1,
    '\\',
    alt((
      "\\".value("\\"),
      "\"".value("\""),
      "n".value("\n"),
    ))
  ).parse_next(input)
}

assert_eq!(parser.parse_peek("ab\\\"cd"), Ok(("", String::from("ab\"cd"))));
assert_eq!(parser.parse_peek("ab\\ncd"), Ok(("", String::from("ab\ncd"))));
# }
```

```rust
# #[cfg(feature = "std")] {
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::Needed};
# use std::str::from_utf8;
# use winnow::Partial;
use winnow::token::literal;
use winnow::ascii::escaped_transform;
use winnow::ascii::alpha1;
use winnow::combinator::alt;

fn parser<'s>(input: &mut Partial<&'s str>) -> ModalResult<String> {
  escaped_transform(
    alpha1,
    '\\',
    alt((
      "\\".value("\\"),
      "\"".value("\""),
      "n".value("\n"),
    ))
  ).parse_next(input)
}

assert_eq!(parser.parse_peek(Partial::new("ab\\\"cd\"")), Ok((Partial::new("\""), String::from("ab\"cd"))));
# }
```

```rust
pub fn escaped<Input, Error, Normal, NormalOutput, Escape, EscapeOutput, Output>(normal: Normal, control_char: char, escape: Escape) -> impl Parser<Input, Output, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    Normal: Parser<Input, NormalOutput, Error>,
    Escape: Parser<Input, EscapeOutput, Error>,
    Output: crate::stream::Accumulate<NormalOutput> + crate::stream::Accumulate<EscapeOutput>,
    Error: ParserError<Input> { /* ... */ }
```

## Module `binary`

**Attributes:**

- `#[allow(clippy::match_same_arms)]`

Parsers recognizing numbers

```rust
pub mod binary { /* ... */ }
```

### Modules

## Module `bits`

Bit level parsers


```rust
pub mod bits { /* ... */ }
```

### Functions

#### Function `bits`

Converts a byte-level input to a bit-level input

See [`bytes`] to convert it back.

# Example
```rust
# use winnow::prelude::*;
# use winnow::Bytes;
# use winnow::binary::bits::{bits, take};
# use winnow::error::ContextError;
# use winnow::error::ErrMode;
type Stream<'i> = &'i Bytes;

fn stream(b: &[u8]) -> Stream<'_> {
    Bytes::new(b)
}

fn parse(input: &mut Stream<'_>) -> ModalResult<(u8, u8)> {
    bits::<_, _, ErrMode<ContextError>, _, _>((take(4usize), take(8usize))).parse_next(input)
}

let input = stream(&[0x12, 0x34, 0xff, 0xff]);

let output = parse.parse_peek(input).expect("We take 1.5 bytes and the input is longer than 2 bytes");

// The first byte is consumed, the second byte is partially consumed and dropped.
let remaining = output.0;
assert_eq!(remaining, stream(&[0xff, 0xff]));

let parsed = output.1;
assert_eq!(parsed.0, 0x01);
assert_eq!(parsed.1, 0x23);
```

```rust
pub fn bits<Input, Output, BitError, ByteError, ParseNext>(parser: ParseNext) -> impl Parser<Input, Output, ByteError>
where
    BitError: ParserError<(Input, usize)> + ErrorConvert<ByteError>,
    ByteError: ParserError<Input>,
    (Input, usize): Stream,
    Input: Stream + Clone,
    ParseNext: Parser<(Input, usize), Output, BitError> { /* ... */ }
```

#### Function `bytes`

Convert a [`bits`] stream back into a byte stream

<div class="warning">

**Warning:** A partial byte remaining in the input will be ignored and the given parser will
start parsing at the next full byte.

</div>

# Examples

```
# use winnow::prelude::*;
# use winnow::Bytes;
# use winnow::token::rest;
# use winnow::error::ContextError;
# use winnow::error::ErrMode;
use winnow::binary::bits::{bits, bytes, take};

type Stream<'i> = &'i Bytes;

fn stream(b: &[u8]) -> Stream<'_> {
    Bytes::new(b)
}

fn parse<'i>(input: &mut Stream<'i>) -> ModalResult<(u8, u8, &'i [u8])> {
  bits::<_, _, ErrMode<ContextError>, _, _>((
    take(4usize),
    take(8usize),
    bytes::<_, _, ErrMode<ContextError>, _, _>(rest)
  )).parse_next(input)
}

let input = stream(&[0x12, 0x34, 0xff, 0xff]);

assert_eq!(parse.parse_peek(input), Ok(( stream(&[]), (0x01, 0x23, &[0xff, 0xff][..]) )));
```

```rust
pub fn bytes<Input, Output, ByteError, BitError, ParseNext>(parser: ParseNext) -> impl Parser<(Input, usize), Output, BitError>
where
    ByteError: ParserError<Input> + ErrorConvert<BitError>,
    BitError: ParserError<(Input, usize)>,
    Input: Stream<Token = u8> + Clone,
    ParseNext: Parser<Input, Output, ByteError> { /* ... */ }
```

#### Function `take`

**Attributes:**

- `#[inline(always)]`

Parse taking `count` bits

# Effective Signature

Assuming you are parsing a `(&[u8], usize)` bit [Stream]:
```rust
# use winnow::prelude::*;;
# use winnow::error::ContextError;
pub fn take<'i>(count: usize) -> impl Parser<(&'i [u8], usize), u8, ContextError>
# {
#     winnow::binary::bits::take(count)
# }
```

# Example
```rust
# use winnow::prelude::*;
# use winnow::Bytes;
# use winnow::error::ContextError;
use winnow::binary::bits::take;

type Stream<'i> = &'i Bytes;

fn stream(b: &[u8]) -> Stream<'_> {
    Bytes::new(b)
}

// Consumes 0 bits, returns 0
assert_eq!(take::<_, usize, _, ContextError>(0usize).parse_peek((stream(&[0b00010010]), 0)), Ok(((stream(&[0b00010010]), 0), 0)));

// Consumes 4 bits, returns their values and increase offset to 4
assert_eq!(take::<_, usize, _, ContextError>(4usize).parse_peek((stream(&[0b00010010]), 0)), Ok(((stream(&[0b00010010]), 4), 0b00000001)));

// Consumes 4 bits, offset is 4, returns their values and increase offset to 0 of next byte
assert_eq!(take::<_, usize, _, ContextError>(4usize).parse_peek((stream(&[0b00010010]), 4)), Ok(((stream(&[]), 0), 0b00000010)));

// Tries to consume 12 bits but only 8 are available
assert!(take::<_, usize, _, ContextError>(12usize).parse_peek((stream(&[0b00010010]), 0)).is_err());
```

```rust
pub fn take<Input, Output, Count, Error>(count: Count) -> impl Parser<(Input, usize), Output, Error>
where
    Input: Stream<Token = u8> + StreamIsPartial + Clone,
    Output: From<u8> + AddAssign + Shl<usize, Output = Output> + Shr<usize, Output = Output>,
    Count: ToUsize,
    Error: ParserError<(Input, usize)> { /* ... */ }
```

#### Function `pattern`

**Attributes:**

- `#[inline(always)]`
- `#[doc(alias = "literal")]`
- `#[doc(alias = "just")]`
- `#[doc(alias = "tag")]`

Parse taking `count` bits and comparing them to `pattern`

# Effective Signature

Assuming you are parsing a `(&[u8], usize)` bit [Stream]:
```rust
# use winnow::prelude::*;;
# use winnow::error::ContextError;
pub fn pattern<'i>(pattern: u8, count: usize) -> impl Parser<(&'i [u8], usize), u8, ContextError>
# {
#     winnow::binary::bits::pattern(pattern, count)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::Bytes;
# use winnow::error::ContextError;
use winnow::binary::bits::pattern;

type Stream<'i> = &'i Bytes;

fn stream(b: &[u8]) -> Stream<'_> {
    Bytes::new(b)
}

/// Compare the lowest `count` bits of `input` against the lowest `count` bits of `pattern`.
/// Return Ok and the matching section of `input` if there's a match.
/// Return Err if there's no match.
fn parser(bits: u8, count: u8, input: &mut (Stream<'_>, usize)) -> ModalResult<u8> {
    pattern(bits, count).parse_next(input)
}

// The lowest 4 bits of 0b00001111 match the lowest 4 bits of 0b11111111.
assert_eq!(
    pattern::<_, usize, _, ContextError>(0b0000_1111, 4usize).parse_peek((stream(&[0b1111_1111]), 0)),
    Ok(((stream(&[0b1111_1111]), 4), 0b0000_1111))
);

// The lowest bit of 0b00001111 matches the lowest bit of 0b11111111 (both are 1).
assert_eq!(
    pattern::<_, usize, _, ContextError>(0b00000001, 1usize).parse_peek((stream(&[0b11111111]), 0)),
    Ok(((stream(&[0b11111111]), 1), 0b00000001))
);

// The lowest 2 bits of 0b11111111 and 0b00000001 are different.
assert!(pattern::<_, usize, _, ContextError>(0b000000_01, 2usize).parse_peek((stream(&[0b111111_11]), 0)).is_err());

// The lowest 8 bits of 0b11111111 and 0b11111110 are different.
assert!(pattern::<_, usize, _, ContextError>(0b11111110, 8usize).parse_peek((stream(&[0b11111111]), 0)).is_err());
```

```rust
pub fn pattern<Input, Output, Count, Error: ParserError<(Input, usize)>>(pattern: Output, count: Count) -> impl Parser<(Input, usize), Output, Error>
where
    Input: Stream<Token = u8> + StreamIsPartial + Clone,
    Count: ToUsize,
    Output: From<u8> + AddAssign + Shl<usize, Output = Output> + Shr<usize, Output = Output> + PartialEq { /* ... */ }
```

#### Function `bool`

**Attributes:**

- `#[doc(alias = "any")]`

Parses one specific bit as a bool.

# Effective Signature

Assuming you are parsing a `(&[u8], usize)` bit [Stream]:
```rust
# use winnow::prelude::*;;
# use winnow::error::ContextError;
pub fn bool(input: &mut (&[u8], usize)) -> ModalResult<bool>
# {
#     winnow::binary::bits::bool.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::Bytes;
# use winnow::error::InputError;
use winnow::binary::bits::bool;

type Stream<'i> = &'i Bytes;

fn stream(b: &[u8]) -> Stream<'_> {
    Bytes::new(b)
}

fn parse(input: &mut (Stream<'_>, usize)) -> ModalResult<bool> {
    bool.parse_next(input)
}

assert_eq!(parse.parse_peek((stream(&[0b10000000]), 0)), Ok(((stream(&[0b10000000]), 1), true)));
assert_eq!(parse.parse_peek((stream(&[0b10000000]), 1)), Ok(((stream(&[0b10000000]), 2), false)));
```

```rust
pub fn bool<Input, Error: ParserError<(Input, usize)>>(input: &mut (Input, usize)) -> crate::Result<bool, Error>
where
    Input: Stream<Token = u8> + StreamIsPartial + Clone { /* ... */ }
```

### Types

#### Enum `Endianness`

Configurable endianness

```rust
pub enum Endianness {
    Big,
    Little,
    Native,
}
```

##### Variants

###### `Big`

Big endian

###### `Little`

Little endian

###### `Native`

Will match the host's endianness

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
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
    fn eq(self: &Self, other: &Endianness) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Endianness { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Unpin**
- **Send**
- **Freeze**
### Functions

#### Function `be_u8`

**Attributes:**

- `#[inline(always)]`

Recognizes an unsigned 1 byte integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_u8;

fn parser(s: &mut &[u8]) -> ModalResult<u8> {
    be_u8.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
assert!(parser.parse_peek(&b""[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_u8;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u8> {
    be_u8.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01abcd"[..])), Ok((Partial::new(&b"\x01abcd"[..]), 0x00)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn be_u8<Input, Error>(input: &mut Input) -> crate::Result<u8, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_u16`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian unsigned 2 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_u16;

fn parser(s: &mut &[u8]) -> ModalResult<u16> {
    be_u16.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_u16;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u16> {
    be_u16.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x0001)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn be_u16<Input, Error>(input: &mut Input) -> crate::Result<u16, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_u24`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian unsigned 3 byte integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_u24;

fn parser(s: &mut &[u8]) -> ModalResult<u32> {
    be_u24.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_u24;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u32> {
    be_u24.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x000102)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(2))));
```

```rust
pub fn be_u24<Input, Error>(input: &mut Input) -> crate::Result<u32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_u32`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian unsigned 4 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_u32;

fn parser(s: &mut &[u8]) -> ModalResult<u32> {
    be_u32.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_u32;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u32> {
    be_u32.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x00010203)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(3))));
```

```rust
pub fn be_u32<Input, Error>(input: &mut Input) -> crate::Result<u32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_u64`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian unsigned 8 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_u64;

fn parser(s: &mut &[u8]) -> ModalResult<u64> {
    be_u64.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_u64;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u64> {
    be_u64.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x0001020304050607)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(7))));
```

```rust
pub fn be_u64<Input, Error>(input: &mut Input) -> crate::Result<u64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_u128`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian unsigned 16 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_u128;

fn parser(s: &mut &[u8]) -> ModalResult<u128> {
    be_u128.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_u128;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u128> {
    be_u128.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x00010203040506070809101112131415)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(15))));
```

```rust
pub fn be_u128<Input, Error>(input: &mut Input) -> crate::Result<u128, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_i8`

**Attributes:**

- `#[inline(always)]`

Recognizes a signed 1 byte integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_i8;

fn parser(s: &mut &[u8]) -> ModalResult<i8> {
    be_i8.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
assert!(parser.parse_peek(&b""[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_i8;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i8> {
      be_i8.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01abcd"[..])), Ok((Partial::new(&b"\x01abcd"[..]), 0x00)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn be_i8<Input, Error>(input: &mut Input) -> crate::Result<i8, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_i16`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian signed 2 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_i16;

fn parser(s: &mut &[u8]) -> ModalResult<i16> {
    be_i16.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_i16;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i16> {
      be_i16.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x0001)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(2))));
```

```rust
pub fn be_i16<Input, Error>(input: &mut Input) -> crate::Result<i16, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_i24`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian signed 3 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_i24;

fn parser(s: &mut &[u8]) -> ModalResult<i32> {
    be_i24.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_i24;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i32> {
      be_i24.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x000102)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(3))));
```

```rust
pub fn be_i24<Input, Error>(input: &mut Input) -> crate::Result<i32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_i32`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian signed 4 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_i32;

fn parser(s: &mut &[u8]) -> ModalResult<i32> {
      be_i32.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_i32;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i32> {
      be_i32.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x00010203)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(4))));
```

```rust
pub fn be_i32<Input, Error>(input: &mut Input) -> crate::Result<i32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_i64`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian signed 8 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_i64;

fn parser(s: &mut &[u8]) -> ModalResult<i64> {
      be_i64.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_i64;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i64> {
      be_i64.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x0001020304050607)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(7))));
```

```rust
pub fn be_i64<Input, Error>(input: &mut Input) -> crate::Result<i64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_i128`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian signed 16 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_i128;

fn parser(s: &mut &[u8]) -> ModalResult<i128> {
      be_i128.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_i128;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i128> {
      be_i128.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x00010203040506070809101112131415)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(15))));
```

```rust
pub fn be_i128<Input, Error>(input: &mut Input) -> crate::Result<i128, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_u8`

**Attributes:**

- `#[inline(always)]`

Recognizes an unsigned 1 byte integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_u8;

fn parser(s: &mut &[u8]) -> ModalResult<u8> {
      le_u8.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
assert!(parser.parse_peek(&b""[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_u8;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u8> {
      le_u8.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01abcd"[..])), Ok((Partial::new(&b"\x01abcd"[..]), 0x00)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn le_u8<Input, Error>(input: &mut Input) -> crate::Result<u8, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_u16`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian unsigned 2 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_u16;

fn parser(s: &mut &[u8]) -> ModalResult<u16> {
      le_u16.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_u16;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u16> {
      le_u16.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x0100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn le_u16<Input, Error>(input: &mut Input) -> crate::Result<u16, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_u24`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian unsigned 3 byte integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_u24;

fn parser(s: &mut &[u8]) -> ModalResult<u32> {
      le_u24.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_u24;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u32> {
      le_u24.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x020100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(2))));
```

```rust
pub fn le_u24<Input, Error>(input: &mut Input) -> crate::Result<u32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_u32`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian unsigned 4 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_u32;

fn parser(s: &mut &[u8]) -> ModalResult<u32> {
      le_u32.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_u32;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u32> {
      le_u32.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x03020100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(3))));
```

```rust
pub fn le_u32<Input, Error>(input: &mut Input) -> crate::Result<u32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_u64`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian unsigned 8 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_u64;

fn parser(s: &mut &[u8]) -> ModalResult<u64> {
      le_u64.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_u64;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u64> {
      le_u64.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x0706050403020100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(7))));
```

```rust
pub fn le_u64<Input, Error>(input: &mut Input) -> crate::Result<u64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_u128`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian unsigned 16 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_u128;

fn parser(s: &mut &[u8]) -> ModalResult<u128> {
      le_u128.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_u128;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u128> {
      le_u128.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x15141312111009080706050403020100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(15))));
```

```rust
pub fn le_u128<Input, Error>(input: &mut Input) -> crate::Result<u128, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_i8`

**Attributes:**

- `#[inline(always)]`

Recognizes a signed 1 byte integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_i8;

fn parser(s: &mut &[u8]) -> ModalResult<i8> {
      le_i8.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
assert!(parser.parse_peek(&b""[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_i8;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i8> {
      le_i8.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01abcd"[..])), Ok((Partial::new(&b"\x01abcd"[..]), 0x00)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn le_i8<Input, Error>(input: &mut Input) -> crate::Result<i8, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_i16`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian signed 2 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_i16;

fn parser(s: &mut &[u8]) -> ModalResult<i16> {
      le_i16.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_i16;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i16> {
      le_i16.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x0100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn le_i16<Input, Error>(input: &mut Input) -> crate::Result<i16, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_i24`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian signed 3 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_i24;

fn parser(s: &mut &[u8]) -> ModalResult<i32> {
      le_i24.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_i24;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i32> {
      le_i24.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x020100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(2))));
```

```rust
pub fn le_i24<Input, Error>(input: &mut Input) -> crate::Result<i32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_i32`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian signed 4 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_i32;

fn parser(s: &mut &[u8]) -> ModalResult<i32> {
      le_i32.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_i32;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i32> {
      le_i32.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x03020100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(3))));
```

```rust
pub fn le_i32<Input, Error>(input: &mut Input) -> crate::Result<i32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_i64`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian signed 8 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_i64;

fn parser(s: &mut &[u8]) -> ModalResult<i64> {
      le_i64.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_i64;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i64> {
      le_i64.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x0706050403020100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(7))));
```

```rust
pub fn le_i64<Input, Error>(input: &mut Input) -> crate::Result<i64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_i128`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian signed 16 bytes integer.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_i128;

fn parser(s: &mut &[u8]) -> ModalResult<i128> {
      le_i128.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
assert!(parser.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_i128;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i128> {
      le_i128.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..])), Ok((Partial::new(&b"abcd"[..]), 0x15141312111009080706050403020100)));
assert_eq!(parser.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(15))));
```

```rust
pub fn le_i128<Input, Error>(input: &mut Input) -> crate::Result<i128, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `u8`

**Attributes:**

- `#[inline(always)]`

Recognizes an unsigned 1 byte integer

<div class="warning">

**Note:** that endianness does not apply to 1 byte numbers.

</div>

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::u8;

fn parser(s: &mut &[u8]) -> ModalResult<u8> {
      u8.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
assert!(parser.parse_peek(&b""[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::u8;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<u8> {
      u8.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x03abcefg"[..])), Ok((Partial::new(&b"\x03abcefg"[..]), 0x00)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn u8<Input, Error>(input: &mut Input) -> crate::Result<u8, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `u16`

**Attributes:**

- `#[inline(always)]`

Recognizes an unsigned 2 bytes integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian u16 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian u16 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::u16;

fn be_u16(input: &mut &[u8]) -> ModalResult<u16> {
    u16(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u16.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
assert!(be_u16.parse_peek(&b"\x01"[..]).is_err());

fn le_u16(input: &mut &[u8]) -> ModalResult<u16> {
    u16(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u16.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
assert!(le_u16.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::u16;

fn be_u16(input: &mut Partial<&[u8]>) -> ModalResult<u16> {
    u16(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u16.parse_peek(Partial::new(&b"\x00\x03abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x0003)));
assert_eq!(be_u16.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(1))));

fn le_u16(input: &mut Partial<&[u8]>) -> ModalResult< u16> {
    u16(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u16.parse_peek(Partial::new(&b"\x00\x03abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x0300)));
assert_eq!(le_u16.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn u16<Input, Error>(endian: Endianness) -> impl Parser<Input, u16, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `u24`

**Attributes:**

- `#[inline(always)]`

Recognizes an unsigned 3 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian u24 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian u24 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::u24;

fn be_u24(input: &mut &[u8]) -> ModalResult<u32> {
    u24(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u24.parse_peek(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
assert!(be_u24.parse_peek(&b"\x01"[..]).is_err());

fn le_u24(input: &mut &[u8]) -> ModalResult<u32> {
    u24(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u24.parse_peek(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
assert!(le_u24.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::u24;

fn be_u24(input: &mut Partial<&[u8]>) -> ModalResult<u32> {
    u24(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u24.parse_peek(Partial::new(&b"\x00\x03\x05abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x000305)));
assert_eq!(be_u24.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(2))));

fn le_u24(input: &mut Partial<&[u8]>) -> ModalResult<u32> {
    u24(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u24.parse_peek(Partial::new(&b"\x00\x03\x05abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x050300)));
assert_eq!(le_u24.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(2))));
```

```rust
pub fn u24<Input, Error>(endian: Endianness) -> impl Parser<Input, u32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `u32`

**Attributes:**

- `#[inline(always)]`

Recognizes an unsigned 4 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian u32 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian u32 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::u32;

fn be_u32(input: &mut &[u8]) -> ModalResult<u32> {
    u32(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u32.parse_peek(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
assert!(be_u32.parse_peek(&b"\x01"[..]).is_err());

fn le_u32(input: &mut &[u8]) -> ModalResult<u32> {
    u32(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u32.parse_peek(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
assert!(le_u32.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::u32;

fn be_u32(input: &mut Partial<&[u8]>) -> ModalResult<u32> {
    u32(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u32.parse_peek(Partial::new(&b"\x00\x03\x05\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x00030507)));
assert_eq!(be_u32.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(3))));

fn le_u32(input: &mut Partial<&[u8]>) -> ModalResult<u32> {
    u32(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u32.parse_peek(Partial::new(&b"\x00\x03\x05\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x07050300)));
assert_eq!(le_u32.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(3))));
```

```rust
pub fn u32<Input, Error>(endian: Endianness) -> impl Parser<Input, u32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `u64`

**Attributes:**

- `#[inline(always)]`

Recognizes an unsigned 8 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian u64 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian u64 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::u64;

fn be_u64(input: &mut &[u8]) -> ModalResult<u64> {
    u64(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u64.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
assert!(be_u64.parse_peek(&b"\x01"[..]).is_err());

fn le_u64(input: &mut &[u8]) -> ModalResult<u64> {
    u64(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u64.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
assert!(le_u64.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::u64;

fn be_u64(input: &mut Partial<&[u8]>) -> ModalResult<u64> {
    u64(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u64.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x0001020304050607)));
assert_eq!(be_u64.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(7))));

fn le_u64(input: &mut Partial<&[u8]>) -> ModalResult<u64> {
    u64(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u64.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x0706050403020100)));
assert_eq!(le_u64.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(7))));
```

```rust
pub fn u64<Input, Error>(endian: Endianness) -> impl Parser<Input, u64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `u128`

**Attributes:**

- `#[inline(always)]`

Recognizes an unsigned 16 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian u128 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian u128 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::u128;

fn be_u128(input: &mut &[u8]) -> ModalResult<u128> {
    u128(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u128.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
assert!(be_u128.parse_peek(&b"\x01"[..]).is_err());

fn le_u128(input: &mut &[u8]) -> ModalResult<u128> {
    u128(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u128.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
assert!(le_u128.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::u128;

fn be_u128(input: &mut Partial<&[u8]>) -> ModalResult<u128> {
    u128(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_u128.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x00010203040506070001020304050607)));
assert_eq!(be_u128.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(15))));

fn le_u128(input: &mut Partial<&[u8]>) -> ModalResult<u128> {
    u128(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_u128.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x07060504030201000706050403020100)));
assert_eq!(le_u128.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(15))));
```

```rust
pub fn u128<Input, Error>(endian: Endianness) -> impl Parser<Input, u128, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `i8`

**Attributes:**

- `#[inline(always)]`

Recognizes a signed 1 byte integer

<div class="warning">

**Note:** that endianness does not apply to 1 byte numbers.

</div>

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::i8;

fn parser(s: &mut &[u8]) -> ModalResult<i8> {
      i8.parse_next(s)
}

assert_eq!(parser.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
assert!(parser.parse_peek(&b""[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::i8;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<i8> {
      i8.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&b"\x00\x03abcefg"[..])), Ok((Partial::new(&b"\x03abcefg"[..]), 0x00)));
assert_eq!(parser.parse_peek(Partial::new(&b""[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn i8<Input, Error>(input: &mut Input) -> crate::Result<i8, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `i16`

**Attributes:**

- `#[inline(always)]`

Recognizes a signed 2 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian i16 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian i16 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::i16;

fn be_i16(input: &mut &[u8]) -> ModalResult<i16> {
    i16(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i16.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
assert!(be_i16.parse_peek(&b"\x01"[..]).is_err());

fn le_i16(input: &mut &[u8]) -> ModalResult<i16> {
    i16(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i16.parse_peek(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
assert!(le_i16.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::i16;

fn be_i16(input: &mut Partial<&[u8]>) -> ModalResult<i16> {
    i16(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i16.parse_peek(Partial::new(&b"\x00\x03abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x0003)));
assert_eq!(be_i16.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(1))));

fn le_i16(input: &mut Partial<&[u8]>) -> ModalResult<i16> {
    i16(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i16.parse_peek(Partial::new(&b"\x00\x03abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x0300)));
assert_eq!(le_i16.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn i16<Input, Error>(endian: Endianness) -> impl Parser<Input, i16, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `i24`

**Attributes:**

- `#[inline(always)]`

Recognizes a signed 3 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian i24 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian i24 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::i24;

fn be_i24(input: &mut &[u8]) -> ModalResult<i32> {
    i24(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i24.parse_peek(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
assert!(be_i24.parse_peek(&b"\x01"[..]).is_err());

fn le_i24(input: &mut &[u8]) -> ModalResult<i32> {
    i24(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i24.parse_peek(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
assert!(le_i24.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::i24;

fn be_i24(input: &mut Partial<&[u8]>) -> ModalResult<i32> {
    i24(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i24.parse_peek(Partial::new(&b"\x00\x03\x05abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x000305)));
assert_eq!(be_i24.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(2))));

fn le_i24(input: &mut Partial<&[u8]>) -> ModalResult<i32> {
    i24(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i24.parse_peek(Partial::new(&b"\x00\x03\x05abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x050300)));
assert_eq!(le_i24.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(2))));
```

```rust
pub fn i24<Input, Error>(endian: Endianness) -> impl Parser<Input, i32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `i32`

**Attributes:**

- `#[inline(always)]`

Recognizes a signed 4 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian i32 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian i32 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::i32;

fn be_i32(input: &mut &[u8]) -> ModalResult<i32> {
    i32(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i32.parse_peek(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
assert!(be_i32.parse_peek(&b"\x01"[..]).is_err());

fn le_i32(input: &mut &[u8]) -> ModalResult<i32> {
    i32(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i32.parse_peek(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
assert!(le_i32.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::i32;

fn be_i32(input: &mut Partial<&[u8]>) -> ModalResult<i32> {
    i32(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i32.parse_peek(Partial::new(&b"\x00\x03\x05\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x00030507)));
assert_eq!(be_i32.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(3))));

fn le_i32(input: &mut Partial<&[u8]>) -> ModalResult<i32> {
    i32(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i32.parse_peek(Partial::new(&b"\x00\x03\x05\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x07050300)));
assert_eq!(le_i32.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(3))));
```

```rust
pub fn i32<Input, Error>(endian: Endianness) -> impl Parser<Input, i32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `i64`

**Attributes:**

- `#[inline(always)]`

Recognizes a signed 8 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian i64 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian i64 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::i64;

fn be_i64(input: &mut &[u8]) -> ModalResult<i64> {
    i64(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i64.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
assert!(be_i64.parse_peek(&b"\x01"[..]).is_err());

fn le_i64(input: &mut &[u8]) -> ModalResult<i64> {
    i64(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i64.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
assert!(le_i64.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::i64;

fn be_i64(input: &mut Partial<&[u8]>) -> ModalResult<i64> {
    i64(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i64.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x0001020304050607)));
assert_eq!(be_i64.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(7))));

fn le_i64(input: &mut Partial<&[u8]>) -> ModalResult<i64> {
    i64(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i64.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x0706050403020100)));
assert_eq!(le_i64.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(7))));
```

```rust
pub fn i64<Input, Error>(endian: Endianness) -> impl Parser<Input, i64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `i128`

**Attributes:**

- `#[inline(always)]`

Recognizes a signed 16 byte integer

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian i128 integer,
otherwise if `winnow::binary::Endianness::Little` parse a little endian i128 integer.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::i128;

fn be_i128(input: &mut &[u8]) -> ModalResult<i128> {
    i128(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i128.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
assert!(be_i128.parse_peek(&b"\x01"[..]).is_err());

fn le_i128(input: &mut &[u8]) -> ModalResult<i128> {
    i128(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i128.parse_peek(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
assert!(le_i128.parse_peek(&b"\x01"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::i128;

fn be_i128(input: &mut Partial<&[u8]>) -> ModalResult<i128> {
    i128(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_i128.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x00010203040506070001020304050607)));
assert_eq!(be_i128.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(15))));

fn le_i128(input: &mut Partial<&[u8]>) -> ModalResult<i128> {
    i128(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_i128.parse_peek(Partial::new(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..])), Ok((Partial::new(&b"abcefg"[..]), 0x07060504030201000706050403020100)));
assert_eq!(le_i128.parse_peek(Partial::new(&b"\x01"[..])), Err(ErrMode::Incomplete(Needed::new(15))));
```

```rust
pub fn i128<Input, Error>(endian: Endianness) -> impl Parser<Input, i128, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_f32`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian 4 bytes floating point number.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_f32;

fn parser(s: &mut &[u8]) -> ModalResult<f32> {
      be_f32.parse_next(s)
}

assert_eq!(parser.parse_peek(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
assert!(parser.parse_peek(&b"abc"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_f32;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<f32> {
      be_f32.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&[0x40, 0x29, 0x00, 0x00][..])), Ok((Partial::new(&b""[..]), 2.640625)));
assert_eq!(parser.parse_peek(Partial::new(&[0x01][..])), Err(ErrMode::Incomplete(Needed::new(3))));
```

```rust
pub fn be_f32<Input, Error>(input: &mut Input) -> crate::Result<f32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `be_f64`

**Attributes:**

- `#[inline(always)]`

Recognizes a big endian 8 bytes floating point number.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::be_f64;

fn parser(s: &mut &[u8]) -> ModalResult<f64> {
      be_f64.parse_next(s)
}

assert_eq!(parser.parse_peek(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
assert!(parser.parse_peek(&b"abc"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::be_f64;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<f64> {
      be_f64.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..])), Ok((Partial::new(&b""[..]), 12.5)));
assert_eq!(parser.parse_peek(Partial::new(&[0x01][..])), Err(ErrMode::Incomplete(Needed::new(7))));
```

```rust
pub fn be_f64<Input, Error>(input: &mut Input) -> crate::Result<f64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_f32`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian 4 bytes floating point number.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_f32;

fn parser(s: &mut &[u8]) -> ModalResult<f32> {
      le_f32.parse_next(s)
}

assert_eq!(parser.parse_peek(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 12.5)));
assert!(parser.parse_peek(&b"abc"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_f32;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<f32> {
      le_f32.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&[0x00, 0x00, 0x48, 0x41][..])), Ok((Partial::new(&b""[..]), 12.5)));
assert_eq!(parser.parse_peek(Partial::new(&[0x01][..])), Err(ErrMode::Incomplete(Needed::new(3))));
```

```rust
pub fn le_f32<Input, Error>(input: &mut Input) -> crate::Result<f32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `le_f64`

**Attributes:**

- `#[inline(always)]`

Recognizes a little endian 8 bytes floating point number.

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::le_f64;

fn parser(s: &mut &[u8]) -> ModalResult<f64> {
      le_f64.parse_next(s)
}

assert_eq!(parser.parse_peek(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..]), Ok((&b""[..], 12.5)));
assert!(parser.parse_peek(&b"abc"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::binary::le_f64;

fn parser(s: &mut Partial<&[u8]>) -> ModalResult<f64> {
      le_f64.parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x41][..])), Ok((Partial::new(&b""[..]), 3145728.0)));
assert_eq!(parser.parse_peek(Partial::new(&[0x01][..])), Err(ErrMode::Incomplete(Needed::new(7))));
```

```rust
pub fn le_f64<Input, Error>(input: &mut Input) -> crate::Result<f64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `f32`

**Attributes:**

- `#[inline(always)]`

Recognizes a 4 byte floating point number

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian f32 float,
otherwise if `winnow::binary::Endianness::Little` parse a little endian f32 float.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::f32;

fn be_f32(input: &mut &[u8]) -> ModalResult<f32> {
    f32(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_f32.parse_peek(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
assert!(be_f32.parse_peek(&b"abc"[..]).is_err());

fn le_f32(input: &mut &[u8]) -> ModalResult<f32> {
    f32(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_f32.parse_peek(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 12.5)));
assert!(le_f32.parse_peek(&b"abc"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::f32;

fn be_f32(input: &mut Partial<&[u8]>) -> ModalResult<f32> {
    f32(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_f32.parse_peek(Partial::new(&[0x41, 0x48, 0x00, 0x00][..])), Ok((Partial::new(&b""[..]), 12.5)));
assert_eq!(be_f32.parse_peek(Partial::new(&b"abc"[..])), Err(ErrMode::Incomplete(Needed::new(1))));

fn le_f32(input: &mut Partial<&[u8]>) -> ModalResult<f32> {
    f32(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_f32.parse_peek(Partial::new(&[0x00, 0x00, 0x48, 0x41][..])), Ok((Partial::new(&b""[..]), 12.5)));
assert_eq!(le_f32.parse_peek(Partial::new(&b"abc"[..])), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn f32<Input, Error>(endian: Endianness) -> impl Parser<Input, f32, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `f64`

**Attributes:**

- `#[inline(always)]`

Recognizes an 8 byte floating point number

If the parameter is `winnow::binary::Endianness::Big`, parse a big endian f64 float,
otherwise if `winnow::binary::Endianness::Little` parse a little endian f64 float.

*Complete version*: returns an error if there is not enough input data

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
use winnow::binary::f64;

fn be_f64(input: &mut &[u8]) -> ModalResult<f64> {
    f64(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_f64.parse_peek(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
assert!(be_f64.parse_peek(&b"abc"[..]).is_err());

fn le_f64(input: &mut &[u8]) -> ModalResult<f64> {
    f64(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_f64.parse_peek(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..]), Ok((&b""[..], 12.5)));
assert!(le_f64.parse_peek(&b"abc"[..]).is_err());
```

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
# use winnow::error::Needed::Size;
# use winnow::Partial;
use winnow::binary::f64;

fn be_f64(input: &mut Partial<&[u8]>) -> ModalResult<f64> {
    f64(winnow::binary::Endianness::Big).parse_next(input)
};

assert_eq!(be_f64.parse_peek(Partial::new(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..])), Ok((Partial::new(&b""[..]), 12.5)));
assert_eq!(be_f64.parse_peek(Partial::new(&b"abc"[..])), Err(ErrMode::Incomplete(Needed::new(5))));

fn le_f64(input: &mut Partial<&[u8]>) -> ModalResult<f64> {
    f64(winnow::binary::Endianness::Little).parse_next(input)
};

assert_eq!(le_f64.parse_peek(Partial::new(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..])), Ok((Partial::new(&b""[..]), 12.5)));
assert_eq!(le_f64.parse_peek(Partial::new(&b"abc"[..])), Err(ErrMode::Incomplete(Needed::new(5))));
```

```rust
pub fn f64<Input, Error>(endian: Endianness) -> impl Parser<Input, f64, Error>
where
    Input: StreamIsPartial + Stream<Token = u8>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `length_take`

Get a length-prefixed slice ([TLV](https://en.wikipedia.org/wiki/Type-length-value))

To apply a parser to the returned slice, see [`length_and_then`].

If the count is for something besides tokens, see [`length_repeat`].

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::Needed, stream::Partial};
# use winnow::prelude::*;
use winnow::Bytes;
use winnow::binary::be_u16;
use winnow::binary::length_take;

type Stream<'i> = Partial<&'i Bytes>;

fn stream(b: &[u8]) -> Stream<'_> {
    Partial::new(Bytes::new(b))
}

fn parser<'i>(s: &mut Stream<'i>) -> ModalResult<&'i [u8]> {
  length_take(be_u16).parse_next(s)
}

assert_eq!(parser.parse_peek(stream(b"\x00\x03abcefg")), Ok((stream(&b"efg"[..]), &b"abc"[..])));
assert_eq!(parser.parse_peek(stream(b"\x00\x03a")), Err(ErrMode::Incomplete(Needed::new(2))));
```

```rust
pub fn length_take<Input, Count, Error, CountParser>(count: CountParser) -> impl Parser<Input, <Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    Count: ToUsize,
    CountParser: Parser<Input, Count, Error>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `length_and_then`

Parse a length-prefixed slice ([TLV](https://en.wikipedia.org/wiki/Type-length-value))

*Complete version*: Returns an error if there is not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there is not enough data.

# Example

```rust
# use winnow::{error::ErrMode, error::InputError, error::Needed, stream::{Partial, StreamIsPartial}};
# use winnow::prelude::*;
use winnow::Bytes;
use winnow::binary::be_u16;
use winnow::binary::length_and_then;

type Stream<'i> = Partial<&'i Bytes>;

fn stream(b: &[u8]) -> Stream<'_> {
    Partial::new(Bytes::new(b))
}

fn complete_stream(b: &[u8]) -> Stream<'_> {
    let mut p = Partial::new(Bytes::new(b));
    let _ = p.complete();
    p
}

fn parser<'i>(s: &mut Stream<'i>) -> ModalResult<&'i [u8]> {
  length_and_then(be_u16, "abc").parse_next(s)
}

assert_eq!(parser.parse_peek(stream(b"\x00\x03abcefg")), Ok((stream(&b"efg"[..]), &b"abc"[..])));
assert!(parser.parse_peek(stream(b"\x00\x03123123")).is_err());
assert_eq!(parser.parse_peek(stream(b"\x00\x03a")), Err(ErrMode::Incomplete(Needed::new(2))));
```

```rust
pub fn length_and_then<Input, Output, Count, Error, CountParser, ParseNext>(count: CountParser, parser: ParseNext) -> impl Parser<Input, Output, Error>
where
    Input: StreamIsPartial + Stream + UpdateSlice + Clone,
    Count: ToUsize,
    CountParser: Parser<Input, Count, Error>,
    ParseNext: Parser<Input, Output, Error>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `length_repeat`

[`Accumulate`] a length-prefixed sequence of values ([TLV](https://en.wikipedia.org/wiki/Type-length-value))

If the length represents token counts, see instead [`length_take`]

# Example

```rust
# #[cfg(feature = "std")] {
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::InputError, error::Needed};
# use winnow::prelude::*;
use winnow::Bytes;
use winnow::binary::u8;
use winnow::binary::length_repeat;

type Stream<'i> = &'i Bytes;

fn stream(b: &[u8]) -> Stream<'_> {
    Bytes::new(b)
}

fn parser<'i>(s: &mut Stream<'i>) -> ModalResult<Vec<&'i [u8]>> {
  length_repeat(u8.map(|i| {
     println!("got number: {}", i);
     i
  }), "abc").parse_next(s)
}

assert_eq!(parser.parse_peek(stream(b"\x02abcabcabc")), Ok((stream(b"abc"), vec![&b"abc"[..], &b"abc"[..]])));
assert!(parser.parse_peek(stream(b"\x03123123123")).is_err());
# }
```

```rust
pub fn length_repeat<Input, Output, Accumulator, Count, Error, CountParser, ParseNext>(count: CountParser, parser: ParseNext) -> impl Parser<Input, Accumulator, Error>
where
    Input: Stream,
    Count: ToUsize,
    Accumulator: Accumulate<Output>,
    CountParser: Parser<Input, Count, Error>,
    ParseNext: Parser<Input, Output, Error>,
    Error: ParserError<Input> { /* ... */ }
```

## Module `combinator`

# List of parsers and combinators

<div class="warning">

**Note**: this list is meant to provide a nicer way to find a parser than reading through the documentation on docs.rs. Function combinators are organized in module so they are a bit easier to find.

</div>

## Basic elements

Those are used to take a series of tokens for the lowest level elements of your grammar, like, "here is a dot", or "here is an big endian integer".

| combinator | usage | input | new input | output | comment |
|---|---|---|---|---|---|
| [`one_of`][crate::token::one_of] | `one_of(['a', 'b', 'c'])` |  `"abc"` |  `"bc"` | `Ok('a')` |Matches one of the provided [set of tokens][crate::stream::ContainsToken] (works with non ASCII characters too)|
| [`none_of`][crate::token::none_of] | `none_of(['a', 'b', 'c'])` |  `"xyab"` |  `"yab"` | `Ok('x')` |Matches anything but one of the provided [set of tokens][crate::stream::ContainsToken]|
| [`literal`][crate::token::literal] | `"hello"` |  `"hello world"` |  `" world"` | `Ok("hello")` |Recognizes a specific suite of characters or bytes (see also [`Caseless`][crate::ascii::Caseless])|
| [`take`][crate::token::take] | `take(4)` |  `"hello"` |  `"o"` | `Ok("hell")` |Takes a specific number of bytes or characters|
| [`take_while`][crate::token::take_while] | `take_while(0.., is_alphabetic)` |  `"abc123"` |  `"123"` | `Ok("abc")` |Returns the longest slice of bytes or characters for which the provided [set of tokens][crate::stream::ContainsToken] matches.|
| [`take_till`][crate::token::take_till] | `take_till(0.., is_alphabetic)` |  `"123abc"` |  `"abc"` | `Ok("123")` |Returns a slice of bytes or characters until the provided [set of tokens][crate::stream::ContainsToken] matches. This is the reverse behaviour from `take_while`: `take_till(f)` is equivalent to `take_while(0.., \|c\| !f(c))`|
| [`take_until`][crate::token::take_until] | `take_until(0.., "world")` |  `"Hello world"` |  `"world"` | `Ok("Hello ")` |Returns a slice of bytes or characters until the provided [literal][crate::token::literal] is found.|

## Choice combinators

| combinator | usage | input | new input | output | comment |
|---|---|---|---|---|---|
| [`alt`] | `alt(("ab", "cd"))` |  `"cdef"` |  `"ef"` | `Ok("cd")` |Try a list of parsers and return the result of the first successful one|
| [`dispatch`] | \- | \- | \- | \- | `match` for parsers |
| [`permutation`] | `permutation(("ab", "cd", "12"))` | `"cd12abc"` | `"c"` | `Ok(("ab", "cd", "12"))` |Succeeds when all its child parser have succeeded, whatever the order|

## Sequence combinators

| combinator | usage | input | new input | output | comment |
|---|---|---|---|---|---|
| [`(...)` (tuples)][crate::Parser] | `("ab", "XY", take(1))` | `"abXYZ!"` | `"!"` | `Ok(("ab", "XY", "Z"))` |Parse a series of values|
| [`seq!`] | `seq!(_: '(', take(2), _: ')')` | `"(ab)cd"` | `"cd"` | `Ok("ab")` |Parse a series of values, discarding those you specify|
| [`delimited`] | `delimited('(', take(2), ')')` | `"(ab)cd"` | `"cd"` | `Ok("ab")` |Parse three values, discarding the first and third value|
| [`preceded`] | `preceded("ab", "XY")` | `"abXYZ"` | `"Z"` | `Ok("XY")` |Parse two values, discarding the first value|
| [`terminated`] | `terminated("ab", "XY")` | `"abXYZ"` | `"Z"` | `Ok("ab")` |Parse two values, discarding the second value|
| [`separated_pair`] | `separated_pair("hello", ',', "world")` | `"hello,world!"` | `"!"` | `Ok(("hello", "world"))` | Parse three values, discarding the middle value|

## Applying a parser multiple times

| combinator | usage | input | new input | output | comment |
|---|---|---|---|---|---|
| [`repeat`] | `repeat(1..=3, "ab")` | `"ababc"` | `"c"` | `Ok(vec!["ab", "ab"])` |Applies the parser between m and n times (n included) and returns the list of results in a Vec|
| [`repeat_till`] | `repeat_till(0.., "ab", "ef")` | `"ababefg"` | `"g"` | `Ok((vec!["ab", "ab"], "ef"))` |Applies the first parser until the second applies. Returns a tuple containing the list of results from the first in a Vec and the result of the second|
| [`separated`] | `separated(1..=3, "ab", ",")` | `"ab,ab,ab."` | `"."` | `Ok(vec!["ab", "ab", "ab"])` |Applies the parser and separator between m and n times (n included) and returns the list of results in a Vec|
| [`Repeat::fold`] | <code>repeat(1..=2, `be_u8`).fold(\|\| 0, \|acc, item\| acc + item)</code> | `[1, 2, 3]` | `[3]` | `Ok(3)` |Applies the parser between m and n times (n included) and folds the list of return value|

## Partial related

- [`eof`]: Returns its input if it is at the end of input data
- [`Parser::complete_err`]: Replaces an `Incomplete` returned by the child parser with an `Backtrack`

## Modifiers

- [`cond`]: Conditional combinator. Wraps another parser and calls it if the condition is met
- [`Parser::flat_map`]: method to map a new parser from the output of the first parser, then apply that parser over the rest of the input
- [`Parser::value`]: method to replace the result of a parser
- [`Parser::default_value`]: method to replace the result of a parser
- [`Parser::void`]: method to discard the result of a parser
- [`Parser::map`]: method to map a function on the result of a parser
- [`Parser::and_then`]: Applies a second parser over the output of the first one
- [`Parser::verify_map`]: Maps a function returning an `Option` on the output of a parser
- [`Parser::try_map`]: Maps a function returning a `Result` on the output of a parser
- [`Parser::parse_to`]: Apply [`std::str::FromStr`] to the output of the parser
- [`not`]: Returns a result only if the embedded parser returns `Backtrack` or `Incomplete`. Does not consume the input
- [`opt`]: Make the underlying parser optional
- [`peek`]: Returns a result without consuming the input
- [`Parser::take`]: If the child parser was successful, return the consumed input as the produced value
- [`Parser::with_taken`]: If the child parser was successful, return a tuple of the consumed input and the produced output.
- [`Parser::span`]: If the child parser was successful, return the location of the consumed input as the produced value
- [`Parser::with_span`]: If the child parser was successful, return a tuple of the location of the consumed input and the produced output.
- [`Parser::verify`]: Returns the result of the child parser if it satisfies a verification function

## Error management and debugging

- [`cut_err`]: Commit the parse result, disallowing alternative parsers from being attempted
- [`backtrack_err`]: Attempts a parse, allowing alternative parsers to be attempted despite
  use of `cut_err`
- [`Parser::context`]: Add context to the error if the parser fails
- [`trace`]: Print the parse state with the `debug` feature flag
- [`todo()`]: Placeholder parser

## Remaining combinators

- [`empty`]: Succeed, consuming no input
- [`fail`]: Inversion of [`empty`]. Always fails.
- [`Parser::by_ref`]: Allow moving `&mut impl Parser` into other parsers

## Text parsing

- [`any`][crate::token::any]: Matches one token
- [`tab`][crate::ascii::tab]: Matches a tab character `\t`
- [`crlf`][crate::ascii::crlf]: Recognizes the string `\r\n`
- [`line_ending`][crate::ascii::line_ending]: Recognizes an end of line (both `\n` and `\r\n`)
- [`newline`][crate::ascii::newline]: Matches a newline character `\n`
- [`till_line_ending`][crate::ascii::till_line_ending]: Recognizes a string of any char except `\r` or `\n`
- [`rest`][crate::token::rest]: Return the remaining input

- [`alpha0`][crate::ascii::alpha0]: Recognizes zero or more lowercase and uppercase alphabetic characters: `[a-zA-Z]`. [`alpha1`][crate::ascii::alpha1] does the same but returns at least one character
- [`alphanumeric0`][crate::ascii::alphanumeric0]: Recognizes zero or more numerical and alphabetic characters: `[0-9a-zA-Z]`. [`alphanumeric1`][crate::ascii::alphanumeric1] does the same but returns at least one character
- [`space0`][crate::ascii::space0]: Recognizes zero or more spaces and tabs. [`space1`][crate::ascii::space1] does the same but returns at least one character
- [`multispace0`][crate::ascii::multispace0]: Recognizes zero or more spaces, tabs, carriage returns and line feeds. [`multispace1`][crate::ascii::multispace1] does the same but returns at least one character
- [`digit0`][crate::ascii::digit0]: Recognizes zero or more numerical characters: `[0-9]`. [`digit1`][crate::ascii::digit1] does the same but returns at least one character
- [`hex_digit0`][crate::ascii::hex_digit0]: Recognizes zero or more hexadecimal numerical characters: `[0-9A-Fa-f]`. [`hex_digit1`][crate::ascii::hex_digit1] does the same but returns at least one character
- [`oct_digit0`][crate::ascii::oct_digit0]: Recognizes zero or more octal characters: `[0-7]`. [`oct_digit1`][crate::ascii::oct_digit1] does the same but returns at least one character

- [`float`][crate::ascii::float]: Parse a floating point number in a byte string
- [`dec_int`][crate::ascii::dec_int]: Decode a variable-width, decimal signed integer
- [`dec_uint`][crate::ascii::dec_uint]: Decode a variable-width, decimal unsigned integer
- [`hex_uint`][crate::ascii::hex_uint]: Decode a variable-width, hexadecimal integer

- [`take_escaped`][crate::ascii::take_escaped]: Recognize the input slice with escaped characters
- [`escaped_transform`][crate::ascii::escaped_transform]: Parse escaped characters, unescaping them

### Character test functions

Use these functions with a combinator like `take_while`:

- [`AsChar::is_alpha`][crate::stream::AsChar::is_alpha]: Tests if byte is ASCII alphabetic: `[A-Za-z]`
- [`AsChar::is_alphanum`][crate::stream::AsChar::is_alphanum]: Tests if byte is ASCII alphanumeric: `[A-Za-z0-9]`
- [`AsChar::is_dec_digit`][crate::stream::AsChar::is_dec_digit]: Tests if byte is ASCII digit: `[0-9]`
- [`AsChar::is_hex_digit`][crate::stream::AsChar::is_hex_digit]: Tests if byte is ASCII hex digit: `[0-9A-Fa-f]`
- [`AsChar::is_oct_digit`][crate::stream::AsChar::is_oct_digit]: Tests if byte is ASCII octal digit: `[0-7]`
- [`AsChar::is_space`][crate::stream::AsChar::is_space]: Tests if byte is ASCII space or tab: `[ \t]`
- [`AsChar::is_newline`][crate::stream::AsChar::is_newline]: Tests if byte is ASCII newline: `[\n]`

## Binary format parsing

- [`length_repeat`][crate::binary::length_repeat] Gets a number from the first parser, then applies the second parser that many times
- [`length_take`][crate::binary::length_take]: Gets a number from the first parser, then takes a subslice of the input of that size, and returns that subslice
- [`length_and_then`][crate::binary::length_and_then]: Gets a number from the first parser, takes a subslice of the input of that size, then applies the second parser on that subslice. If the second parser returns `Incomplete`, `length_value` will return an error

### Integers

Parsing integers from binary formats can be done in two ways: With parser functions, or combinators with configurable endianness.

- **configurable endianness:** [`i16`][crate::binary::i16], [`i32`][crate::binary::i32],
  [`i64`][crate::binary::i64], [`u16`][crate::binary::u16], [`u32`][crate::binary::u32],
  [`u64`][crate::binary::u64] are combinators that take as argument a
  [`winnow::binary::Endianness`][crate::binary::Endianness], like this: `i16(endianness)`. If the
  parameter is `winnow::binary::Endianness::Big`, parse a big endian `i16` integer, otherwise a
  little endian `i16` integer.
- **fixed endianness**: The functions are prefixed by `be_` for big endian numbers, and by `le_` for little endian numbers, and the suffix is the type they parse to. As an example, `be_u32` parses a big endian unsigned integer stored in 32 bits.
  - [`be_f32`][crate::binary::be_f32], [`be_f64`][crate::binary::be_f64]: Big endian floating point numbers
  - [`le_f32`][crate::binary::le_f32], [`le_f64`][crate::binary::le_f64]: Little endian floating point numbers
  - [`be_i8`][crate::binary::be_i8], [`be_i16`][crate::binary::be_i16], [`be_i24`][crate::binary::be_i24], [`be_i32`][crate::binary::be_i32], [`be_i64`][crate::binary::be_i64], [`be_i128`][crate::binary::be_i128]: Big endian signed integers
  - [`be_u8`][crate::binary::be_u8], [`be_u16`][crate::binary::be_u16], [`be_u24`][crate::binary::be_u24], [`be_u32`][crate::binary::be_u32], [`be_u64`][crate::binary::be_u64], [`be_u128`][crate::binary::be_u128]: Big endian unsigned integers
  - [`le_i8`][crate::binary::le_i8], [`le_i16`][crate::binary::le_i16], [`le_i24`][crate::binary::le_i24], [`le_i32`][crate::binary::le_i32], [`le_i64`][crate::binary::le_i64], [`le_i128`][crate::binary::le_i128]: Little endian signed integers
  - [`le_u8`][crate::binary::le_u8], [`le_u16`][crate::binary::le_u16], [`le_u24`][crate::binary::le_u24], [`le_u32`][crate::binary::le_u32], [`le_u64`][crate::binary::le_u64], [`le_u128`][crate::binary::le_u128]: Little endian unsigned integers

### Bit stream parsing

- [`bits`][crate::binary::bits::bits]: Transforms the current input type (byte slice `&[u8]`) to a bit stream on which bit specific parsers and more general combinators can be applied
- [`bytes`][crate::binary::bits::bytes]: Transforms its bits stream input back into a byte slice for the underlying parser
- [`take`][crate::binary::bits::take]: Take a set number of bits
- [`pattern`][crate::binary::bits::pattern]: Check if a set number of bits matches a pattern
- [`bool`][crate::binary::bits::bool]: Match any one bit

```rust
pub mod combinator { /* ... */ }
```

### Modules

## Module `impls`

Opaque implementations of [`Parser`]

```rust
pub mod impls { /* ... */ }
```

### Types

#### Struct `ByRef`

[`Parser`] implementation for [`Parser::by_ref`]

```rust
pub struct ByRef<''p, P, I, O, E> {
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **ModalParser**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<O, E> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Map`

[`Parser`] implementation for [`Parser::map`]

```rust
pub struct Map<F, G, I, O, O2, E> {
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

- **UnwindSafe**
- **RefUnwindSafe**
- **ModalParser**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<O2, E> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Unpin**
#### Struct `TryMap`

[`Parser`] implementation for [`Parser::try_map`]

```rust
pub struct TryMap<F, G, I, O, O2, E, E2> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ModalParser**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
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

- **Freeze**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<O2, E> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `VerifyMap`

[`Parser`] implementation for [`Parser::verify_map`]

```rust
pub struct VerifyMap<F, G, I, O, O2, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<O2, E> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **ModalParser**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Freeze**
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

- **Unpin**
#### Struct `AndThen`

[`Parser`] implementation for [`Parser::and_then`]

```rust
pub struct AndThen<F, G, I, O, O2, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<O2, E> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
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

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ModalParser**
#### Struct `ParseTo`

[`Parser`] implementation for [`Parser::parse_to`]

```rust
pub struct ParseTo<P, I, O, O2, E> {
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<O2, E> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ModalParser**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
#### Struct `FlatMap`

[`Parser`] implementation for [`Parser::flat_map`]

```rust
pub struct FlatMap<F, G, H, I, O, O2, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ModalParser**
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

- **Send**
- **Unpin**
- **UnwindSafe**
- **Sync**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<O2, E> { /* ... */ }
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

#### Struct `CompleteErr`

[`Parser`] implementation for [`Parser::complete_err`]

```rust
pub struct CompleteErr<P, I, O, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<O, E> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **ModalParser**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Verify`

[`Parser`] implementation for [`Parser::verify`]

```rust
pub struct Verify<F, G, I, O, O2, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

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
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
- **Unpin**
- **ModalParser**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<O, E> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Value`

[`Parser`] implementation for [`Parser::value`]

```rust
pub struct Value<F, I, O, O2, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **ModalParser**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Send**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<O2, E> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `DefaultValue`

[`Parser`] implementation for [`Parser::default_value`]

```rust
pub struct DefaultValue<F, I, O, O2, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **ModalParser**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **RefUnwindSafe**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<O2, E> { /* ... */ }
    ```

#### Struct `Void`

[`Parser`] implementation for [`Parser::void`]

```rust
pub struct Void<F, I, O, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ModalParser**
- **UnwindSafe**
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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<(), E> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
#### Struct `Take`

[`Parser`] implementation for [`Parser::take`]

```rust
pub struct Take<F, I, O, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<<I as Stream>::Slice, E> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ModalParser**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `WithTaken`

[`Parser`] implementation for [`Parser::with_taken`]

```rust
pub struct WithTaken<F, I, O, E> {
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ModalParser**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<(O, <I as Stream>::Slice), E> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Span`

[`Parser`] implementation for [`Parser::span`]

```rust
pub struct Span<F, I, O, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **ModalParser**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<Range<usize>, E> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `WithSpan`

[`Parser`] implementation for [`Parser::with_span`]

```rust
pub struct WithSpan<F, I, O, E> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, input: &mut I) -> Result<(O, Range<usize>), E> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Freeze**
- **Sync**
- **ModalParser**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
#### Struct `OutputInto`

[`Parser`] implementation for [`Parser::output_into`]

```rust
pub struct OutputInto<F, I, O, O2, E> {
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

- **RefUnwindSafe**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<O2, E> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ModalParser**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `ErrInto`

[`Parser`] implementation for [`Parser::err_into`]

```rust
pub struct ErrInto<F, I, O, E, E2> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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

- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<O, E2> { /* ... */ }
    ```

- **Unpin**
- **ModalParser**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `Context`

[`Parser`] implementation for [`Parser::context`]

```rust
pub struct Context<F, I, O, E, C> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ModalParser**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **UnwindSafe**
- **Parser**
  - ```rust
    fn parse_next(self: &mut Self, i: &mut I) -> Result<O, E> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Re-exports

#### Re-export `self::branch::*`

```rust
pub use self::branch::*;
```

#### Re-export `self::core::*`

```rust
pub use self::core::*;
```

#### Re-export `self::debug::*`

```rust
pub use self::debug::*;
```

#### Re-export `self::multi::*`

```rust
pub use self::multi::*;
```

#### Re-export `self::sequence::*`

```rust
pub use self::sequence::*;
```

## Module `token`

Parsers extracting tokens from the stream

```rust
pub mod token { /* ... */ }
```

### Functions

#### Function `any`

**Attributes:**

- `#[inline(always)]`
- `#[doc(alias = "token")]`

Matches one token

*Complete version*: Will return an error if there's not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn any(input: &mut &str) -> ModalResult<char>
# {
#     winnow::token::any.parse_next(input)
# }
```

# Example

```rust
# use winnow::{token::any, error::ErrMode, error::ContextError};
# use winnow::prelude::*;
fn parser(input: &mut &str) -> ModalResult<char> {
    any.parse_next(input)
}

assert_eq!(parser.parse_peek("abc"), Ok(("bc",'a')));
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::{token::any, error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
assert_eq!(any::<_, ErrMode<ContextError>>.parse_peek(Partial::new("abc")), Ok((Partial::new("bc"),'a')));
assert_eq!(any::<_, ErrMode<ContextError>>.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn any<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Token, Error>
where
    Input: StreamIsPartial + Stream,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `literal`

**Attributes:**

- `#[inline(always)]`
- `#[doc(alias = "tag")]`
- `#[doc(alias = "bytes")]`
- `#[doc(alias = "just")]`

Recognizes a literal

The input data will be compared to the literal combinator's argument and will return the part of
the input that matches the argument

It will return `Err(ErrMode::Backtrack(_))` if the input doesn't match the literal

<div class="warning">

**Note:** [`Parser`] is implemented for strings and byte strings as a convenience (complete
only)

</div>

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
# use winnow::error::ContextError;
pub fn literal(literal: &str) -> impl Parser<&str, &str, ContextError>
# {
#     winnow::token::literal(literal)
# }
```

# Example
```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
#
fn parser<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
  "Hello".parse_next(s)
}

assert_eq!(parser.parse_peek("Hello, World!"), Ok((", World!", "Hello")));
assert!(parser.parse_peek("Something").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;

fn parser<'i>(s: &mut Partial<&'i str>) -> ModalResult<&'i str> {
  "Hello".parse_next(s)
}

assert_eq!(parser.parse_peek(Partial::new("Hello, World!")), Ok((Partial::new(", World!"), "Hello")));
assert!(parser.parse_peek(Partial::new("Something")).is_err());
assert!(parser.parse_peek(Partial::new("S")).is_err());
assert_eq!(parser.parse_peek(Partial::new("H")), Err(ErrMode::Incomplete(Needed::Unknown)));
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
use winnow::token::literal;
use winnow::ascii::Caseless;

fn parser<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
  literal(Caseless("hello")).parse_next(s)
}

assert_eq!(parser.parse_peek("Hello, World!"), Ok((", World!", "Hello")));
assert_eq!(parser.parse_peek("hello, World!"), Ok((", World!", "hello")));
assert_eq!(parser.parse_peek("HeLlO, World!"), Ok((", World!", "HeLlO")));
assert!(parser.parse_peek("Something").is_err());
assert!(parser.parse_peek("").is_err());
```

```rust
pub fn literal<Literal, Input, Error>(literal: Literal) -> impl Parser<Input, <Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream + Compare<Literal>,
    Literal: Clone + crate::lib::std::fmt::Debug,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `one_of`

**Attributes:**

- `#[inline(always)]`
- `#[doc(alias = "char")]`
- `#[doc(alias = "token")]`
- `#[doc(alias = "satisfy")]`

Recognize a token that matches a [set of tokens][ContainsToken]

<div class="warning">

**Note:** [`Parser`] is implemented as a convenience (complete
only) for
- `u8`
- `char`

</div>

*Complete version*: Will return an error if there's not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
# use winnow::stream::ContainsToken;
# use winnow::error::ContextError;
pub fn one_of<'i>(set: impl ContainsToken<char>) -> impl Parser<&'i str, char, ContextError>
# {
#     winnow::token::one_of(set)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError};
# use winnow::token::one_of;
assert_eq!(one_of::<_, _, ContextError>(['a', 'b', 'c']).parse_peek("b"), Ok(("", 'b')));
assert!(one_of::<_, _, ContextError>('a').parse_peek("bc").is_err());
assert!(one_of::<_, _, ContextError>('a').parse_peek("").is_err());

fn parser_fn(i: &mut &str) -> ModalResult<char> {
    one_of(|c| c == 'a' || c == 'b').parse_next(i)
}
assert_eq!(parser_fn.parse_peek("abc"), Ok(("bc", 'a')));
assert!(parser_fn.parse_peek("cd").is_err());
assert!(parser_fn.parse_peek("").is_err());
```

```rust
# use winnow::prelude::*;
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::Partial;
# use winnow::token::one_of;
assert_eq!(one_of::<_, _, ErrMode<ContextError>>(['a', 'b', 'c']).parse_peek(Partial::new("b")), Ok((Partial::new(""), 'b')));
assert!(one_of::<_, _, ErrMode<ContextError>>('a').parse_peek(Partial::new("bc")).is_err());
assert_eq!(one_of::<_, _, ErrMode<ContextError>>('a').parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));

fn parser_fn(i: &mut Partial<&str>) -> ModalResult<char> {
    one_of(|c| c == 'a' || c == 'b').parse_next(i)
}
assert_eq!(parser_fn.parse_peek(Partial::new("abc")), Ok((Partial::new("bc"), 'a')));
assert!(parser_fn.parse_peek(Partial::new("cd")).is_err());
assert_eq!(parser_fn.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn one_of<Input, Set, Error>(set: Set) -> impl Parser<Input, <Input as Stream>::Token, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: Clone,
    Set: ContainsToken<<Input as Stream>::Token>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `none_of`

**Attributes:**

- `#[inline(always)]`

Recognize a token that does not match a [set of tokens][ContainsToken]

*Complete version*: Will return an error if there's not enough input data.

*[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
# use winnow::stream::ContainsToken;
# use winnow::error::ContextError;
pub fn none_of<'i>(set: impl ContainsToken<char>) -> impl Parser<&'i str, char, ContextError>
# {
#     winnow::token::none_of(set)
# }
```

# Example

```rust
# use winnow::{error::ErrMode, error::ContextError};
# use winnow::prelude::*;
# use winnow::token::none_of;
assert_eq!(none_of::<_, _, ContextError>(['a', 'b', 'c']).parse_peek("z"), Ok(("", 'z')));
assert!(none_of::<_, _, ContextError>(['a', 'b']).parse_peek("a").is_err());
assert!(none_of::<_, _, ContextError>('a').parse_peek("").is_err());
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
# use winnow::token::none_of;
assert_eq!(none_of::<_, _, ErrMode<ContextError>>(['a', 'b', 'c']).parse_peek(Partial::new("z")), Ok((Partial::new(""), 'z')));
assert!(none_of::<_, _, ErrMode<ContextError>>(['a', 'b']).parse_peek(Partial::new("a")).is_err());
assert_eq!(none_of::<_, _, ErrMode<ContextError>>('a').parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn none_of<Input, Set, Error>(set: Set) -> impl Parser<Input, <Input as Stream>::Token, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: Clone,
    Set: ContainsToken<<Input as Stream>::Token>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `take_while`

**Attributes:**

- `#[inline(always)]`
- `#[doc(alias = "is_a")]`
- `#[doc(alias = "take_while0")]`
- `#[doc(alias = "take_while1")]`

Recognize the longest (m <= len <= n) input slice that matches a [set of tokens][ContainsToken]

It will return an `ErrMode::Backtrack(_)` if the set of tokens wasn't met or is out
of range (m <= len <= n).

*[Partial version][crate::_topic::partial]* will return a `ErrMode::Incomplete(Needed::new(1))` if a member of the set of tokens reaches the end of the input or is too short.

To take a series of tokens, use [`repeat`][crate::combinator::repeat] to [`Accumulate`][crate::stream::Accumulate] into a `()` and then [`Parser::take`].

# Effective Signature

Assuming you are parsing a `&str` [Stream] with `0..` or `1..` [ranges][Range]:
```rust
# use std::ops::RangeFrom;
# use winnow::prelude::*;
# use winnow::stream::ContainsToken;
# use winnow::error::ContextError;
pub fn take_while<'i>(occurrences: RangeFrom<usize>, set: impl ContainsToken<char>) -> impl Parser<&'i str, &'i str, ContextError>
# {
#     winnow::token::take_while(occurrences, set)
# }
```

# Example

Zero or more tokens:
```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
use winnow::token::take_while;
use winnow::stream::AsChar;

fn alpha<'i>(s: &mut &'i [u8]) -> ModalResult<&'i [u8]> {
  take_while(0.., AsChar::is_alpha).parse_next(s)
}

assert_eq!(alpha.parse_peek(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
assert_eq!(alpha.parse_peek(b"12345"), Ok((&b"12345"[..], &b""[..])));
assert_eq!(alpha.parse_peek(b"latin"), Ok((&b""[..], &b"latin"[..])));
assert_eq!(alpha.parse_peek(b""), Ok((&b""[..], &b""[..])));
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::token::take_while;
use winnow::stream::AsChar;

fn alpha<'i>(s: &mut Partial<&'i [u8]>) -> ModalResult<&'i [u8]> {
  take_while(0.., AsChar::is_alpha).parse_next(s)
}

assert_eq!(alpha.parse_peek(Partial::new(b"latin123")), Ok((Partial::new(&b"123"[..]), &b"latin"[..])));
assert_eq!(alpha.parse_peek(Partial::new(b"12345")), Ok((Partial::new(&b"12345"[..]), &b""[..])));
assert_eq!(alpha.parse_peek(Partial::new(b"latin")), Err(ErrMode::Incomplete(Needed::new(1))));
assert_eq!(alpha.parse_peek(Partial::new(b"")), Err(ErrMode::Incomplete(Needed::new(1))));
```

One or more tokens:
```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
use winnow::token::take_while;
use winnow::stream::AsChar;

fn alpha<'i>(s: &mut &'i [u8]) -> ModalResult<&'i [u8]> {
  take_while(1.., AsChar::is_alpha).parse_next(s)
}

assert_eq!(alpha.parse_peek(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
assert_eq!(alpha.parse_peek(b"latin"), Ok((&b""[..], &b"latin"[..])));
assert!(alpha.parse_peek(b"12345").is_err());

fn hex<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
  take_while(1.., ('0'..='9', 'A'..='F')).parse_next(s)
}

assert_eq!(hex.parse_peek("123 and voila"), Ok((" and voila", "123")));
assert_eq!(hex.parse_peek("DEADBEEF and others"), Ok((" and others", "DEADBEEF")));
assert_eq!(hex.parse_peek("BADBABEsomething"), Ok(("something", "BADBABE")));
assert_eq!(hex.parse_peek("D15EA5E"), Ok(("", "D15EA5E")));
assert!(hex.parse_peek("").is_err());
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::token::take_while;
use winnow::stream::AsChar;

fn alpha<'i>(s: &mut Partial<&'i [u8]>) -> ModalResult<&'i [u8]> {
  take_while(1.., AsChar::is_alpha).parse_next(s)
}

assert_eq!(alpha.parse_peek(Partial::new(b"latin123")), Ok((Partial::new(&b"123"[..]), &b"latin"[..])));
assert_eq!(alpha.parse_peek(Partial::new(b"latin")), Err(ErrMode::Incomplete(Needed::new(1))));
assert!(alpha.parse_peek(Partial::new(b"12345")).is_err());

fn hex<'i>(s: &mut Partial<&'i str>) -> ModalResult<&'i str> {
  take_while(1.., ('0'..='9', 'A'..='F')).parse_next(s)
}

assert_eq!(hex.parse_peek(Partial::new("123 and voila")), Ok((Partial::new(" and voila"), "123")));
assert_eq!(hex.parse_peek(Partial::new("DEADBEEF and others")), Ok((Partial::new(" and others"), "DEADBEEF")));
assert_eq!(hex.parse_peek(Partial::new("BADBABEsomething")), Ok((Partial::new("something"), "BADBABE")));
assert_eq!(hex.parse_peek(Partial::new("D15EA5E")), Err(ErrMode::Incomplete(Needed::new(1))));
assert_eq!(hex.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

Arbitrary amount of tokens:
```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
use winnow::token::take_while;
use winnow::stream::AsChar;

fn short_alpha<'i>(s: &mut &'i [u8]) -> ModalResult<&'i [u8]> {
  take_while(3..=6, AsChar::is_alpha).parse_next(s)
}

assert_eq!(short_alpha.parse_peek(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
assert_eq!(short_alpha.parse_peek(b"lengthy"), Ok((&b"y"[..], &b"length"[..])));
assert_eq!(short_alpha.parse_peek(b"latin"), Ok((&b""[..], &b"latin"[..])));
assert!(short_alpha.parse_peek(b"ed").is_err());
assert!(short_alpha.parse_peek(b"12345").is_err());
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::token::take_while;
use winnow::stream::AsChar;

fn short_alpha<'i>(s: &mut Partial<&'i [u8]>) -> ModalResult<&'i [u8]> {
  take_while(3..=6, AsChar::is_alpha).parse_next(s)
}

assert_eq!(short_alpha.parse_peek(Partial::new(b"latin123")), Ok((Partial::new(&b"123"[..]), &b"latin"[..])));
assert_eq!(short_alpha.parse_peek(Partial::new(b"lengthy")), Ok((Partial::new(&b"y"[..]), &b"length"[..])));
assert_eq!(short_alpha.parse_peek(Partial::new(b"latin")), Err(ErrMode::Incomplete(Needed::new(1))));
assert_eq!(short_alpha.parse_peek(Partial::new(b"ed")), Err(ErrMode::Incomplete(Needed::new(1))));
assert!(short_alpha.parse_peek(Partial::new(b"12345")).is_err());
```

```rust
pub fn take_while<Set, Input, Error, /* synthetic */ impl Into<Range>: Into<crate::stream::Range>>(occurrences: impl Into<crate::stream::Range>, set: Set) -> impl Parser<Input, <Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    Set: ContainsToken<<Input as Stream>::Token>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `take_till`

**Attributes:**

- `#[inline(always)]`
- `#[doc(alias = "is_not")]`

Recognize the longest input slice (if any) till a member of a [set of tokens][ContainsToken] is found.

It doesn't consume the terminating token from the set.

*[Partial version][crate::_topic::partial]* will return a `ErrMode::Incomplete(Needed::new(1))` if the match reaches the
end of input or if there was not match.

See also
- [`take_until`] for recognizing up-to a [`literal`] (w/ optional simd optimizations)
- [`repeat_till`][crate::combinator::repeat_till] with [`Parser::take`] for taking tokens up to a [`Parser`]

# Effective Signature

Assuming you are parsing a `&str` [Stream] with `0..` or `1..` [ranges][Range]:
```rust
# use std::ops::RangeFrom;
# use winnow::prelude::*;
# use winnow::stream::ContainsToken;
# use winnow::error::ContextError;
pub fn take_till<'i>(occurrences: RangeFrom<usize>, set: impl ContainsToken<char>) -> impl Parser<&'i str, &'i str, ContextError>
# {
#     winnow::token::take_till(occurrences, set)
# }
```

# Example

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
use winnow::token::take_till;

fn till_colon<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
  take_till(0.., |c| c == ':').parse_next(s)
}

assert_eq!(till_colon.parse_peek("latin:123"), Ok((":123", "latin")));
assert_eq!(till_colon.parse_peek(":empty matched"), Ok((":empty matched", ""))); //allowed
assert_eq!(till_colon.parse_peek("12345"), Ok(("", "12345")));
assert_eq!(till_colon.parse_peek(""), Ok(("", "")));
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::token::take_till;

fn till_colon<'i>(s: &mut Partial<&'i str>) -> ModalResult<&'i str> {
  take_till(0.., |c| c == ':').parse_next(s)
}

assert_eq!(till_colon.parse_peek(Partial::new("latin:123")), Ok((Partial::new(":123"), "latin")));
assert_eq!(till_colon.parse_peek(Partial::new(":empty matched")), Ok((Partial::new(":empty matched"), ""))); //allowed
assert_eq!(till_colon.parse_peek(Partial::new("12345")), Err(ErrMode::Incomplete(Needed::new(1))));
assert_eq!(till_colon.parse_peek(Partial::new("")), Err(ErrMode::Incomplete(Needed::new(1))));
```

```rust
pub fn take_till<Set, Input, Error, /* synthetic */ impl Into<Range>: Into<crate::stream::Range>>(occurrences: impl Into<crate::stream::Range>, set: Set) -> impl Parser<Input, <Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    Set: ContainsToken<<Input as Stream>::Token>,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `take`

**Attributes:**

- `#[inline(always)]`

Recognize an input slice containing the first N input elements (I[..N]).

*Complete version*: It will return `Err(ErrMode::Backtrack(_))` if the input is shorter than the argument.

*[Partial version][crate::_topic::partial]*: if the input has less than N elements, `take` will
return a `ErrMode::Incomplete(Needed::new(M))` where M is the number of
additional bytes the parser would need to succeed.
It is well defined for `&[u8]` as the number of elements is the byte size,
but for types like `&str`, we cannot know how many bytes correspond for
the next few chars, so the result will be `ErrMode::Incomplete(Needed::Unknown)`

# Effective Signature

Assuming you are parsing a `&str` [Stream] with `0..` or `1..` ranges:
```rust
# use std::ops::RangeFrom;
# use winnow::prelude::*;
# use winnow::stream::ContainsToken;
# use winnow::error::ContextError;
pub fn take<'i>(token_count: usize) -> impl Parser<&'i str, &'i str, ContextError>
# {
#     winnow::token::take(token_count)
# }
```

# Example

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
use winnow::token::take;

fn take6<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
  take(6usize).parse_next(s)
}

assert_eq!(take6.parse_peek("1234567"), Ok(("7", "123456")));
assert_eq!(take6.parse_peek("things"), Ok(("", "things")));
assert!(take6.parse_peek("short").is_err());
assert!(take6.parse_peek("").is_err());
```

The units that are taken will depend on the input type. For example, for a
`&str` it will take a number of `char`'s, whereas for a `&[u8]` it will
take that many `u8`'s:

```rust
# use winnow::prelude::*;
use winnow::error::ContextError;
use winnow::token::take;

assert_eq!(take::<_, _, ContextError>(1usize).parse_peek("💙"), Ok(("", "💙")));
assert_eq!(take::<_, _, ContextError>(1usize).parse_peek("💙".as_bytes()), Ok((b"\x9F\x92\x99".as_ref(), b"\xF0".as_ref())));
```

```rust
# use winnow::prelude::*;
# use winnow::error::{ErrMode, ContextError, Needed};
# use winnow::Partial;
use winnow::token::take;

fn take6<'i>(s: &mut Partial<&'i str>) -> ModalResult<&'i str> {
  take(6usize).parse_next(s)
}

assert_eq!(take6.parse_peek(Partial::new("1234567")), Ok((Partial::new("7"), "123456")));
assert_eq!(take6.parse_peek(Partial::new("things")), Ok((Partial::new(""), "things")));
// `Unknown` as we don't know the number of bytes that `count` corresponds to
assert_eq!(take6.parse_peek(Partial::new("short")), Err(ErrMode::Incomplete(Needed::Unknown)));
```

```rust
pub fn take<UsizeLike, Input, Error>(token_count: UsizeLike) -> impl Parser<Input, <Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    UsizeLike: ToUsize,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `take_until`

**Attributes:**

- `#[inline(always)]`

Recognize the input slice up to the first occurrence of a [literal].

Feature `simd` will enable the use of [`memchr`](https://docs.rs/memchr/latest/memchr/).

It doesn't consume the literal.

*Complete version*: It will return `Err(ErrMode::Backtrack(_))`
if the literal wasn't met.

*[Partial version][crate::_topic::partial]*: will return a `ErrMode::Incomplete(Needed::new(N))` if the input doesn't
contain the literal or if the input is smaller than the literal.

See also
- [`take_till`] for recognizing up-to a [set of tokens][ContainsToken]
- [`repeat_till`][crate::combinator::repeat_till] with [`Parser::take`] for taking tokens up to a [`Parser`]

# Effective Signature

Assuming you are parsing a `&str` [Stream] with `0..` or `1..` [ranges][Range]:
```rust
# use std::ops::RangeFrom;
# use winnow::prelude::*;;
# use winnow::error::ContextError;
pub fn take_until(occurrences: RangeFrom<usize>, literal: &str) -> impl Parser<&str, &str, ContextError>
# {
#     winnow::token::take_until(occurrences, literal)
# }
```

# Example

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
use winnow::token::take_until;

fn until_eof<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
  take_until(0.., "eof").parse_next(s)
}

assert_eq!(until_eof.parse_peek("hello, worldeof"), Ok(("eof", "hello, world")));
assert!(until_eof.parse_peek("hello, world").is_err());
assert!(until_eof.parse_peek("").is_err());
assert_eq!(until_eof.parse_peek("1eof2eof"), Ok(("eof2eof", "1")));
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::token::take_until;

fn until_eof<'i>(s: &mut Partial<&'i str>) -> ModalResult<&'i str> {
  take_until(0.., "eof").parse_next(s)
}

assert_eq!(until_eof.parse_peek(Partial::new("hello, worldeof")), Ok((Partial::new("eof"), "hello, world")));
assert_eq!(until_eof.parse_peek(Partial::new("hello, world")), Err(ErrMode::Incomplete(Needed::Unknown)));
assert_eq!(until_eof.parse_peek(Partial::new("hello, worldeo")), Err(ErrMode::Incomplete(Needed::Unknown)));
assert_eq!(until_eof.parse_peek(Partial::new("1eof2eof")), Ok((Partial::new("eof2eof"), "1")));
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
use winnow::token::take_until;

fn until_eof<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
  take_until(1.., "eof").parse_next(s)
}

assert_eq!(until_eof.parse_peek("hello, worldeof"), Ok(("eof", "hello, world")));
assert!(until_eof.parse_peek("hello, world").is_err());
assert!(until_eof.parse_peek("").is_err());
assert_eq!(until_eof.parse_peek("1eof2eof"), Ok(("eof2eof", "1")));
assert!(until_eof.parse_peek("eof").is_err());
```

```rust
# use winnow::{error::ErrMode, error::ContextError, error::Needed};
# use winnow::prelude::*;
# use winnow::Partial;
use winnow::token::take_until;

fn until_eof<'i>(s: &mut Partial<&'i str>) -> ModalResult<&'i str> {
  take_until(1.., "eof").parse_next(s)
}

assert_eq!(until_eof.parse_peek(Partial::new("hello, worldeof")), Ok((Partial::new("eof"), "hello, world")));
assert_eq!(until_eof.parse_peek(Partial::new("hello, world")), Err(ErrMode::Incomplete(Needed::Unknown)));
assert_eq!(until_eof.parse_peek(Partial::new("hello, worldeo")), Err(ErrMode::Incomplete(Needed::Unknown)));
assert_eq!(until_eof.parse_peek(Partial::new("1eof2eof")), Ok((Partial::new("eof2eof"), "1")));
assert!(until_eof.parse_peek(Partial::new("eof")).is_err());
```

```rust
pub fn take_until<Literal, Input, Error, /* synthetic */ impl Into<Range>: Into<crate::stream::Range>>(occurrences: impl Into<crate::stream::Range>, literal: Literal) -> impl Parser<Input, <Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream + FindSlice<Literal>,
    Literal: Clone,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `rest`

**Attributes:**

- `#[inline]`

Return the remaining input.

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn rest<'i>(input: &mut &'i str) -> ModalResult<&'i str>
# {
#     winnow::token::rest.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::error::ContextError;
use winnow::token::rest;
assert_eq!(rest::<_,ContextError>.parse_peek("abc"), Ok(("", "abc")));
assert_eq!(rest::<_,ContextError>.parse_peek(""), Ok(("", "")));
```

```rust
pub fn rest<Input, Error>(input: &mut Input) -> crate::Result<<Input as Stream>::Slice, Error>
where
    Input: Stream,
    Error: ParserError<Input> { /* ... */ }
```

#### Function `rest_len`

**Attributes:**

- `#[inline]`

Return the length of the remaining input.

<div class="warning">

Note: this does not advance the [`Stream`]

</div>

# Effective Signature

Assuming you are parsing a `&str` [Stream]:
```rust
# use winnow::prelude::*;;
pub fn rest_len(input: &mut &str) -> ModalResult<usize>
# {
#     winnow::token::rest_len.parse_next(input)
# }
```

# Example

```rust
# use winnow::prelude::*;
# use winnow::error::ContextError;
use winnow::token::rest_len;
assert_eq!(rest_len::<_,ContextError>.parse_peek("abc"), Ok(("abc", 3)));
assert_eq!(rest_len::<_,ContextError>.parse_peek(""), Ok(("", 0)));
```

```rust
pub fn rest_len<Input, Error>(input: &mut Input) -> crate::Result<usize, Error>
where
    Input: Stream,
    Error: ParserError<Input> { /* ... */ }
```

## Module `prelude`

Core concepts available for glob import

Including
- [`StreamIsPartial`][crate::stream::StreamIsPartial]
- [`Parser`]

## Example

```rust
use winnow::prelude::*;

fn parse_data(input: &mut &str) -> ModalResult<u64> {
    // ...
#   winnow::ascii::dec_uint(input)
}

fn main() {
  let result = parse_data.parse("100");
  assert_eq!(result, Ok(100));
}
```

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `ModalError`

```rust
pub use crate::error::ModalError as _;
```

#### Re-export `ParserError`

```rust
pub use crate::error::ParserError as _;
```

#### Re-export `AsChar`

```rust
pub use crate::stream::AsChar as _;
```

#### Re-export `ContainsToken`

```rust
pub use crate::stream::ContainsToken as _;
```

#### Re-export `Stream`

```rust
pub use crate::stream::Stream as _;
```

#### Re-export `StreamIsPartial`

```rust
pub use crate::stream::StreamIsPartial as _;
```

#### Re-export `ModalParser`

```rust
pub use crate::ModalParser;
```

#### Re-export `ModalResult`

```rust
pub use crate::ModalResult;
```

#### Re-export `Parser`

```rust
pub use crate::Parser;
```

## Re-exports

### Re-export `ModalResult`

```rust
pub use error::ModalResult;
```

### Re-export `Result`

```rust
pub use error::Result;
```

### Re-export `BStr`

```rust
pub use stream::BStr;
```

### Re-export `Bytes`

```rust
pub use stream::Bytes;
```

### Re-export `LocatingSlice`

```rust
pub use stream::LocatingSlice;
```

### Re-export `Partial`

```rust
pub use stream::Partial;
```

### Re-export `Stateful`

```rust
pub use stream::Stateful;
```

### Re-export `Str`

```rust
pub use stream::Str;
```

### Re-export `parser::*`

```rust
pub use parser::*;
```

