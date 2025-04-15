# Crate Documentation

**Version:** 0.6.18

**Format Version:** 43

# Module `anstream`

**Auto-adapting [`stdout`] / [`stderr`] streams**

*A portmanteau of "ansi stream"*

[`AutoStream`] always accepts [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code),
[adapting to the user's terminal's capabilities][AutoStream].

Benefits
- Allows the caller to not be concerned with the terminal's capabilities
- Semver safe way of passing styled text between crates as ANSI escape codes offer more
  compatibility than most crate APIs.

Available styling crates:
- [anstyle](https://docs.rs/anstyle) for minimal runtime styling, designed to go in public APIs
- [owo-colors](https://docs.rs/owo-colors) for feature-rich runtime styling
- [color-print](https://docs.rs/color-print) for feature-rich compile-time styling

# Example

```
#  #[cfg(feature = "auto")] {
use anstream::println;
use owo_colors::OwoColorize as _;

// Foreground colors
println!("My number is {:#x}!", 10.green());
// Background colors
println!("My number is not {}!", 4.on_red());
# }
```

And this will correctly handle piping to a file, etc

## Modules

## Module `adapter`

Gracefully degrade styled output

```rust
pub mod adapter { /* ... */ }
```

### Re-exports

#### Re-export `strip_bytes`

```rust
pub use strip::strip_bytes;
```

#### Re-export `strip_str`

```rust
pub use strip::strip_str;
```

#### Re-export `StripBytes`

```rust
pub use strip::StripBytes;
```

#### Re-export `StripBytesIter`

```rust
pub use strip::StripBytesIter;
```

#### Re-export `StripStr`

```rust
pub use strip::StripStr;
```

#### Re-export `StripStrIter`

```rust
pub use strip::StripStrIter;
```

#### Re-export `StrippedBytes`

```rust
pub use strip::StrippedBytes;
```

#### Re-export `StrippedStr`

```rust
pub use strip::StrippedStr;
```

#### Re-export `WinconBytes`

```rust
pub use wincon::WinconBytes;
```

#### Re-export `WinconBytesIter`

```rust
pub use wincon::WinconBytesIter;
```

## Module `stream`

Higher-level traits to describe writeable streams

```rust
pub mod stream { /* ... */ }
```

### Traits

#### Trait `RawStream`

**Attributes:**

- `#[<cfg>(not(all(windows, feature = "wincon")))]`

Required functionality for underlying [`std::io::Write`] for adaptation

```rust
pub trait RawStream: std::io::Write + IsTerminal + private::Sealed {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `&mut T` with <T: RawStream + ?Sized>
- `Box<T>` with <T: RawStream + ?Sized>
- `std::io::Stdout`
- `std::io::StdoutLock<''_>`
- `std::io::Stderr`
- `std::io::StderrLock<''_>`
- `dyn std::io::Write`
- `dyn std::io::Write + Send`
- `dyn std::io::Write + Send + Sync`
- `Vec<u8>`
- `std::fs::File`

#### Trait `IsTerminal`

Trait to determine if a descriptor/handle refers to a terminal/tty.

```rust
pub trait IsTerminal: private::Sealed {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `is_terminal`: Returns `true` if the descriptor/handle refers to a terminal/tty.

##### Implementations

This trait is implemented for the following types:

- `&T` with <T: IsTerminal + ?Sized>
- `&mut T` with <T: IsTerminal + ?Sized>
- `Box<T>` with <T: IsTerminal + ?Sized>
- `std::io::Stdout`
- `std::io::StdoutLock<''_>`
- `std::io::Stderr`
- `std::io::StderrLock<''_>`
- `dyn std::io::Write`
- `dyn std::io::Write + Send`
- `dyn std::io::Write + Send + Sync`
- `Vec<u8>`
- `std::fs::File`

#### Trait `AsLockedWrite`

Lock a stream

```rust
pub trait AsLockedWrite: private::Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Write`: Locked writer type

###### Required Methods

- `as_locked_write`: Lock a stream

##### Implementations

This trait is implemented for the following types:

- `&mut T` with <T: AsLockedWrite + ?Sized>
- `Box<T>` with <T: AsLockedWrite + ?Sized>
- `std::io::Stdout`
- `std::io::StdoutLock<''static>`
- `std::io::Stderr`
- `std::io::StderrLock<''static>`
- `dyn std::io::Write`
- `dyn std::io::Write + Send`
- `dyn std::io::Write + Send + Sync`
- `Vec<u8>`
- `std::fs::File`

## Types

### Type Alias `Stdout`

An adaptive wrapper around the global standard output stream of the current process

```rust
pub type Stdout = AutoStream<std::io::Stdout>;
```

### Type Alias `Stderr`

An adaptive wrapper around the global standard error stream of the current process

```rust
pub type Stderr = AutoStream<std::io::Stderr>;
```

## Functions

### Function `stdout`

**Attributes:**

- `#[<cfg>(feature = "auto")]`

Create an ANSI escape code compatible stdout

**Note:** Call [`AutoStream::lock`] in loops to avoid the performance hit of acquiring/releasing
from the implicit locking in each [`std::io::Write`] call

```rust
pub fn stdout() -> Stdout { /* ... */ }
```

### Function `stderr`

**Attributes:**

- `#[<cfg>(feature = "auto")]`

Create an ANSI escape code compatible stderr

**Note:** Call [`AutoStream::lock`] in loops to avoid the performance hit of acquiring/releasing
from the implicit locking in each [`std::io::Write`] call

```rust
pub fn stderr() -> Stderr { /* ... */ }
```

## Macros

### Macro `print`

**Attributes:**

- `#[<cfg>(feature = "auto")]`
- `#[macro_export]`

Prints to [`stdout`][crate::stdout].

Equivalent to the [`println!`] macro except that a newline is not printed at
the end of the message.

Note that stdout is frequently line-buffered by default so it may be
necessary to use [`std::io::Write::flush()`] to ensure the output is emitted
immediately.

**NOTE:** The `print!` macro will lock the standard output on each call. If you call
`print!` within a hot loop, this behavior may be the bottleneck of the loop.
To avoid this, lock stdout with [`AutoStream::lock`][crate::AutoStream::lock]:
```
#  #[cfg(feature = "auto")] {
use std::io::Write as _;

let mut lock = anstream::stdout().lock();
write!(lock, "hello world").unwrap();
# }
```

Use `print!` only for the primary output of your program. Use
[`eprint!`] instead to print error and progress messages.

**NOTE:** Not all `print!` calls will be captured in tests like [`std::print!`]
- Capturing will automatically be activated in test binaries
- Otherwise, only when the `test` feature is enabled

# Panics

Panics if writing to `stdout` fails for any reason **except** broken pipe.

Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.

# Examples

```
#  #[cfg(feature = "auto")] {
use std::io::Write as _;
use anstream::print;
use anstream::stdout;

print!("this ");
print!("will ");
print!("be ");
print!("on ");
print!("the ");
print!("same ");
print!("line ");

stdout().flush().unwrap();

print!("this string has a newline, why not choose println! instead?\n");

stdout().flush().unwrap();
# }
```

```rust
pub macro_rules! print {
    /* macro_rules! print {
    ($($arg:tt)*) => { ... };
} */
}
```

### Macro `println`

**Attributes:**

- `#[<cfg>(feature = "auto")]`
- `#[macro_export]`

Prints to [`stdout`][crate::stdout], with a newline.

On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
(no additional CARRIAGE RETURN (`\r`/`U+000D`)).

This macro uses the same syntax as [`format!`], but writes to the standard output instead.
See [`std::fmt`] for more information.

**NOTE:** The `println!` macro will lock the standard output on each call. If you call
`println!` within a hot loop, this behavior may be the bottleneck of the loop.
To avoid this, lock stdout with [`AutoStream::lock`][crate::AutoStream::lock]:
```
#  #[cfg(feature = "auto")] {
use std::io::Write as _;

let mut lock = anstream::stdout().lock();
writeln!(lock, "hello world").unwrap();
# }
```

Use `println!` only for the primary output of your program. Use
[`eprintln!`] instead to print error and progress messages.

**NOTE:** Not all `println!` calls will be captured in tests like [`std::println!`]
- Capturing will automatically be activated in test binaries
- Otherwise, only when the `test` feature is enabled

# Panics

Panics if writing to `stdout` fails for any reason **except** broken pipe.

Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.

# Examples

```
#  #[cfg(feature = "auto")] {
use anstream::println;

println!(); // prints just a newline
println!("hello there!");
println!("format {} arguments", "some");
let local_variable = "some";
println!("format {local_variable} arguments");
# }
```

```rust
pub macro_rules! println {
    /* macro_rules! println {
    () => { ... };
    ($($arg:tt)*) => { ... };
} */
}
```

### Macro `eprint`

**Attributes:**

- `#[<cfg>(feature = "auto")]`
- `#[macro_export]`

Prints to [`stderr`][crate::stderr].

Equivalent to the [`print!`] macro, except that output goes to
`stderr` instead of `stdout`. See [`print!`] for
example usage.

Use `eprint!` only for error and progress messages. Use `print!`
instead for the primary output of your program.

**NOTE:** Not all `eprint!` calls will be captured in tests like [`std::eprint!`]
- Capturing will automatically be activated in test binaries
- Otherwise, only when the `test` feature is enabled

# Panics

Panics if writing to `stderr` fails for any reason **except** broken pipe.

Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.

# Examples

```
#  #[cfg(feature = "auto")] {
use anstream::eprint;

eprint!("Error: Could not complete task");
# }
```

```rust
pub macro_rules! eprint {
    /* macro_rules! eprint {
    ($($arg:tt)*) => { ... };
} */
}
```

### Macro `eprintln`

**Attributes:**

- `#[<cfg>(feature = "auto")]`
- `#[macro_export]`

Prints to [`stderr`][crate::stderr], with a newline.

Equivalent to the [`println!`] macro, except that output goes to
`stderr` instead of `stdout`. See [`println!`] for
example usage.

Use `eprintln!` only for error and progress messages. Use `println!`
instead for the primary output of your program.

**NOTE:** Not all `eprintln!` calls will be captured in tests like [`std::eprintln!`]
- Capturing will automatically be activated in test binaries
- Otherwise, only when the `test` feature is enabled

# Panics

Panics if writing to `stderr` fails for any reason **except** broken pipe.

Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.

# Examples

```
#  #[cfg(feature = "auto")] {
use anstream::eprintln;

eprintln!("Error: Could not complete task");
# }
```

```rust
pub macro_rules! eprintln {
    /* macro_rules! eprintln {
    () => { ... };
    ($($arg:tt)*) => { ... };
} */
}
```

### Macro `panic`

**Attributes:**

- `#[<cfg>(feature = "auto")]`
- `#[macro_export]`

Panics the current thread.

This allows a program to terminate immediately and provide feedback
to the caller of the program.

This macro is the perfect way to assert conditions in example code and in
tests. `panic!` is closely tied with the `unwrap` method of both
[`Option`][ounwrap] and [`Result`][runwrap] enums. Both implementations call
`panic!` when they are set to [`None`] or [`Err`] variants.

When using `panic!()` you can specify a string payload, that is built using
the [`format!`] syntax. That payload is used when injecting the panic into
the calling Rust thread, causing the thread to panic entirely.

The behavior of the default `std` hook, i.e. the code that runs directly
after the panic is invoked, is to print the message payload to
`stderr` along with the file/line/column information of the `panic!()`
call. You can override the panic hook using [`std::panic::set_hook()`].
Inside the hook a panic can be accessed as a `&dyn Any + Send`,
which contains either a `&str` or `String` for regular `panic!()` invocations.
To panic with a value of another other type, [`panic_any`] can be used.

See also the macro [`compile_error!`], for raising errors during compilation.

# When to use `panic!` vs `Result`

The Rust language provides two complementary systems for constructing /
representing, reporting, propagating, reacting to, and discarding errors. These
responsibilities are collectively known as "error handling." `panic!` and
`Result` are similar in that they are each the primary interface of their
respective error handling systems; however, the meaning these interfaces attach
to their errors and the responsibilities they fulfill within their respective
error handling systems differ.

The `panic!` macro is used to construct errors that represent a bug that has
been detected in your program. With `panic!` you provide a message that
describes the bug and the language then constructs an error with that message,
reports it, and propagates it for you.

`Result` on the other hand is used to wrap other types that represent either
the successful result of some computation, `Ok(T)`, or error types that
represent an anticipated runtime failure mode of that computation, `Err(E)`.
`Result` is used alongside user defined types which represent the various
anticipated runtime failure modes that the associated computation could
encounter. `Result` must be propagated manually, often with the the help of the
`?` operator and `Try` trait, and they must be reported manually, often with
the help of the `Error` trait.

For more detailed information about error handling check out the [book] or the
[`std::result`] module docs.

[ounwrap]: Option::unwrap
[runwrap]: Result::unwrap
[`std::panic::set_hook()`]: ../std/panic/fn.set_hook.html
[`panic_any`]: ../std/panic/fn.panic_any.html
[`Box`]: ../std/boxed/struct.Box.html
[`Any`]: crate::any::Any
[`format!`]: ../std/macro.format.html
[book]: ../book/ch09-00-error-handling.html
[`std::result`]: ../std/result/index.html

# Current implementation

If the main thread panics it will terminate all your threads and end your
program with code `101`.

# Examples

```should_panic
# #![allow(unreachable_code)]
use anstream::panic;
panic!();
panic!("this is a terrible mistake!");
panic!("this is a {} {message}", "fancy", message = "message");
```

```rust
pub macro_rules! panic {
    /* macro_rules! panic {
    () => { ... };
    ($($arg:tt)*) => { ... };
} */
}
```

## Re-exports

### Re-export `AutoStream`

```rust
pub use auto::AutoStream;
```

### Re-export `StripStream`

```rust
pub use strip::StripStream;
```

### Re-export `ColorChoice`

Selection for overriding color output

```rust
pub use colorchoice::ColorChoice;
```

