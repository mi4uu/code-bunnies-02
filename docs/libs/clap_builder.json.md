# Crate Documentation

**Version:** 4.5.36

**Format Version:** 43

# Module `clap_builder`

# `clap_builder`

Builder implementation for clap.

[docs.rs](https://docs.rs/clap)
- [Derive Tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [Derive Reference](https://docs.rs/clap/latest/clap/_derive/index.html)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more details.

## Modules

## Module `builder`

Define [`Command`] line [arguments][`Arg`]

```rust
pub mod builder { /* ... */ }
```

### Modules

## Module `styling`

Terminal [`Styles`] for help and error output

```rust
pub mod styling { /* ... */ }
```

### Types

#### Struct `Styles`

**Attributes:**

- `#[allow(missing_copy_implementations)]`

Terminal styling definitions

See also [`Command::styles`][crate::Command::styles].

# Example

clap v3 styling
```rust
# use clap_builder as clap;
# use clap::builder::styling::*;
let styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
```

```rust
pub struct Styles {
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
  pub const fn plain() -> Self { /* ... */ }
  ```
  No terminal styling

- ```rust
  pub const fn styled() -> Self { /* ... */ }
  ```
  Default terminal styling

- ```rust
  pub const fn header(self: Self, style: Style) -> Self { /* ... */ }
  ```
  General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]

- ```rust
  pub const fn error(self: Self, style: Style) -> Self { /* ... */ }
  ```
  Error heading

- ```rust
  pub const fn usage(self: Self, style: Style) -> Self { /* ... */ }
  ```
  Usage heading

- ```rust
  pub const fn literal(self: Self, style: Style) -> Self { /* ... */ }
  ```
  Literal command-line syntax, e.g. `--help`

- ```rust
  pub const fn placeholder(self: Self, style: Style) -> Self { /* ... */ }
  ```
  Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]

- ```rust
  pub const fn valid(self: Self, style: Style) -> Self { /* ... */ }
  ```
  Highlight suggested usage

- ```rust
  pub const fn invalid(self: Self, style: Style) -> Self { /* ... */ }
  ```
  Highlight invalid usage

- ```rust
  pub const fn get_header(self: &Self) -> &Style { /* ... */ }
  ```
  General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]

- ```rust
  pub const fn get_error(self: &Self) -> &Style { /* ... */ }
  ```
  Error heading

- ```rust
  pub const fn get_usage(self: &Self) -> &Style { /* ... */ }
  ```
  Usage heading

- ```rust
  pub const fn get_literal(self: &Self) -> &Style { /* ... */ }
  ```
  Literal command-line syntax, e.g. `--help`

- ```rust
  pub const fn get_placeholder(self: &Self) -> &Style { /* ... */ }
  ```
  Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]

- ```rust
  pub const fn get_valid(self: &Self) -> &Style { /* ... */ }
  ```
  Highlight suggested usage

