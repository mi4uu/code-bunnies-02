# Crate Documentation

**Version:** 0.5.5

**Format Version:** 43

# Module `writeable`

`writeable` is a utility crate of the [`ICU4X`] project.

It includes [`Writeable`], a core trait representing an object that can be written to a
sink implementing `std::fmt::Write`. It is an alternative to `std::fmt::Display` with the
addition of a function indicating the number of bytes to be written.

`Writeable` improves upon `std::fmt::Display` in two ways:

1. More efficient, since the sink can pre-allocate bytes.
2. Smaller code, since the format machinery can be short-circuited.

# Examples

```
use std::fmt;
use writeable::assert_writeable_eq;
use writeable::LengthHint;
use writeable::Writeable;

struct WelcomeMessage<'s> {
    pub name: &'s str,
}

impl<'s> Writeable for WelcomeMessage<'s> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str("Hello, ")?;
        sink.write_str(self.name)?;
        sink.write_char('!')?;
        Ok(())
    }

    fn writeable_length_hint(&self) -> LengthHint {
        // "Hello, " + '!' + length of name
        LengthHint::exact(8 + self.name.len())
    }
}

let message = WelcomeMessage { name: "Alice" };
assert_writeable_eq!(&message, "Hello, Alice!");

// Types implementing `Writeable` are recommended to also implement `fmt::Display`.
// This can be simply done by redirecting to the `Writeable` implementation:
writeable::impl_display_with_writeable!(WelcomeMessage<'_>);
```

[`ICU4X`]: ../icu/index.html

## Modules

## Module `adapters`

Helper types for trait impls.

```rust
pub mod adapters { /* ... */ }
```

### Re-exports

#### Re-export `CoreWriteAsPartsWrite`

```rust
pub use parts_write_adapter::CoreWriteAsPartsWrite;
```

#### Re-export `TryWriteableInfallibleAsWriteable`

```rust
pub use try_writeable::TryWriteableInfallibleAsWriteable;
```

#### Re-export `WriteableAsTryWriteableInfallible`

```rust
pub use try_writeable::WriteableAsTryWriteableInfallible;
```

## Types

### Struct `LengthHint`

**Attributes:**

- `#[non_exhaustive]`

A hint to help consumers of `Writeable` pre-allocate bytes before they call
[`write_to`](Writeable::write_to).

This behaves like `Iterator::size_hint`: it is a tuple where the first element is the
lower bound, and the second element is the upper bound. If the upper bound is `None`
either there is no known upper bound, or the upper bound is larger than `usize`.

`LengthHint` implements std`::ops::{Add, Mul}` and similar traits for easy composition.
During computation, the lower bound will saturate at `usize::MAX`, while the upper
bound will become `None` if `usize::MAX` is exceeded.

```rust
pub struct LengthHint(pub usize, pub Option<usize>);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |
| 1 | `Option<usize>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn undefined() -> Self { /* ... */ }
  ```

- ```rust
  pub fn exact(n: usize) -> Self { /* ... */ }
  ```
  `write_to` will use exactly n bytes.

- ```rust
  pub fn at_least(n: usize) -> Self { /* ... */ }
  ```
  `write_to` will use at least n bytes.

- ```rust
  pub fn at_most(n: usize) -> Self { /* ... */ }
  ```
  `write_to` will use at most n bytes.

- ```rust
  pub fn between(n: usize, m: usize) -> Self { /* ... */ }
  ```
  `write_to` will use between `n` and `m` bytes.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns a recommendation for the number of bytes to pre-allocate.

- ```rust
  pub fn is_zero(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the `LengthHint` indicates that the string is exactly 0 bytes long.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Mul**
  - ```rust
    fn mul(self: Self, other: usize) -> Self { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, other: usize) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: LengthHint) -> Self { /* ... */ }
    ```
    Returns a new hint that is correct wherever `self` is correct, and wherever

- **Freeze**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LengthHint) -> bool { /* ... */ }
    ```

- **Eq**
- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, other: Self) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, other: usize) { /* ... */ }
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LengthHint { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sum**
  - ```rust
    fn sum<I>(iter: I) -> Self
