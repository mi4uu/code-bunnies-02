# Crate Documentation

**Version:** 0.46.0

**Format Version:** 43

# Module `nu_ansi_term`

This is a library for controlling colors and formatting, such as
red bold text or blue underlined text, on ANSI terminals.


## Basic usage

There are three main types in this crate that you need to be
concerned with: [`AnsiString`], [`Style`], and [`Color`].

A `Style` holds stylistic information: foreground and background colors,
whether the text should be bold, or blinking, or other properties. The
[`Color`] enum represents the available colors. And an [`AnsiString`] is a
string paired with a [`Style`].

[`Color`] is also available as an alias to `Color`.

To format a string, call the `paint` method on a `Style` or a `Color`,
passing in the string you want to format as the argument. For example,
here’s how to get some red text:

```
use nu_ansi_term::Color::Red;

println!("This is in red: {}", Red.paint("a red string"));
```

It’s important to note that the `paint` method does *not* actually return a
string with the ANSI control characters surrounding it. Instead, it returns
an [`AnsiString`] value that has a [`Display`] implementation that, when
formatted, returns the characters. This allows strings to be printed with a
minimum of [`String`] allocations being performed behind the scenes.

If you *do* want to get at the escape codes, then you can convert the
[`AnsiString`] to a string as you would any other `Display` value:

```
use nu_ansi_term::Color::Red;

let red_string = Red.paint("a red string").to_string();
```


## Bold, underline, background, and other styles

For anything more complex than plain foreground color changes, you need to
construct `Style` values themselves, rather than beginning with a `Color`.
You can do this by chaining methods based on a new `Style`, created with
[`Style::new()`]. Each method creates a new style that has that specific
property set. For example:

```
use nu_ansi_term::Style;

println!("How about some {} and {}?",
         Style::new().bold().paint("bold"),
         Style::new().underline().paint("underline"));
```

For brevity, these methods have also been implemented for `Color` values,
so you can give your styles a foreground color without having to begin with
an empty `Style` value:

```
use nu_ansi_term::Color::{Blue, Yellow};

println!("Demonstrating {} and {}!",
         Blue.bold().paint("blue bold"),
         Yellow.underline().paint("yellow underline"));

println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
```

The complete list of styles you can use are: [`bold`], [`dimmed`], [`italic`],
[`underline`], [`blink`], [`reverse`], [`hidden`], [`strikethrough`], and [`on`] for
background colors.

In some cases, you may find it easier to change the foreground on an
existing `Style` rather than starting from the appropriate `Color`.
You can do this using the [`fg`] method:

```
use nu_ansi_term::Style;
use nu_ansi_term::Color::{Blue, Cyan, Yellow};

println!("Yellow on blue: {}", Style::new().on(Blue).fg(Yellow).paint("yow!"));
println!("Also yellow on blue: {}", Cyan.on(Blue).fg(Yellow).paint("zow!"));
```

You can turn a `Color` into a `Style` with the [`normal`] method.
This will produce the exact same `AnsiString` as if you just used the
`paint` method on the `Color` directly, but it’s useful in certain cases:
for example, you may have a method that returns `Styles`, and need to
represent both the “red bold” and “red, but not bold” styles with values of
the same type. The `Style` struct also has a [`Default`] implementation if you
want to have a style with *nothing* set.

```
use nu_ansi_term::Style;
use nu_ansi_term::Color::Red;

Red.normal().paint("yet another red string");
Style::default().paint("a completely regular string");
```


## Extended colors

You can access the extended range of 256 colors by using the `Color::Fixed`
variant, which takes an argument of the color number to use. This can be
included wherever you would use a `Color`:

```
use nu_ansi_term::Color::Fixed;

Fixed(134).paint("A sort of light purple");
Fixed(221).on(Fixed(124)).paint("Mustard in the ketchup");
```

The first sixteen of these values are the same as the normal and bold
standard color variants. There’s nothing stopping you from using these as
`Fixed` colors instead, but there’s nothing to be gained by doing so
either.

