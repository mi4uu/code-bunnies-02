# Crate Documentation

**Version:** 2.2.0

**Format Version:** 43

# Module `colored`

Coloring terminal so simple, you already know how to do it !

    use colored::Colorize;

    "this is blue".blue();
    "this is red".red();
    "this is red on blue".red().on_blue();
    "this is also red on blue".on_blue().red();
    "you can use truecolor values too!".truecolor(0, 255, 136);
    "background truecolor also works :)".on_truecolor(135, 28, 167);
    "you can also make bold comments".bold();
    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    "or change advice. This is red".yellow().blue().red();
    "or clear things up. This is default color and style".red().bold().clear();
    "purple and magenta are the same".purple().magenta();
    "bright colors are also allowed".bright_blue().on_bright_white();
    "you can specify color by string".color("blue").on_color("red");
    "and so are normal and clear".normal().clear();
    String::from("this also works!").green().bold();
    format!("{:30}", "format works as expected. This will be padded".blue());
    format!("{:.3}", "and this will be green but truncated to 3 chars".green());


 See [the `Colorize` trait](./trait.Colorize.html) for all the methods.

 Note: The methods of [`Colorize`], when used on [`str`]'s, return
 [`ColoredString`]'s. See [`ColoredString`] to learn more about them and
 what you can do with them beyond continue to use [`Colorize`] to further
 modify them.

## Modules

## Module `control`

A couple of functions to enable and disable coloring.

```rust
pub mod control { /* ... */ }
```

### Types

#### Struct `ShouldColorize`

A flag for whether coloring should occur.

```rust
pub struct ShouldColorize {
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
  pub fn from_env() -> Self { /* ... */ }
  ```
  Reads environment variables and checks if output is a tty to determine

- ```rust
  pub fn should_colorize(self: &Self) -> bool { /* ... */ }
  ```
  Returns if the current coloring is expected.

- ```rust
  pub fn set_override(self: &Self, override_colorize: bool) { /* ... */ }
  ```
  Use this to force colored to ignore the environment and always/never colorize

