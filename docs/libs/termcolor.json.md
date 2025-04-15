# Crate Documentation

**Version:** 1.4.1

**Format Version:** 43

# Module `termcolor`

This crate provides a cross platform abstraction for writing colored text to
a terminal. Colors are written using either ANSI escape sequences or by
communicating with a Windows console. Much of this API was motivated by use
inside command line applications, where colors or styles can be configured
by the end user and/or the environment.

This crate also provides platform independent support for writing colored text
to an in memory buffer. While this is easy to do with ANSI escape sequences
(because they are in the buffer themselves), it is trickier to do with the
Windows console API, which requires synchronous communication.

In ANSI mode, this crate also provides support for writing hyperlinks.

# Organization

The `WriteColor` trait extends the `io::Write` trait with methods for setting
colors or resetting them.

`StandardStream` and `StandardStreamLock` both satisfy `WriteColor` and are
analogous to `std::io::Stdout` and `std::io::StdoutLock`, or `std::io::Stderr`
and `std::io::StderrLock`.

`Buffer` is an in memory buffer that supports colored text. In a parallel
program, each thread might write to its own buffer. A buffer can be printed to
using a `BufferWriter`. The advantage of this design is that each thread can
work in parallel on a buffer without having to synchronize access to global
resources such as the Windows console. Moreover, this design also prevents
interleaving of buffer output.

`Ansi` and `NoColor` both satisfy `WriteColor` for arbitrary implementors of
`io::Write`. These types are useful when you know exactly what you need. An
analogous type for the Windows console is not provided since it cannot exist.

# Example: using `StandardStream`

The `StandardStream` type in this crate works similarly to `std::io::Stdout`,
except it is augmented with methods for coloring by the `WriteColor` trait.
For example, to write some green text:

```rust,no_run
# fn test() -> Result<(), Box<::std::error::Error>> {
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

let mut stdout = StandardStream::stdout(ColorChoice::Always);
stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
writeln!(&mut stdout, "green text!")?;
# Ok(()) }
```

