# Crate Documentation

**Version:** 0.8.5

**Format Version:** 43

# Module `regex_syntax`

This crate provides a robust regular expression parser.

This crate defines two primary types:

* [`Ast`](ast::Ast) is the abstract syntax of a regular expression.
  An abstract syntax corresponds to a *structured representation* of the
  concrete syntax of a regular expression, where the concrete syntax is the
  pattern string itself (e.g., `foo(bar)+`). Given some abstract syntax, it
  can be converted back to the original concrete syntax (modulo some details,
  like whitespace). To a first approximation, the abstract syntax is complex
  and difficult to analyze.
* [`Hir`](hir::Hir) is the high-level intermediate representation
  ("HIR" or "high-level IR" for short) of regular expression. It corresponds to
  an intermediate state of a regular expression that sits between the abstract
  syntax and the low level compiled opcodes that are eventually responsible for
  executing a regular expression search. Given some high-level IR, it is not
  possible to produce the original concrete syntax (although it is possible to
  produce an equivalent concrete syntax, but it will likely scarcely resemble
  the original pattern). To a first approximation, the high-level IR is simple
  and easy to analyze.

These two types come with conversion routines:

* An [`ast::parse::Parser`] converts concrete syntax (a `&str`) to an
[`Ast`](ast::Ast).
* A [`hir::translate::Translator`] converts an [`Ast`](ast::Ast) to a
[`Hir`](hir::Hir).

As a convenience, the above two conversion routines are combined into one via
the top-level [`Parser`] type. This `Parser` will first convert your pattern to
an `Ast` and then convert the `Ast` to an `Hir`. It's also exposed as top-level
[`parse`] free function.


# Example

This example shows how to parse a pattern string into its HIR:

```
use regex_syntax::{hir::Hir, parse};

let hir = parse("a|b")?;
assert_eq!(hir, Hir::alternation(vec![
    Hir::literal("a".as_bytes()),
    Hir::literal("b".as_bytes()),
]));
# Ok::<(), Box<dyn std::error::Error>>(())
```


# Concrete syntax supported

The concrete syntax is documented as part of the public API of the
[`regex` crate](https://docs.rs/regex/%2A/regex/#syntax).


# Input safety

A key feature of this library is that it is safe to use with end user facing
input. This plays a significant role in the internal implementation. In
particular:

1. Parsers provide a `nest_limit` option that permits callers to control how
   deeply nested a regular expression is allowed to be. This makes it possible
   to do case analysis over an `Ast` or an `Hir` using recursion without
   worrying about stack overflow.
2. Since relying on a particular stack size is brittle, this crate goes to
   great lengths to ensure that all interactions with both the `Ast` and the
   `Hir` do not use recursion. Namely, they use constant stack space and heap
   space proportional to the size of the original pattern string (in bytes).
   This includes the type's corresponding destructors. (One exception to this
   is literal extraction, but this will eventually get fixed.)


# Error reporting

The `Display` implementations on all `Error` types exposed in this library
provide nice human readable errors that are suitable for showing to end users
in a monospace font.


# Literal extraction

This crate provides limited support for [literal extraction from `Hir`
values](hir::literal). Be warned that literal extraction uses recursion, and
therefore, stack size proportional to the size of the `Hir`.

The purpose of literal extraction is to speed up searches. That is, if you
know a regular expression must match a prefix or suffix literal, then it is
often quicker to search for instances of that literal, and then confirm or deny
the match using the full regular expression engine. These optimizations are
done automatically in the `regex` crate.


# Crate features

An important feature provided by this crate is its Unicode support. This
includes things like case folding, boolean properties, general categories,
scripts and Unicode-aware support for the Perl classes `\w`, `\s` and `\d`.
However, a downside of this support is that it requires bundling several
Unicode data tables that are substantial in size.

A fair number of use cases do not require full Unicode support. For this
reason, this crate exposes a number of features to control which Unicode
data is available.

If a regular expression attempts to use a Unicode feature that is not available
because the corresponding crate feature was disabled, then translating that
regular expression to an `Hir` will return an error. (It is still possible
construct an `Ast` for such a regular expression, since Unicode data is not
used until translation to an `Hir`.) Stated differently, enabling or disabling
any of the features below can only add or subtract from the total set of valid
regular expressions. Enabling or disabling a feature will never modify the
match semantics of a regular expression.

The following features are available:

* **std** -
  Enables support for the standard library. This feature is enabled by default.
  When disabled, only `core` and `alloc` are used. Otherwise, enabling `std`
  generally just enables `std::error::Error` trait impls for the various error
  types.
* **unicode** -
  Enables all Unicode features. This feature is enabled by default, and will
  always cover all Unicode features, even if more are added in the future.
