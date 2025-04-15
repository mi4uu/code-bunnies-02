# Crate Documentation

**Version:** 0.6.12

**Format Version:** 43

# Module `eyre`

This library provides [`eyre::Report`][Report], a trait object based
error handling type for easy idiomatic error handling and reporting in Rust
applications.

This crate is a fork of [`anyhow`]  with support for customized
error reports. For more details on customization, check out the docs on
[`eyre::EyreHandler`].

## Custom Report Handlers

The heart of this crate is its ability to swap out the Handler type to change
what information is carried alongside errors and how the end report is
formatted. This crate is meant to be used alongside companion crates that
customize its behavior. Below is a list of known crates that export report
handlers for eyre and short summaries of what features they provide.

- [`stable-eyre`]: Switches the backtrace type from `std`'s to `backtrace-rs`'s
  so that it can be captured on stable. The report format is identical to
  `DefaultHandler`'s report format.
- [`color-eyre`]: Captures a `backtrace::Backtrace` and a
  `tracing_error::SpanTrace`. Provides a `Section` trait for attaching warnings
  and suggestions to error reports. The end report is then pretty printed with
  the help of [`color-backtrace`], [`color-spantrace`], and `ansi_term`. Check
  out the README on [`color-eyre`] for details on the report format.
- [`simple-eyre`]: A minimal `EyreHandler` that captures no additional
  information, for when you do not wish to capture `Backtrace`s with errors.
- [`jane-eyre`]: A report handler crate that exists purely for the pun.
  Currently just re-exports `color-eyre`.

## Usage Recommendations and Stability Considerations

**We recommend users do not re-export types from this library as part their
own public API for libraries with external users.** The main reason for this
is that it will make your library API break if we ever bump the major version
number on eyre and your users upgrade the eyre version they use in their
application code before you upgrade your own eyre dep version[^1].

However, even beyond this API stability hazard, there are other good reasons
to avoid using `eyre::Report` as your public error type.

- You export an undocumented error interface that is otherwise still
accessible via downcast, making it hard for users to react to specific
errors while not preventing them from depending on details you didn't mean
to make part of your public API.
  - This in turn makes the error types of all libraries you use a part of
  your public API as well, and makes changing any of those libraries into
  undetectable runtime breakage.
- If many of your errors are constructed from strings, you encourage your
users to use string comparison for reacting to specific errors, which is
brittle and turns updating error messages into potentially undetectable
runtime breakage.

## Details

- Use `Result<T, eyre::Report>`, or equivalently `eyre::Result<T>`, as the
  return type of any fallible function.

  Within the function, use `?` to easily propagate any error that implements the
  `std::error::Error` trait.

  ```rust
  # pub trait Deserialize {}
  #
  # mod serde_json {
  #     use super::Deserialize;
  #     use std::io;
  #
  #     pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
  #         unimplemented!()
  #     }
  # }
  #
  # struct ClusterMap;
  #
  # impl Deserialize for ClusterMap {}
  #
  use eyre::Result;

  fn get_cluster_info() -> Result<ClusterMap> {
      let config = std::fs::read_to_string("cluster.json")?;
      let map: ClusterMap = serde_json::from_str(&config)?;
      Ok(map)
  }
  #
  # fn main() {}
  ```

- Wrap a lower level error with a new error created from a message to help the
  person troubleshooting understand the chain of failures that occurred. A
  low-level error like "No such file or directory" can be annoying to debug
  without more information about what higher level step the application was in
  the middle of.

  ```rust
  # struct It;
  #
  # impl It {
  #     fn detach(&self) -> Result<()> {
  #         unimplemented!()
  #     }
  # }
  #
  use eyre::{WrapErr, Result};

  fn main() -> Result<()> {
      # return Ok(());
      #
      # const _: &str = stringify! {
      ...
      # };
      #
      # let it = It;
      # let path = "./path/to/instrs.json";
      #
      it.detach().wrap_err("Failed to detach the important thing")?;

      let content = std::fs::read(path)
          .wrap_err_with(|| format!("Failed to read instrs from {}", path))?;
      #
      # const _: &str = stringify! {
      ...
      # };
      #
      # Ok(())
  }
  ```

  ```console
  Error: Failed to read instrs from ./path/to/instrs.json

  Caused by:
      No such file or directory (os error 2)
  ```

