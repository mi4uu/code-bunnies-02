# Crate Documentation

**Version:** 0.2.0

**Format Version:** 43

# Module `tracing_log`

Adapters for connecting unstructured log records from the `log` crate into
the `tracing` ecosystem.

# Overview

[`tracing`] is a framework for instrumenting Rust programs with context-aware,
structured, event-based diagnostic information. This crate provides
compatibility layers for using `tracing` alongside the logging facade provided
by the [`log`] crate.

This crate provides:

- [`AsTrace`] and [`AsLog`] traits for converting between `tracing` and `log` types.
- [`LogTracer`], a [`log::Log`] implementation that consumes [`log::Record`]s
  and outputs them as [`tracing::Event`].

*Compiler support: [requires `rustc` 1.56+][msrv]*

[msrv]: #supported-rust-versions

# Usage

## Convert log records to tracing `Event`s

To convert [`log::Record`]s as [`tracing::Event`]s, set `LogTracer` as the default
logger by calling its [`init`] or [`init_with_filter`] methods.

```rust
# use std::error::Error;
use tracing_log::LogTracer;
use log;

# fn main() -> Result<(), Box<Error>> {
LogTracer::init()?;

// will be available for Subscribers as a tracing Event
log::trace!("an example trace log");
# Ok(())
# }
```

This conversion does not convert unstructured data in log records (such as
values passed as format arguments to the `log!` macro) to structured
`tracing` fields. However, it *does* attach these new events to to the
span that was currently executing when the record was logged. This is the
primary use-case for this library: making it possible to locate the log
records emitted by dependencies which use `log` within the context of a
trace.

## Convert tracing `Event`s to logs

Enabling the ["log" and "log-always" feature flags][flags] on the `tracing`
crate will cause all `tracing` spans and events to emit `log::Record`s as
they occur.

## Caution: Mixing both conversions

Note that `log::Logger` implementations that convert log records to trace events
should not be used with `Subscriber`s that convert trace events _back_ into
`log` records, as doing so will result in the event recursing between the subscriber
and the logger forever (or, in real life, probably overflowing the call stack).

If the logging of trace events generated from log records produced by the
`log` crate is desired, either the `log` crate should not be used to
implement this logging, or an additional layer of filtering will be
required to avoid infinitely converting between `Event` and `log::Record`.

## Feature Flags

* `std`: enables features that require the Rust standard library (on by default)
* `log-tracer`: enables the `LogTracer` type (on by default)
* `interest-cache`: makes it possible to configure an interest cache for
  logs emitted through the `log` crate (see [`Builder::with_interest_cache`]); requires `std`

## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.56. The current Tracing version is not guaranteed to build on
Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.69, the minimum supported version will not be
increased past 1.66, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.

[`init`]: LogTracer::init
[`init_with_filter`]: LogTracer::init_with_filter
[`tracing`]: https://crates.io/crates/tracing
[`tracing::Subscriber`]: https://docs.rs/tracing/latest/tracing/trait.Subscriber.html
[`Subscriber`]: https://docs.rs/tracing/latest/tracing/trait.Subscriber.html
[`tracing::Event`]: https://docs.rs/tracing/latest/tracing/struct.Event.html
[flags]: https://docs.rs/tracing/latest/tracing/#crate-feature-flags
[`Builder::with_interest_cache`]: log_tracer::Builder::with_interest_cache

## Modules

## Module `log_tracer`

**Attributes:**

- `#[<cfg>(feature = "log-tracer")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "log-tracer")))]`

An adapter for converting [`log`] records into `tracing` `Event`s.

This module provides the [`LogTracer`] type which implements `log`'s [logger
interface] by recording log records as `tracing` `Event`s. This is intended for
use in conjunction with a `tracing` `Subscriber` to consume events from
dependencies that emit [`log`] records within a trace context.

# Usage

To create and initialize a `LogTracer` with the default configurations, use:

* [`init`] if you want to convert all logs, regardless of log level,
  allowing the tracing `Subscriber` to perform any filtering
* [`init_with_filter`] to convert all logs up to a specified log level

In addition, a [builder] is available for cases where more advanced
configuration is required. In particular, the builder can be used to [ignore
log records][ignore] emitted by particular crates. This is useful in cases
such as when a crate emits both `tracing` diagnostics _and_ log records by
default.

[logger interface]: log::Log
[`init`]: LogTracer.html#method.init
[`init_with_filter`]: LogTracer.html#method.init_with_filter
[builder]: LogTracer::builder()
[ignore]: Builder::ignore_crate()

```rust
pub mod log_tracer { /* ... */ }
```

### Types

#### Struct `LogTracer`

A simple "logger" that converts all log records into `tracing` `Event`s.

