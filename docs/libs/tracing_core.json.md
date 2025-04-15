# Crate Documentation

**Version:** 0.1.33

**Format Version:** 43

# Module `tracing_core`

Core primitives for `tracing`.

[`tracing`] is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information. This crate defines the core
primitives of `tracing`.

This crate provides:

* [`span::Id`] identifies a span within the execution of a program.

* [`Event`] represents a single event within a trace.

* [`Subscriber`], the trait implemented to collect trace data.

* [`Metadata`] and [`Callsite`] provide information describing spans and
  `Event`s.

* [`Field`], [`FieldSet`], [`Value`], and [`ValueSet`] represent the
  structured data attached to a span.

* [`Dispatch`] allows spans and events to be dispatched to `Subscriber`s.

In addition, it defines the global callsite registry and per-thread current
dispatcher which other components of the tracing system rely on.

*Compiler support: [requires `rustc` 1.63+][msrv]*

[msrv]: #supported-rust-versions

## Usage

Application authors will typically not use this crate directly. Instead,
they will use the [`tracing`] crate, which provides a much more
fully-featured API. However, this crate's API will change very infrequently,
so it may be used when dependencies must be very stable.

`Subscriber` implementations may depend on `tracing-core` rather than
`tracing`, as the additional APIs provided by `tracing` are primarily useful
for instrumenting libraries and applications, and are generally not
necessary for `Subscriber` implementations.

The [`tokio-rs/tracing`] repository contains less stable crates designed to
be used with the `tracing` ecosystem. It includes a collection of
`Subscriber` implementations, as well as utility and adapter crates.

## Crate Feature Flags

The following crate [feature flags] are available:

* `std`: Depend on the Rust standard library (enabled by default).

  `no_std` users may disable this feature with `default-features = false`:

  ```toml
  [dependencies]
  tracing-core = { version = "0.1.22", default-features = false }
  ```

  **Note**:`tracing-core`'s `no_std` support requires `liballoc`.

### Unstable Features

These feature flags enable **unstable** features. The public API may break in 0.1.x
releases. To enable these features, the `--cfg tracing_unstable` must be passed to
`rustc` when compiling.

The following unstable feature flags are currently available:

* `valuable`: Enables support for recording [field values] using the
  [`valuable`] crate.

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
[field values]: crate::field
[`valuable`]: https://crates.io/crates/valuable

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


[`span::Id`]: span::Id
[`Event`]: event::Event
[`Subscriber`]: subscriber::Subscriber
[`Metadata`]: metadata::Metadata
[`Callsite`]: callsite::Callsite
[`Field`]: field::Field
[`FieldSet`]: field::FieldSet
[`Value`]: field::Value
[`ValueSet`]: field::ValueSet
[`Dispatch`]: dispatcher::Dispatch
[`tokio-rs/tracing`]: https://github.com/tokio-rs/tracing
[`tracing`]: https://crates.io/crates/tracing

## Modules

## Module `callsite`

Callsites represent the source locations from which spans or events
originate.

# What Are Callsites?

Every span or event in `tracing` is associated with a [`Callsite`]. A
callsite is a small `static` value that is responsible for the following:

* Storing the span or event's [`Metadata`],
* Uniquely [identifying](Identifier) the span or event definition,
* Caching the subscriber's [`Interest`][^1] in that span or event, to avoid
  re-evaluating filters.

# Registering Callsites

When a span or event is recorded for the first time, its callsite
[`register`]s itself with the global callsite registry. Registering a
callsite calls the [`Subscriber::register_callsite`][`register_callsite`]
method with that callsite's [`Metadata`] on every currently active
subscriber. This serves two primary purposes: informing subscribers of the
callsite's existence, and performing static filtering.

## Callsite Existence

If a [`Subscriber`] implementation wishes to allocate storage for each
unique span/event location in the program, or pre-compute some value
that will be used to record that span or event in the future, it can
do so in its [`register_callsite`] method.

## Performing Static Filtering

The [`register_callsite`] method returns an [`Interest`] value,
which indicates that the subscriber either [always] wishes to record
that span or event, [sometimes] wishes to record it based on a
dynamic filter evaluation, or [never] wishes to record it.

When registering a new callsite, the [`Interest`]s returned by every
currently active subscriber are combined, and the result is stored at
each callsite. This way, when the span or event occurs in the
future, the cached [`Interest`] value can be checked efficiently
to determine if the span or event should be recorded, without
needing to perform expensive filtering (i.e. calling the
[`Subscriber::enabled`] method every time a span or event occurs).

### Rebuilding Cached Interest

When a new [`Dispatch`] is created (i.e. a new subscriber becomes
active), any previously cached [`Interest`] values are re-evaluated
for all callsites in the program. This way, if the new subscriber
will enable a callsite that was not previously enabled, the
[`Interest`] in that callsite is updated. Similarly, when a
subscriber is dropped, the interest cache is also re-evaluated, so
that any callsites enabled only by that subscriber are disabled.

In addition, the [`rebuild_interest_cache`] function in this module can be
used to manually invalidate all cached interest and re-register those
callsites. This function is useful in situations where a subscriber's
interest can change, but it does so relatively infrequently. The subscriber
may wish for its interest to be cached most of the time, and return
[`Interest::always`][always] or [`Interest::never`][never] in its
[`register_callsite`] method, so that its [`Subscriber::enabled`] method
doesn't need to be evaluated every time a span or event is recorded.
However, when the configuration changes, the subscriber can call
[`rebuild_interest_cache`] to re-evaluate the entire interest cache with its
new configuration. This is a relatively costly operation, but if the
configuration changes infrequently, it may be more efficient than calling
[`Subscriber::enabled`] frequently.

# Implementing Callsites