Note that any text written to the terminal now will be colored
green when using ANSI escape sequences, even if it is written via
stderr, and even if stderr had previously been set to `Color::Red`.
Users will need to manage any color changes themselves by calling
[`WriteColor::set_color`](trait.WriteColor.html#tymethod.set_color), and this
may include calling [`WriteColor::reset`](trait.WriteColor.html#tymethod.reset)
before the program exits to a shell.

# Example: using `BufferWriter`

A `BufferWriter` can create buffers and write buffers to stdout or stderr. It
does *not* implement `io::Write` or `WriteColor` itself. Instead, `Buffer`
implements `io::Write` and `io::WriteColor`.

This example shows how to print some green text to stderr.

```rust,no_run
# fn test() -> Result<(), Box<::std::error::Error>> {
use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

let mut bufwtr = BufferWriter::stderr(ColorChoice::Always);
let mut buffer = bufwtr.buffer();
buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
writeln!(&mut buffer, "green text!")?;
bufwtr.print(&buffer)?;
# Ok(()) }
```

# Detecting presence of a terminal

In many scenarios when using color, one often wants to enable colors
automatically when writing to a terminal and disable colors automatically when
writing to anything else. The typical way to achieve this in Unix environments
is via libc's
[`isatty`](https://man7.org/linux/man-pages/man3/isatty.3.html)
function.
Unfortunately, this notoriously does not work well in Windows environments. To
work around that, the recommended solution is to use the standard library's
[`IsTerminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html) trait.
It goes out of its way to get it as right as possible in Windows environments.

For example, in a command line application that exposes a `--color` flag,
your logic for how to enable colors might look like this:

```ignore
use std::io::IsTerminal;

use termcolor::{ColorChoice, StandardStream};

let preference = argv.get_flag("color").unwrap_or("auto");
let mut choice = preference.parse::<ColorChoice>()?;
if choice == ColorChoice::Auto && !std::io::stdin().is_terminal() {
    choice = ColorChoice::Never;
}
let stdout = StandardStream::stdout(choice);
// ... write to stdout
```

Currently, `termcolor` does not provide anything to do this for you.

## Types

### Enum `ColorChoice`

ColorChoice represents the color preferences of an end user.

The `Default` implementation for this type will select `Auto`, which tries
to do the right thing based on the current environment.

The `FromStr` implementation for this type converts a lowercase kebab-case
string of the variant name to the corresponding variant. Any other string
results in an error.

```rust
pub enum ColorChoice {
    Always,
    AlwaysAnsi,
    Auto,
    Never,
}
```

#### Variants

##### `Always`

Try very hard to emit colors. This includes emitting ANSI colors
on Windows if the console API is unavailable.

##### `AlwaysAnsi`

AlwaysAnsi is like Always, except it never tries to use anything other
than emitting ANSI color codes.

##### `Auto`

Try to use colors, but don't force the issue. If the console isn't
available on Windows, or if TERM=dumb, or if `NO_COLOR` is defined, for
example, then don't use colors.

##### `Never`

Never emit colors.

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ColorChoice { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ColorChoice) -> bool { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<ColorChoice, ColorChoiceParseError> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Default**
  - ```rust
    fn default() -> ColorChoice { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `ColorChoiceParseError`

An error that occurs when parsing a `ColorChoice` fails.

```rust
pub struct ColorChoiceParseError {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

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

- **Unpin**
- **Error**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ColorChoiceParseError { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
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
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Struct `StandardStream`

Satisfies `io::Write` and `WriteColor`, and supports optional coloring
to either of the standard output streams, stdout and stderr.

```rust
pub struct StandardStream {
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
  pub fn stdout(choice: ColorChoice) -> StandardStream { /* ... */ }
  ```
  Create a new `StandardStream` with the given color preferences that

- ```rust
  pub fn stderr(choice: ColorChoice) -> StandardStream { /* ... */ }
  ```
  Create a new `StandardStream` with the given color preferences that

- ```rust
  pub fn lock(self: &Self) -> StandardStreamLock<''_> { /* ... */ }
  ```
  Lock the underlying writer.

##### Trait Implementations

- **Unpin**
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

- **WriteColor**
  - ```rust
    fn supports_color(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn supports_hyperlinks(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn set_color(self: &mut Self, spec: &ColorSpec) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn set_hyperlink(self: &mut Self, link: &HyperlinkSpec<''_>) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn reset(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn is_synchronous(self: &Self) -> bool { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, b: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
### Struct `StandardStreamLock`

`StandardStreamLock` is a locked reference to a `StandardStream`.

This implements the `io::Write` and `WriteColor` traits, and is constructed
via the `Write::lock` method.

The lifetime `'a` refers to the lifetime of the corresponding
`StandardStream`.

```rust
pub struct StandardStreamLock<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WriteColor**
  - ```rust
    fn supports_color(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn supports_hyperlinks(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn set_color(self: &mut Self, spec: &ColorSpec) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn set_hyperlink(self: &mut Self, link: &HyperlinkSpec<''_>) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn reset(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn is_synchronous(self: &Self) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, b: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Struct `BufferedStandardStream`

Like `StandardStream`, but does buffered writing.

```rust
pub struct BufferedStandardStream {
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
  pub fn stdout(choice: ColorChoice) -> BufferedStandardStream { /* ... */ }
  ```
  Create a new `BufferedStandardStream` with the given color preferences

- ```rust
  pub fn stderr(choice: ColorChoice) -> BufferedStandardStream { /* ... */ }
  ```
  Create a new `BufferedStandardStream` with the given color preferences

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Write**
  - ```rust
    fn write(self: &mut Self, b: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **WriteColor**
  - ```rust
    fn supports_color(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn supports_hyperlinks(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn set_color(self: &mut Self, spec: &ColorSpec) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn set_hyperlink(self: &mut Self, link: &HyperlinkSpec<''_>) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn reset(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn is_synchronous(self: &Self) -> bool { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
### Struct `BufferWriter`

Writes colored buffers to stdout or stderr.

Writable buffers can be obtained by calling `buffer` on a `BufferWriter`.

This writer works with terminals that support ANSI escape sequences or
with a Windows console.

It is intended for a `BufferWriter` to be put in an `Arc` and written to
from multiple threads simultaneously.

```rust
pub struct BufferWriter {
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
  pub fn stdout(choice: ColorChoice) -> BufferWriter { /* ... */ }
  ```
  Create a new `BufferWriter` that writes to stdout with the given

- ```rust
  pub fn stderr(choice: ColorChoice) -> BufferWriter { /* ... */ }
  ```
  Create a new `BufferWriter` that writes to stderr with the given

- ```rust
  pub fn separator(self: &mut Self, sep: Option<Vec<u8>>) { /* ... */ }
  ```
  If set, the separator given is printed between buffers. By default, no

- ```rust
  pub fn buffer(self: &Self) -> Buffer { /* ... */ }
  ```
  Creates a new `Buffer` with the current color preferences.

- ```rust
  pub fn print(self: &Self, buf: &Buffer) -> io::Result<()> { /* ... */ }
  ```
  Prints the contents of the given buffer.

##### Trait Implementations

- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
### Struct `Buffer`

Write colored text to memory.

`Buffer` is a platform independent abstraction for printing colored text to
an in memory buffer. When the buffer is printed using a `BufferWriter`, the
color information will be applied to the output device (a tty on Unix and a
console on Windows).

A `Buffer` is typically created by calling the `BufferWriter.buffer`
method, which will take color preferences and the environment into
account. However, buffers can also be manually created using `no_color`,
`ansi` or `console` (on Windows).

```rust
pub struct Buffer(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn no_color() -> Buffer { /* ... */ }
  ```
  Create a buffer that drops all color information.

- ```rust
  pub fn ansi() -> Buffer { /* ... */ }
  ```
  Create a buffer that uses ANSI escape sequences.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this buffer is empty.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the length of this buffer in bytes.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears this buffer.

- ```rust
  pub fn into_inner(self: Self) -> Vec<u8> { /* ... */ }
  ```
  Consume this buffer and return the underlying raw data.

- ```rust
  pub fn as_slice(self: &Self) -> &[u8] { /* ... */ }
  ```
  Return the underlying data of the buffer.

- ```rust
  pub fn as_mut_slice(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Return the underlying data of the buffer as a mutable slice.

##### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Buffer { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WriteColor**
  - ```rust
    fn supports_color(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn supports_hyperlinks(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn set_color(self: &mut Self, spec: &ColorSpec) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn set_hyperlink(self: &mut Self, link: &HyperlinkSpec<''_>) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn reset(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn is_synchronous(self: &Self) -> bool { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Struct `NoColor`

Satisfies `WriteColor` but ignores all color options.

```rust
pub struct NoColor<W>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new(wtr: W) -> NoColor<W> { /* ... */ }
  ```
  Create a new writer that satisfies `WriteColor` but drops all color

- ```rust
  pub fn into_inner(self: Self) -> W { /* ... */ }
  ```
  Consume this `NoColor` value and return the inner writer.

- ```rust
  pub fn get_ref(self: &Self) -> &W { /* ... */ }
  ```
  Return a reference to the inner writer.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut W { /* ... */ }
  ```
  Return a mutable reference to the inner writer.

##### Trait Implementations

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NoColor<W> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Unpin**
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

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **WriteColor**
  - ```rust
    fn supports_color(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn supports_hyperlinks(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn set_color(self: &mut Self, _: &ColorSpec) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn set_hyperlink(self: &mut Self, _: &HyperlinkSpec<''_>) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn reset(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn is_synchronous(self: &Self) -> bool { /* ... */ }
    ```

- **Freeze**
### Struct `Ansi`

Satisfies `WriteColor` using standard ANSI escape sequences.

```rust
pub struct Ansi<W>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new(wtr: W) -> Ansi<W> { /* ... */ }
  ```
  Create a new writer that satisfies `WriteColor` using standard ANSI

- ```rust
  pub fn into_inner(self: Self) -> W { /* ... */ }
  ```
  Consume this `Ansi` value and return the inner writer.

- ```rust
  pub fn get_ref(self: &Self) -> &W { /* ... */ }
  ```
  Return a reference to the inner writer.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut W { /* ... */ }
  ```
  Return a mutable reference to the inner writer.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **WriteColor**
  - ```rust
    fn supports_color(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn supports_hyperlinks(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn set_color(self: &mut Self, spec: &ColorSpec) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn set_hyperlink(self: &mut Self, link: &HyperlinkSpec<''_>) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn reset(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn is_synchronous(self: &Self) -> bool { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ansi<W> { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `ColorSpec`

A color specification.

```rust
pub struct ColorSpec {
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
  pub fn new() -> ColorSpec { /* ... */ }
  ```
  Create a new color specification that has no colors or styles.

- ```rust
  pub fn fg(self: &Self) -> Option<&Color> { /* ... */ }
  ```
  Get the foreground color.

- ```rust
  pub fn set_fg(self: &mut Self, color: Option<Color>) -> &mut ColorSpec { /* ... */ }
  ```
  Set the foreground color.

- ```rust
  pub fn bg(self: &Self) -> Option<&Color> { /* ... */ }
  ```
  Get the background color.

- ```rust
  pub fn set_bg(self: &mut Self, color: Option<Color>) -> &mut ColorSpec { /* ... */ }
  ```
  Set the background color.

- ```rust
  pub fn bold(self: &Self) -> bool { /* ... */ }
  ```
  Get whether this is bold or not.

- ```rust
  pub fn set_bold(self: &mut Self, yes: bool) -> &mut ColorSpec { /* ... */ }
  ```
  Set whether the text is bolded or not.

- ```rust
  pub fn dimmed(self: &Self) -> bool { /* ... */ }
  ```
  Get whether this is dimmed or not.

- ```rust
  pub fn set_dimmed(self: &mut Self, yes: bool) -> &mut ColorSpec { /* ... */ }
  ```
  Set whether the text is dimmed or not.

- ```rust
  pub fn italic(self: &Self) -> bool { /* ... */ }
  ```
  Get whether this is italic or not.

- ```rust
  pub fn set_italic(self: &mut Self, yes: bool) -> &mut ColorSpec { /* ... */ }
  ```
  Set whether the text is italicized or not.

- ```rust
  pub fn underline(self: &Self) -> bool { /* ... */ }
  ```
  Get whether this is underline or not.

- ```rust
  pub fn set_underline(self: &mut Self, yes: bool) -> &mut ColorSpec { /* ... */ }
  ```
  Set whether the text is underlined or not.

- ```rust
  pub fn strikethrough(self: &Self) -> bool { /* ... */ }
  ```
  Get whether this is strikethrough or not.

- ```rust
  pub fn set_strikethrough(self: &mut Self, yes: bool) -> &mut ColorSpec { /* ... */ }
  ```
  Set whether the text is strikethrough or not.

- ```rust
  pub fn reset(self: &Self) -> bool { /* ... */ }
  ```
  Get whether reset is enabled or not.

- ```rust
  pub fn set_reset(self: &mut Self, yes: bool) -> &mut ColorSpec { /* ... */ }
  ```
  Set whether to reset the terminal whenever color settings are applied.

- ```rust
  pub fn intense(self: &Self) -> bool { /* ... */ }
  ```
  Get whether this is intense or not.

- ```rust
  pub fn set_intense(self: &mut Self, yes: bool) -> &mut ColorSpec { /* ... */ }
  ```
  Set whether the text is intense or not.

- ```rust
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this color specification has no colors or styles.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears this color specification so that it has no color/style settings.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ColorSpec) -> bool { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> ColorSpec { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ColorSpec { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
### Enum `Color`

**Attributes:**

- `#[allow(missing_docs)]`

The set of available colors for the terminal foreground/background.

The `Ansi256` and `Rgb` colors will only output the correct codes when
paired with the `Ansi` `WriteColor` implementation.

The `Ansi256` and `Rgb` color types are not supported when writing colors
on Windows using the console. If they are used on Windows, then they are
silently ignored and no colors will be emitted.

This set may expand over time.

This type has a `FromStr` impl that can parse colors from their human
readable form. The format is as follows:

1. Any of the explicitly listed colors in English. They are matched
   case insensitively.
2. A single 8-bit integer, in either decimal or hexadecimal format.
3. A triple of 8-bit integers separated by a comma, where each integer is
   in decimal or hexadecimal format.

Hexadecimal numbers are written with a `0x` prefix.

```rust
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    Ansi256(u8),
    Rgb(u8, u8, u8),
    // Some variants omitted
}
```

#### Variants

##### `Black`

##### `Blue`

##### `Green`

##### `Red`

##### `Cyan`

##### `Magenta`

##### `Yellow`

##### `White`

##### `Ansi256`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### `Rgb`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |
| 1 | `u8` |  |
| 2 | `u8` |  |

*Note: Some variants have been omitted because they are private or hidden.*

#### Implementations

##### Trait Implementations

- **Sync**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Color, ParseColorError> { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Color { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Send**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Color) -> bool { /* ... */ }
    ```

### Struct `ParseColorError`

An error from parsing an invalid color specification.

```rust
pub struct ParseColorError {
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
  pub fn invalid(self: &Self) -> &str { /* ... */ }
  ```
  Return the string that couldn't be parsed as a valid color.

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParseColorError { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Sync**
- **Send**
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
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
    fn eq(self: &Self, other: &ParseColorError) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `HyperlinkSpec`

A hyperlink specification.

```rust
pub struct HyperlinkSpec<''a> {
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
  pub fn open(uri: &''a [u8]) -> HyperlinkSpec<''a> { /* ... */ }
  ```
  Creates a new hyperlink specification.

- ```rust
  pub fn close() -> HyperlinkSpec<''a> { /* ... */ }
  ```
  Creates a hyperlink specification representing no hyperlink.

- ```rust
  pub fn uri(self: &Self) -> Option<&''a [u8]> { /* ... */ }
  ```
  Returns the URI of the hyperlink if one is attached to this spec.

##### Trait Implementations

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HyperlinkSpec<''a> { /* ... */ }
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

- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
## Traits

### Trait `WriteColor`

This trait describes the behavior of writers that support colored output.

```rust
pub trait WriteColor: io::Write {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `supports_color`: Returns true if and only if the underlying writer supports colors.
- `set_color`: Set the color settings of the writer.
- `reset`: Reset the current color settings to their original settings.

#### Provided Methods

- ```rust
  fn is_synchronous(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if the underlying writer must synchronously

- ```rust
  fn set_hyperlink(self: &mut Self, _link: &HyperlinkSpec<''_>) -> io::Result<()> { /* ... */ }
  ```
  Set the current hyperlink of the writer.

- ```rust
  fn supports_hyperlinks(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if the underlying writer supports hyperlinks.

#### Implementations

This trait is implemented for the following types:

- `&''a mut T` with <''a, T: ?Sized + WriteColor>
- `Box<T>` with <T: ?Sized + WriteColor>
- `StandardStream`
- `StandardStreamLock<''a>` with <''a>
- `BufferedStandardStream`
- `Buffer`
- `NoColor<W>` with <W: io::Write>
- `Ansi<W>` with <W: io::Write>
- `io::Sink`

