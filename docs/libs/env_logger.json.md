# Crate Documentation

**Version:** 0.10.2

**Format Version:** 43

# Module `env_logger`

A simple logger that can be configured via environment variables, for use
with the logging facade exposed by the [`log` crate][log-crate-url].

Despite having "env" in its name, **`env_logger`** can also be configured by
other means besides environment variables. See [the examples][gh-repo-examples]
in the source repository for more approaches.

By default, `env_logger` writes logs to `stderr`, but can be configured to
instead write them to `stdout`.

## Example

```
use log::{debug, error, log_enabled, info, Level};

env_logger::init();

debug!("this is a debug {}", "message");
error!("this is printed by default");

if log_enabled!(Level::Info) {
    let x = 3 * 4; // expensive computation
    info!("the answer was: {}", x);
}
```

Assumes the binary is `main`:

```{.bash}
$ RUST_LOG=error ./main
[2017-11-09T02:12:24Z ERROR main] this is printed by default
```

```{.bash}
$ RUST_LOG=info ./main
[2017-11-09T02:12:24Z ERROR main] this is printed by default
[2017-11-09T02:12:24Z INFO main] the answer was: 12
```

```{.bash}
$ RUST_LOG=debug ./main
[2017-11-09T02:12:24Z DEBUG main] this is a debug message
[2017-11-09T02:12:24Z ERROR main] this is printed by default
[2017-11-09T02:12:24Z INFO main] the answer was: 12
```

You can also set the log level on a per module basis:

```{.bash}
$ RUST_LOG=main=info ./main
[2017-11-09T02:12:24Z ERROR main] this is printed by default
[2017-11-09T02:12:24Z INFO main] the answer was: 12
```

And enable all logging:

```{.bash}
$ RUST_LOG=main ./main
[2017-11-09T02:12:24Z DEBUG main] this is a debug message
[2017-11-09T02:12:24Z ERROR main] this is printed by default
[2017-11-09T02:12:24Z INFO main] the answer was: 12
```

If the binary name contains hyphens, you will need to replace
them with underscores:

```{.bash}
$ RUST_LOG=my_app ./my-app
[2017-11-09T02:12:24Z DEBUG my_app] this is a debug message
[2017-11-09T02:12:24Z ERROR my_app] this is printed by default
[2017-11-09T02:12:24Z INFO my_app] the answer was: 12
```

This is because Rust modules and crates cannot contain hyphens
in their name, although `cargo` continues to accept them.

See the documentation for the [`log` crate][log-crate-url] for more
information about its API.

## Enabling logging

**By default all logging is disabled except for the `error` level**

The **`RUST_LOG`** environment variable controls logging with the syntax:
```text
RUST_LOG=[target][=][level][,...]
```
Or in other words, its a comma-separated list of directives.
Directives can filter by **target**, by **level**, or both (using `=`).

For example,
```text
RUST_LOG=data=debug,hardware=debug
```

**target** is typically the path of the module the message
in question originated from, though it can be overridden.
The path is rooted in the name of the crate it was compiled for, so if
your program is in a file called, for example, `hello.rs`, the path would
simply be `hello`.

Furthermore, the log can be filtered using prefix-search based on the
specified log target.

For example, `RUST_LOG=example` would match the following targets:
- `example`
- `example::test`
- `example::test::module::submodule`
- `examples::and_more_examples`

When providing the crate name or a module path, explicitly specifying the
log level is optional. If omitted, all logging for the item will be
enabled.

**level** is the maximum [`log::Level`][level-enum] to be shown and includes:
- `error`
- `warn`
- `info`
- `debug`
- `trace`
- `off` (pseudo level to disable all logging for the target)

Logging level names are case-insensitive; e.g.,
`debug`, `DEBUG`, and `dEbuG` all represent the same logging level. For
consistency, our convention is to use the lower case names. Where our docs
do use other forms, they do so in the context of specific examples, so you
won't be surprised if you see similar usage in the wild.

Some examples of valid values of `RUST_LOG` are:

