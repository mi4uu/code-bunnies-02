# Crate Documentation

**Version:** 0.1.41

**Format Version:** 43

# Module `tracing`

 A scoped, structured logging and diagnostics system.

 # Overview

 `tracing` is a framework for instrumenting Rust programs to collect
 structured, event-based diagnostic information.

 In asynchronous systems like Tokio, interpreting traditional log messages can
 often be quite challenging. Since individual tasks are multiplexed on the same
 thread, associated events and log lines are intermixed making it difficult to
 trace the logic flow. `tracing` expands upon logging-style diagnostics by
 allowing libraries and applications to record structured events with additional
 information about *temporality* and *causality* — unlike a log message, a span
 in `tracing` has a beginning and end time, may be entered and exited by the
 flow of execution, and may exist within a nested tree of similar spans. In
 addition, `tracing` spans are *structured*, with the ability to record typed
 data as well as textual messages.

 The `tracing` crate provides the APIs necessary for instrumenting libraries
 and applications to emit trace data.

 *Compiler support: [requires `rustc` 1.63+][msrv]*

 [msrv]: #supported-rust-versions
 # Core Concepts

 The core of `tracing`'s API is composed of _spans_, _events_ and
 _subscribers_. We'll cover these in turn.

 ## Spans

 To record the flow of execution through a program, `tracing` introduces the
 concept of [spans]. Unlike a log line that represents a _moment in
 time_, a span represents a _period of time_ with a beginning and an end. When a
 program begins executing in a context or performing a unit of work, it
 _enters_ that context's span, and when it stops executing in that context,
 it _exits_ the span. The span in which a thread is currently executing is
 referred to as that thread's _current_ span.

 For example:
 ```
 use tracing::{span, Level};
 # fn main() {
 let span = span!(Level::TRACE, "my_span");
 // `enter` returns a RAII guard which, when dropped, exits the span. this
 // indicates that we are in the span for the current lexical scope.
 let _enter = span.enter();
 // perform some work in the context of `my_span`...
 # }
```

 The [`span` module][span]'s documentation provides further details on how to
 use spans.

 <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">

  **Warning**: In asynchronous code that uses async/await syntax,
  `Span::enter` may produce incorrect traces if the returned drop
  guard is held across an await point. See
  [the method documentation][Span#in-asynchronous-code] for details.

 </pre></div>

 ## Events

 An [`Event`] represents a _moment_ in time. It signifies something that
 happened while a trace was being recorded. `Event`s are comparable to the log
 records emitted by unstructured logging code, but unlike a typical log line,
 an `Event` may occur within the context of a span.

 For example:
 ```
 use tracing::{event, span, Level};

 # fn main() {
 // records an event outside of any span context:
 event!(Level::INFO, "something happened");

 let span = span!(Level::INFO, "my_span");
 let _guard = span.enter();

 // records an event within "my_span".
 event!(Level::DEBUG, "something happened inside my_span");
 # }
```

 In general, events should be used to represent points in time _within_ a
 span — a request returned with a given status code, _n_ new items were
 taken from a queue, and so on.

 The [`Event` struct][`Event`] documentation provides further details on using
 events.

 ## Subscribers

 As `Span`s and `Event`s occur, they are recorded or aggregated by
 implementations of the [`Subscriber`] trait. `Subscriber`s are notified
 when an `Event` takes place and when a `Span` is entered or exited. These
 notifications are represented by the following `Subscriber` trait methods:

 + [`event`][Subscriber::event], called when an `Event` takes place,
 + [`enter`], called when execution enters a `Span`,
 + [`exit`], called when execution exits a `Span`

 In addition, subscribers may implement the [`enabled`] function to _filter_
 the notifications they receive based on [metadata] describing each `Span`
 or `Event`. If a call to `Subscriber::enabled` returns `false` for a given
 set of metadata, that `Subscriber` will *not* be notified about the
 corresponding `Span` or `Event`. For performance reasons, if no currently
 active subscribers express interest in a given set of metadata by returning
 `true`, then the corresponding `Span` or `Event` will never be constructed.

 # Usage

 First, add this to your `Cargo.toml`:

 ```toml
 [dependencies]
 tracing = "0.1"
 ```

 ## Recording Spans and Events

 Spans and events are recorded using macros.

 ### Spans

 The [`span!`] macro expands to a [`Span` struct][`Span`] which is used to
 record a span. The [`Span::enter`] method on that struct records that the
 span has been entered, and returns a [RAII] guard object, which will exit
 the span when dropped.

 For example:

 ```rust
 use tracing::{span, Level};
 # fn main() {
 // Construct a new span named "my span" with trace log level.
 let span = span!(Level::TRACE, "my span");

 // Enter the span, returning a guard object.
 let _enter = span.enter();

 // Any trace events that occur before the guard is dropped will occur
 // within the span.

 // Dropping the guard will exit the span.
 # }
 ```

 The [`#[instrument]`][instrument] attribute provides an easy way to
 add `tracing` spans to functions. A function annotated with `#[instrument]`
 will create and enter a span with that function's name every time the
 function is called, with arguments to that function will be recorded as
 fields using `fmt::Debug`.

 For example:
 ```ignore
 # // this doctest is ignored because we don't have a way to say
 # // that it should only be run with cfg(feature = "attributes")
 use tracing::{Level, event, instrument};

 #[instrument]
 pub fn my_function(my_arg: usize) {
     // This event will be recorded inside a span named `my_function` with the
     // field `my_arg`.
     event!(Level::INFO, "inside my_function!");
     // ...
 }
 # fn main() {}
 ```

 For functions which don't have built-in tracing support and can't have
 the `#[instrument]` attribute applied (such as from an external crate),
 the [`Span` struct][`Span`] has a [`in_scope()` method][`in_scope`]
 which can be used to easily wrap synchronous code in a span.

 For example:
 ```rust
 use tracing::info_span;

 # fn doc() -> Result<(), ()> {
 # mod serde_json {
 #    pub(crate) fn from_slice(buf: &[u8]) -> Result<(), ()> { Ok(()) }
 # }
 # let buf: [u8; 0] = [];
 let json = info_span!("json.parse").in_scope(|| serde_json::from_slice(&buf))?;
 # let _ = json; // suppress unused variable warning
 # Ok(())
 # }
 ```

 You can find more examples showing how to use this crate [here][examples].

 [RAII]: https://github.com/rust-unofficial/patterns/blob/main/src/patterns/behavioural/RAII.md
 [examples]: https://github.com/tokio-rs/tracing/tree/master/examples

 ### Events

 [`Event`]s are recorded using the [`event!`] macro:

 ```rust
 # fn main() {
 use tracing::{event, Level};
 event!(Level::INFO, "something has happened!");
 # }
 ```

 ## Using the Macros

 The [`span!`] and [`event!`] macros as well as the `#[instrument]` attribute
 use fairly similar syntax, with some exceptions.

 ### Configuring Attributes

 Both macros require a [`Level`] specifying the verbosity of the span or
 event. Optionally, the, [target] and [parent span] may be overridden. If the
 target and parent span are not overridden, they will default to the
 module path where the macro was invoked and the current span (as determined
 by the subscriber), respectively.

 For example:

 ```
 # use tracing::{span, event, Level};
 # fn main() {
 span!(target: "app_spans", Level::TRACE, "my span");
 event!(target: "app_events", Level::INFO, "something has happened!");
 # }
 ```
 ```
 # use tracing::{span, event, Level};
 # fn main() {
 let span = span!(Level::TRACE, "my span");
 event!(parent: &span, Level::INFO, "something has happened!");
 # }
 ```

 The span macros also take a string literal after the level, to set the name
 of the span (as above).  In the case of the event macros, the name of the event can
 be overridden (the default is `event file:line`) using the `name:` specifier.

 ```
 # use tracing::{span, event, Level};
 # fn main() {
 span!(Level::TRACE, "my span");
 event!(name: "some_info", Level::INFO, "something has happened!");
 # }
 ```

 ### Recording Fields

 Structured fields on spans and events are specified using the syntax
 `field_name = field_value`. Fields are separated by commas.

 ```
 # use tracing::{event, Level};
 # fn main() {
 // records an event with two fields:
 //  - "answer", with the value 42
 //  - "question", with the value "life, the universe and everything"
 event!(Level::INFO, answer = 42, question = "life, the universe, and everything");
 # }
 ```

 As shorthand, local variables may be used as field values without an
 assignment, similar to [struct initializers]. For example:

 ```
 # use tracing::{span, Level};
 # fn main() {
 let user = "ferris";

 span!(Level::TRACE, "login", user);
 // is equivalent to:
 span!(Level::TRACE, "login", user = user);
 # }
```

 Field names can include dots, but should not be terminated by them:
 ```
 # use tracing::{span, Level};
 # fn main() {
 let user = "ferris";
 let email = "ferris@rust-lang.org";
 span!(Level::TRACE, "login", user, user.email = email);
 # }
```

 Since field names can include dots, fields on local structs can be used
 using the local variable shorthand:
 ```
 # use tracing::{span, Level};
 # fn main() {
 # struct User {
 #    name: &'static str,
 #    email: &'static str,
 # }
 let user = User {
     name: "ferris",
     email: "ferris@rust-lang.org",
 };
 // the span will have the fields `user.name = "ferris"` and
 // `user.email = "ferris@rust-lang.org"`.
 span!(Level::TRACE, "login", user.name, user.email);
 # }
```

 Fields with names that are not Rust identifiers, or with names that are Rust reserved words,
 may be created using quoted string literals. However, this may not be used with the local
 variable shorthand.
 ```
 # use tracing::{span, Level};
 # fn main() {
 // records an event with fields whose names are not Rust identifiers
 //  - "guid:x-request-id", containing a `:`, with the value "abcdef"
 //  - "type", which is a reserved word, with the value "request"
 span!(Level::TRACE, "api", "guid:x-request-id" = "abcdef", "type" = "request");
 # }
```

 Constant expressions can also be used as field names. Constants
 must be enclosed in curly braces (`{}`) to indicate that the *value*
 of the constant is to be used as the field name, rather than the
 constant's name. For example:
 ```
 # use tracing::{span, Level};
 # fn main() {
 const RESOURCE_NAME: &str = "foo";
 // this span will have the field `foo = "some_id"`
 span!(Level::TRACE, "get", { RESOURCE_NAME } = "some_id");
 # }
```

 The `?` sigil is shorthand that specifies a field should be recorded using
 its [`fmt::Debug`] implementation:
 ```
 # use tracing::{event, Level};
 # fn main() {
 #[derive(Debug)]
 struct MyStruct {
     field: &'static str,
 }

 let my_struct = MyStruct {
     field: "Hello world!"
 };

 // `my_struct` will be recorded using its `fmt::Debug` implementation.
 event!(Level::TRACE, greeting = ?my_struct);
 // is equivalent to:
 event!(Level::TRACE, greeting = tracing::field::debug(&my_struct));
 # }
 ```

 The `%` sigil operates similarly, but indicates that the value should be
 recorded using its [`fmt::Display`] implementation:
 ```
 # use tracing::{event, Level};
 # fn main() {
 # #[derive(Debug)]
 # struct MyStruct {
 #     field: &'static str,
 # }
 #
 # let my_struct = MyStruct {
 #     field: "Hello world!"
 # };
 // `my_struct.field` will be recorded using its `fmt::Display` implementation.
 event!(Level::TRACE, greeting = %my_struct.field);
 // is equivalent to:
 event!(Level::TRACE, greeting = tracing::field::display(&my_struct.field));
 # }
 ```

 The `%` and `?` sigils may also be used with local variable shorthand:

 ```
 # use tracing::{event, Level};
 # fn main() {
 # #[derive(Debug)]
 # struct MyStruct {
 #     field: &'static str,
 # }
 #
 # let my_struct = MyStruct {
 #     field: "Hello world!"
 # };
 // `my_struct.field` will be recorded using its `fmt::Display` implementation.
 event!(Level::TRACE, %my_struct.field);
 # }
 ```

 Additionally, a span may declare fields with the special value [`Empty`],
 which indicates that that the value for that field does not currently exist
 but may be recorded later. For example:

 ```
 use tracing::{trace_span, field};

 // Create a span with two fields: `greeting`, with the value "hello world", and
 // `parting`, without a value.
 let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);

 // ...

 // Now, record a value for parting as well.
 span.record("parting", &"goodbye world!");
 ```

 Finally, events may also include human-readable messages, in the form of a
 [format string][fmt] and (optional) arguments, **after** the event's
 key-value fields. If a format string and arguments are provided,
 they will implicitly create a new field named `message` whose value is the
 provided set of format arguments.

 For example:

 ```
 # use tracing::{event, Level};
 # fn main() {
 let question = "the ultimate question of life, the universe, and everything";
 let answer = 42;
 // records an event with the following fields:
 // - `question.answer` with the value 42,
 // - `question.tricky` with the value `true`,
 // - "message", with the value "the answer to the ultimate question of life, the
 //    universe, and everything is 42."
 event!(
     Level::DEBUG,
     question.answer = answer,
     question.tricky = true,
     "the answer to {} is {}.", question, answer
 );
 # }
 ```

 Specifying a formatted message in this manner does not allocate by default.

 [struct initializers]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name
 [target]: Metadata::target
 [parent span]: span::Attributes::parent
 [determined contextually]: span::Attributes::is_contextual
 [`fmt::Debug`]: std::fmt::Debug
 [`fmt::Display`]: std::fmt::Display
 [fmt]: std::fmt#usage
 [`Empty`]: field::Empty

 ### Shorthand Macros

 `tracing` also offers a number of macros with preset verbosity levels.
 The [`trace!`], [`debug!`], [`info!`], [`warn!`], and [`error!`] behave
 similarly to the [`event!`] macro, but with the [`Level`] argument already
 specified, while the corresponding [`trace_span!`], [`debug_span!`],
 [`info_span!`], [`warn_span!`], and [`error_span!`] macros are the same,
 but for the [`span!`] macro.

 These are intended both as a shorthand, and for compatibility with the [`log`]
 crate (see the next section).

 [`span!`]: span!
 [`event!`]: event!
 [`trace!`]: trace!
 [`debug!`]: debug!
 [`info!`]: info!
 [`warn!`]: warn!
 [`error!`]: error!
 [`trace_span!`]: trace_span!
 [`debug_span!`]: debug_span!
 [`info_span!`]: info_span!
 [`warn_span!`]: warn_span!
 [`error_span!`]: error_span!

 ### For `log` Users

 Users of the [`log`] crate should note that `tracing` exposes a set of
 macros for creating `Event`s (`trace!`, `debug!`, `info!`, `warn!`, and
 `error!`) which may be invoked with the same syntax as the similarly-named
 macros from the `log` crate. Often, the process of converting a project to
 use `tracing` can begin with a simple drop-in replacement.

 Let's consider the `log` crate's yak-shaving example:

 ```rust,ignore
 use std::{error::Error, io};
 use tracing::{debug, error, info, span, warn, Level};

 // the `#[tracing::instrument]` attribute creates and enters a span
 // every time the instrumented function is called. The span is named after the
 // the function or method. Parameters passed to the function are recorded as fields.
 #[tracing::instrument]
 pub fn shave(yak: usize) -> Result<(), Box<dyn Error + 'static>> {
     // this creates an event at the DEBUG level with two fields:
     // - `excitement`, with the key "excitement" and the value "yay!"
     // - `message`, with the key "message" and the value "hello! I'm gonna shave a yak."
     //
     // unlike other fields, `message`'s shorthand initialization is just the string itself.
     debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");
     if yak == 3 {
         warn!("could not locate yak!");
         // note that this is intended to demonstrate `tracing`'s features, not idiomatic
         // error handling! in a library or application, you should consider returning
         // a dedicated `YakError`. libraries like snafu or thiserror make this easy.
         return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
     } else {
         debug!("yak shaved successfully");
     }
     Ok(())
 }

 pub fn shave_all(yaks: usize) -> usize {
     // Constructs a new span named "shaving_yaks" at the TRACE level,
     // and a field whose key is "yaks". This is equivalent to writing:
     //
     // let span = span!(Level::TRACE, "shaving_yaks", yaks = yaks);
     //
     // local variables (`yaks`) can be used as field values
     // without an assignment, similar to struct initializers.
     let _span = span!(Level::TRACE, "shaving_yaks", yaks).entered();

     info!("shaving yaks");

     let mut yaks_shaved = 0;
     for yak in 1..=yaks {
         let res = shave(yak);
         debug!(yak, shaved = res.is_ok());

         if let Err(ref error) = res {
             // Like spans, events can also use the field initialization shorthand.
             // In this instance, `yak` is the field being initalized.
             error!(yak, error = error.as_ref(), "failed to shave yak!");
         } else {
             yaks_shaved += 1;
         }
         debug!(yaks_shaved);
     }

     yaks_shaved
 }
 ```

 ## In libraries

 Libraries should link only to the `tracing` crate, and use the provided
 macros to record whatever information will be useful to downstream
 consumers.

 ## In executables

 In order to record trace events, executables have to use a `Subscriber`
 implementation compatible with `tracing`. A `Subscriber` implements a
 way of collecting trace data, such as by logging it to standard output.

 This library does not contain any `Subscriber` implementations; these are
 provided by [other crates](#related-crates).

 The simplest way to use a subscriber is to call the [`set_global_default`]
 function:

 ```
 extern crate tracing;
 # pub struct FooSubscriber;
 # use tracing::{span::{Id, Attributes, Record}, Metadata};
 # impl tracing::Subscriber for FooSubscriber {
 #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
 #   fn record(&self, _: &Id, _: &Record) {}
 #   fn event(&self, _: &tracing::Event) {}
 #   fn record_follows_from(&self, _: &Id, _: &Id) {}
 #   fn enabled(&self, _: &Metadata) -> bool { false }
 #   fn enter(&self, _: &Id) {}
 #   fn exit(&self, _: &Id) {}
 # }
 # impl FooSubscriber {
 #   fn new() -> Self { FooSubscriber }
 # }
 # fn main() {

 let my_subscriber = FooSubscriber::new();
 tracing::subscriber::set_global_default(my_subscriber)
     .expect("setting tracing default failed");
 # }
 ```

 <pre class="compile_fail" style="white-space:normal;font:inherit;">
     <strong>Warning</strong>: In general, libraries should <em>not</em> call
     <code>set_global_default()</code>! Doing so will cause conflicts when
     executables that depend on the library try to set the default later.
 </pre>

 This subscriber will be used as the default in all threads for the
 remainder of the duration of the program, similar to setting the logger
 in the `log` crate.

 In addition, the default subscriber can be set through using the
 [`with_default`] function. This follows the `tokio` pattern of using
 closures to represent executing code in a context that is exited at the end
 of the closure. For example:

 ```rust
 # pub struct FooSubscriber;
 # use tracing::{span::{Id, Attributes, Record}, Metadata};
 # impl tracing::Subscriber for FooSubscriber {
 #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
 #   fn record(&self, _: &Id, _: &Record) {}
 #   fn event(&self, _: &tracing::Event) {}
 #   fn record_follows_from(&self, _: &Id, _: &Id) {}
 #   fn enabled(&self, _: &Metadata) -> bool { false }
 #   fn enter(&self, _: &Id) {}
 #   fn exit(&self, _: &Id) {}
 # }
 # impl FooSubscriber {
 #   fn new() -> Self { FooSubscriber }
 # }
 # fn main() {

 let my_subscriber = FooSubscriber::new();
 # #[cfg(feature = "std")]
 tracing::subscriber::with_default(my_subscriber, || {
     // Any trace events generated in this closure or by functions it calls
     // will be collected by `my_subscriber`.
 })
 # }
 ```

 This approach allows trace data to be collected by multiple subscribers
 within different contexts in the program. Note that the override only applies to the
 currently executing thread; other threads will not see the change from with_default.

 Any trace events generated outside the context of a subscriber will not be collected.

 Once a subscriber has been set, instrumentation points may be added to the
 executable using the `tracing` crate's macros.

 ## `log` Compatibility

 The [`log`] crate provides a simple, lightweight logging facade for Rust.
 While `tracing` builds upon `log`'s foundation with richer structured
 diagnostic data, `log`'s simplicity and ubiquity make it the "lowest common
 denominator" for text-based logging in Rust — a vast majority of Rust
 libraries and applications either emit or consume `log` records. Therefore,
 `tracing` provides multiple forms of interoperability with `log`: `tracing`
 instrumentation can emit `log` records, and a compatibility layer enables
 `tracing` [`Subscriber`]s to consume `log` records as `tracing` [`Event`]s.

 ### Emitting `log` Records

 This crate provides two feature flags, "log" and "log-always", which will
 cause [spans] and [events] to emit `log` records. When the "log" feature is
 enabled, if no `tracing` `Subscriber` is active, invoking an event macro or
 creating a span with fields will emit a `log` record. This is intended
 primarily for use in libraries which wish to emit diagnostics that can be
 consumed by applications using `tracing` *or* `log`, without paying the
 additional overhead of emitting both forms of diagnostics when `tracing` is
 in use.

 Enabling the "log-always" feature will cause `log` records to be emitted
 even if a `tracing` `Subscriber` _is_ set. This is intended to be used in
 applications where a `log` `Logger` is being used to record a textual log,
 and `tracing` is used only to record other forms of diagnostics (such as
 metrics, profiling, or distributed tracing data). Unlike the "log" feature,
 libraries generally should **not** enable the "log-always" feature, as doing
 so will prevent applications from being able to opt out of the `log` records.

 See [here][flags] for more details on this crate's feature flags.

 The generated `log` records' messages will be a string representation of the
 span or event's fields, and all additional information recorded by `log`
 (target, verbosity level, module path, file, and line number) will also be
 populated. Additionally, `log` records are also generated when spans are
 entered, exited, and closed. Since these additional span lifecycle logs have
 the potential to be very verbose, and don't include additional fields, they
 will always be emitted at the `Trace` level, rather than inheriting the
 level of the span that generated them. Furthermore, they are are categorized
 under a separate `log` target, "tracing::span" (and its sub-target,
 "tracing::span::active", for the logs on entering and exiting a span), which
 may be enabled or disabled separately from other `log` records emitted by
 `tracing`.

 ### Consuming `log` Records

 The [`tracing-log`] crate provides a compatibility layer which
 allows a `tracing` [`Subscriber`] to consume `log` records as though they
 were `tracing` [events]. This allows applications using `tracing` to record
 the logs emitted by dependencies using `log` as events within the context of
 the application's trace tree. See [that crate's documentation][log-tracer]
 for details.

 [log-tracer]: https://docs.rs/tracing-log/latest/tracing_log/#convert-log-records-to-tracing-events

 ## Related Crates

 In addition to `tracing` and `tracing-core`, the [`tokio-rs/tracing`] repository
 contains several additional crates designed to be used with the `tracing` ecosystem.
 This includes a collection of `Subscriber` implementations, as well as utility
 and adapter crates to assist in writing `Subscriber`s and instrumenting
 applications.

 In particular, the following crates are likely to be of interest:

  - [`tracing-futures`] provides a compatibility layer with the `futures`
    crate, allowing spans to be attached to `Future`s, `Stream`s, and `Executor`s.
  - [`tracing-subscriber`] provides `Subscriber` implementations and
    utilities for working with `Subscriber`s. This includes a [`FmtSubscriber`]
    `FmtSubscriber` for logging formatted trace data to stdout, with similar
    filtering and formatting to the [`env_logger`] crate.
  - [`tracing-log`] provides a compatibility layer with the [`log`] crate,
    allowing log messages to be recorded as `tracing` `Event`s within the
    trace tree. This is useful when a project using `tracing` have
    dependencies which use `log`. Note that if you're using
    `tracing-subscriber`'s `FmtSubscriber`, you don't need to depend on
    `tracing-log` directly.
  - [`tracing-appender`] provides utilities for outputting tracing data,
     including a file appender and non blocking writer.

 Additionally, there are also several third-party crates which are not
 maintained by the `tokio` project. These include:

  - [`tracing-timing`] implements inter-event timing metrics on top of `tracing`.
    It provides a subscriber that records the time elapsed between pairs of
    `tracing` events and generates histograms.
  - [`tracing-opentelemetry`] provides a subscriber for emitting traces to
    [OpenTelemetry]-compatible distributed tracing systems.
  - [`tracing-honeycomb`] Provides a layer that reports traces spanning multiple machines to [honeycomb.io]. Backed by [`tracing-distributed`].
  - [`tracing-distributed`] Provides a generic implementation of a layer that reports traces spanning multiple machines to some backend.
  - [`tracing-actix-web`] provides `tracing` integration for the `actix-web` web framework.
  - [`tracing-actix`] provides `tracing` integration for the `actix` actor
    framework.
  - [`axum-insights`] provides `tracing` integration and Application insights export for the `axum` web framework.
  - [`tracing-gelf`] implements a subscriber for exporting traces in Greylog
    GELF format.
  - [`tracing-coz`] provides integration with the [coz] causal profiler
    (Linux-only).
  - [`tracing-bunyan-formatter`] provides a layer implementation that reports events and spans
    in [bunyan] format, enriched with timing information.
  - [`tracing-wasm`] provides a `Subscriber`/`Layer` implementation that reports
    events and spans via browser `console.log` and [User Timing API (`window.performance`)].
  - [`tracing-web`] provides a layer implementation of level-aware logging of events
    to web browsers' `console.*` and span events to the [User Timing API (`window.performance`)].
  - [`tide-tracing`] provides a [tide] middleware to trace all incoming requests and responses.
  - [`test-log`] takes care of initializing `tracing` for tests, based on
    environment variables with an `env_logger` compatible syntax.
  - [`tracing-unwrap`] provides convenience methods to report failed unwraps
    on `Result` or `Option` types to a `Subscriber`.
  - [`diesel-tracing`] provides integration with [`diesel`] database connections.
  - [`tracing-tracy`] provides a way to collect [Tracy] profiles in instrumented
    applications.
  - [`tracing-elastic-apm`] provides a layer for reporting traces to [Elastic APM].
  - [`tracing-etw`] provides a layer for emitting Windows [ETW] events.
  - [`tracing-fluent-assertions`] provides a fluent assertions-style testing
    framework for validating the behavior of `tracing` spans.
  - [`sentry-tracing`] provides a layer for reporting events and traces to [Sentry].
  - [`tracing-forest`] provides a subscriber that preserves contextual coherence by
    grouping together logs from the same spans during writing.
  - [`tracing-loki`] provides a layer for shipping logs to [Grafana Loki].
  - [`tracing-logfmt`] provides a layer that formats events and spans into the logfmt format.
  - [`reqwest-tracing`] provides a middleware to trace [`reqwest`] HTTP requests.
  - [`tracing-cloudwatch`] provides a layer that sends events to AWS CloudWatch Logs.
  - [`clippy-tracing`] provides a tool to add, remove and check for `tracing::instrument`.
  - [`json-subscriber`] provides a subscriber for emitting JSON logs. The output can be customized much more than with [`tracing-subscriber`]'s JSON output.

 If you're the maintainer of a `tracing` ecosystem crate not listed above,
 please let us know! We'd love to add your project to the list!

 [`tracing-opentelemetry`]: https://crates.io/crates/tracing-opentelemetry
 [OpenTelemetry]: https://opentelemetry.io/
 [`tracing-honeycomb`]: https://crates.io/crates/tracing-honeycomb
 [`tracing-distributed`]: https://crates.io/crates/tracing-distributed
 [honeycomb.io]: https://www.honeycomb.io/
 [`tracing-actix-web`]: https://crates.io/crates/tracing-actix-web
 [`tracing-actix`]: https://crates.io/crates/tracing-actix
 [`axum-insights`]: https://crates.io/crates/axum-insights
 [`tracing-gelf`]: https://crates.io/crates/tracing-gelf
 [`tracing-coz`]: https://crates.io/crates/tracing-coz
 [coz]: https://github.com/plasma-umass/coz
 [`tracing-bunyan-formatter`]: https://crates.io/crates/tracing-bunyan-formatter
 [bunyan]: https://github.com/trentm/node-bunyan
 [`tracing-wasm`]: https://docs.rs/tracing-wasm
 [`tracing-web`]: https://docs.rs/tracing-web
 [User Timing API (`window.performance`)]: https://developer.mozilla.org/en-US/docs/Web/API/User_Timing_API
 [`tide-tracing`]: https://crates.io/crates/tide-tracing
 [tide]: https://crates.io/crates/tide
 [`test-log`]: https://crates.io/crates/test-log
 [`tracing-unwrap`]: https://docs.rs/tracing-unwrap
 [`diesel`]: https://crates.io/crates/diesel
 [`diesel-tracing`]: https://crates.io/crates/diesel-tracing
 [`tracing-tracy`]: https://crates.io/crates/tracing-tracy
 [Tracy]: https://github.com/wolfpld/tracy
 [`tracing-elastic-apm`]: https://crates.io/crates/tracing-elastic-apm
 [Elastic APM]: https://www.elastic.co/apm
 [`tracing-etw`]: https://github.com/microsoft/rust_win_etw/tree/main/win_etw_tracing
 [ETW]: https://docs.microsoft.com/en-us/windows/win32/etw/about-event-tracing
 [`tracing-fluent-assertions`]: https://crates.io/crates/tracing-fluent-assertions
 [`sentry-tracing`]: https://crates.io/crates/sentry-tracing
 [Sentry]: https://sentry.io/welcome/
 [`tracing-forest`]: https://crates.io/crates/tracing-forest
 [`tracing-loki`]: https://crates.io/crates/tracing-loki
 [Grafana Loki]: https://grafana.com/oss/loki/
 [`tracing-logfmt`]: https://crates.io/crates/tracing-logfmt
 [`reqwest-tracing`]: https://crates.io/crates/reqwest-tracing
 [`reqwest`]: https://crates.io/crates/reqwest
 [`tracing-cloudwatch`]: https://crates.io/crates/tracing-cloudwatch
 [`clippy-tracing`]: https://crates.io/crates/clippy-tracing
 [`json-subscriber`]: https://crates.io/crates/json-subscriber

 <pre class="ignore" style="white-space:normal;font:inherit;">
     <strong>Note</strong>: Some of these ecosystem crates are currently
     unreleased and/or in earlier stages of development. They may be less stable
     than <code>tracing</code> and <code>tracing-core</code>.
 </pre>

 ## Crate Feature Flags

 The following crate [feature flags] are available:

 * A set of features controlling the [static verbosity level].
 * `log`: causes trace instrumentation points to emit [`log`] records as well
   as trace events, if a default `tracing` subscriber has not been set. This
   is intended for use in libraries whose users may be using either `tracing`
   or `log`.
 * `log-always`: Emit `log` records from all `tracing` spans and events, even
   if a `tracing` subscriber has been set. This should be set only by
   applications which intend to collect traces and logs separately; if an
   adapter is used to convert `log` records into `tracing` events, this will
   cause duplicate events to occur.
 * `attributes`: Includes support for the `#[instrument]` attribute.
   This is on by default, but does bring in the `syn` crate as a dependency,
   which may add to the compile time of crates that do not already use it.
 * `std`: Depend on the Rust standard library (enabled by default).

   `no_std` users may disable this feature with `default-features = false`:

   ```toml
   [dependencies]
   tracing = { version = "0.1.38", default-features = false }
   ```

 <pre class="ignore" style="white-space:normal;font:inherit;">
     <strong>Note</strong>: <code>tracing</code>'s <code>no_std</code> support
     requires <code>liballoc</code>.
 </pre>

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

 [`log`]: https://docs.rs/log/0.4.6/log/
 [span]: mod@span
 [spans]: mod@span
 [`Span`]: span::Span
 [`in_scope`]: span::Span::in_scope
 [event]: Event
 [events]: Event
 [`Subscriber`]: subscriber::Subscriber
 [Subscriber::event]: subscriber::Subscriber::event
 [`enter`]: subscriber::Subscriber::enter
 [`exit`]: subscriber::Subscriber::exit
 [`enabled`]: subscriber::Subscriber::enabled
 [metadata]: Metadata
 [`field::display`]: field::display
 [`field::debug`]: field::debug
 [`set_global_default`]: subscriber::set_global_default
 [`with_default`]: subscriber::with_default
 [`tokio-rs/tracing`]: https://github.com/tokio-rs/tracing
 [`tracing-futures`]: https://crates.io/crates/tracing-futures
 [`tracing-subscriber`]: https://crates.io/crates/tracing-subscriber
 [`tracing-log`]: https://crates.io/crates/tracing-log
 [`tracing-timing`]: https://crates.io/crates/tracing-timing
 [`tracing-appender`]: https://crates.io/crates/tracing-appender
 [`env_logger`]: https://crates.io/crates/env_logger
 [`FmtSubscriber`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Subscriber.html
 [static verbosity level]: level_filters#compile-time-filters
 [instrument]: https://docs.rs/tracing-attributes/latest/tracing_attributes/attr.instrument.html
 [flags]: #crate-feature-flags

## Modules

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
<strong>Note</strong>: The thread-local scoped dispatcher (<code>with_default</code>)
requires the Rust standard library. <code>no_std</code> users should
use <a href="fn.set_global_default.html"><code>set_global_default</code></a>
instead.
</pre>

## Accessing the Default Subscriber

A thread's current default subscriber can be accessed using the
[`get_default`] function, which executes a closure with a reference to the
currently default `Dispatch`. This is used primarily by `tracing`
instrumentation.

[`Subscriber`]: crate::Subscriber

```rust
pub mod dispatcher { /* ... */ }
```

### Re-exports

#### Re-export `set_default`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use tracing_core::dispatcher::set_default;
```

#### Re-export `with_default`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use tracing_core::dispatcher::with_default;
```

#### Re-export `DefaultGuard`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use tracing_core::dispatcher::DefaultGuard;
```

#### Re-export `get_default`

```rust
pub use tracing_core::dispatcher::get_default;
```

#### Re-export `set_global_default`

```rust
pub use tracing_core::dispatcher::set_global_default;
```

#### Re-export `Dispatch`

```rust
pub use tracing_core::dispatcher::Dispatch;
```

#### Re-export `SetGlobalDefaultError`

```rust
pub use tracing_core::dispatcher::SetGlobalDefaultError;
```

#### Re-export `WeakDispatch`

```rust
pub use tracing_core::dispatcher::WeakDispatch;
```

## Module `field`

`Span` and `Event` key-value data.

Spans and events may be annotated with key-value data, referred to as _fields_.
These fields consist of a mapping from a key (corresponding to
a `&str` but represented internally as an array index) to a [`Value`].

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
(`i64`, `u64`, `f64`, `bool`, and `&str`) or using a `fmt::Display` or
`fmt::Debug` implementation. `Subscriber`s are provided these primitive
value types as `dyn Value` trait objects.

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

[`fmt::Debug`]: std::fmt::Debug
[`fmt::Display`]: std::fmt::Debug
[`valuable`]: https://crates.io/crates/valuable
[`valuable::Valuable`]: https://docs.rs/valuable/latest/valuable/trait.Valuable.html
[`as_value`]: https://docs.rs/valuable/latest/valuable/trait.Valuable.html#tymethod.as_value
[`valuable::Value`]: https://docs.rs/valuable/latest/valuable/enum.Value.html
[`Subscriber`]: crate::Subscriber
[`record_value`]: Visit::record_value
[`record_debug`]: Visit::record_debug
[span]: mod@crate::span
[`Event`]: crate::event::Event
[`Metadata`]: crate::Metadata
[`Attributes`]: crate::span::Attributes
[`Record`]: crate::span::Record
[`new_span`]: crate::Subscriber::new_span
[`record`]: crate::Subscriber::record
[`event`]: crate::Subscriber::event

```rust
pub mod field { /* ... */ }
```

### Traits

#### Trait `AsField`

Trait implemented to allow a type to be used as a field key.

<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: Although this is implemented for both the
<a href="./struct.Field.html"><code>Field</code></a> type <em>and</em> any
type that can be borrowed as an <code>&str</code>, only <code>Field</code>
allows <em>O</em>(1) access.
Indexing a field with a string results in an iterative search that performs
string comparisons. Thus, if possible, once the key for a field is known, it
should be used whenever possible.
</pre>

```rust
pub trait AsField: crate::sealed::Sealed {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_field`: Attempts to convert `&self` into a `Field` with the specified `metadata`.

##### Implementations

This trait is implemented for the following types:

- `Field`
- `&''a Field` with <''a>
- `str`

### Re-exports

#### Re-export `tracing_core::field::*`

```rust
pub use tracing_core::field::*;
```

## Module `instrument`

Attach a span to a `std::future::Future`.

```rust
pub mod instrument { /* ... */ }
```

### Types

#### Struct `WithDispatch`

**Attributes:**

- `#[must_use = "futures do nothing unless you `.await` or poll them"]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

A [`Future`] that has been instrumented with a `tracing` [`Subscriber`].

This type is returned by the [`WithSubscriber`] extension trait. See that
trait's documentation for details.

[`Future`]: std::future::Future
[`Subscriber`]: crate::Subscriber

```rust
pub struct WithDispatch<T> {
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
  pub fn dispatcher(self: &Self) -> &Dispatch { /* ... */ }
  ```
  Borrows the [`Dispatch`] that is entered when this type is polled.

- ```rust
  pub fn inner(self: &Self) -> &T { /* ... */ }
  ```
  Borrows the wrapped type.

- ```rust
  pub fn inner_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Mutably borrows the wrapped type.

- ```rust
  pub fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T> { /* ... */ }
  ```
  Get a pinned reference to the wrapped type.

- ```rust
  pub fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> { /* ... */ }
  ```
  Get a pinned mutable reference to the wrapped type.

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Consumes the `Instrumented`, returning the wrapped type.

###### Trait Implementations

- **UnwindSafe**
- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **Freeze**
- **Instrument**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> WithDispatch<T> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Instrumented`

**Attributes:**

- `#[must_use = "futures do nothing unless you `.await` or poll them"]`

A [`Future`] that has been instrumented with a `tracing` [`Span`].

This type is returned by the [`Instrument`] extension trait. See that
trait's documentation for details.

[`Future`]: std::future::Future
[`Span`]: crate::Span

```rust
pub struct Instrumented<T> {
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
  pub fn span(self: &Self) -> &Span { /* ... */ }
  ```
  Borrows the `Span` that this type is instrumented by.

- ```rust
  pub fn span_mut(self: &mut Self) -> &mut Span { /* ... */ }
  ```
  Mutably borrows the `Span` that this type is instrumented by.

- ```rust
  pub fn inner(self: &Self) -> &T { /* ... */ }
  ```
  Borrows the wrapped type.

- ```rust
  pub fn inner_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Mutably borrows the wrapped type.

- ```rust
  pub fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T> { /* ... */ }
  ```
  Get a pinned reference to the wrapped type.

- ```rust
  pub fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> { /* ... */ }
  ```
  Get a pinned mutable reference to the wrapped type.

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Consumes the `Instrumented`, returning the wrapped type.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

- **Sync**
- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Instrumented<T> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `Instrument`

Attaches spans to a [`std::future::Future`].

Extension trait allowing futures to be
instrumented with a `tracing` [span].

[span]: super::Span

```rust
pub trait Instrument: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn instrument(self: Self, span: Span) -> Instrumented<Self> { /* ... */ }
  ```
  Instruments this type with the provided [`Span`], returning an

- ```rust
  fn in_current_span(self: Self) -> Instrumented<Self> { /* ... */ }
  ```
  Instruments this type with the [current] [`Span`], returning an

##### Implementations

This trait is implemented for the following types:

- `T` with <T: Sized>

#### Trait `WithSubscriber`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Extension trait allowing futures to be instrumented with
a `tracing` [`Subscriber`](crate::Subscriber).

```rust
pub trait WithSubscriber: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn with_subscriber<S>(self: Self, subscriber: S) -> WithDispatch<Self>
where
    S: Into<Dispatch> { /* ... */ }
  ```
  Attaches the provided [`Subscriber`] to this type, returning a

- ```rust
  fn with_current_subscriber(self: Self) -> WithDispatch<Self> { /* ... */ }
  ```
  Attaches the current [default] [`Subscriber`] to this type, returning a

##### Implementations

This trait is implemented for the following types:

- `T` with <T: Sized>

## Module `level_filters`

Trace verbosity level filtering.

# Compile time filters

Trace verbosity levels can be statically disabled at compile time via Cargo
features, similar to the [`log` crate]. Trace instrumentation at disabled
levels will be skipped and will not even be present in the resulting binary
unless the verbosity level is specified dynamically. This level is
configured separately for release and debug builds. The features are:

* `max_level_off`
* `max_level_error`
* `max_level_warn`
* `max_level_info`
* `max_level_debug`
* `max_level_trace`
* `release_max_level_off`
* `release_max_level_error`
* `release_max_level_warn`
* `release_max_level_info`
* `release_max_level_debug`
* `release_max_level_trace`

These features control the value of the `STATIC_MAX_LEVEL` constant. The
instrumentation macros macros check this value before recording an event or
constructing a span. By default, no levels are disabled.

For example, a crate can disable trace level instrumentation in debug builds
and trace, debug, and info level instrumentation in release builds with the
following configuration:

```toml
[dependencies]
tracing = { version = "0.1", features = ["max_level_debug", "release_max_level_warn"] }
```
## Notes

Please note that `tracing`'s static max level features do *not* control the
[`log`] records that may be emitted when [`tracing`'s "log" feature flag][f] is
enabled. This is to allow `tracing` to be disabled entirely at compile time
while still emitting `log` records --- such as when a library using
`tracing` is used by an application using `log` that doesn't want to
generate any `tracing`-related code, but does want to collect `log` records.

This means that if the "log" feature is in use, some code may be generated
for `log` records emitted by disabled `tracing` events. If this is not
desirable, `log` records may be disabled separately using [`log`'s static
max level features][`log` crate].

[`log`]: https://docs.rs/log/
[`log` crate]: https://docs.rs/log/latest/log/#compile-time-filters
[f]: https://docs.rs/tracing/latest/tracing/#emitting-log-records

```rust
pub mod level_filters { /* ... */ }
```

### Constants and Statics

#### Constant `STATIC_MAX_LEVEL`

The statically configured maximum trace level.

See the [module-level documentation] for information on how to configure
this.

This value is checked by the `event!` and `span!` macros. Code that
manually constructs events or spans via the `Event::record` function or
`Span` constructors should compare the level against this value to
determine if those spans or events are enabled.

[module-level documentation]: self#compile-time-filters

```rust
pub const STATIC_MAX_LEVEL: LevelFilter = _;
```

### Re-exports

#### Re-export `ParseLevelFilterError`

```rust
pub use tracing_core::metadata::ParseLevelFilterError;
```

#### Re-export `LevelFilter`

```rust
pub use tracing_core::LevelFilter;
```

## Module `span`

 Spans represent periods of time in which a program was executing in a
 particular context.

 A span consists of [fields], user-defined key-value pairs of arbitrary data
 that describe the context the span represents, and a set of fixed attributes
 that describe all `tracing` spans and events. Attributes describing spans
 include:

 - An [`Id`] assigned by the subscriber that uniquely identifies it in relation
   to other spans.
 - The span's [parent] in the trace tree.
 - [Metadata] that describes static characteristics of all spans
   originating from that callsite, such as its name, source code location,
   [verbosity level], and the names of its fields.

 # Creating Spans

 Spans are created using the [`span!`] macro. This macro is invoked with the
 following arguments, in order:

 - The [`target`] and/or [`parent`][parent] attributes, if the user wishes to
   override their default values.
 - The span's [verbosity level]
 - A string literal providing the span's name.
 - Finally, zero or more arbitrary key/value fields.

 [`target`]: super::Metadata::target

 For example:
 ```rust
 use tracing::{span, Level};

 /// Construct a new span at the `INFO` level named "my_span", with a single
 /// field named answer , with the value `42`.
 let my_span = span!(Level::INFO, "my_span", answer = 42);
 ```

 The documentation for the [`span!`] macro provides additional examples of
 the various options that exist when creating spans.

 The [`trace_span!`], [`debug_span!`], [`info_span!`], [`warn_span!`], and
 [`error_span!`] exist as shorthand for constructing spans at various
 verbosity levels.

 ## Recording Span Creation

 The [`Attributes`] type contains data associated with a span, and is
 provided to the [`Subscriber`] when a new span is created. It contains
 the span's metadata, the ID of [the span's parent][parent] if one was
 explicitly set, and any fields whose values were recorded when the span was
 constructed. The subscriber, which is responsible for recording `tracing`
 data, can then store or record these values.

 # The Span Lifecycle

 ## Entering a Span

 A thread of execution is said to _enter_ a span when it begins executing,
 and _exit_ the span when it switches to another context. Spans may be
 entered through the [`enter`], [`entered`], and [`in_scope`] methods.

 The [`enter`] method enters a span, returning a [guard] that exits the span
 when dropped
 ```
 # use tracing::{span, Level};
 let my_var: u64 = 5;
 let my_span = span!(Level::TRACE, "my_span", my_var);

 // `my_span` exists but has not been entered.

 // Enter `my_span`...
 let _enter = my_span.enter();

 // Perform some work inside of the context of `my_span`...
 // Dropping the `_enter` guard will exit the span.
```

 <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
     <strong>Warning</strong>: In asynchronous code that uses async/await syntax,
     <code>Span::enter</code> may produce incorrect traces if the returned drop
     guard is held across an await point. See
     <a href="struct.Span.html#in-asynchronous-code">the method documentation</a>
     for details.
 </pre></div>

 The [`entered`] method is analogous to [`enter`], but moves the span into
 the returned guard, rather than borrowing it. This allows creating and
 entering a span in a single expression:

 ```
 # use tracing::{span, Level};
 // Create a span and enter it, returning a guard:
 let span = span!(Level::INFO, "my_span").entered();

 // We are now inside the span! Like `enter()`, the guard returned by
 // `entered()` will exit the span when it is dropped...

 // ...but, it can also be exited explicitly, returning the `Span`
 // struct:
 let span = span.exit();
 ```

 Finally, [`in_scope`] takes a closure or function pointer and executes it
 inside the span:

 ```
 # use tracing::{span, Level};
 let my_var: u64 = 5;
 let my_span = span!(Level::TRACE, "my_span", my_var = &my_var);

 my_span.in_scope(|| {
     // perform some work in the context of `my_span`...
 });

 // Perform some work outside of the context of `my_span`...

 my_span.in_scope(|| {
     // Perform some more work in the context of `my_span`.
 });
 ```

 <pre class="ignore" style="white-space:normal;font:inherit;">
     <strong>Note</strong>: Since entering a span takes <code>&self</code>, and
     <code>Span</code>s are <code>Clone</code>, <code>Send</code>, and
     <code>Sync</code>, it is entirely valid for multiple threads to enter the
     same span concurrently.
 </pre>

 ## Span Relationships

 Spans form a tree structure — unless it is a root span, all spans have a
 _parent_, and may have one or more _children_. When a new span is created,
 the current span becomes the new span's parent. The total execution time of
 a span consists of the time spent in that span and in the entire subtree
 represented by its children. Thus, a parent span always lasts for at least
 as long as the longest-executing span in its subtree.

 ```
 # use tracing::{Level, span};
 // this span is considered the "root" of a new trace tree:
 span!(Level::INFO, "root").in_scope(|| {
     // since we are now inside "root", this span is considered a child
     // of "root":
     span!(Level::DEBUG, "outer_child").in_scope(|| {
         // this span is a child of "outer_child", which is in turn a
         // child of "root":
         span!(Level::TRACE, "inner_child").in_scope(|| {
             // and so on...
         });
     });
     // another span created here would also be a child of "root".
 });
```

 In addition, the parent of a span may be explicitly specified in
 the `span!` macro. For example:

 ```rust
 # use tracing::{Level, span};
 // Create, but do not enter, a span called "foo".
 let foo = span!(Level::INFO, "foo");

 // Create and enter a span called "bar".
 let bar = span!(Level::INFO, "bar");
 let _enter = bar.enter();

 // Although we have currently entered "bar", "baz"'s parent span
 // will be "foo".
 let baz = span!(parent: &foo, Level::INFO, "baz");
 ```

 A child span should typically be considered _part_ of its parent. For
 example, if a subscriber is recording the length of time spent in various
 spans, it should generally include the time spent in a span's children as
 part of that span's duration.

 In addition to having zero or one parent, a span may also _follow from_ any
 number of other spans. This indicates a causal relationship between the span
 and the spans that it follows from, but a follower is *not* typically
 considered part of the duration of the span it follows. Unlike the parent, a
 span may record that it follows from another span after it is created, using
 the [`follows_from`] method.

 As an example, consider a listener task in a server. As the listener accepts
 incoming connections, it spawns new tasks that handle those connections. We
 might want to have a span representing the listener, and instrument each
 spawned handler task with its own span. We would want our instrumentation to
 record that the handler tasks were spawned as a result of the listener task.
 However, we might not consider the handler tasks to be _part_ of the time
 spent in the listener task, so we would not consider those spans children of
 the listener span. Instead, we would record that the handler tasks follow
 from the listener, recording the causal relationship but treating the spans
 as separate durations.

 ## Closing Spans

 Execution may enter and exit a span multiple times before that span is
 _closed_. Consider, for example, a future which has an associated
 span and enters that span every time it is polled:
 ```rust
 # use std::future::Future;
 # use std::task::{Context, Poll};
 # use std::pin::Pin;
 struct MyFuture {
    // data
    span: tracing::Span,
 }

 impl Future for MyFuture {
     type Output = ();

     fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
         let _enter = self.span.enter();
         // Do actual future work...
 # Poll::Ready(())
     }
 }
 ```

 If this future was spawned on an executor, it might yield one or more times
 before `poll` returns [`Poll::Ready`]. If the future were to yield, then
 the executor would move on to poll the next future, which may _also_ enter
 an associated span or series of spans. Therefore, it is valid for a span to
 be entered repeatedly before it completes. Only the time when that span or
 one of its children was the current span is considered to be time spent in
 that span. A span which is not executing and has not yet been closed is said
 to be _idle_.

 Because spans may be entered and exited multiple times before they close,
 [`Subscriber`]s have separate trait methods which are called to notify them
 of span exits and when span handles are dropped. When execution exits a
 span, [`exit`] will always be called with that span's ID to notify the
 subscriber that the span has been exited. When span handles are dropped, the
 [`drop_span`] method is called with that span's ID. The subscriber may use
 this to determine whether or not the span will be entered again.

 If there is only a single handle with the capacity to exit a span, dropping
 that handle "closes" the span, since the capacity to enter it no longer
 exists. For example:
 ```
 # use tracing::{Level, span};
 {
     span!(Level::TRACE, "my_span").in_scope(|| {
         // perform some work in the context of `my_span`...
     }); // --> Subscriber::exit(my_span)

     // The handle to `my_span` only lives inside of this block; when it is
     // dropped, the subscriber will be informed via `drop_span`.

 } // --> Subscriber::drop_span(my_span)
 ```

 However, if multiple handles exist, the span can still be re-entered even if
 one or more is dropped. For determining when _all_ handles to a span have
 been dropped, `Subscriber`s have a [`clone_span`] method, which is called
 every time a span handle is cloned. Combined with `drop_span`, this may be
 used to track the number of handles to a given span — if `drop_span` has
 been called one more time than the number of calls to `clone_span` for a
 given ID, then no more handles to the span with that ID exist. The
 subscriber may then treat it as closed.

 # When to use spans

 As a rule of thumb, spans should be used to represent discrete units of work
 (e.g., a given request's lifetime in a server) or periods of time spent in a
 given context (e.g., time spent interacting with an instance of an external
 system, such as a database).

 Which scopes in a program correspond to new spans depend somewhat on user
 intent. For example, consider the case of a loop in a program. Should we
 construct one span and perform the entire loop inside of that span, like:

 ```rust
 # use tracing::{Level, span};
 # let n = 1;
 let span = span!(Level::TRACE, "my_loop");
 let _enter = span.enter();
 for i in 0..n {
     # let _ = i;
     // ...
 }
 ```
 Or, should we create a new span for each iteration of the loop, as in:
 ```rust
 # use tracing::{Level, span};
 # let n = 1u64;
 for i in 0..n {
     let span = span!(Level::TRACE, "my_loop", iteration = i);
     let _enter = span.enter();
     // ...
 }
 ```

 Depending on the circumstances, we might want to do either, or both. For
 example, if we want to know how long was spent in the loop overall, we would
 create a single span around the entire loop; whereas if we wanted to know how
 much time was spent in each individual iteration, we would enter a new span
 on every iteration.

 [fields]: super::field
 [Metadata]: super::Metadata
 [verbosity level]: super::Level
 [`Poll::Ready`]: std::task::Poll::Ready
 [`span!`]: super::span!
 [`trace_span!`]: super::trace_span!
 [`debug_span!`]: super::debug_span!
 [`info_span!`]: super::info_span!
 [`warn_span!`]: super::warn_span!
 [`error_span!`]: super::error_span!
 [`clone_span`]: super::subscriber::Subscriber::clone_span()
 [`drop_span`]: super::subscriber::Subscriber::drop_span()
 [`exit`]: super::subscriber::Subscriber::exit
 [`Subscriber`]: super::subscriber::Subscriber
 [`enter`]: Span::enter()
 [`entered`]: Span::entered()
 [`in_scope`]: Span::in_scope()
 [`follows_from`]: Span::follows_from()
 [guard]: Entered
 [parent]: #span-relationships

```rust
pub mod span { /* ... */ }
```

### Types

#### Struct `Span`

A handle representing a span, with the capability to enter the span if it
exists.

If the span was rejected by the current `Subscriber`'s filter, entering the
span will silently do nothing. Thus, the handle can be used in the same
manner regardless of whether or not the trace is currently being collected.

```rust
pub struct Span {
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
  pub fn new(meta: &''static Metadata<''static>, values: &field::ValueSet<''_>) -> Span { /* ... */ }
  ```
  Constructs a new `Span` with the given [metadata] and set of

- ```rust
  pub fn new_root(meta: &''static Metadata<''static>, values: &field::ValueSet<''_>) -> Span { /* ... */ }
  ```
  Constructs a new `Span` as the root of its own trace tree, with the

- ```rust
  pub fn child_of</* synthetic */ impl Into<Option<Id>>: Into<Option<Id>>>(parent: impl Into<Option<Id>>, meta: &''static Metadata<''static>, values: &field::ValueSet<''_>) -> Span { /* ... */ }
  ```
  Constructs a new `Span` as child of the given parent span, with the

- ```rust
  pub fn new_disabled(meta: &''static Metadata<''static>) -> Span { /* ... */ }
  ```
  Constructs a new disabled span with the given `Metadata`.

- ```rust
  pub const fn none() -> Span { /* ... */ }
  ```
  Constructs a new span that is *completely disabled*.

- ```rust
  pub fn current() -> Span { /* ... */ }
  ```
  Returns a handle to the span [considered by the `Subscriber`] to be the

- ```rust
  pub fn enter(self: &Self) -> Entered<''_> { /* ... */ }
  ```
  Enters this span, returning a guard that will exit the span when dropped.

- ```rust
  pub fn entered(self: Self) -> EnteredSpan { /* ... */ }
  ```
  Enters this span, consuming it and returning a [guard][`EnteredSpan`]

- ```rust
  pub fn or_current(self: Self) -> Self { /* ... */ }
  ```
  Returns this span, if it was [enabled] by the current [`Subscriber`], or

- ```rust
  pub fn in_scope<F: FnOnce() -> T, T>(self: &Self, f: F) -> T { /* ... */ }
  ```
  Executes the given function in the context of this span.

- ```rust
  pub fn field<Q: field::AsField + ?Sized>(self: &Self, field: &Q) -> Option<field::Field> { /* ... */ }
  ```
  Returns a [`Field`][super::field::Field] for the field with the

- ```rust
  pub fn has_field<Q: field::AsField + ?Sized>(self: &Self, field: &Q) -> bool { /* ... */ }
  ```
  Returns true if this `Span` has a field for the given

- ```rust
  pub fn record<Q: field::AsField + ?Sized, V: field::Value>(self: &Self, field: &Q, value: V) -> &Self { /* ... */ }
  ```
  Records that the field described by `field` has the value `value`.

- ```rust
  pub fn record_all(self: &Self, values: &field::ValueSet<''_>) -> &Self { /* ... */ }
  ```
  Records all the fields in the provided `ValueSet`.

- ```rust
  pub fn is_disabled(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this span was disabled by the subscriber and does not

- ```rust
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this span was constructed by [`Span::none`] and is

- ```rust
  pub fn follows_from</* synthetic */ impl Into<Option<Id>>: Into<Option<Id>>>(self: &Self, from: impl Into<Option<Id>>) -> &Self { /* ... */ }
  ```
  Indicates that the span with the given ID has an indirect causal

- ```rust
  pub fn id(self: &Self) -> Option<Id> { /* ... */ }
  ```
  Returns this span's `Id`, if it is enabled.

- ```rust
  pub fn metadata(self: &Self) -> Option<&''static Metadata<''static>> { /* ... */ }
  ```
  Returns this span's `Metadata`, if it is enabled.

- ```rust
  pub fn with_subscriber<T, /* synthetic */ impl FnOnce((&Id, &Dispatch)) -> T: FnOnce((&Id, &Dispatch)) -> T>(self: &Self, f: impl FnOnce((&Id, &Dispatch)) -> T) -> Option<T> { /* ... */ }
  ```
  Invokes a function with a reference to this span's ID and subscriber.

###### Trait Implementations

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Span { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **WithSubscriber**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(span: &''a Span) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(span: &''a Span) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(span: Span) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Sync**
- **Send**
- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, hasher: &mut H) { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Instrument**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Entered`

**Attributes:**

- `#[must_use = "once a span has been entered, it should be exited"]`

A guard representing a span which has been entered and is currently
executing.

When the guard is dropped, the span will be exited.

This is returned by the [`Span::enter`] function.

[`Span::enter`]: super::Span::enter

```rust
pub struct Entered<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Instrument**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `EnteredSpan`

**Attributes:**

- `#[must_use = "once a span has been entered, it should be exited"]`

An owned version of [`Entered`], a guard representing a span which has been
entered and is currently executing.

When the guard is dropped, the span will be exited.

This is returned by the [`Span::entered`] function.

[`Span::entered`]: super::Span::entered()

```rust
pub struct EnteredSpan {
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
  pub fn id(self: &Self) -> Option<Id> { /* ... */ }
  ```
  Returns this span's `Id`, if it is enabled.

- ```rust
  pub fn exit(self: Self) -> Span { /* ... */ }
  ```
  Exits this span, returning the underlying [`Span`].

###### Trait Implementations

- **Receiver**
- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Span { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(span: &''a EnteredSpan) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(span: &''a EnteredSpan) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **WithSubscriber**
- **Instrument**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
### Traits

#### Trait `AsId`

Trait implemented by types which have a span `Id`.

```rust
pub trait AsId: crate::sealed::Sealed {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_id`: Returns the `Id` of the span that `self` corresponds to, or `None` if

### Re-exports

#### Re-export `Attributes`

```rust
pub use tracing_core::span::Attributes;
```

#### Re-export `Id`

```rust
pub use tracing_core::span::Id;
```

#### Re-export `Record`

```rust
pub use tracing_core::span::Record;
```

## Module `subscriber`

Collects and records trace data.

```rust
pub mod subscriber { /* ... */ }
```

### Functions

#### Function `with_default`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Sets this [`Subscriber`] as the default for the current thread for the
duration of a closure.

The default subscriber is used when creating a new [`Span`] or
[`Event`].


[`Span`]: super::span::Span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

```rust
pub fn with_default<T, S, /* synthetic */ impl FnOnce() -> T: FnOnce() -> T>(subscriber: S, f: impl FnOnce() -> T) -> T
where
    S: Subscriber + Send + Sync + ''static { /* ... */ }
```

#### Function `set_global_default`

Sets this subscriber as the global default for the duration of the entire program.
Will be used as a fallback if no thread-local subscriber has been set in a thread (using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail.
Returns whether the initialization was successful.

Note: Libraries should *NOT* call `set_global_default()`! That will cause conflicts when
executables try to set them later.

[span]: super::span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

```rust
pub fn set_global_default<S>(subscriber: S) -> Result<(), SetGlobalDefaultError>
where
    S: Subscriber + Send + Sync + ''static { /* ... */ }
```

#### Function `set_default`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[must_use = "Dropping the guard unregisters the subscriber."]`

Sets the [`Subscriber`] as the default for the current thread for the
duration of the lifetime of the returned [`DefaultGuard`].

The default subscriber is used when creating a new [`Span`] or [`Event`].

[`Span`]: super::span::Span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event
[`DefaultGuard`]: super::dispatcher::DefaultGuard

```rust
pub fn set_default<S>(subscriber: S) -> DefaultGuard
where
    S: Subscriber + Send + Sync + ''static { /* ... */ }
```

### Re-exports

#### Re-export `DefaultGuard`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use tracing_core::dispatcher::DefaultGuard;
```

#### Re-export `SetGlobalDefaultError`

```rust
pub use tracing_core::dispatcher::SetGlobalDefaultError;
```

#### Re-export `tracing_core::subscriber::*`

```rust
pub use tracing_core::subscriber::*;
```

## Macros

### Macro `event`

**Attributes:**

- `#[macro_export]`

Constructs a new `Event`.

The event macro is invoked with a `Level` and up to 32 key-value fields.
Optionally, a format string and arguments may follow the fields; this will
be used to construct an implicit field named "message".

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros

# Examples

```rust
use tracing::{event, Level};

# fn main() {
let data = (42, "forty-two");
let private_data = "private";
let error = "a bad error";

event!(Level::ERROR, %error, "Received error");
event!(
    target: "app_events",
    Level::WARN,
    private_data,
    ?data,
    "App warning: {}",
    error
);
event!(name: "answer", Level::INFO, the_answer = data.0);
event!(Level::INFO, the_answer = data.0);
# }
```


```rust
pub macro_rules! event {
    /* macro_rules! event {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (target: $target:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    ($lvl:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    ($lvl:expr, $($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, ?$($k:ident).+) => { ... };
    ($lvl:expr, %$($k:ident).+) => { ... };
    ($lvl:expr, $($k:ident).+) => { ... };
    ( $lvl:expr, $($arg:tt)+ ) => { ... };
} */
}
```

### Macro `span`

**Attributes:**

- `#[macro_export]`

Constructs a new span.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros

# Examples

Creating a new span:
```
# use tracing::{span, Level};
# fn main() {
let span = span!(Level::TRACE, "my span");
let _enter = span.enter();
// do work inside the span...
# }
```

```rust
pub macro_rules! span {
    /* macro_rules! span {
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr) => { ... };
    ($lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    ($lvl:expr, $name:expr) => { ... };
} */
}
```

### Macro `trace_span`

**Attributes:**

- `#[macro_export]`

Constructs a span at the trace level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{trace_span, span, Level};
# fn main() {
trace_span!("my_span");
// is equivalent to:
span!(Level::TRACE, "my_span");
# }
```

```rust
# use tracing::{trace_span, span, Level};
# fn main() {
let span = trace_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
pub macro_rules! trace_span {
    /* macro_rules! trace_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
} */
}
```

### Macro `debug_span`

**Attributes:**

- `#[macro_export]`

Constructs a span at the debug level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{debug_span, span, Level};
# fn main() {
debug_span!("my_span");
// is equivalent to:
span!(Level::DEBUG, "my_span");
# }
```

```rust
# use tracing::debug_span;
# fn main() {
let span = debug_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
pub macro_rules! debug_span {
    /* macro_rules! debug_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
} */
}
```

### Macro `info_span`

**Attributes:**

- `#[macro_export]`

Constructs a span at the info level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{span, info_span, Level};
# fn main() {
info_span!("my_span");
// is equivalent to:
span!(Level::INFO, "my_span");
# }
```

```rust
# use tracing::info_span;
# fn main() {
let span = info_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
pub macro_rules! info_span {
    /* macro_rules! info_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
} */
}
```

### Macro `warn_span`

**Attributes:**

- `#[macro_export]`

Constructs a span at the warn level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{warn_span, span, Level};
# fn main() {
warn_span!("my_span");
// is equivalent to:
span!(Level::WARN, "my_span");
# }
```

```rust
use tracing::warn_span;
# fn main() {
let span = warn_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
pub macro_rules! warn_span {
    /* macro_rules! warn_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
} */
}
```

### Macro `error_span`

**Attributes:**

- `#[macro_export]`

Constructs a span at the error level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{span, error_span, Level};
# fn main() {
error_span!("my_span");
// is equivalent to:
span!(Level::ERROR, "my_span");
# }
```

```rust
# use tracing::error_span;
# fn main() {
let span = error_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
pub macro_rules! error_span {
    /* macro_rules! error_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
} */
}
```

### Macro `event_enabled`

**Attributes:**

- `#[macro_export]`

Tests whether an event with the specified level and target would be enabled.

This is similar to [`enabled!`], but queries the current subscriber specifically for
an event, whereas [`enabled!`] queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also [`span_enabled!`].

# Examples

```rust
# use tracing::{event_enabled, Level};
if event_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if event_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if event_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```

[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

```rust
pub macro_rules! event_enabled {
    /* macro_rules! event_enabled {
    ($($rest:tt)*) => { ... };
} */
}
```

### Macro `span_enabled`

**Attributes:**

- `#[macro_export]`

Tests whether a span with the specified level and target would be enabled.

This is similar to [`enabled!`], but queries the current subscriber specifically for
an event, whereas [`enabled!`] queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also [`span_enabled!`].

# Examples

```rust
# use tracing::{span_enabled, Level};
if span_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if span_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if span_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```

[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

```rust
pub macro_rules! span_enabled {
    /* macro_rules! span_enabled {
    ($($rest:tt)*) => { ... };
} */
}
```

### Macro `enabled`

**Attributes:**

- `#[macro_export]`

Checks whether a span or event is [enabled] based on the provided [metadata].

[enabled]: crate::Subscriber::enabled
[metadata]: crate::Metadata

This macro is a specialized tool: it is intended to be used prior
to an expensive computation required *just* for that event, but
*cannot* be done as part of an argument to that event, such as
when multiple events are emitted (e.g., iterating over a collection
and emitting an event for each item).

# Usage

[Subscribers] can make filtering decisions based all the data included in a
span or event's [`Metadata`]. This means that it is possible for `enabled!`
to return a _false positive_ (indicating that something would be enabled
when it actually would not be) or a _false negative_ (indicating that
something would be disabled when it would actually be enabled).

[Subscribers]: crate::subscriber::Subscriber
[`Metadata`]: crate::metadata::Metadata

This occurs when a subscriber is using a _more specific_ filter than the
metadata provided to the `enabled!` macro. Some situations that can result
in false positives or false negatives include:

- If a subscriber is using a filter which may enable a span or event based
  on field names, but `enabled!` is invoked without listing field names,
  `enabled!` may return a false negative if a specific field name would
  cause the subscriber to enable something that would otherwise be disabled.
- If a subscriber is using a filter which enables or disables specific events by
  file path and line number,  a particular event may be enabled/disabled
  even if an `enabled!` invocation with the same level, target, and fields
  indicated otherwise.
- The subscriber can choose to enable _only_ spans or _only_ events, which `enabled`
  will not reflect.

`enabled!()` requires a [level](crate::Level) argument, an optional `target:`
argument, and an optional set of field names. If the fields are not provided,
they are considered to be unknown. `enabled!` attempts to match the
syntax of `event!()` as closely as possible, which can be seen in the
examples below.

# Examples

If the current subscriber is interested in recording `DEBUG`-level spans and
events in the current file and module path, this will evaluate to true:
```rust
use tracing::{enabled, Level};

if enabled!(Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", and at the
level  `DEBUG`, this will evaluate to true:
```rust
# use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", at
the level `DEBUG`, and with a field named "hello", this will evaluate
to true:

```rust
# use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG, hello) {
    // some expensive work...
}
```

# Alternatives

`enabled!` queries subscribers with [`Metadata`] where
[`is_event`] and [`is_span`] both return `false`. Alternatively,
use [`event_enabled!`] or [`span_enabled!`] to ensure one of these
returns true.


[`Metadata`]: crate::Metadata
[`is_event`]: crate::Metadata::is_event
[`is_span`]: crate::Metadata::is_span
[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

```rust
pub macro_rules! enabled {
    /* macro_rules! enabled {
    (kind: $kind:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (kind: $kind:expr, target: $target:expr, $lvl:expr ) => { ... };
    (target: $target:expr, $lvl:expr ) => { ... };
    (kind: $kind:expr, target: $target:expr, $lvl:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $($field:tt)*) => { ... };
    (kind: $kind:expr, $lvl:expr, $($field:tt)*) => { ... };
    (kind: $kind:expr, $lvl:expr) => { ... };
    ($lvl:expr) => { ... };
    ($lvl:expr, $($field:tt)*) => { ... };
} */
}
```

### Macro `trace`

**Attributes:**

- `#[macro_export]`

Constructs an event at the trace level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::trace;
# #[derive(Debug, Copy, Clone)] struct Position { x: f32, y: f32 }
# impl Position {
# const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
# fn dist(&self, other: Position) -> f32 {
#    let x = (other.x - self.x).exp2(); let y = (self.y - other.y).exp2();
#    (x + y).sqrt()
# }
# }
# fn main() {
let pos = Position { x: 3.234, y: -1.223 };
let origin_dist = pos.dist(Position::ORIGIN);

trace!(position = ?pos, ?origin_dist);
trace!(
    target: "app_events",
    position = ?pos,
    "x is {} and y is {}",
    if pos.x >= 0.0 { "positive" } else { "negative" },
    if pos.y >= 0.0 { "positive" } else { "negative" }
);
trace!(name: "completed", position = ?pos);
# }
```

```rust
pub macro_rules! trace {
    /* macro_rules! trace {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
} */
}
```

### Macro `debug`

**Attributes:**

- `#[macro_export]`

Constructs an event at the debug level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::debug;
# fn main() {
# #[derive(Debug)] struct Position { x: f32, y: f32 }

let pos = Position { x: 3.234, y: -1.223 };

debug!(?pos.x, ?pos.y);
debug!(target: "app_events", position = ?pos, "New position");
debug!(name: "completed", position = ?pos);
# }
```

```rust
pub macro_rules! debug {
    /* macro_rules! debug {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
} */
}
```

### Macro `info`

**Attributes:**

- `#[macro_export]`

Constructs an event at the info level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::info;
# // this is so the test will still work in no-std mode
# #[derive(Debug)]
# pub struct Ipv4Addr;
# impl Ipv4Addr { fn new(o1: u8, o2: u8, o3: u8, o4: u8) -> Self { Self } }
# fn main() {
# struct Connection { port: u32, speed: f32 }
use tracing::field;

let addr = Ipv4Addr::new(127, 0, 0, 1);
let conn = Connection { port: 40, speed: 3.20 };

info!(conn.port, "connected to {:?}", addr);
info!(
    target: "connection_events",
    ip = ?addr,
    conn.port,
    ?conn.speed,
);
info!(name: "completed", "completed connection to {:?}", addr);
# }
```

```rust
pub macro_rules! info {
    /* macro_rules! info {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
} */
}
```

### Macro `warn`

**Attributes:**

- `#[macro_export]`

Constructs an event at the warn level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::warn;
# fn main() {

let warn_description = "Invalid Input";
let input = &[0x27, 0x45];

warn!(?input, warning = warn_description);
warn!(
    target: "input_events",
    warning = warn_description,
    "Received warning for input: {:?}", input,
);
warn!(name: "invalid", ?input);
# }
```

```rust
pub macro_rules! warn {
    /* macro_rules! warn {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
} */
}
```

### Macro `error`

**Attributes:**

- `#[macro_export]`

Constructs an event at the error level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::error;
# fn main() {

let (err_info, port) = ("No connection", 22);

error!(port, error = %err_info);
error!(target: "app_events", "App Error: {}", err_info);
error!({ info = err_info }, "error on port: {}", port);
error!(name: "invalid_input", "Invalid input: {}", err_info);
# }
```

```rust
pub macro_rules! error {
    /* macro_rules! error {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
} */
}
```

## Re-exports

### Re-export `Instrument`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::instrument::Instrument;
```

### Re-export `Dispatch`

```rust
pub use self::dispatcher::Dispatch;
```

### Re-export `Event`

```rust
pub use self::event::Event;
```

### Re-export `Value`

```rust
pub use self::field::Value;
```

### Re-export `Subscriber`

```rust
pub use self::subscriber::Subscriber;
```

### Re-export `event`

```rust
pub use tracing_core::event;
```

### Re-export `Level`

```rust
pub use tracing_core::Level;
```

### Re-export `Metadata`

```rust
pub use tracing_core::Metadata;
```

### Re-export `Span`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use self::span::Span;
```

### Re-export `instrument`

**Attributes:**

- `#[<cfg>(feature = "attributes")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "attributes")))]`
- `#[doc(inline)]`

```rust
pub use tracing_attributes::instrument;
```

