# Crate Documentation

**Version:** 0.1.28

**Format Version:** 43

# Module `tracing_attributes`

A procedural macro attribute for instrumenting functions with [`tracing`].

[`tracing`] is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information. This crate provides the
[`#[instrument]`][instrument] procedural macro attribute.

Note that this macro is also re-exported by the main `tracing` crate.

*Compiler support: [requires `rustc` 1.63+][msrv]*

[msrv]: #supported-rust-versions

## Usage

In the `Cargo.toml`:

```toml
[dependencies]
tracing-attributes = "0.1.24"
```

The [`#[instrument]`][instrument] attribute can now be added to a function
to automatically create and enter `tracing` [span] when that function is
called. For example:

```
use tracing::instrument;

#[instrument]
pub fn my_function(my_arg: usize) {
    // ...
}

# fn main() {}
```

[`tracing`]: https://crates.io/crates/tracing
[span]: https://docs.rs/tracing/latest/tracing/span/index.html
[instrument]: macro@self::instrument

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


## Macros

### Procedural Macro `instrument`

**Attributes:**

- `#[proc_macro_attribute]`

Instruments a function to create and enter a `tracing` [span] every time
the function is called.

Unless overridden, a span with the [`INFO`] [level] will be generated.
The generated span's name will be the name of the function.
By default, all arguments to the function are included as fields on the
span. Arguments that are `tracing` [primitive types] implementing the
[`Value` trait] will be recorded as fields of that type. Types which do
not implement `Value` will be recorded using [`fmt::Debug`].

[primitive types]: https://docs.rs/tracing/latest/tracing/field/trait.Value.html#foreign-impls
[`Value` trait]: https://docs.rs/tracing/latest/tracing/field/trait.Value.html

# Overriding Span Attributes

To change the [name] of the generated span, add a `name` argument to the
`#[instrument]` macro, followed by an equals sign and a string literal. For
example:

```
# use tracing_attributes::instrument;

// The generated span's name will be "my_span" rather than "my_function".
#[instrument(name = "my_span")]
pub fn my_function() {
    // ... do something incredibly interesting and important ...
}
```

To override the [target] of the generated span, add a `target` argument to
the `#[instrument]` macro, followed by an equals sign and a string literal
for the new target. The [module path] is still recorded separately. For
example:

```
pub mod my_module {
    # use tracing_attributes::instrument;
    // The generated span's target will be "my_crate::some_special_target",
    // rather than "my_crate::my_module".
    #[instrument(target = "my_crate::some_special_target")]
    pub fn my_function() {
        // ... all kinds of neat code in here ...
    }
}
```

Finally, to override the [level] of the generated span, add a `level`
argument, followed by an equals sign and a string literal with the name of
the desired level. Level names are not case sensitive. For example:

```
# use tracing_attributes::instrument;
// The span's level will be TRACE rather than INFO.
#[instrument(level = "trace")]
pub fn my_function() {
    // ... I have written a truly marvelous implementation of this function,
    // which this example is too narrow to contain ...
}
```

# Skipping Fields

To skip recording one or more arguments to a function or method, pass
the argument's name inside the `skip()` argument on the `#[instrument]`
macro. This can be used when an argument to an instrumented function does
not implement [`fmt::Debug`], or to exclude an argument with a verbose or
costly `Debug` implementation. Note that:

- multiple argument names can be passed to `skip`.
- arguments passed to `skip` do _not_ need to implement `fmt::Debug`.

You can also use `skip_all` to skip all arguments.

## Examples

```
# use tracing_attributes::instrument;
# use std::collections::HashMap;
// This type doesn't implement `fmt::Debug`!
struct NonDebug;

// `arg` will be recorded, while `non_debug` will not.
#[instrument(skip(non_debug))]
fn my_function(arg: usize, non_debug: NonDebug) {
    // ...
}

// These arguments are huge
#[instrument(skip_all)]
fn my_big_data_function(large: Vec<u8>, also_large: HashMap<String, String>) {
    // ...
}
```

Skipping the `self` parameter:

```
# use tracing_attributes::instrument;
#[derive(Debug)]
struct MyType {
   data: Vec<u8>, // Suppose this buffer is often quite long...
}

impl MyType {
    // Suppose we don't want to print an entire kilobyte of `data`
    // every time this is called...
    #[instrument(skip(self))]
    pub fn my_method(&mut self, an_interesting_argument: usize) {
         // ... do something (hopefully, using all that `data`!)
    }
}
```

# Adding Fields

Additional fields (key-value pairs with arbitrary data) can be passed to
to the generated span through the `fields` argument on the
`#[instrument]` macro. Strings, integers or boolean literals are accepted values
for each field. The name of the field must be a single valid Rust
identifier, nested (dotted) field names are not supported. Any
Rust expression can be used as a field value in this manner. These
expressions will be evaluated at the beginning of the function's body, so
arguments to the function may be used in these expressions. Field names may
also be specified *without* values. Doing so will result in an [empty field]
whose value may be recorded later within the function body.