- ```rust
  pub const fn get_invalid(self: &Self) -> &Style { /* ... */ }
  ```
  Highlight invalid usage

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Styles { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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

- **Unpin**
- **Sync**
- **UnwindSafe**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Re-exports

#### Re-export `anstyle::*`

```rust
pub use anstyle::*;
```

### Re-exports

#### Re-export `Str`

```rust
pub use self::str::Str;
```

#### Re-export `ArgAction`

```rust
pub use action::ArgAction;
```

#### Re-export `Arg`

```rust
pub use arg::Arg;
```

#### Re-export `ArgGroup`

```rust
pub use arg_group::ArgGroup;
```

#### Re-export `ArgPredicate`

```rust
pub use arg_predicate::ArgPredicate;
```

#### Re-export `Command`

```rust
pub use command::Command;
```

#### Re-export `OsStr`

```rust
pub use os_str::OsStr;
```

#### Re-export `PossibleValue`

```rust
pub use possible_value::PossibleValue;
```

#### Re-export `ValueRange`

```rust
pub use range::ValueRange;
```

#### Re-export `IntoResettable`

```rust
pub use resettable::IntoResettable;
```

#### Re-export `Resettable`

```rust
pub use resettable::Resettable;
```

#### Re-export `StyledStr`

```rust
pub use styled_str::StyledStr;
```

#### Re-export `Styles`

```rust
pub use styling::Styles;
```

#### Re-export `ValueHint`

```rust
pub use value_hint::ValueHint;
```

#### Re-export `BoolValueParser`

```rust
pub use value_parser::BoolValueParser;
```

#### Re-export `BoolishValueParser`

```rust
pub use value_parser::BoolishValueParser;
```

#### Re-export `EnumValueParser`

```rust
pub use value_parser::EnumValueParser;
```

#### Re-export `FalseyValueParser`

```rust
pub use value_parser::FalseyValueParser;
```

#### Re-export `MapValueParser`

```rust
pub use value_parser::MapValueParser;
```

#### Re-export `NonEmptyStringValueParser`

```rust
pub use value_parser::NonEmptyStringValueParser;
```

#### Re-export `OsStringValueParser`

```rust
pub use value_parser::OsStringValueParser;
```

#### Re-export `PathBufValueParser`

```rust
pub use value_parser::PathBufValueParser;
```

#### Re-export `PossibleValuesParser`

```rust
pub use value_parser::PossibleValuesParser;
```

#### Re-export `RangedI64ValueParser`

```rust
pub use value_parser::RangedI64ValueParser;
```

#### Re-export `RangedU64ValueParser`

```rust
pub use value_parser::RangedU64ValueParser;
```

#### Re-export `StringValueParser`

```rust
pub use value_parser::StringValueParser;
```

#### Re-export `TryMapValueParser`

```rust
pub use value_parser::TryMapValueParser;
```

#### Re-export `TypedValueParser`

```rust
pub use value_parser::TypedValueParser;
```

#### Re-export `UnknownArgumentValueParser`

```rust
pub use value_parser::UnknownArgumentValueParser;
```

#### Re-export `ValueParser`

```rust
pub use value_parser::ValueParser;
```

#### Re-export `ValueParserFactory`

```rust
pub use value_parser::ValueParserFactory;
```

## Module `error`

**Attributes:**

- `#[<cfg_attr>(not(feature = "error-context"), allow(dead_code))]`
- `#[<cfg_attr>(not(feature = "error-context"), allow(unused_imports))]`
- `#[<cfg_attr>(not(feature = "error-context"), allow(unused_variables))]`
- `#[<cfg_attr>(not(feature = "error-context"), allow(unused_mut))]`
- `#[<cfg_attr>(not(feature = "error-context"), allow(clippy::let_and_return))]`

Error reporting

```rust
pub mod error { /* ... */ }
```

### Types

#### Type Alias `Result`

Short hand for [`Result`] type

[`Result`]: std::result::Result

```rust
pub type Result<T, E = Error> = std::result::Result<T, E>;
```

#### Struct `Error`

Command Line Argument Parser Error

See [`Command::error`] to create an error.

[`Command::error`]: crate::Command::error

```rust
pub struct Error<F: ErrorFormatter = DefaultFormatter> {
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
  pub fn raw</* synthetic */ impl Display: Display>(kind: ErrorKind, message: impl Display) -> Self { /* ... */ }
  ```
  Create an unformatted error

- ```rust
  pub fn format(self: Self, cmd: &mut Command) -> Self { /* ... */ }
  ```
  Format the existing message with the Command's context

- ```rust
  pub fn new(kind: ErrorKind) -> Self { /* ... */ }
  ```
  Create an error with a pre-defined message

- ```rust
  pub fn with_cmd(self: Self, cmd: &Command) -> Self { /* ... */ }
  ```
  Apply [`Command`]'s formatting to the error

- ```rust
  pub fn apply<EF: ErrorFormatter>(self: Self) -> Error<EF> { /* ... */ }
  ```
  Apply an alternative formatter to the error

- ```rust
  pub fn kind(self: &Self) -> ErrorKind { /* ... */ }
  ```
  Type of error for programmatic processing

- ```rust
  pub fn context(self: &Self) -> impl Iterator<Item = (ContextKind, &ContextValue)> { /* ... */ }
  ```
  Additional information to further qualify the error

- ```rust
  pub fn get(self: &Self, kind: ContextKind) -> Option<&ContextValue> { /* ... */ }
  ```
  Lookup a piece of context

- ```rust
  pub fn insert(self: &mut Self, kind: ContextKind, value: ContextValue) -> Option<ContextValue> { /* ... */ }
  ```
  Insert a piece of context

- ```rust
  pub fn remove(self: &mut Self, kind: ContextKind) -> Option<ContextValue> { /* ... */ }
  ```
  Remove a piece of context, return the old value if any

- ```rust
  pub fn use_stderr(self: &Self) -> bool { /* ... */ }
  ```
  Should the message be written to `stdout` or not?

- ```rust
  pub fn exit_code(self: &Self) -> i32 { /* ... */ }
  ```
  Returns the exit code that `.exit` will exit the process with.

- ```rust
  pub fn exit(self: &Self) -> never { /* ... */ }
  ```
  Prints the error and exits.

- ```rust
  pub fn print(self: &Self) -> io::Result<()> { /* ... */ }
  ```
  Prints formatted and colored error to `stdout` or `stderr` according to its error kind

- ```rust
  pub fn render(self: &Self) -> StyledStr { /* ... */ }
  ```
  Render the error message to a [`StyledStr`].

###### Trait Implementations

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
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
    fn from(e: io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: fmt::Error) -> Self { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Re-exports

#### Re-export `ErrorFormatter`

```rust
pub use format::ErrorFormatter;
```

#### Re-export `KindFormatter`

```rust
pub use format::KindFormatter;
```

#### Re-export `ErrorKind`

```rust
pub use kind::ErrorKind;
```

#### Re-export `ContextKind`

**Attributes:**

- `#[<cfg>(feature = "error-context")]`

```rust
pub use context::ContextKind;
```

#### Re-export `ContextValue`

**Attributes:**

- `#[<cfg>(feature = "error-context")]`

```rust
pub use context::ContextValue;
```

#### Re-export `RichFormatter`

**Attributes:**

- `#[<cfg>(feature = "error-context")]`

```rust
pub use format::RichFormatter;
```

#### Re-export `RichFormatter`

**Attributes:**

- `#[<cfg>(feature = "error-context")]`

```rust
pub use RichFormatter as DefaultFormatter;
```

## Module `parser`

[`Command`][crate::Command] line argument parser

```rust
pub mod parser { /* ... */ }
```

### Re-exports

#### Re-export `IdsRef`

```rust
pub use self::matches::IdsRef;
```

#### Re-export `RawValues`

```rust
pub use self::matches::RawValues;
```

#### Re-export `Values`

```rust
pub use self::matches::Values;
```

#### Re-export `ValuesRef`

```rust
pub use self::matches::ValuesRef;
```

#### Re-export `ArgMatches`

```rust
pub use self::matches::ArgMatches;
```

#### Re-export `Indices`

```rust
pub use self::matches::Indices;
```

#### Re-export `ValueSource`

```rust
pub use self::matches::ValueSource;
```

#### Re-export `MatchesError`

```rust
pub use error::MatchesError;
```

## Types

### Type Alias `Error`

Command Line Argument Parser Error

See [`Command::error`] to create an error.

[`Command::error`]: crate::Command::error

```rust
pub type Error = error::Error<error::DefaultFormatter>;
```

## Macros

### Macro `command`

**Attributes:**

- `#[<cfg>(not(feature = "cargo"))]`
- `#[macro_export]`

Requires `cargo` feature flag to be enabled.

```rust
pub macro_rules! command {
    /* macro_rules! command {
    () => { ... };
    ($name:expr) => { ... };
} */
}
```

### Macro `arg`

**Attributes:**

- `#[macro_export]`

Create an [`Arg`] from a usage string.

Allows creation of basic settings for the [`Arg`].

<div class="warning">

**NOTE**: Not all settings may be set using the usage string method. Some properties are
only available via the builder pattern.

</div>

# Syntax

Usage strings typically following the form:

```notrust
[explicit name] [short] [long] [value names] [...] [help string]
```

### Explicit Name

The name may be either a bare-word or a string, followed by a `:`, like `name:` or
`"name":`.

*Note:* This is an optional field, if it's omitted the argument will use one of the additional
fields as the name using the following priority order:

 1. Explicit Name
 2. Long
 3. Value Name

See [`Arg::id`][crate::Arg::id].

### Short

A short flag is a `-` followed by either a bare-character or quoted character, like `-f` or
`-'f'`.

See [`Arg::short`][crate::Arg::short].

### Long

A long flag is a `--` followed by either a bare-word or a string, like `--foo` or
`--"foo"`.

<div class="warning">

**NOTE:** Dashes in the long name (e.g. `--foo-bar`) is not supported and quoting is required
(e.g. `--"foo-bar"`).

</div>

See [`Arg::long`][crate::Arg::long].

### Values (Value Notation)

This is set by placing bare-word between:
- `[]` like `[FOO]`
  - Positional argument: optional
  - Named argument: optional value
- `<>` like `<FOO>`: required

See [`Arg::value_name`][crate::Arg::value_name].

### `...`

`...` (three consecutive dots/periods) specifies that this argument may occur multiple
times (not to be confused with multiple values per occurrence).

See [`ArgAction::Count`][crate::ArgAction::Count] and [`ArgAction::Append`][crate::ArgAction::Append].

### Help String

The help string is denoted between a pair of double quotes `""` and may contain any
characters.

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, Arg, arg};
let cmd = Command::new("prog")
    .args(&[
        arg!(--config <FILE> "a required file for the configuration and no short"),
        arg!(-d --debug ... "turns on debugging information and allows multiples"),
        arg!([input] "an optional input file to use")
    ]);

let m = cmd.try_get_matches_from(["prog", "--config", "file.toml"]).unwrap();
assert_eq!(m.get_one::<String>("config").unwrap(), "file.toml");
assert_eq!(*m.get_one::<u8>("debug").unwrap(), 0);
assert_eq!(m.get_one::<String>("input"), None);
```
[`Arg`]: crate::Arg

```rust
pub macro_rules! arg {
    /* macro_rules! arg {
    ( $name:ident: $($tail:tt)+ ) => { ... };
    ( $($tail:tt)+ ) => { ... };
} */
}
```

### Macro `value_parser`

**Attributes:**

- `#[macro_export]`

Select a [`ValueParser`] implementation from the intended type

Supported types
- [`ValueParserFactory` types][ValueParserFactory], including
  - [Native types][ValueParser]: `bool`, `String`, `OsString`, `PathBuf`
  - [Ranged numeric types][RangedI64ValueParser]: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`
- [`ValueEnum` types][crate::ValueEnum]
- [`From<OsString>` types][std::convert::From] and [`From<&OsStr>` types][std::convert::From]
- [`From<String>` types][std::convert::From] and [`From<&str>` types][std::convert::From]
- [`FromStr` types][std::str::FromStr], including usize, isize

# Example

Usage:
```rust
# use clap_builder as clap;
# use std::path::PathBuf;
# use std::path::Path;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("output")
            .value_parser(clap::value_parser!(PathBuf))
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "file.txt"]).unwrap();
let port: &PathBuf = m.get_one("output")
    .expect("required");