```rust
pub struct LogTracer {
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
  pub fn builder() -> Builder { /* ... */ }
  ```
  Returns a builder that allows customizing a `LogTracer` and setting it

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new `LogTracer` that can then be used as a logger for the `log` crate.

- ```rust
  pub fn init_with_filter(level: log::LevelFilter) -> Result<(), SetLoggerError> { /* ... */ }
  ```
  Sets up `LogTracer` as global logger for the `log` crate,

- ```rust
  pub fn init() -> Result<(), SetLoggerError> { /* ... */ }
  ```
  Sets a `LogTracer` as the global logger for the `log` crate.

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Log**
  - ```rust
    fn enabled(self: &Self, metadata: &log::Metadata<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn log(self: &Self, record: &log::Record<''_>) { /* ... */ }
    ```

  - ```rust
    fn flush(self: &Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `Builder`

Configures a new `LogTracer`.

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
  pub fn new() -> Self { /* ... */ }
  ```
  Returns a new `Builder` to construct a [`LogTracer`].

- ```rust
  pub fn with_max_level</* synthetic */ impl Into<log::LevelFilter>: Into<log::LevelFilter>>(self: Self, filter: impl Into<log::LevelFilter>) -> Self { /* ... */ }
  ```
  Sets a global maximum level for `log` records.

- ```rust
  pub fn ignore_crate</* synthetic */ impl Into<String>: Into<String>>(self: Self, name: impl Into<String>) -> Self { /* ... */ }
  ```
  Configures the `LogTracer` to ignore all log records whose target

- ```rust
  pub fn ignore_all<I, /* synthetic */ impl IntoIterator<Item = I>: IntoIterator<Item = I>>(self: Self, crates: impl IntoIterator<Item = I>) -> Self
where
    I: Into<String> { /* ... */ }
  ```
  Configures the `LogTracer` to ignore all log records whose target

- ```rust
  pub fn init(self: Self) -> Result<(), SetLoggerError> { /* ... */ }
  ```
  Constructs a new `LogTracer` with the provided configuration and sets it

###### Trait Implementations

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Re-exports

#### Re-export `SetLoggerError`

```rust
pub use log::SetLoggerError;
```

## Traits

### Trait `AsLog`

Trait implemented for `tracing` types that can be converted to a `log`
equivalent.

```rust
pub trait AsLog: crate::sealed::Sealed {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Log`: The `log` type that this type can be converted into.

##### Required Methods

- `as_log`: Returns the `log` equivalent of `self`.

#### Implementations

This trait is implemented for the following types:

- `tracing_core::Metadata<''a>` with <''a>
- `tracing_core::Level`
- `tracing_core::LevelFilter`

### Trait `AsTrace`

Trait implemented for `log` types that can be converted to a `tracing`
equivalent.

```rust
pub trait AsTrace: crate::sealed::Sealed {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Trace`: The `tracing` type that this type can be converted into.

##### Required Methods

- `as_trace`: Returns the `tracing` equivalent of `self`.

#### Implementations

This trait is implemented for the following types:

- `log::Metadata<''a>` with <''a>
- `log::Record<''a>` with <''a>
- `log::Level`
- `log::LevelFilter`

### Trait `NormalizeEvent`

Extends log `Event`s to provide complete `Metadata`.

In `tracing-log`, an `Event` produced by a log (through [`AsTrace`]) has an hard coded
"log" target and no `file`, `line`, or `module_path` attributes. This happens because `Event`
requires its `Metadata` to be `'static`, while [`log::Record`]s provide them with a generic
lifetime.

However, these values are stored in the `Event`'s fields and
the [`normalized_metadata`] method allows to build a new `Metadata`
that only lives as long as its source `Event`, but provides complete
data.

It can typically be used by `Subscriber`s when processing an `Event`,
to allow accessing its complete metadata in a consistent way,
regardless of the source of its source.

[`normalized_metadata`]: NormalizeEvent#normalized_metadata

```rust
pub trait NormalizeEvent<''a>: crate::sealed::Sealed {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `normalized_metadata`: If this `Event` comes from a `log`, this method provides a new
- `is_log`: Returns whether this `Event` represents a log (from the `log` crate)

#### Implementations

This trait is implemented for the following types:

- `tracing_core::Event<''a>` with <''a>

## Functions

### Function `format_trace`

Format a log record as a trace event in the current span.

```rust
pub fn format_trace(record: &log::Record<''_>) -> io::Result<()> { /* ... */ }
```

## Re-exports

### Re-export `LogTracer`

**Attributes:**

- `#[<cfg>(feature = "log-tracer")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "log-tracer")))]`
- `#[doc(inline)]`

```rust
pub use self::log_tracer::LogTracer;
```

### Re-export `log`

```rust
pub use log;
```

