# Crate Documentation

**Version:** 0.27.1

**Format Version:** 43

# Module `opentelemetry`

Implements the [`API`] component of [OpenTelemetry].

*[Supported Rust Versions](#supported-rust-versions)*

[`API`]: https://opentelemetry.io/docs/specs/otel/overview/#api
[OpenTelemetry]: https://opentelemetry.io/docs/what-is-opentelemetry/

# Getting Started with Traces

The [`trace`] module includes types for tracking the progression of a single
request while it is handled by services that make up an application. A trace
is a tree of [`Span`]s which are objects that represent the work being done
by individual services or components involved in a request as it flows
through a system.

```
# #[cfg(feature = "trace")]
# {
use opentelemetry::{global, trace::{Span, Tracer}, KeyValue};

// get a tracer from a provider
let tracer = global::tracer("my_service");

// start a new span
let mut span = tracer.start("my_span");

// set some attributes
span.set_attribute(KeyValue::new("http.client_ip", "83.164.160.102"));

// perform some more work...

// end or drop the span to export
span.end();
# }
```

See the [examples](https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples) directory for different integration patterns.

See the [`trace`] module docs for more information on creating and managing
spans.

[`Span`]: crate::trace::Span

# Getting Started with Metrics

The [`metrics`] module provides types for recording measurements about a
service at runtime. Below are the key steps to report measurements using
OpenTelemetry Metrics:

1. **Obtain a Meter:** Get a `Meter` from a `MeterProvider`.
2. **Create Instruments:** Use the `Meter` to create one or more instruments
   (e.g., counters, histograms).
3. **Record Measurements:** Use the instruments to record measurement values
   along with optional attributes.

## How Metrics work in OpenTelemetry
In OpenTelemetry, raw measurements recorded using instruments are
**aggregated in memory** to form metrics. These aggregated metrics are
periodically exported by the [`opentelemetry_sdk`] at fixed intervals (e.g.,
every 60 seconds) via exporters such as [`opentelemetry-stdout`] or
[`opentelemetry-otlp`]. This reduces reporting overhead while ensuring
up-to-date data. The aggregation strategy and export interval can be
customized in the [`opentelemetry_sdk`] based on your use case.

## Choosing the Right Instrument
Selecting the correct instrument is critical for accurately representing
your metrics data:

- Use **Counters** for values that only increase, such as the number of
  requests served or errors encountered.
- Use **UpDownCounters** for values that can increase or decrease, such as
  the number of active connections, number of items in a queue etc.
- **Gauges:** Use for values that can go up or down and represent the
  current state, such as CPU usage, temperature etc.
- Use **Histograms** for measuring the distribution of a value, such as
  response times or payload sizes.

### Observable Instruments

Counters, UpDownCounters, and Gauges have Observable variants that allow
values to be reported through a callback function. Observable instruments
are ideal when the metric value is managed elsewhere and needs to be
observed by OpenTelemetry instrumentation. The callbacks are automatically
invoked by the OpenTelemetry SDK before every export (e.g., every 60
seconds).

For example:
- An **ObservableCounter** can monitor the number of page faults in a
  process as reported by the operating system.
- An **ObservableUpDownCounter** can monitor the size of an in-memory queue
  by reporting the size using queue's len() method within the callback
  function.
- An **ObservableGauge** can monitor the CPU temperature by using
  temperature sensor APIs within the callback function.
   
For detailed guidance, refer to [OpenTelemetry Metrics API - Instrumentation
Guidance](https://opentelemetry.io/docs/specs/otel/metrics/supplementary-guidelines/#instrument-selection).

## Best Practices
- **Re-use Instruments:** Instruments are designed for
  reuse. Avoid creating new instruments repeatedly.
- **Clone for Sharing:** If the same instrument needs to be used across
  multiple parts of your code, you can safely clone it to share.

## Example Usage
```
use opentelemetry::{global, KeyValue};

// Get a meter from a provider.
let meter = global::meter("my_service");

// Create an instrument (in this case, a Counter).
let counter = meter.u64_counter("request.count").build();

// Record a measurement by passing the value and a set of attributes.
counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);

// Create an ObservableCounter and register a callback that reports the measurement.
let _observable_counter = meter
.u64_observable_counter("bytes_received")
.with_callback(|observer| {
    observer.observe(
        100,
        &[
            KeyValue::new("protocol", "udp"),
        ],
    )
})
.build();
```

See the
[examples](https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples/metrics-basic)
directory that show a runnable example with all type of instruments.


See the [`metrics`] module docs for more information on creating and
managing instruments.


# Getting Started with Logs

 The [`logs`] module contains the Logs Bridge API. It is not intended to be
 called by application developers directly. It is provided for logging
 library authors to build log appenders, that bridges existing logging
 systems with OpenTelemetry. Bridges for
 [`log`](https://crates.io/crates/log) and
 [`tracing`](https://crates.io/crates/tracing) libraries are provided via
 the
 [`opentelemetry-appender-log`](https://crates.io/crates/opentelemetry-appender-log)
 and
 [`opentelemetry-appender-tracing`](https://crates.io/crates/opentelemetry-appender-tracing)
 crates.

# Crate Feature Flags

The following core crate feature flags are available:

* `trace`: Includes the trace API.
* `metrics`: Includes the metrics API.
* `logs`: Includes the logs bridge API.
* `internal-logs`: Includes internal logging for the OpenTelemetry library via `tracing`.

The default feature flags are ["trace", "metrics", "logs", "internal-logs"].

The following feature flags provides additional configuration for `logs`:
* `spec_unstable_logs_enabled`: Allow users to control the log level

The following feature flags enable APIs defined in OpenTelemetry specification that is in experimental phase:
* `otel_unstable`: Includes unstable APIs. There are no features behind this flag at the moment.

# Related Crates

In addition to `opentelemetry`, the [`open-telemetry/opentelemetry-rust`]
repository contains several additional crates designed to be used with the
`opentelemetry` ecosystem. This includes exporters, samplers, as well as
utility and adapter crates to assist in propagating context and
instrumenting applications.

In particular, the following crates are likely to be of interest:

- [`opentelemetry_sdk`] provides the OpenTelemetry SDK used to configure providers.
- [`opentelemetry-http`] provides an interface for injecting and extracting
  trace information from [`http`] headers.
- [`opentelemetry-otlp`] exporter for sending telemetry in the
  OTLP format.
- [`opentelemetry-stdout`] provides ability to output telemetry to stdout,
  primarily used for learning/debugging purposes.
- [`opentelemetry-prometheus`] provides a pipeline and exporter for sending
  metrics information to [`Prometheus`].
- [`opentelemetry-zipkin`] provides a pipeline and exporter for sending
  trace information to [`Zipkin`].

 In addition, there are several other useful crates in the [OTel Rust
 Contrib
 repo](https://github.com/open-telemetry/opentelemetry-rust-contrib). A lot
 of crates maintained outside OpenTelemetry owned repos can be found in the
 [OpenTelemetry
 Registry](https://opentelemetry.io/ecosystem/registry/?language=rust).

[`http`]: https://crates.io/crates/http
[`open-telemetry/opentelemetry-rust`]: https://github.com/open-telemetry/opentelemetry-rust
[`opentelemetry_sdk`]: https://crates.io/crates/opentelemetry_sdk
[`opentelemetry-stdout`]: https://crates.io/crates/opentelemetry_stdout
[`opentelemetry-http`]: https://crates.io/crates/opentelemetry-http
[`opentelemetry-otlp`]: https://crates.io/crates/opentelemetry-otlp
[`opentelemetry-prometheus`]: https://crates.io/crates/opentelemetry-prometheus
[`opentelemetry-zipkin`]: https://crates.io/crates/opentelemetry-zipkin
[`Prometheus`]: https://prometheus.io
[`Zipkin`]: https://zipkin.io

# Supported Rust Versions

OpenTelemetry is built against the latest stable release. The minimum
supported version is 1.70. The current OpenTelemetry version is not
guaranteed to build on Rust versions earlier than the minimum supported
version.

The current stable Rust compiler and the three most recent minor versions
before it will always be supported. For example, if the current stable
compiler version is 1.49, the minimum supported version will not be
increased past 1.46, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.

## Modules

## Module `global`

Utilities for working with global telemetry primitives

## Global Trace API

The global trace API **provides applications access to their configured
[`TracerProvider`] instance from anywhere in the codebase**. This allows
applications to be less coupled to the specific Open Telemetry SDK while not
manually passing references to each part of the code that needs to create
[`Span`]s. Additionally, **3rd party middleware** or **library code** can be
written against this generic API and not constrain users to a specific
implementation choice.

### Usage in Applications

Applications configure their tracer either by installing a trace pipeline,
or calling [`set_tracer_provider`].

```
# #[cfg(feature="trace")]
# {
use opentelemetry::trace::{Tracer, noop::NoopTracerProvider};
use opentelemetry::global;

fn init_tracer() {
    // Swap this no-op provider for your tracing service of choice (jaeger, zipkin, etc)
    let provider = NoopTracerProvider::new();

    // Configure the global `TracerProvider` singleton when your app starts
    // (there is a no-op default if this is not set by your application)
    let _ = global::set_tracer_provider(provider);
}

fn do_something_tracked() {
    // Then you can get a named tracer instance anywhere in your codebase.
    let tracer = global::tracer("my-component");

    tracer.in_span("doing_work", |cx| {
        // Traced app logic here...
    });
}

// in main or other app start
init_tracer();
do_something_tracked();
# }
```

### Usage in Libraries

```
# #[cfg(feature="trace")]
# {
use std::sync::Arc;
use opentelemetry::trace::Tracer;
use opentelemetry::global;
use opentelemetry::InstrumentationScope;

pub fn my_traced_library_function() {
    // End users of your library will configure their global tracer provider
    // so you can use the global tracer without any setup

    let scope = InstrumentationScope::builder("my_library-name")
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_schema_url("https://opentelemetry.io/schemas/1.17.0")
        .build();

    let tracer = global::tracer_with_scope(scope);

    tracer.in_span("doing_library_work", |cx| {
        // Traced library logic here...
    });
}
# }
```

[`TracerProvider`]: crate::trace::TracerProvider
[`Span`]: crate::trace::Span

## Global Metrics API

The global metrics API **provides applications access to their configured
[`MeterProvider`] instance from anywhere in the codebase**. This allows
applications to be less coupled to the specific Open Telemetry SDK while not
manually passing references to each part of the code that needs to create
metric instruments. Additionally, **3rd party middleware** or **library code** can be
written against this generic API and not constrain users to a specific
implementation choice.

### Usage in Applications and libraries

Applications and libraries can obtain meter from the global meter provider,
and use the meter to create instruments to emit measurements.

```
# #[cfg(feature="metrics")]
# {
use opentelemetry::metrics::{Meter};
use opentelemetry::{global, KeyValue};

   fn do_something_instrumented() {
    let meter = global::meter("my-component");
    // It is recommended to reuse the same counter instance for the
    // lifetime of the application
    let counter = meter.u64_counter("my_counter").build();

    // record measurements
    counter.add(1, &[KeyValue::new("mykey", "myvalue")]);
    }
}
```

### Usage in Applications
Application owners have the responsibility to set the global meter provider.
The global meter provider can be set using the [`set_meter_provider`] function.
As set_meter_provider takes ownership of the provider, it is recommended to
provide a clone of the provider, if the application needs to use the provider
later to perform operations like shutdown.
```
# #[cfg(feature="metrics")]
# {
use opentelemetry::{global, KeyValue};

fn main() {
   // Set the global meter provider
   // global::set_meter_provider(my_meter_provider().clone());
}
# }
```

[`MeterProvider`]: crate::metrics::MeterProvider
[`set_meter_provider`]: crate::global::set_meter_provider

```rust
pub mod global { /* ... */ }
```

### Re-exports

#### Re-export `metrics::*`

**Attributes:**

- `#[<cfg>(feature = "metrics")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "metrics")))]`

```rust
pub use metrics::*;
```

#### Re-export `propagation::*`

**Attributes:**

- `#[<cfg>(feature = "trace")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "trace")))]`

```rust
pub use propagation::*;
```

#### Re-export `trace::*`

**Attributes:**

- `#[<cfg>(feature = "trace")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "trace")))]`

```rust
pub use trace::*;
```

## Module `baggage`

Primitives for sending name/value data across system boundaries.

Baggage is used to annotate telemetry, adding context and information to
metrics, traces, and logs. It is a set of name/value pairs describing
user-defined properties. Each name in Baggage is associated with exactly one
value.

Main types in this module are:

* [`Baggage`]: A set of name/value pairs describing user-defined properties.
* [`BaggageExt`]: Extensions for managing `Baggage` in a [`Context`].

Baggage can be sent between systems using a baggage propagator in
accordance with the [W3C Baggage] specification.

[W3C Baggage]: https://w3c.github.io/baggage

```rust
pub mod baggage { /* ... */ }
```

### Types

#### Struct `Baggage`

A set of name/value pairs describing user-defined properties.

### Baggage Names

* ASCII strings according to the token format, defined in [RFC2616, Section 2.2]

### Baggage Values

* URL encoded UTF-8 strings.

### Baggage Value Metadata

Additional metadata can be added to values in the form of a property set,
represented as semi-colon `;` delimited list of names and/or name/value pairs,
e.g. `;k1=v1;k2;k3=v3`.

### Limits

* Maximum number of name/value pairs: `180`.
* Maximum number of bytes per a single name/value pair: `4096`.
* Maximum total length of all name/value pairs: `8192`.

[RFC2616, Section 2.2]: https://tools.ietf.org/html/rfc2616#section-2.2

```rust
pub struct Baggage {
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
  Creates an empty `Baggage`.

- ```rust
  pub fn get<K: AsRef<str>>(self: &Self, key: K) -> Option<&Value> { /* ... */ }
  ```
  Returns a reference to the value associated with a given name

- ```rust
  pub fn get_with_metadata<K: AsRef<str>>(self: &Self, key: K) -> Option<&(Value, BaggageMetadata)> { /* ... */ }
  ```
  Returns a reference to the value and metadata associated with a given name

- ```rust
  pub fn insert<K, V>(self: &mut Self, key: K, value: V) -> Option<Value>
where
    K: Into<Key>,
    V: Into<Value> { /* ... */ }
  ```
  Inserts a name/value pair into the baggage.

- ```rust
  pub fn insert_with_metadata<K, V, S>(self: &mut Self, key: K, value: V, metadata: S) -> Option<(Value, BaggageMetadata)>
where
    K: Into<Key>,
    V: Into<Value>,
    S: Into<BaggageMetadata> { /* ... */ }
  ```
  Inserts a name/value pair into the baggage.

- ```rust
  pub fn remove<K: Into<Key>>(self: &mut Self, key: K) -> Option<(Value, BaggageMetadata)> { /* ... */ }
  ```
  Removes a name from the baggage, returning the value

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of attributes for this baggage

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the baggage contains no items.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_> { /* ... */ }
  ```
  Gets an iterator over the baggage items, sorted by name.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **WithSubscriber**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = (Key, (Value, BaggageMetadata))>>(iter: I) -> Self { /* ... */ }
    ```

  - ```rust
    fn from_iter<I: IntoIterator<Item = KeyValue>>(iter: I) -> Self { /* ... */ }
    ```

  - ```rust
    fn from_iter<I: IntoIterator<Item = KeyValueMetadata>>(iter: I) -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FutureExt**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Baggage { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Send**
- **Instrument**
#### Struct `Iter`

An iterator over the entries of a [`Baggage`].

```rust
pub struct Iter<''a>(/* private field */);
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **WithSubscriber**
- **Unpin**
- **FutureExt**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `BaggageMetadata`

An optional property set that can be added to [`Baggage`] values.

`BaggageMetadata` can be added to values in the form of a property set,
represented as semi-colon `;` delimited list of names and/or name/value
pairs, e.g. `;k1=v1;k2;k3=v3`.

```rust
pub struct BaggageMetadata(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Return underlying string

###### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Instrument**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &BaggageMetadata) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Eq**
- **Default**
  - ```rust
    fn default() -> BaggageMetadata { /* ... */ }
    ```

- **StructuralPartialEq**
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
    fn clone(self: &Self) -> BaggageMetadata { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BaggageMetadata) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FutureExt**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(s: String) -> BaggageMetadata { /* ... */ }
    ```

  - ```rust
    fn from(s: &str) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **WithSubscriber**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
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

#### Struct `KeyValueMetadata`

[`Baggage`] name/value pairs with their associated metadata.

```rust
pub struct KeyValueMetadata {
    pub key: crate::Key,
    pub value: crate::Value,
    pub metadata: BaggageMetadata,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `crate::Key` | Dimension or event key |
| `value` | `crate::Value` | Dimension or event value |
| `metadata` | `BaggageMetadata` | Metadata associate with this key value pair |

##### Implementations

###### Methods

- ```rust
  pub fn new<K, V, S>(key: K, value: V, metadata: S) -> Self
where
    K: Into<Key>,
    V: Into<Value>,
    S: Into<BaggageMetadata> { /* ... */ }
  ```
  Create a new `KeyValue` pair with metadata

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &KeyValueMetadata) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Instrument**
- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FutureExt**
- **Freeze**
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

  - ```rust
    fn from(kv: KeyValue) -> Self { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = KeyValueMetadata>>(iter: I) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> KeyValueMetadata { /* ... */ }
    ```

### Traits

#### Trait `BaggageExt`

Methods for sorting and retrieving baggage data in a context.

```rust
pub trait BaggageExt {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `with_baggage`: Returns a clone of the given context with the included name/value pairs.
- `current_with_baggage`: Returns a clone of the current context with the included name/value pairs.
- `with_cleared_baggage`: Returns a clone of the given context with no baggage.
- `baggage`: Returns a reference to this context's baggage, or the default

##### Implementations

This trait is implemented for the following types:

- `crate::Context`

## Module `metrics`

**Attributes:**

- `#[<cfg>(feature = "metrics")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "metrics")))]`

# OpenTelemetry Metrics API

```rust
pub mod metrics { /* ... */ }
```

### Traits

#### Trait `InstrumentProvider`

SDK implemented trait for creating instruments

```rust
pub trait InstrumentProvider {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn u64_counter(self: &Self, _builder: InstrumentBuilder<''_, Counter<u64>>) -> Counter<u64> { /* ... */ }
  ```
  creates an instrument for recording increasing values.

- ```rust
  fn f64_counter(self: &Self, _builder: InstrumentBuilder<''_, Counter<f64>>) -> Counter<f64> { /* ... */ }
  ```
  creates an instrument for recording increasing values.

- ```rust
  fn u64_observable_counter(self: &Self, _builder: AsyncInstrumentBuilder<''_, ObservableCounter<u64>, u64>) -> ObservableCounter<u64> { /* ... */ }
  ```
  creates an instrument for recording increasing values via callback.

- ```rust
  fn f64_observable_counter(self: &Self, _builder: AsyncInstrumentBuilder<''_, ObservableCounter<f64>, f64>) -> ObservableCounter<f64> { /* ... */ }
  ```
  creates an instrument for recording increasing values via callback.

- ```rust
  fn i64_up_down_counter(self: &Self, _builder: InstrumentBuilder<''_, UpDownCounter<i64>>) -> UpDownCounter<i64> { /* ... */ }
  ```
  creates an instrument for recording changes of a value.

- ```rust
  fn f64_up_down_counter(self: &Self, _builder: InstrumentBuilder<''_, UpDownCounter<f64>>) -> UpDownCounter<f64> { /* ... */ }
  ```
  creates an instrument for recording changes of a value.

- ```rust
  fn i64_observable_up_down_counter(self: &Self, _builder: AsyncInstrumentBuilder<''_, ObservableUpDownCounter<i64>, i64>) -> ObservableUpDownCounter<i64> { /* ... */ }
  ```
  creates an instrument for recording changes of a value.

- ```rust
  fn f64_observable_up_down_counter(self: &Self, _builder: AsyncInstrumentBuilder<''_, ObservableUpDownCounter<f64>, f64>) -> ObservableUpDownCounter<f64> { /* ... */ }
  ```
  creates an instrument for recording changes of a value via callback.

- ```rust
  fn u64_gauge(self: &Self, _builder: InstrumentBuilder<''_, Gauge<u64>>) -> Gauge<u64> { /* ... */ }
  ```
  creates an instrument for recording independent values.

- ```rust
  fn f64_gauge(self: &Self, _builder: InstrumentBuilder<''_, Gauge<f64>>) -> Gauge<f64> { /* ... */ }
  ```
  creates an instrument for recording independent values.

- ```rust
  fn i64_gauge(self: &Self, _builder: InstrumentBuilder<''_, Gauge<i64>>) -> Gauge<i64> { /* ... */ }
  ```
  creates an instrument for recording independent values.

- ```rust
  fn u64_observable_gauge(self: &Self, _builder: AsyncInstrumentBuilder<''_, ObservableGauge<u64>, u64>) -> ObservableGauge<u64> { /* ... */ }
  ```
  creates an instrument for recording the current value via callback.

- ```rust
  fn i64_observable_gauge(self: &Self, _builder: AsyncInstrumentBuilder<''_, ObservableGauge<i64>, i64>) -> ObservableGauge<i64> { /* ... */ }
  ```
  creates an instrument for recording the current value via callback.

- ```rust
  fn f64_observable_gauge(self: &Self, _builder: AsyncInstrumentBuilder<''_, ObservableGauge<f64>, f64>) -> ObservableGauge<f64> { /* ... */ }
  ```
  creates an instrument for recording the current value via callback.

- ```rust
  fn f64_histogram(self: &Self, _builder: HistogramBuilder<''_, Histogram<f64>>) -> Histogram<f64> { /* ... */ }
  ```
  creates an instrument for recording a distribution of values.

- ```rust
  fn u64_histogram(self: &Self, _builder: HistogramBuilder<''_, Histogram<u64>>) -> Histogram<u64> { /* ... */ }
  ```
  creates an instrument for recording a distribution of values.

### Re-exports

#### Re-export `Counter`

```rust
pub use instruments::counter::Counter;
```

#### Re-export `ObservableCounter`

```rust
pub use instruments::counter::ObservableCounter;
```

#### Re-export `Gauge`

```rust
pub use instruments::gauge::Gauge;
```

#### Re-export `ObservableGauge`

```rust
pub use instruments::gauge::ObservableGauge;
```

#### Re-export `Histogram`

```rust
pub use instruments::histogram::Histogram;
```

#### Re-export `ObservableUpDownCounter`

```rust
pub use instruments::up_down_counter::ObservableUpDownCounter;
```

#### Re-export `UpDownCounter`

```rust
pub use instruments::up_down_counter::UpDownCounter;
```

#### Re-export `AsyncInstrument`

```rust
pub use instruments::AsyncInstrument;
```

#### Re-export `AsyncInstrumentBuilder`

```rust
pub use instruments::AsyncInstrumentBuilder;
```

#### Re-export `Callback`

```rust
pub use instruments::Callback;
```

#### Re-export `HistogramBuilder`

```rust
pub use instruments::HistogramBuilder;
```

#### Re-export `InstrumentBuilder`

```rust
pub use instruments::InstrumentBuilder;
```

#### Re-export `SyncInstrument`

```rust
pub use instruments::SyncInstrument;
```

#### Re-export `Meter`

```rust
pub use meter::Meter;
```

#### Re-export `MeterProvider`

```rust
pub use meter::MeterProvider;
```

## Module `propagation`

# OpenTelemetry Propagator interface
Cross-cutting concerns send their state to the next process using Propagators, which are defined
as objects used to read and write context data to and from messages exchanged by the applications.

`Propagator`s leverage the [`Context`] to inject and extract data for each cross-cutting concern,
such as `TraceContext` and [`Baggage`].

The Propagators API is expected to be leveraged by users writing instrumentation libraries.

Currently, the following `Propagator` types are supported:
-  [`TextMapPropagator`], inject values into and extracts values from carriers as string key/value pairs

A binary Propagator type will be added in
the future, See [tracking issues](https://github.com/open-telemetry/opentelemetry-specification/issues/437)).

`Propagator`s uses [`Injector`] and [`Extractor`] to read and write context data to and from messages.
Each specific Propagator type defines its expected carrier type, such as a string map or a byte array.

[`Baggage`]: crate::baggage::Baggage
[`Context`]: crate::Context

```rust
pub mod propagation { /* ... */ }
```

### Modules

## Module `composite`

# Composite Propagator

A utility over multiple `Propagator`s to group multiple Propagators from different cross-cutting
concerns in order to leverage them as a single entity.

Each composite Propagator will implement a specific Propagator type, such as TextMapPropagator,
as different Propagator types will likely operate on different data types.

```rust
pub mod composite { /* ... */ }
```

### Types

#### Struct `TextMapCompositePropagator`

Composite propagator for [`TextMapPropagator`]s.

A propagator that chains multiple [`TextMapPropagator`] propagators together,
injecting or extracting by their respective HTTP header names.

Injection and extraction from this propagator will preserve the order of the
injectors and extractors passed in during initialization.

# Examples

```
use opentelemetry::{
    baggage::BaggageExt,
    propagation::{TextMapPropagator, TextMapCompositePropagator},

    trace::{TraceContextExt, Tracer, TracerProvider},
    Context, KeyValue,
};
use opentelemetry_sdk::propagation::{
    BaggagePropagator, TraceContextPropagator,
};
use opentelemetry_sdk::trace as sdktrace;
use std::collections::HashMap;

// First create 1 or more propagators
let baggage_propagator = BaggagePropagator::new();
let trace_context_propagator = TraceContextPropagator::new();

// Then create a composite propagator
let composite_propagator = TextMapCompositePropagator::new(vec![
    Box::new(baggage_propagator),
    Box::new(trace_context_propagator),
]);

// Then for a given implementation of `Injector`
let mut injector = HashMap::new();

// And a given span
let example_span = sdktrace::TracerProvider::default()
    .tracer("example-component")
    .start("span-name");

// with the current context, call inject to add the headers
composite_propagator.inject_context(
    &Context::current_with_span(example_span)
        .with_baggage(vec![KeyValue::new("test", "example")]),
    &mut injector,
);

// The injector now has both `baggage` and `traceparent` headers
assert!(injector.get("baggage").is_some());
assert!(injector.get("traceparent").is_some());
```

```rust
pub struct TextMapCompositePropagator {
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
  pub fn new(propagators: Vec<Box<dyn TextMapPropagator + Send + Sync>>) -> Self { /* ... */ }
  ```
  Constructs a new propagator out of instances of [`TextMapPropagator`].

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **WithSubscriber**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Instrument**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TextMapPropagator**
  - ```rust
    fn inject_context(self: &Self, context: &Context, injector: &mut dyn Injector) { /* ... */ }
    ```
    Encodes the values of the `Context` and injects them into the `Injector`.

  - ```rust
    fn extract_with_context(self: &Self, cx: &Context, extractor: &dyn Extractor) -> Context { /* ... */ }
    ```
    Retrieves encoded `Context` information using the `Extractor`. If no data was

  - ```rust
    fn fields(self: &Self) -> FieldIter<''_> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FutureExt**
## Module `text_map_propagator`

# TextMapPropagator

[`TextMapPropagator`] performs the injection and extraction of a cross-cutting concern value as
string key/values pairs into carriers that travel in-band across process boundaries.

The carrier of propagated data on both the client (injector) and server (extractor) side is
usually an HTTP request.

In order to increase compatibility, the key/value pairs MUST only consist of US-ASCII characters
that make up valid HTTP header fields as per RFC 7230.

```rust
pub mod text_map_propagator { /* ... */ }
```

### Types

#### Struct `FieldIter`

An iterator over fields of a [`TextMapPropagator`]


```rust
pub struct FieldIter<''a>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(fields: &''a [String]) -> Self { /* ... */ }
  ```
  Create a new `FieldIter` from a slice of propagator fields

###### Trait Implementations

- **Instrument**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **FutureExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **WithSubscriber**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
### Traits

#### Trait `TextMapPropagator`

Methods to inject and extract a value as text into injectors and extractors that travel
in-band across process boundaries.

```rust
pub trait TextMapPropagator: Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `inject_context`: Properly encodes the values of the [`Context`] and injects them into the
- `extract_with_context`: Retrieves encoded data using the provided [`Extractor`]. If no data for this
- `fields`: Returns iter of fields used by [`TextMapPropagator`]

##### Provided Methods

- ```rust
  fn inject(self: &Self, injector: &mut dyn Injector) { /* ... */ }
  ```
  Properly encodes the values of the current [`Context`] and injects them into

- ```rust
  fn extract(self: &Self, extractor: &dyn Extractor) -> Context { /* ... */ }
  ```
  Retrieves encoded data using the provided [`Extractor`]. If no data for this

##### Implementations

This trait is implemented for the following types:

- `TextMapCompositePropagator`
- `NoopTextMapPropagator`

### Types

#### Struct `PropagationError`

**Attributes:**

- `#[error("Cannot {} from {}, {}", ops, message, propagator_name)]`

Error when extracting or injecting context data(i.e propagating) across application boundaries.

```rust
pub struct PropagationError {
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
  pub fn extract(message: &''static str, propagator_name: &''static str) -> Self { /* ... */ }
  ```
  Error happens when extracting information

- ```rust
  pub fn inject(message: &''static str, propagator_name: &''static str) -> Self { /* ... */ }
  ```
  Error happens when extracting information

###### Trait Implementations

- **FutureExt**
- **Freeze**
- **Unpin**
- **WithSubscriber**
- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
- **Instrument**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `Injector`

Injector provides an interface for adding fields from an underlying struct like `HashMap`

```rust
pub trait Injector {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `set`: Add a key and value to the underlying data.

##### Implementations

This trait is implemented for the following types:

- `std::collections::HashMap<String, String, S>` with <S: std::hash::BuildHasher>

#### Trait `Extractor`

Extractor provides an interface for removing fields from an underlying struct like `HashMap`

```rust
pub trait Extractor {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get`: Get a value from a key from the underlying data.
- `keys`: Collect all the keys from the underlying data.

##### Implementations

This trait is implemented for the following types:

- `std::collections::HashMap<String, String, S>` with <S: std::hash::BuildHasher>

### Re-exports

#### Re-export `TextMapCompositePropagator`

```rust
pub use composite::TextMapCompositePropagator;
```

#### Re-export `TextMapPropagator`

```rust
pub use text_map_propagator::TextMapPropagator;
```

## Module `trace`

**Attributes:**

- `#[<cfg>(feature = "trace")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "trace")))]`

API for tracing applications and libraries.

The `trace` module includes types for tracking the progression of a single
request while it is handled by services that make up an application. A trace
is a tree of [`Span`]s which are objects that represent the work being done
by individual services or components involved in a request as it flows
through a system. This module implements the OpenTelemetry [trace
specification].

[trace specification]: https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/api.md

## Getting Started

In application code:

```
use opentelemetry::trace::{Tracer, noop::NoopTracerProvider};
use opentelemetry::global;

fn init_tracer() {
    // Swap this no-op provider for your tracing service of choice (jaeger, zipkin, etc)
    let provider = NoopTracerProvider::new();

    // Configure the global `TracerProvider` singleton when your app starts
    // (there is a no-op default if this is not set by your application)
    let _ = global::set_tracer_provider(provider);
}

fn do_something_tracked() {
    // Then you can get a named tracer instance anywhere in your codebase.
    let tracer = global::tracer("my-component");

    tracer.in_span("doing_work", |cx| {
        // Traced app logic here...
    });
}

// in main or other app start
init_tracer();
do_something_tracked();
```

In library code:

```
use opentelemetry::{global, trace::{Span, Tracer, TracerProvider}};
use opentelemetry::InstrumentationScope;
use std::sync::Arc;

fn my_library_function() {
    // Use the global tracer provider to get access to the user-specified
    // tracer configuration
    let tracer_provider = global::tracer_provider();

    // Get a tracer for this library
    let scope = InstrumentationScope::builder("my_name")
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_schema_url("https://opentelemetry.io/schemas/1.17.0")
        .build();

    let tracer = tracer_provider.tracer_with_scope(scope);

    // Create spans
    let mut span = tracer.start("doing_work");

    // Do work...

    // End the span
    span.end();
}
```

## Overview

The tracing API consists of a three main traits:

* [`TracerProvider`]s are the entry point of the API. They provide access to
  `Tracer`s.
* [`Tracer`]s are types responsible for creating `Span`s.
* [`Span`]s provide the API to trace an operation.

## Working with Async Runtimes

Exporting spans often involves sending data over a network or performing
other I/O tasks. OpenTelemetry allows you to schedule these tasks using
whichever runtime you are already using such as [Tokio] or [async-std].
When using an async runtime it's best to use the batch span processor
where the spans will be sent in batches as opposed to being sent once ended,
which often ends up being more efficient.

[Tokio]: https://tokio.rs
[async-std]: https://async.rs

## Managing Active Spans

Spans can be marked as "active" for a given [`Context`], and all newly
created spans will automatically be children of the currently active span.

The active span for a given thread can be managed via [`get_active_span`]
and [`mark_span_as_active`].

[`Context`]: crate::Context

```
use opentelemetry::{global, trace::{self, Span, Status, Tracer, TracerProvider}};

fn may_error(rand: f32) {
    if rand < 0.5 {
        // Get the currently active span to record additional attributes,
        // status, etc.
        trace::get_active_span(|span| {
            span.set_status(Status::error("value too small"));
        });
    }
}

// Get a tracer
let tracer = global::tracer("my_tracer");

// Create a span
let span = tracer.start("parent_span");

// Mark the span as active
let active = trace::mark_span_as_active(span);

// Any span created here will be a child of `parent_span`...

// Drop the guard and the span will no longer be active
drop(active)
```

Additionally [`Tracer::in_span`] can be used as shorthand to simplify
managing the parent context.

```
use opentelemetry::{global, trace::Tracer};

// Get a tracer
let tracer = global::tracer("my_tracer");

// Use `in_span` to create a new span and mark it as the parent, dropping it
// at the end of the block.
tracer.in_span("parent_span", |cx| {
    // spans created here will be children of `parent_span`
});
```

#### Async active spans

Async spans can be propagated with [`TraceContextExt`] and [`FutureExt`].

```
use opentelemetry::{Context, global, trace::{FutureExt, TraceContextExt, Tracer}};

async fn some_work() { }
# async fn in_an_async_context() {

// Get a tracer
let tracer = global::tracer("my_tracer");

// Start a span
let span = tracer.start("my_span");

// Perform some async work with this span as the currently active parent.
some_work().with_context(Context::current_with_span(span)).await;
# }
```

```rust
pub mod trace { /* ... */ }
```

### Modules

## Module `noop`

No-op trace impls

This implementation is returned as the global tracer if no `Tracer`
has been set. It is also useful for testing purposes as it is intended
to have minimal resource utilization and runtime impact.

```rust
pub mod noop { /* ... */ }
```

### Types

#### Struct `NoopTracerProvider`

A no-op instance of a `TracerProvider`.

```rust
pub struct NoopTracerProvider {
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
  Create a new no-op tracer provider

###### Trait Implementations

- **ObjectSafeTracerProvider**
  - ```rust
    fn boxed_tracer(self: &Self, scope: InstrumentationScope) -> Box<dyn ObjectSafeTracer + Sync + Send> { /* ... */ }
    ```
    Return a versioned boxed tracer

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> NoopTracerProvider { /* ... */ }
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

- **Instrument**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
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

- **Send**
- **Sync**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NoopTracerProvider { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TracerProvider**
  - ```rust
    fn tracer_with_scope(self: &Self, _scope: InstrumentationScope) -> <Self as >::Tracer { /* ... */ }
    ```
    Returns a new `NoopTracer` instance.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FutureExt**
- **Freeze**
#### Struct `NoopSpan`

A no-op instance of a `Span`.

```rust
pub struct NoopSpan {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

###### Trait Implementations

- **WithSubscriber**
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

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Instrument**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NoopSpan { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Span**
  - ```rust
    fn add_event<T>(self: &mut Self, _name: T, _attributes: Vec<KeyValue>)
where
    T: Into<Cow<''static, str>> { /* ... */ }
    ```
    Ignores all events

  - ```rust
    fn add_event_with_timestamp<T>(self: &mut Self, _name: T, _timestamp: SystemTime, _attributes: Vec<KeyValue>)
where
    T: Into<Cow<''static, str>> { /* ... */ }
    ```
    Ignores all events with timestamps

  - ```rust
    fn span_context(self: &Self) -> &trace::SpanContext { /* ... */ }
    ```
    Returns an invalid `SpanContext`.

  - ```rust
    fn is_recording(self: &Self) -> bool { /* ... */ }
    ```
    Returns false, signifying that this span is never recording.

  - ```rust
    fn set_attribute(self: &mut Self, _attribute: KeyValue) { /* ... */ }
    ```
    Ignores all attributes

  - ```rust
    fn set_status(self: &mut Self, _status: trace::Status) { /* ... */ }
    ```
    Ignores status

  - ```rust
    fn update_name<T>(self: &mut Self, _new_name: T)
where
    T: Into<Cow<''static, str>> { /* ... */ }
    ```
    Ignores name updates

  - ```rust
    fn add_link(self: &mut Self, _span_context: trace::SpanContext, _attributes: Vec<KeyValue>) { /* ... */ }
    ```

  - ```rust
    fn end_with_timestamp(self: &mut Self, _timestamp: SystemTime) { /* ... */ }
    ```
    Ignores `Span` endings

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ObjectSafeSpan**
  - ```rust
    fn add_event_with_timestamp(self: &mut Self, name: Cow<''static, str>, timestamp: SystemTime, attributes: Vec<KeyValue>) { /* ... */ }
    ```

  - ```rust
    fn span_context(self: &Self) -> &SpanContext { /* ... */ }
    ```

  - ```rust
    fn is_recording(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn set_attribute(self: &mut Self, attribute: KeyValue) { /* ... */ }
    ```

  - ```rust
    fn set_status(self: &mut Self, status: Status) { /* ... */ }
    ```

  - ```rust
    fn update_name(self: &mut Self, new_name: Cow<''static, str>) { /* ... */ }
    ```

  - ```rust
    fn add_link(self: &mut Self, span_context: SpanContext, attributes: Vec<KeyValue>) { /* ... */ }
    ```

  - ```rust
    fn end_with_timestamp(self: &mut Self, timestamp: SystemTime) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FutureExt**
- **Send**
#### Struct `NoopTracer`

A no-op instance of a `Tracer`.

```rust
pub struct NoopTracer {
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
  Create a new no-op tracer

###### Trait Implementations

- **WithSubscriber**
- **Freeze**
- **ObjectSafeTracer**
  - ```rust
    fn build_with_context_boxed(self: &Self, builder: SpanBuilder, parent_cx: &Context) -> Box<dyn ObjectSafeSpan + Sync + Send> { /* ... */ }
    ```
    Returns a trait object so the underlying implementation can be swapped

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NoopTracer { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Tracer**
  - ```rust
    fn build_with_context(self: &Self, _builder: trace::SpanBuilder, parent_cx: &Context) -> <Self as >::Span { /* ... */ }
    ```
    Builds a `NoopSpan` from a `SpanBuilder`.

- **UnwindSafe**
- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FutureExt**
- **Default**
  - ```rust
    fn default() -> NoopTracer { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Instrument**
#### Struct `NoopTextMapPropagator`

A no-op instance of an [`TextMapPropagator`].

[`TextMapPropagator`]: crate::propagation::TextMapPropagator

```rust
pub struct NoopTextMapPropagator {
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
  Create a new noop text map propagator

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Instrument**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FutureExt**
- **Default**
  - ```rust
    fn default() -> NoopTextMapPropagator { /* ... */ }
    ```

- **TextMapPropagator**
  - ```rust
    fn inject_context(self: &Self, _cx: &Context, _injector: &mut dyn Injector) { /* ... */ }
    ```

  - ```rust
    fn extract_with_context(self: &Self, _cx: &Context, _extractor: &dyn Extractor) -> Context { /* ... */ }
    ```

  - ```rust
    fn fields(self: &Self) -> FieldIter<''_> { /* ... */ }
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

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **WithSubscriber**
### Types

#### Type Alias `TraceResult`

Describe the result of operations in tracing API.

```rust
pub type TraceResult<T> = Result<T, TraceError>;
```

#### Enum `TraceError`

**Attributes:**

- `#[non_exhaustive]`

Errors returned by the trace API.

```rust
pub enum TraceError {
    ExportFailed(Box<dyn ExportError>),
    ExportTimedOut(time::Duration),
    TracerProviderAlreadyShutdown,
    Other(Box<dyn std::error::Error + Send + Sync + ''static>),
}
```

##### Variants

###### `ExportFailed`

Export failed with the error returned by the exporter

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<dyn ExportError>` |  |

###### `ExportTimedOut`

Export failed to finish after certain period and processor stopped the export.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `time::Duration` |  |

###### `TracerProviderAlreadyShutdown`

already shutdown error

###### `Other`

Other errors propagated from trace SDK that weren't covered above

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<dyn std::error::Error + Send + Sync + ''static>` |  |

##### Implementations

###### Trait Implementations

- **Instrument**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **UnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: Box<dyn std::error::Error + Send + Sync + ''static>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: T) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err_msg: String) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err_msg: &''static str) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: PoisonError<T>) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **FutureExt**
- **WithSubscriber**
- **Send**
- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **Freeze**
#### Struct `Event`

**Attributes:**

- `#[non_exhaustive]`

Events record things that happened during a [`Span`]'s lifetime.

```rust
pub struct Event {
    pub name: std::borrow::Cow<''static, str>,
    pub timestamp: time::SystemTime,
    pub attributes: Vec<crate::KeyValue>,
    pub dropped_attributes_count: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `std::borrow::Cow<''static, str>` | The name of this event. |
| `timestamp` | `time::SystemTime` | The time at which this event occurred. |
| `attributes` | `Vec<crate::KeyValue>` | Attributes that describe this event. |
| `dropped_attributes_count` | `u32` | The number of attributes that were above the configured limit, and thus<br>dropped. |

##### Implementations

###### Methods

- ```rust
  pub fn new<T: Into<Cow<''static, str>>>(name: T, timestamp: time::SystemTime, attributes: Vec<KeyValue>, dropped_attributes_count: u32) -> Self { /* ... */ }
  ```
  Create new `Event`

- ```rust
  pub fn with_name<T: Into<Cow<''static, str>>>(name: T) -> Self { /* ... */ }
  ```
  Create new `Event` with a given name.

###### Trait Implementations

- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **FutureExt**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **WithSubscriber**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Event { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Event) -> bool { /* ... */ }
    ```

- **Instrument**
#### Struct `Link`

**Attributes:**

- `#[non_exhaustive]`

Link is the relationship between two Spans.

The relationship can be within the same trace or across different traces.

```rust
pub struct Link {
    pub span_context: SpanContext,
    pub attributes: Vec<crate::KeyValue>,
    pub dropped_attributes_count: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span_context` | `SpanContext` | The span context of the linked span. |
| `attributes` | `Vec<crate::KeyValue>` | Attributes that describe this link. |
| `dropped_attributes_count` | `u32` | The number of attributes that were above the configured limit, and thus<br>dropped. |

##### Implementations

###### Methods

- ```rust
  pub fn new(span_context: SpanContext, attributes: Vec<KeyValue>, dropped_attributes_count: u32) -> Self { /* ... */ }
  ```
  Create new `Link`

- ```rust
  pub fn with_context(span_context: SpanContext) -> Self { /* ... */ }
  ```
  Create new `Link` with given context

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **FutureExt**
- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Link { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Link) -> bool { /* ... */ }
    ```

### Traits

#### Trait `ExportError`

Trait for errors returned by exporters

```rust
pub trait ExportError: std::error::Error + Send + Sync + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `exporter_name`: The name of exporter that returned this error

### Re-exports

#### Re-export `get_active_span`

```rust
pub use self::context::get_active_span;
```

#### Re-export `mark_span_as_active`

```rust
pub use self::context::mark_span_as_active;
```

#### Re-export `FutureExt`

```rust
pub use self::context::FutureExt;
```

#### Re-export `SpanRef`

```rust
pub use self::context::SpanRef;
```

#### Re-export `TraceContextExt`

```rust
pub use self::context::TraceContextExt;
```

#### Re-export `WithContext`

```rust
pub use self::context::WithContext;
```

#### Re-export `Span`

```rust
pub use self::span::Span;
```

#### Re-export `SpanKind`

```rust
pub use self::span::SpanKind;
```

#### Re-export `Status`

```rust
pub use self::span::Status;
```

#### Re-export `SpanContext`

```rust
pub use self::span_context::SpanContext;
```

#### Re-export `SpanId`

```rust
pub use self::span_context::SpanId;
```

#### Re-export `TraceFlags`

```rust
pub use self::span_context::TraceFlags;
```

#### Re-export `TraceId`

```rust
pub use self::span_context::TraceId;
```

#### Re-export `TraceState`

```rust
pub use self::span_context::TraceState;
```

#### Re-export `SamplingDecision`

```rust
pub use self::tracer::SamplingDecision;
```

#### Re-export `SamplingResult`

```rust
pub use self::tracer::SamplingResult;
```

#### Re-export `SpanBuilder`

```rust
pub use self::tracer::SpanBuilder;
```

#### Re-export `Tracer`

```rust
pub use self::tracer::Tracer;
```

#### Re-export `TracerProvider`

```rust
pub use self::tracer_provider::TracerProvider;
```

## Module `logs`

**Attributes:**

- `#[<cfg>(feature = "logs")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "logs")))]`

# OpenTelemetry Logs Bridge API

```rust
pub mod logs { /* ... */ }
```

### Re-exports

#### Re-export `Logger`

```rust
pub use logger::Logger;
```

#### Re-export `LoggerProvider`

```rust
pub use logger::LoggerProvider;
```

#### Re-export `NoopLoggerProvider`

```rust
pub use noop::NoopLoggerProvider;
```

#### Re-export `AnyValue`

```rust
pub use record::AnyValue;
```

#### Re-export `LogRecord`

```rust
pub use record::LogRecord;
```

#### Re-export `Severity`

```rust
pub use record::Severity;
```

## Macros

### Macro `otel_info`

**Attributes:**

- `#[macro_export]`


**Note**: These macros (`otel_info!`, `otel_warn!`, `otel_debug!`, and `otel_error!`) are intended to be used
**internally within OpenTelemetry code** or for **custom exporters and processors**. They are not designed
for general application logging and should not be used for that purpose.

Macro for logging informational messages in OpenTelemetry.

# Fields:
- `name`: The operation or action being logged.
- Additional optional key-value pairs can be passed as attributes.

# Example:
```rust
use opentelemetry::otel_info;
otel_info!(name: "sdk_start", version = "1.0.0", schema_url = "http://example.com");
```


```rust
pub macro_rules! otel_info {
    /* macro_rules! otel_info {
    (name: $name:expr $(,)?) => { ... };
    (name: $name:expr, $($key:ident = $value:expr),+ $(,)?) => { ... };
} */
}
```

### Macro `otel_warn`

**Attributes:**

- `#[macro_export]`

Macro for logging warning messages in OpenTelemetry.

# Fields:
- `name`: The operation or action being logged.
- Additional optional key-value pairs can be passed as attributes.

# Example:
```rust
use opentelemetry::otel_warn;
otel_warn!(name: "export_warning", error_code = 404, version = "1.0.0");
```

```rust
pub macro_rules! otel_warn {
    /* macro_rules! otel_warn {
    (name: $name:expr $(,)?) => { ... };
    (name: $name:expr, $($key:ident = $value:expr),+ $(,)?) => { ... };
} */
}
```

### Macro `otel_debug`

**Attributes:**

- `#[macro_export]`

Macro for logging debug messages in OpenTelemetry.

# Fields:
- `name`: The operation or action being logged.
- Additional optional key-value pairs can be passed as attributes.

# Example:
```rust
use opentelemetry::otel_debug;
otel_debug!(name: "debug_operation", debug_level = "high", version = "1.0.0");
```

```rust
pub macro_rules! otel_debug {
    /* macro_rules! otel_debug {
    (name: $name:expr $(,)?) => { ... };
    (name: $name:expr, $($key:ident = $value:expr),+ $(,)?) => { ... };
} */
}
```

### Macro `otel_error`

**Attributes:**

- `#[macro_export]`

Macro for logging error messages in OpenTelemetry.

# Fields:
- `name`: The operation or action being logged.
- Additional optional key-value pairs can be passed as attributes.

# Example:
```rust
use opentelemetry::otel_error;
otel_error!(name: "export_failure", error_code = 500, version = "1.0.0");
```

```rust
pub macro_rules! otel_error {
    /* macro_rules! otel_error {
    (name: $name:expr $(,)?) => { ... };
    (name: $name:expr, $($key:ident = $value:expr),+ $(,)?) => { ... };
} */
}
```

## Re-exports

### Re-export `Context`

```rust
pub use context::Context;
```

### Re-export `ContextGuard`

```rust
pub use context::ContextGuard;
```

### Re-export `Array`

```rust
pub use common::Array;
```

### Re-export `InstrumentationScope`

```rust
pub use common::InstrumentationScope;
```

### Re-export `InstrumentationScopeBuilder`

```rust
pub use common::InstrumentationScopeBuilder;
```

### Re-export `Key`

```rust
pub use common::Key;
```

### Re-export `KeyValue`

```rust
pub use common::KeyValue;
```

### Re-export `StringValue`

```rust
pub use common::StringValue;
```

### Re-export `Value`

```rust
pub use common::Value;
```