assert_eq!(port, Path::new("file.txt"));
```

Example mappings:
```rust
# use clap_builder as clap;
# use clap::ColorChoice;
// Built-in types
let parser = clap::value_parser!(String);
assert_eq!(format!("{parser:?}"), "ValueParser::string");
let parser = clap::value_parser!(std::ffi::OsString);
assert_eq!(format!("{parser:?}"), "ValueParser::os_string");
let parser = clap::value_parser!(std::path::PathBuf);
assert_eq!(format!("{parser:?}"), "ValueParser::path_buf");
clap::value_parser!(u16).range(3000..);
clap::value_parser!(u64).range(3000..);

// FromStr types
let parser = clap::value_parser!(usize);
assert_eq!(format!("{parser:?}"), "_AnonymousValueParser(ValueParser::other(usize))");

// ValueEnum types
clap::value_parser!(ColorChoice);
```

```rust
pub macro_rules! value_parser {
    /* macro_rules! value_parser {
    ($name:ty) => { ... };
} */
}
```

## Re-exports

### Re-export `ArgAction`

```rust
pub use crate::builder::ArgAction;
```

### Re-export `Command`

```rust
pub use crate::builder::Command;
```

### Re-export `ValueHint`

```rust
pub use crate::builder::ValueHint;
```

### Re-export `Arg`

```rust
pub use crate::builder::Arg;
```

### Re-export `ArgGroup`

```rust
pub use crate::builder::ArgGroup;
```

### Re-export `ArgMatches`

```rust
pub use crate::parser::ArgMatches;
```

### Re-export `ColorChoice`

```rust
pub use crate::util::color::ColorChoice;
```

### Re-export `Id`

```rust
pub use crate::util::Id;
```

### Re-export `Args`

```rust
pub use crate::derive::Args;
```

### Re-export `CommandFactory`

```rust
pub use crate::derive::CommandFactory;
```

### Re-export `FromArgMatches`

```rust
pub use crate::derive::FromArgMatches;
```

### Re-export `Parser`

```rust
pub use crate::derive::Parser;
```

### Re-export `Subcommand`

```rust
pub use crate::derive::Subcommand;
```

### Re-export `ValueEnum`

```rust
pub use crate::derive::ValueEnum;
```