- `RUST_LOG=hello` turns on all logging for the `hello` module
- `RUST_LOG=trace` turns on all logging for the application, regardless of its name
- `RUST_LOG=TRACE` turns on all logging for the application, regardless of its name (same as previous)
- `RUST_LOG=info` turns on all info logging
- `RUST_LOG=INFO` turns on all info logging (same as previous)
- `RUST_LOG=hello=debug` turns on debug logging for `hello`
- `RUST_LOG=hello=DEBUG` turns on debug logging for `hello` (same as previous)
- `RUST_LOG=hello,std::option` turns on `hello`, and std's option logging
- `RUST_LOG=error,hello=warn` turn on global error logging and also warn for `hello`
- `RUST_LOG=error,hello=off`  turn on global error logging, but turn off logging for `hello`
- `RUST_LOG=off` turns off all logging for the application
- `RUST_LOG=OFF` turns off all logging for the application (same as previous)

## Filtering results

A `RUST_LOG` directive may include a regex filter. The syntax is to append `/`
followed by a regex. Each message is checked against the regex, and is only
logged if it matches. Note that the matching is done after formatting the
log string but before adding any logging meta-data. There is a single filter
for all modules.

Some examples:

* `hello/foo` turns on all logging for the 'hello' module where the log
  message includes 'foo'.
* `info/f.o` turns on all info logging where the log message includes 'foo',
  'f1o', 'fao', etc.
* `hello=debug/foo*foo` turns on debug logging for 'hello' where the log
  message includes 'foofoo' or 'fofoo' or 'fooooooofoo', etc.
* `error,hello=warn/[0-9]scopes` turn on global error logging and also
  warn for hello. In both cases the log message must include a single digit
  number followed by 'scopes'.

## Capturing logs in tests

Records logged during `cargo test` will not be captured by the test harness by default.
The [`Builder::is_test`] method can be used in unit tests to ensure logs will be captured:

```
#[cfg(test)]
mod tests {
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn it_works() {
        init();

        info!("This record will be captured by `cargo test`");

        assert_eq!(2, 1 + 1);
    }
}
```

Enabling test capturing comes at the expense of color and other style support
and may have performance implications.

## Disabling colors

Colors and other styles can be configured with the `RUST_LOG_STYLE`
environment variable. It accepts the following values:

* `auto` (default) will attempt to print style characters, but don't force the issue.
If the console isn't available on Windows, or if TERM=dumb, for example, then don't print colors.
* `always` will always print style characters even if they aren't supported by the terminal.
This includes emitting ANSI colors on Windows if the console API is unavailable.
* `never` will never print style characters.

## Tweaking the default format

Parts of the default format can be excluded from the log output using the [`Builder`].
The following example excludes the timestamp from the log output:

```
env_logger::builder()
    .format_timestamp(None)
    .init();
```

### Stability of the default format

The default format won't optimise for long-term stability, and explicitly makes no
guarantees about the stability of its output across major, minor or patch version
bumps during `0.x`.

If you want to capture or interpret the output of `env_logger` programmatically
then you should use a custom format.

### Using a custom format

Custom formats can be provided as closures to the [`Builder`].
These closures take a [`Formatter`][crate::fmt::Formatter] and `log::Record` as arguments:

```
use std::io::Write;

env_logger::builder()
    .format(|buf, record| {
        writeln!(buf, "{}: {}", record.level(), record.args())
    })
    .init();
```

See the [`fmt`] module for more details about custom formats.

## Specifying defaults for environment variables

`env_logger` can read configuration from environment variables.
If these variables aren't present, the default value to use can be tweaked with the [`Env`] type.
The following example defaults to log `warn` and above if the `RUST_LOG` environment variable
isn't set:

```
use env_logger::Env;

env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
```

[gh-repo-examples]: https://github.com/rust-cli/env_logger/tree/main/examples
[level-enum]: https://docs.rs/log/latest/log/enum.Level.html
[log-crate-url]: https://docs.rs/log
[`Builder`]: struct.Builder.html
[`Builder::is_test`]: struct.Builder.html#method.is_test
[`Env`]: struct.Env.html
[`fmt`]: fmt/index.html

