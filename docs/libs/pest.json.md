# Crate Documentation

**Version:** 2.8.0

**Format Version:** 43

# Module `pest`

# pest. The Elegant Parser

pest is a general purpose parser written in Rust with a focus on accessibility, correctness,
and performance. It uses parsing expression grammars (or [PEG]) as input, which are similar in
spirit to regular expressions, but which offer the enhanced expressivity needed to parse
complex languages.

[PEG]: https://en.wikipedia.org/wiki/Parsing_expression_grammar

## Getting started

The recommended way to start parsing with pest is to read the official [book].

Other helpful resources:

* API reference on [docs.rs]
* play with grammars and share them on our [fiddle]
* find previous common questions answered or ask questions on [GitHub Discussions]
* leave feedback, ask questions, or greet us on [Gitter] or [Discord]

[book]: https://pest.rs/book
[docs.rs]: https://docs.rs/pest
[fiddle]: https://pest.rs/#editor
[Gitter]: https://gitter.im/pest-parser/pest
[Discord]: https://discord.gg/XEGACtWpT2
[GitHub Discussions]: https://github.com/pest-parser/pest/discussions

## Usage

The core of pest is the trait [`Parser`], which provides an interface to the parsing
functionality.

The accompanying crate `pest_derive` can automatically generate a [`Parser`] from a PEG
grammar. Using `pest_derive` is highly encouraged, but it is also possible to implement
[`Parser`] manually if required.

## `.pest` files

Grammar definitions reside in custom `.pest` files located in the crate `src` directory.
Parsers are automatically generated from these files using `#[derive(Parser)]` and a special
`#[grammar = "..."]` attribute on a dummy struct.

```ignore
#[derive(Parser)]
#[grammar = "path/to/my_grammar.pest"] // relative to src
struct MyParser;
```

The syntax of `.pest` files is documented in the [`pest_derive` crate].

## Inline grammars

Grammars can also be inlined by using the `#[grammar_inline = "..."]` attribute.

[`Parser`]: trait.Parser.html
[`pest_derive` crate]: https://docs.rs/pest_derive/

## Grammar

A grammar is a series of rules separated by whitespace, possibly containing comments.

### Comments

Comments start with `//` and end at the end of the line.

```text
// a comment
```

### Rules

Rules have the following form:

```ignore
name = optional_modifier { expression }
```

The name of the rule is formed from alphanumeric characters or `_` with the condition that the
first character is not a digit and is used to create token pairs. When the rule starts being
parsed, the starting part of the token is being produced, with the ending part being produced
when the rule finishes parsing.

The following token pair notation `a(b(), c())` denotes the tokens: start `a`, start `b`, end
`b`, start `c`, end `c`, end `a`.

#### Modifiers

Modifiers are optional and can be one of `_`, `@`, `$`, or `!`. These modifiers change the
behavior of the rules.

1. Silent (`_`)

    Silent rules do not create token pairs during parsing, nor are they error-reported.

    ```ignore
    a = _{ "a" }
    b =  { a ~ "b" }
    ```

    Parsing `"ab"` produces the token pair `b()`.

2. Atomic (`@`)

    Atomic rules do not accept whitespace or comments within their expressions and have a
    cascading effect on any rule they call. I.e. rules that are not atomic but are called by atomic
    rules behave atomically.

    Any rules called by atomic rules do not generate token pairs.

    ```ignore
    a =  { "a" }
    b = @{ a ~ "b" }

    WHITESPACE = _{ " " }
    ```

    Parsing `"ab"` produces the token pair `b()`, while `"a   b"` produces an error.

3. Compound-atomic (`$`)

    Compound-atomic are identical to atomic rules with the exception that rules called by them are
    not forbidden from generating token pairs.

    ```ignore
    a =  { "a" }
    b = ${ a ~ "b" }

    WHITESPACE = _{ " " }
    ```

    Parsing `"ab"` produces the token pairs `b(a())`, while `"a   b"` produces an error.

4. Non-atomic (`!`)

    Non-atomic are identical to normal rules with the exception that they stop the cascading effect
    of atomic and compound-atomic rules.

    ```ignore
    a =  { "a" }
    b = !{ a ~ "b" }
    c = @{ b }

    WHITESPACE = _{ " " }
    ```

    Parsing both `"ab"` and `"a   b"` produce the token pairs `c(a())`.

