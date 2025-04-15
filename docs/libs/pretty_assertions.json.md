# Crate Documentation

**Version:** 1.4.1

**Format Version:** 43

# Module `pretty_assertions`

# Pretty Assertions

When writing tests in Rust, you'll probably use `assert_eq!(a, b)` _a lot_.

If such a test fails, it will present all the details of `a` and `b`.
But you have to spot the differences yourself, which is not always straightforward,
like here:

![standard assertion](https://raw.githubusercontent.com/rust-pretty-assertions/rust-pretty-assertions/2d2357ff56d22c51a86b2f1cfe6efcee9f5a8081/examples/standard_assertion.png)

Wouldn't that task be _much_ easier with a colorful diff?

![pretty assertion](https://raw.githubusercontent.com/rust-pretty-assertions/rust-pretty-assertions/2d2357ff56d22c51a86b2f1cfe6efcee9f5a8081/examples/pretty_assertion.png)

Yep — and you only need **one line of code** to make it happen:

```rust
use pretty_assertions::{assert_eq, assert_ne};
```

<details>
<summary>Show the example behind the screenshots above.</summary>

```rust,should_panic
// 1. add the `pretty_assertions` dependency to `Cargo.toml`.
// 2. insert this line at the top of each module, as needed
use pretty_assertions::{assert_eq, assert_ne};

#[derive(Debug, PartialEq)]
struct Foo {
    lorem: &'static str,
    ipsum: u32,
    dolor: Result<String, String>,
}

let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});
let y = Some(Foo { lorem: "Hello Wrold!", ipsum: 42, dolor: Ok("hey ho!".to_string())});

assert_eq!(x, y);
```
</details>

## Tip

Specify it as [`[dev-dependencies]`](http://doc.crates.io/specifying-dependencies.html#development-dependencies)
and it will only be used for compiling tests, examples, and benchmarks.
This way the compile time of `cargo build` won't be affected!

Also add `#[cfg(test)]` to your `use` statements, like this:

```rust
#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};
```

## Note

* Since `Rust 2018` edition, you need to declare
  `use pretty_assertions::{assert_eq, assert_ne};` per module.
  Before you would write `#[macro_use] extern crate pretty_assertions;`.
* The replacement is only effective in your own crate, not in other libraries
  you include.
* `assert_ne` is also switched to multi-line presentation, but does _not_ show
  a diff.

## Features

Features provided by the crate are:

- `std`: Use the Rust standard library. Enabled by default.
  Exactly one of `std` and `alloc` is required.
- `alloc`: Use the `alloc` crate.
  Exactly one of `std` and `alloc` is required.
- `unstable`: opt-in to unstable features that may not follow Semantic Versioning.
  The implementation behind this feature is subject to change without warning between patch versions.

## Types

### Struct `Comparison`

A comparison of two values.

Where both values implement `Debug`, the comparison can be displayed as a pretty diff.

```
use pretty_assertions::Comparison;

print!("{}", Comparison::new(&123, &134));
```

The values may have different types, although in practice they are usually the same.

```rust
pub struct Comparison<''a, TLeft, TRight> {
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
  pub fn new(left: &''a TLeft, right: &''a TRight) -> Comparison<''a, TLeft, TRight> { /* ... */ }
  ```
  Store two values to be compared in future.

##### Trait Implementations

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Paint**
  - ```rust
    fn fg(self: &Self, value: Color) -> Painted<&T> { /* ... */ }
    ```
    Returns a styled value derived from `self` with the foreground set to

  - ```rust
    fn primary(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn fixed(self: &Self, color: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn rgb(self: &Self, r: u8, g: u8, b: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bg(self: &Self, value: Color) -> Painted<&T> { /* ... */ }
    ```
    Returns a styled value derived from `self` with the background set to

  - ```rust
    fn on_primary(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_fixed(self: &Self, color: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_rgb(self: &Self, r: u8, g: u8, b: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn attr(self: &Self, value: Attribute) -> Painted<&T> { /* ... */ }
    ```
    Enables the styling [`Attribute`] `value`.

  - ```rust
    fn bold(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn dim(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn italic(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn underline(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn blink(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn rapid_blink(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn invert(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn conceal(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn strike(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn quirk(self: &Self, value: Quirk) -> Painted<&T> { /* ... */ }
    ```
    Enables the `yansi` [`Quirk`] `value`.

  - ```rust
    fn mask(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn wrap(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn linger(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn clear(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn resetting(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn whenever(self: &Self, value: Condition) -> Painted<&T> { /* ... */ }
    ```
    Conditionally enable styling based on whether the [`Condition`] `value`

- **Sync**
- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Struct `StrComparison`

A comparison of two strings.

In contrast to [`Comparison`], which uses the [`core::fmt::Debug`] representation,
`StrComparison` uses the string values directly, resulting in multi-line output for multiline strings.

```
use pretty_assertions::StrComparison;

print!("{}", StrComparison::new("foo\nbar", "foo\nbaz"));
```

## Value type bounds

Any value that can be referenced as a [`str`] via [`AsRef`] may be used:

```
use pretty_assertions::StrComparison;

#[derive(PartialEq)]
struct MyString(String);

impl AsRef<str> for MyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

print!(
    "{}",
    StrComparison::new(
        &MyString("foo\nbar".to_owned()),
        &MyString("foo\nbaz".to_owned()),
    ),
);
```

The values may have different types, although in practice they are usually the same.

```rust
pub struct StrComparison<''a, TLeft, TRight> {
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
  pub fn new(left: &''a TLeft, right: &''a TRight) -> StrComparison<''a, TLeft, TRight> { /* ... */ }
  ```
  Store two values to be compared in future.

##### Trait Implementations

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Paint**
  - ```rust
    fn fg(self: &Self, value: Color) -> Painted<&T> { /* ... */ }
    ```
    Returns a styled value derived from `self` with the foreground set to

  - ```rust
    fn primary(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn fixed(self: &Self, color: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn rgb(self: &Self, r: u8, g: u8, b: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bg(self: &Self, value: Color) -> Painted<&T> { /* ... */ }
    ```
    Returns a styled value derived from `self` with the background set to

  - ```rust
    fn on_primary(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_fixed(self: &Self, color: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_rgb(self: &Self, r: u8, g: u8, b: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn attr(self: &Self, value: Attribute) -> Painted<&T> { /* ... */ }
    ```
    Enables the styling [`Attribute`] `value`.

  - ```rust
    fn bold(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn dim(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn italic(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn underline(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn blink(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn rapid_blink(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn invert(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn conceal(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn strike(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn quirk(self: &Self, value: Quirk) -> Painted<&T> { /* ... */ }
    ```
    Enables the `yansi` [`Quirk`] `value`.

  - ```rust
    fn mask(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn wrap(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn linger(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn clear(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn resetting(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn whenever(self: &Self, value: Condition) -> Painted<&T> { /* ... */ }
    ```
    Conditionally enable styling based on whether the [`Condition`] `value`

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Macros

### Macro `assert_eq`

**Attributes:**

- `#[macro_export]`

Asserts that two expressions are equal to each other (using [`PartialEq`]).

On panic, this macro will print a diff derived from [`Debug`] representation of
each value.

This is a drop in replacement for [`core::assert_eq!`].
You can provide a custom panic message if desired.

# Examples

```
use pretty_assertions::assert_eq;

let a = 3;
let b = 1 + 2;
assert_eq!(a, b);

assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
```

```rust
pub macro_rules! assert_eq {
    /* macro_rules! assert_eq {
    ($left:expr, $right:expr$(,)?) => { ... };
    ($left:expr, $right:expr, $($arg:tt)*) => { ... };
    (@ $left:expr, $right:expr, $maybe_colon:expr, $($arg:tt)*) => { ... };
} */
}
```

### Macro `assert_str_eq`

**Attributes:**

- `#[macro_export]`

Asserts that two expressions are equal to each other (using [`PartialEq`]).

On panic, this macro will print a diff derived from each value's [`str`] representation.
See [`StrComparison`] for further details.

This is a drop in replacement for [`core::assert_eq!`].
You can provide a custom panic message if desired.

# Examples

```
use pretty_assertions::assert_str_eq;

let a = "foo\nbar";
let b = ["foo", "bar"].join("\n");
assert_str_eq!(a, b);

assert_str_eq!(a, b, "we are testing concatenation with {} and {}", a, b);
```

```rust
pub macro_rules! assert_str_eq {
    /* macro_rules! assert_str_eq {
    ($left:expr, $right:expr$(,)?) => { ... };
    ($left:expr, $right:expr, $($arg:tt)*) => { ... };
    (@ $left:expr, $right:expr, $maybe_colon:expr, $($arg:tt)*) => { ... };
} */
}
```

### Macro `assert_ne`

**Attributes:**

- `#[macro_export]`

Asserts that two expressions are not equal to each other (using [`PartialEq`]).

On panic, this macro will print the values of the expressions with their
[`Debug`] representations.

This is a drop in replacement for [`core::assert_ne!`].
You can provide a custom panic message if desired.

# Examples

```
use pretty_assertions::assert_ne;

let a = 3;
let b = 2;
assert_ne!(a, b);

assert_ne!(a, b, "we are testing that the values are not equal");
```

```rust
pub macro_rules! assert_ne {
    /* macro_rules! assert_ne {
    ($left:expr, $right:expr$(,)?) => { ... };
    ($left:expr, $right:expr, $($arg:tt)+) => { ... };
    (@ $left:expr, $right:expr, $maybe_colon:expr, $($arg:tt)+) => { ... };
} */
}
```

