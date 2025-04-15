# Crate Documentation

**Version:** 0.28.0

**Format Version:** 43

# Module `tracing_opentelemetry`

# Tracing OpenTelemetry

[`tracing`] is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information. This crate provides a layer
that connects spans from multiple systems into a trace and emits them to
[OpenTelemetry]-compatible distributed tracing systems for processing and
visualization.

[OpenTelemetry]: https://opentelemetry.io
[`tracing`]: https://github.com/tokio-rs/tracing

*Compiler support: [requires `rustc` 1.65+][msrv]*

[msrv]: #supported-rust-versions

### Special Fields

Fields with an `otel.` prefix are reserved for this crate and have specific
meaning. They are treated as ordinary fields by other layers. The current
special fields are:

* `otel.name`: Override the span name sent to OpenTelemetry exporters.
   Setting this field is useful if you want to display non-static information
   in your span name.
* `otel.kind`: Set the span kind to one of the supported OpenTelemetry [span kinds].
* `otel.status_code`: Set the span status code to one of the supported OpenTelemetry [span status codes].
* `otel.status_message`: Set the span status message.

[span kinds]: opentelemetry::trace::SpanKind
[span status codes]: opentelemetry::trace::Status

### Semantic Conventions

OpenTelemetry defines conventional names for attributes of common
operations. These names can be assigned directly as fields, e.g.
`trace_span!("request", "otel.kind" = %SpanKind::Client, "url.full" = ..)`, and they
will be passed through to your configured OpenTelemetry exporter. You can
find the full list of the operations and their expected field names in the
[semantic conventions] spec.

[semantic conventions]: https://github.com/open-telemetry/semantic-conventions

### Stability Status

The OpenTelemetry specification is currently in beta so some breaking
changes may still occur on the path to 1.0. You can follow the changes via
the [spec repository] to track progress toward stabilization.

[spec repository]: https://github.com/open-telemetry/opentelemetry-specification

## Examples

```
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry::trace::{Tracer, TracerProvider as _};
use tracing::{error, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

// Create a new OpenTelemetry trace pipeline that prints to stdout
let provider = TracerProvider::builder()
    .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
    .build();
let tracer = provider.tracer("readme_example");

// Create a tracing layer with the configured tracer
let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

// Use the tracing subscriber `Registry`, or any other subscriber
// that impls `LookupSpan`
let subscriber = Registry::default().with(telemetry);

// Trace executed code
tracing::subscriber::with_default(subscriber, || {
    // Spans will be sent to the configured OpenTelemetry exporter
    let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    let _enter = root.enter();

    error!("This event will be logged in the root span.");
});
```

## Feature Flags

- `metrics`: Enables the [`MetricsLayer`] type, a [layer] that
  exports OpenTelemetry metrics from specifically-named events. This enables
  the `metrics` feature flag on the `opentelemetry` crate.  *Enabled by
  default*.

[layer]: tracing_subscriber::layer

## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.60. The current Tracing version is not guaranteed to build on
Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.45, the minimum supported version will not be
increased past 1.42, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.

[subscriber]: tracing_subscriber::subscribe

## Types

### Struct `OtelData`

Per-span OpenTelemetry data tracked by this crate.

Useful for implementing [PreSampledTracer] in alternate otel SDKs.

```rust
pub struct OtelData {
    pub parent_cx: opentelemetry::Context,
    pub builder: opentelemetry::trace::SpanBuilder,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `parent_cx` | `opentelemetry::Context` | The parent otel `Context` for the current tracing span. |
| `builder` | `opentelemetry::trace::SpanBuilder` | The otel span data recorded during the current tracing span. |

#### Implementations

##### Trait Implementations

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FutureExt**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Instrument**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OtelData { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
## Re-exports

### Re-export `layer`

```rust
pub use layer::layer;
```

### Re-export `OpenTelemetryLayer`

```rust
pub use layer::OpenTelemetryLayer;
```

### Re-export `MetricsLayer`

**Attributes:**

- `#[<cfg>(feature = "metrics")]`

```rust
pub use metrics::MetricsLayer;
```

### Re-export `OpenTelemetrySpanExt`

```rust
pub use span_ext::OpenTelemetrySpanExt;
```

### Re-export `PreSampledTracer`

```rust
pub use tracer::PreSampledTracer;
```