Note that overlap between the names of fields and (non-skipped) arguments
will result in a compile error.

## Examples

Adding a new field based on the value of an argument:

```
# use tracing_attributes::instrument;

// This will record a field named "i" with the value of `i` *and* a field
// named "next" with the value of `i` + 1.
#[instrument(fields(next = i + 1))]
pub fn my_function(i: usize) {
    // ...
}
```

Recording specific properties of a struct as their own fields:

```
# mod http {
#   pub struct Error;
#   pub struct Response<B> { pub(super) _b: std::marker::PhantomData<B> }
#   pub struct Request<B> { _b: B }
#   impl<B> std::fmt::Debug for Request<B> {
#       fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
#           f.pad("request")
#       }
#   }
#   impl<B> Request<B> {
#       pub fn uri(&self) -> &str { "fake" }
#       pub fn method(&self) -> &str { "GET" }
#   }
# }
# use tracing_attributes::instrument;

// This will record the request's URI and HTTP method as their own separate
// fields.
#[instrument(fields(http.uri = req.uri(), http.method = req.method()))]
pub fn handle_request<B>(req: http::Request<B>) -> http::Response<B> {
    // ... handle the request ...
    # http::Response { _b: std::marker::PhantomData }
}
```

This can be used in conjunction with `skip` or `skip_all` to record only
some fields of a struct:
```
# use tracing_attributes::instrument;
// Remember the struct with the very large `data` field from the earlier
// example? Now it also has a `name`, which we might want to include in
// our span.
#[derive(Debug)]
struct MyType {
   name: &'static str,
   data: Vec<u8>,
}

impl MyType {
    // This will skip the `data` field, but will include `self.name`,
    // formatted using `fmt::Display`.
    #[instrument(skip(self), fields(self.name = %self.name))]
    pub fn my_method(&mut self, an_interesting_argument: usize) {
         // ... do something (hopefully, using all that `data`!)
    }
}
```

Adding an empty field to be recorded later:

```
# use tracing_attributes::instrument;

// This function does a very interesting and important mathematical calculation.
// Suppose we want to record both the inputs to the calculation *and* its result...
#[instrument(fields(result))]
pub fn do_calculation(input_1: usize, input_2: usize) -> usize {
    // Rerform the calculation.
    let result = input_1 + input_2;

    // Record the result as part of the current span.
    tracing::Span::current().record("result", &result);

    // Now, the result will also be included on this event!
    tracing::info!("calculation complete!");

    // ... etc ...
    # 0
}
```

# Examples

Instrumenting a function:

```
# use tracing_attributes::instrument;
#[instrument]
pub fn my_function(my_arg: usize) {
    // This event will be recorded inside a span named `my_function` with the
    // field `my_arg`.
    tracing::info!("inside my_function!");
    // ...
}
```
Setting the level for the generated span:
```
# use tracing_attributes::instrument;
# use tracing::Level;
#[instrument(level = Level::DEBUG)]
pub fn my_function() {
    // ...
}
```
Levels can be specified either with [`Level`] constants, literal strings
(e.g., `"debug"`, `"info"`) or numerically (1—5, corresponding to [`Level::TRACE`]—[`Level::ERROR`]).

Overriding the generated span's name:
```
# use tracing_attributes::instrument;
#[instrument(name = "my_name")]
pub fn my_function() {
    // ...
}
```
Overriding the generated span's target:
```
# use tracing_attributes::instrument;
#[instrument(target = "my_target")]
pub fn my_function() {
    // ...
}
```
Overriding the generated span's parent:
```
# use tracing_attributes::instrument;
#[instrument(parent = None)]
pub fn my_function() {
    // ...
}
```
```
# use tracing_attributes::instrument;
// A struct which owns a span handle.
struct MyStruct
{
    span: tracing::Span
}

impl MyStruct
{
    // Use the struct's `span` field as the parent span
    #[instrument(parent = &self.span, skip(self))]
    fn my_method(&self) {}
}
```
Specifying [`follows_from`] relationships:
```
# use tracing_attributes::instrument;
#[instrument(follows_from = causes)]
pub fn my_function(causes: &[tracing::Id]) {
    // ...
}
```
Any expression of type `impl IntoIterator<Item = impl Into<Option<Id>>>`
may be provided to `follows_from`; e.g.:
```
# use tracing_attributes::instrument;
#[instrument(follows_from = [cause])]
pub fn my_function(cause: &tracing::span::EnteredSpan) {
    // ...
}
```


To skip recording an argument, pass the argument's name to the `skip`:

```
# use tracing_attributes::instrument;
struct NonDebug;

#[instrument(skip(non_debug))]
fn my_function(arg: usize, non_debug: NonDebug) {
    // ...
}
```

To add additional context to the span, pass key-value pairs to `fields`:

```
# use tracing_attributes::instrument;
#[instrument(fields(foo="bar", id=1, show=true))]
fn my_function(arg: usize) {
    // ...
}
```