#### Expressions

Expressions can be either terminals or non-terminals.

1. Terminals

| Terminal   | Usage                                                          |
|------------|----------------------------------------------------------------|
| `"a"`      | matches the exact string `"a"`                                 |
| `^"a"`     | matches the exact string `"a"` case insensitively (ASCII only) |
| `'a'..'z'` | matches one character between `'a'` and `'z'`                  |
| `a`        | matches rule `a`                                               |

Strings and characters follow
[Rust's escape mechanisms](https://doc.rust-lang.org/reference/tokens.html#byte-escapes), while
identifiers can contain alphanumeric characters and underscores (`_`), as long as they do not
start with a digit.

2. Non-terminals

| Non-terminal          | Usage                                                      |
|-----------------------|------------------------------------------------------------|
| `(e)`                 | matches `e`                                                |
| `e1 ~ e2`             | matches the sequence `e1` `e2`                             |
| <code>e1 \| e2</code> | matches either `e1` or `e2`                                |
| `e*`                  | matches `e` zero or more times                             |
| `e+`                  | matches `e` one or more times                              |
| `e{n}`                | matches `e` exactly `n` times                              |
| `e{, n}`              | matches `e` at most `n` times                              |
| `e{n,}`               | matches `e` at least `n` times                             |
| `e{m, n}`             | matches `e` between `m` and `n` times inclusively          |
| `e?`                  | optionally matches `e`                                     |
| `&e`                  | matches `e` without making progress                        |
| `!e`                  | matches if `e` doesn't match without making progress       |
| `PUSH(e)`             | matches `e` and pushes it's captured string down the stack |

where `e`, `e1`, and `e2` are expressions.

Matching is greedy, without backtracking.  Note the difference in behavior for
these two rules in matching identifiers that don't end in an underscore:

```ignore
// input: ab_bb_b

identifier = @{ "a" ~ ("b"|"_")* ~ "b" }
// matches:      a     b_bb_b       nothing -> error!      

identifier = @{ "a" ~ ("_"* ~ "b")* }
// matches:      a     b, _bb, _b   in three repetitions
```

Expressions can modify the stack only if they match the input. For example,
if `e1` in the compound expression `e1 | e2` does not match the input, then
it does not modify the stack, so `e2` sees the stack in the same state as
`e1` did. Repetitions and optionals (`e*`, `e+`, `e{, n}`, `e{n,}`,
`e{m,n}`, `e?`) can modify the stack each time `e` matches. The `!e` and `&e`
expressions are a special case; they never modify the stack.
Many languages have "keyword" tokens (e.g. if, for, while) as well as general
tokens (e.g. identifier) that matches any word. In order to match a keyword,
generally, you may need to restrict that is not immediately followed by another
letter or digit (otherwise it would be matched as an identifier).

## Special rules

Special rules can be called within the grammar. They are:

* `WHITESPACE` - runs between rules and sub-rules
* `COMMENT` - runs between rules and sub-rules
* `ANY` - matches exactly one `char`
* `SOI` - (start-of-input) matches only when a `Parser` is still at the starting position
* `EOI` - (end-of-input) matches only when a `Parser` has reached its end
* `PUSH` - matches a string and pushes it to the stack
* `PUSH_LITERAL` - pushes a literal string to the stack
* `POP` - pops a string from the stack and matches it
* `POP_ALL` - pops the entire state of the stack and matches it
* `PEEK` - peeks a string from the stack and matches it
* `PEEK[a..b]` - peeks part of the stack and matches it
* `PEEK_ALL` - peeks the entire state of the stack and matches it
* `DROP` - drops the top of the stack (fails to match if the stack is empty)

`WHITESPACE` and `COMMENT` should be defined manually if needed. All other rules cannot be
overridden.

## `WHITESPACE` and `COMMENT`

When defined, these rules get matched automatically in sequences (`~`) and repetitions
(`*`, `+`) between expressions. Atomic rules and those rules called by atomic rules are exempt
from this behavior.

These rules should be defined so as to match one whitespace character and one comment only since
they are run in repetitions.

If both `WHITESPACE` and `COMMENT` are defined, this grammar:

```ignore
a = { b ~ c }
```

is effectively transformed into this one behind the scenes:

```ignore
a = { b ~ WHITESPACE* ~ (COMMENT ~ WHITESPACE*)* ~ c }
```

## `PUSH`, `PUSH_LITERAL`, `POP`, `DROP`, and `PEEK`

`PUSH(e)` simply pushes the captured string of the expression `e` down a stack. This stack can
then later be used to match grammar based on its content with `POP` and `PEEK`.

`PUSH_LITERAL("a")` pushes the argument to the stack without considering the input. The
argument must be a literal string. This is often useful in conjunction with another rule before
it. For example, `"[" ~ PUSH_LITERAL("]")` will look for an opening bracket `[` and, if it
finds one, will push a closing bracket `]` to the stack. **Note**: `PUSH_LITERAL` requires the
`grammar-extras` feature to be enabled.

`PEEK` always matches the string at the top of stack. So, if the stack contains `["b", "a"]`
(`"a"` being on top), this grammar:

```ignore
a = { PEEK }
```

is effectively transformed into at parse time:

```ignore
a = { "a" }
```

`POP` works the same way with the exception that it pops the string off of the stack if the
match worked. With the stack from above, if `POP` matches `"a"`, the stack will be mutated
to `["b"]`.

`DROP` makes it possible to remove the string at the top of the stack
without matching it. If the stack is nonempty, `DROP` drops the top of the
stack. If the stack is empty, then `DROP` fails to match.

### Advanced peeking

`PEEK[start..end]` and `PEEK_ALL` allow to peek deeper into the stack. The syntax works exactly
like Rust’s exclusive slice syntax. Additionally, negative indices can be used to indicate an
offset from the top. If the end lies before or at the start, the expression matches (as does
a `PEEK_ALL` on an empty stack). With the stack `["c", "b", "a"]` (`"a"` on top):

```ignore
fill = PUSH("c") ~ PUSH("b") ~ PUSH("a")
v = { PEEK_ALL } = { "a" ~ "b" ~ "c" }  // top to bottom
w = { PEEK[..] } = { "c" ~ "b" ~ "a" }  // bottom to top
x = { PEEK[1..2] } = { PEEK[1..-1] } = { "b" }
y = { PEEK[..-2] } = { PEEK[0..1] } = { "a" }
z = { PEEK[1..] } = { PEEK[-2..3] } = { "c" ~ "b" }
n = { PEEK[2..-2] } = { PEEK[2..1] } = { "" }
```

For historical reasons, `PEEK_ALL` matches from top to bottom, while `PEEK[start..end]` matches
from bottom to top. There is currently no syntax to match a slice of the stack top to bottom.

## `Rule`

All rules defined or used in the grammar populate a generated `enum` called `Rule`. This
implements `pest`'s `RuleType` and can be used throughout the API.

## `Built-in rules`

Pest also comes with a number of built-in rules for convenience. They are:

* `ASCII_DIGIT` - matches a numeric character from 0..9
* `ASCII_NONZERO_DIGIT` - matches a numeric character from 1..9
* `ASCII_BIN_DIGIT` - matches a numeric character from 0..1
* `ASCII_OCT_DIGIT` - matches a numeric character from 0..7
* `ASCII_HEX_DIGIT` - matches a numeric character from 0..9 or a..f or A..F
* `ASCII_ALPHA_LOWER` - matches a character from a..z
* `ASCII_ALPHA_UPPER` - matches a character from A..Z
* `ASCII_ALPHA` - matches a character from a..z or A..Z
* `ASCII_ALPHANUMERIC` - matches a character from a..z or A..Z or 0..9
* `ASCII` - matches a character from \x00..\x7f
* `NEWLINE` - matches either "\n" or "\r\n" or "\r"

## Modules

## Module `error`

Types for different kinds of parsing failures.

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `Error`

**Attributes:**

- `#[<cfg_attr>(feature = "std", derive(thiserror::Error))]`

Parse-related error type.

```rust
pub struct Error<R> {
    pub variant: ErrorVariant<R>,
    pub location: InputLocation,
    pub line_col: LineColLocation,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `variant` | `ErrorVariant<R>` | Variant of the error |
| `location` | `InputLocation` | Location within the input string |
| `line_col` | `LineColLocation` | Line/column within the input string |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new_from_pos(variant: ErrorVariant<R>, pos: Position<''_>) -> Error<R> { /* ... */ }
  ```
  Creates `Error` from `ErrorVariant` and `Position`.

- ```rust
  pub fn new_from_span(variant: ErrorVariant<R>, span: Span<''_>) -> Error<R> { /* ... */ }
  ```
  Creates `Error` from `ErrorVariant` and `Span`.

- ```rust
  pub fn with_path(self: Self, path: &str) -> Error<R> { /* ... */ }
  ```
  Returns `Error` variant with `path` which is shown when formatted with `Display`.

- ```rust
  pub fn path(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns the path set using [`Error::with_path()`].

- ```rust
  pub fn line(self: &Self) -> &str { /* ... */ }
  ```
  Returns the line that the error is on.

- ```rust
  pub fn renamed_rules<F>(self: Self, f: F) -> Error<R>
where
    F: FnMut(&R) -> String { /* ... */ }
  ```
  Renames all `Rule`s if this is a [`ParsingError`]. It does nothing when called on a

- ```rust
  pub fn parse_attempts(self: &Self) -> Option<ParseAttempts<R>> { /* ... */ }
  ```
  Get detailed information about errored rules sequence.

- ```rust
  pub fn parse_attempts_error(self: &Self, input: &str, rule_to_message: &RuleToMessageFn<R>, is_whitespace: &IsWhitespaceFn) -> Option<Error<R>> { /* ... */ }
  ```
  Get error message based on parsing attempts.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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
- **Freeze**
- **Error**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error<R> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error<R>) -> bool { /* ... */ }
    ```

- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Enum `ErrorVariant`

**Attributes:**

- `#[<cfg_attr>(feature = "std", derive(thiserror::Error))]`

Different kinds of parsing errors.

```rust
pub enum ErrorVariant<R> {
    ParsingError {
        positives: alloc::vec::Vec<R>,
        negatives: alloc::vec::Vec<R>,
    },
    CustomError {
        message: alloc::string::String,
    },
}
```

##### Variants

###### `ParsingError`

Generated parsing error with expected and unexpected `Rule`s

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `positives` | `alloc::vec::Vec<R>` | Positive attempts |
| `negatives` | `alloc::vec::Vec<R>` | Negative attempts |

###### `CustomError`

Custom error with a message

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `message` | `alloc::string::String` | Short explanation |

##### Implementations

###### Methods

- ```rust
  pub fn message(self: &Self) -> Cow<''_, str> { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Error**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ErrorVariant<R> { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ErrorVariant<R>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `InputLocation`

Where an `Error` has occurred.

```rust
pub enum InputLocation {
    Pos(usize),
    Span((usize, usize)),
}
```

##### Variants

###### `Pos`

`Error` was created by `Error::new_from_pos`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `Span`

`Error` was created by `Error::new_from_span`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `(usize, usize)` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InputLocation { /* ... */ }
    ```

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

- **Sync**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InputLocation) -> bool { /* ... */ }
    ```

#### Enum `LineColLocation`

Line/column where an `Error` has occurred.

```rust
pub enum LineColLocation {
    Pos((usize, usize)),
    Span((usize, usize), (usize, usize)),
}
```

##### Variants

###### `Pos`

Line/column pair if `Error` was created by `Error::new_from_pos`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `(usize, usize)` |  |

###### `Span`

Line/column pairs if `Error` was created by `Error::new_from_span`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `(usize, usize)` |  |
| 1 | `(usize, usize)` |  |

##### Implementations

###### Trait Implementations

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LineColLocation) -> bool { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> LineColLocation { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: Position<''_>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Span<''_>) -> Self { /* ... */ }
    ```

