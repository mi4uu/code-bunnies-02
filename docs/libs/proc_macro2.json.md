# Crate Documentation

**Version:** 1.0.94

**Format Version:** 43

# Module `proc_macro2`

[![github]](https://github.com/dtolnay/proc-macro2)&ensp;[![crates-io]](https://crates.io/crates/proc-macro2)&ensp;[![docs-rs]](crate)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A wrapper around the procedural macro API of the compiler's [`proc_macro`]
crate. This library serves two purposes:

- **Bring proc-macro-like functionality to other contexts like build.rs and
  main.rs.** Types from `proc_macro` are entirely specific to procedural
  macros and cannot ever exist in code outside of a procedural macro.
  Meanwhile `proc_macro2` types may exist anywhere including non-macro code.
  By developing foundational libraries like [syn] and [quote] against
  `proc_macro2` rather than `proc_macro`, the procedural macro ecosystem
  becomes easily applicable to many other use cases and we avoid
  reimplementing non-macro equivalents of those libraries.

- **Make procedural macros unit testable.** As a consequence of being
  specific to procedural macros, nothing that uses `proc_macro` can be
  executed from a unit test. In order for helper libraries or components of
  a macro to be testable in isolation, they must be implemented using
  `proc_macro2`.

[syn]: https://github.com/dtolnay/syn
[quote]: https://github.com/dtolnay/quote

# Usage

The skeleton of a typical procedural macro typically looks like this:

```
extern crate proc_macro;

# const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
# };
# #[cfg(wrap_proc_macro)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        /* transform input */
        # input
    };

    proc_macro::TokenStream::from(output)
}
```

If parsing with [Syn], you'll use [`parse_macro_input!`] instead to
propagate parse errors correctly back to the compiler when parsing fails.

[`parse_macro_input!`]: https://docs.rs/syn/2.0/syn/macro.parse_macro_input.html

# Unstable features

The default feature set of proc-macro2 tracks the most recent stable
compiler API. Functionality in `proc_macro` that is not yet stable is not
exposed by proc-macro2 by default.

To opt into the additional APIs available in the most recent nightly
compiler, the `procmacro2_semver_exempt` config flag must be passed to
rustc. We will polyfill those nightly-only APIs back to Rust 1.56.0. As
these are unstable APIs that track the nightly compiler, minor versions of
proc-macro2 may make breaking changes to them at any time.

```sh
RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build
```

Note that this must not only be done for your crate, but for any crate that
depends on your crate. This infectious nature is intentional, as it serves
as a reminder that you are outside of the normal semver guarantees.

Semver exempt methods are marked as such in the proc-macro2 documentation.

# Thread-Safety

Most types in this crate are `!Sync` because the underlying compiler
types make use of thread-local memory, meaning they cannot be accessed from
a different thread.

## Modules

## Module `extra`

Items which do not have a correspondence to any API in the proc_macro crate,
but are necessary to include in proc-macro2.

```rust
pub mod extra { /* ... */ }
```

### Types

#### Struct `DelimSpan`

An object that holds a [`Group`]'s `span_open()` and `span_close()` together
in a more compact representation than holding those 2 spans individually.

[`Group`]: crate::Group

```rust
pub struct DelimSpan {
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
  pub fn join(self: &Self) -> Span { /* ... */ }
  ```
  Returns a span covering the entire delimited group.

- ```rust
  pub fn open(self: &Self) -> Span { /* ... */ }
  ```
  Returns a span for the opening punctuation of the group only.

- ```rust
  pub fn close(self: &Self) -> Span { /* ... */ }
  ```
  Returns a span for the closing punctuation of the group only.

###### Trait Implementations

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DelimSpan { /* ... */ }
    ```

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

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

## Module `token_stream`

Public implementation details for the `TokenStream` type, such as iterators.

```rust
pub mod token_stream { /* ... */ }
```

### Types

#### Struct `IntoIter`

An iterator over `TokenStream`'s `TokenTree`s.

The iteration is "shallow", e.g. the iterator doesn't recurse into
delimited groups, and returns whole groups as token trees.

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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<TokenTree> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntoIter { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Re-exports

#### Re-export `TokenStream`

```rust
pub use crate::TokenStream;
```

## Types

### Struct `TokenStream`

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro]`,
`#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.

```rust
pub struct TokenStream {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Returns an empty `TokenStream` containing no token trees.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Checks if this `TokenStream` is empty.

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = TokenTree>>(streams: I) -> Self { /* ... */ }
    ```

  - ```rust
    fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TokenStream { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(inner: proc_macro::TokenStream) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(inner: TokenStream) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(token: TokenTree) -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(src: &str) -> Result<TokenStream, LexError> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = TokenTree>>(self: &mut Self, streams: I) { /* ... */ }
    ```

  - ```rust
    fn extend<I: IntoIterator<Item = TokenStream>>(self: &mut Self, streams: I) { /* ... */ }
    ```

- **UnwindSafe**
### Struct `LexError`

Error returned from `TokenStream::from_str`.

```rust
pub struct LexError {
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
  pub fn span(self: &Self) -> Span { /* ... */ }
  ```

##### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Error**
### Struct `Span`

A region of source code, along with macro expansion information.

```rust
pub struct Span {
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
  pub fn call_site() -> Self { /* ... */ }
  ```
  The span of the invocation of the current procedural macro.

- ```rust
  pub fn mixed_site() -> Self { /* ... */ }
  ```
  The span located at the invocation of the procedural macro, but with

- ```rust
  pub fn resolved_at(self: &Self, other: Span) -> Span { /* ... */ }
  ```
  Creates a new span with the same line/column information as `self` but

- ```rust
  pub fn located_at(self: &Self, other: Span) -> Span { /* ... */ }
  ```
  Creates a new span with the same name resolution behavior as `self` but

- ```rust
  pub fn unwrap(self: Self) -> proc_macro::Span { /* ... */ }
  ```
  Convert `proc_macro2::Span` to `proc_macro::Span`.

- ```rust
  pub fn join(self: &Self, other: Span) -> Option<Span> { /* ... */ }
  ```
  Create a new span encompassing `self` and `other`.

- ```rust
  pub fn source_text(self: &Self) -> Option<String> { /* ... */ }
  ```
  Returns the source text behind a span. This preserves the original

##### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Span { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

  - ```rust
    fn from(proc_span: proc_macro::Span) -> Self { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
### Enum `TokenTree`

A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).

```rust
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}
```

#### Variants

##### `Group`

A token stream surrounded by bracket delimiters.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Group` |  |

##### `Ident`

An identifier.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Ident` |  |

##### `Punct`

A single punctuation character (`+`, `,`, `$`, etc.).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Punct` |  |

##### `Literal`

A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Literal` |  |

#### Implementations

##### Methods

- ```rust
  pub fn span(self: &Self) -> Span { /* ... */ }
  ```
  Returns the span of this tree, delegating to the `span` method of

- ```rust
  pub fn set_span(self: &mut Self, span: Span) { /* ... */ }
  ```
  Configures the span for *only this token*.

##### Trait Implementations

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = TokenTree>>(self: &mut Self, streams: I) { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TokenTree { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(token: TokenTree) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(g: Group) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(g: Ident) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(g: Punct) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(g: Literal) -> Self { /* ... */ }
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

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = TokenTree>>(streams: I) -> Self { /* ... */ }
    ```

### Struct `Group`

A delimited token stream.

A `Group` internally contains a `TokenStream` which is surrounded by
`Delimiter`s.

```rust
pub struct Group {
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
  pub fn new(delimiter: Delimiter, stream: TokenStream) -> Self { /* ... */ }
  ```
  Creates a new `Group` with the given delimiter and token stream.

- ```rust
  pub fn delimiter(self: &Self) -> Delimiter { /* ... */ }
  ```
  Returns the punctuation used as the delimiter for this group: a set of

- ```rust
  pub fn stream(self: &Self) -> TokenStream { /* ... */ }
  ```
  Returns the `TokenStream` of tokens that are delimited in this `Group`.

- ```rust
  pub fn span(self: &Self) -> Span { /* ... */ }
  ```
  Returns the span for the delimiters of this token stream, spanning the

- ```rust
  pub fn span_open(self: &Self) -> Span { /* ... */ }
  ```
  Returns the span pointing to the opening delimiter of this group.

- ```rust
  pub fn span_close(self: &Self) -> Span { /* ... */ }
  ```
  Returns the span pointing to the closing delimiter of this group.

- ```rust
  pub fn delim_span(self: &Self) -> DelimSpan { /* ... */ }
  ```
  Returns an object that holds this group's `span_open()` and

- ```rust
  pub fn set_span(self: &mut Self, span: Span) { /* ... */ }
  ```
  Configures the span for this `Group`'s delimiters, but not its internal

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Group { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(g: Group) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Enum `Delimiter`

Describes how a sequence of token trees is delimited.

```rust
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None,
}
```

#### Variants

##### `Parenthesis`

`( ... )`

##### `Brace`

`{ ... }`

##### `Bracket`

`[ ... ]`

##### `None`

`∅ ... ∅`

An invisible delimiter, that may, for example, appear around tokens
coming from a "macro variable" `$var`. It is important to preserve
operator priorities in cases like `$var * 3` where `$var` is `1 + 2`.
Invisible delimiters may not survive roundtrip of a token stream through
a string.

<div class="warning">

Note: rustc currently can ignore the grouping of tokens delimited by `None` in the output
of a proc_macro. Only `None`-delimited groups created by a macro_rules macro in the input
of a proc_macro macro are preserved, and only in very specific circumstances.
Any `None`-delimited groups (re)created by a proc_macro will therefore not preserve
operator priorities as indicated above. The other `Delimiter` variants should be used
instead in this context. This is a rustc bug. For details, see
[rust-lang/rust#67062](https://github.com/rust-lang/rust/issues/67062).

</div>

#### Implementations

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Delimiter { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Delimiter) -> bool { /* ... */ }
    ```

- **Sync**
- **Eq**
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
### Struct `Punct`

A `Punct` is a single punctuation character like `+`, `-` or `#`.

Multicharacter operators like `+=` are represented as two instances of
`Punct` with different forms of `Spacing` returned.

```rust
pub struct Punct {
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
  pub fn new(ch: char, spacing: Spacing) -> Self { /* ... */ }
  ```
  Creates a new `Punct` from the given character and spacing.

- ```rust
  pub fn as_char(self: &Self) -> char { /* ... */ }
  ```
  Returns the value of this punctuation character as `char`.

- ```rust
  pub fn spacing(self: &Self) -> Spacing { /* ... */ }
  ```
  Returns the spacing of this punctuation character, indicating whether

- ```rust
  pub fn span(self: &Self) -> Span { /* ... */ }
  ```
  Returns the span for this punctuation character.

- ```rust
  pub fn set_span(self: &mut Self, span: Span) { /* ... */ }
  ```
  Configure the span for this punctuation character.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Punct { /* ... */ }
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

- **RefUnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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
    fn from(g: Punct) -> Self { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Enum `Spacing`

Whether a `Punct` is followed immediately by another `Punct` or followed by
another token or whitespace.

```rust
pub enum Spacing {
    Alone,
    Joint,
}
```

#### Variants

##### `Alone`

E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.

##### `Joint`

E.g. `+` is `Joint` in `+=` or `'` is `Joint` in `'#`.

Additionally, single quote `'` can join with identifiers to form
lifetimes `'ident`.

#### Implementations

##### Trait Implementations

- **Eq**
- **Freeze**
- **UnwindSafe**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Spacing) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Copy**
- **Unpin**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Spacing { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

### Struct `Ident`

A word of Rust code, which may be a keyword or legal variable name.

An identifier consists of at least one Unicode code point, the first of
which has the XID_Start property and the rest of which have the XID_Continue
property.

- The empty string is not an identifier. Use `Option<Ident>`.
- A lifetime is not an identifier. Use `syn::Lifetime` instead.

An identifier constructed with `Ident::new` is permitted to be a Rust
keyword, though parsing one through its [`Parse`] implementation rejects
Rust keywords. Use `input.call(Ident::parse_any)` when parsing to match the
behaviour of `Ident::new`.

[`Parse`]: https://docs.rs/syn/2.0/syn/parse/trait.Parse.html

# Examples

A new ident can be created from a string using the `Ident::new` function.
A span must be provided explicitly which governs the name resolution
behavior of the resulting identifier.

```
use proc_macro2::{Ident, Span};

fn main() {
    let call_ident = Ident::new("calligraphy", Span::call_site());

    println!("{}", call_ident);
}
```

An ident can be interpolated into a token stream using the `quote!` macro.

```
use proc_macro2::{Ident, Span};
use quote::quote;

fn main() {
    let ident = Ident::new("demo", Span::call_site());

    // Create a variable binding whose name is this ident.
    let expanded = quote! { let #ident = 10; };

    // Create a variable binding with a slightly different name.
    let temp_ident = Ident::new(&format!("new_{}", ident), Span::call_site());
    let expanded = quote! { let #temp_ident = 10; };
}
```

A string representation of the ident is available through the `to_string()`
method.

```
# use proc_macro2::{Ident, Span};
#
# let ident = Ident::new("another_identifier", Span::call_site());
#
// Examine the ident as a string.
let ident_string = ident.to_string();
if ident_string.len() > 60 {
    println!("Very long identifier: {}", ident_string)
}
```

```rust
pub struct Ident {
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
  pub fn new(string: &str, span: Span) -> Self { /* ... */ }
  ```
  Creates a new `Ident` with the given `string` as well as the specified

- ```rust
  pub fn new_raw(string: &str, span: Span) -> Self { /* ... */ }
  ```
  Same as `Ident::new`, but creates a raw identifier (`r#ident`). The

- ```rust
  pub fn span(self: &Self) -> Span { /* ... */ }
  ```
  Returns the span of this `Ident`.

- ```rust
  pub fn set_span(self: &mut Self, span: Span) { /* ... */ }
  ```
  Configures the span of this `Ident`, possibly changing its hygiene

##### Trait Implementations

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Ident) -> Ordering { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ident) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

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
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Ident) -> Option<Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, hasher: &mut H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ident { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(g: Ident) -> Self { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `Literal`

A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
byte character (`b'a'`), an integer or floating point number with or without
a suffix (`1`, `1u8`, `2.3`, `2.3f32`).

Boolean literals like `true` and `false` do not belong here, they are
`Ident`s.

```rust
pub struct Literal {
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
  pub fn u8_suffixed(n: u8) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn u16_suffixed(n: u16) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn u32_suffixed(n: u32) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn u64_suffixed(n: u64) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn u128_suffixed(n: u128) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn usize_suffixed(n: usize) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn i8_suffixed(n: i8) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn i16_suffixed(n: i16) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn i32_suffixed(n: i32) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn i64_suffixed(n: i64) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn i128_suffixed(n: i128) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn isize_suffixed(n: isize) -> Literal { /* ... */ }
  ```
  Creates a new suffixed integer literal with the specified value.

- ```rust
  pub fn u8_unsuffixed(n: u8) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn u16_unsuffixed(n: u16) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn u32_unsuffixed(n: u32) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn u64_unsuffixed(n: u64) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn u128_unsuffixed(n: u128) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn usize_unsuffixed(n: usize) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn i8_unsuffixed(n: i8) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn i16_unsuffixed(n: i16) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn i32_unsuffixed(n: i32) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn i64_unsuffixed(n: i64) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn i128_unsuffixed(n: i128) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn isize_unsuffixed(n: isize) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed integer literal with the specified value.

- ```rust
  pub fn f64_unsuffixed(f: f64) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed floating-point literal.

- ```rust
  pub fn f64_suffixed(f: f64) -> Literal { /* ... */ }
  ```
  Creates a new suffixed floating-point literal.

- ```rust
  pub fn f32_unsuffixed(f: f32) -> Literal { /* ... */ }
  ```
  Creates a new unsuffixed floating-point literal.

- ```rust
  pub fn f32_suffixed(f: f32) -> Literal { /* ... */ }
  ```
  Creates a new suffixed floating-point literal.

- ```rust
  pub fn string(string: &str) -> Literal { /* ... */ }
  ```
  String literal.

- ```rust
  pub fn character(ch: char) -> Literal { /* ... */ }
  ```
  Character literal.

- ```rust
  pub fn byte_character(byte: u8) -> Literal { /* ... */ }
  ```
  Byte character literal.

- ```rust
  pub fn byte_string(bytes: &[u8]) -> Literal { /* ... */ }
  ```
  Byte string literal.

- ```rust
  pub fn c_string(string: &CStr) -> Literal { /* ... */ }
  ```
  C string literal.

- ```rust
  pub fn span(self: &Self) -> Span { /* ... */ }
  ```
  Returns the span encompassing this literal.

- ```rust
  pub fn set_span(self: &mut Self, span: Span) { /* ... */ }
  ```
  Configures the span associated for this literal.

- ```rust
  pub fn subspan<R: RangeBounds<usize>>(self: &Self, range: R) -> Option<Span> { /* ... */ }
  ```
  Returns a `Span` that is a subset of `self.span()` containing only

##### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(repr: &str) -> Result<Self, LexError> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(g: Literal) -> Self { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Literal { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