Adding the `ret` argument to `#[instrument]` will emit an event with the function's
return value when the function returns:

```
# use tracing_attributes::instrument;
#[instrument(ret)]
fn my_function() -> i32 {
    42
}
```
The return value event will have the same level as the span generated by `#[instrument]`.
By default, this will be [`INFO`], but if the level is overridden, the event will be at the same
level.

It's also possible to override the level for the `ret` event independently:

```
# use tracing_attributes::instrument;
# use tracing::Level;
#[instrument(ret(level = Level::WARN))]
fn my_function() -> i32 {
    42
}
```

**Note**:  if the function returns a `Result<T, E>`, `ret` will record returned values if and
only if the function returns [`Result::Ok`].

By default, returned values will be recorded using their [`std::fmt::Debug`] implementations.
If a returned value implements [`std::fmt::Display`], it can be recorded using its `Display`
implementation instead, by writing `ret(Display)`:

```
# use tracing_attributes::instrument;
#[instrument(ret(Display))]
fn my_function() -> i32 {
    42
}
```

If the function returns a `Result<T, E>` and `E` implements `std::fmt::Display`, adding
`err` or `err(Display)` will emit error events when the function returns `Err`:

```
# use tracing_attributes::instrument;
#[instrument(err)]
fn my_function(arg: usize) -> Result<(), std::io::Error> {
    Ok(())
}
```

The level of the error value event defaults to `ERROR`.

Similarly, overriding the level of the `err` event :

```
# use tracing_attributes::instrument;
# use tracing::Level;
#[instrument(err(level = Level::INFO))]
fn my_function(arg: usize) -> Result<(), std::io::Error> {
    Ok(())
}
```

By default, error values will be recorded using their `std::fmt::Display` implementations.
If an error implements `std::fmt::Debug`, it can be recorded using its `Debug` implementation
instead by writing `err(Debug)`:

```
# use tracing_attributes::instrument;
#[instrument(err(Debug))]
fn my_function(arg: usize) -> Result<(), std::io::Error> {
    Ok(())
}
```

If a `target` is specified, both the `ret` and `err` arguments will emit outputs to
the declared target (or the default channel if `target` is not specified).

The `ret` and `err` arguments can be combined in order to record an event if a
function returns [`Result::Ok`] or [`Result::Err`]:

```
# use tracing_attributes::instrument;
#[instrument(err, ret)]
fn my_function(arg: usize) -> Result<(), std::io::Error> {
    Ok(())
}
```

`async fn`s may also be instrumented:

```
# use tracing_attributes::instrument;
#[instrument]
pub async fn my_function() -> Result<(), ()> {
    // ...
    # Ok(())
}
```

It also works with [async-trait](https://crates.io/crates/async-trait)
(a crate that allows defining async functions in traits,
something not currently possible in Rust),
and hopefully most libraries that exhibit similar behaviors:

```
# use tracing::instrument;
use async_trait::async_trait;

#[async_trait]
pub trait Foo {
    async fn foo(&self, arg: usize);
}

#[derive(Debug)]
struct FooImpl(usize);

#[async_trait]
impl Foo for FooImpl {
    #[instrument(fields(value = self.0, tmp = std::any::type_name::<Self>()))]
    async fn foo(&self, arg: usize) {}
}
```

`const fn` cannot be instrumented, and will result in a compilation failure:

```compile_fail
# use tracing_attributes::instrument;
#[instrument]
const fn my_const_function() {}
```

[span]: https://docs.rs/tracing/latest/tracing/span/index.html
[name]: https://docs.rs/tracing/latest/tracing/struct.Metadata.html#method.name
[target]: https://docs.rs/tracing/latest/tracing/struct.Metadata.html#method.target
[level]: https://docs.rs/tracing/latest/tracing/struct.Level.html
[module path]: https://docs.rs/tracing/latest/tracing/struct.Metadata.html#method.module_path
[`INFO`]: https://docs.rs/tracing/latest/tracing/struct.Level.html#associatedconstant.INFO
[empty field]: https://docs.rs/tracing/latest/tracing/field/struct.Empty.html
[field syntax]: https://docs.rs/tracing/latest/tracing/#recording-fields
[`follows_from`]: https://docs.rs/tracing/latest/tracing/struct.Span.html#method.follows_from
[`tracing`]: https://github.com/tokio-rs/tracing
[`fmt::Debug`]: std::fmt::Debug
[`Level`]: https://docs.rs/tracing/latest/tracing/struct.Level.html
[`Level::TRACE`]: https://docs.rs/tracing/latest/tracing/struct.Level.html#associatedconstant.TRACE
[`Level::ERROR`]: https://docs.rs/tracing/latest/tracing/struct.Level.html#associatedconstant.ERROR

```rust
pub #[proc_macro_attribute]
pub fn instrument(/* ... */) -> /* ... */ {
    /* ... */
}
```