## Modules

## Module `filter`

Filtering for log records.

This module contains the log filtering used by `env_logger` to match records.
You can use the `Filter` type in your own logger implementation to use the same
filter parsing and matching as `env_logger`. For more details about the format
for directive strings see [Enabling Logging].

## Using `env_logger` in your own logger

You can use `env_logger`'s filtering functionality with your own logger.
Call [`Builder::parse`] to parse directives from a string when constructing
your logger. Call [`Filter::matches`] to check whether a record should be
logged based on the parsed filters when log records are received.

```
extern crate log;
extern crate env_logger;
use env_logger::filter::Filter;
use log::{Log, Metadata, Record};

struct MyLogger {
    filter: Filter
}

impl MyLogger {
    fn new() -> MyLogger {
        use env_logger::filter::Builder;
        let mut builder = Builder::new();

        // Parse a directives string from an environment variable
        if let Ok(ref filter) = std::env::var("MY_LOG_LEVEL") {
           builder.parse(filter);
        }

        MyLogger {
            filter: builder.build()
        }
    }
}

impl Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        // Check if the record is matched by the filter
        if self.filter.matches(record) {
            println!("{:?}", record);
        }
    }

    fn flush(&self) {}
}
```

[Enabling Logging]: ../index.html#enabling-logging
[`Builder::parse`]: struct.Builder.html#method.parse
[`Filter::matches`]: struct.Filter.html#method.matches

```rust
pub mod filter { /* ... */ }
```

### Types

#### Struct `Builder`

A builder for a log filter.

It can be used to parse a set of directives from a string before building
a [`Filter`] instance.

## Example

```
# #[macro_use] extern crate log;
# use std::env;
use env_logger::filter::Builder;

let mut builder = Builder::new();

// Parse a logging filter from an environment variable.
if let Ok(rust_log) = env::var("RUST_LOG") {
    builder.parse(&rust_log);
}

let filter = builder.build();
```

[`Filter`]: struct.Filter.html

```rust
pub struct Builder {
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
  pub fn new() -> Builder { /* ... */ }
  ```
  Initializes the filter builder with defaults.

- ```rust
  pub fn from_env(env: &str) -> Builder { /* ... */ }
  ```
  Initializes the filter builder from an environment.

- ```rust
  pub fn filter_module(self: &mut Self, module: &str, level: LevelFilter) -> &mut Self { /* ... */ }
  ```
  Adds a directive to the filter for a specific module.

- ```rust
  pub fn filter_level(self: &mut Self, level: LevelFilter) -> &mut Self { /* ... */ }
  ```
  Adds a directive to the filter for all modules.

- ```rust
  pub fn filter(self: &mut Self, module: Option<&str>, level: LevelFilter) -> &mut Self { /* ... */ }
  ```
  Adds a directive to the filter.

- ```rust
  pub fn parse(self: &mut Self, filters: &str) -> &mut Self { /* ... */ }
  ```
  Parses the directives string.