In most cases, instrumenting code using `tracing` should *not* require
implementing the [`Callsite`] trait directly. When using the [`tracing`
crate's macros][macros] or the [`#[instrument]` attribute][instrument], a
`Callsite` is automatically generated.

However, code which provides alternative forms of `tracing` instrumentation
may need to interact with the callsite system directly. If
instrumentation-side code needs to produce a `Callsite` to emit spans or
events, the [`DefaultCallsite`] struct provided in this module is a
ready-made `Callsite` implementation that is suitable for most uses. When
possible, the use of `DefaultCallsite` should be preferred over implementing
[`Callsite`] for user types, as `DefaultCallsite` may benefit from
additional performance optimizations.

[^1]: Returned by the [`Subscriber::register_callsite`][`register_callsite`]
    method.

[`Metadata`]: crate::metadata::Metadata
[`Interest`]: crate::subscriber::Interest
[`Subscriber`]: crate::subscriber::Subscriber
[`register_callsite`]: crate::subscriber::Subscriber::register_callsite
[`Subscriber::enabled`]: crate::subscriber::Subscriber::enabled
[always]: crate::subscriber::Interest::always
[sometimes]: crate::subscriber::Interest::sometimes
[never]: crate::subscriber::Interest::never
[`Dispatch`]: crate::dispatch::Dispatch
[macros]: https://docs.rs/tracing/latest/tracing/#macros
[instrument]: https://docs.rs/tracing/latest/tracing/attr.instrument.html

```rust
pub mod callsite { /* ... */ }
```

### Types

#### Struct `Identifier`

Uniquely identifies a [`Callsite`]

Two `Identifier`s are equal if they both refer to the same callsite.

[`Callsite`]: super::callsite::Callsite

```rust
pub struct Identifier(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Identifier { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Identifier) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
- **UnwindSafe**
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: Hasher { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `DefaultCallsite`

A default [`Callsite`] implementation.

```rust
pub struct DefaultCallsite {
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
  pub const fn new(meta: &''static Metadata<''static>) -> Self { /* ... */ }
  ```
  Returns a new `DefaultCallsite` with the specified `Metadata`.

- ```rust
  pub fn register(self: &''static Self) -> Interest { /* ... */ }
  ```
  Registers this callsite with the global callsite registry.

- ```rust
  pub fn interest(self: &''static Self) -> Interest { /* ... */ }
  ```
  Returns the callsite's cached `Interest`, or registers it for the

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Callsite**
  - ```rust
    fn set_interest(self: &Self, interest: Interest) { /* ... */ }
    ```

  - ```rust
    fn metadata(self: &Self) -> &Metadata<''static> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `Callsite`

Trait implemented by callsites.

These functions are only intended to be called by the callsite registry, which
correctly handles determining the common interest between all subscribers.

See the [module-level documentation](crate::callsite) for details on
callsites.

```rust
pub trait Callsite: Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `set_interest`: Sets the [`Interest`] for this callsite.
- `metadata`: Returns the [metadata] associated with the callsite.

##### Implementations

This trait is implemented for the following types:

- `DefaultCallsite`

### Functions

#### Function `rebuild_interest_cache`

Clear and reregister interest on every [`Callsite`]

This function is intended for runtime reconfiguration of filters on traces
when the filter recalculation is much less frequent than trace events are.
The alternative is to have the [`Subscriber`] that supports runtime
reconfiguration of filters always return [`Interest::sometimes()`] so that
[`enabled`] is evaluated for every event.

This function will also re-compute the global maximum level as determined by
the [`max_level_hint`] method. If a [`Subscriber`]
implementation changes the value returned by its `max_level_hint`
implementation at runtime, then it **must** call this function after that
value changes, in order for the change to be reflected.

See the [documentation on callsite interest caching][cache-docs] for
additional information on this function's usage.

[`max_level_hint`]: super::subscriber::Subscriber::max_level_hint
[`Callsite`]: super::callsite::Callsite
[`enabled`]: super::subscriber::Subscriber#tymethod.enabled
[`Interest::sometimes()`]: super::subscriber::Interest::sometimes
[`Subscriber`]: super::subscriber::Subscriber
[cache-docs]: crate::callsite#rebuilding-cached-interest

```rust
pub fn rebuild_interest_cache() { /* ... */ }
```

#### Function `register`

Register a new [`Callsite`] with the global registry.

This should be called once per callsite after the callsite has been
constructed.

See the [documentation on callsite registration][reg-docs] for details
on the global callsite registry.

[`Callsite`]: crate::callsite::Callsite
[reg-docs]: crate::callsite#registering-callsites

```rust
pub fn register(callsite: &''static dyn Callsite) { /* ... */ }
```

## Module `dispatcher`

Dispatches trace events to [`Subscriber`]s.

The _dispatcher_ is the component of the tracing system which is responsible
for forwarding trace data from the instrumentation points that generate it
to the subscriber that collects it.

# Using the Trace Dispatcher

Every thread in a program using `tracing` has a _default subscriber_. When
events occur, or spans are created, they are dispatched to the thread's
current subscriber.

## Setting the Default Subscriber

By default, the current subscriber is an empty implementation that does
nothing. To use a subscriber implementation, it must be set as the default.
There are two methods for doing so: [`with_default`] and
[`set_global_default`]. `with_default` sets the default subscriber for the
duration of a scope, while `set_global_default` sets a default subscriber
for the entire process.

To use either of these functions, we must first wrap our subscriber in a
[`Dispatch`], a cloneable, type-erased reference to a subscriber. For
example:
```rust
# pub struct FooSubscriber;
# use tracing_core::{
#   dispatcher, Event, Metadata,
#   span::{Attributes, Id, Record}
# };
# impl tracing_core::Subscriber for FooSubscriber {
#   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
#   fn record(&self, _: &Id, _: &Record) {}
#   fn event(&self, _: &Event) {}
#   fn record_follows_from(&self, _: &Id, _: &Id) {}
#   fn enabled(&self, _: &Metadata) -> bool { false }
#   fn enter(&self, _: &Id) {}
#   fn exit(&self, _: &Id) {}
# }
# impl FooSubscriber { fn new() -> Self { FooSubscriber } }
use dispatcher::Dispatch;

let my_subscriber = FooSubscriber::new();
let my_dispatch = Dispatch::new(my_subscriber);
```
Then, we can use [`with_default`] to set our `Dispatch` as the default for
the duration of a block:
```rust
# pub struct FooSubscriber;
# use tracing_core::{
#   dispatcher, Event, Metadata,
#   span::{Attributes, Id, Record}
# };
# impl tracing_core::Subscriber for FooSubscriber {
#   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
#   fn record(&self, _: &Id, _: &Record) {}
#   fn event(&self, _: &Event) {}
#   fn record_follows_from(&self, _: &Id, _: &Id) {}
#   fn enabled(&self, _: &Metadata) -> bool { false }
#   fn enter(&self, _: &Id) {}
#   fn exit(&self, _: &Id) {}
# }
# impl FooSubscriber { fn new() -> Self { FooSubscriber } }
# let my_subscriber = FooSubscriber::new();
# let my_dispatch = dispatcher::Dispatch::new(my_subscriber);
// no default subscriber

# #[cfg(feature = "std")]
dispatcher::with_default(&my_dispatch, || {
    // my_subscriber is the default
});

// no default subscriber again
```
It's important to note that `with_default` will not propagate the current
thread's default subscriber to any threads spawned within the `with_default`
block. To propagate the default subscriber to new threads, either use
`with_default` from the new thread, or use `set_global_default`.

As an alternative to `with_default`, we can use [`set_global_default`] to
set a `Dispatch` as the default for all threads, for the lifetime of the
program. For example:
```rust
# pub struct FooSubscriber;
# use tracing_core::{
#   dispatcher, Event, Metadata,
#   span::{Attributes, Id, Record}
# };
# impl tracing_core::Subscriber for FooSubscriber {
#   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
#   fn record(&self, _: &Id, _: &Record) {}
#   fn event(&self, _: &Event) {}
#   fn record_follows_from(&self, _: &Id, _: &Id) {}
#   fn enabled(&self, _: &Metadata) -> bool { false }
#   fn enter(&self, _: &Id) {}
#   fn exit(&self, _: &Id) {}
# }
# impl FooSubscriber { fn new() -> Self { FooSubscriber } }
# let my_subscriber = FooSubscriber::new();
# let my_dispatch = dispatcher::Dispatch::new(my_subscriber);
// no default subscriber

dispatcher::set_global_default(my_dispatch)
    // `set_global_default` will return an error if the global default
    // subscriber has already been set.
    .expect("global default was already set!");

// `my_subscriber` is now the default
```

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>:the thread-local scoped dispatcher
    (<a href="#fn.with_default"><code>with_default</code></a>) requires the
    Rust standard library. <code>no_std</code> users should use
    <a href="#fn.set_global_default"><code>set_global_default</code></a>
    instead.
</pre>

## Accessing the Default Subscriber

A thread's current default subscriber can be accessed using the
[`get_default`] function, which executes a closure with a reference to the
currently default `Dispatch`. This is used primarily by `tracing`
instrumentation.


```rust
pub mod dispatcher { /* ... */ }
```

### Types

#### Struct `Dispatch`

`Dispatch` trace data to a [`Subscriber`].

```rust
pub struct Dispatch {
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
  pub fn none() -> Self { /* ... */ }
  ```
  Returns a new `Dispatch` that discards events and spans.

- ```rust
  pub fn new<S>(subscriber: S) -> Self
where
    S: Subscriber + Send + Sync + ''static { /* ... */ }
  ```
  Returns a `Dispatch` that forwards to the given [`Subscriber`].

- ```rust
  pub fn downgrade(self: &Self) -> WeakDispatch { /* ... */ }
  ```
  Creates a [`WeakDispatch`] from this `Dispatch`.

- ```rust
  pub fn register_callsite(self: &Self, metadata: &''static Metadata<''static>) -> subscriber::Interest { /* ... */ }
  ```
  Registers a new callsite with this subscriber, returning whether or not

- ```rust
  pub fn new_span(self: &Self, span: &span::Attributes<''_>) -> span::Id { /* ... */ }
  ```
  Record the construction of a new span, returning a new [ID] for the

- ```rust
  pub fn record(self: &Self, span: &span::Id, values: &span::Record<''_>) { /* ... */ }
  ```
  Record a set of values on a span.

- ```rust
  pub fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id) { /* ... */ }
  ```
  Adds an indication that `span` follows from the span with the id

- ```rust
  pub fn enabled(self: &Self, metadata: &Metadata<''_>) -> bool { /* ... */ }
  ```
  Returns true if a span with the specified [metadata] would be

- ```rust
  pub fn event(self: &Self, event: &Event<''_>) { /* ... */ }
  ```
  Records that an [`Event`] has occurred.

- ```rust
  pub fn enter(self: &Self, span: &span::Id) { /* ... */ }
  ```
  Records that a span has been can_enter.

- ```rust
  pub fn exit(self: &Self, span: &span::Id) { /* ... */ }
  ```
  Records that a span has been exited.

- ```rust
  pub fn clone_span(self: &Self, id: &span::Id) -> span::Id { /* ... */ }
  ```
  Notifies the subscriber that a [span ID] has been cloned.

- ```rust
  pub fn drop_span(self: &Self, id: span::Id) { /* ... */ }
  ```
  Notifies the subscriber that a [span ID] has been dropped.

- ```rust
  pub fn try_close(self: &Self, id: span::Id) -> bool { /* ... */ }
  ```
  Notifies the subscriber that a [span ID] has been dropped, and returns

- ```rust
  pub fn current_span(self: &Self) -> span::Current { /* ... */ }
  ```
  Returns a type representing this subscriber's view of the current span.

- ```rust
  pub fn is<T: Any>(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this `Dispatch` forwards to a `Subscriber` of type

- ```rust
  pub fn downcast_ref<T: Any>(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Returns some reference to the `Subscriber` this `Dispatch` forwards to

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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
    fn clone(self: &Self) -> Dispatch { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(subscriber: S) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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
    Returns the current default dispatcher

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `WeakDispatch`

`WeakDispatch` is a version of [`Dispatch`] that holds a non-owning reference
to a [`Subscriber`].

The Subscriber` may be accessed by calling [`WeakDispatch::upgrade`],
which returns an `Option<Dispatch>`. If all [`Dispatch`] clones that point
at the `Subscriber` have been dropped, [`WeakDispatch::upgrade`] will return
`None`. Otherwise, it will return `Some(Dispatch)`.

A `WeakDispatch` may be created from a [`Dispatch`] by calling the
[`Dispatch::downgrade`] method. The primary use for creating a
[`WeakDispatch`] is to allow a Subscriber` to hold a cyclical reference to
itself without creating a memory leak. See [here] for details.

This type is analogous to the [`std::sync::Weak`] type, but for a
[`Dispatch`] rather than an [`Arc`].

[`Arc`]: std::sync::Arc
[here]: Subscriber#avoiding-memory-leaks

```rust
pub struct WeakDispatch {
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
  pub fn upgrade(self: &Self) -> Option<Dispatch> { /* ... */ }
  ```
  Attempts to upgrade this `WeakDispatch` to a [`Dispatch`].

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WeakDispatch { /* ... */ }
    ```

#### Struct `DefaultGuard`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

A guard that resets the current default dispatcher to the prior
default dispatcher when dropped.

```rust
pub struct DefaultGuard(/* private field */);
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

- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
#### Struct `SetGlobalDefaultError`

Returned if setting the global dispatcher fails.

```rust
pub struct SetGlobalDefaultError {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Error**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
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

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `with_default`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Sets this dispatch as the default for the duration of a closure.

The default dispatcher is used when creating a new [span] or
[`Event`].

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This function required the Rust standard library.
    <code>no_std</code> users should use <a href="../fn.set_global_default.html">
    <code>set_global_default</code></a> instead.
</pre>

[span]: super::span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event
[`set_global_default`]: super::set_global_default

```rust
pub fn with_default<T, /* synthetic */ impl FnOnce() -> T: FnOnce() -> T>(dispatcher: &Dispatch, f: impl FnOnce() -> T) -> T { /* ... */ }
```

#### Function `set_default`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[must_use = "Dropping the guard unregisters the dispatcher."]`

Sets the dispatch as the default dispatch for the duration of the lifetime
of the returned DefaultGuard

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This function required the Rust standard library.
    <code>no_std</code> users should use <a href="../fn.set_global_default.html">
    <code>set_global_default</code></a> instead.
</pre>

[`set_global_default`]: super::set_global_default

```rust
pub fn set_default(dispatcher: &Dispatch) -> DefaultGuard { /* ... */ }
```

#### Function `set_global_default`

Sets this dispatch as the global default for the duration of the entire program.
Will be used as a fallback if no thread-local dispatch has been set in a thread
(using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail.
Returns `Err` if the global default has already been set.

<div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
    <strong>Warning</strong>: In general, libraries should <em>not</em> call
    <code>set_global_default()</code>! Doing so will cause conflicts when
    executables that depend on the library try to set the default later.
</pre></div>

[span]: super::span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

```rust
pub fn set_global_default(dispatcher: Dispatch) -> Result<(), SetGlobalDefaultError> { /* ... */ }
```

#### Function `get_default`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Executes a closure with a reference to this thread's current [dispatcher].

Note that calls to `get_default` should not be nested; if this function is
called while inside of another `get_default`, that closure will be provided
with `Dispatch::none` rather than the previously set dispatcher.

[dispatcher]: super::dispatcher::Dispatch

```rust
pub fn get_default<T, F>(f: F) -> T
where
    F: FnMut(&Dispatch) -> T { /* ... */ }
```

## Module `event`

Events represent single points in time during the execution of a program.

```rust
pub mod event { /* ... */ }
```

### Types

#### Struct `Event`

`Event`s represent single points in time where something occurred during the
execution of a program.

An `Event` can be compared to a log record in unstructured logging, but with
two key differences:
- `Event`s exist _within the context of a [span]_. Unlike log lines, they
  may be located within the trace tree, allowing visibility into the
  _temporal_ context in which the event occurred, as well as the source
  code location.
- Like spans, `Event`s have structured key-value data known as _[fields]_,
  which may include textual message. In general, a majority of the data
  associated with an event should be in the event's fields rather than in
  the textual message, as the fields are more structured.

[span]: super::span
[fields]: super::field

```rust
pub struct Event<''a> {
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
  pub fn dispatch(metadata: &''static Metadata<''static>, fields: &''a field::ValueSet<''_>) { /* ... */ }
  ```
  Constructs a new `Event` with the specified metadata and set of values,

- ```rust
  pub fn new(metadata: &''static Metadata<''static>, fields: &''a field::ValueSet<''a>) -> Self { /* ... */ }
  ```
  Returns a new `Event` in the current span, with the specified metadata

- ```rust
  pub fn new_child_of</* synthetic */ impl Into<Option<Id>>: Into<Option<Id>>>(parent: impl Into<Option<Id>>, metadata: &''static Metadata<''static>, fields: &''a field::ValueSet<''a>) -> Self { /* ... */ }
  ```
  Returns a new `Event` as a child of the specified span, with the

- ```rust
  pub fn child_of</* synthetic */ impl Into<Option<Id>>: Into<Option<Id>>>(parent: impl Into<Option<Id>>, metadata: &''static Metadata<''static>, fields: &''a field::ValueSet<''_>) { /* ... */ }
  ```
  Constructs a new `Event` with the specified metadata and set of values,

- ```rust
  pub fn record(self: &Self, visitor: &mut dyn field::Visit) { /* ... */ }
  ```
  Visits all the fields on this `Event` with the specified [visitor].

- ```rust
  pub fn fields(self: &Self) -> field::Iter { /* ... */ }
  ```
  Returns an iterator over the set of values on this `Event`.

- ```rust
  pub fn metadata(self: &Self) -> &''static Metadata<''static> { /* ... */ }
  ```
  Returns [metadata] describing this `Event`.

- ```rust
  pub fn is_root(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the new event should be a root.

- ```rust
  pub fn is_contextual(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the new event's parent should be determined based on the

- ```rust
  pub fn parent(self: &Self) -> Option<&Id> { /* ... */ }
  ```
  Returns the new event's explicitly-specified parent, if there is one.

###### Trait Implementations

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

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

## Module `field`

`Span` and `Event` key-value data.

Spans and events may be annotated with key-value data, known as _fields_.
These fields consist of a mapping from a key (corresponding to a `&str` but
represented internally as an array index) to a [`Value`].

# `Value`s and `Subscriber`s

`Subscriber`s consume `Value`s as fields attached to [span]s or [`Event`]s.
The set of field keys on a given span or event is defined on its [`Metadata`].
When a span is created, it provides [`Attributes`] to the `Subscriber`'s
[`new_span`] method, containing any fields whose values were provided when
the span was created; and may call the `Subscriber`'s [`record`] method
with additional [`Record`]s if values are added for more of its fields.
Similarly, the [`Event`] type passed to the subscriber's [`event`] method
will contain any fields attached to each event.

`tracing` represents values as either one of a set of Rust primitives
(`i64`, `u64`, `f64`, `i128`, `u128`, `bool`, and `&str`) or using a
`fmt::Display` or `fmt::Debug` implementation. `Subscriber`s are provided
these primitive value types as `dyn Value` trait objects.

These trait objects can be formatted using `fmt::Debug`, but may also be
recorded as typed data by calling the [`Value::record`] method on these
trait objects with a _visitor_ implementing the [`Visit`] trait. This trait
represents the behavior used to record values of various types. For example,
an implementation of `Visit` might record integers by incrementing counters
for their field names rather than printing them.


# Using `valuable`

`tracing`'s [`Value`] trait is intentionally minimalist: it supports only a small
number of Rust primitives as typed values, and only permits recording
user-defined types with their [`fmt::Debug`] or [`fmt::Display`]
implementations. However, there are some cases where it may be useful to record
nested values (such as arrays, `Vec`s, or `HashMap`s containing values), or
user-defined `struct` and `enum` types without having to format them as
unstructured text.

To address `Value`'s limitations, `tracing` offers experimental support for
the [`valuable`] crate, which provides object-safe inspection of structured
values. User-defined types can implement the [`valuable::Valuable`] trait,
and be recorded as a `tracing` field by calling their [`as_value`] method.
If the [`Subscriber`] also supports the `valuable` crate, it can
then visit those types fields as structured values using `valuable`.

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: <code>valuable</code> support is an
    <a href = "../index.html#unstable-features">unstable feature</a>. See
    the documentation on unstable features for details on how to enable it.
</pre>

For example:
```ignore
// Derive `Valuable` for our types:
use valuable::Valuable;

#[derive(Clone, Debug, Valuable)]
struct User {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Clone, Debug, Valuable)]
struct Address {
    country: String,
    city: String,
    street: String,
}

let user = User {
    name: "Arwen Undomiel".to_string(),
    age: 3000,
    address: Address {
        country: "Middle Earth".to_string(),
        city: "Rivendell".to_string(),
        street: "leafy lane".to_string(),
    },
};

// Recording `user` as a `valuable::Value` will allow the `tracing` subscriber
// to traverse its fields as a nested, typed structure:
tracing::info!(current_user = user.as_value());
```

Alternatively, the [`valuable()`] function may be used to convert a type
implementing [`Valuable`] into a `tracing` field value.

When the `valuable` feature is enabled, the [`Visit`] trait will include an
optional [`record_value`] method. `Visit` implementations that wish to
record `valuable` values can implement this method with custom behavior.
If a visitor does not implement `record_value`, the [`valuable::Value`] will
be forwarded to the visitor's [`record_debug`] method.

[`valuable`]: https://crates.io/crates/valuable
[`as_value`]: valuable::Valuable::as_value
[`Subscriber`]: crate::Subscriber
[`record_value`]: Visit::record_value
[`record_debug`]: Visit::record_debug

[span]: super::span
[`Event`]: super::event::Event
[`Metadata`]: super::metadata::Metadata
[`Attributes`]:  super::span::Attributes
[`Record`]: super::span::Record
[`new_span`]: super::subscriber::Subscriber::new_span
[`record`]: super::subscriber::Subscriber::record
[`event`]:  super::subscriber::Subscriber::event
[`Value::record`]: Value::record

```rust
pub mod field { /* ... */ }
```

### Types

#### Struct `Field`

An opaque key allowing _O_(1) access to a field in a `Span`'s key-value
data.

As keys are defined by the _metadata_ of a span, rather than by an
individual instance of a span, a key may be used to access the same field
across all instances of a given span with the same metadata. Thus, when a
subscriber observes a new span, it need only access a field by name _once_,
and use the key for that name for all other accesses.

```rust
pub struct Field {
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
  pub fn callsite(self: &Self) -> callsite::Identifier { /* ... */ }
  ```
  Returns an [`Identifier`] that uniquely identifies the [`Callsite`]

- ```rust
  pub fn name(self: &Self) -> &''static str { /* ... */ }
  ```
  Returns a string representing the name of the field.

- ```rust
  pub fn index(self: &Self) -> usize { /* ... */ }
  ```
  Returns the index of this field in its [`FieldSet`].

###### Trait Implementations

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: Hasher { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Empty`

An empty field.

This can be used to indicate that the value of a field is not currently
present but will be recorded later.

When a field's value is `Empty`. it will not be recorded.

```rust
pub struct Empty;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Eq**
- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Value**
  - ```rust
    fn record(self: &Self, _: &Field, _: &mut dyn Visit) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Empty) -> bool { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `FieldSet`

Describes the fields present on a span.

## Equality

In well-behaved applications, two `FieldSet`s [initialized] with equal
[callsite identifiers] will have identical fields. Consequently, in release
builds, [`FieldSet::eq`] *only* checks that its arguments have equal
callsites. However, the equality of field names is checked in debug builds.

[initialized]: Self::new
[callsite identifiers]: callsite::Identifier

```rust
pub struct FieldSet {
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
  pub const fn new(names: &''static [&''static str], callsite: callsite::Identifier) -> Self { /* ... */ }
  ```
  Constructs a new `FieldSet` with the given array of field names and callsite.

- ```rust
  pub fn field<Q: Borrow<str> + ?Sized>(self: &Self, name: &Q) -> Option<Field> { /* ... */ }
  ```
  Returns the [`Field`] named `name`, or `None` if no such field exists.

- ```rust
  pub fn contains(self: &Self, field: &Field) -> bool { /* ... */ }
  ```
  Returns `true` if `self` contains the given `field`.

- ```rust
  pub fn iter(self: &Self) -> Iter { /* ... */ }
  ```
  Returns an iterator over the `Field`s in this `FieldSet`.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of fields in this `FieldSet`.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether or not this `FieldSet` has fields.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
#### Struct `ValueSet`

A set of fields and values for a span.

```rust
pub struct ValueSet<''a> {
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
  pub fn callsite(self: &Self) -> callsite::Identifier { /* ... */ }
  ```
  Returns an [`Identifier`] that uniquely identifies the [`Callsite`]

- ```rust
  pub fn record(self: &Self, visitor: &mut dyn Visit) { /* ... */ }
  ```
  Visits all the fields in this `ValueSet` with the provided [visitor].

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of fields in this `ValueSet` that would be visited

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this `ValueSet` contains _no_ values.

###### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Freeze**
- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **RefUnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `Iter`

An iterator over a set of fields.

```rust
pub struct Iter {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Field> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Send**
- **Unpin**
#### Struct `DisplayValue`

A `Value` which serializes using `fmt::Display`.

Uses `record_debug` in the `Value` implementation to
avoid an unnecessary evaluation.

```rust
pub struct DisplayValue<T: fmt::Display>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DisplayValue<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Value**
  - ```rust
    fn record(self: &Self, key: &Field, visitor: &mut dyn Visit) { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

#### Struct `DebugValue`

A `Value` which serializes as a string using `fmt::Debug`.

```rust
pub struct DebugValue<T: fmt::Debug>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Freeze**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DebugValue<T> { /* ... */ }
    ```

- **Value**
  - ```rust
    fn record(self: &Self, key: &Field, visitor: &mut dyn Visit) { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

### Traits

#### Trait `Visit`

Visits typed values.

An instance of `Visit` ("a visitor") represents the logic necessary to
record field values of various types. When an implementor of [`Value`] is
[recorded], it calls the appropriate method on the provided visitor to
indicate the type that value should be recorded as.

When a [`Subscriber`] implementation [records an `Event`] or a
[set of `Value`s added to a `Span`], it can pass an `&mut Visit` to the
`record` method on the provided [`ValueSet`] or [`Event`]. This visitor
will then be used to record all the field-value pairs present on that
`Event` or `ValueSet`.

# Examples

A simple visitor that writes to a string might be implemented like so:
```
# extern crate tracing_core as tracing;
use std::fmt::{self, Write};
use tracing::field::{Value, Visit, Field};
pub struct StringVisitor<'a> {
    string: &'a mut String,
}

impl<'a> Visit for StringVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        write!(self.string, "{} = {:?}; ", field.name(), value).unwrap();
    }
}
```
This visitor will format each recorded value using `fmt::Debug`, and
append the field name and formatted value to the provided string,
regardless of the type of the recorded value. When all the values have
been recorded, the `StringVisitor` may be dropped, allowing the string
to be printed or stored in some other data structure.

The `Visit` trait provides default implementations for `record_i64`,
`record_u64`, `record_bool`, `record_str`, and `record_error`, which simply
forward the recorded value to `record_debug`. Thus, `record_debug` is the
only method which a `Visit` implementation *must* implement. However,
visitors may override the default implementations of these functions in
order to implement type-specific behavior.

Additionally, when a visitor receives a value of a type it does not care
about, it is free to ignore those values completely. For example, a
visitor which only records numeric data might look like this:

```
# extern crate tracing_core as tracing;
# use std::fmt::{self, Write};
# use tracing::field::{Value, Visit, Field};
pub struct SumVisitor {
    sum: i64,
}

impl Visit for SumVisitor {
    fn record_i64(&mut self, _field: &Field, value: i64) {
       self.sum += value;
    }

    fn record_u64(&mut self, _field: &Field, value: u64) {
        self.sum += value as i64;
    }

    fn record_debug(&mut self, _field: &Field, _value: &fmt::Debug) {
        // Do nothing
    }
}
```

This visitor (which is probably not particularly useful) keeps a running
sum of all the numeric values it records, and ignores all other values. A
more practical example of recording typed values is presented in
`examples/counters.rs`, which demonstrates a very simple metrics system
implemented using `tracing`.

<div class="example-wrap" style="display:inline-block">
<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: The <code>record_error</code> trait method is only
available when the Rust standard library is present, as it requires the
<code>std::error::Error</code> trait.
</pre></div>

[recorded]: Value::record
[`Subscriber`]: super::subscriber::Subscriber
[records an `Event`]: super::subscriber::Subscriber::event
[set of `Value`s added to a `Span`]: super::subscriber::Subscriber::record
[`Event`]: super::event::Event

```rust
pub trait Visit {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `record_debug`: Visit a value implementing `fmt::Debug`.

##### Provided Methods

- ```rust
  fn record_f64(self: &mut Self, field: &Field, value: f64) { /* ... */ }
  ```
  Visit a double-precision floating point value.

- ```rust
  fn record_i64(self: &mut Self, field: &Field, value: i64) { /* ... */ }
  ```
  Visit a signed 64-bit integer value.

- ```rust
  fn record_u64(self: &mut Self, field: &Field, value: u64) { /* ... */ }
  ```
  Visit an unsigned 64-bit integer value.

- ```rust
  fn record_i128(self: &mut Self, field: &Field, value: i128) { /* ... */ }
  ```
  Visit a signed 128-bit integer value.

- ```rust
  fn record_u128(self: &mut Self, field: &Field, value: u128) { /* ... */ }
  ```
  Visit an unsigned 128-bit integer value.

- ```rust
  fn record_bool(self: &mut Self, field: &Field, value: bool) { /* ... */ }
  ```
  Visit a boolean value.

- ```rust
  fn record_str(self: &mut Self, field: &Field, value: &str) { /* ... */ }
  ```
  Visit a string value.

- ```rust
  fn record_bytes(self: &mut Self, field: &Field, value: &[u8]) { /* ... */ }
  ```
  Visit a byte slice.

- ```rust
  fn record_error(self: &mut Self, field: &Field, value: &dyn std::error::Error + ''static) { /* ... */ }
  ```
  Records a type implementing `Error`.

##### Implementations

This trait is implemented for the following types:

- `fmt::DebugStruct<''a, ''b>` with <''a, ''b>
- `fmt::DebugMap<''a, ''b>` with <''a, ''b>
- `F` with <F>

#### Trait `Value`

A field value of an erased type.

Implementors of `Value` may call the appropriate typed recording methods on
the [visitor] passed to their `record` method in order to indicate how
their data should be recorded.

[visitor]: Visit

```rust
pub trait Value: crate::sealed::Sealed {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `record`: Visits this value with the given `Visitor`.

##### Implementations

This trait is implemented for the following types:

- `u64`
- `NonZeroU64`
- `usize`
- `NonZeroUsize`
- `u32`
- `NonZeroU32`
- `u16`
- `NonZeroU16`
- `u8`
- `NonZeroU8`
- `i64`
- `NonZeroI64`
- `isize`
- `NonZeroIsize`
- `i32`
- `NonZeroI32`
- `i16`
- `NonZeroI16`
- `i8`
- `NonZeroI8`
- `u128`
- `NonZeroU128`
- `i128`
- `NonZeroI128`
- `bool`
- `f64`
- `f32`
- `Wrapping<T>` with <T: crate::field::Value>
- `str`
- `[u8]`
- `dyn std::error::Error + ''static`
- `dyn std::error::Error + Send + ''static`
- `dyn std::error::Error + Sync + ''static`
- `dyn std::error::Error + Send + Sync + ''static`
- `&''a T` with <''a, T>
- `&''a mut T` with <''a, T>
- `fmt::Arguments<''a>` with <''a>
- `crate::stdlib::boxed::Box<T>` with <T>
- `crate::stdlib::string::String`
- `DisplayValue<T>` with <T>
- `DebugValue<T>` with <T>
- `Empty`
- `Option<T>` with <T: Value>

### Functions

#### Function `display`

Wraps a type implementing `fmt::Display` as a `Value` that can be
recorded using its `Display` implementation.

```rust
pub fn display<T>(t: T) -> DisplayValue<T>
where
    T: fmt::Display { /* ... */ }
```

#### Function `debug`

Wraps a type implementing `fmt::Debug` as a `Value` that can be
recorded using its `Debug` implementation.

```rust
pub fn debug<T>(t: T) -> DebugValue<T>
where
    T: fmt::Debug { /* ... */ }
```

## Module `metadata`

Metadata describing trace data.

```rust
pub mod metadata { /* ... */ }
```

### Types

#### Struct `Metadata`

Metadata describing a [span] or [event].

All spans and events have the following metadata:
- A [name], represented as a static string.
- A [target], a string that categorizes part of the system where the span
  or event occurred. The `tracing` macros default to using the module
  path where the span or event originated as the target, but it may be
  overridden.
- A [verbosity level]. This determines how verbose a given span or event
  is, and allows enabling or disabling more verbose diagnostics
  situationally. See the documentation for the [`Level`] type for details.
- The names of the [fields] defined by the span or event.
- Whether the metadata corresponds to a span or event.

In addition, the following optional metadata describing the source code
location where the span or event originated _may_ be provided:
- The [file name]
- The [line number]
- The [module path]

Metadata is used by [`Subscriber`]s when filtering spans and events, and it
may also be used as part of their data payload.

When created by the `event!` or `span!` macro, the metadata describing a
particular event or span is constructed statically and exists as a single
static instance. Thus, the overhead of creating the metadata is
_significantly_ lower than that of creating the actual span. Therefore,
filtering is based on metadata, rather than on the constructed span.

## Equality

In well-behaved applications, two `Metadata` with equal
[callsite identifiers] will be equal in all other ways (i.e., have the same
`name`, `target`, etc.). Consequently, in release builds, [`Metadata::eq`]
*only* checks that its arguments have equal callsites. However, the equality
of `Metadata`'s other fields is checked in debug builds.

[span]: super::span
[event]: super::event
[name]: Self::name
[target]: Self::target
[fields]: Self::fields
[verbosity level]: Self::level
[file name]: Self::file
[line number]: Self::line
[module path]: Self::module_path
[`Subscriber`]: super::subscriber::Subscriber
[callsite identifiers]: Self::callsite

```rust
pub struct Metadata<''a> {
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
  pub const fn new(name: &''static str, target: &''a str, level: Level, file: Option<&''a str>, line: Option<u32>, module_path: Option<&''a str>, fields: field::FieldSet, kind: Kind) -> Self { /* ... */ }
  ```
  Construct new metadata for a span or event, with a name, target, level, field

- ```rust
  pub fn fields(self: &Self) -> &field::FieldSet { /* ... */ }
  ```
  Returns the names of the fields on the described span or event.

- ```rust
  pub fn level(self: &Self) -> &Level { /* ... */ }
  ```
  Returns the level of verbosity of the described span or event.

- ```rust
  pub fn name(self: &Self) -> &''static str { /* ... */ }
  ```
  Returns the name of the span.

- ```rust
  pub fn target(self: &Self) -> &''a str { /* ... */ }
  ```
  Returns a string describing the part of the system where the span or

- ```rust
  pub fn module_path(self: &Self) -> Option<&''a str> { /* ... */ }
  ```
  Returns the path to the Rust module where the span occurred, or

- ```rust
  pub fn file(self: &Self) -> Option<&''a str> { /* ... */ }
  ```
  Returns the name of the source code file where the span

- ```rust
  pub fn line(self: &Self) -> Option<u32> { /* ... */ }
  ```
  Returns the line number in the source code file where the span

- ```rust
  pub fn callsite(self: &Self) -> callsite::Identifier { /* ... */ }
  ```
  Returns an opaque `Identifier` that uniquely identifies the callsite

- ```rust
  pub fn is_event(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the callsite kind is `Event`.

- ```rust
  pub fn is_span(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the callsite kind is `Span`.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
#### Struct `Kind`

Indicates whether the callsite is a span or event.

```rust
pub struct Kind(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn is_span(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the callsite kind is `Span`

- ```rust
  pub fn is_event(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the callsite kind is `Event`

- ```rust
  pub fn is_hint(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the callsite kind is `Hint`

- ```rust
  pub const fn hint(self: Self) -> Self { /* ... */ }
  ```
  Sets that this `Kind` is a [hint](Self::HINT).

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Kind { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Kind) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Level`

Describes the level of verbosity of a span or event.

# Comparing Levels

`Level` implements the [`PartialOrd`] and [`Ord`] traits, allowing two
`Level`s to be compared to determine which is considered more or less
verbose. Levels which are more verbose are considered "greater than" levels
which are less verbose, with [`Level::ERROR`] considered the lowest, and
[`Level::TRACE`] considered the highest.

For example:
```
use tracing_core::Level;

assert!(Level::TRACE > Level::DEBUG);
assert!(Level::ERROR < Level::WARN);
assert!(Level::INFO <= Level::DEBUG);
assert_eq!(Level::TRACE, Level::TRACE);
```

# Filtering

`Level`s are typically used to implement filtering that determines which
spans and events are enabled. Depending on the use case, more or less
verbose diagnostics may be desired. For example, when running in
development, [`DEBUG`]-level traces may be enabled by default. When running in
production, only [`INFO`]-level and lower traces might be enabled. Libraries
may include very verbose diagnostics at the [`DEBUG`] and/or [`TRACE`] levels.
Applications using those libraries typically chose to ignore those traces. However, when
debugging an issue involving said libraries, it may be useful to temporarily
enable the more verbose traces.

The [`LevelFilter`] type is provided to enable filtering traces by
verbosity. `Level`s can be compared against [`LevelFilter`]s, and
[`LevelFilter`] has a variant for each `Level`, which compares analogously
to that level. In addition, [`LevelFilter`] adds a [`LevelFilter::OFF`]
variant, which is considered "less verbose" than every other `Level`. This is
intended to allow filters to completely disable tracing in a particular context.

For example:
```
use tracing_core::{Level, LevelFilter};

assert!(LevelFilter::OFF < Level::TRACE);
assert!(LevelFilter::TRACE > Level::DEBUG);
assert!(LevelFilter::ERROR < Level::WARN);
assert!(LevelFilter::INFO <= Level::DEBUG);
assert!(LevelFilter::INFO >= Level::INFO);
```

## Examples

Below is a simple example of how a [`Subscriber`] could implement filtering through
a [`LevelFilter`]. When a span or event is recorded, the [`Subscriber::enabled`] method
compares the span or event's `Level` against the configured [`LevelFilter`].
The optional [`Subscriber::max_level_hint`] method can also be implemented to allow spans
and events above a maximum verbosity level to be skipped more efficiently,
often improving performance in short-lived programs.

```
use tracing_core::{span, Event, Level, LevelFilter, Subscriber, Metadata};
# use tracing_core::span::{Id, Record, Current};

#[derive(Debug)]
pub struct MySubscriber {
    /// The most verbose level that this subscriber will enable.
    max_level: LevelFilter,

    // ...
}

impl MySubscriber {
    /// Returns a new `MySubscriber` which will record spans and events up to
    /// `max_level`.
    pub fn with_max_level(max_level: LevelFilter) -> Self {
        Self {
            max_level,
            // ...
        }
    }
}
impl Subscriber for MySubscriber {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        // A span or event is enabled if it is at or below the configured
        // maximum level.
        meta.level() <= &self.max_level
    }

    // This optional method returns the most verbose level that this
    // subscriber will enable. Although implementing this method is not
    // *required*, it permits additional optimizations when it is provided,
    // allowing spans and events above the max level to be skipped
    // more efficiently.
    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(self.max_level)
    }

    // Implement the rest of the subscriber...
    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        // ...
        # drop(span); Id::from_u64(1)
    }
    fn event(&self, event: &Event<'_>) {
        // ...
        # drop(event);
    }

    // ...
    # fn enter(&self, _: &Id) {}
    # fn exit(&self, _: &Id) {}
    # fn record(&self, _: &Id, _: &Record<'_>) {}
    # fn record_follows_from(&self, _: &Id, _: &Id) {}
}
```

It is worth noting that the `tracing-subscriber` crate provides [additional
APIs][envfilter] for performing more sophisticated filtering, such as
enabling different levels based on which module or crate a span or event is
recorded in.

[`DEBUG`]: Level::DEBUG
[`INFO`]: Level::INFO
[`TRACE`]: Level::TRACE
[`Subscriber::enabled`]: crate::subscriber::Subscriber::enabled
[`Subscriber::max_level_hint`]: crate::subscriber::Subscriber::max_level_hint
[`Subscriber`]: crate::subscriber::Subscriber
[envfilter]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html

```rust
pub struct Level(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn as_str(self: &Self) -> &''static str { /* ... */ }
  ```
  Returns the string representation of the `Level`.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(level: Level) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn lt(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn le(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn gt(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn ge(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn lt(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn le(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn gt(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn ge(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn lt(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn le(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn gt(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn ge(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, ParseLevelError> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Level { /* ... */ }
    ```

#### Struct `LevelFilter`

A filter comparable to a verbosity [`Level`].

If a [`Level`] is considered less than or equal to a `LevelFilter`, it
should be considered enabled; if greater than the `LevelFilter`, that level
is disabled. See [`LevelFilter::current`] for more details.

Note that this is essentially identical to the `Level` type, but with the
addition of an [`OFF`] level that completely disables all trace
instrumentation.

See the documentation for the [`Level`] type to see how `Level`s
and `LevelFilter`s interact.

[`OFF`]: LevelFilter::OFF

```rust
pub struct LevelFilter(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_level(level: Level) -> Self { /* ... */ }
  ```
  Returns a `LevelFilter` that enables spans and events with verbosity up

- ```rust
  pub const fn into_level(self: Self) -> Option<Level> { /* ... */ }
  ```
  Returns the most verbose [`Level`] that this filter accepts, or `None`

- ```rust
  pub fn current() -> Self { /* ... */ }
  ```
  Returns a `LevelFilter` that matches the most verbose [`Level`] that any

###### Trait Implementations

- **Send**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LevelFilter { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(from: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn lt(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn le(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn gt(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn ge(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn lt(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn le(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn gt(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn ge(self: &Self, other: &LevelFilter) -> bool { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn lt(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn le(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn gt(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

  - ```rust
    fn ge(self: &Self, other: &Level) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(level: Level) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(level: Option<Level>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(filter: LevelFilter) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

#### Struct `ParseLevelFilterError`

Indicates that a string could not be parsed to a valid level.

```rust
pub struct ParseLevelFilterError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Error**
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

- **Unpin**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParseLevelFilterError { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `ParseLevelError`

Returned if parsing a `Level` fails.

```rust
pub struct ParseLevelError {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

## Module `span`

Spans represent periods of time in the execution of a program.

```rust
pub mod span { /* ... */ }
```

### Types

#### Struct `Id`

Identifies a span within the context of a subscriber.

They are generated by [`Subscriber`]s for each span as it is created, by
the [`new_span`] trait method. See the documentation for that method for
more information on span ID generation.

[`Subscriber`]: super::subscriber::Subscriber
[`new_span`]: super::subscriber::Subscriber::new_span

```rust
pub struct Id(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_u64(u: u64) -> Self { /* ... */ }
  ```
  Constructs a new span ID from the given `u64`.

- ```rust
  pub const fn from_non_zero_u64(id: NonZeroU64) -> Self { /* ... */ }
  ```
  Constructs a new span ID from the given `NonZeroU64`.

- ```rust
  pub fn into_u64(self: &Self) -> u64 { /* ... */ }
  ```
  Returns the span's ID as a `u64`.

- ```rust
  pub const fn into_non_zero_u64(self: &Self) -> NonZeroU64 { /* ... */ }
  ```
  Returns the span's ID as a `NonZeroU64`.

###### Trait Implementations

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Id) -> bool { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
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

  - ```rust
    fn from(id: &''a Id) -> Self { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Id { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Attributes`

Attributes provided to a `Subscriber` describing a new span when it is
created.

```rust
pub struct Attributes<''a> {
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
  pub fn new(metadata: &''static Metadata<''static>, values: &''a field::ValueSet<''a>) -> Self { /* ... */ }
  ```
  Returns `Attributes` describing a new child span of the current span,

- ```rust
  pub fn new_root(metadata: &''static Metadata<''static>, values: &''a field::ValueSet<''a>) -> Self { /* ... */ }
  ```
  Returns `Attributes` describing a new span at the root of its own trace

- ```rust
  pub fn child_of(parent: Id, metadata: &''static Metadata<''static>, values: &''a field::ValueSet<''a>) -> Self { /* ... */ }
  ```
  Returns `Attributes` describing a new child span of the specified

- ```rust
  pub fn metadata(self: &Self) -> &''static Metadata<''static> { /* ... */ }
  ```
  Returns a reference to the new span's metadata.

- ```rust
  pub fn values(self: &Self) -> &field::ValueSet<''a> { /* ... */ }
  ```
  Returns a reference to a `ValueSet` containing any values the new span

- ```rust
  pub fn is_root(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the new span should be a root.

- ```rust
  pub fn is_contextual(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the new span's parent should be determined based on the

- ```rust
  pub fn parent(self: &Self) -> Option<&Id> { /* ... */ }
  ```
  Returns the new span's explicitly-specified parent, if there is one.

- ```rust
  pub fn record(self: &Self, visitor: &mut dyn field::Visit) { /* ... */ }
  ```
  Records all the fields in this set of `Attributes` with the provided

- ```rust
  pub fn contains(self: &Self, field: &field::Field) -> bool { /* ... */ }
  ```
  Returns `true` if this set of `Attributes` contains a value for the

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this set of `Attributes` contains _no_ values.

- ```rust
  pub fn fields(self: &Self) -> &FieldSet { /* ... */ }
  ```
  Returns the set of all [fields] defined by this span's [`Metadata`].

###### Trait Implementations

- **Unpin**
- **Sync**
- **RefUnwindSafe**
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

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Record`

A set of fields recorded by a span.

```rust
pub struct Record<''a> {
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
  pub fn new(values: &''a field::ValueSet<''a>) -> Self { /* ... */ }
  ```
  Constructs a new `Record` from a `ValueSet`.

- ```rust
  pub fn record(self: &Self, visitor: &mut dyn field::Visit) { /* ... */ }
  ```
  Records all the fields in this `Record` with the provided [Visitor].

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of fields that would be visited from this `Record`

- ```rust
  pub fn contains(self: &Self, field: &field::Field) -> bool { /* ... */ }
  ```
  Returns `true` if this `Record` contains a value for the given `Field`.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this `Record` contains _no_ values.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Sync**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Current`

Indicates what [the `Subscriber` considers] the "current" span.

As subscribers may not track a notion of a current span, this has three
possible states:
- "unknown", indicating that the subscriber does not track a current span,
- "none", indicating that the current context is known to not be in a span,
- "some", with the current span's [`Id`] and [`Metadata`].

[the `Subscriber` considers]: super::subscriber::Subscriber::current_span
[`Metadata`]: super::metadata::Metadata

```rust
pub struct Current {
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
  pub fn new(id: Id, metadata: &''static Metadata<''static>) -> Self { /* ... */ }
  ```
  Constructs a new `Current` that indicates the current context is a span

- ```rust
  pub fn none() -> Self { /* ... */ }
  ```
  Constructs a new `Current` that indicates the current context is *not*

- ```rust
  pub fn is_known(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the `Subscriber` that constructed this `Current` tracks a

- ```rust
  pub fn into_inner(self: Self) -> Option<(Id, &''static Metadata<''static>)> { /* ... */ }
  ```
  Consumes `self` and returns the span `Id` and `Metadata` of the current

- ```rust
  pub fn id(self: &Self) -> Option<&Id> { /* ... */ }
  ```
  Borrows the `Id` of the current span, if one exists and is known.

- ```rust
  pub fn metadata(self: &Self) -> Option<&''static Metadata<''static>> { /* ... */ }
  ```
  Borrows the `Metadata` of the current span, if one exists and is known.

###### Trait Implementations

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

  - ```rust
    fn from(cur: &''a Current) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(cur: &''a Current) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(cur: Current) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(cur: &''a Current) -> Self { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
## Module `subscriber`

Collectors collect and record trace data.

```rust
pub mod subscriber { /* ... */ }
```

### Types

#### Struct `Interest`

Indicates a [`Subscriber`]'s interest in a particular callsite.

`Subscriber`s return an `Interest` from their [`register_callsite`] methods
in order to determine whether that span should be enabled or disabled.

[`Subscriber`]: super::Subscriber
[`register_callsite`]: super::Subscriber::register_callsite

```rust
pub struct Interest(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn never() -> Self { /* ... */ }
  ```
  Returns an `Interest` indicating that the subscriber is never interested

- ```rust
  pub fn sometimes() -> Self { /* ... */ }
  ```
  Returns an `Interest` indicating the subscriber is sometimes interested

- ```rust
  pub fn always() -> Self { /* ... */ }
  ```
  Returns an `Interest` indicating the subscriber is always interested in

- ```rust
  pub fn is_never(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the subscriber is never interested in being notified

- ```rust
  pub fn is_sometimes(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the subscriber is sometimes interested in being notified

- ```rust
  pub fn is_always(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the subscriber is always interested in being notified

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **RefUnwindSafe**
- **Unpin**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Interest { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `NoSubscriber`

A no-op [`Subscriber`].

[`NoSubscriber`] implements the [`Subscriber`] trait by never being enabled,
never being interested in any callsite, and dropping all spans and events.

```rust
pub struct NoSubscriber(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Returns a new `NoSubscriber`.

###### Trait Implementations

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

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Subscriber**
  - ```rust
    fn register_callsite(self: &Self, _: &''static Metadata<''static>) -> Interest { /* ... */ }
    ```

  - ```rust
    fn new_span(self: &Self, _: &span::Attributes<''_>) -> span::Id { /* ... */ }
    ```

  - ```rust
    fn event(self: &Self, _event: &Event<''_>) { /* ... */ }
    ```

  - ```rust
    fn record(self: &Self, _span: &span::Id, _values: &span::Record<''_>) { /* ... */ }
    ```

  - ```rust
    fn record_follows_from(self: &Self, _span: &span::Id, _follows: &span::Id) { /* ... */ }
    ```

  - ```rust
    fn enabled(self: &Self, _metadata: &Metadata<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn enter(self: &Self, _span: &span::Id) { /* ... */ }
    ```

  - ```rust
    fn exit(self: &Self, _span: &span::Id) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> NoSubscriber { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NoSubscriber { /* ... */ }
    ```

- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `Subscriber`

Trait representing the functions required to collect trace data.

Crates that provide implementations of methods for collecting or recording
trace data should implement the `Subscriber` interface. This trait is
intended to represent fundamental primitives for collecting trace events and
spans — other libraries may offer utility functions and types to make
subscriber implementations more modular or improve the ergonomics of writing
subscribers.

A subscriber is responsible for the following:
- Registering new spans as they are created, and providing them with span
  IDs. Implicitly, this means the subscriber may determine the strategy for
  determining span equality.
- Recording the attachment of field values and follows-from annotations to
  spans.
- Filtering spans and events, and determining when those filters must be
  invalidated.
- Observing spans as they are entered, exited, and closed, and events as
  they occur.

When a span is entered or exited, the subscriber is provided only with the
[ID] with which it tagged that span when it was created. This means
that it is up to the subscriber to determine whether and how span _data_ —
the fields and metadata describing the span — should be stored. The
[`new_span`] function is called when a new span is created, and at that
point, the subscriber _may_ choose to store the associated data if it will
be referenced again. However, if the data has already been recorded and will
not be needed by the implementations of `enter` and `exit`, the subscriber
may freely discard that data without allocating space to store it.

## Overriding default impls

Some trait methods on `Subscriber` have default implementations, either in
order to reduce the surface area of implementing `Subscriber`, or for
backward-compatibility reasons. However, many subscribers will likely want
to override these default implementations.

The following methods are likely of interest:

- [`register_callsite`] is called once for each callsite from which a span
  event may originate, and returns an [`Interest`] value describing whether or
  not the subscriber wishes to see events or spans from that callsite. By
  default, it calls [`enabled`], and returns `Interest::always()` if
  `enabled` returns true, or `Interest::never()` if enabled returns false.
  However, if the subscriber's interest can change dynamically at runtime,
  it may want to override this function to return `Interest::sometimes()`.
  Additionally, subscribers which wish to perform a behaviour once for each
  callsite, such as allocating storage for data related to that callsite,
  can perform it in `register_callsite`.

  See also the [documentation on the callsite registry][cs-reg] for details
  on [`register_callsite`].

- [`event_enabled`] is called once before every call to the [`event`]
  method. This can be used to implement filtering on events once their field
  values are known, but before any processing is done in the `event` method.
- [`clone_span`] is called every time a span ID is cloned, and [`try_close`]
  is called when a span ID is dropped. By default, these functions do
  nothing. However, they can be used to implement reference counting for
  spans, allowing subscribers to free storage for span data and to determine
  when a span has _closed_ permanently (rather than being exited).
  Subscribers which store per-span data or which need to track span closures
  should override these functions together.

[ID]: super::span::Id
[`new_span`]: Subscriber::new_span
[`register_callsite`]: Subscriber::register_callsite
[`enabled`]: Subscriber::enabled
[`clone_span`]: Subscriber::clone_span
[`try_close`]: Subscriber::try_close
[cs-reg]: crate::callsite#registering-callsites
[`event`]: Subscriber::event
[`event_enabled`]: Subscriber::event_enabled

```rust
pub trait Subscriber: ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `enabled`: Returns true if a span or event with the specified [metadata] would be
- `new_span`: Visit the construction of a new span, returning a new [span ID] for the
- `record`: Record a set of values on a span.
- `record_follows_from`: Adds an indication that `span` follows from the span with the id
- `event`: Records that an [`Event`] has occurred.
- `enter`: Records that a span has been entered.
- `exit`: Records that a span has been exited.

##### Provided Methods

- ```rust
  fn on_register_dispatch(self: &Self, subscriber: &Dispatch) { /* ... */ }
  ```
  Invoked when this subscriber becomes a [`Dispatch`].

- ```rust
  fn register_callsite(self: &Self, metadata: &''static Metadata<''static>) -> Interest { /* ... */ }
  ```
  Registers a new [callsite] with this subscriber, returning whether or not

- ```rust
  fn max_level_hint(self: &Self) -> Option<LevelFilter> { /* ... */ }
  ```
  Returns the highest [verbosity level][level] that this `Subscriber` will

- ```rust
  fn event_enabled(self: &Self, event: &Event<''_>) -> bool { /* ... */ }
  ```
  Determine if an [`Event`] should be recorded.

- ```rust
  fn clone_span(self: &Self, id: &span::Id) -> span::Id { /* ... */ }
  ```
  Notifies the subscriber that a [span ID] has been cloned.

- ```rust
  fn drop_span(self: &Self, _id: span::Id) { /* ... */ }
  ```
  **This method is deprecated.**

- ```rust
  fn try_close(self: &Self, id: span::Id) -> bool { /* ... */ }
  ```
  Notifies the subscriber that a [span ID] has been dropped, and returns

- ```rust
  fn current_span(self: &Self) -> span::Current { /* ... */ }
  ```
  Returns a type representing this subscriber's view of the current span.

- ```rust
  unsafe fn downcast_raw(self: &Self, id: TypeId) -> Option<*const ()> { /* ... */ }
  ```
  If `self` is the same type as the provided `TypeId`, returns an untyped

##### Implementations

This trait is implemented for the following types:

- `NoSubscriber`
- `crate::stdlib::boxed::Box<S>` with <S>
- `crate::stdlib::sync::Arc<S>` with <S>

## Macros

### Macro `identify_callsite`

**Attributes:**

- `#[macro_export]`

Statically constructs an [`Identifier`] for the provided [`Callsite`].

This may be used in contexts such as static initializers.

For example:
```rust
use tracing_core::{callsite, identify_callsite};
# use tracing_core::{Metadata, subscriber::Interest};
# fn main() {
pub struct MyCallsite {
   // ...
}
impl callsite::Callsite for MyCallsite {
# fn set_interest(&self, _: Interest) { unimplemented!() }
# fn metadata(&self) -> &Metadata { unimplemented!() }
    // ...
}

static CALLSITE: MyCallsite = MyCallsite {
    // ...
};

static CALLSITE_ID: callsite::Identifier = identify_callsite!(&CALLSITE);
# }
```

[`Identifier`]: callsite::Identifier
[`Callsite`]: callsite::Callsite

```rust
pub macro_rules! identify_callsite {
    /* macro_rules! identify_callsite {
    ($callsite:expr) => { ... };
} */
}
```

### Macro `metadata`

**Attributes:**

- `#[macro_export]`

Statically constructs new span [metadata].

/// For example:
```rust
# use tracing_core::{callsite::Callsite, subscriber::Interest};
use tracing_core::metadata;
use tracing_core::metadata::{Kind, Level, Metadata};
# fn main() {
# pub struct MyCallsite { }
# impl Callsite for MyCallsite {
# fn set_interest(&self, _: Interest) { unimplemented!() }
# fn metadata(&self) -> &Metadata { unimplemented!() }
# }
#
static FOO_CALLSITE: MyCallsite = MyCallsite {
    // ...
};

static FOO_METADATA: Metadata = metadata!{
    name: "foo",
    target: module_path!(),
    level: Level::DEBUG,
    fields: &["bar", "baz"],
    callsite: &FOO_CALLSITE,
    kind: Kind::SPAN,
};
# }
```

[metadata]: metadata::Metadata
[`Metadata::new`]: metadata::Metadata::new

```rust
pub macro_rules! metadata {
    /* macro_rules! metadata {
    (
        name: $name:expr,
        target: $target:expr,
        level: $level:expr,
        fields: $fields:expr,
        callsite: $callsite:expr,
        kind: $kind:expr
    ) => { ... };
    (
        name: $name:expr,
        target: $target:expr,
        level: $level:expr,
        fields: $fields:expr,
        callsite: $callsite:expr,
        kind: $kind:expr,
    ) => { ... };
} */
}
```

## Re-exports

### Re-export `Once`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use stdlib::sync::Once;
```

### Re-export `Callsite`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::callsite::Callsite;
```

### Re-export `Dispatch`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::dispatcher::Dispatch;
```

### Re-export `Event`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::event::Event;
```

### Re-export `Field`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::field::Field;
```

### Re-export `Level`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::metadata::Level;
```

### Re-export `LevelFilter`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::metadata::LevelFilter;
```

### Re-export `Metadata`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::metadata::Metadata;
```

### Re-export `Subscriber`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::subscriber::Subscriber;
```

### Re-export `Kind`

```rust
pub use self::metadata::Kind;
```

### Re-export `Interest`

```rust
pub use self::subscriber::Interest;
```