#### Type Alias `RuleToMessageFn`

Function mapping rule to its helper message defined by user.

```rust
pub type RuleToMessageFn<R> = alloc::boxed::Box<dyn Fn(&R) -> Option<alloc::string::String>>;
```

#### Type Alias `IsWhitespaceFn`

Function mapping string element to bool denoting whether it's a whitespace defined by user.

```rust
pub type IsWhitespaceFn = alloc::boxed::Box<dyn Fn(alloc::string::String) -> bool>;
```

## Module `iterators`

Types and iterators for parser output.

```rust
pub mod iterators { /* ... */ }
```

### Re-exports

#### Re-export `FlatPairs`

```rust
pub use self::flat_pairs::FlatPairs;
```

#### Re-export `Pair`

```rust
pub use self::pair::Pair;
```

#### Re-export `Pairs`

```rust
pub use self::pairs::Pairs;
```

#### Re-export `Tokens`

```rust
pub use self::tokens::Tokens;
```

## Module `pratt_parser`

Constructs useful in prefix, postfix, and infix operator parsing with the
Pratt parsing method.

```rust
pub mod pratt_parser { /* ... */ }
```

### Types

#### Enum `Assoc`

Associativity of an infix binary operator, used by [`Op::infix(Assoc)`].

[`Op::infix(Assoc)`]: struct.Op.html