You can also access full 24-bit color by using the `Color::Rgb` variant,
which takes separate `u8` arguments for red, green, and blue:

```
use nu_ansi_term::Color::Rgb;

Rgb(70, 130, 180).paint("Steel blue");
```

## Combining successive colored strings

The benefit of writing ANSI escape codes to the terminal is that they
*stack*: you do not need to end every colored string with a reset code if
the text that follows it is of a similar style. For example, if you want to
have some blue text followed by some blue bold text, it’s possible to send
the ANSI code for blue, followed by the ANSI code for bold, and finishing
with a reset code without having to have an extra one between the two
strings.

This crate can optimise the ANSI codes that get printed in situations like
this, making life easier for your terminal renderer. The [`AnsiStrings`]
type takes a slice of several [`AnsiString`] values, and will iterate over
each of them, printing only the codes for the styles that need to be updated
as part of its formatting routine.

The following code snippet uses this to enclose a binary number displayed in
red bold text inside some red, but not bold, brackets:

```
use nu_ansi_term::Color::Red;
use nu_ansi_term::{AnsiString, AnsiStrings};

let some_value = format!("{:b}", 42);
let strings: &[AnsiString<'static>] = &[
    Red.paint("["),
    Red.bold().paint(some_value),
    Red.paint("]"),
];

println!("Value: {}", AnsiStrings(strings));
```

There are several things to note here. Firstly, the [`paint`] method can take
*either* an owned [`String`] or a borrowed [`&str`]. Internally, an [`AnsiString`]
holds a copy-on-write ([`Cow`]) string value to deal with both owned and
borrowed strings at the same time. This is used here to display a `String`,
the result of the `format!` call, using the same mechanism as some
statically-available `&str` slices. Secondly, that the [`AnsiStrings`] value
works in the same way as its singular counterpart, with a [`Display`]
implementation that only performs the formatting when required.

## Byte strings

This library also supports formatting `\[u8]` byte strings; this supports
applications working with text in an unknown encoding.  [`Style`] and
[`Color`] support painting `\[u8]` values, resulting in an [`AnsiByteString`].
This type does not implement [`Display`], as it may not contain UTF-8, but
it does provide a method [`write_to`] to write the result to any value that
implements [`Write`]:

```
use nu_ansi_term::Color::Green;

Green.paint("user data".as_bytes()).write_to(&mut std::io::stdout()).unwrap();
```

Similarly, the type [`AnsiByteStrings`] supports writing a list of
[`AnsiByteString`] values with minimal escape sequences:

```
use nu_ansi_term::Color::Green;
use nu_ansi_term::AnsiByteStrings;

AnsiByteStrings(&[
    Green.paint("user data 1\n".as_bytes()),
    Green.bold().paint("user data 2\n".as_bytes()),
]).write_to(&mut std::io::stdout()).unwrap();
```

[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`&str`]: https://doc.rust-lang.org/std/primitive.str.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`Style`]: struct.Style.html
[`Style::new()`]: struct.Style.html#method.new
[`Color`]: enum.Color.html
[`Color`]: enum.Color.html
[`AnsiString`]: type.AnsiString.html
[`AnsiStrings`]: type.AnsiStrings.html
[`AnsiByteString`]: type.AnsiByteString.html
[`AnsiByteStrings`]: type.AnsiByteStrings.html
[`write_to`]: type.AnsiByteString.html#method.write_to
[`paint`]: type.AnsiByteString.html#method.write_to
[`normal`]: enum.Color.html#method.normal

[`bold`]: struct.Style.html#method.bold
[`dimmed`]: struct.Style.html#method.dimmed
[`italic`]: struct.Style.html#method.italic
[`underline`]: struct.Style.html#method.underline
[`blink`]: struct.Style.html#method.blink
[`reverse`]: struct.Style.html#method.reverse
[`hidden`]: struct.Style.html#method.hidden
[`strikethrough`]: struct.Style.html#method.strikethrough
[`fg`]: struct.Style.html#method.fg
[`on`]: struct.Style.html#method.on