where
    I: Iterator<Item = LengthHint> { /* ... */ }
    ```

  - ```rust
    fn sum<I>(iter: I) -> Self
where
    I: Iterator<Item = usize> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: LengthHint) -> Self { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, other: usize) -> Self { /* ... */ }
    ```

### Struct `Part`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

[`Part`]s are used as annotations for formatted strings. For example, a string like
`Alice, Bob` could assign a `NAME` part to the substrings `Alice` and `Bob`, and a
`PUNCTUATION` part to `, `. This allows for example to apply styling only to names.

`Part` contains two fields, whose usage is left up to the producer of the [`Writeable`].
Conventionally, the `category` field will identify the formatting logic that produces
the string/parts, whereas the `value` field will have semantic meaning. `NAME` and
`PUNCTUATION` could thus be defined as
```
# use writeable::Part;
const NAME: Part = Part {
    category: "userlist",
    value: "name",
};
const PUNCTUATION: Part = Part {
    category: "userlist",
    value: "punctuation",
};
```

That said, consumers should not usually have to inspect `Part` internals. Instead,
formatters should expose the `Part`s they produces as constants.

```rust
pub struct Part {
    pub category: &''static str,
    pub value: &''static str,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `category` | `&''static str` |  |
| `value` | `&''static str` |  |

#### Implementations

##### Methods

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Part { /* ... */ }
    ```

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

- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Part) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

## Traits

### Trait `PartsWrite`

A sink that supports annotating parts of the string with `Part`s.

```rust
pub trait PartsWrite: fmt::Write {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `SubPartsWrite`

##### Required Methods

- `with_part`

#### Implementations

This trait is implemented for the following types:

- `CoreWriteAsPartsWrite<W>` with <W: fmt::Write + ?Sized>

### Trait `Writeable`

`Writeable` is an alternative to `std::fmt::Display` with the addition of a length function.

```rust
pub trait Writeable {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Provided Methods

- ```rust
  fn write_to<W: fmt::Write + ?Sized>(self: &Self, sink: &mut W) -> fmt::Result { /* ... */ }
  ```
  Writes a string to the given sink. Errors from the sink are bubbled up.

- ```rust
  fn write_to_parts<S: PartsWrite + ?Sized>(self: &Self, sink: &mut S) -> fmt::Result { /* ... */ }
  ```
  Write bytes and `Part` annotations to the given sink. Errors from the

- ```rust
  fn writeable_length_hint(self: &Self) -> LengthHint { /* ... */ }
  ```
  Returns a hint for the number of UTF-8 bytes that will be written to the sink.

- ```rust
  fn write_to_string(self: &Self) -> Cow<''_, str> { /* ... */ }
  ```
  Creates a new `String` with the data from this `Writeable`. Like `ToString`,

- ```rust
  fn writeable_cmp_bytes(self: &Self, other: &[u8]) -> core::cmp::Ordering { /* ... */ }
  ```
  Compares the contents of this `Writeable` to the given bytes

#### Implementations

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
- `str`
- `alloc::string::String`
- `char`
- `&T` with <T: Writeable + ?Sized>
- `alloc::borrow::Cow<''a, T>` with <''a, T: ?Sized + Writeable + alloc::borrow::ToOwned>
- `alloc::boxed::Box<T>` with <''a, T: ?Sized + Writeable + Writeable>
- `alloc::rc::Rc<T>` with <''a, T: ?Sized + Writeable + Writeable>
- `alloc::sync::Arc<T>` with <''a, T: ?Sized + Writeable + Writeable>
- `TryWriteableInfallibleAsWriteable<T>` with <T>

## Macros

### Macro `assert_try_writeable_eq`

**Attributes:**

- `#[macro_export]`

Testing macros for types implementing [`TryWriteable`].

Arguments, in order:

1. The [`TryWriteable`] under test
2. The expected string value
3. The expected result value, or `Ok(())` if omitted
3. [`*_parts_eq`] only: a list of parts (`[(start, end, Part)]`)

Any remaining arguments get passed to `format!`

The macros tests the following:

- Equality of string content
- Equality of parts ([`*_parts_eq`] only)
- Validity of size hint
- Reflexivity of `cmp_bytes` and order against largest and smallest strings

For a usage example, see [`TryWriteable`].

[`*_parts_eq`]: assert_try_writeable_parts_eq

```rust
pub macro_rules! assert_try_writeable_eq {
    /* macro_rules! assert_try_writeable_eq {
    ($actual_writeable:expr, $expected_str:expr $(,)?) => { ... };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr $(,)?) => { ... };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr, $($arg:tt)+) => { ... };
    (@internal, $actual_writeable:expr, $expected_str:expr, $expected_result:expr, $($arg:tt)+) => { ... };
} */
}
```

### Macro `assert_try_writeable_parts_eq`

**Attributes:**

- `#[macro_export]`

See [`assert_try_writeable_eq`].

```rust
pub macro_rules! assert_try_writeable_parts_eq {
    /* macro_rules! assert_try_writeable_parts_eq {
    ($actual_writeable:expr, $expected_str:expr, $expected_parts:expr $(,)?) => { ... };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr, $expected_parts:expr $(,)?) => { ... };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr, $expected_parts:expr, $($arg:tt)+) => { ... };
} */
}
```

### Macro `impl_display_with_writeable`

**Attributes:**

- `#[macro_export]`

Implements [`Display`](core::fmt::Display) for types that implement [`Writeable`].

It's recommended to do this for every [`Writeable`] type, as it will add
support for `core::fmt` features like [`fmt!`](std::fmt),
[`print!`](std::print), [`write!`](std::write), etc.

```rust
pub macro_rules! impl_display_with_writeable {
    /* macro_rules! impl_display_with_writeable {
    ($type:ty) => { ... };
} */
}
```

### Macro `assert_writeable_eq`

**Attributes:**

- `#[macro_export]`

Testing macros for types implementing [`Writeable`].

Arguments, in order:

1. The [`Writeable`] under test
2. The expected string value
3. [`*_parts_eq`] only: a list of parts (`[(start, end, Part)]`)

Any remaining arguments get passed to `format!`

The macros tests the following:

- Equality of string content
- Equality of parts ([`*_parts_eq`] only)
- Validity of size hint
- Reflexivity of `cmp_bytes` and order against largest and smallest strings

# Examples

```
# use writeable::Writeable;
# use writeable::LengthHint;
# use writeable::Part;
# use writeable::assert_writeable_eq;
# use writeable::assert_writeable_parts_eq;
# use std::fmt::{self, Write};

const WORD: Part = Part {
    category: "foo",
    value: "word",
};

struct Demo;
impl Writeable for Demo {
    fn write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> fmt::Result {
        sink.with_part(WORD, |w| w.write_str("foo"))
    }
    fn writeable_length_hint(&self) -> LengthHint {
        LengthHint::exact(3)
    }
}

writeable::impl_display_with_writeable!(Demo);

assert_writeable_eq!(&Demo, "foo");
assert_writeable_eq!(&Demo, "foo", "Message: {}", "Hello World");

assert_writeable_parts_eq!(&Demo, "foo", [(0, 3, WORD)]);
assert_writeable_parts_eq!(
    &Demo,
    "foo",
    [(0, 3, WORD)],
    "Message: {}",
    "Hello World"
);
```

[`*_parts_eq`]: assert_writeable_parts_eq

```rust
pub macro_rules! assert_writeable_eq {
    /* macro_rules! assert_writeable_eq {
    ($actual_writeable:expr, $expected_str:expr $(,)?) => { ... };
    ($actual_writeable:expr, $expected_str:expr, $($arg:tt)+) => { ... };
    (@internal, $actual_writeable:expr, $expected_str:expr, $($arg:tt)+) => { ... };
} */
}
```

### Macro `assert_writeable_parts_eq`

**Attributes:**

- `#[macro_export]`

See [`assert_writeable_eq`].

```rust
pub macro_rules! assert_writeable_parts_eq {
    /* macro_rules! assert_writeable_parts_eq {
    ($actual_writeable:expr, $expected_str:expr, $expected_parts:expr $(,)?) => { ... };
    ($actual_writeable:expr, $expected_str:expr, $expected_parts:expr, $($arg:tt)+) => { ... };
} */
}
```

## Re-exports

### Re-export `TryWriteable`

```rust
pub use try_writeable::TryWriteable;
```