```rust
pub enum Assoc {
    Left,
    Right,
}
```

##### Variants

###### `Left`

Left operator associativity. Evaluate expressions from left-to-right.

###### `Right`

Right operator associativity. Evaluate expressions from right-to-left.

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **UnwindSafe**
- **Send**
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
    fn clone(self: &Self) -> Assoc { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Assoc) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **Sync**
- **Freeze**
- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `Op`

An operator that corresponds to a rule.

```rust
pub struct Op<R: RuleType> {
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
  pub fn prefix(rule: R) -> Self { /* ... */ }
  ```
  Defines `rule` as a prefix unary operator.

- ```rust
  pub fn postfix(rule: R) -> Self { /* ... */ }
  ```
  Defines `rule` as a postfix unary operator.

- ```rust
  pub fn infix(rule: R, assoc: Assoc) -> Self { /* ... */ }
  ```
  Defines `rule` as an infix binary operator with associativity `assoc`.

###### Trait Implementations

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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Self) -> Self { /* ... */ }
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
- **UnwindSafe**
- **RefUnwindSafe**
#### Struct `PrattParser`

Struct containing operators and precedences, which can perform [Pratt parsing][1] on
primary, prefix, postfix and infix expressions over [`Pairs`]. The tokens in [`Pairs`]
should alternate in the order:
`prefix* ~ primary ~ postfix* ~ (infix ~ prefix* ~ primary ~ postfix*)*`

