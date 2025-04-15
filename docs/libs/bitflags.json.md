# Crate Documentation

**Version:** 2.9.0

**Format Version:** 43

# Module `bitflags`

Generate types for C-style flags with ergonomic APIs.

# Getting started

Add `bitflags` to your `Cargo.toml`:

```toml
[dependencies.bitflags]
version = "2.9.0"
```

## Generating flags types

Use the [`bitflags`] macro to generate flags types:

```rust
use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
```

See the docs for the `bitflags` macro for the full syntax.

Also see the [`example_generated`](./example_generated/index.html) module for an example of what the `bitflags` macro generates for a flags type.

### Externally defined flags

If you're generating flags types for an external source, such as a C API, you can define
an extra unnamed flag as a mask of all bits the external source may ever set. Usually this would be all bits (`!0`):

```rust
# use bitflags::bitflags;
bitflags! {
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;

        // The source may set any bits
        const _ = !0;
    }
}
```

Why should you do this? Generated methods like `all` and truncating operators like `!` only consider
bits in defined flags. Adding an unnamed flag makes those methods consider additional bits,
without generating additional constants for them. It helps compatibility when the external source
may start setting additional bits at any time. The [known and unknown bits](#known-and-unknown-bits)
section has more details on this behavior.

### Custom derives

You can derive some traits on generated flags types if you enable Cargo features. The following
libraries are currently supported:

- `serde`: Support `#[derive(Serialize, Deserialize)]`, using text for human-readable formats,
  and a raw number for binary formats.
- `arbitrary`: Support `#[derive(Arbitrary)]`, only generating flags values with known bits.
- `bytemuck`: Support `#[derive(Pod, Zeroable)]`, for casting between flags values and their
  underlying bits values.

You can also define your own flags type outside of the [`bitflags`] macro and then use it to generate methods.
This can be useful if you need a custom `#[derive]` attribute for a library that `bitflags` doesn't
natively support:

```rust
# use std::fmt::Debug as SomeTrait;
# use bitflags::bitflags;
#[derive(SomeTrait)]
pub struct Flags(u32);

bitflags! {
    impl Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
```

### Adding custom methods

The [`bitflags`] macro supports attributes on generated flags types within the macro itself, while
`impl` blocks can be added outside of it:

```rust
# use bitflags::bitflags;
bitflags! {
    // Attributes can be applied to flags types
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}

// Impl blocks can be added to flags types
impl Flags {
    pub fn as_u64(&self) -> u64 {
        self.bits() as u64
    }
}
```

## Working with flags values

Use generated constants and standard bitwise operators to interact with flags values:

```rust
# use bitflags::bitflags;
# bitflags! {
#     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#     pub struct Flags: u32 {
#         const A = 0b00000001;
#         const B = 0b00000010;
#         const C = 0b00000100;
#     }
# }
// union
let ab = Flags::A | Flags::B;

// intersection
let a = ab & Flags::A;

// difference
let b = ab - Flags::A;

// complement
let c = !ab;
```

See the docs for the [`Flags`] trait for more details on operators and how they behave.

# Formatting and parsing

`bitflags` defines a text format that can be used to convert any flags value to and from strings.

See the [`parser`] module for more details.

# Specification

The terminology and behavior of generated flags types is
[specified in the source repository](https://github.com/bitflags/bitflags/blob/main/spec.md).
Details are repeated in these docs where appropriate, but is exhaustively listed in the spec. Some
things are worth calling out explicitly here.

## Flags types, flags values, flags

The spec and these docs use consistent terminology to refer to things in the bitflags domain:

- **Bits type**: A type that defines a fixed number of bits at specific locations.
- **Flag**: A set of bits in a bits type that may have a unique name.
- **Flags type**: A set of defined flags over a specific bits type.
- **Flags value**: An instance of a flags type using its specific bits value for storage.

```
# use bitflags::bitflags;
bitflags! {
    struct FlagsType: u8 {
//                    -- Bits type
//         --------- Flags type
        const A = 1;
//            ----- Flag
    }
}

let flag = FlagsType::A;
//  ---- Flags value
```

## Known and unknown bits

Any bits in a flag you define are called _known bits_. Any other bits are _unknown bits_.
In the following flags type:

```
# use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}
```

The known bits are `0b0000_0111` and the unknown bits are `0b1111_1000`.

`bitflags` doesn't guarantee that a flags value will only ever have known bits set, but some operators
will unset any unknown bits they encounter. In a future version of `bitflags`, all operators will
unset unknown bits.

If you're using `bitflags` for flags types defined externally, such as from C, you probably want all
bits to be considered known, in case that external source changes. You can do this using an unnamed
flag, as described in [externally defined flags](#externally-defined-flags).

## Zero-bit flags

Flags with no bits set should be avoided because they interact strangely with [`Flags::contains`]
and [`Flags::intersects`]. A zero-bit flag is always contained, but is never intersected. The
names of zero-bit flags can be parsed, but are never formatted.

## Multi-bit flags

Flags that set multiple bits should be avoided unless each bit is also in a single-bit flag.
Take the following flags type as an example:

```
# use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 | 1 << 1;
    }
}
```

The result of `Flags::A ^ Flags::B` is `0b0000_0010`, which doesn't correspond to either
`Flags::A` or `Flags::B` even though it's still a known bit.

## Modules

## Module `iter`

Yield the bits of a source flags value in a set of contained flags values.

```rust
pub mod iter { /* ... */ }
```

### Types

#### Struct `Iter`

An iterator over flags values.

This iterator will yield flags values for contained, defined flags first, with any remaining bits yielded
as a final flags value.

```rust
pub struct Iter<B: ''static> {
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
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `IterNames`

An iterator over flags values.

This iterator only yields flags values for contained, defined, named flags. Any remaining bits
won't be yielded, but can be found with the [`IterNames::remaining`] method.

```rust
pub struct IterNames<B: ''static> {
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
  pub fn remaining(self: &Self) -> &B { /* ... */ }
  ```
  Get a flags value of any remaining bits that haven't been yielded yet.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
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

- **Send**
- **Freeze**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `parser`

**Attributes:**

- `#[allow(clippy::let_unit_value)]`

Parsing flags from text.

Format and parse a flags value as text using the following grammar:

- _Flags:_ (_Whitespace_ _Flag_ _Whitespace_)`|`*
- _Flag:_ _Name_ | _Hex Number_
- _Name:_ The name of any defined flag
- _Hex Number_: `0x`([0-9a-fA-F])*
- _Whitespace_: (\s)*

As an example, this is how `Flags::A | Flags::B | 0x0c` can be represented as text:

```text
A | B | 0x0c
```

Alternatively, it could be represented without whitespace:

```text
A|B|0x0C
```

Note that identifiers are *case-sensitive*, so the following is *not equivalent*:

```text
a|b|0x0C
```

```rust
pub mod parser { /* ... */ }
```

### Types

#### Struct `ParseError`

An error encountered while parsing flags from text.

```rust
pub struct ParseError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn invalid_hex_flag</* synthetic */ impl fmt::Display: fmt::Display>(flag: impl fmt::Display) -> Self { /* ... */ }
  ```
  An invalid hex flag was encountered.

- ```rust
  pub fn invalid_named_flag</* synthetic */ impl fmt::Display: fmt::Display>(flag: impl fmt::Display) -> Self { /* ... */ }
  ```
  A named flag that doesn't correspond to any on the flags type was encountered.

- ```rust
  pub const fn empty_flag() -> Self { /* ... */ }
  ```
  A hex or named flag wasn't found between separators.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Error**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `WriteHex`

Encode a value as a hex string.

Implementors of this trait should not write the `0x` prefix.

```rust
pub trait WriteHex {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `write_hex`: Write the value as hex.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`

#### Trait `ParseHex`

Parse a value from a hex string.

```rust
pub trait ParseHex {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `parse_hex`: Parse the value from hex.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`

### Functions

#### Function `to_writer`

Write a flags value as text.

Any bits that aren't part of a contained flag will be formatted as a hex number.

```rust
pub fn to_writer<B: Flags, /* synthetic */ impl Write: Write>(flags: &B, writer: impl Write) -> Result<(), fmt::Error>
where
    <B as >::Bits: WriteHex { /* ... */ }
```

#### Function `from_str`

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
Unknown bits will be retained.

```rust
pub fn from_str<B: Flags>(input: &str) -> Result<B, ParseError>
where
    <B as >::Bits: ParseHex { /* ... */ }
```

#### Function `to_writer_truncate`

Write a flags value as text, ignoring any unknown bits.

```rust
pub fn to_writer_truncate<B: Flags, /* synthetic */ impl Write: Write>(flags: &B, writer: impl Write) -> Result<(), fmt::Error>
where
    <B as >::Bits: WriteHex { /* ... */ }
```

#### Function `from_str_truncate`

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
Unknown bits will be ignored.

```rust
pub fn from_str_truncate<B: Flags>(input: &str) -> Result<B, ParseError>
where
    <B as >::Bits: ParseHex { /* ... */ }
```

#### Function `to_writer_strict`

Write only the contained, defined, named flags in a flags value as text.

```rust
pub fn to_writer_strict<B: Flags, /* synthetic */ impl Write: Write>(flags: &B, writer: impl Write) -> Result<(), fmt::Error> { /* ... */ }
```

#### Function `from_str_strict`

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
This function will fail to parse hex values.

```rust
pub fn from_str_strict<B: Flags>(input: &str) -> Result<B, ParseError> { /* ... */ }
```

## Macros

### Macro `bitflags`

**Attributes:**

- `#[macro_export]`

Generate a flags type.

# `struct` mode

A declaration that begins with `$vis struct` will generate a `struct` for a flags type, along with
methods and trait implementations for it. The body of the declaration defines flags as constants,
where each constant is a flags value of the generated flags type.

## Examples

Generate a flags type using `u8` as the bits type:

```
# use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 0b0000_0100;
    }
}
```

Flags types are private by default and accept standard visibility modifiers. Flags themselves
are always public:

```
# use bitflags::bitflags;
bitflags! {
    pub struct Flags: u8 {
        // Constants are always `pub`
        const A = 1;
    }
}
```

Flags may refer to other flags using their [`Flags::bits`] value:

```
# use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const AB = Flags::A.bits() | Flags::B.bits();
    }
}
```

A single `bitflags` invocation may include zero or more flags type declarations:

```
# use bitflags::bitflags;
bitflags! {}

bitflags! {
    struct Flags1: u8 {
        const A = 1;
    }

    struct Flags2: u8 {
        const A = 1;
    }
}
```

# `impl` mode

A declaration that begins with `impl` will only generate methods and trait implementations for the
`struct` defined outside of the `bitflags` macro.

The struct itself must be a newtype using the bits type as its field.

The syntax for `impl` mode is identical to `struct` mode besides the starting token.

## Examples

Implement flags methods and traits for a custom flags type using `u8` as its underlying bits type:

```
# use bitflags::bitflags;
struct Flags(u8);

bitflags! {
    impl Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 0b0000_0100;
    }
}
```

# Named and unnamed flags

Constants in the body of a declaration are flags. The identifier of the constant is the name of
the flag. If the identifier is `_`, then the flag is unnamed. Unnamed flags don't appear in the
generated API, but affect how bits are truncated.

## Examples

Adding an unnamed flag that makes all bits known:

```
# use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;

        const _ = !0;
    }
}
```

Flags types may define multiple unnamed flags:

```
# use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const _ = 1;
        const _ = 1 << 1;
    }
}
```

```rust
pub macro_rules! bitflags {
    /* macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:tt = $value:expr;
            )*
        }

        $($t:tt)*
    ) => { ... };
    (
        $(#[$outer:meta])*
        impl $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:tt = $value:expr;
            )*
        }

        $($t:tt)*
    ) => { ... };
    () => { ... };
} */
}
```

### Macro `bitflags_match`

**Attributes:**

- `#[macro_export]`

A macro that matches flags values, similar to Rust's `match` statement.

In a regular `match` statement, the syntax `Flag::A | Flag::B` is interpreted as an or-pattern,
instead of the bitwise-or of `Flag::A` and `Flag::B`. This can be surprising when combined with flags types
because `Flag::A | Flag::B` won't match the pattern `Flag::A | Flag::B`. This macro is an alternative to
`match` for flags values that doesn't have this issue.

# Syntax

```ignore
bitflags_match!(expression, {
    pattern1 => result1,
    pattern2 => result2,
    ..
    _ => default_result,
})
```

The final `_ => default_result` arm is required, otherwise the macro will fail to compile.

# Examples

```rust
use bitflags::{bitflags, bitflags_match};

bitflags! {
    #[derive(PartialEq)]
    struct Flags: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}

let flags = Flags::A | Flags::B;

bitflags_match!(flags, {
    Flags::A | Flags::B => println!("A and/or B are set"),
    _ => println!("neither A nor B are set"),
})
```

# How it works

The macro expands to a series of `if` statements, checking equality between the input expression
and each pattern. This allows for correct matching of bitflag combinations, which is not possible
with a regular match expression due to the way bitflags are implemented.

Patterns are evaluated in order.

```rust
pub macro_rules! bitflags_match {
    /* macro_rules! bitflags_match {
    ($operation:expr, {
        $($t:tt)*
    }) => { ... };
} */
}
```

## Re-exports

### Re-export `Bits`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use traits::Bits;
```

### Re-export `Flag`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use traits::Flag;
```

### Re-export `Flags`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use traits::Flags;
```

### Re-export `external::*`

**Attributes:**

- `#[allow(unused_imports)]`

```rust
pub use external::*;
```

