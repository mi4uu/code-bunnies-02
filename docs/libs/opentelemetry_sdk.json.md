# Crate Documentation

**Version:** 0.27.1

**Format Version:** 43

# Module `opentelemetry_sdk`

Implements the [`SDK`] component of [OpenTelemetry].

*[Supported Rust Versions](#supported-rust-versions)*

[`SDK`]: https://opentelemetry.io/docs/specs/otel/overview/#sdk
[OpenTelemetry]: https://opentelemetry.io/docs/what-is-opentelemetry/
[msrv]: #supported-rust-versions

# Getting Started

```no_run
# #[cfg(feature = "trace")]
# {
use opentelemetry::{global, trace::{Tracer, TracerProvider as _}};
use opentelemetry_sdk::trace::TracerProvider;

fn main() {
    // Choose an exporter like `opentelemetry_stdout::SpanExporter`
    # fn example<T: opentelemetry_sdk::export::trace::SpanExporter + 'static>(new_exporter: impl Fn() -> T) {
    let exporter = new_exporter();

    // Create a new trace pipeline that prints to stdout
    let provider = TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
    let tracer = provider.tracer("readme_example");

    tracer.in_span("doing_work", |cx| {
        // Traced app logic here...
    });

    // Shutdown trace pipeline
    global::shutdown_tracer_provider();
    # }
}
# }
```

See the [examples] directory for different integration patterns.

See the API [`trace`] module docs for more information on creating and managing
spans.

[examples]: https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples
[`trace`]: https://docs.rs/opentelemetry/latest/opentelemetry/trace/index.html

# Metrics (Alpha)

Note: the metrics implementation is **still in progress** and **subject to major
changes**.

### Creating instruments and recording measurements

```
# #[cfg(feature = "metrics")]
# {
use opentelemetry::{global, KeyValue};

// get a meter from a provider
let meter = global::meter("my_service");

// create an instrument
let counter = meter.u64_counter("my_counter").build();

// record a measurement
counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);
# }
```

See the [examples] directory for different integration patterns.

See the API [`metrics`] module docs for more information on creating and
managing instruments.

[examples]: https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples
[`metrics`]: https://docs.rs/opentelemetry/latest/opentelemetry/metrics/index.html

## Crate Feature Flags

The following feature flags can used to control the telemetry signals to use:

* `trace`: Includes the trace SDK (enabled by default).
* `metrics`: Includes the metrics SDK.
* `logs`: Includes the logs SDK.

For `trace` the following feature flags are available:

* `jaeger_remote_sampler`: Enables the [Jaeger remote sampler](https://www.jaegertracing.io/docs/1.53/sampling/).

For `logs` the following feature flags are available:

* `spec_unstable_logs_enabled`: control the log level

Support for recording and exporting telemetry asynchronously and perform
metrics aggregation can be added via the following flags:

* `rt-tokio`: Spawn telemetry tasks using [tokio]'s multi-thread runtime.
* `rt-tokio-current-thread`: Spawn telemetry tasks on a separate runtime so that the main runtime won't be blocked.
* `rt-async-std`: Spawn telemetry tasks using [async-std]'s runtime.

[tokio]: https://crates.io/crates/tokio
[async-std]: https://crates.io/crates/async-std

## Modules

## Module `export`

Telemetry Export

```rust
pub mod export { /* ... */ }
```

### Modules

## Module `trace`

**Attributes:**

- `#[<cfg>(feature = "trace")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "trace")))]`

Trace exporters

```rust
pub mod trace { /* ... */ }
```

### Types

#### Type Alias `ExportResult`

Describes the result of an export.

```rust
pub type ExportResult = Result<(), opentelemetry::trace::TraceError>;
```

#### Struct `SpanData`

`SpanData` contains all the information collected by a `Span` and can be used
by exporters as a standard input.

```rust
pub struct SpanData {
    pub span_context: opentelemetry::trace::SpanContext,
    pub parent_span_id: opentelemetry::trace::SpanId,
    pub span_kind: opentelemetry::trace::SpanKind,
    pub name: std::borrow::Cow<''static, str>,
    pub start_time: std::time::SystemTime,
    pub end_time: std::time::SystemTime,
    pub attributes: Vec<opentelemetry::KeyValue>,
    pub dropped_attributes_count: u32,
    pub events: crate::trace::SpanEvents,
    pub links: crate::trace::SpanLinks,
    pub status: opentelemetry::trace::Status,
    pub instrumentation_scope: opentelemetry::InstrumentationScope,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `span_context` | `opentelemetry::trace::SpanContext` | Exportable `SpanContext` |
| `parent_span_id` | `opentelemetry::trace::SpanId` | Span parent id |
| `span_kind` | `opentelemetry::trace::SpanKind` | Span kind |
| `name` | `std::borrow::Cow<''static, str>` | Span name |
| `start_time` | `std::time::SystemTime` | Span start time |
| `end_time` | `std::time::SystemTime` | Span end time |
| `attributes` | `Vec<opentelemetry::KeyValue>` | Span attributes |
| `dropped_attributes_count` | `u32` | The number of attributes that were above the configured limit, and thus<br>dropped. |
| `events` | `crate::trace::SpanEvents` | Span events |
| `links` | `crate::trace::SpanLinks` | Span Links |
| `status` | `opentelemetry::trace::Status` | Span status |
| `instrumentation_scope` | `opentelemetry::InstrumentationScope` | Instrumentation scope that produced this span |

##### Implementations

###### Trait Implementations

- **Send**
- **UnwindSafe**
- **FutureExt**
- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Instrument**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SpanData { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SpanData) -> bool { /* ... */ }
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `SpanExporter`

`SpanExporter` defines the interface that protocol-specific exporters must
implement so that they can be plugged into OpenTelemetry SDK and support
sending of telemetry data.

The goal of the interface is to minimize burden of implementation for
protocol-dependent telemetry exporters. The protocol exporter is expected to
be primarily a simple telemetry data encoder and transmitter.

```rust
pub trait SpanExporter: Send + Sync + Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `export`: Exports a batch of readable spans. Protocol exporters that will

##### Provided Methods

- ```rust
  fn shutdown(self: &mut Self) { /* ... */ }
  ```
  Shuts down the exporter. Called when SDK is shut down. This is an

- ```rust
  fn force_flush(self: &mut Self) -> BoxFuture<''static, ExportResult> { /* ... */ }
  ```
  This is a hint to ensure that the export of any Spans the exporter

- ```rust
  fn set_resource(self: &mut Self, _resource: &Resource) { /* ... */ }
  ```
  Set the resource for the exporter.

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

## Module `metrics`

**Attributes:**

- `#[<cfg>(feature = "metrics")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "metrics")))]`

The crust of the OpenTelemetry metrics SDK.

## Configuration

The metrics SDK configuration is stored with each [SdkMeterProvider].
Configuration for [Resource]s, [View]s, and [ManualReader] or
[PeriodicReader] instances can be specified.

### Example

```
use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry_sdk::{metrics::SdkMeterProvider, Resource};

// Generate SDK configuration, resource, views, etc
let resource = Resource::default(); // default attributes about the current process

// Create a meter provider with the desired config
let meter_provider = SdkMeterProvider::builder().with_resource(resource).build();
global::set_meter_provider(meter_provider.clone());

// Use the meter provider to create meter instances
let meter = global::meter("my_app");

// Create instruments scoped to the meter
let counter = meter
    .u64_counter("power_consumption")
    .with_unit("kWh")
    .build();

// use instruments to record measurements
counter.add(10, &[KeyValue::new("rate", "standard")]);

// shutdown the provider at the end of the application to ensure any metrics not yet
// exported are flushed.
meter_provider.shutdown().unwrap();
```

[Resource]: crate::Resource

```rust
pub mod metrics { /* ... */ }
```

### Modules

## Module `data`

Types for delivery of pre-aggregated metric time series data.

```rust
pub mod data { /* ... */ }
```

### Types

#### Struct `ResourceMetrics`

A collection of [ScopeMetrics] and the associated [Resource] that created them.

```rust
pub struct ResourceMetrics {
    pub resource: crate::Resource,
    pub scope_metrics: Vec<ScopeMetrics>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `resource` | `crate::Resource` | The entity that collected the metrics. |
| `scope_metrics` | `Vec<ScopeMetrics>` | The collection of metrics with unique [InstrumentationScope]s. |

##### Implementations

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **FutureExt**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Instrument**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **WithSubscriber**
#### Struct `ScopeMetrics`

A collection of metrics produced by a meter.

```rust
pub struct ScopeMetrics {
    pub scope: opentelemetry::InstrumentationScope,
    pub metrics: Vec<Metric>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `scope` | `opentelemetry::InstrumentationScope` | The [InstrumentationScope] that the meter was created with. |
| `metrics` | `Vec<Metric>` | The list of aggregations created by the meter. |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **FutureExt**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Instrument**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
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

- **Sync**
- **Default**
  - ```rust
    fn default() -> ScopeMetrics { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Metric`

A collection of one or more aggregated time series from an [Instrument].

[Instrument]: crate::metrics::Instrument

```rust
pub struct Metric {
    pub name: std::borrow::Cow<''static, str>,
    pub description: std::borrow::Cow<''static, str>,
    pub unit: std::borrow::Cow<''static, str>,
    pub data: Box<dyn Aggregation>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `std::borrow::Cow<''static, str>` | The name of the instrument that created this data. |
| `description` | `std::borrow::Cow<''static, str>` | The description of the instrument, which can be used in documentation. |
| `unit` | `std::borrow::Cow<''static, str>` | The unit in which the instrument reports. |
| `data` | `Box<dyn Aggregation>` | The aggregated data from an instrument. |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **FutureExt**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
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

- **Instrument**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Gauge`

A measurement of the current value of an instrument.

```rust
pub struct Gauge<T> {
    pub data_points: Vec<DataPoint<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data_points` | `Vec<DataPoint<T>>` | Represents individual aggregated measurements with unique attributes. |

##### Implementations

###### Trait Implementations

- **Send**
- **UnwindSafe**
- **Unpin**
- **Freeze**
- **WithSubscriber**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Aggregation**
  - ```rust
    fn as_any(self: &Self) -> &dyn any::Any { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut dyn any::Any { /* ... */ }
    ```

- **Instrument**
- **RefUnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FutureExt**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Sum`

Represents the sum of all measurements of values from an instrument.

```rust
pub struct Sum<T> {
    pub data_points: Vec<DataPoint<T>>,
    pub temporality: super::Temporality,
    pub is_monotonic: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data_points` | `Vec<DataPoint<T>>` | Represents individual aggregated measurements with unique attributes. |
| `temporality` | `super::Temporality` | Describes if the aggregation is reported as the change from the last report<br>time, or the cumulative changes since a fixed start time. |
| `is_monotonic` | `bool` | Whether this aggregation only increases or decreases. |

##### Implementations

###### Trait Implementations

- **Freeze**
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

- **Send**
- **UnwindSafe**
- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Aggregation**
  - ```rust
    fn as_any(self: &Self) -> &dyn any::Any { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut dyn any::Any { /* ... */ }
    ```

- **Instrument**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **FutureExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
#### Struct `DataPoint`

DataPoint is a single data point in a time series.

```rust
pub struct DataPoint<T> {
    pub attributes: Vec<opentelemetry::KeyValue>,
    pub start_time: Option<std::time::SystemTime>,
    pub time: Option<std::time::SystemTime>,
    pub value: T,
    pub exemplars: Vec<Exemplar<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `attributes` | `Vec<opentelemetry::KeyValue>` | Attributes is the set of key value pairs that uniquely identify the<br>time series. |
| `start_time` | `Option<std::time::SystemTime>` | The time when the time series was started. |
| `time` | `Option<std::time::SystemTime>` | The time when the time series was recorded. |
| `value` | `T` | The value of this data point. |
| `exemplars` | `Vec<Exemplar<T>>` | The sampled [Exemplar]s collected during the time series. |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FutureExt**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DataPoint<T>) -> bool { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **WithSubscriber**
- **Send**
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

#### Struct `Histogram`

Represents the histogram of all measurements of values from an instrument.

```rust
pub struct Histogram<T> {
    pub data_points: Vec<HistogramDataPoint<T>>,
    pub temporality: super::Temporality,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data_points` | `Vec<HistogramDataPoint<T>>` | Individual aggregated measurements with unique attributes. |
| `temporality` | `super::Temporality` | Describes if the aggregation is reported as the change from the last report<br>time, or the cumulative changes since a fixed start time. |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Aggregation**
  - ```rust
    fn as_any(self: &Self) -> &dyn any::Any { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut dyn any::Any { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **UnwindSafe**
- **Send**
- **FutureExt**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **WithSubscriber**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
#### Struct `HistogramDataPoint`

A single histogram data point in a time series.

```rust
pub struct HistogramDataPoint<T> {
    pub attributes: Vec<opentelemetry::KeyValue>,
    pub start_time: std::time::SystemTime,
    pub time: std::time::SystemTime,
    pub count: u64,
    pub bounds: Vec<f64>,
    pub bucket_counts: Vec<u64>,
    pub min: Option<T>,
    pub max: Option<T>,
    pub sum: T,
    pub exemplars: Vec<Exemplar<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `attributes` | `Vec<opentelemetry::KeyValue>` | The set of key value pairs that uniquely identify the time series. |
| `start_time` | `std::time::SystemTime` | The time when the time series was started. |
| `time` | `std::time::SystemTime` | The time when the time series was recorded. |
| `count` | `u64` | The number of updates this histogram has been calculated with. |
| `bounds` | `Vec<f64>` | The upper bounds of the buckets of the histogram.<br><br>Because the last boundary is +infinity this one is implied. |
| `bucket_counts` | `Vec<u64>` | The count of each of the buckets. |
| `min` | `Option<T>` | The minimum value recorded. |
| `max` | `Option<T>` | The maximum value recorded. |
| `sum` | `T` | The sum of the values recorded. |
| `exemplars` | `Vec<Exemplar<T>>` | The sampled [Exemplar]s collected during the time series. |

##### Implementations

###### Trait Implementations

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Sync**
- **WithSubscriber**
- **Instrument**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FutureExt**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HistogramDataPoint<T>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `ExponentialHistogram`

The histogram of all measurements of values from an instrument.

```rust
pub struct ExponentialHistogram<T> {
    pub data_points: Vec<ExponentialHistogramDataPoint<T>>,
    pub temporality: super::Temporality,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data_points` | `Vec<ExponentialHistogramDataPoint<T>>` | The individual aggregated measurements with unique attributes. |
| `temporality` | `super::Temporality` | Describes if the aggregation is reported as the change from the last report<br>time, or the cumulative changes since a fixed start time. |

##### Implementations

###### Trait Implementations

- **WithSubscriber**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Sync**
- **Aggregation**
  - ```rust
    fn as_any(self: &Self) -> &dyn any::Any { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut dyn any::Any { /* ... */ }
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

- **FutureExt**
- **Unpin**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
#### Struct `ExponentialHistogramDataPoint`

A single exponential histogram data point in a time series.

```rust
pub struct ExponentialHistogramDataPoint<T> {
    pub attributes: Vec<opentelemetry::KeyValue>,
    pub start_time: std::time::SystemTime,
    pub time: std::time::SystemTime,
    pub count: usize,
    pub min: Option<T>,
    pub max: Option<T>,
    pub sum: T,
    pub scale: i8,
    pub zero_count: u64,
    pub positive_bucket: ExponentialBucket,
    pub negative_bucket: ExponentialBucket,
    pub zero_threshold: f64,
    pub exemplars: Vec<Exemplar<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `attributes` | `Vec<opentelemetry::KeyValue>` | The set of key value pairs that uniquely identify the time series. |
| `start_time` | `std::time::SystemTime` | When the time series was started. |
| `time` | `std::time::SystemTime` | The time when the time series was recorded. |
| `count` | `usize` | The number of updates this histogram has been calculated with. |
| `min` | `Option<T>` | The minimum value recorded. |
| `max` | `Option<T>` | The maximum value recorded. |
| `sum` | `T` | The sum of the values recorded. |
| `scale` | `i8` | Describes the resolution of the histogram.<br><br>Boundaries are located at powers of the base, where:<br><br>  base = 2 ^ (2 ^ -scale) |
| `zero_count` | `u64` | The number of values whose absolute value is less than or equal to<br>`zero_threshold`.<br><br>When `zero_threshold` is `0`, this is the number of values that cannot be<br>expressed using the standard exponential formula as well as values that have<br>been rounded to zero. |
| `positive_bucket` | `ExponentialBucket` | The range of positive value bucket counts. |
| `negative_bucket` | `ExponentialBucket` | The range of negative value bucket counts. |
| `zero_threshold` | `f64` | The width of the zero region.<br><br>Where the zero region is defined as the closed interval<br>[-zero_threshold, zero_threshold]. |
| `exemplars` | `Vec<Exemplar<T>>` | The sampled exemplars collected during the time series. |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExponentialHistogramDataPoint<T>) -> bool { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FutureExt**
- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
#### Struct `ExponentialBucket`

A set of bucket counts, encoded in a contiguous array of counts.

```rust
pub struct ExponentialBucket {
    pub offset: i32,
    pub counts: Vec<u64>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `offset` | `i32` | The bucket index of the first entry in the `counts` vec. |
| `counts` | `Vec<u64>` | A vec where `counts[i]` carries the count of the bucket at index `offset + i`.<br><br>`counts[i]` is the count of values greater than base^(offset+i) and less than<br>or equal to base^(offset+i+1). |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FutureExt**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExponentialBucket) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Instrument**
- **WithSubscriber**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Exemplar`

A measurement sampled from a time series providing a typical example.

```rust
pub struct Exemplar<T> {
    pub filtered_attributes: Vec<opentelemetry::KeyValue>,
    pub time: std::time::SystemTime,
    pub value: T,
    pub span_id: [u8; 8],
    pub trace_id: [u8; 16],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `filtered_attributes` | `Vec<opentelemetry::KeyValue>` | The attributes recorded with the measurement but filtered out of the<br>time series' aggregated data. |
| `time` | `std::time::SystemTime` | The time when the measurement was recorded. |
| `value` | `T` | The measured value. |
| `span_id` | `[u8; 8]` | The ID of the span that was active during the measurement.<br><br>If no span was active or the span was not sampled this will be empty. |
| `trace_id` | `[u8; 16]` | The ID of the trace the active span belonged to during the measurement.<br><br>If no span was active or the span was not sampled this will be empty. |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **WithSubscriber**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Exemplar<T>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FutureExt**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Instrument**
- **StructuralPartialEq**
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

- **Unpin**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Traits

#### Trait `Aggregation`

The store of data reported by an [Instrument].

It will be one of: [Gauge], [Sum], or [Histogram].

[Instrument]: crate::metrics::Instrument

```rust
pub trait Aggregation: fmt::Debug + any::Any + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_any`: Support downcasting
- `as_mut`: Support downcasting during aggregation

##### Implementations

This trait is implemented for the following types:

- `Gauge<T>` with <T: fmt::Debug + Send + Sync + ''static>
- `Sum<T>` with <T: fmt::Debug + Send + Sync + ''static>
- `Histogram<T>` with <T: fmt::Debug + Send + Sync + ''static>
- `ExponentialHistogram<T>` with <T: fmt::Debug + Send + Sync + ''static>

## Module `exporter`

Interfaces for exporting metrics

```rust
pub mod exporter { /* ... */ }
```

### Traits

#### Trait `PushMetricExporter`

Exporter handles the delivery of metric data to external receivers.

This is the final component in the metric push pipeline.

```rust
pub trait PushMetricExporter: Send + Sync + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `export`: Export serializes and transmits metric data to a receiver.
- `force_flush`: Flushes any metric data held by an exporter.
- `shutdown`: Releases any held computational resources.
- `temporality`: Access the [Temporality] of the MetricExporter.

## Module `reader`

Interfaces for reading and producing metrics

```rust
pub mod reader { /* ... */ }
```

### Traits

#### Trait `MetricReader`

The interface used between the SDK and an exporter.

Control flow is bi-directional through the `MetricReader`, since the SDK
initiates `force_flush` and `shutdown` while the reader initiates
collection. The `register_pipeline` method here informs the metric reader
that it can begin reading, signaling the start of bi-directional control
flow.

Typically, push-based exporters that are periodic will implement
`MetricExporter` themselves and construct a `PeriodicReader` to satisfy this
interface.

Pull-based exporters will typically implement `MetricReader` themselves,
since they read on demand.

```rust
pub trait MetricReader: fmt::Debug + Send + Sync + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `register_pipeline`: Registers a [MetricReader] with a [Pipeline].
- `collect`: Gathers and returns all metric data related to the [MetricReader] from the
- `force_flush`: Flushes all metric measurements held in an export pipeline.
- `shutdown`: Flushes all metric measurements held in an export pipeline and releases any
- `temporality`: The output temporality, a function of instrument kind.

##### Implementations

This trait is implemented for the following types:

- `ManualReader`
- `PeriodicReader`

### Types

#### Enum `Temporality`

**Attributes:**

- `#[non_exhaustive]`

Defines the window that an aggregation was calculated over.

```rust
pub enum Temporality {
    Cumulative,
    Delta,
    LowMemory,
}
```

##### Variants

###### `Cumulative`

A measurement interval that continues to expand forward in time from a
starting point.

New measurements are added to all previous measurements since a start time.

###### `Delta`

A measurement interval that resets each cycle.

Measurements from one cycle are recorded independently, measurements from
other cycles do not affect them.

###### `LowMemory`

Configures Synchronous Counter and Histogram instruments to use
Delta aggregation temporality, which allows them to shed memory
following a cardinality explosion, thus use less memory.

##### Implementations

###### Trait Implementations

- **WithSubscriber**
- **Sync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Temporality { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Temporality) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FutureExt**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Temporality { /* ... */ }
    ```

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Instrument**
### Re-exports

#### Re-export `MetricError`

```rust
pub use error::MetricError;
```

#### Re-export `MetricResult`

```rust
pub use error::MetricResult;
```

#### Re-export `InstrumentKind`

```rust
pub use instrument::InstrumentKind;
```

#### Re-export `aggregation::*`

```rust
pub use aggregation::*;
```

#### Re-export `manual_reader::*`

```rust
pub use manual_reader::*;
```

#### Re-export `meter_provider::*`

```rust
pub use meter_provider::*;
```

#### Re-export `periodic_reader::*`

```rust
pub use periodic_reader::*;
```

## Module `propagation`

**Attributes:**

- `#[<cfg>(feature = "trace")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "trace")))]`

OpenTelemetry Propagators

```rust
pub mod propagation { /* ... */ }
```

### Re-exports

#### Re-export `BaggagePropagator`

```rust
pub use baggage::BaggagePropagator;
```

#### Re-export `TraceContextPropagator`

```rust
pub use trace_context::TraceContextPropagator;
```

## Module `resource`

Representations of entities producing telemetry.

A [Resource] is an immutable representation of the entity producing
telemetry as attributes. For example, a process producing telemetry that is
running in a container on Kubernetes has a Pod name, it is in a namespace
and possibly is part of a Deployment which also has a name. All three of
these attributes can be included in the `Resource`. Note that there are
certain ["standard attributes"] that have prescribed meanings.

["standard attributes"]: https://github.com/open-telemetry/opentelemetry-specification/blob/v1.9.0/specification/resource/semantic_conventions/README.md

# Resource detectors

[`ResourceDetector`]s are used to detect resource from runtime or
environmental variables. The following are provided by default with this
SDK.

- [`EnvResourceDetector`] - detect resource from environmental variables.
- [`TelemetryResourceDetector`] - detect telemetry SDK's information.

The OS and Process resource detectors are packaged separately in the
[`opentelemetry-resource-detector` crate](https://github.com/open-telemetry/opentelemetry-rust-contrib/tree/main/opentelemetry-resource-detectors).

```rust
pub mod resource { /* ... */ }
```

### Types

#### Struct `Resource`

An immutable representation of the entity producing telemetry as attributes.
Utilizes `Arc` for efficient sharing and cloning.

```rust
pub struct Resource {
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
  pub fn empty() -> Self { /* ... */ }
  ```
  Creates an empty resource.

- ```rust
  pub fn new<T: IntoIterator<Item = KeyValue>>(kvs: T) -> Self { /* ... */ }
  ```
  Create a new `Resource` from key value pairs.

- ```rust
  pub fn new_with_defaults<T: IntoIterator<Item = KeyValue>>(keys: T) -> Self { /* ... */ }
  ```
  Create a new `Resource::default()` and merge with provided key value pairs.

- ```rust
  pub fn from_schema_url<KV, S>(kvs: KV, schema_url: S) -> Self
where
    KV: IntoIterator<Item = KeyValue>,
    S: Into<Cow<''static, str>> { /* ... */ }
  ```
  Create a new `Resource` from a key value pairs and [schema url].

- ```rust
  pub fn from_detectors(timeout: Duration, detectors: Vec<Box<dyn ResourceDetector>>) -> Self { /* ... */ }
  ```
  Create a new `Resource` from resource detectors.

- ```rust
  pub fn merge<T: Deref<Target = Self>>(self: &Self, other: T) -> Self { /* ... */ }
  ```
  Create a new `Resource` by combining two resources.

- ```rust
  pub fn schema_url(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Return the [schema url] of the resource. If the resource does not have a schema url, return `None`.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of attributes for this resource

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the resource contains no attributes.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_> { /* ... */ }
  ```
  Gets an iterator over the attributes of this resource.

- ```rust
  pub fn get(self: &Self, key: Key) -> Option<Value> { /* ... */ }
  ```
  Retrieve the value from resource associate with given key.

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FutureExt**
- **Instrument**
- **UnwindSafe**
- **WithSubscriber**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Resource { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
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

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Resource) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `Iter`

An iterator over the entries of a `Resource`.

```rust
pub struct Iter<''a>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **IteratorRandom**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Instrument**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **WithSubscriber**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **FutureExt**
### Traits

#### Trait `ResourceDetector`

ResourceDetector detects OpenTelemetry resource information

Implementations of this trait can be passed to
the [`Resource::from_detectors`] function to generate a Resource from the merged information.

```rust
pub trait ResourceDetector {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `detect`: detect returns an initialized Resource based on gathered information.

##### Implementations

This trait is implemented for the following types:

- `EnvResourceDetector`
- `SdkProvidedResourceDetector`
- `TelemetryResourceDetector`

### Re-exports

#### Re-export `EnvResourceDetector`

```rust
pub use env::EnvResourceDetector;
```

#### Re-export `SdkProvidedResourceDetector`

```rust
pub use env::SdkProvidedResourceDetector;
```

#### Re-export `TelemetryResourceDetector`

```rust
pub use telemetry::TelemetryResourceDetector;
```

## Module `runtime`

Provides an abstraction of several async runtimes

This  allows OpenTelemetry to work with any current or future runtime. There are currently
builtin implementations for [Tokio] and [async-std].

[Tokio]: https://crates.io/crates/tokio
[async-std]: https://crates.io/crates/async-std

```rust
pub mod runtime { /* ... */ }
```

### Types

#### Enum `TrySendError`

Error returned by a [`TrySend`] implementation.

```rust
pub enum TrySendError {
    ChannelFull,
    ChannelClosed,
    Other(Box<dyn std::error::Error + Send + Sync + ''static>),
}
```

##### Variants

###### `ChannelFull`

Send failed due to the channel being full.

###### `ChannelClosed`

Send failed due to the channel being closed.

###### `Other`

Any other send error that isnt covered above.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<dyn std::error::Error + Send + Sync + ''static>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FutureExt**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **WithSubscriber**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: Box<dyn std::error::Error + Send + Sync + ''static>) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Instrument**
- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
### Traits

#### Trait `Runtime`

A runtime is an abstraction of an async runtime like [Tokio] or [async-std]. It allows
OpenTelemetry to work with any current and hopefully future runtime implementation.

[Tokio]: https://crates.io/crates/tokio
[async-std]: https://crates.io/crates/async-std

```rust
pub trait Runtime: Clone + Send + Sync + ''static {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Interval`: A future stream, which returns items in a previously specified interval. The item type is
- `Delay`: A future, which resolves after a previously specified amount of time. The output type is

###### Required Methods

- `interval`: Create a [futures_util::stream::Stream], which returns a new item every
- `spawn`: Spawn a new task or thread, which executes the given future.
- `delay`: Return a new future, which resolves after the specified [std::time::Duration].

#### Trait `RuntimeChannel`

`RuntimeChannel` is an extension to [`Runtime`]. Currently, it provides a
channel that is used by the [log] and [span] batch processors.

[log]: crate::logs::BatchLogProcessor
[span]: crate::trace::BatchSpanProcessor

```rust
pub trait RuntimeChannel: Runtime {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Receiver`: A future stream to receive batch messages from channels.
- `Sender`: A batch messages sender that can be sent across threads safely.

###### Required Methods

- `batch_message_channel`: Return the sender and receiver used to send batch messages.

#### Trait `TrySend`

TrySend is an abstraction of `Sender` that is capable of sending messages through a reference.

```rust
pub trait TrySend: Sync + Send {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Message`: The message that will be sent.

###### Required Methods

- `try_send`: Try to send a message batch to a worker thread.

## Module `trace`

**Attributes:**

- `#[allow(deprecated)]`
- `#[<cfg>(feature = "trace")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "trace")))]`

# OpenTelemetry Trace SDK

The tracing SDK consist of a few main structs:

* The [`Tracer`] struct which performs all tracing operations.
* The [`Span`] struct with is a mutable object storing information about the
  current operation execution.
* The [`TracerProvider`] struct which configures and produces [`Tracer`]s.

```rust
pub mod trace { /* ... */ }
```

### Re-exports

#### Re-export `config`

```rust
pub use config::config;
```

#### Re-export `Config`

```rust
pub use config::Config;
```

#### Re-export `SpanEvents`

```rust
pub use events::SpanEvents;
```

#### Re-export `IdGenerator`

```rust
pub use id_generator::IdGenerator;
```

#### Re-export `RandomIdGenerator`

```rust
pub use id_generator::RandomIdGenerator;
```

#### Re-export `SpanLinks`

```rust
pub use links::SpanLinks;
```

#### Re-export `Builder`

```rust
pub use provider::Builder;
```

#### Re-export `TracerProvider`

```rust
pub use provider::TracerProvider;
```

#### Re-export `Sampler`

```rust
pub use sampler::Sampler;
```

#### Re-export `ShouldSample`

```rust
pub use sampler::ShouldSample;
```

#### Re-export `Span`

```rust
pub use span::Span;
```

#### Re-export `SpanLimits`

```rust
pub use span_limit::SpanLimits;
```

#### Re-export `BatchConfig`

```rust
pub use span_processor::BatchConfig;
```

#### Re-export `BatchConfigBuilder`

```rust
pub use span_processor::BatchConfigBuilder;
```

#### Re-export `BatchSpanProcessor`

```rust
pub use span_processor::BatchSpanProcessor;
```

#### Re-export `BatchSpanProcessorBuilder`

```rust
pub use span_processor::BatchSpanProcessorBuilder;
```

#### Re-export `SimpleSpanProcessor`

```rust
pub use span_processor::SimpleSpanProcessor;
```

#### Re-export `SpanProcessor`

```rust
pub use span_processor::SpanProcessor;
```

#### Re-export `Tracer`

```rust
pub use tracer::Tracer;
```

## Module `error`

Wrapper for error from trace, logs and metrics part of open telemetry.

```rust
pub mod error { /* ... */ }
```

### Types

#### Enum `Error`

**Attributes:**

- `#[non_exhaustive]`

Wrapper for error from both tracing and metrics part of open telemetry.

```rust
pub enum Error {
    Trace(opentelemetry::trace::TraceError),
    Metric(crate::metrics::MetricError),
    Propagation(opentelemetry::propagation::PropagationError),
    Other(String),
}
```

##### Variants

###### `Trace`

Failed to export traces.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `opentelemetry::trace::TraceError` |  |

###### `Metric`

An issue raised by the metrics module.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::metrics::MetricError` |  |

###### `Propagation`

Error happens when injecting and extracting information using propagators.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `opentelemetry::propagation::PropagationError` |  |

###### `Other`

Other types of failures not covered by the variants above.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **WithSubscriber**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: TraceError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: MetricError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: PropagationError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: PoisonError<T>) -> Self { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Instrument**
- **Sync**
- **Unpin**
- **RefUnwindSafe**
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

- **FutureExt**
## Re-exports

### Re-export `Resource`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use resource::Resource;
```