# Panics

Panics will occur when:
* `pairs` is empty
* The tokens in `pairs` does not alternate in the expected order.
* No `map_*` function is specified for a certain kind of operator encountered in `pairs`.

# Example

The following pest grammar defines a calculator which can be used for Pratt parsing.

```pest
WHITESPACE   =  _{ " " | "\t" | NEWLINE }
  
program      =   { SOI ~ expr ~ EOI }
  expr       =   { prefix* ~ primary ~ postfix* ~ (infix ~ prefix* ~ primary ~ postfix* )* }
    infix    =  _{ add | sub | mul | div | pow }
      add    =   { "+" } // Addition
      sub    =   { "-" } // Subtraction
      mul    =   { "*" } // Multiplication
      div    =   { "/" } // Division
      pow    =   { "^" } // Exponentiation
    prefix   =  _{ neg }
      neg    =   { "-" } // Negation
    postfix  =  _{ fac }
      fac    =   { "!" } // Factorial
    primary  =  _{ int | "(" ~ expr ~ ")" }
      int    =  @{ (ASCII_NONZERO_DIGIT ~ ASCII_DIGIT+ | ASCII_DIGIT) }
```

Below is a [`PrattParser`] that is able to parse an `expr` in the above grammar. The order
of precedence corresponds to the order in which [`op`] is called. Thus, `mul` will
have higher precedence than `add`. Operators can also be chained with `|` to give them equal
precedence.