## Modules

## Module `ansi`

**Attributes:**

- `#[allow(missing_docs)]`

```rust
pub mod ansi { /* ... */ }
```

### Types

#### Struct `Prefix`

Like `AnsiString`, but only displays the style prefix.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
[`Style::prefix`](struct.Style.html#method.prefix).

```rust
pub struct Prefix(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

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

- **RefUnwindSafe**
- **UnwindSafe**
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
    fn clone(self: &Self) -> Prefix { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Freeze**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Infix`

Like `AnsiString`, but only displays the difference between two
styles.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
[`Style::infix`](struct.Style.html#method.infix).

```rust
pub struct Infix(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Infix { /* ... */ }
    ```

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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `Suffix`

Like `AnsiString`, but only displays the style suffix.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
[`Style::suffix`](struct.Style.html#method.suffix).

```rust
pub struct Suffix(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Suffix { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

### Constants and Statics

#### Static `RESET`

The code to send to reset all styles and return to `Style::default()`.

```rust
pub static RESET: &str = "\x1B[0m";
```

## Module `gradient`

```rust
pub mod gradient { /* ... */ }
```

### Types

#### Struct `Gradient`

Linear color gradient between two color stops

```rust
pub struct Gradient {
    pub start: crate::rgb::Rgb,
    pub end: crate::rgb::Rgb,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `crate::rgb::Rgb` | Start Color of Gradient |
| `end` | `crate::rgb::Rgb` | End Color of Gradient |

##### Implementations

###### Methods

- ```rust
  pub const fn new(start: Rgb, end: Rgb) -> Self { /* ... */ }
  ```
  Creates a new [Gradient] with two [Rgb] colors, `start` and `end`

- ```rust
  pub const fn from_color_rgb(start: Color, end: Color) -> Self { /* ... */ }
  ```

- ```rust
  pub fn at(self: &Self, t: f32) -> Rgb { /* ... */ }
  ```
  Computes the [Rgb] color between `start` and `end` for `t`

- ```rust
  pub const fn reverse(self: &Self) -> Self { /* ... */ }
  ```
  Returns the reverse of `self`

- ```rust
  pub fn build(self: &Self, text: &str, target: TargetGround) -> String { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **StructuralPartialEq**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Gradient { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Gradient) -> bool { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Copy**
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

#### Enum `TargetGround`

```rust
pub enum TargetGround {
    Foreground,
    Background,
}
```

##### Variants

###### `Foreground`

###### `Background`

##### Implementations

###### Methods

- ```rust
  pub const fn code(self: &Self) -> u8 { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **Eq**
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
    fn clone(self: &Self) -> TargetGround { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TargetGround) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Sync**
### Traits

#### Trait `ANSIColorCode`

```rust
pub trait ANSIColorCode {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `ansi_color_code`

##### Implementations

This trait is implemented for the following types:

- `Rgb`

### Functions

#### Function `build_all_gradient_text`

**Attributes:**

- `#[allow(dead_code)]`

```rust
pub fn build_all_gradient_text(text: &str, foreground: Gradient, background: Gradient) -> String { /* ... */ }
```

## Re-exports

### Re-export `Infix`

```rust
pub use ansi::Infix;
```

### Re-export `Prefix`

```rust
pub use ansi::Prefix;
```

### Re-export `Suffix`

```rust
pub use ansi::Suffix;
```

### Re-export `Color`

```rust
pub use style::Color;
```

### Re-export `Style`

```rust
pub use style::Style;
```

### Re-export `display::*`

```rust
pub use display::*;
```

### Re-export `windows::*`

```rust
pub use windows::*;
```

### Re-export `util::*`

```rust
pub use util::*;
```

### Re-export `gradient::*`

```rust
pub use gradient::*;
```

### Re-export `rgb::*`

```rust
pub use rgb::*;
```