- ```rust
  pub fn build(self: &mut Self) -> Filter { /* ... */ }
  ```
  Build a log filter.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Filter`

A log filter.

This struct can be used to determine whether or not a log record
should be written to the output.
Use the [`Builder`] type to parse and construct a `Filter`.

[`Builder`]: struct.Builder.html

```rust
pub struct Filter {
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
  pub fn filter(self: &Self) -> LevelFilter { /* ... */ }
  ```
  Returns the maximum `LevelFilter` that this filter instance is

- ```rust
  pub fn matches(self: &Self, record: &Record<''_>) -> bool { /* ... */ }
  ```
  Checks if this record matches the configured filter.

- ```rust
  pub fn enabled(self: &Self, metadata: &Metadata<''_>) -> bool { /* ... */ }
  ```
  Determines if a log message with the specified metadata would be logged.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `fmt`

Formatting for log records.

This module contains a [`Formatter`] that can be used to format log records
into without needing temporary allocations. Usually you won't need to worry
about the contents of this module and can use the `Formatter` like an ordinary
[`Write`].

# Formatting log records

The format used to print log records can be customised using the [`Builder::format`]
method.
Custom formats can apply different color and weight to printed values using
[`Style`] builders.

```
use std::io::Write;

let mut builder = env_logger::Builder::new();

builder.format(|buf, record| {
    writeln!(buf, "{}: {}",
        record.level(),
        record.args())
});
```

[`Formatter`]: struct.Formatter.html
[`Style`]: struct.Style.html
[`Builder::format`]: ../struct.Builder.html#method.format
[`Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html

```rust
pub mod fmt { /* ... */ }
```

### Types

#### Enum `TimestampPrecision`

Formatting precision of timestamps.

Seconds give precision of full seconds, milliseconds give thousands of a
second (3 decimal digits), microseconds are millionth of a second (6 decimal
digits) and nanoseconds are billionth of a second (9 decimal digits).

```rust
pub enum TimestampPrecision {
    Seconds,
    Millis,
    Micros,
    Nanos,
}
```

##### Variants

###### `Seconds`

Full second precision (0 decimal digits)

###### `Millis`

Millisecond precision (3 decimal digits)

###### `Micros`

Microsecond precision (6 decimal digits)

###### `Nanos`

Nanosecond precision (9 decimal digits)

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn clone(self: &Self) -> TimestampPrecision { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Formatter`

A formatter to write logs into.

`Formatter` implements the standard [`Write`] trait for writing log records.
It also supports terminal colors, through the [`style`] method.

# Examples

Use the [`writeln`] macro to format a log record.
An instance of a `Formatter` is passed to an `env_logger` format as `buf`:

```
use std::io::Write;

let mut builder = env_logger::Builder::new();

builder.format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()));
```

[`Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
[`writeln`]: https://doc.rust-lang.org/stable/std/macro.writeln.html
[`style`]: #method.style

```rust
pub struct Formatter {
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
  pub fn timestamp(self: &Self) -> Timestamp { /* ... */ }
  ```
  Get a [`Timestamp`] for the current date and time in UTC.

- ```rust
  pub fn timestamp_seconds(self: &Self) -> Timestamp { /* ... */ }
  ```
  Get a [`Timestamp`] for the current date and time in UTC with full

- ```rust
  pub fn timestamp_millis(self: &Self) -> Timestamp { /* ... */ }
  ```
  Get a [`Timestamp`] for the current date and time in UTC with

- ```rust
  pub fn timestamp_micros(self: &Self) -> Timestamp { /* ... */ }
  ```
  Get a [`Timestamp`] for the current date and time in UTC with

- ```rust
  pub fn timestamp_nanos(self: &Self) -> Timestamp { /* ... */ }
  ```
  Get a [`Timestamp`] for the current date and time in UTC with

- ```rust
  pub fn style(self: &Self) -> Style { /* ... */ }
  ```
  Begin a new [`Style`].

- ```rust
  pub fn default_level_style(self: &Self, level: Level) -> Style { /* ... */ }
  ```
  Get the default [`Style`] for the given level.

- ```rust
  pub fn default_styled_level(self: &Self, level: Level) -> StyledValue<''static, Level> { /* ... */ }
  ```
  Get a printable [`Style`] for the given level.

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **Unpin**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Re-exports

#### Re-export `Color`

**Attributes:**

- `#[<cfg>(feature = "color")]`

```rust
pub use style::Color;
```

#### Re-export `Style`

**Attributes:**

- `#[<cfg>(feature = "color")]`

```rust
pub use style::Style;
```

#### Re-export `StyledValue`

**Attributes:**

- `#[<cfg>(feature = "color")]`

```rust
pub use style::StyledValue;
```

#### Re-export `Timestamp`

**Attributes:**

- `#[<cfg>(feature = "humantime")]`

```rust
pub use self::humantime::Timestamp;
```

#### Re-export `self::writer::glob::*`

```rust
pub use self::writer::glob::*;
```

## Re-exports

### Re-export `self::fmt::glob::*`

```rust
pub use self::fmt::glob::*;
```

### Re-export `self::logger::*`

```rust
pub use self::logger::*;
```