* **unicode-age** -
  Provide the data for the
  [Unicode `Age` property](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
  This makes it possible to use classes like `\p{Age:6.0}` to refer to all
  codepoints first introduced in Unicode 6.0
* **unicode-bool** -
  Provide the data for numerous Unicode boolean properties. The full list
  is not included here, but contains properties like `Alphabetic`, `Emoji`,
  `Lowercase`, `Math`, `Uppercase` and `White_Space`.
* **unicode-case** -
  Provide the data for case insensitive matching using
  [Unicode's "simple loose matches" specification](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
* **unicode-gencat** -
  Provide the data for
  [Unicode general categories](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
  This includes, but is not limited to, `Decimal_Number`, `Letter`,
  `Math_Symbol`, `Number` and `Punctuation`.
* **unicode-perl** -
  Provide the data for supporting the Unicode-aware Perl character classes,
  corresponding to `\w`, `\s` and `\d`. This is also necessary for using
  Unicode-aware word boundary assertions. Note that if this feature is
  disabled, the `\s` and `\d` character classes are still available if the
  `unicode-bool` and `unicode-gencat` features are enabled, respectively.
* **unicode-script** -
  Provide the data for
  [Unicode scripts and script extensions](https://www.unicode.org/reports/tr24/).
  This includes, but is not limited to, `Arabic`, `Cyrillic`, `Hebrew`,
  `Latin` and `Thai`.
* **unicode-segment** -
  Provide the data necessary to provide the properties used to implement the
  [Unicode text segmentation algorithms](https://www.unicode.org/reports/tr29/).
  This enables using classes like `\p{gcb=Extend}`, `\p{wb=Katakana}` and
  `\p{sb=ATerm}`.
* **arbitrary** -
  Enabling this feature introduces a public dependency on the
  [`arbitrary`](https://crates.io/crates/arbitrary)
  crate. Namely, it implements the `Arbitrary` trait from that crate for the
  [`Ast`](crate::ast::Ast) type. This feature is disabled by default.

## Modules

## Module `ast`

Defines an abstract syntax for regular expressions.

```rust
pub mod ast { /* ... */ }
```

### Modules

## Module `parse`

This module provides a regular expression parser.

```rust
pub mod parse { /* ... */ }
```

### Types

#### Struct `ParserBuilder`

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

```rust
pub struct ParserBuilder {
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
  pub fn new() -> ParserBuilder { /* ... */ }
  ```
  Create a new parser builder with a default configuration.

- ```rust
  pub fn build(self: &Self) -> Parser { /* ... */ }
  ```
  Build a parser from this configuration with the given pattern.

- ```rust
  pub fn nest_limit(self: &mut Self, limit: u32) -> &mut ParserBuilder { /* ... */ }
  ```
  Set the nesting limit for this parser.

- ```rust
  pub fn octal(self: &mut Self, yes: bool) -> &mut ParserBuilder { /* ... */ }
  ```
  Whether to support octal syntax or not.

- ```rust
  pub fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut ParserBuilder { /* ... */ }
  ```
  Enable verbose mode in the regular expression.

- ```rust
  pub fn empty_min_range(self: &mut Self, yes: bool) -> &mut ParserBuilder { /* ... */ }
  ```
  Allow using `{,n}` as an equivalent to `{0,n}`.

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParserBuilder { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> ParserBuilder { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Parser`

A regular expression parser.

This parses a string representation of a regular expression into an
abstract syntax tree. The size of the tree is proportional to the length
of the regular expression pattern.

A `Parser` can be configured in more detail via a [`ParserBuilder`].

```rust
pub struct Parser {
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
  pub fn new() -> Parser { /* ... */ }
  ```
  Create a new parser with a default configuration.

- ```rust
  pub fn parse(self: &mut Self, pattern: &str) -> core::result::Result<Ast, ast::Error> { /* ... */ }
  ```
  Parse the regular expression into an abstract syntax tree.

- ```rust
  pub fn parse_with_comments(self: &mut Self, pattern: &str) -> core::result::Result<ast::WithComments, ast::Error> { /* ... */ }
  ```
  Parse the regular expression and return an abstract syntax tree with

###### Trait Implementations

- **Sync**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **RefUnwindSafe**
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
    fn clone(self: &Self) -> Parser { /* ... */ }
    ```

## Module `print`

This module provides a regular expression printer for `Ast`.

```rust
pub mod print { /* ... */ }
```

### Types

#### Struct `Printer`

A printer for a regular expression abstract syntax tree.

A printer converts an abstract syntax tree (AST) to a regular expression
pattern string. This particular printer uses constant stack space and heap
space proportional to the size of the AST.

This printer will not necessarily preserve the original formatting of the
regular expression pattern string. For example, all whitespace and comments
are ignored.

```rust
pub struct Printer {
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
  pub fn new() -> Printer { /* ... */ }
  ```
  Create a new printer.

- ```rust
  pub fn print<W: fmt::Write>(self: &mut Self, ast: &Ast, wtr: W) -> fmt::Result { /* ... */ }
  ```
  Print the given `Ast` to the given writer. The writer must implement

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Types

#### Struct `Error`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

An error that occurred while parsing a regular expression into an abstract
syntax tree.

Note that not all ASTs represents a valid regular expression. For example,
an AST is constructed without error for `\p{Quux}`, but `Quux` is not a
valid Unicode property name. That particular error is reported when
translating an AST to the high-level intermediate representation (`HIR`).

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
  pub fn kind(self: &Self) -> &ErrorKind { /* ... */ }
  ```
  Return the type of this error.

- ```rust
  pub fn pattern(self: &Self) -> &str { /* ... */ }
  ```
  The original pattern string in which this error occurred.

- ```rust
  pub fn span(self: &Self) -> &Span { /* ... */ }
  ```
  Return the span at which this error occurred.

- ```rust
  pub fn auxiliary_span(self: &Self) -> Option<&Span> { /* ... */ }
  ```
  Return an auxiliary span. This span exists only for some errors that

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Error**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

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

  - ```rust
    fn from(err: ast::Error) -> Error { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
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

#### Enum `ErrorKind`

**Attributes:**

- `#[non_exhaustive]`
- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The type of an error that occurred while building an AST.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

```rust
pub enum ErrorKind {
    CaptureLimitExceeded,
    ClassEscapeInvalid,
    ClassRangeInvalid,
    ClassRangeLiteral,
    ClassUnclosed,
    DecimalEmpty,
    DecimalInvalid,
    EscapeHexEmpty,
    EscapeHexInvalid,
    EscapeHexInvalidDigit,
    EscapeUnexpectedEof,
    EscapeUnrecognized,
    FlagDanglingNegation,
    FlagDuplicate {
        original: Span,
    },
    FlagRepeatedNegation {
        original: Span,
    },
    FlagUnexpectedEof,
    FlagUnrecognized,
    GroupNameDuplicate {
        original: Span,
    },
    GroupNameEmpty,
    GroupNameInvalid,
    GroupNameUnexpectedEof,
    GroupUnclosed,
    GroupUnopened,
    NestLimitExceeded(u32),
    RepetitionCountInvalid,
    RepetitionCountDecimalEmpty,
    RepetitionCountUnclosed,
    RepetitionMissing,
    SpecialWordBoundaryUnclosed,
    SpecialWordBoundaryUnrecognized,
    SpecialWordOrRepetitionUnexpectedEof,
    UnicodeClassInvalid,
    UnsupportedBackreference,
    UnsupportedLookAround,
}
```

##### Variants

###### `CaptureLimitExceeded`

The capturing group limit was exceeded.

Note that this represents a limit on the total number of capturing
groups in a regex and not necessarily the number of nested capturing
groups. That is, the nest limit can be low and it is still possible for
this error to occur.

###### `ClassEscapeInvalid`

An invalid escape sequence was found in a character class set.

###### `ClassRangeInvalid`

An invalid character class range was found. An invalid range is any
range where the start is greater than the end.

###### `ClassRangeLiteral`

An invalid range boundary was found in a character class. Range
boundaries must be a single literal codepoint, but this error indicates
that something else was found, such as a nested class.

###### `ClassUnclosed`

An opening `[` was found with no corresponding closing `]`.

###### `DecimalEmpty`

Note that this error variant is no longer used. Namely, a decimal
number can only appear as a repetition quantifier. When the number
in a repetition quantifier is empty, then it gets its own specialized
error, `RepetitionCountDecimalEmpty`.

###### `DecimalInvalid`

An invalid decimal number was given where one was expected.

###### `EscapeHexEmpty`

A bracketed hex literal was empty.

###### `EscapeHexInvalid`

A bracketed hex literal did not correspond to a Unicode scalar value.

###### `EscapeHexInvalidDigit`

An invalid hexadecimal digit was found.

###### `EscapeUnexpectedEof`

EOF was found before an escape sequence was completed.

###### `EscapeUnrecognized`

An unrecognized escape sequence.

###### `FlagDanglingNegation`

A dangling negation was used when setting flags, e.g., `i-`.

###### `FlagDuplicate`

A flag was used twice, e.g., `i-i`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `original` | `Span` | The position of the original flag. The error position<br>points to the duplicate flag. |

###### `FlagRepeatedNegation`

The negation operator was used twice, e.g., `-i-s`.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `original` | `Span` | The position of the original negation operator. The error position<br>points to the duplicate negation operator. |

###### `FlagUnexpectedEof`

Expected a flag but got EOF, e.g., `(?`.

###### `FlagUnrecognized`

Unrecognized flag, e.g., `a`.

###### `GroupNameDuplicate`

A duplicate capture name was found.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `original` | `Span` | The position of the initial occurrence of the capture name. The<br>error position itself points to the duplicate occurrence. |

###### `GroupNameEmpty`

A capture group name is empty, e.g., `(?P<>abc)`.

###### `GroupNameInvalid`

An invalid character was seen for a capture group name. This includes
errors where the first character is a digit (even though subsequent
characters are allowed to be digits).

###### `GroupNameUnexpectedEof`

A closing `>` could not be found for a capture group name.

###### `GroupUnclosed`

An unclosed group, e.g., `(ab`.

The span of this error corresponds to the unclosed parenthesis.

###### `GroupUnopened`

An unopened group, e.g., `ab)`.

###### `NestLimitExceeded`

The nest limit was exceeded. The limit stored here is the limit
configured in the parser.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

###### `RepetitionCountInvalid`

The range provided in a counted repetition operator is invalid. The
range is invalid if the start is greater than the end.

###### `RepetitionCountDecimalEmpty`

An opening `{` was not followed by a valid decimal value.
For example, `x{}` or `x{]}` would fail.

###### `RepetitionCountUnclosed`

An opening `{` was found with no corresponding closing `}`.

###### `RepetitionMissing`

A repetition operator was applied to a missing sub-expression. This
occurs, for example, in the regex consisting of just a `*` or even
`(?i)*`. It is, however, possible to create a repetition operating on
an empty sub-expression. For example, `()*` is still considered valid.

###### `SpecialWordBoundaryUnclosed`

The special word boundary syntax, `\b{something}`, was used, but
either EOF without `}` was seen, or an invalid character in the
braces was seen.

###### `SpecialWordBoundaryUnrecognized`

The special word boundary syntax, `\b{something}`, was used, but
`something` was not recognized as a valid word boundary kind.

###### `SpecialWordOrRepetitionUnexpectedEof`

The syntax `\b{` was observed, but afterwards the end of the pattern
was observed without being able to tell whether it was meant to be a
bounded repetition on the `\b` or the beginning of a special word
boundary assertion.

###### `UnicodeClassInvalid`

The Unicode class is not valid. This typically occurs when a `\p` is
followed by something other than a `{`.

###### `UnsupportedBackreference`

When octal support is disabled, this error is produced when an octal
escape is used. The octal escape is assumed to be an invocation of
a backreference, which is the common case.

###### `UnsupportedLookAround`

When syntax similar to PCRE's look-around is used, this error is
returned. Some example syntaxes that are rejected include, but are
not necessarily limited to, `(?=re)`, `(?!re)`, `(?<=re)` and
`(?<!re)`. Note that all of these syntaxes are otherwise invalid; this
error is used to improve the user experience.

##### Implementations

###### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ErrorKind { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ErrorKind) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

#### Struct `Span`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

Span represents the position information of a single AST item.

All span positions are absolute byte offsets that can be used on the
original regular expression that was parsed.

```rust
pub struct Span {
    pub start: Position,
    pub end: Position,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `Position` | The start byte offset. |
| `end` | `Position` | The end byte offset. |

##### Implementations

###### Methods

- ```rust
  pub fn new(start: Position, end: Position) -> Span { /* ... */ }
  ```
  Create a new span with the given positions.

- ```rust
  pub fn splat(pos: Position) -> Span { /* ... */ }
  ```
  Create a new span using the given position as the start and end.

- ```rust
  pub fn with_start(self: Self, pos: Position) -> Span { /* ... */ }
  ```
  Create a new span by replacing the starting the position with the one

- ```rust
  pub fn with_end(self: Self, pos: Position) -> Span { /* ... */ }
  ```
  Create a new span by replacing the ending the position with the one

- ```rust
  pub fn is_one_line(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this span occurs on a single line.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this span is empty. That is, it points to

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Span { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Span) -> Option<Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Span) -> bool { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Span) -> Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Position`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A single position in a regular expression.

A position encodes one half of a span, and include the byte offset, line
number and column number.

```rust
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `offset` | `usize` | The absolute offset of this position, starting at `0` from the<br>beginning of the regular expression pattern string. |
| `line` | `usize` | The line number, starting at `1`. |
| `column` | `usize` | The approximate column number, starting at `1`. |

##### Implementations

###### Methods

- ```rust
  pub fn new(offset: usize, line: usize, column: usize) -> Position { /* ... */ }
  ```
  Create a new position with the given information.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Position { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Position) -> Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Position) -> bool { /* ... */ }
    ```

- **Freeze**
- **Unpin**
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
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Send**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Position) -> Option<Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `WithComments`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

An abstract syntax tree for a singular expression along with comments
found.

Comments are not stored in the tree itself to avoid complexity. Each
comment contains a span of precisely where it occurred in the original
regular expression.

```rust
pub struct WithComments {
    pub ast: Ast,
    pub comments: alloc::vec::Vec<Comment>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ast` | `Ast` | The actual ast. |
| `comments` | `alloc::vec::Vec<Comment>` | All comments found in the original regular expression. |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WithComments { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WithComments) -> bool { /* ... */ }
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Unpin**
#### Struct `Comment`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A comment from a regular expression with an associated span.

A regular expression can only contain comments when the `x` flag is
enabled.

```rust
pub struct Comment {
    pub span: Span,
    pub comment: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this comment, including the beginning `#` and ending `\n`. |
| `comment` | `alloc::string::String` | The comment text, starting with the first character following the `#`<br>and ending with the last character preceding the `\n`. |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Send**
- **Eq**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Comment) -> bool { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Comment { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
#### Enum `Ast`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

An abstract syntax tree for a single regular expression.

An `Ast`'s `fmt::Display` implementation uses constant stack space and heap
space proportional to the size of the `Ast`.

This type defines its own destructor that uses constant stack space and
heap space proportional to the size of the `Ast`.

```rust
pub enum Ast {
    Empty(alloc::boxed::Box<Span>),
    Flags(alloc::boxed::Box<SetFlags>),
    Literal(alloc::boxed::Box<Literal>),
    Dot(alloc::boxed::Box<Span>),
    Assertion(alloc::boxed::Box<Assertion>),
    ClassUnicode(alloc::boxed::Box<ClassUnicode>),
    ClassPerl(alloc::boxed::Box<ClassPerl>),
    ClassBracketed(alloc::boxed::Box<ClassBracketed>),
    Repetition(alloc::boxed::Box<Repetition>),
    Group(alloc::boxed::Box<Group>),
    Alternation(alloc::boxed::Box<Alternation>),
    Concat(alloc::boxed::Box<Concat>),
}
```

##### Variants

###### `Empty`

An empty regex that matches everything.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Span>` |  |

###### `Flags`

A set of flags, e.g., `(?is)`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<SetFlags>` |  |

###### `Literal`

A single character literal, which includes escape sequences.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Literal>` |  |

###### `Dot`

The "any character" class.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Span>` |  |

###### `Assertion`

A single zero-width assertion.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Assertion>` |  |

###### `ClassUnicode`

A single Unicode character class, e.g., `\pL` or `\p{Greek}`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<ClassUnicode>` |  |

###### `ClassPerl`

A single perl character class, e.g., `\d` or `\W`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<ClassPerl>` |  |

###### `ClassBracketed`

A single bracketed character class set, which may contain zero or more
character ranges and/or zero or more nested classes. e.g.,
`[a-zA-Z\pL]`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<ClassBracketed>` |  |

###### `Repetition`

A repetition operator applied to an arbitrary regular expression.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Repetition>` |  |

###### `Group`

A grouped regular expression.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Group>` |  |

###### `Alternation`

An alternation of regular expressions.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Alternation>` |  |

###### `Concat`

A concatenation of regular expressions.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Concat>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn empty(span: Span) -> Ast { /* ... */ }
  ```
  Create an "empty" AST item.

- ```rust
  pub fn flags(e: SetFlags) -> Ast { /* ... */ }
  ```
  Create a "flags" AST item.

- ```rust
  pub fn literal(e: Literal) -> Ast { /* ... */ }
  ```
  Create a "literal" AST item.

- ```rust
  pub fn dot(span: Span) -> Ast { /* ... */ }
  ```
  Create a "dot" AST item.

- ```rust
  pub fn assertion(e: Assertion) -> Ast { /* ... */ }
  ```
  Create a "assertion" AST item.

- ```rust
  pub fn class_unicode(e: ClassUnicode) -> Ast { /* ... */ }
  ```
  Create a "Unicode class" AST item.

- ```rust
  pub fn class_perl(e: ClassPerl) -> Ast { /* ... */ }
  ```
  Create a "Perl class" AST item.

- ```rust
  pub fn class_bracketed(e: ClassBracketed) -> Ast { /* ... */ }
  ```
  Create a "bracketed class" AST item.

- ```rust
  pub fn repetition(e: Repetition) -> Ast { /* ... */ }
  ```
  Create a "repetition" AST item.

- ```rust
  pub fn group(e: Group) -> Ast { /* ... */ }
  ```
  Create a "group" AST item.

- ```rust
  pub fn alternation(e: Alternation) -> Ast { /* ... */ }
  ```
  Create a "alternation" AST item.

- ```rust
  pub fn concat(e: Concat) -> Ast { /* ... */ }
  ```
  Create a "concat" AST item.

- ```rust
  pub fn span(self: &Self) -> &Span { /* ... */ }
  ```
  Return the span of this abstract syntax tree.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Return true if and only if this Ast is empty.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ast) -> bool { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
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

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **Freeze**
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
    fn clone(self: &Self) -> Ast { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `Alternation`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

An alternation of regular expressions.

```rust
pub struct Alternation {
    pub span: Span,
    pub asts: alloc::vec::Vec<Ast>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this alternation. |
| `asts` | `alloc::vec::Vec<Ast>` | The alternate regular expressions. |

##### Implementations

###### Methods

- ```rust
  pub fn into_ast(self: Self) -> Ast { /* ... */ }
  ```
  Return this alternation as an AST.

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Alternation { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
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

- **Send**
- **UnwindSafe**
- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Alternation) -> bool { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
#### Struct `Concat`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A concatenation of regular expressions.

```rust
pub struct Concat {
    pub span: Span,
    pub asts: alloc::vec::Vec<Ast>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this concatenation. |
| `asts` | `alloc::vec::Vec<Ast>` | The concatenation regular expressions. |

##### Implementations

###### Methods

- ```rust
  pub fn into_ast(self: Self) -> Ast { /* ... */ }
  ```
  Return this concatenation as an AST.

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Concat) -> bool { /* ... */ }
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

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Concat { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
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

#### Struct `Literal`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A single literal expression.

A literal corresponds to a single Unicode scalar value. Literals may be
represented in their literal form, e.g., `a` or in their escaped form,
e.g., `\x61`.

```rust
pub struct Literal {
    pub span: Span,
    pub kind: LiteralKind,
    pub c: char,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this literal. |
| `kind` | `LiteralKind` | The kind of this literal. |
| `c` | `char` | The Unicode scalar value corresponding to this literal. |

##### Implementations

###### Methods

- ```rust
  pub fn byte(self: &Self) -> Option<u8> { /* ... */ }
  ```
  If this literal was written as a `\x` hex escape, then this returns

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Literal { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Literal) -> bool { /* ... */ }
    ```

#### Enum `LiteralKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The kind of a single literal expression.

```rust
pub enum LiteralKind {
    Verbatim,
    Meta,
    Superfluous,
    Octal,
    HexFixed(HexLiteralKind),
    HexBrace(HexLiteralKind),
    Special(SpecialLiteralKind),
}
```

##### Variants

###### `Verbatim`

The literal is written verbatim, e.g., `a` or `☃`.

###### `Meta`

The literal is written as an escape because it is otherwise a special
regex meta character, e.g., `\*` or `\[`.

###### `Superfluous`

The literal is written as an escape despite the fact that the escape is
unnecessary, e.g., `\%` or `\/`.

###### `Octal`

The literal is written as an octal escape, e.g., `\141`.

###### `HexFixed`

The literal is written as a hex code with a fixed number of digits
depending on the type of the escape, e.g., `\x61` or `\u0061` or
`\U00000061`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `HexLiteralKind` |  |

###### `HexBrace`

The literal is written as a hex code with a bracketed number of
digits. The only restriction is that the bracketed hex code must refer
to a valid Unicode scalar value.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `HexLiteralKind` |  |

###### `Special`

The literal is written as a specially recognized escape, e.g., `\f`
or `\n`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SpecialLiteralKind` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LiteralKind) -> bool { /* ... */ }
    ```

- **Unpin**
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

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> LiteralKind { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
#### Enum `SpecialLiteralKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The type of a special literal.

A special literal is a special escape sequence recognized by the regex
parser, e.g., `\f` or `\n`.

```rust
pub enum SpecialLiteralKind {
    Bell,
    FormFeed,
    Tab,
    LineFeed,
    CarriageReturn,
    VerticalTab,
    Space,
}
```

##### Variants

###### `Bell`

Bell, spelled `\a` (`\x07`).

###### `FormFeed`

Form feed, spelled `\f` (`\x0C`).

###### `Tab`

Tab, spelled `\t` (`\x09`).

###### `LineFeed`

Line feed, spelled `\n` (`\x0A`).

###### `CarriageReturn`

Carriage return, spelled `\r` (`\x0D`).

###### `VerticalTab`

Vertical tab, spelled `\v` (`\x0B`).

###### `Space`

Space, spelled `\ ` (`\x20`). Note that this can only appear when
parsing in verbose mode.

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SpecialLiteralKind { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SpecialLiteralKind) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Freeze**
#### Enum `HexLiteralKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The type of a Unicode hex literal.

Note that all variants behave the same when used with brackets. They only
differ when used without brackets in the number of hex digits that must
follow.

```rust
pub enum HexLiteralKind {
    X,
    UnicodeShort,
    UnicodeLong,
}
```

##### Variants

###### `X`

A `\x` prefix. When used without brackets, this form is limited to
two digits.

###### `UnicodeShort`

A `\u` prefix. When used without brackets, this form is limited to
four digits.

###### `UnicodeLong`

A `\U` prefix. When used without brackets, this form is limited to
eight digits.

##### Implementations

###### Methods

- ```rust
  pub fn digits(self: &Self) -> u32 { /* ... */ }
  ```
  The number of digits that must be used with this literal form when

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HexLiteralKind) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HexLiteralKind { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Sync**
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

#### Struct `ClassPerl`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A Perl character class.

```rust
pub struct ClassPerl {
    pub span: Span,
    pub kind: ClassPerlKind,
    pub negated: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this class. |
| `kind` | `ClassPerlKind` | The kind of Perl class. |
| `negated` | `bool` | Whether the class is negated or not. e.g., `\d` is not negated but<br>`\D` is. |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassPerl { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **UnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassPerl) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Enum `ClassPerlKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The available Perl character classes.

```rust
pub enum ClassPerlKind {
    Digit,
    Space,
    Word,
}
```

##### Variants

###### `Digit`

Decimal numbers.

###### `Space`

Whitespace.

###### `Word`

Word characters.

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassPerlKind) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassPerlKind { /* ... */ }
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

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **Eq**
- **StructuralPartialEq**
#### Struct `ClassAscii`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

An ASCII character class.

```rust
pub struct ClassAscii {
    pub span: Span,
    pub kind: ClassAsciiKind,
    pub negated: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this class. |
| `kind` | `ClassAsciiKind` | The kind of ASCII class. |
| `negated` | `bool` | Whether the class is negated or not. e.g., `[[:alpha:]]` is not negated<br>but `[[:^alpha:]]` is. |

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
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

- **StructuralPartialEq**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassAscii) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassAscii { /* ... */ }
    ```

- **Send**
#### Enum `ClassAsciiKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The available ASCII character classes.

```rust
pub enum ClassAsciiKind {
    Alnum,
    Alpha,
    Ascii,
    Blank,
    Cntrl,
    Digit,
    Graph,
    Lower,
    Print,
    Punct,
    Space,
    Upper,
    Word,
    Xdigit,
}
```

##### Variants

###### `Alnum`

`[0-9A-Za-z]`

###### `Alpha`

`[A-Za-z]`

###### `Ascii`

`[\x00-\x7F]`

###### `Blank`

`[ \t]`

###### `Cntrl`

`[\x00-\x1F\x7F]`

###### `Digit`

`[0-9]`

###### `Graph`

`[!-~]`

###### `Lower`

`[a-z]`

###### `Print`

`[ -~]`

###### `Punct`

`[!-/:-@\[-`{-~]`

###### `Space`

`[\t\n\v\f\r ]`

###### `Upper`

`[A-Z]`

###### `Word`

`[0-9A-Za-z_]`

###### `Xdigit`

`[0-9A-Fa-f]`

##### Implementations

###### Methods

- ```rust
  pub fn from_name(name: &str) -> Option<ClassAsciiKind> { /* ... */ }
  ```
  Return the corresponding ClassAsciiKind variant for the given name.

###### Trait Implementations

- **StructuralPartialEq**
- **Freeze**
- **Sync**
- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassAsciiKind) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassAsciiKind { /* ... */ }
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

- **Send**
- **Eq**
- **Unpin**
#### Struct `ClassUnicode`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A Unicode character class.

```rust
pub struct ClassUnicode {
    pub span: Span,
    pub negated: bool,
    pub kind: ClassUnicodeKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this class. |
| `negated` | `bool` | Whether this class is negated or not.<br><br>Note: be careful when using this attribute. This specifically refers<br>to whether the class is written as `\p` or `\P`, where the latter<br>is `negated = true`. However, it also possible to write something like<br>`\P{scx!=Katakana}` which is actually equivalent to<br>`\p{scx=Katakana}` and is therefore not actually negated even though<br>`negated = true` here. To test whether this class is truly negated<br>or not, use the `is_negated` method. |
| `kind` | `ClassUnicodeKind` | The kind of Unicode class. |

##### Implementations

###### Methods

- ```rust
  pub fn is_negated(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this class has been negated.

###### Trait Implementations

- **Freeze**
- **StructuralPartialEq**
- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassUnicode) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassUnicode { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
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

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `ClassUnicodeKind`

The available forms of Unicode character classes.

```rust
pub enum ClassUnicodeKind {
    OneLetter(char),
    Named(alloc::string::String),
    NamedValue {
        op: ClassUnicodeOpKind,
        name: alloc::string::String,
        value: alloc::string::String,
    },
}
```

##### Variants

###### `OneLetter`

A one letter abbreviated class, e.g., `\pN`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

###### `Named`

A binary property, general category or script. The string may be
empty.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `NamedValue`

A property name and an associated value.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `op` | `ClassUnicodeOpKind` | The type of Unicode op used to associate `name` with `value`. |
| `name` | `alloc::string::String` | The property name (which may be empty). |
| `value` | `alloc::string::String` | The property value (which may be empty). |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassUnicodeKind { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **StructuralPartialEq**
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

- **Unpin**
- **RefUnwindSafe**
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

- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassUnicodeKind) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Enum `ClassUnicodeOpKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The type of op used in a Unicode character class.

```rust
pub enum ClassUnicodeOpKind {
    Equal,
    Colon,
    NotEqual,
}
```

##### Variants

###### `Equal`

A property set to a specific value, e.g., `\p{scx=Katakana}`.

###### `Colon`

A property set to a specific value using a colon, e.g.,
`\p{scx:Katakana}`.

###### `NotEqual`

A property that isn't a particular value, e.g., `\p{scx!=Katakana}`.

##### Implementations

###### Methods

- ```rust
  pub fn is_equal(self: &Self) -> bool { /* ... */ }
  ```
  Whether the op is an equality op or not.

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassUnicodeOpKind { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassUnicodeOpKind) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `ClassBracketed`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A bracketed character class, e.g., `[a-z0-9]`.

```rust
pub struct ClassBracketed {
    pub span: Span,
    pub negated: bool,
    pub kind: ClassSet,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this class. |
| `negated` | `bool` | Whether this class is negated or not. e.g., `[a]` is not negated but<br>`[^a]` is. |
| `kind` | `ClassSet` | The type of this set. A set is either a normal union of things, e.g.,<br>`[abc]` or a result of applying set operations, e.g., `[\pL--c]`. |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassBracketed { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassBracketed) -> bool { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
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

- **StructuralPartialEq**
- **Eq**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `ClassSet`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A character class set.

This type corresponds to the internal structure of a bracketed character
class. That is, every bracketed character is one of two types: a union of
items (literals, ranges, other bracketed classes) or a tree of binary set
operations.

```rust
pub enum ClassSet {
    Item(ClassSetItem),
    BinaryOp(ClassSetBinaryOp),
}
```

##### Variants

###### `Item`

An item, which can be a single literal, range, nested character class
or a union of items.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassSetItem` |  |

###### `BinaryOp`

A single binary operation (i.e., &&, -- or ~~).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassSetBinaryOp` |  |

##### Implementations

###### Methods

- ```rust
  pub fn union(ast: ClassSetUnion) -> ClassSet { /* ... */ }
  ```
  Build a set from a union.

- ```rust
  pub fn span(self: &Self) -> &Span { /* ... */ }
  ```
  Return the span of this character class set.

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassSet { /* ... */ }
    ```

- **Eq**
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

- **UnwindSafe**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassSet) -> bool { /* ... */ }
    ```

#### Enum `ClassSetItem`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A single component of a character class set.

```rust
pub enum ClassSetItem {
    Empty(Span),
    Literal(Literal),
    Range(ClassSetRange),
    Ascii(ClassAscii),
    Unicode(ClassUnicode),
    Perl(ClassPerl),
    Bracketed(alloc::boxed::Box<ClassBracketed>),
    Union(ClassSetUnion),
}
```

##### Variants

###### `Empty`

An empty item.

Note that a bracketed character class cannot contain a single empty
item. Empty items can appear when using one of the binary operators.
For example, `[&&]` is the intersection of two empty classes.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Span` |  |

###### `Literal`

A single literal.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Literal` |  |

###### `Range`

A range between two literals.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassSetRange` |  |

###### `Ascii`

An ASCII character class, e.g., `[:alnum:]` or `[:punct:]`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassAscii` |  |

###### `Unicode`

A Unicode character class, e.g., `\pL` or `\p{Greek}`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassUnicode` |  |

###### `Perl`

A perl character class, e.g., `\d` or `\W`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassPerl` |  |

###### `Bracketed`

A bracketed character class set, which may contain zero or more
character ranges and/or zero or more nested classes. e.g.,
`[a-zA-Z\pL]`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<ClassBracketed>` |  |

###### `Union`

A union of items.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassSetUnion` |  |

##### Implementations

###### Methods

- ```rust
  pub fn span(self: &Self) -> &Span { /* ... */ }
  ```
  Return the span of this character class set item.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassSetItem) -> bool { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassSetItem { /* ... */ }
    ```

#### Struct `ClassSetRange`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A single character class range in a set.

```rust
pub struct ClassSetRange {
    pub span: Span,
    pub start: Literal,
    pub end: Literal,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this range. |
| `start` | `Literal` | The start of this range. |
| `end` | `Literal` | The end of this range. |

##### Implementations

###### Methods

- ```rust
  pub fn is_valid(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this character class range is valid.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassSetRange { /* ... */ }
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
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

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassSetRange) -> bool { /* ... */ }
    ```

- **Sync**
#### Struct `ClassSetUnion`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A union of items inside a character class set.

```rust
pub struct ClassSetUnion {
    pub span: Span,
    pub items: alloc::vec::Vec<ClassSetItem>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of the items in this operation. e.g., the `a-z0-9` in<br>`[^a-z0-9]` |
| `items` | `alloc::vec::Vec<ClassSetItem>` | The sequence of items that make up this union. |

##### Implementations

###### Methods

- ```rust
  pub fn push(self: &mut Self, item: ClassSetItem) { /* ... */ }
  ```
  Push a new item in this union.

- ```rust
  pub fn into_item(self: Self) -> ClassSetItem { /* ... */ }
  ```
  Return this union as a character class set item.

###### Trait Implementations

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassSetUnion) -> bool { /* ... */ }
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

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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
    fn clone(self: &Self) -> ClassSetUnion { /* ... */ }
    ```

#### Struct `ClassSetBinaryOp`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A Unicode character class set operation.

```rust
pub struct ClassSetBinaryOp {
    pub span: Span,
    pub kind: ClassSetBinaryOpKind,
    pub lhs: alloc::boxed::Box<ClassSet>,
    pub rhs: alloc::boxed::Box<ClassSet>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this operation. e.g., the `a-z--[h-p]` in `[a-z--h-p]`. |
| `kind` | `ClassSetBinaryOpKind` | The type of this set operation. |
| `lhs` | `alloc::boxed::Box<ClassSet>` | The left hand side of the operation. |
| `rhs` | `alloc::boxed::Box<ClassSet>` | The right hand side of the operation. |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassSetBinaryOp { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassSetBinaryOp) -> bool { /* ... */ }
    ```

- **Unpin**
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

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `ClassSetBinaryOpKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The type of a Unicode character class set operation.

Note that this doesn't explicitly represent union since there is no
explicit union operator. Concatenation inside a character class corresponds
to the union operation.

```rust
pub enum ClassSetBinaryOpKind {
    Intersection,
    Difference,
    SymmetricDifference,
}
```

##### Variants

###### `Intersection`

The intersection of two sets, e.g., `\pN&&[a-z]`.

###### `Difference`

The difference of two sets, e.g., `\pN--[0-9]`.

###### `SymmetricDifference`

The symmetric difference of two sets. The symmetric difference is the
set of elements belonging to one but not both sets.
e.g., `[\pL~~[:ascii:]]`.

##### Implementations

###### Trait Implementations

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Eq**
- **RefUnwindSafe**
- **Unpin**
- **StructuralPartialEq**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassSetBinaryOpKind { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassSetBinaryOpKind) -> bool { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Assertion`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A single zero-width assertion.

```rust
pub struct Assertion {
    pub span: Span,
    pub kind: AssertionKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this assertion. |
| `kind` | `AssertionKind` | The assertion kind, e.g., `\b` or `^`. |

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Assertion) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Assertion { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

#### Enum `AssertionKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

An assertion kind.

```rust
pub enum AssertionKind {
    StartLine,
    EndLine,
    StartText,
    EndText,
    WordBoundary,
    NotWordBoundary,
    WordBoundaryStart,
    WordBoundaryEnd,
    WordBoundaryStartAngle,
    WordBoundaryEndAngle,
    WordBoundaryStartHalf,
    WordBoundaryEndHalf,
}
```

##### Variants

###### `StartLine`

`^`

###### `EndLine`

`$`

###### `StartText`

`\A`

###### `EndText`

`\z`

###### `WordBoundary`

`\b`

###### `NotWordBoundary`

`\B`

###### `WordBoundaryStart`

`\b{start}`

###### `WordBoundaryEnd`

`\b{end}`

###### `WordBoundaryStartAngle`

`\<` (alias for `\b{start}`)

###### `WordBoundaryEndAngle`

`\>` (alias for `\b{end}`)

###### `WordBoundaryStartHalf`

`\b{start-half}`

###### `WordBoundaryEndHalf`

`\b{end-half}`

##### Implementations

###### Trait Implementations

- **Eq**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AssertionKind) -> bool { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AssertionKind { /* ... */ }
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
#### Struct `Repetition`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A repetition operation applied to a regular expression.

```rust
pub struct Repetition {
    pub span: Span,
    pub op: RepetitionOp,
    pub greedy: bool,
    pub ast: alloc::boxed::Box<Ast>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this operation. |
| `op` | `RepetitionOp` | The actual operation. |
| `greedy` | `bool` | Whether this operation was applied greedily or not. |
| `ast` | `alloc::boxed::Box<Ast>` | The regular expression under repetition. |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Repetition) -> bool { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
    fn clone(self: &Self) -> Repetition { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `RepetitionOp`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The repetition operator itself.

```rust
pub struct RepetitionOp {
    pub span: Span,
    pub kind: RepetitionKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this operator. This includes things like `+`, `*?` and<br>`{m,n}`. |
| `kind` | `RepetitionKind` | The type of operation. |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Sync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RepetitionOp) -> bool { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RepetitionOp { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `RepetitionKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The kind of a repetition operator.

```rust
pub enum RepetitionKind {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Range(RepetitionRange),
}
```

##### Variants

###### `ZeroOrOne`

`?`

###### `ZeroOrMore`

`*`

###### `OneOrMore`

`+`

###### `Range`

`{m,n}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `RepetitionRange` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RepetitionKind) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RepetitionKind { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `RepetitionRange`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A range repetition operator.

```rust
pub enum RepetitionRange {
    Exactly(u32),
    AtLeast(u32),
    Bounded(u32, u32),
}
```

##### Variants

###### `Exactly`

`{m}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

###### `AtLeast`

`{m,}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

###### `Bounded`

`{m,n}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |
| 1 | `u32` |  |

##### Implementations

###### Methods

- ```rust
  pub fn is_valid(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this repetition range is valid.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RepetitionRange) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Unpin**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RepetitionRange { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
#### Struct `Group`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A grouped regular expression.

This includes both capturing and non-capturing groups. This does **not**
include flag-only groups like `(?is)`, but does contain any group that
contains a sub-expression, e.g., `(a)`, `(?P<name>a)`, `(?:a)` and
`(?is:a)`.

```rust
pub struct Group {
    pub span: Span,
    pub kind: GroupKind,
    pub ast: alloc::boxed::Box<Ast>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this group. |
| `kind` | `GroupKind` | The kind of this group. |
| `ast` | `alloc::boxed::Box<Ast>` | The regular expression in this group. |

##### Implementations

###### Methods

- ```rust
  pub fn flags(self: &Self) -> Option<&Flags> { /* ... */ }
  ```
  If this group is non-capturing, then this returns the (possibly empty)

- ```rust
  pub fn is_capturing(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this group is capturing.

- ```rust
  pub fn capture_index(self: &Self) -> Option<u32> { /* ... */ }
  ```
  Returns the capture index of this group, if this is a capturing group.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Group { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Group) -> bool { /* ... */ }
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

#### Enum `GroupKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The kind of a group.

```rust
pub enum GroupKind {
    CaptureIndex(u32),
    CaptureName {
        starts_with_p: bool,
        name: CaptureName,
    },
    NonCapturing(Flags),
}
```

##### Variants

###### `CaptureIndex`

`(a)`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

###### `CaptureName`

`(?<name>a)` or `(?P<name>a)`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `starts_with_p` | `bool` | True if the `?P<` syntax is used and false if the `?<` syntax is used. |
| `name` | `CaptureName` | The capture name. |

###### `NonCapturing`

`(?:a)` and `(?i:a)`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Flags` |  |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GroupKind) -> bool { /* ... */ }
    ```

- **Send**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GroupKind { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **Unpin**
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

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `CaptureName`

A capture name.

This corresponds to the name itself between the angle brackets in, e.g.,
`(?P<foo>expr)`.

```rust
pub struct CaptureName {
    pub span: Span,
    pub name: alloc::string::String,
    pub index: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this capture name. |
| `name` | `alloc::string::String` | The capture name. |
| `index` | `u32` | The capture index. |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CaptureName) -> bool { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **Unpin**
- **Eq**
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

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CaptureName { /* ... */ }
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

#### Struct `SetFlags`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A group of flags that is not applied to a particular regular expression.

```rust
pub struct SetFlags {
    pub span: Span,
    pub flags: Flags,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of these flags, including the grouping parentheses. |
| `flags` | `Flags` | The actual sequence of flags. |

##### Implementations

###### Trait Implementations

- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SetFlags { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SetFlags) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Eq**
#### Struct `Flags`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A group of flags.

This corresponds only to the sequence of flags themselves, e.g., `is-u`.

```rust
pub struct Flags {
    pub span: Span,
    pub items: alloc::vec::Vec<FlagsItem>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this group of flags. |
| `items` | `alloc::vec::Vec<FlagsItem>` | A sequence of flag items. Each item is either a flag or a negation<br>operator. |

##### Implementations

###### Methods

- ```rust
  pub fn add_item(self: &mut Self, item: FlagsItem) -> Option<usize> { /* ... */ }
  ```
  Add the given item to this sequence of flags.

- ```rust
  pub fn flag_state(self: &Self, flag: Flag) -> Option<bool> { /* ... */ }
  ```
  Returns the state of the given flag in this set.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Flags { /* ... */ }
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

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Flags) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `FlagsItem`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A single item in a group of flags.

```rust
pub struct FlagsItem {
    pub span: Span,
    pub kind: FlagsItemKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span` | `Span` | The span of this item. |
| `kind` | `FlagsItemKind` | The kind of this item. |

##### Implementations

###### Trait Implementations

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FlagsItem { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FlagsItem) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `FlagsItemKind`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

The kind of an item in a group of flags.

```rust
pub enum FlagsItemKind {
    Negation,
    Flag(Flag),
}
```

##### Variants

###### `Negation`

A negation operator applied to all subsequent flags in the enclosing
group.

###### `Flag`

A single flag in a group.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Flag` |  |

##### Implementations

###### Methods

- ```rust
  pub fn is_negation(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this item is a negation operator.

###### Trait Implementations

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FlagsItemKind { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FlagsItemKind) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
#### Enum `Flag`

**Attributes:**

- `#[<cfg_attr>(feature = "arbitrary", derive(arbitrary::Arbitrary))]`

A single flag.

```rust
pub enum Flag {
    CaseInsensitive,
    MultiLine,
    DotMatchesNewLine,
    SwapGreed,
    Unicode,
    CRLF,
    IgnoreWhitespace,
}
```

##### Variants

###### `CaseInsensitive`

`i`

###### `MultiLine`

`m`

###### `DotMatchesNewLine`

`s`

###### `SwapGreed`

`U`

###### `Unicode`

`u`

###### `CRLF`

`R`

###### `IgnoreWhitespace`

`x`

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Flag { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Unpin**
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

- **Freeze**
- **Eq**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Flag) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Re-exports

#### Re-export `visit`

```rust
pub use crate::ast::visitor::visit;
```

#### Re-export `Visitor`

```rust
pub use crate::ast::visitor::Visitor;
```

## Module `hir`

Defines a high-level intermediate (HIR) representation for regular expressions.

The HIR is represented by the [`Hir`] type, and it principally constructed via
[translation](translate) from an [`Ast`](crate::ast::Ast). Alternatively, users
may use the smart constructors defined on `Hir` to build their own by hand. The
smart constructors simultaneously simplify and "optimize" the HIR, and are also
the same routines used by translation.

Most regex engines only have an HIR like this, and usually construct it
directly from the concrete syntax. This crate however first parses the
concrete syntax into an `Ast`, and only then creates the HIR from the `Ast`,
as mentioned above. It's done this way to facilitate better error reporting,
and to have a structured representation of a regex that faithfully represents
its concrete syntax. Namely, while an `Hir` value can be converted back to an
equivalent regex pattern string, it is unlikely to look like the original due
to its simplified structure.

```rust
pub mod hir { /* ... */ }
```

### Modules

## Module `literal`

Provides literal extraction from `Hir` expressions.

An [`Extractor`] pulls literals out of [`Hir`] expressions and returns a
[`Seq`] of [`Literal`]s.

The purpose of literal extraction is generally to provide avenues for
optimizing regex searches. The main idea is that substring searches can be an
order of magnitude faster than a regex search. Therefore, if one can execute
a substring search to find candidate match locations and only run the regex
search at those locations, then it is possible for huge improvements in
performance to be realized.

With that said, literal optimizations are generally a black art because even
though substring search is generally faster, if the number of candidates
produced is high, then it can create a lot of overhead by ping-ponging between
the substring search and the regex search.

Here are some heuristics that might be used to help increase the chances of
effective literal optimizations:

* Stick to small [`Seq`]s. If you search for too many literals, it's likely
to lead to substring search that is only a little faster than a regex search,
and thus the overhead of using literal optimizations in the first place might
make things slower overall.
* The literals in your [`Seq`] shouldn't be too short. In general, longer is
better. A sequence corresponding to single bytes that occur frequently in the
haystack, for example, is probably a bad literal optimization because it's
likely to produce many false positive candidates. Longer literals are less
likely to match, and thus probably produce fewer false positives.
* If it's possible to estimate the approximate frequency of each byte according
to some pre-computed background distribution, it is possible to compute a score
of how "good" a `Seq` is. If a `Seq` isn't good enough, you might consider
skipping the literal optimization and just use the regex engine.

(It should be noted that there are always pathological cases that can make
any kind of literal optimization be a net slower result. This is why it
might be a good idea to be conservative, or to even provide a means for
literal optimizations to be dynamically disabled if they are determined to be
ineffective according to some measure.)

You're encouraged to explore the methods on [`Seq`], which permit shrinking
the size of sequences in a preference-order preserving fashion.

Finally, note that it isn't strictly necessary to use an [`Extractor`]. Namely,
an `Extractor` only uses public APIs of the [`Seq`] and [`Literal`] types,
so it is possible to implement your own extractor. For example, for n-grams
or "inner" literals (i.e., not prefix or suffix literals). The `Extractor`
is mostly responsible for the case analysis over `Hir` expressions. Much of
the "trickier" parts are how to combine literal sequences, and that is all
implemented on [`Seq`].

```rust
pub mod literal { /* ... */ }
```

### Types

#### Struct `Extractor`

Extracts prefix or suffix literal sequences from [`Hir`] expressions.

Literal extraction is based on the following observations:

* Many regexes start with one or a small number of literals.
* Substring search for literals is often much faster (sometimes by an order
of magnitude) than a regex search.

Thus, in many cases, one can search for literals to find candidate starting
locations of a match, and then only run the full regex engine at each such
location instead of over the full haystack.

The main downside of literal extraction is that it can wind up causing a
search to be slower overall. For example, if there are many matches or if
there are many candidates that don't ultimately lead to a match, then a
lot of overhead will be spent in shuffing back-and-forth between substring
search and the regex engine. This is the fundamental reason why literal
optimizations for regex patterns is sometimes considered a "black art."

# Look-around assertions

Literal extraction treats all look-around assertions as-if they match every
empty string. So for example, the regex `\bquux\b` will yield a sequence
containing a single exact literal `quux`. However, not all occurrences
of `quux` correspond to a match a of the regex. For example, `\bquux\b`
does not match `ZquuxZ` anywhere because `quux` does not fall on a word
boundary.

In effect, if your regex contains look-around assertions, then a match of
an exact literal does not necessarily mean the regex overall matches. So
you may still need to run the regex engine in such cases to confirm the
match.

The precise guarantee you get from a literal sequence is: if every literal
in the sequence is exact and the original regex contains zero look-around
assertions, then a preference-order multi-substring search of those
literals will precisely match a preference-order search of the original
regex.

# Example

This shows how to extract prefixes:

```
use regex_syntax::{hir::literal::{Extractor, Literal, Seq}, parse};

let hir = parse(r"(a|b|c)(x|y|z)[A-Z]+foo")?;
let got = Extractor::new().extract(&hir);
// All literals returned are "inexact" because none of them reach the
// match state.
let expected = Seq::from_iter([
    Literal::inexact("ax"),
    Literal::inexact("ay"),
    Literal::inexact("az"),
    Literal::inexact("bx"),
    Literal::inexact("by"),
    Literal::inexact("bz"),
    Literal::inexact("cx"),
    Literal::inexact("cy"),
    Literal::inexact("cz"),
]);
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

This shows how to extract suffixes:

```
use regex_syntax::{
    hir::literal::{Extractor, ExtractKind, Literal, Seq},
    parse,
};

let hir = parse(r"foo|[A-Z]+bar")?;
let got = Extractor::new().kind(ExtractKind::Suffix).extract(&hir);
// Since 'foo' gets to a match state, it is considered exact. But 'bar'
// does not because of the '[A-Z]+', and thus is marked inexact.
let expected = Seq::from_iter([
    Literal::exact("foo"),
    Literal::inexact("bar"),
]);
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct Extractor {
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
  pub fn new() -> Extractor { /* ... */ }
  ```
  Create a new extractor with a default configuration.

- ```rust
  pub fn extract(self: &Self, hir: &Hir) -> Seq { /* ... */ }
  ```
  Execute the extractor and return a sequence of literals.

- ```rust
  pub fn kind(self: &mut Self, kind: ExtractKind) -> &mut Extractor { /* ... */ }
  ```
  Set the kind of literal sequence to extract from an [`Hir`] expression.

- ```rust
  pub fn limit_class(self: &mut Self, limit: usize) -> &mut Extractor { /* ... */ }
  ```
  Configure a limit on the length of the sequence that is permitted for

- ```rust
  pub fn limit_repeat(self: &mut Self, limit: usize) -> &mut Extractor { /* ... */ }
  ```
  Configure a limit on the total number of repetitions that is permitted

- ```rust
  pub fn limit_literal_len(self: &mut Self, limit: usize) -> &mut Extractor { /* ... */ }
  ```
  Configure a limit on the maximum length of any literal in a sequence.

- ```rust
  pub fn limit_total(self: &mut Self, limit: usize) -> &mut Extractor { /* ... */ }
  ```
  Configure a limit on the total number of literals that will be

###### Trait Implementations

- **Freeze**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Extractor { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Sync**
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
    fn clone(self: &Self) -> Extractor { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Enum `ExtractKind`

**Attributes:**

- `#[non_exhaustive]`

The kind of literals to extract from an [`Hir`] expression.

The default extraction kind is `Prefix`.

```rust
pub enum ExtractKind {
    Prefix,
    Suffix,
}
```

##### Variants

###### `Prefix`

Extracts only prefix literals from a regex.

###### `Suffix`

Extracts only suffix literals from a regex.

Note that the sequence returned by suffix literals currently may
not correctly represent leftmost-first or "preference" order match
semantics.

##### Implementations

###### Methods

- ```rust
  pub fn is_prefix(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this kind is the `Prefix` variant.

- ```rust
  pub fn is_suffix(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this kind is the `Suffix` variant.

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExtractKind { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ExtractKind { /* ... */ }
    ```

#### Struct `Seq`

A sequence of literals.

A `Seq` is very much like a set in that it represents a union of its
members. That is, it corresponds to a set of literals where at least one
must match in order for a particular [`Hir`] expression to match. (Whether
this corresponds to the entire `Hir` expression, a prefix of it or a suffix
of it depends on how the `Seq` was extracted from the `Hir`.)

It is also unlike a set in that multiple identical literals may appear,
and that the order of the literals in the `Seq` matters. For example, if
the sequence is `[sam, samwise]` and leftmost-first matching is used, then
`samwise` can never match and the sequence is equivalent to `[sam]`.

# States of a sequence

A `Seq` has a few different logical states to consider:

* The sequence can represent "any" literal. When this happens, the set does
not have a finite size. The purpose of this state is to inhibit callers
from making assumptions about what literals are required in order to match
a particular [`Hir`] expression. Generally speaking, when a set is in this
state, literal optimizations are inhibited. A good example of a regex that
will cause this sort of set to appear is `[A-Za-z]`. The character class
is just too big (and also too narrow) to be usefully expanded into 52
different literals. (Note that the decision for when a seq should become
infinite is determined by the caller. A seq itself has no hard-coded
limits.)
* The sequence can be empty, in which case, it is an affirmative statement
that there are no literals that can match the corresponding `Hir`.
Consequently, the `Hir` never matches any input. For example, `[a&&b]`.
* The sequence can be non-empty, in which case, at least one of the
literals must match in order for the corresponding `Hir` to match.

# Example

This example shows how literal sequences can be simplified by stripping
suffixes and minimizing while maintaining preference order.

```
use regex_syntax::hir::literal::{Literal, Seq};

let mut seq = Seq::new(&[
    "farm",
    "appliance",
    "faraway",
    "apple",
    "fare",
    "gap",
    "applicant",
    "applaud",
]);
seq.keep_first_bytes(3);
seq.minimize_by_preference();
// Notice that 'far' comes before 'app', which matches the order in the
// original sequence. This guarantees that leftmost-first semantics are
// not altered by simplifying the set.
let expected = Seq::from_iter([
    Literal::inexact("far"),
    Literal::inexact("app"),
    Literal::exact("gap"),
]);
assert_eq!(expected, seq);
```

```rust
pub struct Seq {
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
  pub fn empty() -> Seq { /* ... */ }
  ```
  Returns an empty sequence.

- ```rust
  pub fn infinite() -> Seq { /* ... */ }
  ```
  Returns a sequence of literals without a finite size and may contain

- ```rust
  pub fn singleton(lit: Literal) -> Seq { /* ... */ }
  ```
  Returns a sequence containing a single literal.

- ```rust
  pub fn new<I, B>(it: I) -> Seq
where
    I: IntoIterator<Item = B>,
    B: AsRef<[u8]> { /* ... */ }
  ```
  Returns a sequence of exact literals from the given byte strings.

- ```rust
  pub fn literals(self: &Self) -> Option<&[Literal]> { /* ... */ }
  ```
  If this is a finite sequence, return its members as a slice of

- ```rust
  pub fn push(self: &mut Self, lit: Literal) { /* ... */ }
  ```
  Push a literal to the end of this sequence.

- ```rust
  pub fn make_inexact(self: &mut Self) { /* ... */ }
  ```
  Make all of the literals in this sequence inexact.

- ```rust
  pub fn make_infinite(self: &mut Self) { /* ... */ }
  ```
  Converts this sequence to an infinite sequence.

- ```rust
  pub fn cross_forward(self: &mut Self, other: &mut Seq) { /* ... */ }
  ```
  Modify this sequence to contain the cross product between it and the

- ```rust
  pub fn cross_reverse(self: &mut Self, other: &mut Seq) { /* ... */ }
  ```
  Modify this sequence to contain the cross product between it and

- ```rust
  pub fn union(self: &mut Self, other: &mut Seq) { /* ... */ }
  ```
  Unions the `other` sequence into this one.

- ```rust
  pub fn union_into_empty(self: &mut Self, other: &mut Seq) { /* ... */ }
  ```
  Unions the `other` sequence into this one by splice the `other`

- ```rust
  pub fn dedup(self: &mut Self) { /* ... */ }
  ```
  Deduplicate adjacent equivalent literals in this sequence.

- ```rust
  pub fn sort(self: &mut Self) { /* ... */ }
  ```
  Sorts this sequence of literals lexicographically.

- ```rust
  pub fn reverse_literals(self: &mut Self) { /* ... */ }
  ```
  Reverses all of the literals in this sequence.

- ```rust
  pub fn minimize_by_preference(self: &mut Self) { /* ... */ }
  ```
  Shrinks this seq to its minimal size while respecting the preference

- ```rust
  pub fn keep_first_bytes(self: &mut Self, len: usize) { /* ... */ }
  ```
  Trims all literals in this seq such that only the first `len` bytes

- ```rust
  pub fn keep_last_bytes(self: &mut Self, len: usize) { /* ... */ }
  ```
  Trims all literals in this seq such that only the last `len` bytes

- ```rust
  pub fn is_finite(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this sequence is finite.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this sequence is finite and empty.

- ```rust
  pub fn len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the number of literals in this sequence if the sequence is

- ```rust
  pub fn is_exact(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if all literals in this sequence are exact.

- ```rust
  pub fn is_inexact(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if all literals in this sequence are inexact.

- ```rust
  pub fn max_union_len(self: &Self, other: &Seq) -> Option<usize> { /* ... */ }
  ```
  Return the maximum length of the sequence that would result from

- ```rust
  pub fn max_cross_len(self: &Self, other: &Seq) -> Option<usize> { /* ... */ }
  ```
  Return the maximum length of the sequence that would result from the

- ```rust
  pub fn min_literal_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length of the shortest literal in this sequence.

- ```rust
  pub fn max_literal_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length of the longest literal in this sequence.

- ```rust
  pub fn longest_common_prefix(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
  Returns the longest common prefix from this seq.

- ```rust
  pub fn longest_common_suffix(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
  Returns the longest common suffix from this seq.

- ```rust
  pub fn optimize_for_prefix_by_preference(self: &mut Self) { /* ... */ }
  ```
  Optimizes this seq while treating its literals as prefixes and

- ```rust
  pub fn optimize_for_suffix_by_preference(self: &mut Self) { /* ... */ }
  ```
  Optimizes this seq while treating its literals as suffixes and

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Seq { /* ... */ }
    ```

- **StructuralPartialEq**
- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Seq) -> bool { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Send**
#### Struct `Literal`

A single literal extracted from an [`Hir`] expression.

A literal is composed of two things:

* A sequence of bytes. No guarantees with respect to UTF-8 are provided.
In particular, even if the regex a literal is extracted from is UTF-8, the
literal extracted may not be valid UTF-8. (For example, if an [`Extractor`]
limit resulted in trimming a literal in a way that splits a codepoint.)
* Whether the literal is "exact" or not. An "exact" literal means that it
has not been trimmed, and may continue to be extended. If a literal is
"exact" after visiting the entire `Hir` expression, then this implies that
the literal leads to a match state. (Although it doesn't necessarily imply
all occurrences of the literal correspond to a match of the regex, since
literal extraction ignores look-around assertions.)

```rust
pub struct Literal {
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
  pub fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal { /* ... */ }
  ```
  Returns a new exact literal containing the bytes given.

- ```rust
  pub fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal { /* ... */ }
  ```
  Returns a new inexact literal containing the bytes given.

- ```rust
  pub fn as_bytes(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns the bytes in this literal.

- ```rust
  pub fn into_bytes(self: Self) -> Vec<u8> { /* ... */ }
  ```
  Yields ownership of the bytes inside this literal.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the length of this literal in bytes.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this literal has zero bytes.

- ```rust
  pub fn is_exact(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this literal is exact.

- ```rust
  pub fn make_inexact(self: &mut Self) { /* ... */ }
  ```
  Marks this literal as inexact.

- ```rust
  pub fn reverse(self: &mut Self) { /* ... */ }
  ```
  Reverse the bytes in this literal.

- ```rust
  pub fn extend(self: &mut Self, lit: &Literal) { /* ... */ }
  ```
  Extend this literal with the literal given.

- ```rust
  pub fn keep_first_bytes(self: &mut Self, len: usize) { /* ... */ }
  ```
  Trims this literal such that only the first `len` bytes remain. If

- ```rust
  pub fn keep_last_bytes(self: &mut Self, len: usize) { /* ... */ }
  ```
  Trims this literal such that only the last `len` bytes remain. If this

###### Trait Implementations

- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq { /* ... */ }
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

  - ```rust
    fn from(byte: u8) -> Literal { /* ... */ }
    ```

  - ```rust
    fn from(ch: char) -> Literal { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Literal { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Literal) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Literal) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Literal) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
### Functions

#### Function `rank`

Returns the "rank" of the given byte.

The minimum rank value is `0` and the maximum rank value is `255`.

The rank of a byte is derived from a heuristic background distribution of
relative frequencies of bytes. The heuristic says that lower the rank of a
byte, the less likely that byte is to appear in any arbitrary haystack.

```rust
pub fn rank(byte: u8) -> u8 { /* ... */ }
```

## Module `print`

This module provides a regular expression printer for `Hir`.

```rust
pub mod print { /* ... */ }
```

### Types

#### Struct `Printer`

A printer for a regular expression's high-level intermediate
representation.

A printer converts a high-level intermediate representation (HIR) to a
regular expression pattern string. This particular printer uses constant
stack space and heap space proportional to the size of the HIR.

Since this printer is only using the HIR, the pattern it prints will likely
not resemble the original pattern at all. For example, a pattern like
`\pL` will have its entire class written out.

The purpose of this printer is to provide a means to mutate an HIR and then
build a regular expression from the result of that mutation. (A regex
library could provide a constructor from this HIR explicitly, but that
creates an unnecessary public coupling between the regex library and this
specific HIR representation.)

```rust
pub struct Printer {
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
  pub fn new() -> Printer { /* ... */ }
  ```
  Create a new printer.

- ```rust
  pub fn print<W: fmt::Write>(self: &mut Self, hir: &Hir, wtr: W) -> fmt::Result { /* ... */ }
  ```
  Print the given `Ast` to the given writer. The writer must implement

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Freeze**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
## Module `translate`

Defines a translator that converts an `Ast` to an `Hir`.

```rust
pub mod translate { /* ... */ }
```

### Types

#### Struct `TranslatorBuilder`

A builder for constructing an AST->HIR translator.

```rust
pub struct TranslatorBuilder {
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
  pub fn new() -> TranslatorBuilder { /* ... */ }
  ```
  Create a new translator builder with a default c onfiguration.

- ```rust
  pub fn build(self: &Self) -> Translator { /* ... */ }
  ```
  Build a translator using the current configuration.

- ```rust
  pub fn utf8(self: &mut Self, yes: bool) -> &mut TranslatorBuilder { /* ... */ }
  ```
  When disabled, translation will permit the construction of a regular

- ```rust
  pub fn line_terminator(self: &mut Self, byte: u8) -> &mut TranslatorBuilder { /* ... */ }
  ```
  Sets the line terminator for use with `(?u-s:.)` and `(?-us:.)`.

- ```rust
  pub fn case_insensitive(self: &mut Self, yes: bool) -> &mut TranslatorBuilder { /* ... */ }
  ```
  Enable or disable the case insensitive flag (`i`) by default.

- ```rust
  pub fn multi_line(self: &mut Self, yes: bool) -> &mut TranslatorBuilder { /* ... */ }
  ```
  Enable or disable the multi-line matching flag (`m`) by default.

- ```rust
  pub fn dot_matches_new_line(self: &mut Self, yes: bool) -> &mut TranslatorBuilder { /* ... */ }
  ```
  Enable or disable the "dot matches any character" flag (`s`) by

- ```rust
  pub fn crlf(self: &mut Self, yes: bool) -> &mut TranslatorBuilder { /* ... */ }
  ```
  Enable or disable the CRLF mode flag (`R`) by default.

- ```rust
  pub fn swap_greed(self: &mut Self, yes: bool) -> &mut TranslatorBuilder { /* ... */ }
  ```
  Enable or disable the "swap greed" flag (`U`) by default.

- ```rust
  pub fn unicode(self: &mut Self, yes: bool) -> &mut TranslatorBuilder { /* ... */ }
  ```
  Enable or disable the Unicode flag (`u`) by default.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TranslatorBuilder { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
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

- **Default**
  - ```rust
    fn default() -> TranslatorBuilder { /* ... */ }
    ```

#### Struct `Translator`

A translator maps abstract syntax to a high level intermediate
representation.

A translator may be benefit from reuse. That is, a translator can translate
many abstract syntax trees.

A `Translator` can be configured in more detail via a
[`TranslatorBuilder`].

```rust
pub struct Translator {
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
  pub fn new() -> Translator { /* ... */ }
  ```
  Create a new translator using the default configuration.

- ```rust
  pub fn translate(self: &mut Self, pattern: &str, ast: &Ast) -> core::result::Result<Hir, crate::hir::Error> { /* ... */ }
  ```
  Translate the given abstract syntax tree (AST) into a high level

###### Trait Implementations

- **Send**
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Translator { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

### Types

#### Struct `Error`

An error that can occur while translating an `Ast` to a `Hir`.

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
  pub fn kind(self: &Self) -> &ErrorKind { /* ... */ }
  ```
  Return the type of this error.

- ```rust
  pub fn pattern(self: &Self) -> &str { /* ... */ }
  ```
  The original pattern string in which this error occurred.

- ```rust
  pub fn span(self: &Self) -> &Span { /* ... */ }
  ```
  Return the span at which this error occurred.

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: hir::Error) -> Error { /* ... */ }
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

- **Error**
- **StructuralPartialEq**
- **Eq**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Freeze**
#### Enum `ErrorKind`

**Attributes:**

- `#[non_exhaustive]`

The type of an error that occurred while building an `Hir`.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

```rust
pub enum ErrorKind {
    UnicodeNotAllowed,
    InvalidUtf8,
    InvalidLineTerminator,
    UnicodePropertyNotFound,
    UnicodePropertyValueNotFound,
    UnicodePerlClassNotFound,
    UnicodeCaseUnavailable,
}
```

##### Variants

###### `UnicodeNotAllowed`

This error occurs when a Unicode feature is used when Unicode
support is disabled. For example `(?-u:\pL)` would trigger this error.

###### `InvalidUtf8`

This error occurs when translating a pattern that could match a byte
sequence that isn't UTF-8 and `utf8` was enabled.

###### `InvalidLineTerminator`

This error occurs when one uses a non-ASCII byte for a line terminator,
but where Unicode mode is enabled and UTF-8 mode is disabled.

###### `UnicodePropertyNotFound`

This occurs when an unrecognized Unicode property name could not
be found.

###### `UnicodePropertyValueNotFound`

This occurs when an unrecognized Unicode property value could not
be found.

###### `UnicodePerlClassNotFound`

This occurs when a Unicode-aware Perl character class (`\w`, `\s` or
`\d`) could not be found. This can occur when the `unicode-perl`
crate feature is not enabled.

###### `UnicodeCaseUnavailable`

This occurs when the Unicode simple case mapping tables are not
available, and the regular expression required Unicode aware case
insensitivity.

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
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

- **Eq**
- **Unpin**
- **Sync**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ErrorKind) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ErrorKind { /* ... */ }
    ```

#### Struct `Hir`

A high-level intermediate representation (HIR) for a regular expression.

An HIR value is a combination of a [`HirKind`] and a set of [`Properties`].
An `HirKind` indicates what kind of regular expression it is (a literal,
a repetition, a look-around assertion, etc.), where as a `Properties`
describes various facts about the regular expression. For example, whether
it matches UTF-8 or if it matches the empty string.

The HIR of a regular expression represents an intermediate step between
its abstract syntax (a structured description of the concrete syntax) and
an actual regex matcher. The purpose of HIR is to make regular expressions
easier to analyze. In particular, the AST is much more complex than the
HIR. For example, while an AST supports arbitrarily nested character
classes, the HIR will flatten all nested classes into a single set. The HIR
will also "compile away" every flag present in the concrete syntax. For
example, users of HIR expressions never need to worry about case folding;
it is handled automatically by the translator (e.g., by translating
`(?i:A)` to `[aA]`).

The specific type of an HIR expression can be accessed via its `kind`
or `into_kind` methods. This extra level of indirection exists for two
reasons:

1. Construction of an HIR expression *must* use the constructor methods on
this `Hir` type instead of building the `HirKind` values directly. This
permits construction to enforce invariants like "concatenations always
consist of two or more sub-expressions."
2. Every HIR expression contains attributes that are defined inductively,
and can be computed cheaply during the construction process. For example,
one such attribute is whether the expression must match at the beginning of
the haystack.

In particular, if you have an `HirKind` value, then there is intentionally
no way to build an `Hir` value from it. You instead need to do case
analysis on the `HirKind` value and build the `Hir` value using its smart
constructors.

# UTF-8

If the HIR was produced by a translator with
[`TranslatorBuilder::utf8`](translate::TranslatorBuilder::utf8) enabled,
then the HIR is guaranteed to match UTF-8 exclusively for all non-empty
matches.

For empty matches, those can occur at any position. It is the
responsibility of the regex engine to determine whether empty matches are
permitted between the code units of a single codepoint.

# Stack space

This type defines its own destructor that uses constant stack space and
heap space proportional to the size of the HIR.

Also, an `Hir`'s `fmt::Display` implementation prints an HIR as a regular
expression pattern string, and uses constant stack space and heap space
proportional to the size of the `Hir`. The regex it prints is guaranteed to
be _semantically_ equivalent to the original concrete syntax, but it may
look very different. (And potentially not practically readable by a human.)

An `Hir`'s `fmt::Debug` implementation currently does not use constant
stack space. The implementation will also suppress some details (such as
the `Properties` inlined into every `Hir` value to make it less noisy).

```rust
pub struct Hir {
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
  pub fn kind(self: &Self) -> &HirKind { /* ... */ }
  ```
  Returns a reference to the underlying HIR kind.

- ```rust
  pub fn into_kind(self: Self) -> HirKind { /* ... */ }
  ```
  Consumes ownership of this HIR expression and returns its underlying

- ```rust
  pub fn properties(self: &Self) -> &Properties { /* ... */ }
  ```
  Returns the properties computed for this `Hir`.

- ```rust
  pub fn empty() -> Hir { /* ... */ }
  ```
  Returns an empty HIR expression.

- ```rust
  pub fn fail() -> Hir { /* ... */ }
  ```
  Returns an HIR expression that can never match anything. That is,

- ```rust
  pub fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir { /* ... */ }
  ```
  Creates a literal HIR expression.

- ```rust
  pub fn class(class: Class) -> Hir { /* ... */ }
  ```
  Creates a class HIR expression. The class may either be defined over

- ```rust
  pub fn look(look: Look) -> Hir { /* ... */ }
  ```
  Creates a look-around assertion HIR expression.

- ```rust
  pub fn repetition(rep: Repetition) -> Hir { /* ... */ }
  ```
  Creates a repetition HIR expression.

- ```rust
  pub fn capture(capture: Capture) -> Hir { /* ... */ }
  ```
  Creates a capture HIR expression.

- ```rust
  pub fn concat(subs: Vec<Hir>) -> Hir { /* ... */ }
  ```
  Returns the concatenation of the given expressions.

- ```rust
  pub fn alternation(subs: Vec<Hir>) -> Hir { /* ... */ }
  ```
  Returns the alternation of the given expressions.

- ```rust
  pub fn dot(dot: Dot) -> Hir { /* ... */ }
  ```
  Returns an HIR expression for `.`.

###### Trait Implementations

- **Sync**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Unpin**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Hir { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Hir) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Enum `HirKind`

The underlying kind of an arbitrary [`Hir`] expression.

An `HirKind` is principally useful for doing case analysis on the type
of a regular expression. If you're looking to build new `Hir` values,
then you _must_ use the smart constructors defined on `Hir`, like
[`Hir::repetition`], to build new `Hir` values. The API intentionally does
not expose any way of building an `Hir` directly from an `HirKind`.

```rust
pub enum HirKind {
    Empty,
    Literal(Literal),
    Class(Class),
    Look(Look),
    Repetition(Repetition),
    Capture(Capture),
    Concat(alloc::vec::Vec<Hir>),
    Alternation(alloc::vec::Vec<Hir>),
}
```

##### Variants

###### `Empty`

The empty regular expression, which matches everything, including the
empty string.

###### `Literal`

A literalstring that matches exactly these bytes.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Literal` |  |

###### `Class`

A single character class that matches any of the characters in the
class. A class can either consist of Unicode scalar values as
characters, or it can use bytes.

A class may be empty. In which case, it matches nothing.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Class` |  |

###### `Look`

A look-around assertion. A look-around match always has zero length.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Look` |  |

###### `Repetition`

A repetition operation applied to a sub-expression.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Repetition` |  |

###### `Capture`

A capturing group, which contains a sub-expression.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Capture` |  |

###### `Concat`

A concatenation of expressions.

A concatenation matches only if each of its sub-expressions match one
after the other.

Concatenations are guaranteed by `Hir`'s smart constructors to always
have at least two sub-expressions.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::vec::Vec<Hir>` |  |

###### `Alternation`

An alternation of expressions.

An alternation matches only if at least one of its sub-expressions
match. If multiple sub-expressions match, then the leftmost is
preferred.

Alternations are guaranteed by `Hir`'s smart constructors to always
have at least two sub-expressions.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::vec::Vec<Hir>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn subs(self: &Self) -> &[Hir] { /* ... */ }
  ```
  Returns a slice of this kind's sub-expressions, if any.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
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
    fn clone(self: &Self) -> HirKind { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HirKind) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Literal`

The high-level intermediate representation of a literal.

A literal corresponds to `0` or more bytes that should be matched
literally. The smart constructors defined on `Hir` will automatically
concatenate adjacent literals into one literal, and will even automatically
replace empty literals with `Hir::empty()`.

Note that despite a literal being represented by a sequence of bytes, its
`Debug` implementation will attempt to print it as a normal string. (That
is, not a sequence of decimal numbers.)

```rust
pub struct Literal(pub alloc::boxed::Box<[u8]>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<[u8]>` |  |

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Literal) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **RefUnwindSafe**
- **Unpin**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Literal { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
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
- **Freeze**
#### Enum `Class`

The high-level intermediate representation of a character class.

A character class corresponds to a set of characters. A character is either
defined by a Unicode scalar value or a byte.

A character class, regardless of its character type, is represented by a
sequence of non-overlapping non-adjacent ranges of characters.

There are no guarantees about which class variant is used. Generally
speaking, the Unicode variat is used whenever a class needs to contain
non-ASCII Unicode scalar values. But the Unicode variant can be used even
when Unicode mode is disabled. For example, at the time of writing, the
regex `(?-u:a|\xc2\xa0)` will compile down to HIR for the Unicode class
`[a\u00A0]` due to optimizations.

Note that `Bytes` variant may be produced even when it exclusively matches
valid UTF-8. This is because a `Bytes` variant represents an intention by
the author of the regular expression to disable Unicode mode, which in turn
impacts the semantics of case insensitive matching. For example, `(?i)k`
and `(?i-u)k` will not match the same set of strings.

```rust
pub enum Class {
    Unicode(ClassUnicode),
    Bytes(ClassBytes),
}
```

##### Variants

###### `Unicode`

A set of characters represented by Unicode scalar values.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassUnicode` |  |

###### `Bytes`

A set of characters represented by arbitrary bytes (one byte per
character).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ClassBytes` |  |

##### Implementations

###### Methods

- ```rust
  pub fn case_fold_simple(self: &mut Self) { /* ... */ }
  ```
  Apply Unicode simple case folding to this character class, in place.

- ```rust
  pub fn try_case_fold_simple(self: &mut Self) -> core::result::Result<(), CaseFoldError> { /* ... */ }
  ```
  Apply Unicode simple case folding to this character class, in place.

- ```rust
  pub fn negate(self: &mut Self) { /* ... */ }
  ```
  Negate this character class in place.

- ```rust
  pub fn is_utf8(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this character class will only ever match

- ```rust
  pub fn minimum_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length, in bytes, of the smallest string matched by this

- ```rust
  pub fn maximum_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length, in bytes, of the longest string matched by this

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this character class is empty. That is,

- ```rust
  pub fn literal(self: &Self) -> Option<Vec<u8>> { /* ... */ }
  ```
  If this class consists of exactly one element (whether a codepoint or a

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Class) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Class { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ClassUnicode`

A set of characters represented by Unicode scalar values.

```rust
pub struct ClassUnicode {
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
  pub fn new<I>(ranges: I) -> ClassUnicode
where
    I: IntoIterator<Item = ClassUnicodeRange> { /* ... */ }
  ```
  Create a new class from a sequence of ranges.

- ```rust
  pub fn empty() -> ClassUnicode { /* ... */ }
  ```
  Create a new class with no ranges.

- ```rust
  pub fn push(self: &mut Self, range: ClassUnicodeRange) { /* ... */ }
  ```
  Add a new range to this set.

- ```rust
  pub fn iter(self: &Self) -> ClassUnicodeIter<''_> { /* ... */ }
  ```
  Return an iterator over all ranges in this class.

- ```rust
  pub fn ranges(self: &Self) -> &[ClassUnicodeRange] { /* ... */ }
  ```
  Return the underlying ranges as a slice.

- ```rust
  pub fn case_fold_simple(self: &mut Self) { /* ... */ }
  ```
  Expand this character class such that it contains all case folded

- ```rust
  pub fn try_case_fold_simple(self: &mut Self) -> core::result::Result<(), CaseFoldError> { /* ... */ }
  ```
  Expand this character class such that it contains all case folded

- ```rust
  pub fn negate(self: &mut Self) { /* ... */ }
  ```
  Negate this character class.

- ```rust
  pub fn union(self: &mut Self, other: &ClassUnicode) { /* ... */ }
  ```
  Union this character class with the given character class, in place.

- ```rust
  pub fn intersect(self: &mut Self, other: &ClassUnicode) { /* ... */ }
  ```
  Intersect this character class with the given character class, in

- ```rust
  pub fn difference(self: &mut Self, other: &ClassUnicode) { /* ... */ }
  ```
  Subtract the given character class from this character class, in place.

- ```rust
  pub fn symmetric_difference(self: &mut Self, other: &ClassUnicode) { /* ... */ }
  ```
  Compute the symmetric difference of the given character classes, in

- ```rust
  pub fn is_ascii(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this character class will either match

- ```rust
  pub fn minimum_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length, in bytes, of the smallest string matched by this

- ```rust
  pub fn maximum_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length, in bytes, of the longest string matched by this

- ```rust
  pub fn literal(self: &Self) -> Option<Vec<u8>> { /* ... */ }
  ```
  If this class consists of exactly one codepoint, then return it as

- ```rust
  pub fn to_byte_class(self: &Self) -> Option<ClassBytes> { /* ... */ }
  ```
  If this class consists of only ASCII ranges, then return its

###### Trait Implementations

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassUnicode) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassUnicode { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
#### Struct `ClassUnicodeIter`

An iterator over all ranges in a Unicode character class.

The lifetime `'a` refers to the lifetime of the underlying class.

```rust
pub struct ClassUnicodeIter<''a>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a ClassUnicodeRange> { /* ... */ }
    ```

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
#### Struct `ClassUnicodeRange`

A single range of characters represented by Unicode scalar values.

The range is closed. That is, the start and end of the range are included
in the range.

```rust
pub struct ClassUnicodeRange {
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
  pub fn new(start: char, end: char) -> ClassUnicodeRange { /* ... */ }
  ```
  Create a new Unicode scalar value range for a character class.

- ```rust
  pub fn start(self: &Self) -> char { /* ... */ }
  ```
  Return the start of this range.

- ```rust
  pub fn end(self: &Self) -> char { /* ... */ }
  ```
  Return the end of this range.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of codepoints in this range.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> ClassUnicodeRange { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ClassUnicodeRange) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassUnicodeRange { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Eq**
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
    fn eq(self: &Self, other: &ClassUnicodeRange) -> bool { /* ... */ }
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

- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ClassUnicodeRange) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ClassBytes`

A set of characters represented by arbitrary bytes.

Each byte corresponds to one character.

```rust
pub struct ClassBytes {
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
  pub fn new<I>(ranges: I) -> ClassBytes
where
    I: IntoIterator<Item = ClassBytesRange> { /* ... */ }
  ```
  Create a new class from a sequence of ranges.

- ```rust
  pub fn empty() -> ClassBytes { /* ... */ }
  ```
  Create a new class with no ranges.

- ```rust
  pub fn push(self: &mut Self, range: ClassBytesRange) { /* ... */ }
  ```
  Add a new range to this set.

- ```rust
  pub fn iter(self: &Self) -> ClassBytesIter<''_> { /* ... */ }
  ```
  Return an iterator over all ranges in this class.

- ```rust
  pub fn ranges(self: &Self) -> &[ClassBytesRange] { /* ... */ }
  ```
  Return the underlying ranges as a slice.

- ```rust
  pub fn case_fold_simple(self: &mut Self) { /* ... */ }
  ```
  Expand this character class such that it contains all case folded

- ```rust
  pub fn negate(self: &mut Self) { /* ... */ }
  ```
  Negate this byte class.

- ```rust
  pub fn union(self: &mut Self, other: &ClassBytes) { /* ... */ }
  ```
  Union this byte class with the given byte class, in place.

- ```rust
  pub fn intersect(self: &mut Self, other: &ClassBytes) { /* ... */ }
  ```
  Intersect this byte class with the given byte class, in place.

- ```rust
  pub fn difference(self: &mut Self, other: &ClassBytes) { /* ... */ }
  ```
  Subtract the given byte class from this byte class, in place.

- ```rust
  pub fn symmetric_difference(self: &mut Self, other: &ClassBytes) { /* ... */ }
  ```
  Compute the symmetric difference of the given byte classes, in place.

- ```rust
  pub fn is_ascii(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this character class will either match

- ```rust
  pub fn minimum_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length, in bytes, of the smallest string matched by this

- ```rust
  pub fn maximum_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length, in bytes, of the longest string matched by this

- ```rust
  pub fn literal(self: &Self) -> Option<Vec<u8>> { /* ... */ }
  ```
  If this class consists of exactly one byte, then return it as

- ```rust
  pub fn to_unicode_class(self: &Self) -> Option<ClassUnicode> { /* ... */ }
  ```
  If this class consists of only ASCII ranges, then return its

###### Trait Implementations

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassBytes { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassBytes) -> bool { /* ... */ }
    ```

#### Struct `ClassBytesIter`

An iterator over all ranges in a byte character class.

The lifetime `'a` refers to the lifetime of the underlying class.

```rust
pub struct ClassBytesIter<''a>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a ClassBytesRange> { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **Sync**
#### Struct `ClassBytesRange`

A single range of characters represented by arbitrary bytes.

The range is closed. That is, the start and end of the range are included
in the range.

```rust
pub struct ClassBytesRange {
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
  pub fn new(start: u8, end: u8) -> ClassBytesRange { /* ... */ }
  ```
  Create a new byte range for a character class.

- ```rust
  pub fn start(self: &Self) -> u8 { /* ... */ }
  ```
  Return the start of this range.

- ```rust
  pub fn end(self: &Self) -> u8 { /* ... */ }
  ```
  Return the end of this range.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of bytes in this range.

###### Trait Implementations

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassBytesRange) -> bool { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassBytesRange { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ClassBytesRange) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Copy**
- **Unpin**
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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ClassBytesRange) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Eq**
- **Default**
  - ```rust
    fn default() -> ClassBytesRange { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `Look`

The high-level intermediate representation for a look-around assertion.

An assertion match is always zero-length. Also called an "empty match."

```rust
pub enum Look {
    Start = { _ },
    End = { _ },
    StartLF = { _ },
    EndLF = { _ },
    StartCRLF = { _ },
    EndCRLF = { _ },
    WordAscii = { _ },
    WordAsciiNegate = { _ },
    WordUnicode = { _ },
    WordUnicodeNegate = { _ },
    WordStartAscii = { _ },
    WordEndAscii = { _ },
    WordStartUnicode = { _ },
    WordEndUnicode = { _ },
    WordStartHalfAscii = { _ },
    WordEndHalfAscii = { _ },
    WordStartHalfUnicode = { _ },
    WordEndHalfUnicode = { _ },
}
```

##### Variants

###### `Start`

Match the beginning of text. Specifically, this matches at the starting
position of the input.

Discriminant: `{ _ }`

Discriminant value: `1`

###### `End`

Match the end of text. Specifically, this matches at the ending
position of the input.

Discriminant: `{ _ }`

Discriminant value: `2`

###### `StartLF`

Match the beginning of a line or the beginning of text. Specifically,
this matches at the starting position of the input, or at the position
immediately following a `\n` character.

Discriminant: `{ _ }`

Discriminant value: `4`

###### `EndLF`

Match the end of a line or the end of text. Specifically, this matches
at the end position of the input, or at the position immediately
preceding a `\n` character.

Discriminant: `{ _ }`

Discriminant value: `8`

###### `StartCRLF`

Match the beginning of a line or the beginning of text. Specifically,
this matches at the starting position of the input, or at the position
immediately following either a `\r` or `\n` character, but never after
a `\r` when a `\n` follows.

Discriminant: `{ _ }`

Discriminant value: `16`

###### `EndCRLF`

Match the end of a line or the end of text. Specifically, this matches
at the end position of the input, or at the position immediately
preceding a `\r` or `\n` character, but never before a `\n` when a `\r`
precedes it.

Discriminant: `{ _ }`

Discriminant value: `32`

###### `WordAscii`

Match an ASCII-only word boundary. That is, this matches a position
where the left adjacent character and right adjacent character
correspond to a word and non-word or a non-word and word character.

Discriminant: `{ _ }`

Discriminant value: `64`

###### `WordAsciiNegate`

Match an ASCII-only negation of a word boundary.

Discriminant: `{ _ }`

Discriminant value: `128`

###### `WordUnicode`

Match a Unicode-aware word boundary. That is, this matches a position
where the left adjacent character and right adjacent character
correspond to a word and non-word or a non-word and word character.

Discriminant: `{ _ }`

Discriminant value: `256`

###### `WordUnicodeNegate`

Match a Unicode-aware negation of a word boundary.

Discriminant: `{ _ }`

Discriminant value: `512`

###### `WordStartAscii`

Match the start of an ASCII-only word boundary. That is, this matches a
position at either the beginning of the haystack or where the previous
character is not a word character and the following character is a word
character.

Discriminant: `{ _ }`

Discriminant value: `1024`

###### `WordEndAscii`

Match the end of an ASCII-only word boundary. That is, this matches
a position at either the end of the haystack or where the previous
character is a word character and the following character is not a word
character.

Discriminant: `{ _ }`

Discriminant value: `2048`

###### `WordStartUnicode`

Match the start of a Unicode word boundary. That is, this matches a
position at either the beginning of the haystack or where the previous
character is not a word character and the following character is a word
character.

Discriminant: `{ _ }`

Discriminant value: `4096`

###### `WordEndUnicode`

Match the end of a Unicode word boundary. That is, this matches a
position at either the end of the haystack or where the previous
character is a word character and the following character is not a word
character.

Discriminant: `{ _ }`

Discriminant value: `8192`

###### `WordStartHalfAscii`

Match the start half of an ASCII-only word boundary. That is, this
matches a position at either the beginning of the haystack or where the
previous character is not a word character.

Discriminant: `{ _ }`

Discriminant value: `16384`

###### `WordEndHalfAscii`

Match the end half of an ASCII-only word boundary. That is, this
matches a position at either the end of the haystack or where the
following character is not a word character.

Discriminant: `{ _ }`

Discriminant value: `32768`

###### `WordStartHalfUnicode`

Match the start half of a Unicode word boundary. That is, this matches
a position at either the beginning of the haystack or where the
previous character is not a word character.

Discriminant: `{ _ }`

Discriminant value: `65536`

###### `WordEndHalfUnicode`

Match the end half of a Unicode word boundary. That is, this matches
a position at either the end of the haystack or where the following
character is not a word character.

Discriminant: `{ _ }`

Discriminant value: `131072`

##### Implementations

###### Methods

- ```rust
  pub const fn reversed(self: Self) -> Look { /* ... */ }
  ```
  Flip the look-around assertion to its equivalent for reverse searches.

- ```rust
  pub const fn as_repr(self: Self) -> u32 { /* ... */ }
  ```
  Return the underlying representation of this look-around enumeration

- ```rust
  pub const fn from_repr(repr: u32) -> Option<Look> { /* ... */ }
  ```
  Given the underlying representation of a `Look` value, return the

- ```rust
  pub const fn as_char(self: Self) -> char { /* ... */ }
  ```
  Returns a convenient single codepoint representation of this

###### Trait Implementations

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **UnwindSafe**
- **Send**
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
    fn clone(self: &Self) -> Look { /* ... */ }
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
- **Copy**
- **Freeze**
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Look) -> bool { /* ... */ }
    ```

#### Struct `Capture`

The high-level intermediate representation for a capturing group.

A capturing group always has an index and a child expression. It may
also have a name associated with it (e.g., `(?P<foo>\w)`), but it's not
necessary.

Note that there is no explicit representation of a non-capturing group
in a `Hir`. Instead, non-capturing grouping is handled automatically by
the recursive structure of the `Hir` itself.

```rust
pub struct Capture {
    pub index: u32,
    pub name: Option<alloc::boxed::Box<str>>,
    pub sub: alloc::boxed::Box<Hir>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `index` | `u32` | The capture index of the capture. |
| `name` | `Option<alloc::boxed::Box<str>>` | The name of the capture, if it exists. |
| `sub` | `alloc::boxed::Box<Hir>` | The expression inside the capturing group, which may be empty. |

##### Implementations

###### Trait Implementations

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Eq**
- **Send**
- **Sync**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Capture) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Capture { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Repetition`

The high-level intermediate representation of a repetition operator.

A repetition operator permits the repetition of an arbitrary
sub-expression.

```rust
pub struct Repetition {
    pub min: u32,
    pub max: Option<u32>,
    pub greedy: bool,
    pub sub: alloc::boxed::Box<Hir>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `min` | `u32` | The minimum range of the repetition.<br><br>Note that special cases like `?`, `+` and `*` all get translated into<br>the ranges `{0,1}`, `{1,}` and `{0,}`, respectively.<br><br>When `min` is zero, this expression can match the empty string<br>regardless of what its sub-expression is. |
| `max` | `Option<u32>` | The maximum range of the repetition.<br><br>Note that when `max` is `None`, `min` acts as a lower bound but where<br>there is no upper bound. For something like `x{5}` where the min and<br>max are equivalent, `min` will be set to `5` and `max` will be set to<br>`Some(5)`. |
| `greedy` | `bool` | Whether this repetition operator is greedy or not. A greedy operator<br>will match as much as it can. A non-greedy operator will match as<br>little as it can.<br><br>Typically, operators are greedy by default and are only non-greedy when<br>a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is<br>not. However, this can be inverted via the `U` "ungreedy" flag. |
| `sub` | `alloc::boxed::Box<Hir>` | The expression being repeated. |

##### Implementations

###### Methods

- ```rust
  pub fn with(self: &Self, sub: Hir) -> Repetition { /* ... */ }
  ```
  Returns a new repetition with the same `min`, `max` and `greedy`

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Repetition { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **RefUnwindSafe**
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

- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Repetition) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `Dot`

**Attributes:**

- `#[non_exhaustive]`

A type describing the different flavors of `.`.

This type is meant to be used with [`Hir::dot`], which is a convenience
routine for building HIR values derived from the `.` regex.

```rust
pub enum Dot {
    AnyChar,
    AnyByte,
    AnyCharExcept(char),
    AnyCharExceptLF,
    AnyCharExceptCRLF,
    AnyByteExcept(u8),
    AnyByteExceptLF,
    AnyByteExceptCRLF,
}
```

##### Variants

###### `AnyChar`

Matches the UTF-8 encoding of any Unicode scalar value.

This is equivalent to `(?su:.)` and also `\p{any}`.

###### `AnyByte`

Matches any byte value.

This is equivalent to `(?s-u:.)` and also `(?-u:[\x00-\xFF])`.

###### `AnyCharExcept`

Matches the UTF-8 encoding of any Unicode scalar value except for the
`char` given.

This is equivalent to using `(?u-s:.)` with the line terminator set
to a particular ASCII byte. (Because of peculiarities in the regex
engines, a line terminator must be a single byte. It follows that when
UTF-8 mode is enabled, this single byte must also be a Unicode scalar
value. That is, ti must be ASCII.)

(This and `AnyCharExceptLF` both exist because of legacy reasons.
`AnyCharExceptLF` will be dropped in the next breaking change release.)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

###### `AnyCharExceptLF`

Matches the UTF-8 encoding of any Unicode scalar value except for `\n`.

This is equivalent to `(?u-s:.)` and also `[\p{any}--\n]`.

###### `AnyCharExceptCRLF`

Matches the UTF-8 encoding of any Unicode scalar value except for `\r`
and `\n`.

This is equivalent to `(?uR-s:.)` and also `[\p{any}--\r\n]`.

###### `AnyByteExcept`

Matches any byte value except for the `u8` given.

This is equivalent to using `(?-us:.)` with the line terminator set
to a particular ASCII byte. (Because of peculiarities in the regex
engines, a line terminator must be a single byte. It follows that when
UTF-8 mode is enabled, this single byte must also be a Unicode scalar
value. That is, ti must be ASCII.)

(This and `AnyByteExceptLF` both exist because of legacy reasons.
`AnyByteExceptLF` will be dropped in the next breaking change release.)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

###### `AnyByteExceptLF`

Matches any byte value except for `\n`.

This is equivalent to `(?-su:.)` and also `(?-u:[[\x00-\xFF]--\n])`.

###### `AnyByteExceptCRLF`

Matches any byte value except for `\r` and `\n`.

This is equivalent to `(?R-su:.)` and also `(?-u:[[\x00-\xFF]--\r\n])`.

##### Implementations

###### Trait Implementations

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Dot) -> bool { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Dot { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `Properties`

A type that collects various properties of an HIR value.

Properties are always scalar values and represent meta data that is
computed inductively on an HIR value. Properties are defined for all
HIR values.

All methods on a `Properties` value take constant time and are meant to
be cheap to call.

```rust
pub struct Properties(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn minimum_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length (in bytes) of the smallest string matched by this

- ```rust
  pub fn maximum_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length (in bytes) of the longest string matched by this

- ```rust
  pub fn look_set(self: &Self) -> LookSet { /* ... */ }
  ```
  Returns a set of all look-around assertions that appear at least once

- ```rust
  pub fn look_set_prefix(self: &Self) -> LookSet { /* ... */ }
  ```
  Returns a set of all look-around assertions that appear as a prefix for

- ```rust
  pub fn look_set_prefix_any(self: &Self) -> LookSet { /* ... */ }
  ```
  Returns a set of all look-around assertions that appear as a _possible_

- ```rust
  pub fn look_set_suffix(self: &Self) -> LookSet { /* ... */ }
  ```
  Returns a set of all look-around assertions that appear as a suffix for

- ```rust
  pub fn look_set_suffix_any(self: &Self) -> LookSet { /* ... */ }
  ```
  Returns a set of all look-around assertions that appear as a _possible_

- ```rust
  pub fn is_utf8(self: &Self) -> bool { /* ... */ }
  ```
  Return true if and only if the corresponding HIR will always match

- ```rust
  pub fn explicit_captures_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of explicit capturing groups in the

- ```rust
  pub fn static_explicit_captures_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the total number of explicit capturing groups that appear in

- ```rust
  pub fn is_literal(self: &Self) -> bool { /* ... */ }
  ```
  Return true if and only if this HIR is a simple literal. This is

- ```rust
  pub fn is_alternation_literal(self: &Self) -> bool { /* ... */ }
  ```
  Return true if and only if this HIR is either a simple literal or an

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total amount of heap memory usage, in bytes, used by this

- ```rust
  pub fn union<I, P>(props: I) -> Properties
where
    I: IntoIterator<Item = P>,
    P: core::borrow::Borrow<Properties> { /* ... */ }
  ```
  Returns a new set of properties that corresponds to the union of the

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Properties) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Properties { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
#### Struct `LookSet`

A set of look-around assertions.

This is useful for efficiently tracking look-around assertions. For
example, an [`Hir`] provides properties that return `LookSet`s.

```rust
pub struct LookSet {
    pub bits: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `bits` | `u32` | The underlying representation this set is exposed to make it possible<br>to store it somewhere efficiently. The representation is that<br>of a bitset, where each assertion occupies bit `i` where `i =<br>Look::as_repr()`.<br><br>Note that users of this internal representation must permit the full<br>range of `u16` values to be represented. For example, even if the<br>current implementation only makes use of the 10 least significant bits,<br>it may use more bits in a future semver compatible release. |

##### Implementations

###### Methods

- ```rust
  pub fn empty() -> LookSet { /* ... */ }
  ```
  Create an empty set of look-around assertions.

- ```rust
  pub fn full() -> LookSet { /* ... */ }
  ```
  Create a full set of look-around assertions.

- ```rust
  pub fn singleton(look: Look) -> LookSet { /* ... */ }
  ```
  Create a look-around set containing the look-around assertion given.

- ```rust
  pub fn len(self: Self) -> usize { /* ... */ }
  ```
  Returns the total number of look-around assertions in this set.

- ```rust
  pub fn is_empty(self: Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set is empty.

- ```rust
  pub fn contains(self: Self, look: Look) -> bool { /* ... */ }
  ```
  Returns true if and only if the given look-around assertion is in this

- ```rust
  pub fn contains_anchor(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any anchor assertions.

- ```rust
  pub fn contains_anchor_haystack(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any "start/end of

- ```rust
  pub fn contains_anchor_line(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any "start/end of line"

- ```rust
  pub fn contains_anchor_lf(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any "start/end of line"

- ```rust
  pub fn contains_anchor_crlf(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any "start/end of line"

- ```rust
  pub fn contains_word(self: Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any word boundary or

- ```rust
  pub fn contains_word_unicode(self: Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any Unicode word boundary

- ```rust
  pub fn contains_word_ascii(self: Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any ASCII word boundary

- ```rust
  pub fn iter(self: Self) -> LookSetIter { /* ... */ }
  ```
  Returns an iterator over all of the look-around assertions in this set.

- ```rust
  pub fn insert(self: Self, look: Look) -> LookSet { /* ... */ }
  ```
  Return a new set that is equivalent to the original, but with the given

- ```rust
  pub fn set_insert(self: &mut Self, look: Look) { /* ... */ }
  ```
  Updates this set in place with the result of inserting the given

- ```rust
  pub fn remove(self: Self, look: Look) -> LookSet { /* ... */ }
  ```
  Return a new set that is equivalent to the original, but with the given

- ```rust
  pub fn set_remove(self: &mut Self, look: Look) { /* ... */ }
  ```
  Updates this set in place with the result of removing the given

- ```rust
  pub fn subtract(self: Self, other: LookSet) -> LookSet { /* ... */ }
  ```
  Returns a new set that is the result of subtracting the given set from

- ```rust
  pub fn set_subtract(self: &mut Self, other: LookSet) { /* ... */ }
  ```
  Updates this set in place with the result of subtracting the given set

- ```rust
  pub fn union(self: Self, other: LookSet) -> LookSet { /* ... */ }
  ```
  Returns a new set that is the union of this and the one given.

- ```rust
  pub fn set_union(self: &mut Self, other: LookSet) { /* ... */ }
  ```
  Updates this set in place with the result of unioning it with the one

- ```rust
  pub fn intersect(self: Self, other: LookSet) -> LookSet { /* ... */ }
  ```
  Returns a new set that is the intersection of this and the one given.

- ```rust
  pub fn set_intersect(self: &mut Self, other: LookSet) { /* ... */ }
  ```
  Updates this set in place with the result of intersecting it with the

- ```rust
  pub fn read_repr(slice: &[u8]) -> LookSet { /* ... */ }
  ```
  Return a `LookSet` from the slice given as a native endian 32-bit

- ```rust
  pub fn write_repr(self: Self, slice: &mut [u8]) { /* ... */ }
  ```
  Write a `LookSet` as a native endian 32-bit integer to the beginning

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LookSet { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LookSet) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **Sync**
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

- **Eq**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Default**
  - ```rust
    fn default() -> LookSet { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

#### Struct `LookSetIter`

An iterator over all look-around assertions in a [`LookSet`].

This iterator is created by [`LookSet::iter`].

```rust
pub struct LookSetIter {
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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Look> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> LookSetIter { /* ... */ }
    ```

### Re-exports

#### Re-export `visit`

```rust
pub use crate::hir::visitor::visit;
```

#### Re-export `Visitor`

```rust
pub use crate::hir::visitor::Visitor;
```

#### Re-export `CaseFoldError`

```rust
pub use crate::unicode::CaseFoldError;
```

## Module `utf8`

Converts ranges of Unicode scalar values to equivalent ranges of UTF-8 bytes.

This is sub-module is useful for constructing byte based automatons that need
to embed UTF-8 decoding. The most common use of this module is in conjunction
with the [`hir::ClassUnicodeRange`](crate::hir::ClassUnicodeRange) type.

See the documentation on the `Utf8Sequences` iterator for more details and
an example.

# Wait, what is this?

This is simplest to explain with an example. Let's say you wanted to test
whether a particular byte sequence was a Cyrillic character. One possible
scalar value range is `[0400-04FF]`. The set of allowed bytes for this
range can be expressed as a sequence of byte ranges:

```text
[D0-D3][80-BF]
```

This is simple enough: simply encode the boundaries, `0400` encodes to
`D0 80` and `04FF` encodes to `D3 BF`, and create ranges from each
corresponding pair of bytes: `D0` to `D3` and `80` to `BF`.

However, what if you wanted to add the Cyrillic Supplementary characters to
your range? Your range might then become `[0400-052F]`. The same procedure
as above doesn't quite work because `052F` encodes to `D4 AF`. The byte ranges
you'd get from the previous transformation would be `[D0-D4][80-AF]`. However,
this isn't quite correct because this range doesn't capture many characters,
for example, `04FF` (because its last byte, `BF` isn't in the range `80-AF`).

Instead, you need multiple sequences of byte ranges:

```text
[D0-D3][80-BF]  # matches codepoints 0400-04FF
[D4][80-AF]     # matches codepoints 0500-052F
```

This gets even more complicated if you want bigger ranges, particularly if
they naively contain surrogate codepoints. For example, the sequence of byte
ranges for the basic multilingual plane (`[0000-FFFF]`) look like this:

```text
[0-7F]
[C2-DF][80-BF]
[E0][A0-BF][80-BF]
[E1-EC][80-BF][80-BF]
[ED][80-9F][80-BF]
[EE-EF][80-BF][80-BF]
```

Note that the byte ranges above will *not* match any erroneous encoding of
UTF-8, including encodings of surrogate codepoints.

And, of course, for all of Unicode (`[000000-10FFFF]`):

```text
[0-7F]
[C2-DF][80-BF]
[E0][A0-BF][80-BF]
[E1-EC][80-BF][80-BF]
[ED][80-9F][80-BF]
[EE-EF][80-BF][80-BF]
[F0][90-BF][80-BF][80-BF]
[F1-F3][80-BF][80-BF][80-BF]
[F4][80-8F][80-BF][80-BF]
```

This module automates the process of creating these byte ranges from ranges of
Unicode scalar values.

# Lineage

I got the idea and general implementation strategy from Russ Cox in his
[article on regexps](https://web.archive.org/web/20160404141123/https://swtch.com/~rsc/regexp/regexp3.html) and RE2.
Russ Cox got it from Ken Thompson's `grep` (no source, folk lore?).
I also got the idea from
[Lucene](https://github.com/apache/lucene-solr/blob/ae93f4e7ac6a3908046391de35d4f50a0d3c59ca/lucene/core/src/java/org/apache/lucene/util/automaton/UTF32ToUTF8.java),
which uses it for executing automata on their term index.

```rust
pub mod utf8 { /* ... */ }
```

### Types

#### Enum `Utf8Sequence`

Utf8Sequence represents a sequence of byte ranges.

To match a Utf8Sequence, a candidate byte sequence must match each
successive range.

For example, if there are two ranges, `[C2-DF][80-BF]`, then the byte
sequence `\xDD\x61` would not match because `0x61 < 0x80`.

```rust
pub enum Utf8Sequence {
    One(Utf8Range),
    Two([Utf8Range; 2]),
    Three([Utf8Range; 3]),
    Four([Utf8Range; 4]),
}
```

##### Variants

###### `One`

One byte range.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Utf8Range` |  |

###### `Two`

Two successive byte ranges.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[Utf8Range; 2]` |  |

###### `Three`

Three successive byte ranges.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[Utf8Range; 3]` |  |

###### `Four`

Four successive byte ranges.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[Utf8Range; 4]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn as_slice(self: &Self) -> &[Utf8Range] { /* ... */ }
  ```
  Returns the underlying sequence of byte ranges as a slice.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of byte ranges in this sequence.

- ```rust
  pub fn reverse(self: &mut Self) { /* ... */ }
  ```
  Reverses the ranges in this sequence.

- ```rust
  pub fn matches(self: &Self, bytes: &[u8]) -> bool { /* ... */ }
  ```
  Returns true if and only if a prefix of `bytes` matches this sequence

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8Sequence { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Sequence) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Utf8Sequence) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Utf8Sequence) -> bool { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Utf8Range`

A single inclusive range of UTF-8 bytes.

```rust
pub struct Utf8Range {
    pub start: u8,
    pub end: u8,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `u8` | Start of byte range (inclusive). |
| `end` | `u8` | End of byte range (inclusive). |

##### Implementations

###### Methods

- ```rust
  pub fn matches(self: &Self, b: u8) -> bool { /* ... */ }
  ```
  Returns true if and only if the given byte is in this range.

###### Trait Implementations

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
    fn eq(self: &Self, other: &Utf8Range) -> bool { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8Range { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Range) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Utf8Range) -> $crate::cmp::Ordering { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
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
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Sync**
#### Struct `Utf8Sequences`

An iterator over ranges of matching UTF-8 byte sequences.

The iteration represents an alternation of comprehensive byte sequences
that match precisely the set of UTF-8 encoded scalar values.

A byte sequence corresponds to one of the scalar values in the range given
if and only if it completely matches exactly one of the sequences of byte
ranges produced by this iterator.

Each sequence of byte ranges matches a unique set of bytes. That is, no two
sequences will match the same bytes.

# Example

This shows how to match an arbitrary byte sequence against a range of
scalar values.

```rust
use regex_syntax::utf8::{Utf8Sequences, Utf8Sequence};

fn matches(seqs: &[Utf8Sequence], bytes: &[u8]) -> bool {
    for range in seqs {
        if range.matches(bytes) {
            return true;
        }
    }
    false
}

// Test the basic multilingual plane.
let seqs: Vec<_> = Utf8Sequences::new('\u{0}', '\u{FFFF}').collect();

// UTF-8 encoding of 'a'.
assert!(matches(&seqs, &[0x61]));
// UTF-8 encoding of '☃' (`\u{2603}`).
assert!(matches(&seqs, &[0xE2, 0x98, 0x83]));
// UTF-8 encoding of `\u{10348}` (outside the BMP).
assert!(!matches(&seqs, &[0xF0, 0x90, 0x8D, 0x88]));
// Tries to match against a UTF-8 encoding of a surrogate codepoint,
// which is invalid UTF-8, and therefore fails, despite the fact that
// the corresponding codepoint (0xD800) falls in the range given.
assert!(!matches(&seqs, &[0xED, 0xA0, 0x80]));
// And fails against plain old invalid UTF-8.
assert!(!matches(&seqs, &[0xFF, 0xFF]));
```

If this example seems circuitous, that's because it is! It's meant to be
illustrative. In practice, you could just try to decode your byte sequence
and compare it with the scalar value range directly. However, this is not
always possible (for example, in a byte based automaton).

```rust
pub struct Utf8Sequences {
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
  pub fn new(start: char, end: char) -> Self { /* ... */ }
  ```
  Create a new iterator over UTF-8 byte ranges for the scalar value range

###### Trait Implementations

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FusedIterator**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

## Functions

### Function `escape`

Escapes all regular expression meta characters in `text`.

The string returned may be safely used as a literal in a regular
expression.

```rust
pub fn escape(text: &str) -> alloc::string::String { /* ... */ }
```

### Function `escape_into`

Escapes all meta characters in `text` and writes the result into `buf`.

This will append escape characters into the given buffer. The characters
that are appended are safe to use as a literal in a regular expression.

```rust
pub fn escape_into(text: &str, buf: &mut alloc::string::String) { /* ... */ }
```

### Function `is_meta_character`

Returns true if the given character has significance in a regex.

Generally speaking, these are the only characters which _must_ be escaped
in order to match their literal meaning. For example, to match a literal
`|`, one could write `\|`. Sometimes escaping isn't always necessary. For
example, `-` is treated as a meta character because of its significance
for writing ranges inside of character classes, but the regex `-` will
match a literal `-` because `-` has no special meaning outside of character
classes.

In order to determine whether a character may be escaped at all, the
[`is_escapeable_character`] routine should be used. The difference between
`is_meta_character` and `is_escapeable_character` is that the latter will
return true for some characters that are _not_ meta characters. For
example, `%` and `\%` both match a literal `%` in all contexts. In other
words, `is_escapeable_character` includes "superfluous" escapes.

Note that the set of characters for which this function returns `true` or
`false` is fixed and won't change in a semver compatible release. (In this
case, "semver compatible release" actually refers to the `regex` crate
itself, since reducing or expanding the set of meta characters would be a
breaking change for not just `regex-syntax` but also `regex` itself.)

# Example

```
use regex_syntax::is_meta_character;

assert!(is_meta_character('?'));
assert!(is_meta_character('-'));
assert!(is_meta_character('&'));
assert!(is_meta_character('#'));

assert!(!is_meta_character('%'));
assert!(!is_meta_character('/'));
assert!(!is_meta_character('!'));
assert!(!is_meta_character('"'));
assert!(!is_meta_character('e'));
```

```rust
pub fn is_meta_character(c: char) -> bool { /* ... */ }
```

### Function `is_escapeable_character`

Returns true if the given character can be escaped in a regex.

This returns true in all cases that `is_meta_character` returns true, but
also returns true in some cases where `is_meta_character` returns false.
For example, `%` is not a meta character, but it is escapeable. That is,
`%` and `\%` both match a literal `%` in all contexts.

The purpose of this routine is to provide knowledge about what characters
may be escaped. Namely, most regex engines permit "superfluous" escapes
where characters without any special significance may be escaped even
though there is no actual _need_ to do so.

This will return false for some characters. For example, `e` is not
escapeable. Therefore, `\e` will either result in a parse error (which is
true today), or it could backwards compatibly evolve into a new construct
with its own meaning. Indeed, that is the purpose of banning _some_
superfluous escapes: it provides a way to evolve the syntax in a compatible
manner.

# Example

```
use regex_syntax::is_escapeable_character;

assert!(is_escapeable_character('?'));
assert!(is_escapeable_character('-'));
assert!(is_escapeable_character('&'));
assert!(is_escapeable_character('#'));
assert!(is_escapeable_character('%'));
assert!(is_escapeable_character('/'));
assert!(is_escapeable_character('!'));
assert!(is_escapeable_character('"'));

assert!(!is_escapeable_character('e'));
```

```rust
pub fn is_escapeable_character(c: char) -> bool { /* ... */ }
```

### Function `is_word_character`

Returns true if and only if the given character is a Unicode word
character.

A Unicode word character is defined by
[UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
In particular, a character
is considered a word character if it is in either of the `Alphabetic` or
`Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
or `Connector_Punctuation` general categories.

# Panics

If the `unicode-perl` feature is not enabled, then this function
panics. For this reason, it is recommended that callers use
[`try_is_word_character`] instead.

```rust
pub fn is_word_character(c: char) -> bool { /* ... */ }
```

### Function `try_is_word_character`

Returns true if and only if the given character is a Unicode word
character.

A Unicode word character is defined by
[UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
In particular, a character
is considered a word character if it is in either of the `Alphabetic` or
`Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
or `Connector_Punctuation` general categories.

# Errors

If the `unicode-perl` feature is not enabled, then this function always
returns an error.

```rust
pub fn try_is_word_character(c: char) -> core::result::Result<bool, UnicodeWordError> { /* ... */ }
```

### Function `is_word_byte`

Returns true if and only if the given character is an ASCII word character.

An ASCII word character is defined by the following character class:
`[_0-9a-zA-Z]`.

```rust
pub fn is_word_byte(c: u8) -> bool { /* ... */ }
```

## Re-exports

### Re-export `Error`

```rust
pub use crate::error::Error;
```

### Re-export `parse`

```rust
pub use crate::parser::parse;
```

### Re-export `Parser`

```rust
pub use crate::parser::Parser;
```

### Re-export `ParserBuilder`

```rust
pub use crate::parser::ParserBuilder;
```

### Re-export `UnicodeWordError`

```rust
pub use crate::unicode::UnicodeWordError;
```