- ```rust
  pub fn unset_override(self: &Self) { /* ... */ }
  ```
  Remove the manual override and let the environment decide if it's ok to colorize

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> ShouldColorize { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `SHOULD_COLORIZE`

**Attributes:**

- `#[allow(missing_copy_implementations)]`
- `#[allow(non_camel_case_types)]`
- `#[allow(dead_code)]`

The persistent [`ShouldColorize`].

```rust
pub struct SHOULD_COLORIZE {
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
- **Sync**
- **LazyStatic**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &ShouldColorize { /* ... */ }
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

- **Receiver**
### Functions

#### Function `set_override`

Use this to force colored to ignore the environment and always/never colorize
See example/control.rs

```rust
pub fn set_override(override_colorize: bool) { /* ... */ }
```

#### Function `unset_override`

Remove the manual override and let the environment decide if it's ok to colorize
See example/control.rs

```rust
pub fn unset_override() { /* ... */ }
```

## Module `customcolors`

Custom colors support.

```rust
pub mod customcolors { /* ... */ }
```

### Types

#### Struct `CustomColor`

Custom color structure, it will generate a true color in the result

```rust
pub struct CustomColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `r` | `u8` | Red |
| `g` | `u8` | Green |
| `b` | `u8` | Blue |

##### Implementations

###### Methods

- ```rust
  pub fn new(r: u8, g: u8, b: u8) -> Self { /* ... */ }
  ```
  Create a new custom color

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CustomColor) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CustomColor { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Send**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Sync**
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

  - ```rust
    fn from((r, g, b): (u8, u8, u8)) -> Self { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
## Types

### Struct `ColoredString`

**Attributes:**

- `#[non_exhaustive]`

A string that may have color and/or style applied to it.

Commonly created via calling the methods of [`Colorize`] on a &str.
All methods of [`Colorize`] either create a new `ColoredString` from
the type called on or modify a callee `ColoredString`. See
[`Colorize`] for more.

The primary usage of `ColoredString`'s is as a way to take text,
apply colors and miscillaneous styling to it (such as bold or
underline), and then use it to create formatted strings that print
to the console with the special styling applied.

## Usage

As stated, `ColoredString`'s, once created, can be printed to the
console with their colors and style or turned into a string
containing special console codes that has the same effect.
This is made easy via `ColoredString`'s implementations of
[`Display`](std::fmt::Display) and [`ToString`] for those purposes
respectively.

Printing a `ColoredString` with its style is as easy as:

```
# use colored::*;
let cstring: ColoredString = "Bold and Red!".bold().red();
println!("{}", cstring);
```

## Manipulating the coloring/style of a `ColoredString`

Getting or changing the foreground color, background color, and or
style of a `ColoredString` is as easy as manually reading / modifying
the fields of `ColoredString`.

```
# use colored::*;
let mut red_text = "Red".red();
// Changing color using re-assignment and [`Colorize`]:
red_text = red_text.blue();
// Manipulating fields of `ColoredString` in-place:
red_text.fgcolor = Some(Color::Blue);

let styled_text1 = "Bold".bold();
let styled_text2 = "Italic".italic();
let mut styled_text3 = ColoredString::from("Bold and Italic");
styled_text3.style = styled_text1.style | styled_text2.style;
```

## Modifying the text of a `ColoredString`

Modifying the text is as easy as modifying the `input` field of
`ColoredString`...

```
# use colored::*;
let mut colored_text = "Magenta".magenta();
colored_text = colored_text.blue();
colored_text.input = "Blue".to_string();
// Note: The above is inefficient and `colored_text.input.replace_range(.., "Blue")` would
// be more proper. This is just for example.

assert_eq!(&*colored_text, "Blue");
```

Notice how this process preserves the coloring and style.

```rust
pub struct ColoredString {
    pub input: String,
    pub fgcolor: Option<Color>,
    pub bgcolor: Option<Color>,
    pub style: style::Style,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `input` | `String` | The plain text that will have color and style applied to it. |
| `fgcolor` | `Option<Color>` | The color of the text as it will be printed. |
| `bgcolor` | `Option<Color>` | The background color (if any). None means that the text will be printed<br>without a special background. |
| `style` | `style::Style` | Any special styling to be applied to the text (see Styles for a list of<br>available options). |

#### Implementations

##### Methods

- ```rust
  pub fn fgcolor(self: &Self) -> Option<Color> { /* ... */ }
  ```
  Get the current background color applied.

- ```rust
  pub fn bgcolor(self: &Self) -> Option<Color> { /* ... */ }
  ```
  Get the current background color applied.

- ```rust
  pub fn style(self: &Self) -> style::Style { /* ... */ }
  ```
  Get the current [`Style`] which can be check if it contains a [`Styles`].

- ```rust
  pub fn clear_fgcolor(self: &mut Self) { /* ... */ }
  ```
  Clears foreground coloring on this `ColoredString`, meaning that it

- ```rust
  pub fn clear_bgcolor(self: &mut Self) { /* ... */ }
  ```
  Gets rid of this `ColoredString`'s background.

- ```rust
  pub fn clear_style(self: &mut Self) { /* ... */ }
  ```
  Clears any special styling and sets it back to the default (plain,

- ```rust
  pub fn is_plain(self: &Self) -> bool { /* ... */ }
  ```
  Checks if the colored string has no color or styling.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ColoredString { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ColoredString { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(s: String) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: &''a str) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(cs: ColoredString) -> Box<dyn Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ColoredString) -> bool { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as Deref>::Target { /* ... */ }
    ```

- **Colorize**
  - ```rust
    fn color<S: Into<Color>>(self: Self, color: S) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn on_color<S: Into<Color>>(self: Self, color: S) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn clear(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn normal(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn bold(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn dimmed(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn italic(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn underline(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn blink(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn reverse(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn reversed(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn hidden(self: Self) -> ColoredString { /* ... */ }
    ```

  - ```rust
    fn strikethrough(self: Self) -> ColoredString { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Receiver**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
## Traits

### Trait `Colorize`

**Attributes:**

- `#[allow(missing_docs)]`

The trait that enables something to be given color.

You can use `colored` effectively simply by importing this trait
and then using its methods on `String` and `&str`.

```rust
pub trait Colorize {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `color`
- `on_color`
- `clear`
- `normal`
- `bold`
- `dimmed`
- `italic`
- `underline`
- `blink`
- `reverse`
- `reversed`
- `hidden`
- `strikethrough`

#### Provided Methods

- ```rust
  fn black(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn red(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn green(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn yellow(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn blue(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn magenta(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn purple(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn cyan(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn white(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_black(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_red(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_green(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_yellow(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_blue(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_magenta(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_purple(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_cyan(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn bright_white(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn truecolor(self: Self, r: u8, g: u8, b: u8) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn custom_color<T>(self: Self, color: T) -> ColoredString
where
    Self: Sized,
    T: Into<CustomColor> { /* ... */ }
  ```

- ```rust
  fn on_black(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_red(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_green(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_yellow(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_blue(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_magenta(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_purple(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_cyan(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_white(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_black(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_red(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_green(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_yellow(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_blue(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_magenta(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_purple(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_cyan(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_bright_white(self: Self) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_truecolor(self: Self, r: u8, g: u8, b: u8) -> ColoredString
where
    Self: Sized { /* ... */ }
  ```

- ```rust
  fn on_custom_color<T>(self: Self, color: T) -> ColoredString
where
    Self: Sized,
    T: Into<CustomColor> { /* ... */ }
  ```

#### Implementations

This trait is implemented for the following types:

- `ColoredString`
- `&str`

## Re-exports

### Re-export `CustomColor`

```rust
pub use self::customcolors::CustomColor;
```

### Re-export `Style`

```rust
pub use style::Style;
```

### Re-export `Styles`

```rust
pub use style::Styles;
```

### Re-export `color::*`

```rust
pub use color::*;
```

