# Crate Documentation

**Version:** 0.3.19

**Format Version:** 43

# Module `tracing_subscriber`

Utilities for implementing and composing [`tracing`] subscribers.

[`tracing`] is a framework for instrumenting Rust programs to collect
scoped, structured, and async-aware diagnostics. The [`Subscriber`] trait
represents the functionality necessary to collect this trace data. This
crate contains tools for composing subscribers out of smaller units of
behaviour, and batteries-included implementations of common subscriber
functionality.

`tracing-subscriber` is intended for use by both `Subscriber` authors and
application authors using `tracing` to instrument their applications.

*Compiler support: [requires `rustc` 1.63+][msrv]*

[msrv]: #supported-rust-versions

## `Layer`s and `Filter`s

The most important component of the `tracing-subscriber` API is the
[`Layer`] trait, which provides a composable abstraction for building
[`Subscriber`]s. Like the [`Subscriber`] trait, a [`Layer`] defines a
particular behavior for collecting trace data. Unlike [`Subscriber`]s,
which implement a *complete* strategy for how trace data is collected,
[`Layer`]s provide *modular* implementations of specific behaviors.
Therefore, they can be [composed together] to form a [`Subscriber`] which is
capable of recording traces in a variety of ways. See the [`layer` module's
documentation][layer] for details on using [`Layer`]s.

In addition, the [`Filter`] trait defines an interface for filtering what
spans and events are recorded by a particular layer. This allows different
[`Layer`]s to handle separate subsets of the trace data emitted by a
program. See the [documentation on per-layer filtering][plf] for more
information on using [`Filter`]s.

[`Layer`]: crate::layer::Layer
[composed together]: crate::layer#composing-layers
[layer]: crate::layer
[`Filter`]: crate::layer::Filter
[plf]: crate::layer#per-layer-filtering

## Included Subscribers

The following `Subscriber`s are provided for application authors:

- [`fmt`] - Formats and logs tracing data (requires the `fmt` feature flag)

## Feature Flags

- `std`: Enables APIs that depend on the Rust standard library
  (enabled by default).
- `alloc`: Depend on [`liballoc`] (enabled by "std").
- `env-filter`: Enables the [`EnvFilter`] type, which implements filtering
  similar to the [`env_logger` crate]. **Requires "std"**.
- `fmt`: Enables the [`fmt`] module, which provides a subscriber
  implementation for printing formatted representations of trace events.
  Enabled by default. **Requires "registry" and "std"**.
- `ansi`: Enables `fmt` support for ANSI terminal colors. Enabled by
  default.
- `registry`: enables the [`registry`] module. Enabled by default.
  **Requires "std"**.
- `json`: Enables `fmt` support for JSON output. In JSON output, the ANSI
  feature does nothing. **Requires "fmt" and "std"**.
- `local-time`: Enables local time formatting when using the [`time`
  crate]'s timestamp formatters with the `fmt` subscriber.

[`registry`]: mod@registry

### Optional Dependencies

- [`tracing-log`]: Enables better formatting for events emitted by `log`
  macros in the `fmt` subscriber. Enabled by default.
- [`time`][`time` crate]: Enables support for using the [`time` crate] for timestamp
  formatting in the `fmt` subscriber.
- [`smallvec`]: Causes the `EnvFilter` type to use the `smallvec` crate (rather
  than `Vec`) as a performance optimization. Enabled by default.
- [`parking_lot`]: Use the `parking_lot` crate's `RwLock` implementation
  rather than the Rust standard library's implementation.

### `no_std` Support

In embedded systems and other bare-metal applications, `tracing` can be
used without requiring the Rust standard library, although some features are
disabled. Although most of the APIs provided by `tracing-subscriber`, such
as [`fmt`] and [`EnvFilter`], require the standard library, some
functionality, such as the [`Layer`] trait, can still be used in
`no_std` environments.

The dependency on the standard library is controlled by two crate feature
flags, "std", which enables the dependency on [`libstd`], and "alloc", which
enables the dependency on [`liballoc`] (and is enabled by the "std"
feature). These features are enabled by default, but `no_std` users can
disable them using:

```toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false }
```

Additional APIs are available when [`liballoc`] is available. To enable
`liballoc` but not `std`, use:

```toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false, features = ["alloc"] }
```

### Unstable Features

These feature flags enable **unstable** features. The public API may break in 0.1.x
releases. To enable these features, the `--cfg tracing_unstable` must be passed to
`rustc` when compiling.

The following unstable feature flags are currently available:

* `valuable`: Enables support for serializing values recorded using the
  [`valuable`] crate as structured JSON in the [`format::Json`] formatter.

#### Enabling Unstable Features

The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
env variable when running `cargo` commands:

```shell
RUSTFLAGS="--cfg tracing_unstable" cargo build
```
Alternatively, the following can be added to the `.cargo/config` file in a
project to automatically enable the cfg flag for that project:

```toml
[build]
rustflags = ["--cfg", "tracing_unstable"]
```