```
# use pest::pratt_parser::{Assoc, Op, PrattParser};
# #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
# enum Rule { program, expr, int, add, mul, sub, div, pow, fac, neg }
let pratt =
    PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
        .op(Op::infix(Rule::pow, Assoc::Right))
        .op(Op::prefix(Rule::neg))
        .op(Op::postfix(Rule::fac));
```

To parse an expression, call the [`map_primary`], [`map_prefix`], [`map_postfix`],
[`map_infix`] and [`parse`] methods as follows:

```
# use pest::{iterators::Pairs, pratt_parser::PrattParser};
# #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
# enum Rule { program, expr, int, add, mul, sub, div, pow, fac, neg }
fn parse_expr(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> i32 {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::int  => primary.as_str().parse().unwrap(),
            Rule::expr => parse_expr(primary.into_inner(), pratt), // from "(" ~ expr ~ ")"
            _          => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::neg  => -rhs,
            _          => unreachable!(),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::fac  => (1..lhs+1).product(),
            _          => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add  => lhs + rhs,
            Rule::sub  => lhs - rhs,
            Rule::mul  => lhs * rhs,
            Rule::div  => lhs / rhs,
            Rule::pow  => (1..rhs+1).map(|_| lhs).product(),
            _          => unreachable!(),
        })
        .parse(pairs)
}
```

Note that [`map_prefix`], [`map_postfix`] and [`map_infix`] only need to be specified if the
grammar contains the corresponding operators.

[1]: https://en.wikipedia.org/wiki/Pratt_parser
[`Pairs`]: ../iterators/struct.Pairs.html
[`PrattParser`]: struct.PrattParser.html
[`map_primary`]: struct.PrattParser.html#method.map_primary
[`map_prefix`]: struct.PrattParserMap.html#method.map_prefix
[`map_postfix`]: struct.PrattParserMap.html#method.map_postfix
[`map_infix`]: struct.PrattParserMap.html#method.map_infix
[`parse`]: struct.PrattParserMap.html#method.parse
[`op`]: struct.PrattParserMap.html#method.op

```rust
pub struct PrattParser<R: RuleType> {
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
  Instantiate a new `PrattParser`.

- ```rust
  pub fn op(self: Self, op: Op<R>) -> Self { /* ... */ }
  ```
  Add `op` to `PrattParser`.

- ```rust
  pub fn map_primary<''pratt, ''a, ''i, X, T>(self: &''pratt Self, primary: X) -> PrattParserMap<''pratt, ''a, ''i, R, X, T>