- Downcasting is supported and can be done by value, by shared reference, or by
  mutable reference as needed.

  ```rust
  # use eyre::{Report, eyre};
  # use std::fmt::{self, Display};
  # use std::task::Poll;
  #
  # #[derive(Debug)]
  # enum DataStoreError {
  #     Censored(()),
  # }
  #
  # impl Display for DataStoreError {
  #     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
  #         unimplemented!()
  #     }
  # }
  #
  # impl std::error::Error for DataStoreError {}
  #
  # const REDACTED_CONTENT: () = ();
  #
  # #[cfg(not(feature = "auto-install"))]
  # eyre::set_hook(Box::new(eyre::DefaultHandler::default_with)).unwrap();
  #
  # let error: Report = eyre!("...");
  # let root_cause = &error;
  #
  # let ret =
  // If the error was caused by redaction, then return a
  // tombstone instead of the content.
  match root_cause.downcast_ref::<DataStoreError>() {
      Some(DataStoreError::Censored(_)) => Ok(Poll::Ready(REDACTED_CONTENT)),
      None => Err(error),
  }
  # ;
  ```

- If using the nightly channel, a backtrace is captured and printed with the
  error if the underlying error type does not already provide its own. In order
  to see backtraces, they must be enabled through the environment variables
  described in [`std::backtrace`]:

  - If you want panics and errors to both have backtraces, set
    `RUST_BACKTRACE=1`;
  - If you want only errors to have backtraces, set `RUST_LIB_BACKTRACE=1`;
  - If you want only panics to have backtraces, set `RUST_BACKTRACE=1` and
    `RUST_LIB_BACKTRACE=0`.

  The tracking issue for this feature is [rust-lang/rust#53487].

  [`std::backtrace`]: https://doc.rust-lang.org/std/backtrace/index.html#environment-variables
  [rust-lang/rust#53487]: https://github.com/rust-lang/rust/issues/53487

- Eyre works with any error type that has an impl of `std::error::Error`,
  including ones defined in your crate. We do not bundle a `derive(Error)` macro
  but you can write the impls yourself or use a standalone macro like
  [thiserror].

  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum FormatError {
      #[error("Invalid header (expected {expected:?}, got {found:?})")]
      InvalidHeader {
          expected: String,
          found: String,
      },
      #[error("Missing attribute: {0}")]
      MissingAttribute(String),
  }
  ```

- One-off error messages can be constructed using the `eyre!` macro, which
  supports string interpolation and produces an `eyre::Report`.

  ```rust
  # use eyre::{eyre, Result};
  #
  # fn demo() -> Result<()> {
  #     let missing = "...";
  return Err(eyre!("Missing attribute: {}", missing));
  #     Ok(())
  # }
  ```

- On newer versions of the compiler (i.e. 1.58 and later) this macro also
  supports format args captures.

  ```rust
  # use eyre::{eyre, Result};
  #
  # fn demo() -> Result<()> {
  #     let missing = "...";
  # #[cfg(not(eyre_no_fmt_args_capture))]
  return Err(eyre!("Missing attribute: {missing}"));
  #     Ok(())
  # }
  ```

## No-std support

No-std support was removed in 2020 in [commit 608a16a] due to unaddressed upstream breakages.

[commit 608a16a]: https://github.com/eyre-rs/eyre/pull/29/commits/608a16aa2c2c27eca6c88001cc94c6973c18f1d5

## Comparison to failure

The `eyre::Report` type works something like `failure::Error`, but unlike
failure ours is built around the standard library's `std::error::Error` trait
rather than a separate trait `failure::Fail`. The standard library has adopted
the necessary improvements for this to be possible as part of [RFC 2504].

[RFC 2504]: https://github.com/rust-lang/rfcs/blob/master/text/2504-fix-error.md

## Comparison to thiserror

Use `eyre` if you don't think you'll do anything with an error other than
report it. This is common in application code. Use `thiserror` if you think
you need an error type that can be handled via match or reported. This is
common in library crates where you don't know how your users will handle
your errors.

[thiserror]: https://github.com/dtolnay/thiserror

## Compatibility with `anyhow`

This crate does its best to be usable as a drop in replacement of `anyhow` and
vice-versa by re-exporting all of the renamed APIs with the names used in
`anyhow`, though there are some differences still.

#### `Context` and `Option`

As part of renaming `Context` to `WrapErr` we also intentionally do not
implement `WrapErr` for `Option`. This decision was made because `wrap_err`
implies that you're creating a new error that saves the old error as its
`source`. With `Option` there is no source error to wrap, so `wrap_err` ends up
being somewhat meaningless.

Instead `eyre` offers [`OptionExt::ok_or_eyre`] to yield _static_ errors from `None`,
and intends for users to use the combinator functions provided by
`std`, converting `Option`s to `Result`s, for _dynamic_ errors.
So where you would write this with
anyhow:

```rust
use anyhow::Context;

let opt: Option<()> = None;
let result_static = opt.context("static error message");
let result_dynamic = opt.with_context(|| format!("{} error message", "dynamic"));
```

With `eyre` we want users to write:

```rust
use eyre::{eyre, OptionExt, Result};

# #[cfg(not(feature = "auto-install"))]
# eyre::set_hook(Box::new(eyre::DefaultHandler::default_with)).unwrap();
#
let opt: Option<()> = None;
let result_static: Result<()> = opt.ok_or_eyre("static error message");
let result_dynamic: Result<()> = opt.ok_or_else(|| eyre!("{} error message", "dynamic"));
```

**NOTE**: However, to help with porting we do provide a `ContextCompat` trait which
implements `context` for options which you can import to make existing
`.context` calls compile.

[^1]: example and explanation of breakage <https://github.com/eyre-rs/eyre/issues/30#issuecomment-647650361>

[Report]: https://docs.rs/eyre/*/eyre/struct.Report.html
[`eyre::EyreHandler`]: https://docs.rs/eyre/*/eyre/trait.EyreHandler.html
[`eyre::WrapErr`]: https://docs.rs/eyre/*/eyre/trait.WrapErr.html
[`anyhow::Context`]: https://docs.rs/anyhow/*/anyhow/trait.Context.html
[`anyhow`]: https://github.com/dtolnay/anyhow
[`tracing_error::SpanTrace`]: https://docs.rs/tracing-error/*/tracing_error/struct.SpanTrace.html
[`stable-eyre`]: https://github.com/eyre-rs/stable-eyre
[`color-eyre`]: https://github.com/eyre-rs/color-eyre
[`jane-eyre`]: https://github.com/yaahc/jane-eyre
[`simple-eyre`]: https://github.com/eyre-rs/simple-eyre
[`color-spantrace`]: https://github.com/eyre-rs/color-spantrace
[`color-backtrace`]: https://github.com/athre0z/color-backtrace

## Types

### Struct `Report`

**Attributes:**

- `#[must_use]`

The core error reporting type of the library, a wrapper around a dynamic error reporting type.

`Report` works a lot like `Box<dyn std::error::Error>`, but with these
differences:

- `Report` requires that the error is `Send`, `Sync`, and `'static`.
- `Report` guarantees that a backtrace is available, even if the underlying
  error type does not provide one.
- `Report` is represented as a narrow pointer &mdash; exactly one word in
  size instead of two.

# Display representations

When you print an error object using "{}" or to_string(), only the outermost underlying error
is printed, not any of the lower level causes. This is exactly as if you had called the Display
impl of the error from which you constructed your eyre::Report.

```console
Failed to read instrs from ./path/to/instrs.json
```

To print causes as well using eyre's default formatting of causes, use the
alternate selector "{:#}".

```console
Failed to read instrs from ./path/to/instrs.json: No such file or directory (os error 2)
```

The Debug format "{:?}" includes your backtrace if one was captured. Note
that this is the representation you get by default if you return an error
from `fn main` instead of printing it explicitly yourself.

```console
Error: Failed to read instrs from ./path/to/instrs.json

Caused by:
    No such file or directory (os error 2)

Stack backtrace:
   0: <E as eyre::context::ext::StdError>::ext_report
             at /git/eyre/src/backtrace.rs:26
   1: core::result::Result<T,E>::map_err
             at /git/rustc/src/libcore/result.rs:596
   2: eyre::context::<impl eyre::WrapErr<T,E,H> for core::result::Result<T,E>>::wrap_err_with
             at /git/eyre/src/context.rs:58
   3: testing::main
             at src/main.rs:5
   4: std::rt::lang_start
             at /git/rustc/src/libstd/rt.rs:61
   5: main
   6: __libc_start_main
   7: _start
```

To see a conventional struct-style Debug representation, use "{:#?}".

```console
Error {
    msg: "Failed to read instrs from ./path/to/instrs.json",
    source: Os {
        code: 2,
        kind: NotFound,
        message: "No such file or directory",
    },
}
```

If none of the built-in representations are appropriate and you would prefer
to render the error and its cause chain yourself, it can be done by defining
your own [`EyreHandler`] and [`hook`] to use it.

[`EyreHandler`]: trait.EyreHandler.html
[`hook`]: fn.set_hook.html

```rust
pub struct Report {
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
  pub fn new<E>(error: E) -> Self
where
    E: StdError + Send + Sync + ''static { /* ... */ }
  ```
  Create a new error object from any error type.

- ```rust
  pub fn msg<M>(message: M) -> Self
where
    M: Display + Debug + Send + Sync + ''static { /* ... */ }
  ```
  Create a new error object from a printable error message.

- ```rust
  pub fn wrap_err<D>(self: Self, msg: D) -> Self
where
    D: Display + Send + Sync + ''static { /* ... */ }
  ```
  Create a new error from an error message to wrap the existing error.

- ```rust
  pub fn chain(self: &Self) -> Chain<''_> { /* ... */ }
  ```
  An iterator of the chain of source errors contained by this Report.

- ```rust
  pub fn root_cause(self: &Self) -> &dyn StdError + ''static { /* ... */ }
  ```
  The lowest level cause of this error &mdash; this error's cause's

- ```rust
  pub fn is<E>(self: &Self) -> bool
where
    E: Display + Debug + Send + Sync + ''static { /* ... */ }
  ```
  Returns true if `E` is the type held by this error object.

- ```rust
  pub fn downcast<E>(self: Self) -> Result<E, Self>
where
    E: Display + Debug + Send + Sync + ''static { /* ... */ }
  ```
  Attempt to downcast the error object to a concrete type.

- ```rust
  pub fn downcast_ref<E>(self: &Self) -> Option<&E>
where
    E: Display + Debug + Send + Sync + ''static { /* ... */ }
  ```
  Downcast this error object by reference.

- ```rust
  pub fn downcast_mut<E>(self: &mut Self) -> Option<&mut E>
where
    E: Display + Debug + Send + Sync + ''static { /* ... */ }
  ```
  Downcast this error object by mutable reference.

- ```rust
  pub fn handler(self: &Self) -> &dyn EyreHandler { /* ... */ }
  ```
  Get a reference to the Handler for this Report.

- ```rust
  pub fn handler_mut(self: &mut Self) -> &mut dyn EyreHandler { /* ... */ }
  ```
  Get a mutable reference to the Handler for this Report.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: never) -> T { /* ... */ }
    ```

  - ```rust
    fn from(error: E) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(error: Report) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(error: Report) -> Self { /* ... */ }
    ```

- **Receiver**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &dyn StdError + Send + Sync + ''static { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &dyn StdError + ''static { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TraitKind**
- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `InstallError`

Error indicating that `set_hook` was unable to install the provided ErrorHook

```rust
pub struct InstallError;
```

#### Implementations

##### Trait Implementations

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **UnwindSafe**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InstallError { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TraitKind**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Error**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `DefaultHandler`

**Attributes:**

- `#[allow(dead_code)]`

The default provided error report handler for `eyre::Report`.

On nightly this supports conditionally capturing a `std::backtrace::Backtrace` if the source
error did not already capture one.

```rust
pub struct DefaultHandler {
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
  pub fn default_with(error: &dyn StdError + ''static) -> Box<dyn EyreHandler> { /* ... */ }
  ```
  Manual hook which constructs `DefaultHandler`s.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **EyreHandler**
  - ```rust
    fn debug(self: &Self, error: &dyn StdError + ''static, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

  - ```rust
    fn track_caller(self: &mut Self, location: &''static std::panic::Location<''static>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Sync**
### Struct `Chain`

**Attributes:**

- `#[allow(missing_debug_implementations)]`

Iterator of a chain of source errors.

This type is the iterator returned by [`Report::chain`].

# Example

```
use eyre::Report;
use std::io;

pub fn underlying_io_error_kind(error: &Report) -> Option<io::ErrorKind> {
    for cause in error.chain() {
        if let Some(io_error) = cause.downcast_ref::<io::Error>() {
            return Some(io_error.kind());
        }
    }
    None
}
```

```rust
pub struct Chain<''a> {
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
  pub fn new(head: &''a dyn StdError + ''static) -> Self { /* ... */ }
  ```
  Construct an iterator over a chain of errors via the `source` method

##### Trait Implementations

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Chain<''a> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

### Type Alias `Result`

type alias for `Result<T, Report>`

This is a reasonable return type to use throughout your application but also for `fn main`; if
you do, failures will be printed along with a backtrace if one was captured.

`eyre::Result` may be used with one *or* two type parameters.

```rust
use eyre::Result;

# const IGNORE: &str = stringify! {
fn demo1() -> Result<T> {...}
           // ^ equivalent to std::result::Result<T, eyre::Report>

fn demo2() -> Result<T, OtherError> {...}
           // ^ equivalent to std::result::Result<T, OtherError>
# };
```

# Example

```
# pub trait Deserialize {}
#
# mod serde_json {
#     use super::Deserialize;
#     use std::io;
#
#     pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
#         unimplemented!()
#     }
# }
#
# #[derive(Debug)]
# struct ClusterMap;
#
# impl Deserialize for ClusterMap {}
#
use eyre::Result;

fn main() -> Result<()> {
    # return Ok(());
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    println!("cluster info: {:#?}", map);
    Ok(())
}
```

```rust
pub type Result<T, E = Report> = core::result::Result<T, E>;
```

## Traits

### Trait `EyreHandler`

Error Report Handler trait for customizing `eyre::Report`

```rust
pub trait EyreHandler: core::any::Any + Send + Sync {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `debug`: Define the report format

#### Provided Methods

- ```rust
  fn display(self: &Self, error: &dyn StdError + ''static, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
  ```
  Override for the `Display` format

- ```rust
  fn track_caller(self: &mut Self, location: &''static std::panic::Location<''static>) { /* ... */ }
  ```
  Store the location of the caller who constructed this error report

#### Implementations

This trait is implemented for the following types:

- `DefaultHandler`

### Trait `WrapErr`

Provides the `wrap_err` method for `Result`.

This trait is sealed and cannot be implemented for types outside of
`eyre`.

# Example

```
use eyre::{WrapErr, Result};
use std::fs;
use std::path::PathBuf;

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    # const IGNORE: &'static str = stringify! {
    pub fn detach(&mut self) -> Result<()> {...}
    # };
    # fn detach(&mut self) -> Result<()> {
    #     unimplemented!()
    # }
}

pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
    it.detach().wrap_err("Failed to detach the important thing")?;

    let path = &it.path;
    let content = fs::read(path)
        .wrap_err_with(|| format!("Failed to read instrs from {}", path.display()))?;

    Ok(content)
}
```

When printed, the outermost error would be printed first and the lower
level underlying causes would be enumerated below.

```console
Error: Failed to read instrs from ./path/to/instrs.json

Caused by:
    No such file or directory (os error 2)
```

# Wrapping Types That Don't impl `Error` (e.g. `&str` and `Box<dyn Error>`)

Due to restrictions for coherence `Report` cannot impl `From` for types that don't impl
`Error`. Attempts to do so will give "this type might implement Error in the future" as an
error. As such, `wrap_err`, which uses `From` under the hood, cannot be used to wrap these
types. Instead we encourage you to use the combinators provided for `Result` in `std`/`core`.

For example, instead of this:

```rust,compile_fail
use std::error::Error;
use eyre::{WrapErr, Report};

fn wrap_example(err: Result<(), Box<dyn Error + Send + Sync + 'static>>) -> Result<(), Report> {
    err.wrap_err("saw a downstream error")
}
```

We encourage you to write this:

```rust
use std::error::Error;
use eyre::{WrapErr, Report, eyre};

fn wrap_example(err: Result<(), Box<dyn Error + Send + Sync + 'static>>) -> Result<(), Report> {
    err.map_err(|e| eyre!(e)).wrap_err("saw a downstream error")
}
```

# Effect on downcasting

After attaching a message of type `D` onto an error of type `E`, the resulting
`eyre::Report` may be downcast to `D` **or** to `E`.

That is, in codebases that rely on downcasting, Eyre's wrap_err supports
both of the following use cases:

  - **Attaching messages whose type is insignificant onto errors whose type
    is used in downcasts.**

    In other error libraries whose wrap_err is not designed this way, it can
    be risky to introduce messages to existing code because new message might
    break existing working downcasts. In Eyre, any downcast that worked
    before adding the message will continue to work after you add a message, so
    you should freely wrap errors wherever it would be helpful.

    ```
    # use eyre::bail;
    # use thiserror::Error;
    #
    # #[derive(Error, Debug)]
    # #[error("???")]
    # struct SuspiciousError;
    #
    # fn helper() -> Result<()> {
    #     bail!(SuspiciousError);
    # }
    #
    use eyre::{WrapErr, Result};

    fn do_it() -> Result<()> {
        helper().wrap_err("Failed to complete the work")?;
        # const IGNORE: &str = stringify! {
        ...
        # };
        # unreachable!()
    }

    fn main() {
        # #[cfg(not(feature = "auto-install"))]
        # eyre::set_hook(Box::new(eyre::DefaultHandler::default_with)).unwrap();
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<SuspiciousError>() {
            // If helper() returned SuspiciousError, this downcast will
            // correctly succeed even with the message in between.
            # return;
        }
        # panic!("expected downcast to succeed");
    }
    ```

  - **Attaching message whose type is used in downcasts onto errors whose
    type is insignificant.**

    Some codebases prefer to use machine-readable messages to categorize
    lower level errors in a way that will be actionable to higher levels of
    the application.

    ```
    # use eyre::bail;
    # use thiserror::Error;
    #
    # #[derive(Error, Debug)]
    # #[error("???")]
    # struct HelperFailed;
    #
    # fn helper() -> Result<()> {
    #     bail!("no such file or directory");
    # }
    #
    use eyre::{WrapErr, Result};

    fn do_it() -> Result<()> {
        helper().wrap_err(HelperFailed)?;
        # const IGNORE: &str = stringify! {
        ...
        # };
        # unreachable!()
    }

    fn main() {
        # #[cfg(not(feature = "auto-install"))]
        # eyre::set_hook(Box::new(eyre::DefaultHandler::default_with)).unwrap();
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<HelperFailed>() {
            // If helper failed, this downcast will succeed because
            // HelperFailed is the message that has been attached to
            // that error.
            # return;
        }
        # panic!("expected downcast to succeed");
    }
    ```

# `wrap_err` vs `wrap_err_with`

`wrap_err` incurs a runtime cost even in the non-error case because it requires eagerly
constructing the error object. `wrap_err_with` avoids this cost through lazy evaluation. This
cost is proportional to the cost of the currently installed [`EyreHandler`]'s creation step.
`wrap_err` is useful in cases where an constructed error object already exists.

```rust
pub trait WrapErr<T, E>: context::private::Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `wrap_err`: Wrap the error value with a new adhoc error
- `wrap_err_with`: Wrap the error value with a new adhoc error that is evaluated lazily
- `context`: Compatibility re-export of wrap_err for interop with `anyhow`
- `with_context`: Compatibility re-export of wrap_err_with for interop with `anyhow`

#### Implementations

This trait is implemented for the following types:

- `Result<T, E>` with <T, E>

### Trait `OptionExt`

Provides the [`ok_or_eyre`][OptionExt::ok_or_eyre] method for [`Option`].

This trait is sealed and cannot be implemented for types outside of
`eyre`.

# Example

```
# #[cfg(not(feature = "auto-install"))]
# eyre::set_hook(Box::new(eyre::DefaultHandler::default_with)).unwrap();
use eyre::OptionExt;

let option: Option<()> = None;

let result = option.ok_or_eyre("static str error");

assert_eq!(result.unwrap_err().to_string(), "static str error");
```

# `ok_or_eyre` vs `ok_or_else`

If string interpolation is required for the generated [report][Report],
use [`ok_or_else`][Option::ok_or_else] instead,
invoking [`eyre!`] to perform string interpolation:

```
# #[cfg(not(feature = "auto-install"))]
# eyre::set_hook(Box::new(eyre::DefaultHandler::default_with)).unwrap();
use eyre::eyre;

let option: Option<()> = None;

let result = option.ok_or_else(|| eyre!("{} error", "dynamic"));

assert_eq!(result.unwrap_err().to_string(), "dynamic error");
```

`ok_or_eyre` incurs no runtime cost, as the error object
is constructed from the provided static argument
only in the `None` case.

```rust
pub trait OptionExt<T>: context::private::Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `ok_or_eyre`: Transform the [`Option<T>`] into a [`Result<T, E>`],

#### Implementations

This trait is implemented for the following types:

- `Option<T>` with <T>

### Trait `ContextCompat`

Provides the `context` method for `Option` when porting from `anyhow`

This trait is sealed and cannot be implemented for types outside of
`eyre`.

## Why Doesn't `Eyre` impl `WrapErr` for `Option`?

`eyre` doesn't impl `WrapErr` for `Option` because `wrap_err` implies that you're creating a
new error that saves the previous error as its `source`. Calling `wrap_err` on an `Option` is
meaningless because there is no source error. `anyhow` avoids this issue by using a different
mental model where you're adding "context" to an error, though this not a mental model for
error handling that `eyre` agrees with.

Instead, `eyre` encourages users to think of each error as distinct, where the previous error
is the context being saved by the new error, which is backwards compared to anyhow's model. In
this model you're encouraged to use combinators provided by `std` for `Option` to convert an
option to a `Result`

# Example

Instead of:

```rust
use eyre::ContextCompat;

fn get_thing(mut things: impl Iterator<Item = u32>) -> eyre::Result<u32> {
    things
        .find(|&thing| thing == 42)
        .context("the thing wasnt in the list")
}
```

We encourage you to use this:

```rust
use eyre::eyre;

fn get_thing(mut things: impl Iterator<Item = u32>) -> eyre::Result<u32> {
    things
        .find(|&thing| thing == 42)
        .ok_or_else(|| eyre!("the thing wasnt in the list"))
}
```

```rust
pub trait ContextCompat<T>: context::private::Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `context`: Compatibility version of `wrap_err` for creating new errors with new source on `Option`
- `with_context`: Compatibility version of `wrap_err_with` for creating new errors with new source on `Option`
- `wrap_err`: Compatibility re-export of `context` for porting from `anyhow` to `eyre`
- `wrap_err_with`: Compatibility re-export of `with_context` for porting from `anyhow` to `eyre`

#### Implementations

This trait is implemented for the following types:

- `Option<T>` with <T>

## Functions

### Function `set_hook`

Install the provided error hook for constructing EyreHandlers when converting
Errors to Reports

# Details

To customize the format and content of error reports from `eyre` you must
first define a new `EyreHandler` type to capture and store the extra context
and to define the format of how to display the chain of errors and this
stored context. Once this type has been defined you must also define a global
hook used to construct these handlers whenever `Report`s are constructed.

# Examples

```rust,should_panic
use backtrace::Backtrace;
use eyre::EyreHandler;
use std::error::Error;
use std::{fmt, iter};

fn main() -> eyre::Result<()> {
    // Install our custom eyre report hook for constructing our custom Handlers
    install().unwrap();

    // construct a report with, hopefully, our custom handler!
    let mut report = eyre::eyre!("hello from custom error town!");

    // manually set the custom msg for this report after it has been constructed
    if let Some(handler) = report.handler_mut().downcast_mut::<Handler>() {
        handler.custom_msg = Some("you're the best users, you know that right???");
    }

    // print that shit!!
    Err(report)
}

// define a handler that captures backtraces unless told not to
fn install() -> Result<(), impl Error> {
    let capture_backtrace = std::env::var("RUST_BACKWARDS_TRACE")
        .map(|val| val != "0")
        .unwrap_or(true);

    let hook = Hook { capture_backtrace };

    eyre::set_hook(Box::new(move |e| Box::new(hook.make_handler(e))))
}

struct Hook {
    capture_backtrace: bool,
}

impl Hook {
    fn make_handler(&self, _error: &(dyn Error + 'static)) -> Handler {
        let backtrace = if self.capture_backtrace {
            Some(Backtrace::new())
        } else {
            None
        };

        Handler {
            backtrace,
            custom_msg: None,
        }
    }
}

struct Handler {
    // custom configured backtrace capture
    backtrace: Option<Backtrace>,
    // customizable message payload associated with reports
    custom_msg: Option<&'static str>,
}

impl EyreHandler for Handler {
    fn debug(&self, error: &(dyn Error + 'static), f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(error, f);
        }

        let errors = iter::successors(Some(error), |error| (*error).source());

        for (ind, error) in errors.enumerate() {
            write!(f, "\n{:>4}: {}", ind, error)?;
        }

        if let Some(backtrace) = self.backtrace.as_ref() {
            writeln!(f, "\n\nBacktrace:\n{:?}", backtrace)?;
        }

        if let Some(msg) = self.custom_msg.as_ref() {
            writeln!(f, "\n\n{}", msg)?;
        }

        Ok(())
    }
}
```

```rust
pub fn set_hook(hook: Box<dyn Fn(&dyn StdError + ''static) -> Box<dyn EyreHandler> + Sync + Send + ''static>) -> Result<(), InstallError> { /* ... */ }
```

### Function `Ok`

**Attributes:**

- `#[allow(non_snake_case)]`

Equivalent to Ok::<_, eyre::Error>(value).

This simplifies creation of an eyre::Result in places where type inference
cannot deduce the `E` type of the result &mdash; without needing to write
`Ok::<_, eyre::Error>(value)`.

One might think that `eyre::Result::Ok(value)` would work in such cases
but it does not.

```console
error[E0282]: type annotations needed for `std::result::Result<i32, E>`
  --> src/main.rs:11:13
   |
11 |     let _ = eyre::Result::Ok(1);
   |         -   ^^^^^^^^^^^^^^^^ cannot infer type for type parameter `E` declared on the enum `Result`
   |         |
   |         consider giving this pattern the explicit type `std::result::Result<i32, E>`, where the type parameter `E` is specified
```

```rust
pub fn Ok<T>(t: T) -> Result<T> { /* ... */ }
```

## Macros

### Macro `bail`

**Attributes:**

- `#[macro_export]`

Return early with an error.

This macro is equivalent to `return Err(eyre!(<args>))`.

# Example

```
# use eyre::{bail, Result};
#
# fn has_permission(user: usize, resource: usize) -> bool {
#     true
# }
#
# fn main() -> Result<()> {
#     let user = 0;
#     let resource = 0;
#
if !has_permission(user, resource) {
    bail!("permission denied for accessing {}", resource);
}
#     Ok(())
# }
```

```
# use eyre::{bail, Result};
# use thiserror::Error;
#
# const MAX_DEPTH: usize = 1;
#
#[derive(Error, Debug)]
enum ScienceError {
    #[error("recursion limit exceeded")]
    RecursionLimitExceeded,
    # #[error("...")]
    # More = (stringify! {
    ...
    # }, 1).1,
}

# fn main() -> Result<()> {
#     let depth = 0;
#     let err: &'static dyn std::error::Error = &ScienceError::RecursionLimitExceeded;
#
if depth > MAX_DEPTH {
    bail!(ScienceError::RecursionLimitExceeded);
}
#     Ok(())
# }
```

```rust
pub macro_rules! bail {
    /* macro_rules! bail {
    ($msg:literal $(,)?) => { ... };
    ($err:expr $(,)?) => { ... };
    ($fmt:expr, $($arg:tt)*) => { ... };
} */
}
```

### Macro `ensure`

**Attributes:**

- `#[macro_export]`

Return early with an error if a condition is not satisfied.

This macro is equivalent to `if !$cond { return Err(eyre!(<other args>)); }`.

Analogously to `assert!`, `ensure!` takes a condition and exits the function
if the condition fails. Unlike `assert!`, `ensure!` returns an `eyre::Result`
rather than panicking.

# Example

```
# use eyre::{ensure, Result};
#
# fn main() -> Result<()> {
#     let user = 0;
#
ensure!(user == 0, "only user 0 is allowed");
#     Ok(())
# }
```

```
# use eyre::{ensure, Result};
# use thiserror::Error;
#
# const MAX_DEPTH: usize = 1;
#
#[derive(Error, Debug)]
enum ScienceError {
    #[error("recursion limit exceeded")]
    RecursionLimitExceeded,
    # #[error("...")]
    # More = (stringify! {
    ...
    # }, 1).1,
}

# fn main() -> Result<()> {
#     let depth = 0;
#
ensure!(depth <= MAX_DEPTH, ScienceError::RecursionLimitExceeded);
#     Ok(())
# }
```

```rust
pub macro_rules! ensure {
    /* macro_rules! ensure {
    ($cond:expr $(,)?) => { ... };
    ($cond:expr, $msg:literal $(,)?) => { ... };
    ($cond:expr, $err:expr $(,)?) => { ... };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => { ... };
} */
}
```

### Macro `eyre`

**Attributes:**

- `#[macro_export]`

Construct an ad-hoc error from a string.

This evaluates to a `Report`. It can take either just a string, or a format
string with arguments. It also can take any custom type which implements
`Debug` and `Display`.

# Example

```
# type V = ();
#
use eyre::{eyre, Result};

fn lookup(key: &str) -> Result<V> {
    if key.len() != 16 {
        return Err(eyre!("key length must be 16 characters, got {:?}", key));
    }

    // ...
    # Ok(())
}
```

```rust
pub macro_rules! eyre {
    /* macro_rules! eyre {
    ($msg:literal $(,)?) => { ... };
    ($err:expr $(,)?) => { ... };
    ($fmt:expr, $($arg:tt)*) => { ... };
} */
}
```

## Re-exports

### Re-export `eyre`

```rust
pub use eyre as format_err;
```

### Re-export `eyre`

Compatibility re-export of `eyre` for interop with `anyhow`

```rust
pub use eyre as anyhow;
```

### Re-export `Report`

Compatibility re-export of `Report` for interop with `anyhow`

```rust
pub use Report as Error;
```

### Re-export `WrapErr`

Compatibility re-export of `WrapErr` for interop with `anyhow`

```rust
pub use WrapErr as Context;
```