[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[`valuable`]: https://crates.io/crates/valuable
[`format::Json`]: crate::fmt::format::Json

## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.63. The current Tracing version is not guaranteed to build on
Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.69, the minimum supported version will not be
increased past 1.66, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.

[`Subscriber`]: tracing_core::subscriber::Subscriber
[`tracing`]: https://docs.rs/tracing/latest/tracing
[`EnvFilter`]: filter::EnvFilter
[`fmt`]: mod@fmt
[`tracing-log`]: https://crates.io/crates/tracing-log
[`smallvec`]: https://crates.io/crates/smallvec
[`env_logger` crate]: https://crates.io/crates/env_logger
[`parking_lot`]: https://crates.io/crates/parking_lot
[`time` crate]: https://crates.io/crates/time
[`liballoc`]: https://doc.rust-lang.org/alloc/index.html
[`libstd`]: https://doc.rust-lang.org/std/index.html

## Modules

## Module `field`

Utilities for working with [fields] and [field visitors].

[fields]: tracing_core::field
[field visitors]: tracing_core::field::Visit

```rust
pub mod field { /* ... */ }
```

### Modules

## Module `debug`

`MakeVisitor` wrappers for working with `fmt::Debug` fields.

```rust
pub mod debug { /* ... */ }
```

### Types

#### Struct `Alt`

A visitor wrapper that ensures any `fmt::Debug` fields are formatted using
the alternate (`:#`) formatter.

```rust
pub struct Alt<V>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(inner: V) -> Self { /* ... */ }
  ```
  Wraps the provided visitor so that any `fmt::Debug` fields are formatted

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MakeExt**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Visit**
  - ```rust
    fn record_f64(self: &mut Self, field: &Field, value: f64) { /* ... */ }
    ```

  - ```rust
    fn record_i64(self: &mut Self, field: &Field, value: i64) { /* ... */ }
    ```

  - ```rust
    fn record_u64(self: &mut Self, field: &Field, value: u64) { /* ... */ }
    ```

  - ```rust
    fn record_bool(self: &mut Self, field: &Field, value: bool) { /* ... */ }
    ```

  - ```rust
    fn record_str(self: &mut Self, field: &Field, value: &str) { /* ... */ }
    ```
    Visit a string value.

  - ```rust
    fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **VisitOutput**
  - ```rust
    fn finish(self: Self) -> O { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Alt<V> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **MakeVisitor**
  - ```rust
    fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VisitWrite**
  - ```rust
    fn writer(self: &mut Self) -> &mut dyn io::Write { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Unpin**
- **VisitFmt**
  - ```rust
    fn writer(self: &mut Self) -> &mut dyn fmt::Write { /* ... */ }
    ```

## Module `delimited`

A `MakeVisitor` wrapper that separates formatted fields with a delimiter.

```rust
pub mod delimited { /* ... */ }
```

### Types

#### Struct `Delimited`

A `MakeVisitor` wrapper that wraps a visitor that writes formatted output so
that a delimiter is inserted between writing formatted field values.

```rust
pub struct Delimited<D, V> {
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
  pub fn new(delimiter: D, inner: V) -> Self { /* ... */ }
  ```
  Returns a new [`MakeVisitor`] implementation that wraps `inner` so that

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
- **MakeExt**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn clone(self: &Self) -> Delimited<D, V> { /* ... */ }
    ```

- **MakeVisitor**
  - ```rust
    fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `VisitDelimited`

A visitor wrapper that inserts a delimiter after the wrapped visitor formats
a field value.

```rust
pub struct VisitDelimited<D, V> {
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
  pub fn new(delimiter: D, inner: V) -> Self { /* ... */ }
  ```
  Returns a new [`Visit`] implementation that wraps `inner` so that

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **VisitFmt**
  - ```rust
    fn writer(self: &mut Self) -> &mut dyn fmt::Write { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Visit**
  - ```rust
    fn record_i64(self: &mut Self, field: &Field, value: i64) { /* ... */ }
    ```

  - ```rust
    fn record_u64(self: &mut Self, field: &Field, value: u64) { /* ... */ }
    ```

  - ```rust
    fn record_bool(self: &mut Self, field: &Field, value: bool) { /* ... */ }
    ```

  - ```rust
    fn record_str(self: &mut Self, field: &Field, value: &str) { /* ... */ }
    ```

  - ```rust
    fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **VisitOutput**
  - ```rust
    fn finish(self: Self) -> fmt::Result { /* ... */ }
    ```

## Module `display`

`MakeVisitor` wrappers for working with `fmt::Display` fields.

```rust
pub mod display { /* ... */ }
```

### Types

#### Struct `Messages`

A visitor wrapper that ensures any strings named "message" are formatted
using `fmt::Display`

```rust
pub struct Messages<V>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(inner: V) -> Self { /* ... */ }
  ```
  Returns a new [`MakeVisitor`] implementation that will wrap `inner` so

###### Trait Implementations

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Visit**
  - ```rust
    fn record_f64(self: &mut Self, field: &Field, value: f64) { /* ... */ }
    ```

  - ```rust
    fn record_i64(self: &mut Self, field: &Field, value: i64) { /* ... */ }
    ```

  - ```rust
    fn record_u64(self: &mut Self, field: &Field, value: u64) { /* ... */ }
    ```

  - ```rust
    fn record_bool(self: &mut Self, field: &Field, value: bool) { /* ... */ }
    ```

  - ```rust
    fn record_str(self: &mut Self, field: &Field, value: &str) { /* ... */ }
    ```
    Visit a string value.

  - ```rust
    fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug) { /* ... */ }
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

- **Sync**
- **Unpin**
- **UnwindSafe**
- **MakeExt**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Messages<V> { /* ... */ }
    ```

- **MakeVisitor**
  - ```rust
    fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor { /* ... */ }
    ```

- **VisitOutput**
  - ```rust
    fn finish(self: Self) -> O { /* ... */ }
    ```

- **VisitWrite**
  - ```rust
    fn writer(self: &mut Self) -> &mut dyn io::Write { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VisitFmt**
  - ```rust
    fn writer(self: &mut Self) -> &mut dyn fmt::Write { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `MakeVisitor`

Creates new [visitors].

A type implementing `MakeVisitor` represents a composable factory for types
implementing the [`Visit` trait][visitors]. The `MakeVisitor` trait defines
a single function, `make_visitor`, which takes in a `T`-typed `target` and
returns a type implementing `Visit` configured for that target. A target may
be a string, output stream, or data structure that the visitor will record
data to, configuration variables that determine the visitor's behavior, or
`()` when no input is required to produce a visitor.

[visitors]: tracing_core::field::Visit

```rust
pub trait MakeVisitor<T> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Visitor`: The visitor type produced by this `MakeVisitor`.

###### Required Methods

- `make_visitor`: Make a new visitor for the provided `target`.

##### Implementations

This trait is implemented for the following types:

- `Alt<V>` with <T, V>
- `Delimited<D, V>` with <D, V, T>
- `Messages<V>` with <T, V>
- `F` with <T, V, F>
- `PrettyFields` with <''a>
- `DefaultFields` with <''a>
- `FieldFn<F>` with <''a, F>

#### Trait `VisitOutput`

A [visitor] that produces output once it has visited a set of fields.

[visitor]: tracing_core::field::Visit

```rust
pub trait VisitOutput<Out>: Visit {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `finish`: Completes the visitor, returning any output.

##### Provided Methods

- ```rust
  fn visit<R>(self: Self, fields: &R) -> Out
where
    R: RecordFields,
    Self: Sized { /* ... */ }
  ```
  Visit a set of fields, and return the output of finishing the visitor

##### Implementations

This trait is implemented for the following types:

- `Alt<V>` with <V, O>
- `VisitDelimited<D, V>` with <D, V>
- `Messages<V>` with <V, O>
- `PrettyVisitor<''_>`
- `DefaultVisitor<''_>`
- `FieldFnVisitor<''a, F>` with <''a, F>

#### Trait `RecordFields`

Extension trait implemented by types which can be recorded by a [visitor].

This allows writing code that is generic over `tracing_core`'s
[`span::Attributes`][attr], [`span::Record`][rec], and [`Event`]
types. These types all provide inherent `record` methods that allow a
visitor to record their fields, but there is no common trait representing this.

With `RecordFields`, we can write code like this:
```
use tracing_core::field::Visit;
# use tracing_core::field::Field;
use tracing_subscriber::field::RecordFields;

struct MyVisitor {
    // ...
}
# impl MyVisitor { fn new() -> Self { Self{} } }
impl Visit for MyVisitor {
    // ...
# fn record_debug(&mut self, _: &Field, _: &dyn std::fmt::Debug) {}
}

fn record_with_my_visitor<R>(r: R)
where
    R: RecordFields,
{
    let mut visitor = MyVisitor::new();
    r.record(&mut visitor);
}
```
[visitor]: tracing_core::field::Visit
[attr]: tracing_core::span::Attributes
[rec]: tracing_core::span::Record

```rust
pub trait RecordFields: crate::sealed::Sealed<RecordFieldsMarker> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `record`: Record all the fields in `self` with the provided `visitor`.

##### Implementations

This trait is implemented for the following types:

- `tracing_core::Event<''_>`
- `tracing_core::span::Attributes<''_>`
- `tracing_core::span::Record<''_>`
- `&F` with <F>

#### Trait `MakeOutput`

Extension trait implemented for all `MakeVisitor` implementations that
produce a visitor implementing `VisitOutput`.

```rust
pub trait MakeOutput<T, Out>
where
    Self: MakeVisitor<T> + crate::sealed::Sealed<(T, Out)>,
    <Self as >::Visitor: VisitOutput<Out> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn visit_with<F>(self: &Self, target: T, fields: &F) -> Out
where
    F: RecordFields { /* ... */ }
  ```
  Visits all fields in `fields` with a new visitor constructed from

##### Implementations

This trait is implemented for the following types:

- `M` with <T, Out, M>

#### Trait `VisitWrite`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Extension trait implemented by visitors to indicate that they write to an
`io::Write` instance, and allow access to that writer.

```rust
pub trait VisitWrite: VisitOutput<Result<(), io::Error>> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `writer`: Returns the writer that this visitor writes to.

##### Implementations

This trait is implemented for the following types:

- `Alt<V>` with <V>
- `Messages<V>` with <V>

#### Trait `VisitFmt`

Extension trait implemented by visitors to indicate that they write to a
`fmt::Write` instance, and allow access to that writer.

```rust
pub trait VisitFmt: VisitOutput<fmt::Result> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `writer`: Returns the formatter that this visitor writes to.

##### Implementations

This trait is implemented for the following types:

- `Alt<V>` with <V>
- `VisitDelimited<D, V>` with <D, V>
- `Messages<V>` with <V>
- `PrettyVisitor<''_>`
- `DefaultVisitor<''_>`
- `FieldFnVisitor<''a, F>` with <''a, F>

#### Trait `MakeExt`

Extension trait providing `MakeVisitor` combinators.

```rust
pub trait MakeExt<T>
where
    Self: MakeVisitor<T> + Sized + crate::sealed::Sealed<MakeExtMarker<T>> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn debug_alt(self: Self) -> debug::Alt<Self> { /* ... */ }
  ```
  Wraps `self` so that any `fmt::Debug` fields are recorded using the

- ```rust
  fn display_messages(self: Self) -> display::Messages<Self> { /* ... */ }
  ```
  Wraps `self` so that any string fields named "message" are recorded

- ```rust
  fn delimited<D>(self: Self, delimiter: D) -> delimited::Delimited<D, Self>
where
    D: AsRef<str> + Clone,
    <Self as >::Visitor: VisitFmt { /* ... */ }
  ```
  Wraps `self` so that when fields are formatted to a writer, they are

##### Implementations

This trait is implemented for the following types:

- `M` with <T, M>

### Re-exports

#### Re-export `Visit`

```rust
pub use tracing_core::field::Visit;
```

## Module `filter`

[`Layer`]s that control which spans and events are enabled by the wrapped
subscriber.

This module contains a number of types that provide implementations of
various strategies for filtering which spans and events are enabled. For
details on filtering spans and events using [`Layer`]s, see the
[`layer` module's documentation].

[`layer` module's documentation]: crate::layer#filtering-with-layers
[`Layer`]: crate::layer

```rust
pub mod filter { /* ... */ }
```

### Modules

## Module `targets`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "alloc"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]`

A [filter] that enables or disables spans and events based on their [target] and [level].

See [`Targets`] for details.

[target]: tracing_core::Metadata::target
[level]: tracing_core::Level
[filter]: crate::layer#filtering-with-layers

```rust
pub mod targets { /* ... */ }
```

### Types

#### Struct `Targets`

 A filter that enables or disables spans and events based on their [target]
 and [level].

 Targets are typically equal to the Rust module path of the code where the
 span or event was recorded, although they may be overridden.

 This type can be used for both [per-layer filtering][plf] (using its
 [`Filter`] implementation) and [global filtering][global] (using its
 [`Layer`] implementation).

 See the [documentation on filtering with layers][filtering] for details.

 # Filtering With `Targets`

 A `Targets` filter consists of one or more [target] prefixes, paired with
 [`LevelFilter`]s. If a span or event's [target] begins with one of those
 prefixes, and its [level] is at or below the [`LevelFilter`] enabled for
 that prefix, then the span or event will be enabled.

 This is similar to the behavior implemented by the [`env_logger` crate] in
 the `log` ecosystem.

 The [`EnvFilter`] type also provided by this crate is very similar to `Targets`,
 but is capable of a more sophisticated form of filtering where events may
 also be enabled or disabled based on the span they are recorded in.
 `Targets` can be thought of as a lighter-weight form of [`EnvFilter`] that
 can be used instead when this dynamic filtering is not required.

 # Examples

 A `Targets` filter can be constructed by programmatically adding targets and
 levels to enable:

 ```
 use tracing_subscriber::{filter, prelude::*};
 use tracing_core::Level;

 let filter = filter::Targets::new()
     // Enable the `INFO` level for anything in `my_crate`
     .with_target("my_crate", Level::INFO)
     // Enable the `DEBUG` level for a specific module.
     .with_target("my_crate::interesting_module", Level::DEBUG);

 // Build a new subscriber with the `fmt` layer using the `Targets`
 // filter we constructed above.
 tracing_subscriber::registry()
     .with(tracing_subscriber::fmt::layer())
     .with(filter)
     .init();
 ```

 [`LevelFilter::OFF`] can be used to disable a particular target:
 ```
 use tracing_subscriber::filter::{Targets, LevelFilter};
 use tracing_core::Level;

 let filter = Targets::new()
     .with_target("my_crate", Level::INFO)
     // Disable all traces from `annoying_module`.
     .with_target("my_crate::annoying_module", LevelFilter::OFF);
 # drop(filter);
 ```

 Alternatively, `Targets` implements [`std::str::FromStr`], allowing it to be
 parsed from a comma-delimited list of `target=level` pairs. For example:

 ```rust
 # fn main() -> Result<(), Box<dyn std::error::Error>> {
 use tracing_subscriber::filter;
 use tracing_core::Level;

 let filter = "my_crate=info,my_crate::interesting_module=trace,other_crate=debug"
     .parse::<filter::Targets>()?;

 // The parsed filter is identical to a filter constructed using `with_target`:
 assert_eq!(
     filter,
     filter::Targets::new()
         .with_target("my_crate", Level::INFO)
         .with_target("my_crate::interesting_module", Level::TRACE)
         .with_target("other_crate", Level::DEBUG)
 );
 # Ok(()) }
 ```

 This is particularly useful when the list of enabled targets is configurable
 by the user at runtime.

 The `Targets` filter can be used as a [per-layer filter][plf] *and* as a
 [global filter][global]:

 ```rust
 use tracing_subscriber::{
     fmt,
     filter::{Targets, LevelFilter},
     prelude::*,
 };
 use tracing_core::Level;
 use std::{sync::Arc, fs::File};
 # fn docs() -> Result<(), Box<dyn std::error::Error>> {

 // A layer that logs events to stdout using the human-readable "pretty"
 // format.
 let stdout_log = fmt::layer().pretty();

 // A layer that logs events to a file, using the JSON format.
 let file = File::create("debug_log.json")?;
 let debug_log = fmt::layer()
     .with_writer(Arc::new(file))
     .json();

 tracing_subscriber::registry()
     // Only log INFO and above to stdout, unless the span or event
     // has the `my_crate::cool_module` target prefix.
     .with(stdout_log
         .with_filter(
             Targets::default()
                 .with_target("my_crate::cool_module", Level::DEBUG)
                 .with_default(Level::INFO)
        )
     )
     // Log everything enabled by the global filter to `debug_log.json`.
     .with(debug_log)
     // Configure a global filter for the whole subscriber stack. This will
     // control what spans and events are recorded by both the `debug_log`
     // and the `stdout_log` layers, and `stdout_log` will *additionally* be
     // filtered by its per-layer filter.
     .with(
         Targets::default()
             .with_target("my_crate", Level::TRACE)
             .with_target("other_crate", Level::INFO)
             .with_target("other_crate::annoying_module", LevelFilter::OFF)
             .with_target("third_crate", Level::DEBUG)
     ).init();
 # Ok(()) }
```

 [target]: tracing_core::Metadata::target
 [level]: tracing_core::Level
 [`Filter`]: crate::layer::Filter
 [`Layer`]: crate::layer::Layer
 [plf]: crate::layer#per-layer-filtering
 [global]: crate::layer#global-filtering
 [filtering]: crate::layer#filtering-with-layers
 [`env_logger` crate]: https://docs.rs/env_logger/0.9.0/env_logger/index.html#enabling-logging
 [`EnvFilter`]: crate::filter::EnvFilter

```rust
pub struct Targets(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Returns a new `Targets` filter.

- ```rust
  pub fn with_target</* synthetic */ impl Into<String>: Into<String>, /* synthetic */ impl Into<LevelFilter>: Into<LevelFilter>>(self: Self, target: impl Into<String>, level: impl Into<LevelFilter>) -> Self { /* ... */ }
  ```
  Enables spans and events with [target]s starting with the provided target

- ```rust
  pub fn with_targets<T, L, /* synthetic */ impl IntoIterator<Item = (T, L)>: IntoIterator<Item = (T, L)>>(self: Self, targets: impl IntoIterator<Item = (T, L)>) -> Self
where
    String: From<T>,
    LevelFilter: From<L> { /* ... */ }
  ```
  Adds [target]s from an iterator of [target]-[`LevelFilter`] pairs to this filter.

- ```rust
  pub fn with_default</* synthetic */ impl Into<LevelFilter>: Into<LevelFilter>>(self: Self, level: impl Into<LevelFilter>) -> Self { /* ... */ }
  ```
  Sets the default level to enable for spans and events whose targets did

- ```rust
  pub fn default_level(self: &Self) -> Option<LevelFilter> { /* ... */ }
  ```
  Returns the default level for this filter, if one is set.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_> { /* ... */ }
  ```
  Returns an iterator over the [target]-[`LevelFilter`] pairs in this filter.

- ```rust
  pub fn would_enable(self: &Self, target: &str, level: &Level) -> bool { /* ... */ }
  ```
  Returns whether a [target]-[`Level`] pair would be enabled

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Filter**
  - ```rust
    fn enabled(self: &Self, metadata: &Metadata<''_>, _: &layer::Context<''_, S>) -> bool { /* ... */ }
    ```

  - ```rust
    fn callsite_enabled(self: &Self, metadata: &''static Metadata<''static>) -> Interest { /* ... */ }
    ```

  - ```rust
    fn max_level_hint(self: &Self) -> Option<LevelFilter> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **StructuralPartialEq**
- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = (T, L)>>(iter: I) -> Self { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Targets { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Targets { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Targets) -> bool { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = (T, L)>>(self: &mut Self, iter: I) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Layer**
  - ```rust
    fn enabled(self: &Self, metadata: &Metadata<''_>, _: layer::Context<''_, S>) -> bool { /* ... */ }
    ```

  - ```rust
    fn register_callsite(self: &Self, metadata: &''static Metadata<''static>) -> Interest { /* ... */ }
    ```

- **FilterExt**
#### Struct `IntoIter`

An owning iterator over the [target]-[level] pairs of a `Targets` filter.

This struct is created by the `IntoIterator` trait implementation of [`Targets`].

# Examples

Merge the targets from one `Targets` with another:

```
use tracing_subscriber::filter::Targets;
use tracing_core::Level;

let mut filter = Targets::new().with_target("my_crate", Level::INFO);
let overrides = Targets::new().with_target("my_crate::interesting_module", Level::DEBUG);

filter.extend(overrides);
# drop(filter);
```

[target]: tracing_core::Metadata::target
[level]: tracing_core::Level

```rust
pub struct IntoIter(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `Iter`

A borrowing iterator over the [target]-[level] pairs of a `Targets` filter.

This struct is created by [`iter`] method of [`Targets`], or from the `IntoIterator`
implementation for `&Targets`.

[target]: tracing_core::Metadata::target
[level]: tracing_core::Level
[`iter`]: Targets::iter

```rust
pub struct Iter<''a>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Re-exports

#### Re-export `LevelFilter`

```rust
pub use self::level::LevelFilter;
```

#### Re-export `ParseError`

```rust
pub use self::level::ParseError as LevelParseError;
```

#### Re-export `Targets`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "alloc"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]`

```rust
pub use self::targets::Targets;
```

#### Re-export `ParseError`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "alloc"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]`

```rust
pub use self::directive::ParseError;
```

#### Re-export `self::layer_filters::*`

**Attributes:**

- `#[<cfg>(all(feature = "registry", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "registry", feature = "std"))))]`

```rust
pub use self::layer_filters::*;
```

#### Re-export `self::filter_fn::*`

```rust
pub use self::filter_fn::*;
```

## Module `prelude`

The `tracing-subscriber` prelude.

This brings into scope a number of extension traits that define methods on
types defined here and in other crates.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `MakeExt`

```rust
pub use crate::field::MakeExt as __tracing_subscriber_field_MakeExt;
```

#### Re-export `RecordFields`

```rust
pub use crate::field::RecordFields as __tracing_subscriber_field_RecordFields;
```

#### Re-export `Layer`

```rust
pub use crate::layer::Layer as __tracing_subscriber_Layer;
```

#### Re-export `SubscriberExt`

```rust
pub use crate::layer::SubscriberExt as __tracing_subscriber_SubscriberExt;
```

#### Re-export `SubscriberInitExt`

```rust
pub use crate::util::SubscriberInitExt as _;
```

#### Re-export `MakeWriterExt`

**Attributes:**

- `#[<cfg>(all(feature = "fmt", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

```rust
pub use crate::fmt::writer::MakeWriterExt as _;
```

## Module `registry`

Storage for span data shared by multiple [`Layer`]s.

## Using the Span Registry

This module provides the [`Registry`] type, a [`Subscriber`] implementation
which tracks per-span data and exposes it to [`Layer`]s. When a `Registry`
is used as the base `Subscriber` of a `Layer` stack, the
[`layer::Context`][ctx] type will provide methods allowing `Layer`s to
[look up span data][lookup] stored in the registry. While [`Registry`] is a
reasonable default for storing spans and events, other stores that implement
[`LookupSpan`] and [`Subscriber`] themselves (with [`SpanData`] implemented
by the per-span data they store) can be used as a drop-in replacement.

For example, we might create a `Registry` and add multiple `Layer`s like so:
```rust
use tracing_subscriber::{registry::Registry, Layer, prelude::*};
# use tracing_core::Subscriber;
# pub struct FooLayer {}
# pub struct BarLayer {}
# impl<S: Subscriber> Layer<S> for FooLayer {}
# impl<S: Subscriber> Layer<S> for BarLayer {}
# impl FooLayer {
# fn new() -> Self { Self {} }
# }
# impl BarLayer {
# fn new() -> Self { Self {} }
# }

let subscriber = Registry::default()
    .with(FooLayer::new())
    .with(BarLayer::new());
```

If a type implementing `Layer` depends on the functionality of a `Registry`
implementation, it should bound its `Subscriber` type parameter with the
[`LookupSpan`] trait, like so:

```rust
use tracing_subscriber::{registry, Layer};
use tracing_core::Subscriber;

pub struct MyLayer {
    // ...
}

impl<S> Layer<S> for MyLayer
where
    S: Subscriber + for<'a> registry::LookupSpan<'a>,
{
    // ...
}
```
When this bound is added, the `Layer` implementation will be guaranteed
access to the [`Context`][ctx] methods, such as [`Context::span`][lookup], that
require the root subscriber to be a registry.

[`Layer`]: crate::layer::Layer
[`Subscriber`]: tracing_core::Subscriber
[ctx]: crate::layer::Context
[lookup]: crate::layer::Context::span()

```rust
pub mod registry { /* ... */ }
```

### Types

#### Struct `SpanRef`

A reference to [span data] and the associated [registry].

This type implements all the same methods as [`SpanData`], and provides
additional methods for querying the registry based on values from the span.

[registry]: LookupSpan

```rust
pub struct SpanRef<''a, R: LookupSpan<''a>> {
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
  pub fn id(self: &Self) -> Id { /* ... */ }
  ```
  Returns this span's ID.

- ```rust
  pub fn metadata(self: &Self) -> &''static Metadata<''static> { /* ... */ }
  ```
  Returns a static reference to the span's metadata.

- ```rust
  pub fn name(self: &Self) -> &''static str { /* ... */ }
  ```
  Returns the span's name,

- ```rust
  pub fn fields(self: &Self) -> &FieldSet { /* ... */ }
  ```
  Returns a list of [fields] defined by the span.

- ```rust
  pub fn parent(self: &Self) -> Option<Self> { /* ... */ }
  ```
  Returns a `SpanRef` describing this span's parent, or `None` if this

- ```rust
  pub fn scope(self: &Self) -> Scope<''a, R> { /* ... */ }
  ```
  Returns an iterator over all parents of this span, starting with this span,

- ```rust
  pub fn extensions(self: &Self) -> Extensions<''_> { /* ... */ }
  ```
  Returns a reference to this span's `Extensions`.

- ```rust
  pub fn extensions_mut(self: &Self) -> ExtensionsMut<''_> { /* ... */ }
  ```
  Returns a mutable reference to this span's `Extensions`.

###### Trait Implementations

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
#### Struct `Scope`

An iterator over the parents of a span, ordered from leaf to root.

This is returned by the [`SpanRef::scope`] method.

```rust
pub struct Scope<''a, R> {
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
  pub fn from_root(self: Self) -> ScopeFromRoot<''a, R> { /* ... */ }
  ```
  Flips the order of the iterator, so that it is ordered from root to leaf.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `ScopeFromRoot`

**Attributes:**

- `#[<cfg>(any(feature = "alloc", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]`

An iterator over the parents of a span, ordered from root to leaf.

This is returned by the [`Scope::from_root`] method.

```rust
pub struct ScopeFromRoot<''a, R> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
### Traits

#### Trait `LookupSpan`

Provides access to stored span data.

Subscribers which store span data and associate it with span IDs should
implement this trait; if they do, any [`Layer`]s wrapping them can look up
metadata via the [`Context`] type's [`span()`] method.

[`Layer`]: super::layer::Layer
[`Context`]: super::layer::Context
[`span()`]: super::layer::Context::span

```rust
pub trait LookupSpan<''a> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Data`: The type of span data stored in this registry.

###### Required Methods

- `span_data`: Returns the [`SpanData`] for a given `Id`, if it exists.

##### Provided Methods

- ```rust
  fn span(self: &''a Self, id: &Id) -> Option<SpanRef<''a, Self>>
where
    Self: Sized { /* ... */ }
  ```
  Returns a [`SpanRef`] for the span with the given `Id`, if it exists.

- ```rust
  fn register_filter(self: &mut Self) -> FilterId { /* ... */ }
  ```
  Registers a [`Filter`] for [per-layer filtering] with this

##### Implementations

This trait is implemented for the following types:

- `Registry` with <''a>
- `Layered<L, S>` with <''a, L, S>
- `Subscriber<N, E, F, W>` with <''a, N, E, F, W>

#### Trait `SpanData`

A stored representation of data associated with a span.

```rust
pub trait SpanData<''a> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `id`: Returns this span's ID.
- `metadata`: Returns a reference to the span's `Metadata`.
- `parent`: Returns a reference to the ID
- `extensions`: Returns a reference to this span's `Extensions`.
- `extensions_mut`: Returns a mutable reference to this span's `Extensions`.

##### Provided Methods

- ```rust
  fn is_enabled_for(self: &Self, filter: FilterId) -> bool { /* ... */ }
  ```
  Returns `true` if this span is enabled for the [per-layer filter][plf]

##### Implementations

This trait is implemented for the following types:

- `Data<''a>` with <''a>

### Re-exports

#### Re-export `Extensions`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use extensions::Extensions;
```

#### Re-export `ExtensionsMut`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use extensions::ExtensionsMut;
```

#### Re-export `Data`

**Attributes:**

- `#[<cfg>(all(feature = "registry", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "registry", feature = "std"))))]`

```rust
pub use sharded::Data;
```

#### Re-export `Registry`

**Attributes:**

- `#[<cfg>(all(feature = "registry", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "registry", feature = "std"))))]`

```rust
pub use sharded::Registry;
```

## Module `layer`

 The [`Layer`] trait, a composable abstraction for building [`Subscriber`]s.

 The [`Subscriber`] trait in `tracing-core` represents the _complete_ set of
 functionality required to consume `tracing` instrumentation. This means that
 a single `Subscriber` instance is a self-contained implementation of a
 complete strategy for collecting traces; but it _also_ means that the
 `Subscriber` trait cannot easily be composed with other `Subscriber`s.

 In particular, [`Subscriber`]s are responsible for generating [span IDs] and
 assigning them to spans. Since these IDs must uniquely identify a span
 within the context of the current trace, this means that there may only be
 a single `Subscriber` for a given thread at any point in time &mdash;
 otherwise, there would be no authoritative source of span IDs.

 On the other hand, the majority of the [`Subscriber`] trait's functionality
 is composable: any number of subscribers may _observe_ events, span entry
 and exit, and so on, provided that there is a single authoritative source of
 span IDs. The [`Layer`] trait represents this composable subset of the
 [`Subscriber`] behavior; it can _observe_ events and spans, but does not
 assign IDs.

 # Composing Layers

 Since a [`Layer`] does not implement a complete strategy for collecting
 traces, it must be composed with a `Subscriber` in order to be used. The
 [`Layer`] trait is generic over a type parameter (called `S` in the trait
 definition), representing the types of `Subscriber` they can be composed
 with. Thus, a [`Layer`] may be implemented that will only compose with a
 particular `Subscriber` implementation, or additional trait bounds may be
 added to constrain what types implementing `Subscriber` a `Layer` can wrap.

 `Layer`s may be added to a `Subscriber` by using the [`SubscriberExt::with`]
 method, which is provided by `tracing-subscriber`'s [prelude]. This method
 returns a [`Layered`] struct that implements `Subscriber` by composing the
 `Layer` with the `Subscriber`.

 For example:
 ```rust
 use tracing_subscriber::Layer;
 use tracing_subscriber::prelude::*;
 use tracing::Subscriber;

 pub struct MyLayer {
     // ...
 }

 impl<S: Subscriber> Layer<S> for MyLayer {
     // ...
 }

 pub struct MySubscriber {
     // ...
 }

 # use tracing_core::{span::{Id, Attributes, Record}, Metadata, Event};
 impl Subscriber for MySubscriber {
     // ...
 #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(1) }
 #   fn record(&self, _: &Id, _: &Record) {}
 #   fn event(&self, _: &Event) {}
 #   fn record_follows_from(&self, _: &Id, _: &Id) {}
 #   fn enabled(&self, _: &Metadata) -> bool { false }
 #   fn enter(&self, _: &Id) {}
 #   fn exit(&self, _: &Id) {}
 }
 # impl MyLayer {
 # fn new() -> Self { Self {} }
 # }
 # impl MySubscriber {
 # fn new() -> Self { Self { }}
 # }

 let subscriber = MySubscriber::new()
     .with(MyLayer::new());

 tracing::subscriber::set_global_default(subscriber);
 ```

 Multiple `Layer`s may be composed in the same manner:
 ```rust
 # use tracing_subscriber::{Layer, layer::SubscriberExt};
 # use tracing::Subscriber;
 pub struct MyOtherLayer {
     // ...
 }

 impl<S: Subscriber> Layer<S> for MyOtherLayer {
     // ...
 }

 pub struct MyThirdLayer {
     // ...
 }

 impl<S: Subscriber> Layer<S> for MyThirdLayer {
     // ...
 }
 # pub struct MyLayer {}
 # impl<S: Subscriber> Layer<S> for MyLayer {}
 # pub struct MySubscriber { }
 # use tracing_core::{span::{Id, Attributes, Record}, Metadata, Event};
 # impl Subscriber for MySubscriber {
 #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(1) }
 #   fn record(&self, _: &Id, _: &Record) {}
 #   fn event(&self, _: &Event) {}
 #   fn record_follows_from(&self, _: &Id, _: &Id) {}
 #   fn enabled(&self, _: &Metadata) -> bool { false }
 #   fn enter(&self, _: &Id) {}
 #   fn exit(&self, _: &Id) {}
 }
 # impl MyLayer {
 # fn new() -> Self { Self {} }
 # }
 # impl MyOtherLayer {
 # fn new() -> Self { Self {} }
 # }
 # impl MyThirdLayer {
 # fn new() -> Self { Self {} }
 # }
 # impl MySubscriber {
 # fn new() -> Self { Self { }}
 # }

 let subscriber = MySubscriber::new()
     .with(MyLayer::new())
     .with(MyOtherLayer::new())
     .with(MyThirdLayer::new());

 tracing::subscriber::set_global_default(subscriber);
 ```

 The [`Layer::with_subscriber`] constructs the [`Layered`] type from a
 [`Layer`] and [`Subscriber`], and is called by [`SubscriberExt::with`]. In
 general, it is more idiomatic to use [`SubscriberExt::with`], and treat
 [`Layer::with_subscriber`] as an implementation detail, as `with_subscriber`
 calls must be nested, leading to less clear code for the reader.

 ## Runtime Configuration With `Layer`s

 In some cases, a particular [`Layer`] may be enabled or disabled based on
 runtime configuration. This can introduce challenges, because the type of a
 layered [`Subscriber`] depends on which layers are added to it: if an `if`
 or `match` expression adds some [`Layer`] implementation in one branch,
 and other layers in another, the [`Subscriber`] values returned by those
 branches will have different types. For example, the following _will not_
 work:

 ```compile_fail
 # fn docs() -> Result<(), Box<dyn std::error::Error + 'static>> {
 # struct Config {
 #    is_prod: bool,
 #    path: &'static str,
 # }
 # let cfg = Config { is_prod: false, path: "debug.log" };
 use std::fs::File;
 use tracing_subscriber::{Registry, prelude::*};

 let stdout_log = tracing_subscriber::fmt::layer().pretty();
 let subscriber = Registry::default().with(stdout_log);

 // The compile error will occur here because the if and else
 // branches have different (and therefore incompatible) types.
 let subscriber = if cfg.is_prod {
     let file = File::create(cfg.path)?;
     let layer = tracing_subscriber::fmt::layer()
         .json()
         .with_writer(Arc::new(file));
     layer.with(subscriber)
 } else {
     layer
 };

 tracing::subscriber::set_global_default(subscriber)
     .expect("Unable to set global subscriber");
 # Ok(()) }
 ```

 However, a [`Layer`] wrapped in an [`Option`] [also implements the `Layer`
 trait][option-impl]. This allows individual layers to be enabled or disabled at
 runtime while always producing a [`Subscriber`] of the same type. For
 example:

 ```
 # fn docs() -> Result<(), Box<dyn std::error::Error + 'static>> {
 # struct Config {
 #    is_prod: bool,
 #    path: &'static str,
 # }
 # let cfg = Config { is_prod: false, path: "debug.log" };
 use std::fs::File;
 use tracing_subscriber::{Registry, prelude::*};

 let stdout_log = tracing_subscriber::fmt::layer().pretty();
 let subscriber = Registry::default().with(stdout_log);

 // if `cfg.is_prod` is true, also log JSON-formatted logs to a file.
 let json_log = if cfg.is_prod {
     let file = File::create(cfg.path)?;
     let json_log = tracing_subscriber::fmt::layer()
         .json()
         .with_writer(file);
     Some(json_log)
 } else {
     None
 };

 // If `cfg.is_prod` is false, then `json` will be `None`, and this layer
 // will do nothing. However, the subscriber will still have the same type
 // regardless of whether the `Option`'s value is `None` or `Some`.
 let subscriber = subscriber.with(json_log);

 tracing::subscriber::set_global_default(subscriber)
    .expect("Unable to set global subscriber");
 # Ok(()) }
 ```

 If a [`Layer`] may be one of several different types, note that [`Box<dyn
 Layer<S> + Send + Sync>` implements `Layer`][box-impl].
 This may be used to erase the type of a [`Layer`].

 For example, a function that configures a [`Layer`] to log to one of
 several outputs might return a `Box<dyn Layer<S> + Send + Sync + 'static>`:
 ```
 use tracing_subscriber::{
     Layer,
     registry::LookupSpan,
     prelude::*,
 };
 use std::{path::PathBuf, fs::File, io};

 /// Configures whether logs are emitted to a file, to stdout, or to stderr.
 pub enum LogConfig {
     File(PathBuf),
     Stdout,
     Stderr,
 }

 impl LogConfig {
     pub fn layer<S>(self) -> Box<dyn Layer<S> + Send + Sync + 'static>
     where
         S: tracing_core::Subscriber,
         for<'a> S: LookupSpan<'a>,
     {
         // Shared configuration regardless of where logs are output to.
         let fmt = tracing_subscriber::fmt::layer()
             .with_target(true)
             .with_thread_names(true);

         // Configure the writer based on the desired log target:
         match self {
             LogConfig::File(path) => {
                 let file = File::create(path).expect("failed to create log file");
                 Box::new(fmt.with_writer(file))
             },
             LogConfig::Stdout => Box::new(fmt.with_writer(io::stdout)),
             LogConfig::Stderr => Box::new(fmt.with_writer(io::stderr)),
         }
     }
 }

 let config = LogConfig::Stdout;
 tracing_subscriber::registry()
     .with(config.layer())
     .init();
 ```

 The [`Layer::boxed`] method is provided to make boxing a `Layer`
 more convenient, but [`Box::new`] may be used as well.

 When the number of `Layer`s varies at runtime, note that a
 [`Vec<L> where L: Layer` also implements `Layer`][vec-impl]. This
 can be used to add a variable number of `Layer`s to a `Subscriber`:

 ```
 use tracing_subscriber::{Layer, prelude::*};
 struct MyLayer {
     // ...
 }
 # impl MyLayer { fn new() -> Self { Self {} }}

 impl<S: tracing_core::Subscriber> Layer<S> for MyLayer {
     // ...
 }

 /// Returns how many layers we need
 fn how_many_layers() -> usize {
     // ...
     # 3
 }

 // Create a variable-length `Vec` of layers
 let mut layers = Vec::new();
 for _ in 0..how_many_layers() {
     layers.push(MyLayer::new());
 }

 tracing_subscriber::registry()
     .with(layers)
     .init();
 ```

 If a variable number of `Layer` is needed and those `Layer`s have
 different types, a `Vec` of [boxed `Layer` trait objects][box-impl] may
 be used. For example:

 ```
 use tracing_subscriber::{filter::LevelFilter, Layer, prelude::*};
 use std::fs::File;
 # fn main() -> Result<(), Box<dyn std::error::Error>> {
 struct Config {
     enable_log_file: bool,
     enable_stdout: bool,
     enable_stderr: bool,
     // ...
 }
 # impl Config {
 #    fn from_config_file()-> Result<Self, Box<dyn std::error::Error>> {
 #         // don't enable the log file so that the example doesn't actually create it
 #         Ok(Self { enable_log_file: false, enable_stdout: true, enable_stderr: true })
 #    }
 # }

 let cfg = Config::from_config_file()?;

 // Based on our dynamically loaded config file, create any number of layers:
 let mut layers = Vec::new();

 if cfg.enable_log_file {
     let file = File::create("myapp.log")?;
     let layer = tracing_subscriber::fmt::layer()
         .with_thread_names(true)
         .with_target(true)
         .json()
         .with_writer(file)
         // Box the layer as a type-erased trait object, so that it can
         // be pushed to the `Vec`.
         .boxed();
     layers.push(layer);
 }

 if cfg.enable_stdout {
     let layer = tracing_subscriber::fmt::layer()
         .pretty()
         .with_filter(LevelFilter::INFO)
         // Box the layer as a type-erased trait object, so that it can
         // be pushed to the `Vec`.
         .boxed();
     layers.push(layer);
 }

 if cfg.enable_stdout {
     let layer = tracing_subscriber::fmt::layer()
         .with_target(false)
         .with_filter(LevelFilter::WARN)
         // Box the layer as a type-erased trait object, so that it can
         // be pushed to the `Vec`.
         .boxed();
     layers.push(layer);
 }

 tracing_subscriber::registry()
     .with(layers)
     .init();
# Ok(()) }
 ```

 Finally, if the number of layers _changes_ at runtime, a `Vec` of
 subscribers can be used alongside the [`reload`](crate::reload) module to
 add or remove subscribers dynamically at runtime.

 [option-impl]: Layer#impl-Layer<S>-for-Option<L>
 [box-impl]: Layer#impl-Layer%3CS%3E-for-Box%3Cdyn%20Layer%3CS%3E%20+%20Send%20+%20Sync%3E
 [vec-impl]: Layer#impl-Layer<S>-for-Vec<L>
 [prelude]: crate::prelude

 # Recording Traces

 The [`Layer`] trait defines a set of methods for consuming notifications from
 tracing instrumentation, which are generally equivalent to the similarly
 named methods on [`Subscriber`]. Unlike [`Subscriber`], the methods on
 `Layer` are additionally passed a [`Context`] type, which exposes additional
 information provided by the wrapped subscriber (such as [the current span])
 to the layer.

 # Filtering with `Layer`s

 As well as strategies for handling trace events, the `Layer` trait may also
 be used to represent composable _filters_. This allows the determination of
 what spans and events should be recorded to be decoupled from _how_ they are
 recorded: a filtering layer can be applied to other layers or
 subscribers. `Layer`s can be used to implement _global filtering_, where a
 `Layer` provides a filtering strategy for the entire subscriber.
 Additionally, individual recording `Layer`s or sets of `Layer`s may be
 combined with _per-layer filters_ that control what spans and events are
 recorded by those layers.

 ## Global Filtering

 A `Layer` that implements a filtering strategy should override the
 [`register_callsite`] and/or [`enabled`] methods. It may also choose to implement
 methods such as [`on_enter`], if it wishes to filter trace events based on
 the current span context.

 Note that the [`Layer::register_callsite`] and [`Layer::enabled`] methods
 determine whether a span or event is enabled *globally*. Thus, they should
 **not** be used to indicate whether an individual layer wishes to record a
 particular span or event. Instead, if a layer is only interested in a subset
 of trace data, but does *not* wish to disable other spans and events for the
 rest of the layer stack should ignore those spans and events in its
 notification methods.

 The filtering methods on a stack of `Layer`s are evaluated in a top-down
 order, starting with the outermost `Layer` and ending with the wrapped
 [`Subscriber`]. If any layer returns `false` from its [`enabled`] method, or
 [`Interest::never()`] from its [`register_callsite`] method, filter
 evaluation will short-circuit and the span or event will be disabled.

 ### Enabling Interest

 Whenever an tracing event (or span) is emitted, it goes through a number of
 steps to determine how and how much it should be processed. The earlier an
 event is disabled, the less work has to be done to process the event, so
 `Layer`s that implement filtering should attempt to disable unwanted
 events as early as possible. In order, each event checks:

 - [`register_callsite`], once per callsite (roughly: once per time that
   `event!` or `span!` is written in the source code; this is cached at the
   callsite). See [`Subscriber::register_callsite`] and
   [`tracing_core::callsite`] for a summary of how this behaves.
 - [`enabled`], once per emitted event (roughly: once per time that `event!`
   or `span!` is *executed*), and only if `register_callsite` registers an
   [`Interest::sometimes`]. This is the main customization point to globally
   filter events based on their [`Metadata`]. If an event can be disabled
   based only on [`Metadata`], it should be, as this allows the construction
   of the actual `Event`/`Span` to be skipped.
 - For events only (and not spans), [`event_enabled`] is called just before
   processing the event. This gives layers one last chance to say that
   an event should be filtered out, now that the event's fields are known.

 ## Per-Layer Filtering

 **Note**: per-layer filtering APIs currently require the [`"registry"` crate
 feature flag][feat] to be enabled.

 Sometimes, it may be desirable for one `Layer` to record a particular subset
 of spans and events, while a different subset of spans and events are
 recorded by other `Layer`s. For example:

 - A layer that records metrics may wish to observe only events including
   particular tracked values, while a logging layer ignores those events.
 - If recording a distributed trace is expensive, it might be desirable to
   only send spans with `INFO` and lower verbosity to the distributed tracing
   system, while logging more verbose spans to a file.
 - Spans and events with a particular target might be recorded differently
   from others, such as by generating an HTTP access log from a span that
   tracks the lifetime of an HTTP request.

 The [`Filter`] trait is used to control what spans and events are
 observed by an individual `Layer`, while still allowing other `Layer`s to
 potentially record them. The [`Layer::with_filter`] method combines a
 `Layer` with a [`Filter`], returning a [`Filtered`] layer.

 This crate's [`filter`] module provides a number of types which implement
 the [`Filter`] trait, such as [`LevelFilter`], [`Targets`], and
 [`FilterFn`]. These [`Filter`]s provide ready-made implementations of
 common forms of filtering. For custom filtering policies, the [`FilterFn`]
 and [`DynFilterFn`] types allow implementing a [`Filter`] with a closure or
 function pointer. In addition, when more control is required, the [`Filter`]
 trait may also be implemented for user-defined types.

 //! [`Option<Filter>`] also implements [`Filter`], which allows for an optional
 filter. [`None`](Option::None) filters out _nothing_ (that is, allows
 everything through). For example:

 ```rust
 # use tracing_subscriber::{filter::filter_fn, Layer};
 # use tracing_core::{Metadata, subscriber::Subscriber};
 # struct MyLayer<S>(std::marker::PhantomData<S>);
 # impl<S> MyLayer<S> { fn new() -> Self { Self(std::marker::PhantomData)} }
 # impl<S: Subscriber> Layer<S> for MyLayer<S> {}
 # fn my_filter(_: &str) -> impl Fn(&Metadata) -> bool { |_| true  }
 fn setup_tracing<S: Subscriber>(filter_config: Option<&str>) {
     let layer = MyLayer::<S>::new()
         .with_filter(filter_config.map(|config| filter_fn(my_filter(config))));
 //...
 }
 ```

 <pre class="compile_fail" style="white-space:normal;font:inherit;">
     <strong>Warning</strong>: Currently, the <a href="../struct.Registry.html">
     <code>Registry</code></a> type defined in this crate is the only root
     <code>Subscriber</code> capable of supporting <code>Layer</code>s with
     per-layer filters. In the future, new APIs will be added to allow other
     root <code>Subscriber</code>s to support per-layer filters.
 </pre>

 For example, to generate an HTTP access log based on spans with
 the `http_access` target, while logging other spans and events to
 standard out, a [`Filter`] can be added to the access log layer:

 ```
 use tracing_subscriber::{filter, prelude::*};

 // Generates an HTTP access log.
 let access_log = // ...
     # filter::LevelFilter::INFO;

 // Add a filter to the access log layer so that it only observes
 // spans and events with the `http_access` target.
 let access_log = access_log.with_filter(filter::filter_fn(|metadata| {
     // Returns `true` if and only if the span or event's target is
     // "http_access".
     metadata.target() == "http_access"
 }));

 // A general-purpose logging layer.
 let fmt_layer = tracing_subscriber::fmt::layer();

 // Build a subscriber that combines the access log and stdout log
 // layers.
 tracing_subscriber::registry()
     .with(fmt_layer)
     .with(access_log)
     .init();
 ```

 Multiple layers can have their own, separate per-layer filters. A span or
 event will be recorded if it is enabled by _any_ per-layer filter, but it
 will be skipped by the layers whose filters did not enable it. Building on
 the previous example:

 ```
 use tracing_subscriber::{filter::{filter_fn, LevelFilter}, prelude::*};

 let access_log = // ...
     # LevelFilter::INFO;
 let fmt_layer = tracing_subscriber::fmt::layer();

 tracing_subscriber::registry()
     // Add the filter for the "http_access" target to the access
     // log layer, like before.
     .with(access_log.with_filter(filter_fn(|metadata| {
         metadata.target() == "http_access"
     })))
     // Add a filter for spans and events with the INFO level
     // and below to the logging layer.
     .with(fmt_layer.with_filter(LevelFilter::INFO))
     .init();

 // Neither layer will observe this event
 tracing::debug!(does_anyone_care = false, "a tree fell in the forest");

 // This event will be observed by the logging layer, but not
 // by the access log layer.
 tracing::warn!(dose_roentgen = %3.8, "not great, but not terrible");

 // This event will be observed only by the access log layer.
 tracing::trace!(target: "http_access", "HTTP request started");

 // Both layers will observe this event.
 tracing::error!(target: "http_access", "HTTP request failed with a very bad error!");
 ```

 A per-layer filter can be applied to multiple [`Layer`]s at a time, by
 combining them into a [`Layered`] layer using [`Layer::and_then`], and then
 calling [`Layer::with_filter`] on the resulting [`Layered`] layer.

 Consider the following:
 - `layer_a` and `layer_b`, which should only receive spans and events at
   the [`INFO`] [level] and above.
 - A third layer, `layer_c`, which should receive spans and events at
   the [`DEBUG`] [level] as well.

 The layers and filters would be composed thusly:

 ```
 use tracing_subscriber::{filter::LevelFilter, prelude::*};

 let layer_a = // ...
 # LevelFilter::INFO;
 let layer_b =  // ...
 # LevelFilter::INFO;
 let layer_c =  // ...
 # LevelFilter::INFO;

 let info_layers = layer_a
     // Combine `layer_a` and `layer_b` into a `Layered` layer:
     .and_then(layer_b)
     // ...and then add an `INFO` `LevelFilter` to that layer:
     .with_filter(LevelFilter::INFO);

 tracing_subscriber::registry()
     // Add `layer_c` with a `DEBUG` filter.
     .with(layer_c.with_filter(LevelFilter::DEBUG))
     .with(info_layers)
     .init();
```

 If a [`Filtered`] [`Layer`] is combined with another [`Layer`]
 [`Layer::and_then`], and a filter is added to the [`Layered`] layer, that
 layer will be filtered by *both* the inner filter and the outer filter.
 Only spans and events that are enabled by *both* filters will be
 observed by that layer. This can be used to implement complex filtering
 trees.

 As an example, consider the following constraints:
 - Suppose that a particular [target] is used to indicate events that
   should be counted as part of a metrics system, which should be only
   observed by a layer that collects metrics.
 - A log of high-priority events ([`INFO`] and above) should be logged
   to stdout, while more verbose events should be logged to a debugging log file.
 - Metrics-focused events should *not* be included in either log output.

 In that case, it is possible to apply a filter to both logging layers to
 exclude the metrics events, while additionally adding a [`LevelFilter`]
 to the stdout log:

 ```
 # // wrap this in a function so we don't actually create `debug.log` when
 # // running the doctests..
 # fn docs() -> Result<(), Box<dyn std::error::Error + 'static>> {
 use tracing_subscriber::{filter, prelude::*};
 use std::{fs::File, sync::Arc};

 // A layer that logs events to stdout using the human-readable "pretty"
 // format.
 let stdout_log = tracing_subscriber::fmt::layer()
     .pretty();

 // A layer that logs events to a file.
 let file = File::create("debug.log")?;
 let debug_log = tracing_subscriber::fmt::layer()
     .with_writer(Arc::new(file));

 // A layer that collects metrics using specific events.
 let metrics_layer = /* ... */ filter::LevelFilter::INFO;

 tracing_subscriber::registry()
     .with(
         stdout_log
             // Add an `INFO` filter to the stdout logging layer
             .with_filter(filter::LevelFilter::INFO)
             // Combine the filtered `stdout_log` layer with the
             // `debug_log` layer, producing a new `Layered` layer.
             .and_then(debug_log)
             // Add a filter to *both* layers that rejects spans and
             // events whose targets start with `metrics`.
             .with_filter(filter::filter_fn(|metadata| {
                 !metadata.target().starts_with("metrics")
             }))
     )
     .with(
         // Add a filter to the metrics label that *only* enables
         // events whose targets start with `metrics`.
         metrics_layer.with_filter(filter::filter_fn(|metadata| {
             metadata.target().starts_with("metrics")
         }))
     )
     .init();

 // This event will *only* be recorded by the metrics layer.
 tracing::info!(target: "metrics::cool_stuff_count", value = 42);

 // This event will only be seen by the debug log file layer:
 tracing::debug!("this is a message, and part of a system of messages");

 // This event will be seen by both the stdout log layer *and*
 // the debug log file layer, but not by the metrics layer.
 tracing::warn!("the message is a warning about danger!");
 # Ok(()) }
 ```

 [`Subscriber`]: tracing_core::subscriber::Subscriber
 [span IDs]: tracing_core::span::Id
 [the current span]: Context::current_span
 [`register_callsite`]: Layer::register_callsite
 [`enabled`]: Layer::enabled
 [`event_enabled`]: Layer::event_enabled
 [`on_enter`]: Layer::on_enter
 [`Layer::register_callsite`]: Layer::register_callsite
 [`Layer::enabled`]: Layer::enabled
 [`Interest::never()`]: tracing_core::subscriber::Interest::never()
 [`Filtered`]: crate::filter::Filtered
 [`filter`]: crate::filter
 [`Targets`]: crate::filter::Targets
 [`FilterFn`]: crate::filter::FilterFn
 [`DynFilterFn`]: crate::filter::DynFilterFn
 [level]: tracing_core::Level
 [`INFO`]: tracing_core::Level::INFO
 [`DEBUG`]: tracing_core::Level::DEBUG
 [target]: tracing_core::Metadata::target
 [`LevelFilter`]: crate::filter::LevelFilter
 [feat]: crate#feature-flags

```rust
pub mod layer { /* ... */ }
```

### Types

#### Struct `Identity`

A layer that does nothing.

```rust
pub struct Identity {
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
  Returns a new `Identity` layer.

###### Trait Implementations

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Identity { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Identity { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Layer**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `Layer`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(notable_trait))]`

A composable handler for `tracing` events.

A `Layer` implements a behavior for recording or collecting traces that can
be composed together with other `Layer`s to build a [`Subscriber`]. See the
[module-level documentation](crate::layer) for details.

[`Subscriber`]: tracing_core::Subscriber

```rust
pub trait Layer<S>
where
    S: Subscriber,
    Self: ''static {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn on_register_dispatch(self: &Self, subscriber: &Dispatch) { /* ... */ }
  ```
  Performs late initialization when installing this layer as a

- ```rust
  fn on_layer(self: &mut Self, subscriber: &mut S) { /* ... */ }
  ```
  Performs late initialization when attaching a `Layer` to a

- ```rust
  fn register_callsite(self: &Self, metadata: &''static Metadata<''static>) -> Interest { /* ... */ }
  ```
  Registers a new callsite with this layer, returning whether or not

- ```rust
  fn enabled(self: &Self, metadata: &Metadata<''_>, ctx: Context<''_, S>) -> bool { /* ... */ }
  ```
  Returns `true` if this layer is interested in a span or event with the

- ```rust
  fn on_new_span(self: &Self, attrs: &span::Attributes<''_>, id: &span::Id, ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this layer that a new span was constructed with the given

- ```rust
  fn on_record(self: &Self, _span: &span::Id, _values: &span::Record<''_>, _ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this layer that a span with the given `Id` recorded the given

- ```rust
  fn on_follows_from(self: &Self, _span: &span::Id, _follows: &span::Id, _ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this layer that a span with the ID `span` recorded that it

- ```rust
  fn event_enabled(self: &Self, _event: &Event<''_>, _ctx: Context<''_, S>) -> bool { /* ... */ }
  ```
  Called before [`on_event`], to determine if `on_event` should be called.

- ```rust
  fn on_event(self: &Self, _event: &Event<''_>, _ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this layer that an event has occurred.

- ```rust
  fn on_enter(self: &Self, _id: &span::Id, _ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this layer that a span with the given ID was entered.

- ```rust
  fn on_exit(self: &Self, _id: &span::Id, _ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this layer that the span with the given ID was exited.

- ```rust
  fn on_close(self: &Self, _id: span::Id, _ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this layer that the span with the given ID has been closed.

- ```rust
  fn on_id_change(self: &Self, _old: &span::Id, _new: &span::Id, _ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this layer that a span ID has been cloned, and that the

- ```rust
  fn and_then<L>(self: Self, layer: L) -> Layered<L, Self, S>
where
    L: Layer<S>,
    Self: Sized { /* ... */ }
  ```
  Composes this layer around the given `Layer`, returning a `Layered`

- ```rust
  fn with_subscriber(self: Self, inner: S) -> Layered<Self, S>
where
    Self: Sized { /* ... */ }
  ```
   Composes this `Layer` with the given [`Subscriber`], returning a

- ```rust
  fn with_filter<F>(self: Self, filter: F) -> filter::Filtered<Self, F, S>
where
    Self: Sized,
    F: Filter<S> { /* ... */ }
  ```
  Combines `self` with a [`Filter`], returning a [`Filtered`] layer.

- ```rust
  fn boxed(self: Self) -> Box<dyn Layer<S> + Send + Sync + ''static>
where
    Self: Sized + Layer<S> + Send + Sync + ''static,
    S: Subscriber { /* ... */ }
  ```
  Erases the type of this [`Layer`], returning a [`Box`]ed `dyn

##### Implementations

This trait is implemented for the following types:

- `FilterFn<F>` with <S, F>
- `DynFilterFn<S, F, R>` with <S, F, R>
- `Filtered<L, F, S>` with <S, L, F>
- `LevelFilter` with <S: Subscriber>
- `Targets` with <S>
- `Layered<A, B, S>` with <S, A, B>
- `Option<L>` with <L, S>
- `alloc::boxed::Box<L>` with <L, S>
- `alloc::boxed::Box<dyn Layer<S> + Send + Sync>` with <S>
- `Vec<L>` with <S, L>
- `Identity` with <S: Subscriber>
- `Layer<L, S>` with <L, S>
- `Layer<S, N, E, W>` with <S, N, E, W>

#### Trait `Filter`

**Attributes:**

- `#[<cfg>(all(feature = "registry", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "registry", feature = "std"))))]`
- `#[<cfg_attr>(docsrs, doc(notable_trait))]`

A per-[`Layer`] filter that determines whether a span or event is enabled
for an individual layer.

See [the module-level documentation][plf] for details on using [`Filter`]s.

[plf]: crate::layer#per-layer-filtering

```rust
pub trait Filter<S> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `enabled`: Returns `true` if this layer is interested in a span or event with the

##### Provided Methods

- ```rust
  fn callsite_enabled(self: &Self, meta: &''static Metadata<''static>) -> Interest { /* ... */ }
  ```
  Returns an [`Interest`] indicating whether this layer will [always],

- ```rust
  fn event_enabled(self: &Self, event: &Event<''_>, cx: &Context<''_, S>) -> bool { /* ... */ }
  ```
  Called before the filtered [`Layer]'s [`on_event`], to determine if

- ```rust
  fn max_level_hint(self: &Self) -> Option<LevelFilter> { /* ... */ }
  ```
  Returns an optional hint of the highest [verbosity level][level] that

- ```rust
  fn on_new_span(self: &Self, attrs: &span::Attributes<''_>, id: &span::Id, ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this filter that a new span was constructed with the given

- ```rust
  fn on_record(self: &Self, id: &span::Id, values: &span::Record<''_>, ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this filter that a span with the given `Id` recorded the given

- ```rust
  fn on_enter(self: &Self, id: &span::Id, ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this filter that a span with the given ID was entered.

- ```rust
  fn on_exit(self: &Self, id: &span::Id, ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this filter that a span with the given ID was exited.

- ```rust
  fn on_close(self: &Self, id: span::Id, ctx: Context<''_, S>) { /* ... */ }
  ```
  Notifies this filter that a span with the given ID has been closed.

##### Implementations

This trait is implemented for the following types:

- `FilterFn<F>` with <S, F>
- `DynFilterFn<S, F, R>` with <S, F, R>
- `And<A, B, S>` with <A, B, S>
- `Or<A, B, S>` with <A, B, S>
- `Not<A, S>` with <A, S>
- `crate::filter::LevelFilter` with <S>
- `std::sync::Arc<dyn layer::Filter<S> + Send + Sync + ''static>` with <S>
- `Box<dyn layer::Filter<S> + Send + Sync + ''static>` with <S>
- `Option<F>` with <F, S>
- `Targets` with <S>
- `Layer<L, S>` with <S, L>

#### Trait `SubscriberExt`

Extension trait adding a `with(Layer)` combinator to `Subscriber`s.

```rust
pub trait SubscriberExt: Subscriber + crate::sealed::Sealed {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn with<L>(self: Self, layer: L) -> Layered<L, Self>
where
    L: Layer<Self>,
    Self: Sized { /* ... */ }
  ```
  Wraps `self` with the provided `layer`.

##### Implementations

This trait is implemented for the following types:

- `S` with <S: Subscriber>

### Re-exports

#### Re-export `self::context::*`

```rust
pub use self::context::*;
```

#### Re-export `self::layered::*`

```rust
pub use self::layered::*;
```

## Module `util`

Extension traits and other utilities to make working with subscribers more
ergonomic.

```rust
pub mod util { /* ... */ }
```

### Types

#### Struct `TryInitError`

Error returned by [`try_init`](SubscriberInitExt::try_init) if a global default subscriber could not be initialized.

```rust
pub struct TryInitError {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn Error + ''static> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
### Traits

#### Trait `SubscriberInitExt`

Extension trait adding utility methods for subscriber initialization.

This trait provides extension methods to make configuring and setting a
[default subscriber] more ergonomic. It is automatically implemented for all
types that can be converted into a [trace dispatcher]. Since `Dispatch`
implements `From<T>` for all `T: Subscriber`, all `Subscriber`
implementations will implement this extension trait as well. Types which
can be converted into `Subscriber`s, such as builders that construct a
`Subscriber`, may implement `Into<Dispatch>`, and will also receive an
implementation of this trait.

[default subscriber]: https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html#setting-the-default-subscriber
[trace dispatcher]: https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html

```rust
pub trait SubscriberInitExt
where
    Self: Into<tracing_core::dispatcher::Dispatch> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn set_default(self: Self) -> dispatcher::DefaultGuard { /* ... */ }
  ```
  Sets `self` as the [default subscriber] in the current scope, returning a

- ```rust
  fn try_init(self: Self) -> Result<(), TryInitError> { /* ... */ }
  ```
  Attempts to set `self` as the [global default subscriber] in the current

- ```rust
  fn init(self: Self) { /* ... */ }
  ```
  Attempts to set `self` as the [global default subscriber] in the current

##### Implementations

This trait is implemented for the following types:

- `T` with <T>

## Module `reload`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Wrapper for a `Layer` to allow it to be dynamically reloaded.

This module provides a [`Layer` type] implementing the [`Layer` trait] or [`Filter` trait]
which wraps another type implementing the corresponding trait. This
allows the wrapped type to be replaced with another
instance of that type at runtime.

This can be used in cases where a subset of `Layer` or `Filter` functionality
should be dynamically reconfigured, such as when filtering directives may
change at runtime. Note that this layer introduces a (relatively small)
amount of overhead, and should thus only be used as needed.

# Examples

Reloading a [global filtering](crate::layer#global-filtering) layer:

```rust
# use tracing::info;
use tracing_subscriber::{filter, fmt, reload, prelude::*};
let filter = filter::LevelFilter::WARN;
let (filter, reload_handle) = reload::Layer::new(filter);
tracing_subscriber::registry()
  .with(filter)
  .with(fmt::Layer::default())
  .init();
#
# // specifying the Registry type is required
# let _: &reload::Handle<filter::LevelFilter, tracing_subscriber::Registry> = &reload_handle;
#
info!("This will be ignored");
reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO);
info!("This will be logged");
```

Reloading a [`Filtered`](crate::filter::Filtered) layer:

```rust
# use tracing::info;
use tracing_subscriber::{filter, fmt, reload, prelude::*};
let filtered_layer = fmt::Layer::default().with_filter(filter::LevelFilter::WARN);
let (filtered_layer, reload_handle) = reload::Layer::new(filtered_layer);
#
# // specifying the Registry type is required
# let _: &reload::Handle<filter::Filtered<fmt::Layer<tracing_subscriber::Registry>,
# filter::LevelFilter, tracing_subscriber::Registry>,tracing_subscriber::Registry>
# = &reload_handle;
#
tracing_subscriber::registry()
  .with(filtered_layer)
  .init();
info!("This will be ignored");
reload_handle.modify(|layer| *layer.filter_mut() = filter::LevelFilter::INFO);
info!("This will be logged");
```

## Note

The [`Layer`] implementation is unable to implement downcasting functionality,
so certain [`Layer`] will fail to downcast if wrapped in a `reload::Layer`.

If you only want to be able to dynamically change the
`Filter` on a layer, prefer wrapping that `Filter` in the `reload::Layer`.

[`Filter` trait]: crate::layer::Filter
[`Layer` type]: Layer
[`Layer` trait]: super::layer::Layer

```rust
pub mod reload { /* ... */ }
```

### Types

#### Struct `Layer`

Wraps a `Layer` or `Filter`, allowing it to be reloaded dynamically at runtime.

```rust
pub struct Layer<L, S> {
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
  pub fn new(inner: L) -> (Self, Handle<L, S>) { /* ... */ }
  ```
  Wraps the given [`Layer`] or [`Filter`], returning a `reload::Layer`

- ```rust
  pub fn handle(self: &Self) -> Handle<L, S> { /* ... */ }
  ```
  Returns a `Handle` that can be used to reload the wrapped [`Layer`] or [`Filter`].

###### Trait Implementations

- **FilterExt**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Layer**
  - ```rust
    fn on_register_dispatch(self: &Self, subscriber: &Dispatch) { /* ... */ }
    ```

  - ```rust
    fn on_layer(self: &mut Self, subscriber: &mut S) { /* ... */ }
    ```

  - ```rust
    fn register_callsite(self: &Self, metadata: &''static Metadata<''static>) -> Interest { /* ... */ }
    ```

  - ```rust
    fn enabled(self: &Self, metadata: &Metadata<''_>, ctx: layer::Context<''_, S>) -> bool { /* ... */ }
    ```

  - ```rust
    fn on_new_span(self: &Self, attrs: &span::Attributes<''_>, id: &span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_record(self: &Self, span: &span::Id, values: &span::Record<''_>, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_follows_from(self: &Self, span: &span::Id, follows: &span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn event_enabled(self: &Self, event: &Event<''_>, ctx: layer::Context<''_, S>) -> bool { /* ... */ }
    ```

  - ```rust
    fn on_event(self: &Self, event: &Event<''_>, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_enter(self: &Self, id: &span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_exit(self: &Self, id: &span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_close(self: &Self, id: span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_id_change(self: &Self, old: &span::Id, new: &span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

- **Filter**
  - ```rust
    fn callsite_enabled(self: &Self, metadata: &''static Metadata<''static>) -> Interest { /* ... */ }
    ```

  - ```rust
    fn enabled(self: &Self, metadata: &Metadata<''_>, ctx: &layer::Context<''_, S>) -> bool { /* ... */ }
    ```

  - ```rust
    fn on_new_span(self: &Self, attrs: &span::Attributes<''_>, id: &span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_record(self: &Self, span: &span::Id, values: &span::Record<''_>, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_enter(self: &Self, id: &span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_exit(self: &Self, id: &span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn on_close(self: &Self, id: span::Id, ctx: layer::Context<''_, S>) { /* ... */ }
    ```

  - ```rust
    fn max_level_hint(self: &Self) -> Option<LevelFilter> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Handle`

Allows reloading the state of an associated [`Layer`](crate::layer::Layer).

```rust
pub struct Handle<L, S> {
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
  pub fn reload</* synthetic */ impl Into<L>: Into<L>>(self: &Self, new_value: impl Into<L>) -> Result<(), Error> { /* ... */ }
  ```
  Replace the current [`Layer`] or [`Filter`] with the provided `new_value`.

- ```rust
  pub fn modify</* synthetic */ impl FnOnce(&mut L): FnOnce(&mut L)>(self: &Self, f: impl FnOnce(&mut L)) -> Result<(), Error> { /* ... */ }
  ```
  Invokes a closure with a mutable reference to the current layer or filter,

- ```rust
  pub fn clone_current(self: &Self) -> Option<L>
where
    L: Clone { /* ... */ }
  ```
  Returns a clone of the layer or filter's current value if it still exists.

- ```rust
  pub fn with_current<T, /* synthetic */ impl FnOnce(&L) -> T: FnOnce(&L) -> T>(self: &Self, f: impl FnOnce(&L) -> T) -> Result<T, Error> { /* ... */ }
  ```
  Invokes a closure with a borrowed reference to the current layer or filter,

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **Freeze**
- **Sync**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Unpin**
#### Struct `Error`

Indicates that an error occurred when reloading a layer.

```rust
pub struct Error {
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
  pub fn is_poisoned(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this error occurred because the layer was poisoned by

- ```rust
  pub fn is_dropped(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this error occurred because the `Subscriber`

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Error**
## Module `fmt`

**Attributes:**

- `#[<cfg>(all(feature = "fmt", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

A `Subscriber` for formatting and logging `tracing` data.

# Overview

[`tracing`] is a framework for instrumenting Rust programs with context-aware,
structured, event-based diagnostic information. This crate provides an
implementation of the [`Subscriber`] trait that records `tracing`'s `Event`s
and `Span`s by formatting them as text and logging them to stdout.

# Usage

First, add this to your `Cargo.toml` file:

```toml
[dependencies]
tracing-subscriber = "0.3"
```

*Compiler support: [requires `rustc` 1.63+][msrv]*

[msrv]: super#supported-rust-versions

Add the following to your executable to initialize the default subscriber:
```rust
use tracing_subscriber;

tracing_subscriber::fmt::init();
```

## Filtering Events with Environment Variables

The default subscriber installed by `init` enables you to filter events
at runtime using environment variables (using the [`EnvFilter`]).

The filter syntax is a superset of the [`env_logger`] syntax.

For example:
- Setting `RUST_LOG=debug` enables all `Span`s and `Event`s
    set to the log level `DEBUG` or higher
- Setting `RUST_LOG=my_crate=trace` enables `Span`s and `Event`s
    in `my_crate` at all log levels

**Note**: This should **not** be called by libraries. Libraries should use
[`tracing`] to publish `tracing` `Event`s.

# Configuration

You can configure a subscriber instead of using the defaults with
the following functions:

### Subscriber

The [`FmtSubscriber`] formats and records `tracing` events as line-oriented logs.
You can create one by calling:

```rust
let subscriber = tracing_subscriber::fmt()
    // ... add configuration
    .finish();
```

You can find the configuration methods for [`FmtSubscriber`] in
[`SubscriberBuilder`].

## Formatters

The output format used by the layer and subscriber in this module is
represented by implementing the [`FormatEvent`] trait, and can be
customized. This module provides a number of formatter implementations:

* [`format::Full`]: The default formatter. This emits human-readable,
  single-line logs for each event that occurs, with the current span context
  displayed before the formatted representation of the event. See
  [here](format::Full#example-output) for sample output.

* [`format::Compact`]: A variant of the default formatter, optimized for
  short line lengths. Fields from the current span context are appended to
  the fields of the formatted event. See
  [here](format::Compact#example-output) for sample output.

* [`format::Pretty`]: Emits excessively pretty, multi-line logs, optimized
  for human readability. This is primarily intended to be used in local
  development and debugging, or for command-line applications, where
  automated analysis and compact storage of logs is less of a priority than
  readability and visual appeal. See [here](format::Pretty#example-output)
  for sample output.

* [`format::Json`]: Outputs newline-delimited JSON logs. This is intended
  for production use with systems where structured logs are consumed as JSON
  by analysis and viewing tools. The JSON output is not optimized for human
  readability. See [here](format::Json#example-output) for sample output.

### Customizing Formatters

The formatting of log lines for spans and events is controlled by two
traits, [`FormatEvent`] and [`FormatFields`]. The [`FormatEvent`] trait
determines the overall formatting of the log line, such as what information
from the event's metadata and span context is included and in what order.
The [`FormatFields`] trait determines how fields &mdash; both the event's
fields and fields on spans &mdash; are formatted.

The [`fmt::format`] module provides several types which implement these traits,
many of which expose additional configuration options to customize their
output. The [`format::Format`] type implements common configuration used by
all the formatters provided in this crate, and can be used as a builder to
set specific formatting settings. For example:

```
use tracing_subscriber::fmt;

// Configure a custom event formatter
let format = fmt::format()
   .with_level(false) // don't include levels in formatted output
   .with_target(false) // don't include targets
   .with_thread_ids(true) // include the thread ID of the current thread
   .with_thread_names(true) // include the name of the current thread
   .compact(); // use the `Compact` formatting style.

// Create a `fmt` subscriber that uses our custom event format, and set it
// as the default.
tracing_subscriber::fmt()
    .event_format(format)
    .init();
```

However, if a specific output format is needed, other crates can
also implement [`FormatEvent`] and [`FormatFields`]. See those traits'
documentation for details on how to implement them.

## Filters

If you want to filter the `tracing` `Events` based on environment
variables, you can use the [`EnvFilter`] as follows:

```rust
use tracing_subscriber::EnvFilter;

let filter = EnvFilter::from_default_env();
```

As mentioned above, the [`EnvFilter`] allows `Span`s and `Event`s to
be filtered at runtime by setting the `RUST_LOG` environment variable.

You can find the other available [`filter`]s in the documentation.

### Using Your Subscriber

Finally, once you have configured your `Subscriber`, you need to
configure your executable to use it.

A subscriber can be installed globally using:
```rust
use tracing;
use tracing_subscriber::FmtSubscriber;

let subscriber = FmtSubscriber::new();

tracing::subscriber::set_global_default(subscriber)
    .map_err(|_err| eprintln!("Unable to set global default subscriber"));
// Note this will only fail if you try to set the global default
// subscriber multiple times
```

### Composing Layers

Composing an [`EnvFilter`] `Layer` and a [format `Layer`][super::fmt::Layer]:

```rust
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::*;

let fmt_layer = fmt::layer()
    .with_target(false);
let filter_layer = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("info"))
    .unwrap();

tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer)
    .init();
```

[`EnvFilter`]: super::filter::EnvFilter
[`env_logger`]: https://docs.rs/env_logger/
[`filter`]: super::filter
[`FmtSubscriber`]: Subscriber
[`Subscriber`]:
    https://docs.rs/tracing/latest/tracing/trait.Subscriber.html
[`tracing`]: https://crates.io/crates/tracing
[`fmt::format`]: mod@crate::fmt::format

```rust
pub mod fmt { /* ... */ }
```

### Modules

## Module `format`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

Formatters for logging `tracing` events.

This module provides several formatter implementations, as well as utilities
for implementing custom formatters.

# Formatters
This module provides a number of formatter implementations:

* [`Full`]: The default formatter. This emits human-readable,
  single-line logs for each event that occurs, with the current span context
  displayed before the formatted representation of the event. See
  [here](Full#example-output) for sample output.

* [`Compact`]: A variant of the default formatter, optimized for
  short line lengths. Fields from the current span context are appended to
  the fields of the formatted event, and span names are not shown; the
  verbosity level is abbreviated to a single character. See
  [here](Compact#example-output) for sample output.

* [`Pretty`]: Emits excessively pretty, multi-line logs, optimized
  for human readability. This is primarily intended to be used in local
  development and debugging, or for command-line applications, where
  automated analysis and compact storage of logs is less of a priority than
  readability and visual appeal. See [here](Pretty#example-output)
  for sample output.

* [`Json`]: Outputs newline-delimited JSON logs. This is intended
  for production use with systems where structured logs are consumed as JSON
  by analysis and viewing tools. The JSON output is not optimized for human
  readability. See [here](Json#example-output) for sample output.

```rust
pub mod format { /* ... */ }
```

### Types

#### Struct `Writer`

A writer to which formatted representations of spans and events are written.

This type is provided as input to the [`FormatEvent::format_event`] and
[`FormatFields::format_fields`] methods, which will write formatted
representations of [`Event`]s and [fields] to the `Writer`.

This type implements the [`std::fmt::Write`] trait, allowing it to be used
with any function that takes an instance of [`std::fmt::Write`].
Additionally, it can be used with the standard library's [`std::write!`] and
[`std::writeln!`] macros.

Additionally, a `Writer` may expose additional `tracing`-specific
information to the formatter implementation.

[fields]: tracing_core::field

```rust
pub struct Writer<''writer> {
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
  pub fn new</* synthetic */ impl fmt::Write: fmt::Write>(writer: &''writer mut impl fmt::Write) -> Self { /* ... */ }
  ```
  Create a new [`Writer`] from any type that implements [`fmt::Write`].

- ```rust
  pub fn by_ref(self: &mut Self) -> Writer<''_> { /* ... */ }
  ```
  Return a new `Writer` that mutably borrows `self`.

- ```rust
  pub fn write_str(self: &mut Self, s: &str) -> fmt::Result { /* ... */ }
  ```
  Writes a string slice into this `Writer`, returning whether the write succeeded.

- ```rust
  pub fn write_char(self: &mut Self, c: char) -> fmt::Result { /* ... */ }
  ```
  Writes a [`char`] into this writer, returning whether the write succeeded.

- ```rust
  pub fn write_fmt(self: &mut Self, args: fmt::Arguments<''_>) -> fmt::Result { /* ... */ }
  ```
  Glue for usage of the [`write!`] macro with `Writer`s.

- ```rust
  pub fn has_ansi_escapes(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if [ANSI escape codes] may be used to add colors

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Send**
- **Write**
  - ```rust
    fn write_str(self: &mut Self, s: &str) -> fmt::Result { /* ... */ }
    ```

  - ```rust
    fn write_char(self: &mut Self, c: char) -> fmt::Result { /* ... */ }
    ```

  - ```rust
    fn write_fmt(self: &mut Self, args: fmt::Arguments<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MakeVisitor**
  - ```rust
    fn make_visitor(self: &Self, target: Writer<''a>) -> <Self as >::Visitor { /* ... */ }
    ```

  - ```rust
    fn make_visitor(self: &Self, target: Writer<''a>) -> <Self as >::Visitor { /* ... */ }
    ```

  - ```rust
    fn make_visitor(self: &Self, writer: Writer<''a>) -> <Self as >::Visitor { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `FieldFn`

A [`FormatFields`] implementation that formats fields by calling a function
or closure.


```rust
pub struct FieldFn<F>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **MakeExt**
- **Freeze**
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

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FieldFn<F> { /* ... */ }
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

- **MakeVisitor**
  - ```rust
    fn make_visitor(self: &Self, writer: Writer<''a>) -> <Self as >::Visitor { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `FieldFnVisitor`

The [visitor] produced by [`FieldFn`]'s [`MakeVisitor`] implementation.

[visitor]: super::super::field::Visit
[`MakeVisitor`]: super::super::field::MakeVisitor

```rust
pub struct FieldFnVisitor<''a, F> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **VisitOutput**
  - ```rust
    fn finish(self: Self) -> fmt::Result { /* ... */ }
    ```

- **Visit**
  - ```rust
    fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **VisitFmt**
  - ```rust
    fn writer(self: &mut Self) -> &mut dyn fmt::Write { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
#### Struct `Compact`

Marker for [`Format`] that indicates that the compact log format should be used.

The compact format includes fields from all currently entered spans, after
the event's fields. Span fields are ordered (but not grouped) by
span, and span names are  not shown. A more compact representation of the
event's [`Level`] is used, and additional information—such as the event's
target—is disabled by default.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt-compact
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt-compact`
<font color="#AAAAAA">2022-02-17T19:51:05.809287Z </font><font color="#4E9A06"> INFO</font> <b>fmt_compact</b><font color="#AAAAAA">: preparing to shave yaks </font><i>number_of_yaks</i><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809367Z </font><font color="#4E9A06"> INFO</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: shaving yaks </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809414Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809443Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: yak shaved successfully </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809477Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=1 </font><i>shaved</i><font color="#AAAAAA">=true </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809500Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=1 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809531Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809554Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: yak shaved successfully </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809581Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=2 </font><i>shaved</i><font color="#AAAAAA">=true </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809606Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=2 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809635Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809664Z </font><font color="#C4A000"> WARN</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: could not locate yak </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809693Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=3 </font><i>shaved</i><font color="#AAAAAA">=false </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809717Z </font><font color="#CC0000">ERROR</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: failed to shave yak </font><i>yak</i><font color="#AAAAAA">=3 </font><i>error</i><font color="#AAAAAA">=missing yak </font><i>error.sources</i><font color="#AAAAAA">=[out of space, out of cash] </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809743Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=2 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809768Z </font><font color="#4E9A06"> INFO</font> <b>fmt_compact</b><font color="#AAAAAA">: yak shaving completed </font><i>all_yaks_shaved</i><font color="#AAAAAA">=false</font>

</pre>

```rust
pub struct Compact;
```

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Compact) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Compact { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Compact { /* ... */ }
    ```

#### Struct `Full`

Marker for [`Format`] that indicates that the default log format should be used.

This formatter shows the span context before printing event data. Spans are
displayed including their names and fields.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt`
<font color="#AAAAAA">2022-02-15T18:40:14.289898Z </font><font color="#4E9A06"> INFO</font> fmt: preparing to shave yaks <i>number_of_yaks</i><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-15T18:40:14.289974Z </font><font color="#4E9A06"> INFO</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: shaving yaks</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290011Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=1</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290038Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=1</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: yak shaved successfully</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290070Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=1 </font><i>shaved</i><font color="#AAAAAA">=true</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290089Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290114Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=2</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290134Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=2</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: yak shaved successfully</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290157Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=2 </font><i>shaved</i><font color="#AAAAAA">=true</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290174Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290198Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290222Z </font><font color="#C4A000"> WARN</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: could not locate yak</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290247Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=3 </font><i>shaved</i><font color="#AAAAAA">=false</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290268Z </font><font color="#CC0000">ERROR</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: failed to shave yak </font><i>yak</i><font color="#AAAAAA">=3 </font><i>error</i><font color="#AAAAAA">=missing yak </font><i>error.sources</i><font color="#AAAAAA">=[out of space, out of cash]</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290287Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290309Z </font><font color="#4E9A06"> INFO</font> fmt: yak shaving completed. <i>all_yaks_shaved</i><font color="#AAAAAA">=false</font>
</pre>

```rust
pub struct Full;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Full) -> bool { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Full { /* ... */ }
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
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Full { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
#### Struct `Format`

A pre-configured event formatter.

You will usually want to use this as the `FormatEvent` for a `FmtSubscriber`.

The default logging format, [`Full`] includes all fields in each event and its containing
spans. The [`Compact`] logging format is intended to produce shorter log
lines; it displays each event's fields, along with fields from the current
span context, but other information is abbreviated. The [`Pretty`] logging
format is an extra-verbose, multi-line human-readable logging format
intended for use in development.

```rust
pub struct Format<F = Full, T = super::time::SystemTime> {
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
  pub fn compact(self: Self) -> Format<Compact, T> { /* ... */ }
  ```
  Use a less verbose output format.

- ```rust
  pub fn pretty(self: Self) -> Format<Pretty, T> { /* ... */ }
  ```
  Use an excessively pretty, human-readable output format.

- ```rust
  pub fn with_timer<T2>(self: Self, timer: T2) -> Format<F, T2> { /* ... */ }
  ```
  Use the given [`timer`] for log message timestamps.

- ```rust
  pub fn without_time(self: Self) -> Format<F, ()> { /* ... */ }
  ```
  Do not emit timestamps with log messages.

- ```rust
  pub fn with_ansi(self: Self, ansi: bool) -> Format<F, T> { /* ... */ }
  ```
  Enable ANSI terminal colors for formatted output.

- ```rust
  pub fn with_target(self: Self, display_target: bool) -> Format<F, T> { /* ... */ }
  ```
  Sets whether or not an event's target is displayed.

- ```rust
  pub fn with_level(self: Self, display_level: bool) -> Format<F, T> { /* ... */ }
  ```
  Sets whether or not an event's level is displayed.

- ```rust
  pub fn with_thread_ids(self: Self, display_thread_id: bool) -> Format<F, T> { /* ... */ }
  ```
  Sets whether or not the [thread ID] of the current thread is displayed

- ```rust
  pub fn with_thread_names(self: Self, display_thread_name: bool) -> Format<F, T> { /* ... */ }
  ```
  Sets whether or not the [name] of the current thread is displayed

- ```rust
  pub fn with_file(self: Self, display_filename: bool) -> Format<F, T> { /* ... */ }
  ```
  Sets whether or not an event's [source code file path][file] is

- ```rust
  pub fn with_line_number(self: Self, display_line_number: bool) -> Format<F, T> { /* ... */ }
  ```
  Sets whether or not an event's [source code line number][line] is

- ```rust
  pub fn with_source_location(self: Self, display_location: bool) -> Self { /* ... */ }
  ```
  Sets whether or not the source code location from which an event

###### Trait Implementations

- **FormatEvent**
  - ```rust
    fn format_event(self: &Self, ctx: &FmtContext<''_, C, N>, writer: Writer<''_>, event: &Event<''_>) -> fmt::Result { /* ... */ }
    ```

  - ```rust
    fn format_event(self: &Self, ctx: &FmtContext<''_, S, N>, writer: Writer<''_>, event: &Event<''_>) -> fmt::Result { /* ... */ }
    ```

  - ```rust
    fn format_event(self: &Self, ctx: &FmtContext<''_, S, N>, writer: Writer<''_>, event: &Event<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Format<F, T> { /* ... */ }
    ```

#### Struct `DefaultFields`

The default [`FormatFields`] implementation.


```rust
pub struct DefaultFields {
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
  Returns a new default [`FormatFields`] implementation.

###### Trait Implementations

- **Send**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MakeVisitor**
  - ```rust
    fn make_visitor(self: &Self, target: Writer<''a>) -> <Self as >::Visitor { /* ... */ }
    ```

- **MakeExt**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

#### Struct `DefaultVisitor`

The [visitor] produced by [`DefaultFields`]'s [`MakeVisitor`] implementation.

[visitor]: super::super::field::Visit
[`MakeVisitor`]: super::super::field::MakeVisitor

```rust
pub struct DefaultVisitor<''a> {
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
  pub fn new(writer: Writer<''a>, is_empty: bool) -> Self { /* ... */ }
  ```
  Returns a new default visitor that formats to the provided `writer`.

###### Trait Implementations

- **Sync**
- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VisitOutput**
  - ```rust
    fn finish(self: Self) -> fmt::Result { /* ... */ }
    ```

- **VisitFmt**
  - ```rust
    fn writer(self: &mut Self) -> &mut dyn fmt::Write { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Visit**
  - ```rust
    fn record_str(self: &mut Self, field: &Field, value: &str) { /* ... */ }
    ```

  - ```rust
    fn record_error(self: &mut Self, field: &Field, value: &dyn std::error::Error + ''static) { /* ... */ }
    ```

  - ```rust
    fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug) { /* ... */ }
    ```

#### Struct `FmtSpan`

Configures what points in the span lifecycle are logged as events.

See also [`with_span_events`](super::SubscriberBuilder.html::with_span_events).

```rust
pub struct FmtSpan(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, rhs: Self) { /* ... */ }
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

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, rhs: Self) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &FmtSpan) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &FmtSpan) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FmtSpan { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FmtSpan) -> bool { /* ... */ }
    ```

- **Sync**
### Traits

#### Trait `FormatEvent`

A type that can format a tracing [`Event`] to a [`Writer`].

`FormatEvent` is primarily used in the context of [`fmt::Subscriber`] or
[`fmt::Layer`]. Each time an event is dispatched to [`fmt::Subscriber`] or
[`fmt::Layer`], the subscriber or layer
forwards it to its associated `FormatEvent` to emit a log message.

This trait is already implemented for function pointers with the same
signature as `format_event`.

# Arguments

The following arguments are passed to `FormatEvent::format_event`:

* A [`FmtContext`]. This is an extension of the [`layer::Context`] type,
  which can be used for accessing stored information such as the current
  span context an event occurred in.

  In addition, [`FmtContext`] exposes access to the [`FormatFields`]
  implementation that the subscriber was configured to use via the
  [`FmtContext::field_format`] method. This can be used when the
  [`FormatEvent`] implementation needs to format the event's fields.

  For convenience, [`FmtContext`] also [implements `FormatFields`],
  forwarding to the configured [`FormatFields`] type.

* A [`Writer`] to which the formatted representation of the event is
  written. This type implements the [`std::fmt::Write`] trait, and therefore
  can be used with the [`std::write!`] and [`std::writeln!`] macros, as well
  as calling [`std::fmt::Write`] methods directly.

  The [`Writer`] type also implements additional methods that provide
  information about how the event should be formatted. The
  [`Writer::has_ansi_escapes`] method indicates whether [ANSI terminal
  escape codes] are supported by the underlying I/O writer that the event
  will be written to. If this returns `true`, the formatter is permitted to
  use ANSI escape codes to add colors and other text formatting to its
  output. If it returns `false`, the event will be written to an output that
  does not support ANSI escape codes (such as a log file), and they should
  not be emitted.

  Crates like [`nu_ansi_term`] and [`owo-colors`] can be used to add ANSI
  escape codes to formatted output.

* The actual [`Event`] to be formatted.

# Examples

This example re-implements a simiplified version of this crate's [default
formatter]:

```rust
use std::fmt;
use tracing_core::{Subscriber, Event};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
    FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

let _subscriber = tracing_subscriber::fmt()
    .event_format(MyFormatter)
    .init();

let _span = tracing::info_span!("my_span", answer = 42).entered();
tracing::info!(question = "life, the universe, and everything", "hello world");
```

This formatter will print events like this:

```text
DEBUG yak_shaving::shaver: some-span{field-on-span=foo}: started shaving yak
```

[`layer::Context`]: crate::layer::Context
[`fmt::Layer`]: super::Layer
[`fmt::Subscriber`]: super::Subscriber
[`Event`]: tracing::Event
[implements `FormatFields`]: super::FmtContext#impl-FormatFields<'writer>
[ANSI terminal escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
[`Writer::has_ansi_escapes`]: Writer::has_ansi_escapes
[`nu_ansi_term`]: https://crates.io/crates/nu_ansi_term
[`owo-colors`]: https://crates.io/crates/owo-colors
[default formatter]: Full

```rust
pub trait FormatEvent<S, N>
where
    S: Subscriber + for<''a> LookupSpan<''a>,
    N: for<''a> FormatFields<''a> + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `format_event`: Write a log message for `Event` in `Context` to the given [`Writer`].

##### Implementations

This trait is implemented for the following types:

- `Format<Pretty, T>` with <C, N, T>
- `fn(&crate::fmt::fmt_layer::FmtContext<''_, S, N>, Writer<''_>, &tracing_core::Event<''_>) -> fmt::Result` with <S, N>
- `Format<Full, T>` with <S, N, T>
- `Format<Compact, T>` with <S, N, T>

#### Trait `FormatFields`

A type that can format a [set of fields] to a [`Writer`].

`FormatFields` is primarily used in the context of [`FmtSubscriber`]. Each
time a span or event with fields is recorded, the subscriber will format
those fields with its associated `FormatFields` implementation.

[set of fields]: crate::field::RecordFields
[`FmtSubscriber`]: super::Subscriber

```rust
pub trait FormatFields<''writer> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `format_fields`: Format the provided `fields` to the provided [`Writer`], returning a result.

##### Provided Methods

- ```rust
  fn add_fields(self: &Self, current: &''writer mut FormattedFields<Self>, fields: &span::Record<''_>) -> fmt::Result { /* ... */ }
  ```
  Record additional field(s) on an existing span.

##### Implementations

This trait is implemented for the following types:

- `FmtContext<''_, S, N>` with <''writer, S, N>
- `Pretty` with <''writer>
- `M` with <''writer, M>

### Functions

#### Function `format`

Returns the default configuration for an [event formatter].

Methods on the returned event formatter can be used for further
configuration. For example:

```rust
let format = tracing_subscriber::fmt::format()
    .without_time()         // Don't include timestamps
    .with_target(false)     // Don't include event targets.
    .with_level(false)      // Don't include event levels.
    .compact();             // Use a more compact, abbreviated format.

// Use the configured formatter when building a new subscriber.
tracing_subscriber::fmt()
    .event_format(format)
    .init();
```

```rust
pub fn format() -> Format { /* ... */ }
```

#### Function `debug_fn`

Returns a [`FormatFields`] implementation that formats fields using the
provided function or closure.


```rust
pub fn debug_fn<F>(f: F) -> FieldFn<F>
where
    F: Fn(&mut Writer<''_>, &tracing_core::field::Field, &dyn fmt::Debug) -> fmt::Result + Clone { /* ... */ }
```

### Re-exports

#### Re-export `pretty::*`

**Attributes:**

- `#[<cfg>(feature = "ansi")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "ansi")))]`

```rust
pub use pretty::*;
```

## Module `time`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

Formatters for event timestamps.

```rust
pub mod time { /* ... */ }
```

### Types

#### Struct `SystemTime`

Retrieve and print the current wall-clock time.

```rust
pub struct SystemTime;
```

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SystemTime { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **Eq**
- **Default**
  - ```rust
    fn default() -> SystemTime { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
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

- **Send**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SystemTime) -> bool { /* ... */ }
    ```

- **FormatTime**
  - ```rust
    fn format_time(self: &Self, w: &mut Writer<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `Uptime`

Retrieve and print the relative elapsed wall-clock time since an epoch.

The `Default` implementation for `Uptime` makes the epoch the current time.

```rust
pub struct Uptime {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **FormatTime**
  - ```rust
    fn format_time(self: &Self, w: &mut Writer<''_>) -> fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Uptime { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Uptime) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(epoch: Instant) -> Self { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
### Traits

#### Trait `FormatTime`

A type that can measure and format the current time.

This trait is used by `Format` to include a timestamp with each `Event` when it is logged.

Notable default implementations of this trait are `SystemTime` and `()`. The former prints the
current time as reported by `std::time::SystemTime`, and the latter does not print the current
time at all. `FormatTime` is also automatically implemented for any function pointer with the
appropriate signature.

The full list of provided implementations can be found in [`time`].

[`time`]: self

```rust
pub trait FormatTime {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `format_time`: Measure and write out the current time.

##### Implementations

This trait is implemented for the following types:

- `&F` with <F>
- `()`
- `fn(&mut crate::fmt::format::Writer<''_>) -> fmt::Result`
- `SystemTime`
- `Uptime`

### Functions

#### Function `time`

Returns a new `SystemTime` timestamp provider.

This can then be configured further to determine how timestamps should be
configured.

This is equivalent to calling
```rust
# fn timer() -> tracing_subscriber::fmt::time::SystemTime {
tracing_subscriber::fmt::time::SystemTime::default()
# }
```

```rust
pub fn time() -> SystemTime { /* ... */ }
```

#### Function `uptime`

Returns a new `Uptime` timestamp provider.

With this timer, timestamps will be formatted with the amount of time
elapsed since the timestamp provider was constructed.

This can then be configured further to determine how timestamps should be
configured.

This is equivalent to calling
```rust
# fn timer() -> tracing_subscriber::fmt::time::Uptime {
tracing_subscriber::fmt::time::Uptime::default()
# }
```

```rust
pub fn uptime() -> Uptime { /* ... */ }
```

## Module `writer`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

Abstractions for creating [`io::Write`] instances.

[`io::Write`]: std::io::Write

```rust
pub mod writer { /* ... */ }
```

### Types

#### Struct `TestWriter`

A writer intended to support [`libtest`'s output capturing][capturing] for use in unit tests.

`TestWriter` is used by [`fmt::Subscriber`] or [`fmt::Layer`] to enable capturing support.

`cargo test` can only capture output from the standard library's [`print!`] macro. See
[`libtest`'s output capturing][capturing] for more details about output capturing.

Writing to [`io::stdout`] and [`io::stderr`] produces the same results as using
[`libtest`'s `--nocapture` option][nocapture] which may make the results look unreadable.

[`fmt::Subscriber`]: super::Subscriber
[`fmt::Layer`]: super::Layer
[capturing]: https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output
[nocapture]: https://doc.rust-lang.org/cargo/commands/cargo-test.html
[`io::stdout`]: std::io::stdout
[`io::stderr`]: std::io::stderr
[`print!`]: std::print!

```rust
pub struct TestWriter {
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
  Returns a new `TestWriter` with the default configuration.

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> TestWriter { /* ... */ }
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

- **MakeWriterExt**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

- **Unpin**
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

- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **MakeWriter**
  - ```rust
    fn make_writer(self: &''a Self) -> <Self as >::Writer { /* ... */ }
    ```

#### Struct `BoxMakeWriter`

A writer that erases the specific [`io::Write`] and [`MakeWriter`] types being used.

This is useful in cases where the concrete type of the writer cannot be known
until runtime.

# Examples

A function that returns a [`Subscriber`] that will write to either stdout or stderr:

```rust
# use tracing::Subscriber;
# use tracing_subscriber::fmt::writer::BoxMakeWriter;

fn dynamic_writer(use_stderr: bool) -> impl Subscriber {
    let writer = if use_stderr {
        BoxMakeWriter::new(std::io::stderr)
    } else {
        BoxMakeWriter::new(std::io::stdout)
    };

    tracing_subscriber::fmt().with_writer(writer).finish()
}
```

[`Subscriber`]: tracing::Subscriber
[`io::Write`]: std::io::Write

```rust
pub struct BoxMakeWriter {
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
  pub fn new<M>(make_writer: M) -> Self
where
    M: for<''a> MakeWriter<''a> + Send + Sync + ''static { /* ... */ }
  ```
  Constructs a `BoxMakeWriter` wrapping a type implementing [`MakeWriter`].

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **MakeWriter**
  - ```rust
    fn make_writer(self: &''a Self) -> <Self as >::Writer { /* ... */ }
    ```

  - ```rust
    fn make_writer_for(self: &''a Self, meta: &Metadata<''_>) -> <Self as >::Writer { /* ... */ }
    ```

- **MakeWriterExt**
- **RefUnwindSafe**
- **Unpin**
#### Enum `EitherWriter`

A [writer] that is one of two types implementing [`io::Write`].

This may be used by [`MakeWriter`] implementations that may conditionally
return one of two writers.

[writer]: std::io::Write

```rust
pub enum EitherWriter<A, B> {
    A(A),
    B(B),
}
```

##### Variants

###### `A`

A writer of type `A`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `A` |  |

###### `B`

A writer of type `B`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `B` |  |

##### Implementations

###### Methods

- ```rust
  pub fn none() -> Self { /* ... */ }
  ```
  Returns a [disabled writer].

- ```rust
  pub fn some(t: T) -> Self { /* ... */ }
  ```
  Returns an enabled writer of type `T`.

###### Trait Implementations

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EitherWriter<A, B>) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EitherWriter<A, B> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<''_>]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write_fmt(self: &mut Self, fmt: std::fmt::Arguments<''_>) -> io::Result<()> { /* ... */ }
    ```

#### Type Alias `OptionalWriter`

A [writer] which may or may not be enabled.

This may be used by [`MakeWriter`] implementations that wish to
conditionally enable or disable the returned writer based on a span or
event's [`Metadata`].

[writer]: std::io::Write

```rust
pub type OptionalWriter<T> = EitherWriter<T, std::io::Sink>;
```

#### Struct `WithMaxLevel`

A [`MakeWriter`] combinator that only returns an enabled [writer] for spans
and events with metadata at or below a specified verbosity [`Level`].

This is returned by the [`MakeWriterExt::with_max_level`] method. See the
method documentation for details.

[writer]: std::io::Write
[`Level`]: tracing_core::Level

```rust
pub struct WithMaxLevel<M> {
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
  pub fn new(make: M, level: tracing_core::Level) -> Self { /* ... */ }
  ```
  Wraps the provided [`MakeWriter`] with a maximum [`Level`], so that it

###### Trait Implementations

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MakeWriterExt**
- **StructuralPartialEq**
- **UnwindSafe**
- **MakeWriter**
  - ```rust
    fn make_writer(self: &''a Self) -> <Self as >::Writer { /* ... */ }
    ```

  - ```rust
    fn make_writer_for(self: &''a Self, meta: &Metadata<''_>) -> <Self as >::Writer { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WithMaxLevel<M> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WithMaxLevel<M>) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
#### Struct `WithMinLevel`

A [`MakeWriter`] combinator that only returns an enabled [writer] for spans
and events with metadata at or above a specified verbosity [`Level`].

This is returned by the [`MakeWriterExt::with_min_level`] method. See the
method documentation for details.

[writer]: std::io::Write
[`Level`]: tracing_core::Level

```rust
pub struct WithMinLevel<M> {
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
  pub fn new(make: M, level: tracing_core::Level) -> Self { /* ... */ }
  ```
  Wraps the provided [`MakeWriter`] with a minimum [`Level`], so that it

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MakeWriterExt**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WithMinLevel<M> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MakeWriter**
  - ```rust
    fn make_writer(self: &''a Self) -> <Self as >::Writer { /* ... */ }
    ```

  - ```rust
    fn make_writer_for(self: &''a Self, meta: &Metadata<''_>) -> <Self as >::Writer { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WithMinLevel<M>) -> bool { /* ... */ }
    ```

#### Struct `WithFilter`

A [`MakeWriter`] combinator that wraps a [`MakeWriter`] with a predicate for
span and event [`Metadata`], so that the [`MakeWriter::make_writer_for`]
method returns [`OptionalWriter::some`][ows] when the predicate returns `true`,
and [`OptionalWriter::none`][own] when the predicate returns `false`.

This is returned by the [`MakeWriterExt::with_filter`] method. See the
method documentation for details.

[`Metadata`]: tracing_core::Metadata
[ows]: EitherWriter::some
[own]: EitherWriter::none

```rust
pub struct WithFilter<M, F> {
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
  pub fn new(make: M, filter: F) -> Self
where
    F: Fn(&Metadata<''_>) -> bool { /* ... */ }
  ```
  Wraps `make` with the provided `filter`, returning a [`MakeWriter`] that

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MakeWriterExt**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WithFilter<M, F>) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **MakeWriter**
  - ```rust
    fn make_writer(self: &''a Self) -> <Self as >::Writer { /* ... */ }
    ```

  - ```rust
    fn make_writer_for(self: &''a Self, meta: &Metadata<''_>) -> <Self as >::Writer { /* ... */ }
    ```

- **Send**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WithFilter<M, F> { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `OrElse`

Combines a [`MakeWriter`] that returns an [`OptionalWriter`] with another
[`MakeWriter`], so that the second [`MakeWriter`] is used when the first
[`MakeWriter`] returns [`OptionalWriter::none`][own].

This is returned by the [`MakeWriterExt::or_else] method. See the
method documentation for details.

[own]: EitherWriter::none

```rust
pub struct OrElse<A, B> {
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
  pub fn new<''a, W>(inner: A, or_else: B) -> Self
where
    A: MakeWriter<''a, Writer = OptionalWriter<W>>,
    B: MakeWriter<''a>,
    W: Write { /* ... */ }
  ```
  Combines

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OrElse<A, B>) -> bool { /* ... */ }
    ```

- **MakeWriter**
  - ```rust
    fn make_writer(self: &''a Self) -> <Self as >::Writer { /* ... */ }
    ```

  - ```rust
    fn make_writer_for(self: &''a Self, meta: &Metadata<''_>) -> <Self as >::Writer { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> OrElse<A, B> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MakeWriterExt**
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

- **Eq**
- **Unpin**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Tee`

Combines two types implementing [`MakeWriter`] (or [`std::io::Write`]) to
produce a writer that writes to both [`MakeWriter`]'s returned writers.

This is returned by the [`MakeWriterExt::and`] method. See the method
documentation for details.

```rust
pub struct Tee<A, B> {
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
  pub fn new(a: A, b: B) -> Self { /* ... */ }
  ```
  Combines two types implementing [`MakeWriter`], returning

###### Trait Implementations

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Tee<A, B>) -> bool { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<''_>]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write_fmt(self: &mut Self, fmt: std::fmt::Arguments<''_>) -> io::Result<()> { /* ... */ }
    ```

- **StructuralPartialEq**
- **MakeWriterExt**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Tee<A, B> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MakeWriter**
  - ```rust
    fn make_writer(self: &''a Self) -> <Self as >::Writer { /* ... */ }
    ```

  - ```rust
    fn make_writer_for(self: &''a Self, meta: &Metadata<''_>) -> <Self as >::Writer { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `MutexGuardWriter`

A type implementing [`io::Write`] for a [`MutexGuard`] where the type
inside the [`Mutex`] implements [`io::Write`].

This is used by the [`MakeWriter`] implementation for [`Mutex`], because
[`MutexGuard`] itself will not implement [`io::Write`] — instead, it
_dereferences_ to a type implementing [`io::Write`]. Because [`MakeWriter`]
requires the `Writer` type to implement [`io::Write`], it's necessary to add
a newtype that forwards the trait implementation.

[`io::Write`]: std::io::Write
[`MutexGuard`]: std::sync::MutexGuard
[`Mutex`]: std::sync::Mutex

```rust
pub struct MutexGuardWriter<''a, W>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<''_>]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write_fmt(self: &mut Self, fmt: std::fmt::Arguments<''_>) -> io::Result<()> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
### Traits

#### Trait `MakeWriter`

A type that can create [`io::Write`] instances.

`MakeWriter` is used by [`fmt::Layer`] or [`fmt::Subscriber`] to print
formatted text representations of [`Event`]s.

This trait is already implemented for function pointers and
immutably-borrowing closures that return an instance of [`io::Write`], such
as [`io::stdout`] and [`io::stderr`]. Additionally, it is implemented for
[`std::sync::Mutex`] when the type inside the mutex implements
[`io::Write`].

# Examples

The simplest usage is to pass in a named function that returns a writer. For
example, to log all events to stderr, we could write:
```
let subscriber = tracing_subscriber::fmt()
    .with_writer(std::io::stderr)
    .finish();
# drop(subscriber);
```

Any function that returns a writer can be used:

```
fn make_my_great_writer() -> impl std::io::Write {
    // ...
    # std::io::stdout()
}

let subscriber = tracing_subscriber::fmt()
    .with_writer(make_my_great_writer)
    .finish();
# drop(subscriber);
```

A closure can be used to introduce arbitrary logic into how the writer is
created. Consider the (admittedly rather silly) example of sending every 5th
event to stderr, and all other events to stdout:

```
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

let n = AtomicUsize::new(0);
let subscriber = tracing_subscriber::fmt()
    .with_writer(move || -> Box<dyn io::Write> {
        if n.fetch_add(1, Relaxed) % 5 == 0 {
            Box::new(io::stderr())
        } else {
            Box::new(io::stdout())
       }
    })
    .finish();
# drop(subscriber);
```

A single instance of a type implementing [`io::Write`] may be used as a
`MakeWriter` by wrapping it in a [`Mutex`]. For example, we could
write to a file like so:

```
use std::{fs::File, sync::Mutex};

# fn docs() -> Result<(), Box<dyn std::error::Error>> {
let log_file = File::create("my_cool_trace.log")?;
let subscriber = tracing_subscriber::fmt()
    .with_writer(Mutex::new(log_file))
    .finish();
# drop(subscriber);
# Ok(())
# }
```

[`io::Write`]: std::io::Write
[`fmt::Layer`]: crate::fmt::Layer
[`fmt::Subscriber`]: crate::fmt::Subscriber
[`Event`]: tracing_core::event::Event
[`io::stdout`]: std::io::stdout()
[`io::stderr`]: std::io::stderr()
[`MakeWriter::make_writer_for`]: MakeWriter::make_writer_for
[`Metadata`]: tracing_core::Metadata
[levels]: tracing_core::Level
[targets]: tracing_core::Metadata::target

```rust
pub trait MakeWriter<''a> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Writer`: The concrete [`io::Write`] implementation returned by [`make_writer`].

###### Required Methods

- `make_writer`: Returns an instance of [`Writer`].

##### Provided Methods

- ```rust
  fn make_writer_for(self: &''a Self, meta: &Metadata<''_>) -> <Self as >::Writer { /* ... */ }
  ```
  Returns a [`Writer`] for writing data from the span or event described

##### Implementations

This trait is implemented for the following types:

- `F` with <''a, F, W>
- `std::sync::Arc<W>` with <''a, W>
- `std::fs::File` with <''a>
- `TestWriter` with <''a>
- `BoxMakeWriter` with <''a>
- `std::sync::Mutex<W>` with <''a, W>
- `WithMaxLevel<M>` with <''a, M: MakeWriter<''a>>
- `WithMinLevel<M>` with <''a, M: MakeWriter<''a>>
- `WithFilter<M, F>` with <''a, M, F>
- `Tee<A, B>` with <''a, A, B>
- `OrElse<A, B>` with <''a, A, B, W>

#### Trait `MakeWriterExt`

Extension trait adding combinators for working with types implementing
[`MakeWriter`].

This is not intended to be implemented directly for user-defined
[`MakeWriter`]s; instead, it should be imported when the desired methods are
used.

```rust
pub trait MakeWriterExt<''a>: MakeWriter<''a> {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn with_max_level(self: Self, level: tracing_core::Level) -> WithMaxLevel<Self>
where
    Self: Sized { /* ... */ }
  ```
  Wraps `self` and returns a [`MakeWriter`] that will only write output

- ```rust
  fn with_min_level(self: Self, level: tracing_core::Level) -> WithMinLevel<Self>
where
    Self: Sized { /* ... */ }
  ```
  Wraps `self` and returns a [`MakeWriter`] that will only write output

- ```rust
  fn with_filter<F>(self: Self, filter: F) -> WithFilter<Self, F>
where
    Self: Sized,
    F: Fn(&Metadata<''_>) -> bool { /* ... */ }
  ```
  Wraps `self` with a predicate that takes a span or event's [`Metadata`]

- ```rust
  fn and<B>(self: Self, other: B) -> Tee<Self, B>
where
    Self: Sized,
    B: MakeWriter<''a> + Sized { /* ... */ }
  ```
  Combines `self` with another type implementing [`MakeWriter`], returning

- ```rust
  fn or_else<W, B>(self: Self, other: B) -> OrElse<Self, B>
where
    Self: MakeWriter<''a, Writer = OptionalWriter<W>> + Sized,
    B: MakeWriter<''a> + Sized,
    W: Write { /* ... */ }
  ```
  Combines `self` with another type implementing [`MakeWriter`], returning

##### Implementations

This trait is implemented for the following types:

- `M` with <''a, M>

### Types

#### Struct `Subscriber`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

A `Subscriber` that logs formatted representations of `tracing` events.

This consists of an inner `Formatter` wrapped in a layer that performs filtering.

```rust
pub struct Subscriber<N = format::DefaultFields, E = format::Format<format::Full>, F = crate::filter::LevelFilter, W = fn() -> io::Stdout> {
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
  pub fn builder() -> SubscriberBuilder { /* ... */ }
  ```
  Returns a new `SubscriberBuilder` for configuring a format subscriber.

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Returns a new format subscriber with the default configuration.

###### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Subscriber**
  - ```rust
    fn register_callsite(self: &Self, meta: &''static Metadata<''static>) -> Interest { /* ... */ }
    ```

  - ```rust
    fn enabled(self: &Self, meta: &Metadata<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn new_span(self: &Self, attrs: &span::Attributes<''_>) -> span::Id { /* ... */ }
    ```

  - ```rust
    fn record(self: &Self, span: &span::Id, values: &span::Record<''_>) { /* ... */ }
    ```

  - ```rust
    fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id) { /* ... */ }
    ```

  - ```rust
    fn event_enabled(self: &Self, event: &Event<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn event(self: &Self, event: &Event<''_>) { /* ... */ }
    ```

  - ```rust
    fn enter(self: &Self, id: &span::Id) { /* ... */ }
    ```

  - ```rust
    fn exit(self: &Self, id: &span::Id) { /* ... */ }
    ```

  - ```rust
    fn current_span(self: &Self) -> span::Current { /* ... */ }
    ```

  - ```rust
    fn clone_span(self: &Self, id: &span::Id) -> span::Id { /* ... */ }
    ```

  - ```rust
    fn try_close(self: &Self, id: span::Id) -> bool { /* ... */ }
    ```

  - ```rust
    fn max_level_hint(self: &Self) -> Option<tracing_core::LevelFilter> { /* ... */ }
    ```

  - ```rust
    unsafe fn downcast_raw(self: &Self, id: TypeId) -> Option<*const ()> { /* ... */ }
    ```

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

- **LookupSpan**
  - ```rust
    fn span_data(self: &''a Self, id: &span::Id) -> Option<<Self as >::Data> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **SubscriberInitExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **SubscriberExt**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Type Alias `Formatter`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

A `Subscriber` that logs formatted representations of `tracing` events.
This type only logs formatted events; it does not perform any filtering.

```rust
pub type Formatter<N = format::DefaultFields, E = format::Format<format::Full>, W = fn() -> io::Stdout> = layer::Layered<fmt_layer::Layer<crate::registry::Registry, N, E, W>, crate::registry::Registry>;
```

#### Struct `SubscriberBuilder`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`
- `#[must_use]`

Configures and constructs `Subscriber`s.

```rust
pub struct SubscriberBuilder<N = format::DefaultFields, E = format::Format<format::Full>, F = crate::filter::LevelFilter, W = fn() -> io::Stdout> {
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
  pub fn finish(self: Self) -> Subscriber<N, E, F, W> { /* ... */ }
  ```
  Finish the builder, returning a new `FmtSubscriber`.

- ```rust
  pub fn try_init(self: Self) -> Result<(), Box<dyn Error + Send + Sync + ''static>> { /* ... */ }
  ```
  Install this Subscriber as the global default if one is

- ```rust
  pub fn init(self: Self) { /* ... */ }
  ```
  Install this Subscriber as the global default.

- ```rust
  pub fn with_timer<T2>(self: Self, timer: T2) -> SubscriberBuilder<N, format::Format<L, T2>, F, W> { /* ... */ }
  ```
  Use the given [`timer`] for log message timestamps.

- ```rust
  pub fn without_time(self: Self) -> SubscriberBuilder<N, format::Format<L, ()>, F, W> { /* ... */ }
  ```
  Do not emit timestamps with log messages.

- ```rust
  pub fn with_span_events(self: Self, kind: format::FmtSpan) -> Self { /* ... */ }
  ```
  Configures how synthesized events are emitted at points in the [span

- ```rust
  pub fn with_ansi(self: Self, ansi: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W> { /* ... */ }
  ```
  Sets whether or not the formatter emits ANSI terminal escape codes

- ```rust
  pub fn log_internal_errors(self: Self, log_internal_errors: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W> { /* ... */ }
  ```
  Sets whether to write errors from [`FormatEvent`] to the writer.

- ```rust
  pub fn with_target(self: Self, display_target: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W> { /* ... */ }
  ```
  Sets whether or not an event's target is displayed.

- ```rust
  pub fn with_file(self: Self, display_filename: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W> { /* ... */ }
  ```
  Sets whether or not an event's [source code file path][file] is

- ```rust
  pub fn with_line_number(self: Self, display_line_number: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W> { /* ... */ }
  ```
  Sets whether or not an event's [source code line number][line] is

- ```rust
  pub fn with_level(self: Self, display_level: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W> { /* ... */ }
  ```
  Sets whether or not an event's level is displayed.

- ```rust
  pub fn with_thread_names(self: Self, display_thread_names: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W> { /* ... */ }
  ```
  Sets whether or not the [name] of the current thread is displayed

- ```rust
  pub fn with_thread_ids(self: Self, display_thread_ids: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W> { /* ... */ }
  ```
  Sets whether or not the [thread ID] of the current thread is displayed

- ```rust
  pub fn compact(self: Self) -> SubscriberBuilder<N, format::Format<format::Compact, T>, F, W>
where
    N: for<''writer> FormatFields<''writer> + ''static { /* ... */ }
  ```
  Sets the subscriber being built to use a less verbose formatter.

- ```rust
  pub fn pretty(self: Self) -> SubscriberBuilder<format::Pretty, format::Format<format::Pretty, T>, F, W> { /* ... */ }
  ```
  Sets the subscriber being built to use an [excessively pretty, human-readable formatter](crate::fmt::format::Pretty).

- ```rust
  pub fn fmt_fields<N2>(self: Self, fmt_fields: N2) -> SubscriberBuilder<N2, E, F, W>
where
    N2: for<''writer> FormatFields<''writer> + ''static { /* ... */ }
  ```
  Sets the field formatter that the subscriber being built will use to record

- ```rust
  pub fn with_max_level</* synthetic */ impl Into<LevelFilter>: Into<LevelFilter>>(self: Self, filter: impl Into<LevelFilter>) -> SubscriberBuilder<N, E, LevelFilter, W> { /* ... */ }
  ```
  Sets the maximum [verbosity level] that will be enabled by the

- ```rust
  pub fn event_format<E2>(self: Self, fmt_event: E2) -> SubscriberBuilder<N, E2, F, W>
where
    E2: FormatEvent<Registry, N> + ''static,
    N: for<''writer> FormatFields<''writer> + ''static,
    W: for<''writer> MakeWriter<''writer> + ''static { /* ... */ }
  ```
  Sets the [event formatter][`FormatEvent`] that the subscriber being built

- ```rust
  pub fn with_writer<W2>(self: Self, make_writer: W2) -> SubscriberBuilder<N, E, F, W2>
where
    W2: for<''writer> MakeWriter<''writer> + ''static { /* ... */ }
  ```
  Sets the [`MakeWriter`] that the subscriber being built will use to write events.

- ```rust
  pub fn with_test_writer(self: Self) -> SubscriberBuilder<N, E, F, TestWriter> { /* ... */ }
  ```
  Configures the subscriber to support [`libtest`'s output capturing][capturing] when used in

- ```rust
  pub fn map_event_format<E2, /* synthetic */ impl FnOnce(E) -> E2: FnOnce(E) -> E2>(self: Self, f: impl FnOnce(E) -> E2) -> SubscriberBuilder<N, E2, F, W>
where
    E2: FormatEvent<Registry, N> + ''static,
    N: for<''writer> FormatFields<''writer> + ''static,
    W: for<''writer> MakeWriter<''writer> + ''static { /* ... */ }
  ```
  Updates the event formatter by applying a function to the existing event formatter.

- ```rust
  pub fn map_fmt_fields<N2, /* synthetic */ impl FnOnce(N) -> N2: FnOnce(N) -> N2>(self: Self, f: impl FnOnce(N) -> N2) -> SubscriberBuilder<N2, E, F, W>
where
    N2: for<''writer> FormatFields<''writer> + ''static { /* ... */ }
  ```
  Updates the field formatter by applying a function to the existing field formatter.

- ```rust
  pub fn map_writer<W2, /* synthetic */ impl FnOnce(W) -> W2: FnOnce(W) -> W2>(self: Self, f: impl FnOnce(W) -> W2) -> SubscriberBuilder<N, E, F, W2>
where
    W2: for<''writer> MakeWriter<''writer> + ''static { /* ... */ }
  ```
  Updates the [`MakeWriter`] by applying a function to the existing [`MakeWriter`].

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

  - ```rust
    fn from(builder: SubscriberBuilder<N, E, F, W>) -> tracing_core::Dispatch { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **SubscriberInitExt**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `fmt`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

Returns a new [`SubscriberBuilder`] for configuring a [formatting subscriber].

This is essentially shorthand for [`SubscriberBuilder::default()]`.

# Examples

Using [`init`] to set the default subscriber:

```rust
tracing_subscriber::fmt().init();
```

Configuring the output format:

```rust

tracing_subscriber::fmt()
    // Configure formatting settings.
    .with_target(false)
    .with_timer(tracing_subscriber::fmt::time::uptime())
    .with_level(true)
    // Set the subscriber as the default.
    .init();
```

[`try_init`] returns an error if the default subscriber could not be set:

```rust
use std::error::Error;

fn init_subscriber() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        // Configure the subscriber to emit logs in JSON format.
        .json()
        // Configure the subscriber to flatten event fields in the output JSON objects.
        .flatten_event(true)
        // Set the subscriber as the default, returning an error if this fails.
        .try_init()?;

    Ok(())
}
```

Rather than setting the subscriber as the default, [`finish`] _returns_ the
constructed subscriber, which may then be passed to other functions:

```rust
let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .compact()
    .finish();

tracing::subscriber::with_default(subscriber, || {
    // the subscriber will only be set as the default
    // inside this closure...
})
```

[formatting subscriber]: Subscriber
[`SubscriberBuilder::default()`]: SubscriberBuilder::default
[`init`]: SubscriberBuilder::init()
[`try_init`]: SubscriberBuilder::try_init()
[`finish`]: SubscriberBuilder::finish()

```rust
pub fn fmt() -> SubscriberBuilder { /* ... */ }
```

#### Function `layer`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

Returns a new [formatting layer] that can be [composed] with other layers to
construct a [`Subscriber`].

This is a shorthand for the equivalent [`Layer::default()`] function.

[formatting layer]: Layer
[composed]: crate::layer
[`Layer::default()`]: Layer::default

```rust
pub fn layer<S>() -> Layer<S> { /* ... */ }
```

#### Function `try_init`

Install a global tracing subscriber that listens for events and
filters based on the value of the [`RUST_LOG` environment variable],
if one is not already set.

If the `tracing-log` feature is enabled, this will also install
the [`LogTracer`] to convert `log` records into `tracing` `Event`s.

This is shorthand for

```rust
# fn doc() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
tracing_subscriber::fmt().try_init()
# }
```


# Errors

Returns an Error if the initialization was unsuccessful,
likely because a global subscriber was already installed by another
call to `try_init`.

[`LogTracer`]:
    https://docs.rs/tracing-log/0.1.0/tracing_log/struct.LogTracer.html
[`RUST_LOG` environment variable]: crate::filter::EnvFilter::DEFAULT_ENV

```rust
pub fn try_init() -> Result<(), Box<dyn Error + Send + Sync + ''static>> { /* ... */ }
```

#### Function `init`

Install a global tracing subscriber that listens for events and
filters based on the value of the [`RUST_LOG` environment variable].

The configuration of the subscriber initialized by this function
depends on what [feature flags](crate#feature-flags) are enabled.

If the `tracing-log` feature is enabled, this will also install
the LogTracer to convert `Log` records into `tracing` `Event`s.

If the `env-filter` feature is enabled, this is shorthand for

```rust
# use tracing_subscriber::EnvFilter;
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();
```

# Panics
Panics if the initialization was unsuccessful, likely because a
global subscriber was already installed by another call to `try_init`.

[`RUST_LOG` environment variable]: crate::filter::EnvFilter::DEFAULT_ENV

```rust
pub fn init() { /* ... */ }
```

### Re-exports

#### Re-export `FmtContext`

```rust
pub use fmt_layer::FmtContext;
```

#### Re-export `FormattedFields`

```rust
pub use fmt_layer::FormattedFields;
```

#### Re-export `Layer`

```rust
pub use fmt_layer::Layer;
```

#### Re-export `format`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::format::format;
```

#### Re-export `FormatEvent`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::format::FormatEvent;
```

#### Re-export `FormatFields`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::format::FormatFields;
```

#### Re-export `time`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::time::time;
```

#### Re-export `MakeWriter`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::writer::MakeWriter;
```

#### Re-export `TestWriter`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::writer::TestWriter;
```

## Functions

### Function `registry`

**Attributes:**

- `#[<cfg>(all(feature = "registry", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "registry", feature = "std"))))]`

Returns a default [`Registry`].

```rust
pub fn registry() -> Registry { /* ... */ }
```

## Re-exports

### Re-export `fmt`

**Attributes:**

- `#[<cfg>(all(feature = "fmt", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

```rust
pub use fmt::fmt;
```

### Re-export `Subscriber`

**Attributes:**

- `#[<cfg>(all(feature = "fmt", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "fmt", feature = "std"))))]`

```rust
pub use fmt::Subscriber as FmtSubscriber;
```

### Re-export `Layer`

```rust
pub use layer::Layer;
```

### Re-export `Registry`

**Attributes:**

- `#[<cfg>(all(feature = "registry", feature = "std"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(feature = "registry", feature = "std"))))]`

```rust
pub use registry::Registry;
```