where
    X: FnMut(Pair<''i, R>) -> T,
    R: ''pratt { /* ... */ }
  ```
  Maps primary expressions with a closure `primary`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `PrattParserMap`

Product of calling [`map_primary`] on [`PrattParser`], defines how expressions should
be mapped.

[`map_primary`]: struct.PrattParser.html#method.map_primary
[`PrattParser`]: struct.PrattParser.html

```rust
pub struct PrattParserMap<''pratt, ''a, ''i, R, F, T> {
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
  pub fn map_prefix<X>(self: Self, prefix: X) -> Self
where
    X: FnMut(Pair<''i, R>, T) -> T + ''a { /* ... */ }
  ```
  Maps prefix operators with closure `prefix`.

- ```rust
  pub fn map_postfix<X>(self: Self, postfix: X) -> Self
where
    X: FnMut(T, Pair<''i, R>) -> T + ''a { /* ... */ }
  ```
  Maps postfix operators with closure `postfix`.

- ```rust
  pub fn map_infix<X>(self: Self, infix: X) -> Self
where
    X: FnMut(T, Pair<''i, R>, T) -> T + ''a { /* ... */ }
  ```
  Maps infix operators with a closure `infix`.

- ```rust
  pub fn parse<P: Iterator<Item = Pair<''i, R>>>(self: &mut Self, pairs: P) -> T { /* ... */ }
  ```
  The last method to call on the provided pairs to execute the Pratt

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Send**
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `prec_climber`

**⚠️ Deprecated since 2.4.0**: Use `pest::pratt_parser` instead (it is an equivalent which also supports unary prefix/suffix operators).
While prec_climber is going to be kept in 2.x minor and patch releases, it may be removed in a future major release.

Constructs useful in infix operator parsing with the precedence climbing method.

```rust
pub mod prec_climber { /* ... */ }
```

### Types

#### Enum `Assoc`

**⚠️ Deprecated since 2.4.0**: Use `pest::pratt_parser` instead (it is an equivalent which also supports unary prefix/suffix operators).
While prec_climber is going to be kept in 2.x minor and patch releases, it may be removed in a future major release.

Associativity of an [`Operator`].

[`Operator`]: struct.Operator.html

```rust
pub enum Assoc {
    Left,
    Right,
}
```

##### Variants

###### `Left`

Left `Operator` associativity

###### `Right`

Right `Operator` associativity

##### Implementations

###### Trait Implementations

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

- **RefUnwindSafe**
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

- **Sync**
- **Send**
- **StructuralPartialEq**
- **Unpin**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Assoc) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Assoc { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

#### Struct `Operator`

**⚠️ Deprecated since 2.4.0**: Use `pest::pratt_parser` instead (it is an equivalent which also supports unary prefix/suffix operators).
While prec_climber is going to be kept in 2.x minor and patch releases, it may be removed in a future major release.

Infix operator used in [`PrecClimber`].

[`PrecClimber`]: struct.PrecClimber.html

```rust
pub struct Operator<R: RuleType> {
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
  pub fn new(rule: R, assoc: Assoc) -> Operator<R> { /* ... */ }
  ```
  Creates a new `Operator` from a `Rule` and `Assoc`.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Self) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
#### Struct `PrecClimber`

**⚠️ Deprecated since 2.4.0**: Use `pest::pratt_parser` instead (it is an equivalent which also supports unary prefix/suffix operators).
While prec_climber is going to be kept in 2.x minor and patch releases, it may be removed in a future major release.

List of operators and precedences, which can perform [precedence climbing][1] on infix
expressions contained in a [`Pairs`]. The token pairs contained in the `Pairs` should start
with a *primary* pair and then alternate between an *operator* and a *primary*.

[1]: https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method
[`Pairs`]: ../iterators/struct.Pairs.html

```rust
pub struct PrecClimber<R: Clone + ''static> {
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
  pub fn new(ops: Vec<Operator<R>>) -> PrecClimber<R> { /* ... */ }
  ```
  Creates a new `PrecClimber` from the `Operator`s contained in `ops`. Every entry in the

- ```rust
  pub fn climb<''i, P, F, G, T>(self: &Self, pairs: P, primary: F, infix: G) -> T
where
    P: Iterator<Item = Pair<''i, R>>,
    F: FnMut(Pair<''i, R>) -> T,
    G: FnMut(T, Pair<''i, R>, T) -> T { /* ... */ }
  ```
  Performs the precedence climbing algorithm on the `pairs` in a similar manner to map-reduce.

###### Trait Implementations

- **RefUnwindSafe**
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

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **Send**
## Traits

### Trait `RuleType`

A trait which parser rules must implement.

This trait is set up so that any struct that implements all of its required traits will
automatically implement this trait as well.

This is essentially a [trait alias](https://github.com/rust-lang/rfcs/pull/1733). When trait
aliases are implemented, this may be replaced by one.

```rust
pub trait RuleType: Copy + Debug + Eq + Hash + Ord {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Implementations

This trait is implemented for the following types:

- `T` with <T: Copy + Debug + Eq + Hash + Ord>

## Macros

### Macro `parses_to`

**Attributes:**

- `#[macro_export]`

Testing tool that compares produced tokens.

This macro takes several arguments:

* `parser` - name of the data structure implementing `Parser`
* `input` - input to be tested against
* `rule` - `Rule` which will be run
* `tokens` - token pairs of the form `name(start_pos, end_pos, [nested_child_tokens])`

*Note:* `start_pos` and `end_pos` are byte positions.

# Examples

```
# #[macro_use]
# extern crate pest;
# use pest::Parser;
# use pest::error::Error;
# use pest::iterators::Pairs;
# fn main() {
# #[allow(non_camel_case_types)]
# #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
# enum Rule {
#     a,
#     b,
#     c
# }
#
# struct AbcParser;
#
# impl Parser<Rule> for AbcParser {
#     fn parse<'i>(_: Rule, input: &'i str) -> Result<Pairs<'i, Rule>, Error<Rule>> {
#         pest::state(input, |state| {
#             state.rule(Rule::a, |state| {
#                 state.skip(1).unwrap().rule(Rule::b, |state| {
#                     state.skip(1)
#                 }).unwrap().skip(1)
#             }).and_then(|state| {
#                 state.skip(1).unwrap().rule(Rule::c, |state| {
#                     state.skip(1)
#                 })
#             })
#         })
#     }
# }
parses_to! {
    parser: AbcParser,
    input:  "abcde",
    rule:   Rule::a,
    tokens: [
        a(0, 3, [
            b(1, 2)
        ]),
        c(4, 5)
    ]
};
# }
```

```rust
pub macro_rules! parses_to {
    /* macro_rules! parses_to {
    ( parser: $parser:ident, input: $string:expr, rule: $rules:tt :: $rule:tt,
      tokens: [ $( $names:ident $calls:tt ),* $(,)* ] ) => { ... };
} */
}
```

### Macro `fails_with`

**Attributes:**

- `#[macro_export]`

Testing tool that compares produced errors.

This macro takes several arguments:

* `parser` - name of the data structure implementing `Parser`
* `input` - input to be tested against
* `rule` - `Rule` which will be run
* `positives` - positive `Rule` attempts that failed
* `negatives` - negative `Rule` attempts that failed
* `pos` - byte position of failure

# Examples

```
# #[macro_use]
# extern crate pest;
# use pest::Parser;
# use pest::error::Error;
# use pest::iterators::Pairs;
# fn main() {
# #[allow(non_camel_case_types)]
# #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
# enum Rule {
#     a,
#     b,
#     c
# }
#
# struct AbcParser;
#
# impl Parser<Rule> for AbcParser {
#     fn parse<'i>(_: Rule, input: &'i str) -> Result<Pairs<'i, Rule>, Error<Rule>> {
#         pest::state(input, |state| {
#             state.rule(Rule::a, |state| {
#                 state.skip(1).unwrap().rule(Rule::b, |s| {
#                     s.skip(1)
#                 }).unwrap().skip(1)
#             }).and_then(|state| {
#                 state.skip(1).unwrap().rule(Rule::c, |s| {
#                     s.match_string("e")
#                 })
#             })
#         })
#     }
# }
fails_with! {
    parser: AbcParser,
    input: "abcdf",
    rule: Rule::a,
    positives: vec![Rule::c],
    negatives: vec![],
    pos: 4
};
# }
```

```rust
pub macro_rules! fails_with {
    /* macro_rules! fails_with {
    ( parser: $parser:ident, input: $string:expr, rule: $rules:tt :: $rule:tt,
      positives: $positives:expr, negatives: $negatives:expr, pos: $pos:expr ) => { ... };
} */
}
```

## Re-exports

### Re-export `Parser`

```rust
pub use crate::parser::Parser;
```

### Re-export `set_call_limit`

```rust
pub use crate::parser_state::set_call_limit;
```

### Re-export `set_error_detail`

```rust
pub use crate::parser_state::set_error_detail;
```

### Re-export `state`

```rust
pub use crate::parser_state::state;
```

### Re-export `Atomicity`

```rust
pub use crate::parser_state::Atomicity;
```

### Re-export `Lookahead`

```rust
pub use crate::parser_state::Lookahead;
```

### Re-export `MatchDir`

```rust
pub use crate::parser_state::MatchDir;
```

### Re-export `ParseResult`

```rust
pub use crate::parser_state::ParseResult;
```

### Re-export `ParserState`

```rust
pub use crate::parser_state::ParserState;
```

### Re-export `Position`

```rust
pub use crate::position::Position;
```

### Re-export `merge_spans`

```rust
pub use crate::span::merge_spans;
```

### Re-export `Lines`

```rust
pub use crate::span::Lines;
```

### Re-export `LinesSpan`

```rust
pub use crate::span::LinesSpan;
```

### Re-export `Span`

```rust
pub use crate::span::Span;
```

### Re-export `Stack`

```rust
pub use crate::stack::Stack;
```

### Re-export `Token`

```rust
pub use crate::token::Token;
```

