# Crate Documentation

**Version:** 2.9.0

**Format Version:** 43

# Module `minijinja`

<div align=center>
  <img src="https://github.com/mitsuhiko/minijinja/raw/main/artwork/logo.png" alt="" width=320>
  <p><strong>MiniJinja: a powerful template engine for Rust with minimal dependencies</strong></p>
</div>

MiniJinja is a powerful but minimal dependency template engine for Rust which
is based on the syntax and behavior of the
[Jinja2](https://jinja.palletsprojects.com/) template engine for Python.  It's
implemented on top of [`serde`].  The goal is to be able to render a large
chunk of the Jinja2 template ecosystem from Rust with a minimal engine and to
leverage an already existing ecosystem of editor integrations.

```jinja
{% for user in users %}
  <li>{{ user.name }}</li>
{% endfor %}
```

You can play with MiniJinja online [in the browser
playground](https://mitsuhiko.github.io/minijinja-playground/) powered by a
WASM build of MiniJinja.

# Why MiniJinja

MiniJinja by its name wants to be a good default choice if you need a little
bit of templating with minimal dependencies.  It has the following goals:

* Well documented, compact API
* Minimal dependencies, reasonable compile times and decent runtime performance
* Stay close as possible to Jinja2
* Support for expression evaluation
* Support for all `serde` compatible types
* Excellent test coverage
* Support for dynamic runtime objects with methods and dynamic attributes

# Template Usage

To use MiniJinja one needs to create an [`Environment`] and populate it with
templates.  Afterwards templates can be loaded and rendered.  To pass data
one can pass any serde serializable value.  The [`context!`] macro can be
used to quickly construct a template context:

```
use minijinja::{Environment, context};

let mut env = Environment::new();
env.add_template("hello", "Hello {{ name }}!").unwrap();
let tmpl = env.get_template("hello").unwrap();
println!("{}", tmpl.render(context!(name => "John")).unwrap());
```

```plain
Hello John!
```

For super trivial cases where you need to render a string once, you can
also use the [`render!`] macro which acts a bit like a replacement
for the [`format!`] macro.

# Expression Usage

MiniJinja — like Jinja2 — allows to be used as expression language.  This can be
useful to express logic in configuration files or similar things.  For this
purpose the [`Environment::compile_expression`] method can be used.  It returns
an expression object that can then be evaluated, returning the result:

```
use minijinja::{Environment, context};

let env = Environment::new();
let expr = env.compile_expression("number < 42").unwrap();
let result = expr.eval(context!(number => 23)).unwrap();
assert_eq!(result.is_true(), true);
```

This becomes particularly powerful when [dynamic objects](crate::value::Object) are
exposed to templates.

# Custom Filters

MiniJinja lets you register functions as filter functions (see
[`Function`](crate::functions::Function)) with the engine.  These can then be
invoked directly from the template:

```
use minijinja::{Environment, context};

let mut env = Environment::new();
env.add_filter("repeat", str::repeat);
env.add_template("hello", "{{ 'Na '|repeat(3) }} {{ name }}!").unwrap();
let tmpl = env.get_template("hello").unwrap();
println!("{}", tmpl.render(context!(name => "Batman")).unwrap());
```

```plain
Na Na Na Batman!
```

# Learn more

- [`Environment`]: the main API entry point.  Teaches you how to configure the environment.
- [`Template`]: the template object API.  Shows you how templates can be rendered.
- [`syntax`]: provides documentation of the template engine syntax.
- [`filters`]: teaches you how to write custom filters and to see the list of built-in filters.
- [`tests`]: teaches you how to write custom test functions and to see the list of built-in tests.
- [`functions`]: teaches how to write custom functions and to see the list of built-in functions.
- For auto reloading use the [`minijinja-autoreload`](https://docs.rs/minijinja-autoreload) crate.
- For simpler embedding of templates use the [`minijinja-embed`](https://docs.rs/minijinja-embed) crate.

Additionally there is an [list of examples](https://github.com/mitsuhiko/minijinja/tree/main/examples)
with many different small example programs on GitHub to explore.

# Error Handling

MiniJinja tries to give you good errors out of the box.  However if you use includes or
template inheritance your experience will improve greatly if you ensure to render chained
errors.  For more information see [`Error`] with an example.

# Size and Compile Times

MiniJinja attempts to compile fast so it can be used as a sensible template engine choice
when compile times matter.  Because of this it's internally modular so unnecessary bits and
pieces can be removed.  The default set of features tries to strike a balance but in
situations where only a subset is needed (eg: `build.rs`) all default features can be
be disabled.

# Optional Features

MiniJinja comes with a lot of optional features, some of which are turned on by
default.  If you plan on using MiniJinja in a library, please consider turning
off all default features and to opt-in explicitly into the ones you actually
need.

<details><summary><strong style="cursor: pointer">Configurable Features</strong></summary>

To cut down on size of the engine some default functionality can be removed:

- **Engine Features:**

  - `builtins`: if this feature is removed the default filters, tests and
    functions are not implemented.
  - `macros`: when removed the `{% macro %}` tag is not included.
  - `multi_template`: when removed the templates related to imports and extends
    are removed (`{% from %}`, `{% import %}`, `{% include %}`, and `{% extends %}`
    as well as `{% block %}`).
  - `adjacent_loop_items`: when removed the `previtem` and `nextitem` attributes of
    the `loop` object no longer exist.  Removing this feature can provide faster
    template execution when a lot of loops are involved.
  - `unicode`: when added unicode identifiers are supported and the `sort`
    filter's case insensitive comparison changes to using unicode and not
    ASCII rules.  Without this features only ASCII identifiers can be used
    for variable names and attributes.
  - `serde`: enables or disables serde support.  In current versions of MiniJinja
    it's not possible to disable serde but it will become possible.  To prevent
    breakage, MiniJinja warns if this feature is disabled.

- **Rust Functionality:**

  - `debug`: if this feature is removed some debug functionality of the engine is
    removed as well.  This mainly affects the quality of error reporting.
  - `deserialization`: when removed this disables deserialization support for
    the [`Value`] type, removes the `ViaDeserialize` type and the error type
    no longer implements `serde::de::Error`.
  - `std_collections`: if this feature is removed some [`Object`](crate::value::Object)
    implementations for standard library collections are removed.  Only the
    ones needed for the engine to function itself are retained.

There are some additional features that provide extra functionality:

- `fuel`: enables the `fuel` feature which makes the engine track fuel consumption which
  can be used to better protect against expensive templates.
- `loader`: enables owned and dynamic template loading of templates.
- `custom_syntax`: when this feature is enabled, custom delimiters are supported by
  the parser.
- `preserve_order`: When enable the internal value implementation uses an indexmap
  which preserves the original order of maps and structs.
- `json`: When enabled the `tojson` filter is added as builtin filter as well as
  the ability to auto escape via `AutoEscape::Json`.
- `urlencode`: When enabled the `urlencode` filter is added as builtin filter.
- `loop_controls`: enables the `{% break %}` and `{% continue %}` loop control flow
  tags.

Performance and memory related features:

- `stacker`: enables automatic stack growth which permits much larger levels of recursion
  at runtime.  This does not affect the maximum recursion at parsing time which is always
  limited.
- `speedups`: enables all speedups, in particular it turns on the `v_htmlescape` dependency
  for faster HTML escaping.

Internals:

- `unstable_machinery`: exposes an unstable internal API (no semver guarantees) to parse
  templates and interact with the engine internals.  If you need this functionality, please
  leave some feedback on GitHub.

</details>

## Modules

## Module `filters`

Filter functions and abstractions.

MiniJinja inherits from Jinja2 the concept of filter functions.  These are functions
which are applied to values to modify them.  For example the expression `{{ 42|filter(23) }}`
invokes the filter `filter` with the arguments `42` and `23`.

MiniJinja comes with some built-in filters that are listed below. To create a
custom filter write a function that takes at least a value, then registers it
with [`add_filter`](crate::Environment::add_filter).

# Using Filters

Using filters in templates is possible in all places an expression is permitted.
This means they are not just used for printing but also are useful for iteration
or similar situations.

Motivating example:

```jinja
<dl>
{% for key, value in config|items %}
  <dt>{{ key }}
  <dd><pre>{{ value|tojson }}</pre>
{% endfor %}
</dl>
```

# Custom Filters

A custom filter is just a simple function which accepts its inputs
as parameters and then returns a new value.  For instance the following
shows a filter which takes an input value and replaces whitespace with
dashes and converts it to lowercase:

```
# use minijinja::Environment;
# let mut env = Environment::new();
fn slugify(value: String) -> String {
    value.to_lowercase().split_whitespace().collect::<Vec<_>>().join("-")
}

env.add_filter("slugify", slugify);
```

MiniJinja will perform the necessary conversions automatically.  For more
information see the [`Function`](crate::functions::Function) trait.

# Accessing State

In some cases it can be necessary to access the execution [`State`].  Since a borrowed
state implements [`ArgType`](crate::value::ArgType) it's possible to add a
parameter that holds the state.  For instance the following filter appends
the current template name to the string:

```
# use minijinja::Environment;
# let mut env = Environment::new();
use minijinja::{Value, State};

fn append_template(state: &State, value: &Value) -> String {
    format!("{}-{}", value, state.name())
}

env.add_filter("append_template", append_template);
```

# Filter configuration

The recommended pattern for filters to change their behavior is to leverage global
variables in the template.  For instance take a filter that performs date formatting.
You might want to change the default time format format on a per-template basis
without having to update every filter invocation.  In this case the recommended
pattern is to reserve upper case variables and look them up in the filter:

```
# use minijinja::Environment;
# let mut env = Environment::new();
# fn format_unix_timestamp(_: f64, _: &str) -> String { "".into() }
use minijinja::State;

fn timeformat(state: &State, ts: f64) -> String {
    let configured_format = state.lookup("TIME_FORMAT");
    let format = configured_format
        .as_ref()
        .and_then(|x| x.as_str())
        .unwrap_or("HH:MM:SS");
    format_unix_timestamp(ts, format)
}

env.add_filter("timeformat", timeformat);
```

This then later lets a user override the default either by using
[`add_global`](crate::Environment::add_global) or by passing it with the
[`context!`] macro or similar.

```
# use minijinja::context;
# let other_variables = context!{};
let ctx = context! {
    TIME_FORMAT => "HH:MM",
    ..other_variables
};
```

# Built-in Filters

When the `builtins` feature is enabled a range of built-in filters are
automatically added to the environment.  These are also all provided in
this module.  Note though that these functions are not to be
called from Rust code as their exact interface (arguments and return types)
might change from one MiniJinja version to another.

Some additional filters are available in the
[`minijinja-contrib`](https://crates.io/crates/minijinja-contrib) crate.

```rust
pub mod filters { /* ... */ }
```

### Functions

#### Function `safe`

Marks a value as safe.  This converts it into a string.

When a value is marked as safe, no further auto escaping will take place.

```rust
pub fn safe(v: String) -> crate::value::Value { /* ... */ }
```

#### Function `escape`

Escapes a string.  By default to HTML.

By default this filter is also registered under the alias `e`.  Note that
this filter escapes with the format that is native to the format or HTML
otherwise.  This means that if the auto escape setting is set to
`Json` for instance then this filter will serialize to JSON instead.

```rust
pub fn escape(state: &crate::vm::State<''_, ''_>, v: &crate::value::Value) -> Result<crate::value::Value, crate::error::Error> { /* ... */ }
```

### Re-exports

#### Re-export `self::builtins::*`

**Attributes:**

- `#[<cfg>(feature = "builtins")]`

```rust
pub use self::builtins::*;
```

## Module `functions`

**Attributes:**

- `#[<cfg_attr>(feature = "deserialization", doc =
r#"
# Arguments in Custom Functions

All arguments in custom functions must implement the [`ArgType`] trait.
Standard types, such as `String`, `i32`, `bool`, `f64`, etc, already implement this trait.
There are also helper types that will make it easier to extract an arguments with custom types.
The [`ViaDeserialize<T>`](crate::value::ViaDeserialize) type, for instance, can accept any
type `T` that implements the `Deserialize` trait from `serde`.

```rust
# use minijinja::Environment;
# use serde::Deserialize;
# let mut env = Environment::new();
use minijinja::value::ViaDeserialize;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: i32,
}

fn is_adult(person: ViaDeserialize<Person>) -> bool {
    person.age >= 18
}

env.add_function("is_adult", is_adult);
```
"#)]`

Global functions and abstractions.

This module provides the abstractions for functions that can registered as
global functions to the environment via
[`add_function`](crate::Environment::add_function).

# Using Functions

Functions can be called in any place where an expression is valid.  They
are useful to retrieve data.  Some functions are special and provided
by the engine (like `super`) within certain context, others are global.

The following is a motivating example:

```jinja
<pre>{{ debug() }}</pre>
```

# Custom Functions

A custom global function is just a simple rust function which accepts optional
arguments and then returns a result.  Global functions are typically used to
perform a data loading operation.  For instance these functions can be used
to expose data to the template that hasn't been provided by the individual
render invocation.

```rust
# use minijinja::Environment;
# let mut env = Environment::new();
use minijinja::{Error, ErrorKind};

fn include_file(name: String) -> Result<String, Error> {
    std::fs::read_to_string(&name)
        .map_err(|e| Error::new(
            ErrorKind::InvalidOperation,
            "cannot load file"
        ).with_source(e))
}

env.add_function("include_file", include_file);
```

# Arguments in Custom Functions

All arguments in custom functions must implement the [`ArgType`] trait.
Standard types, such as `String`, `i32`, `bool`, `f64`, etc, already implement this trait.
There are also helper types that will make it easier to extract an arguments with custom types.
The [`ViaDeserialize<T>`](crate::value::ViaDeserialize) type, for instance, can accept any
type `T` that implements the `Deserialize` trait from `serde`.

```rust
# use minijinja::Environment;
# use serde::Deserialize;
# let mut env = Environment::new();
use minijinja::value::ViaDeserialize;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: i32,
}

fn is_adult(person: ViaDeserialize<Person>) -> bool {
    person.age >= 18
}

env.add_function("is_adult", is_adult);
```

# Note on Keyword Arguments

MiniJinja inherits a lot of the runtime model from Jinja2.  That includes support for
keyword arguments.  These however are a concept not native to Rust which makes them
somewhat uncomfortable to work with.  In MiniJinja keyword arguments are implemented by
converting them into an extra parameter represented by a map.  That means if you call
a function as `foo(1, 2, three=3, four=4)` the function gets three arguments:

```json
[1, 2, {"three": 3, "four": 4}]
```

If a function wants to disambiguate between a value passed as keyword argument or not,
the [`Value::is_kwargs`] can be used which returns `true` if a value represents
keyword arguments as opposed to just a map.  A more convenient way to work with keyword
arguments is the [`Kwargs`](crate::value::Kwargs) type.

# Built-in Functions

When the `builtins` feature is enabled a range of built-in functions are
automatically added to the environment.  These are also all provided in
this module.  Note though that these functions are not to be
called from Rust code as their exact interface (arguments and return types)
might change from one MiniJinja version to another.

```rust
pub mod functions { /* ... */ }
```

### Traits

#### Trait `Function`

A utility trait that represents global functions.

This trait is used by the [`add_function`](crate::Environment::add_function)
method to abstract over different types of functions.

Functions which at the very least accept the [`State`] by reference as first
parameter and additionally up to 4 further parameters.  They share much of
their interface with [`filters`](crate::filters).

A function can return any of the following types:

* `Rv` where `Rv` implements `Into<Value>`
* `Result<Rv, Error>` where `Rv` implements `Into<Value>`

The parameters can be marked optional by using `Option<T>`.  The last
argument can also use [`Rest<T>`](crate::value::Rest) to capture the
remaining arguments.  All types are supported for which
[`ArgType`] is implemented.

For a list of built-in functions see [`functions`](crate::functions).

**Note:** this trait cannot be implemented and only exists drive the
functionality of [`add_function`](crate::Environment::add_function)
and [`from_function`](crate::value::Value::from_function).  If you want
to implement a custom callable, you can directly implement
[`Object::call`] which is what the engine actually uses internally.

This trait is also used for [`filters`](crate::filters) and
[`tests`](crate::tests).

# Basic Example

```rust
# use minijinja::Environment;
# let mut env = Environment::new();
use minijinja::{Error, ErrorKind};

fn include_file(name: String) -> Result<String, Error> {
    std::fs::read_to_string(&name)
        .map_err(|e| Error::new(
            ErrorKind::InvalidOperation,
            "cannot load file"
        ).with_source(e))
}

env.add_function("include_file", include_file);
```

```jinja
{{ include_file("filename.txt") }}
```

# Variadic

```
# use minijinja::Environment;
# let mut env = Environment::new();
use minijinja::value::Rest;

fn sum(values: Rest<i64>) -> i64 {
    values.iter().sum()
}

env.add_function("sum", sum);
```

```jinja
{{ sum(1, 2, 3) }} -> 6
```

# Optional Arguments

```jinja
# use minijinja::Environment;
# let mut env = Environment::new();
fn substr(value: String, start: u32, end: Option<u32>) -> String {
    let end = end.unwrap_or(value.len() as _);
    value.get(start as usize..end as usize).unwrap_or_default().into()
}

env.add_filter("substr", substr);
```

```jinja
{{ "Foo Bar Baz"|substr(4) }} -> Bar Baz
{{ "Foo Bar Baz"|substr(4, 7) }} -> Bar
```

```rust
pub trait Function<Rv, Args>: Send + Sync + ''static {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `Func` with <Func, Rv>
- `Func` with <Func, Rv, A>
- `Func` with <Func, Rv, A, B>
- `Func` with <Func, Rv, A, B, C>
- `Func` with <Func, Rv, A, B, C, D>
- `Func` with <Func, Rv, A, B, C, D, E>

### Re-exports

#### Re-export `self::builtins::*`

**Attributes:**

- `#[<cfg>(feature = "builtins")]`

```rust
pub use self::builtins::*;
```

## Module `syntax`

**Attributes:**

- `#[<cfg_attr>(feature = "custom_syntax", doc =
"Documents the syntax for templates and provides ways to reconfigure it.")]`
- `#[<cfg_attr>(not(feature = "custom_syntax"), doc =
"Documents the syntax for templates.")]`
- `#[<cfg_attr>(feature = "custom_syntax", doc =
"- [Custom Delimiters](#custom-delimiters)")]`
- `#[<cfg_attr>(feature = "custom_syntax", doc =
r###"
# Custom Delimiters

When MiniJinja has been compiled with the `custom_syntax` feature (see
[`SyntaxConfig`]), it's possible to reconfigure the delimiters of the
templates.  This is generally not recommended but it's useful for situations
where Jinja templates are used to generate files with a syntax that would be
conflicting for Jinja.  With custom delimiters it can for instance be more
convenient to generate LaTeX files:

```
# use minijinja::{Environment, syntax::SyntaxConfig};
let mut environment = Environment::new();
environment.set_syntax(SyntaxConfig::builder()
    .block_delimiters("\\BLOCK{", "}")
    .variable_delimiters("\\VAR{", "}")
    .comment_delimiters("\\#{", "}")
    .build()
    .unwrap()
);
```

And then a template might look like this instead:

```latex
\begin{itemize}
\BLOCK{for item in sequence}
  \item \VAR{item}
\BLOCK{endfor}
\end{itemize}
```

# Line Statements and Comments

MiniJinja supports line statements and comments like Jinja2 does.  Line statements
and comments are an alternative syntax feature where blocks can be placed on their
own line if they are opened with a configured prefix.  They must appear on their own
line but can be prefixed with whitespace.  Line comments are similar but they can
also be trailing.  These syntax features need to be configured explicitly.
There are however small differences with regards to whitespace compared to
Jinja2.

To use line statements and comments the `custom_syntax` feature needs to be
enabled and they need to be configured (see [`SyntaxConfig`]).

```
# use minijinja::{Environment, syntax::SyntaxConfig};
let mut environment = Environment::new();
environment.set_syntax(SyntaxConfig::builder()
    .line_statement_prefix("#")
    .line_comment_prefix("##")
    .build()
    .unwrap()
);
```

With the above config you can render a template like this:

```jinja
## This block is
## completely removed
<ul>
  # for item in [1, 2]
    ## this is a comment that is also removed including leading whitespace
    <li>{{ item }}
  # endfor
</ul>
```

Renders into the following HTML:

```html
<ul>
    <li>1
    <li>2
</ul>
```

Note that this is slightly different than in Jinja2.  Specifically the following
rules apply with regards to whitespace:

* line statements remove all whitespace before and after including the newline
* line comments remove the trailing newline and leading whitespace.

This is different than in Jinja2 where empty lines can remain from line comments.
Additionally whitespace control is not available for line statements.
"###)]`

Documents the syntax for templates.

<details><summary><strong style="cursor: pointer">Table of Contents</strong></summary>

- [Synopsis](#synopsis)
- [Trailing Newlines](#trailing-newlines)
- [Expressions](#expressions)
  - [Literals](#literals)
  - [Math](#math)
  - [Comparisons](#comparisons)
  - [Logic](#logic)
  - [Other Operators](#other-operators)
  - [If Expressions](#if-expressions)
- [Tags](#tags)
  - [`{% for %}`](#-for-)
  - [`{% if %}`](#-if-)
  - [`{% extends %}`](#-extends-)
  - [`{% block %}`](#-block-)
  - [`{% include %}`](#-include-)
  - [`{% import %}`](#-import-)
  - [`{% with %}`](#-with-)
  - [`{% set %}`](#-set-)
  - [`{% filter %}`](#-filter-)
  - [`{% macro %}`](#-macro-)
  - [`{% call %}`](#-call-)
  - [`{% do %}`](#-do-)
  - [`{% autoescape %}`](#-autoescape-)
  - [`{% raw %}`](#-raw-)
  - [`{% break %} / {% continue %}`](#-break----continue-)
- [Whitespace Control](#whitespace-control)

</details>

# Synopsis

A MiniJinja template is simply a text file.  MiniJinja can generate any text-based
format (HTML, XML, CSV, LaTeX, etc.).  A template doesn’t need to have a specific extension
and in fact MiniJinja does not understand much about the file system.  However the default
configuration for [auto escaping](crate::Environment::set_auto_escape_callback) uses file
extensions to configure the initial behavior.

A template contains [**expressions**](#expressions), which get replaced with values when a
template is rendered; and [**tags**](#tags), which control the logic of the template.  The
template syntax is heavily inspired by Jinja2, Django and Python.

This is a minimal template that illustrates a few basics:

```jinja
<!doctype html>
<title>{% block title %}My Website{% endblock %}</title>
<ul id="navigation">
{% for item in navigation %}
    <li><a href="{{ item.href }}">{{ item.caption }}</a></li>
{% endfor %}
</ul>

<h1>My Webpage</h1>
{% block body %}{% endblock %}

{# a comment #}
```

# Trailing Newlines

MiniJinja, like Jinja2, will remove one trailing newline from the end of the file automatically
on parsing.  This lets templates produce a consistent output no matter if the editor adds a
trailing newline or not.  If one wants a trailing newline an extra newline can be added or the
code rendering it adds it manually.

# Expressions

MiniJinja allows basic expressions everywhere. These work largely as you expect from Jinja2.
Even if you have not used Jinja2 you should feel comfortable with it.  To output the result
of an expression wrap it in `{{ .. }}`.

## Literals

The simplest form of expressions are literals. Literals are representations for
objects such as strings and numbers. The following literals exist:

- `"Hello World"`: Everything between two double or single quotes is a string. They are
  useful whenever you need a string in the template (e.g. as arguments to function calls
  and filters, or just to extend or include a template).
- `42`: Integers are whole numbers without a decimal part.  They can be prefixed with
  `0b` to indicate binary, `0o` to indicate octal and `0x` to indicate hexadecimal.
  Underscores are tolerated (and ignored) everywhere a digit is except in the last place.
- `42.0`: Floating point numbers can be written using a `.` as a decimal mark.
  Underscores are tolerated (and ignored) everywhere a digit is except in the last place.
- `['list', 'of', 'objects']`: Everything between two brackets is a list. Lists are useful
  for storing sequential data to be iterated over.
  for compatibility with Jinja2 `('list', 'of', 'objects')` is also allowed.
- `{'map': 'of', 'key': 'and', 'value': 'pairs'}`: A map is a structure that combines keys
  and values. Keys must be unique and always have exactly one value. Maps are rarely
  created in templates.
- `true` / `false` / `none`: boolean values and the special `none` value which maps to the
  unit type in Rust.

## Math

MiniJinja allows you to calculate with values.  The following operators are supported:

- ``+``: Adds two numbers up. ``{{ 1 + 1 }}`` is ``2``.
- ``-``: Subtract the second number from the first one.  ``{{ 3 - 2 }}`` is ``1``.
- ``/``: Divide two numbers. ``{{ 1 / 2 }}`` is ``0.5``.  See note on divisions below.
- ``//``: Integer divide two numbers. ``{{ 5 // 3 }}`` is ``1``.  See note on divisions below.
- ``%``: Calculate the remainder of an integer division.  ``{{ 11 % 7 }}`` is ``4``.
- ``*``: Multiply the left operand with the right one.  ``{{ 2 * 2 }}`` would return ``4``.
- ``**``: Raise the left operand to the power of the right operand.  ``{{ 2**3 }}``
  would return ``8``.

Note on divisions: divisions in Jinja2 are flooring, divisions in MiniJinja
are at present using euclidean division.  They are almost the same but not quite.

## Comparisons
  
- ``==``: Compares two objects for equality.
- ``!=``: Compares two objects for inequality.
- ``>``: ``true`` if the left hand side is greater than the right hand side.
- ``>=``: ``true`` if the left hand side is greater or equal to the right hand side.
- ``<``:``true`` if the left hand side is less than the right hand side.
- ``<=``: ``true`` if the left hand side is less or equal to the right hand side.

## Logic

For ``if`` statements it can be useful to combine multiple expressions:

- ``and``: Return true if the left and the right operand are true.
- ``or``: Return true if the left or the right operand are true.
- ``not``: negate a statement (see below).
- ``(expr)``: Parentheses group an expression.

## Other Operators

The following operators are very useful but don't fit into any of the other
two categories:

- ``is``/``is not``: Performs a [test](crate::tests).
- ``in``/``not in``: Performs a containment check.
- ``|`` (pipe, vertical bar): Applies a [filter](crate::filters).
- ``~`` (tilde): Converts all operands into strings and concatenates them.
  ``{{ "Hello " ~ name ~ "!" }}`` would return (assuming `name` is set
  to ``'John'``) ``Hello John!``.
- ``()``: Call a callable: ``{{ super() }}``.  Inside of the parentheses you
  can use positional arguments.  Additionally keyword arguments are supported
  which are treated like a dict syntax.  Eg: `foo(a=1, b=2)` is the same as
  `foo({"a": 1, "b": 2})`.
- ``.`` / ``[]``: Get an attribute of an object.  If an object does not have a specific
  attribute or item then `undefined` is returned.  Accessing a property of an already
  undefined value will result in an error.
- ``[start:stop]`` / ``[start:stop:step]``: slices a list or string.  All three expressions
  are optional (`start`, `stop`, `step`).  For instance ``"Hello World"[:5]`` will return
  just `"Hello"`.  Likewise ``"Hello"[1:-1]`` will return `"ell"`.  The step component can
  be used to change the step size.  `"12345"[::2]` will return `"135"`.

### If Expressions

It is also possible to use inline _if_ expressions. These are useful in some situations.
For example, you can use this to extend from one template if a variable is defined,
otherwise from the default layout template:

```jinja
{% extends layout_template if layout_template is defined else 'default.html' %}
```

The `else` part is optional. If not provided, the else block implicitly evaluates
into an undefined value:

```jinja
{{ title|upper if title }}
```

Note that for compatibility with Jinja2, when the `else` block is missing the undefined
value will be marked as "silent".  This means even if strict undefined behavior is
requested, this undefined value will print to an empty string.  This means
that this is always valid:

```jinja
{{ value if false }} -> prints an empty string (silent undefined returned from else)
```

# Tags

Tags control logic in templates.  The following tags exist:

## `{% for %}`

The for tag loops over each item in a sequence.  For example, to display a list
of users provided in a variable called `users`:

```jinja
<h1>Members</h1>
<ul>
{% for user in users %}
  <li>{{ user.username }}</li>
{% endfor %}
</ul>
```

It's also possible to unpack tuples while iterating:

```jinja
<h1>Members</h1>
<ul>
{% for (key, value) in list_of_pairs %}
  <li>{{ key }}: {{ value }}</li>
{% endfor %}
</ul>
```

Inside of the for block you can access some special variables:

- `loop.index`: The current iteration of the loop. (1 indexed)
- `loop.index0`: The current iteration of the loop. (0 indexed)
- `loop.revindex`: The number of iterations from the end of the loop (1 indexed)
- `loop.revindex0`: The number of iterations from the end of the loop (0 indexed)
- `loop.first`: True if this is the first iteration.
- `loop.last`: True if this is the last iteration.
- `loop.length`: The number of items in the sequence.
- `loop.cycle`: A helper function to cycle between a list of sequences. See the explanation below.
- `loop.depth`: Indicates how deep in a recursive loop the rendering currently is. Starts at level 1
- `loop.depth0`: Indicates how deep in a recursive loop the rendering currently is. Starts at level 0
- `loop.previtem`: The item from the previous iteration of the loop. `Undefined` during the first iteration.
- `loop.nextitem`: The item from the previous iteration of the loop. `Undefined` during the last iteration.
- `loop.changed(...args)`: Returns true if the passed values have changed since the last time it was called with the same arguments.
- `loop.cycle(...args)`: Returns a value from the passed sequence in a cycle.

A special note on iterators: in the current version of MiniJinja, some sequences are actually
lazy iterators.  They behave a bit like sequences not not entirely.  They can be iterated over,
will happily serialize once into a a list etc.  However when iterating over an actual iterator,
`last`, `revindex` and `revindex0` will always be undefined.

Within a for-loop, it’s possible to cycle among a list of strings/variables each time through
the loop by using the special `loop.cycle` helper:

```jinja
{% for row in rows %}
  <li class="{{ loop.cycle('odd', 'even') }}">{{ row }}</li>
{% endfor %}
```

A `loop.changed()` helper is also available which can be used to detect when
a value changes between the last iteration and the current one.  The method
takes one or more arguments that are all compared.

```jinja
{% for entry in entries %}
  {% if loop.changed(entry.category) %}
    <h2>{{ entry.category }}</h2>
  {% endif %}
  <p>{{ entry.message }}</p>
{% endfor %}
```

Unlike in Rust or Python, it’s not possible to break or continue in a loop. You can,
however, filter the sequence during iteration, which allows you to skip items.  The
following example skips all the users which are hidden:

```jinja
{% for user in users if not user.hidden %}
  <li>{{ user.username }}</li>
{% endfor %}
```

If no iteration took place because the sequence was empty or the filtering
removed all the items from the sequence, you can render a default block by
using else:

```jinja
<ul>
{% for user in users %}
  <li>{{ user.username }}</li>
{% else %}
  <li><em>no users found</em></li>
{% endfor %}
</ul>
```

It is also possible to use loops recursively. This is useful if you are
dealing with recursive data such as sitemaps. To use loops recursively, you
basically have to add the ``recursive`` modifier to the loop definition and
call the loop variable with the new iterable where you want to recurse.

```jinja
<ul class="menu">
{% for item in menu recursive %}
  <li><a href="{{ item.href }}">{{ item.title }}</a>
  {% if item.children %}
    <ul class="submenu">{{ loop(item.children) }}</ul>
  {% endif %}</li>
{% endfor %}
</ul>
```

**Special note:** the `previtem` and `nextitem` attributes are available by default
but can be disabled by removing the `adjacent_loop_items` crate feature.  Removing
these attributes can provide meaningful speedups for templates with a lot of loops.

## `{% if %}`

The `if` statement is comparable with the Python if statement. In the simplest form,
you can use it to test if a variable is defined, not empty and not false:

```jinja
{% if users %}
  <ul>
  {% for user in users %}
    <li>{{ user.username }}</li>
  {% endfor %}
  </ul>
{% endif %}
```

For multiple branches, `elif` and `else` can be used like in Python.  You can use more
complex expressions there too:

```jinja
{% if kenny.sick %}
  Kenny is sick.
{% elif kenny.dead %}
  You killed Kenny!  You bastard!!!
{% else %}
  Kenny looks okay --- so far
{% endif %}
```

## `{% extends %}`

**Feature:** `multi_template` (included by default)

The `extends` tag can be used to extend one template from another.  You can have multiple
`extends` tags in a file, but only one of them may be executed at a time.  For more
information see [block](#-block-).

## `{% block %}`

Blocks are used for inheritance and act as both placeholders and replacements at the
same time:

The most powerful part of MiniJinja is template inheritance. Template inheritance allows
you to build a base "skeleton" template that contains all the common elements of your
site and defines **blocks** that child templates can override.

**Base Template:**

This template, which we'll call ``base.html``, defines a simple HTML skeleton
document that you might use for a simple two-column page. It's the job of
"child" templates to fill the empty blocks with content:

```jinja
<!doctype html>
{% block head %}
<title>{% block title %}{% endblock %}</title>
{% endblock %}
{% block body %}{% endblock %}
```

**Child Template:**

```jinja
{% extends "base.html" %}
{% block title %}Index{% endblock %}
{% block head %}
  {{ super() }}
  <style type="text/css">
    .important { color: #336699; }
  </style>
{% endblock %}
{% block body %}
  <h1>Index</h1>
  <p class="important">
    Welcome to my awesome homepage.
  </p>
{% endblock %}
```

The ``{% extends %}`` tag is the key here. It tells the template engine that
this template "extends" another template.  When the template system evaluates
this template, it first locates the parent.  The extends tag should be the
first tag in the template.

As you can see it's also possible to render the contents of the parent block by calling
``super()``. You can’t define multiple ``{% block %}`` tags with the same name in
the same template. This limitation exists because a block tag works in “both”
directions. That is, a block tag doesn’t just provide a placeholder to fill -
it also defines the content that fills the placeholder in the parent. If
there were two similarly-named ``{% block %}`` tags in a template, that
template’s parent wouldn’t know which one of the blocks’ content to use.

If you want to print a block multiple times, you can, however, use the
special self variable and call the block with that name:

```jinja
<title>{% block title %}{% endblock %}</title>
<h1>{{ self.title() }}</h1>
{% block body %}{% endblock %}
```

MiniJinja allows you to put the name of the block after the end tag for better
readability:

```jinja
{% block sidebar %}
  {% block inner_sidebar %}
    ...
  {% endblock inner_sidebar %}
{% endblock sidebar %}
```

However, the name after the `endblock` word must match the block name.

## `{% include %}`

**Feature:** `multi_template` (included by default)
  
The `include` tag is useful to include a template and return the rendered contents of that file
into the current namespace:
  
```jinja
{% include 'header.html' %}
  Body
{% include 'footer.html' %}
```

Optionally `ignore missing` can be added in which case non existing templates
are silently ignored.

```jinja
{% include 'customization.html' ignore missing %}
```

You can also provide a list of templates that are checked for existence
before inclusion. The first template that exists will be included. If `ignore
missing` is set, it will fall back to rendering nothing if none of the
templates exist, otherwise it will fail with an error.

```jinja
{% include ['page_detailed.html', 'page.html'] %}
{% include ['special_sidebar.html', 'sidebar.html'] ignore missing %}
```
  
Included templates have access to the variables of the active context.

## `{% import %}`

**Feature:** `multi_template` (included by default)

MiniJinja supports the `{% import %}` and `{% from ... import ... %}`
syntax.  With it variables or macros can be included from other templates:

```jinja
{% from "my_template.html" import my_macro, my_variable %}
```

Imports can also be aliased:

```jinja
{% from "my_template.html" import my_macro as other_name %}
{{ other_name() }}
```

Full modules can be imported with `{% import ... as ... %}`:

```jinja
{% import "my_template.html" as helpers %}
{{ helpers.my_macro() }}
```

Note that unlike Jinja2, exported modules do not contain any template code.  Only
variables and macros that are defined can be imported.  Also imports unlike in Jinja2
are not cached and they get access to the full template context.

## `{% with %}`

The with statement makes it possible to create a new inner scope.  Variables set within
this scope are not visible outside of the scope:

```jinja
{% with foo = 42 %}
  {{ foo }}           foo is 42 here
{% endwith %}
foo is not visible here any longer
```

Multiple variables can be set at once and unpacking is supported:

```jinja
{% with a = 1, (b, c) = [2, 3] %}
  {{ a }}, {{ b }}, {{ c }}  (outputs 1, 2, 3)
{% endwith %}
```

## `{% set %}`

The `set` statement can be used to assign to variables on the same scope.  This is
similar to how `with` works but it won't introduce a new scope.

```jinja
{% set navigation = [('index.html', 'Index'), ('about.html', 'About')] %}
```

Please keep in mind that it is not possible to set variables inside a block
and have them show up outside of it.  This also applies to loops.  The only
exception to that rule are if statements which do not introduce a scope.

It's also possible to capture blocks of template code into a variable by using
the `set` statement as a block.   In that case, instead of using an equals sign
and a value, you just write the variable name and then everything until
`{% endset %}` is captured.

```jinja
{% set navigation %}
<li><a href="/">Index</a>
<li><a href="/downloads">Downloads</a>
{% endset %}
```

The `navigation` variable then contains the navigation HTML source.

This can also be combined with applying a filter:

```jinja
{% set title | upper %}Title of the page{% endset %}
```

More complex use cases can be handled using namespace objects which allow
propagating of changes across scopes:

```jinja
{% set ns = namespace(found=false) %}
{% for item in items %}
  {% if item.check_something() %}
    {% set ns.found = true %}
  {% endif %}
  * {{ item.title }}
{% endfor %}
Found item having something: {{ ns.found }}
```

Note that the `obj.attr` notation in the set tag is only allowed for namespace
objects; attempting to assign an attribute on any other object will cause
an error.

## `{% filter %}`

Filter sections allow you to apply regular [filters](crate::filters) on a
block of template data. Just wrap the code in the special filter block:

```jinja
{% filter upper %}
  This text becomes uppercase
{% endfilter %}
```

## `{% macro %}`

**Feature:** `macros` (included by default)

MiniJinja has limited support for macros.  They allow you to write reusable
template functions.  They are useful to put often used idioms into reusable
functions so you don't repeat yourself (“DRY”).

Here’s a small example of a macro that renders a form element:

```jinja
{% macro input(name, value="", type="text") -%}
<input type="{{ type }}" name="{{ name }}" value="{{ value }}">
{%- endmacro %}
```

The macro can then be called like a function in the namespace:

```jinja
<p>{{ input('username') }}</p>
<p>{{ input('password', type='password') }}</p>
```

The behavior of macros with regards to undefined variables is that they capture
them at macro declaration time (eg: they use a closure).

Macros can be imported via `{% import %}` or `{% from ... import %}`.

Macros also accept a hidden `caller` keyword argument for the use with
`{% call %}`.

## `{% call %}`

**Feature:** `macros` (included by default)

This tag functions similar to a macro that is passed to another macro.  You can
think of it as a way to declare an anonymous macro and pass it to another macro
with the `caller` keyword argument.  The following example shows a macro that
takes advantage of the call functionality and how it can be used:

```jinja
{% macro dialog(title) %}
  <div class="dialog">
    <h3>{{ title }}</h3>
    <div class="contents">{{ caller() }}</div>
  </div>
{% endmacro %}

{% call dialog(title="Hello World") %}
  This is the dialog body.
{% endcall %}
```

<details><summary><strong style="cursor: pointer">Macro Alternative:</strong></summary>

The above example is more or less equivalent with the following:

```jinja
{% macro dialog(title) %}
  <div class="dialog">
    <h3>{{ title }}</h3>
    <div class="contents">{{ caller() }}</div>
  </div>
{% endmacro %}

{% macro caller() %}
  This is the dialog body.
{% endmacro %}
{{ dialog(title="Hello World", caller=caller) }}
```

</details>

It’s also possible to pass arguments back to the call block.  This makes it
useful to build macros that behave like if statements or loops.  Arguments
are placed surrounded in parentheses right after the `call` keyword and
is followed by the macro to be called.

```jinja
{% macro render_user_list(users) %}
<ul>
{% for user in users %}
  <li><p>{{ user.username }}</p>{{ caller(user) }}</li>
{% endfor %}
</ul>
{% endmacro %}

{% call(user) render_user_list(list_of_user) %}
<dl>
  <dt>Name</dt>
  <dd>{{ user.name }}</dd>
  <dt>E-Mail</dt>
  <dd>{{ user.email }}</dd>
</dl>
{% endcall %}
```

## `{% do %}`

The do tag has the same functionality as regular template tags (`{{ ... }}`);
except it doesn't output anything when called.

This is useful if you have a function or macro that has a side-effect, and
you don’t want to display output in the template. The following example
shows a macro that uses the do functionality, and how it can be used:

```jinja
{% macro dialog(title) %}
  Dialog is {{ title }}
{% endmacro %}

{% do dialog(title="Hello World") %} <- does not output anything
```

The above example will not output anything when using the `do` tag.

This tag exists for consistency with Jinja2 and can be useful if you have
custom functionality in templates that uses side-effects.  For instance if
you expose a function to templates that can be used to log warnings:

```jinja
{% for user in users %}
  {% if user.deleted %}
    {% log warn("Found unexpected deleted user in template") %}
  {% endif %}
  ...
{% endfor %}
```

## `{% autoescape %}`

If you want you can activate and deactivate the autoescaping from within
the templates.

Example:

```jinja
{% autoescape true %}
  Autoescaping is active within this block
{% endautoescape %}

{% autoescape false %}
  Autoescaping is inactive within this block
{% endautoescape %}
```

After an `endautoescape` the behavior is reverted to what it was before.

The exact auto escaping behavior is determined by the value of
[`AutoEscape`](crate::AutoEscape) set to the template.

## `{% raw %}`

A raw block is a special construct that lets you ignore the embedded template
syntax.  This is particularly useful if a segment of template code would
otherwise require constant escaping with things like `{{ "{{" }}`:

Example:

```jinja
{% raw %}
<ul>
{% for item in seq %}
    <li>{{ item }}</li>
{% endfor %}
</ul>
{% endraw %}
```

## `{% break %}` / `{% continue %}`

If MiniJinja was compiled with the `loop_controls` feature, it’s possible to
use `break`` and `continue`` in loops.  When break is reached, the loop is
terminated; if continue is reached, the processing is stopped and continues
with the next iteration.

Here’s a loop that skips every second item:

```jinja
{% for user in users %}
{%- if loop.index is even %}{% continue %}{% endif %}
...
{% endfor %}
```

Likewise, a loop that stops processing after the 10th iteration:

```jinja
{% for user in users %}
{%- if loop.index >= 10 %}{% break %}{% endif %}
{%- endfor %}
```

**Note on one-shot iterators:** if you break from a loop but you have
accessed the `loop.nextitem` special variable, then you will lose one item.
This is because accessing that attribute will peak into the iterator and
there is no support for "putting values back".

# Whitespace Control

MiniJinja shares the same behavior with Jinja2 when it comes to
whitespace handling.  By default a single trailing newline is stripped if present
and all other whitespace is returned unchanged.

If an application configures Jinja to [`trim_blocks`](crate::Environment::set_trim_blocks),
the first newline after a template tag is removed automatically (like in PHP).  The
[`lstrip_blocks`](crate::Environment::set_lstrip_blocks) option can also be set to strip
tabs and spaces from the beginning of a line to the start of a block. (Nothing will be
stripped if there are other characters before the start of the block.)

With both `trim_blocks` and `lstrip_blocks` enabled, you can put block tags on their
own lines, and the entire block line will be removed when rendered, preserving the
whitespace of the contents.

For example, without the `trim_blocks` and `lstrip_blocks` options, this template:

```jinja
<div>
  {% if True %}
    yay
  {% endif %}
</div>
```

gets rendered with blank lines inside the div:

```jinja
<div>

    yay

</div>
```

But with both `trim_blocks` and `lstrip_blocks` enabled, the template block lines
are removed and other whitespace is preserved:

```jinja
<div>
    yay
</div>
````

You can manually disable the `lstrip_blocks` behavior by putting a plus sign (`+`)
at the start of a block:

```jinja
<div>
  {%+ if something %}yay{% endif %}
</div>
```

Similarly, you can manually disable the `trim_blocks` behavior by putting a plus
sign (`+`) at the end of a block:

```jinja
<div>
{% if something +%}
    yay
{% endif %}
</div>
```

You can also strip whitespace in templates by hand. If you add a minus sign (`-`) to the
start or end of a block (e.g. a for tag), a comment, or a variable expression, the
whitespaces before or after that block will be removed:

```jinja
{% for item in range(1, 10) -%}
{{ item }}
{%- endfor %}
```

This will yield all elements without whitespace between them, in this case
the output would be `123456789`.

By default, MiniJinja also removes trailing newlines.  To keep single trailing newlines,
configure MiniJinja to [`keep_trailing_newline`](crate::Environment::set_keep_trailing_newline).

```rust
pub mod syntax { /* ... */ }
```

### Re-exports

#### Re-export `self::imp::*`

```rust
pub use self::imp::*;
```

## Module `tests`

Test functions and abstractions.

Test functions in MiniJinja are like [`filters`](crate::filters) but a
different syntax is used to invoke them and they have to return boolean
values.  For instance the expression `{% if foo is defined %}` invokes the
[`is_defined`] test to check if the value is indeed an odd number.

MiniJinja comes with some built-in test functions that are listed below. To
create a custom test write a function that takes at least a value argument
that returns a boolean result, then register it with
[`add_filter`](crate::Environment::add_test).

# Using Tests

Tests are useful to "test" a value in a specific way.  For instance if
you want to assign different classes to alternating rows one way is
using the `odd` test:

```jinja
{% if seq is defined %}
  <ul>
  {% for item in seq %}
    <li class="{{ 'even' if loop.index is even else 'odd' }}">{{ item }}</li>
  {% endfor %}
  </ul>
{% endif %}
```

# Custom Tests

A custom test function is just a simple function which accepts its
inputs as parameters and then returns a bool.  For instance the following
shows a test function which takes an input value and checks if it's
lowercase:

```
# use minijinja::Environment;
# let mut env = Environment::new();
fn is_lowercase(value: String) -> bool {
    value.chars().all(|x| x.is_lowercase())
}

env.add_test("lowercase", is_lowercase);
```

MiniJinja will perform the necessary conversions automatically.  For more
information see the [`Function`](crate::functions::Function) trait.  If a
test returns a value that is not a bool, it will be evaluated for truthiness
with [`Value::is_true`].

# Built-in Tests

When the `builtins` feature is enabled a range of built-in tests are
automatically added to the environment.  These are also all provided in
this module.  Note though that these functions are not to be
called from Rust code as their exact interface (arguments and return types)
might change from one MiniJinja version to another.

```rust
pub mod tests { /* ... */ }
```

### Functions

#### Function `is_undefined`

Checks if a value is undefined.

```jinja
{{ 42 is undefined }} -> false
```

```rust
pub fn is_undefined(v: &crate::value::Value) -> bool { /* ... */ }
```

#### Function `is_defined`

Checks if a value is defined.

```jinja
{{ 42 is defined }} -> true
```

```rust
pub fn is_defined(v: &crate::value::Value) -> bool { /* ... */ }
```

#### Function `is_none`

Checks if a value is none.

```jinja
{{ none is none }} -> true
```

```rust
pub fn is_none(v: &crate::value::Value) -> bool { /* ... */ }
```

#### Function `is_safe`

Checks if a value is safe.

```jinja
{{ "<hello>"|escape is safe }} -> true
```

This filter is also registered with the `escaped` alias.

```rust
pub fn is_safe(v: &crate::value::Value) -> bool { /* ... */ }
```

### Re-exports

#### Re-export `self::builtins::*`

**Attributes:**

- `#[<cfg>(feature = "builtins")]`

```rust
pub use self::builtins::*;
```

## Module `value`

**Attributes:**

- `#[<cfg_attr>(feature = "deserialization", doc =
r"
```
# use minijinja::value::Value;
use serde::Deserialize;
let value = Value::from(vec![1, 2, 3]);
let vec = Vec::<i32>::deserialize(value).unwrap();
```
")]`

Provides a dynamic value type abstraction.

This module gives access to a dynamically typed value which is used by
the template engine during execution.

For the most part the existence of the value type can be ignored as
MiniJinja will perform the necessary conversions for you.  For instance
if you write a filter that converts a string you can directly declare the
filter to take a [`String`].  However for some more advanced use cases it's
useful to know that this type exists.

# Basic Value Conversions

Values are typically created via the [`From`] trait:

```
use std::collections::BTreeMap;
# use minijinja::value::Value;
let int_value = Value::from(42);
let none_value = Value::from(());
let true_value = Value::from(true);
let map = Value::from({
    let mut m = BTreeMap::new();
    m.insert("foo", 1);
    m.insert("bar", 2);
    m
});
```

Or via the [`FromIterator`] trait which can create sequences or maps.  When
given a tuple it creates maps, otherwise it makes a sequence.

```
# use minijinja::value::Value;
// collection into a sequence
let value: Value = (1..10).into_iter().collect();

// collection into a map
let value: Value = [("key", "value")].into_iter().collect();
```

For certain types of iterators (`Send` + `Sync` + `'static`) it's also
possible to make the value lazily iterate over the value by using the
[`Value::make_iterable`] function instead.  Whenever the value requires
iteration, the function is called to create that iterator.

```
# use minijinja::value::Value;
let value: Value = Value::make_iterable(|| 1..10);
```

To to into the inverse directly the various [`TryFrom`]
implementations can be used:

```
# use minijinja::value::Value;
use std::convert::TryFrom;
let v = u64::try_from(Value::from(42)).unwrap();
```

The special [`Undefined`](Value::UNDEFINED) value also exists but does not
have a rust equivalent.  It can be created via the [`UNDEFINED`](Value::UNDEFINED)
constant.

# Collections

The standard library's collection types such as
[`HashMap`](std::collections::HashMap), [`Vec`] and various others from the
collections module are implemented are objects.  There is a cavet here which is
that maps can only have string or [`Value`] as key.  The values in the collections
are lazily converted into value when accessed or iterated over.   These types can
be constructed either from [`Value::from`] or [`Value::from_object`].  Because the
types are boxed unchanged, you can also downcast them.

```rust
# use minijinja::Value;
let vec = Value::from(vec![1i32, 2, 3, 4]);
let vec_ref = vec.downcast_object_ref::<Vec<i32>>().unwrap();
assert_eq!(vec_ref, &vec![1, 2, 3, 4]);
```

**Caveat:** for convenience reasons maps with `&str` keys can be stored.  The keys
however are converted into `Arc<str>`.

# Serde Conversions

MiniJinja will usually however create values via an indirection via [`serde`] when
a template is rendered or an expression is evaluated.  This can also be
triggered manually by using the [`Value::from_serialize`] method:

```
# use minijinja::value::Value;
let value = Value::from_serialize(&[1, 2, 3]);
```

The inverse of that operation is to pass a value directly as serializer to
a type that supports deserialization.  This requires the `deserialization`
feature.

```
# use minijinja::value::Value;
use serde::Deserialize;
let value = Value::from(vec![1, 2, 3]);
let vec = Vec::<i32>::deserialize(value).unwrap();
```

# Value Function Arguments

[Filters](crate::filters) and [tests](crate::tests) can take values as arguments
but optionally also rust types directly.  This conversion for function arguments
is performed by the [`FunctionArgs`] and related traits ([`ArgType`], [`FunctionResult`]).

# Memory Management

Values are immutable objects which are internally reference counted which
means they can be copied relatively cheaply.  Special care must be taken
so that cycles are not created to avoid causing memory leaks.

# HTML Escaping

MiniJinja inherits the general desire to be clever about escaping.  For this
purpose a value will (when auto escaping is enabled) always be escaped.  To
prevent this behavior the [`safe`](crate::filters::safe) filter can be used
in the template.  Outside of templates the [`Value::from_safe_string`] method
can be used to achieve the same result.

# Dynamic Objects

Values can also hold "dynamic" objects.  These are objects which implement the
[`Object`] trait.  These can be used to implement dynamic functionality such
as stateful values and more.  Dynamic objects are internally also used to
implement the special `loop` variable, macros and similar things.

To create a [`Value`] from a dynamic object use [`Value::from_object`],
[`Value::from_dyn_object`]:

```rust
# use std::sync::Arc;
# use minijinja::value::{Value, Object, DynObject};
#[derive(Debug)]
struct Foo;

impl Object for Foo {
    /* implementation */
}

let value = Value::from_object(Foo);
let value = Value::from_dyn_object(Arc::new(Foo));
```

# Invalid Values

MiniJinja knows the concept of an "invalid value".  These are rare in practice
and should not be used, but they are needed in some situations.  An invalid value
looks like a value but working with that value in the context of the engine will
fail in most situations.  In principle an invalid value is a value that holds an
error internally.  It's created with [`From`]:

```
use minijinja::{Value, Error, ErrorKind};
let error = Error::new(ErrorKind::InvalidOperation, "failed to generate an item");
let invalid_value = Value::from(error);
```

Invalid values are typically encountered in the following situations:

- serialization fails with an error: this is the case when a value is crated
  via [`Value::from_serialize`] and the underlying [`Serialize`] implementation
  fails with an error.
- fallible iteration: there might be situations where an iterator cannot indicate
  failure ahead of iteration and must abort.  In that case the only option an
  iterator in MiniJinja has is to create an invalid value.

It's generally recommende to ignore the existence of invalid objects and let them
fail naturally as they are encountered.

# Notes on Bytes and Strings

Usually one would pass strings to templates as Jinja is entirely based on string
rendering.  However there are situations where it can be useful to pass bytes instead.
As such MiniJinja allows a value type to carry bytes even though there is no syntax
within the template language to create a byte literal.

When rendering bytes as strings, MiniJinja will attempt to interpret them as
lossy utf-8.  This is a bit different to Jinja2 which in Python 3 stopped
rendering byte strings as strings.  This is an intentional change that was
deemed acceptable given how infrequently bytes are used but how relatively
commonly bytes are often holding "almost utf-8" in templates.  Most
conversions to strings also will do almost the same.  The debug rendering of
bytes however is different and bytes are not iterable.  Like strings however
they can be sliced and indexed, but they will be sliced by bytes and not by
characters.

```rust
pub mod value { /* ... */ }
```

### Types

#### Enum `ValueKind`

**Attributes:**

- `#[non_exhaustive]`

Describes the kind of value.

```rust
pub enum ValueKind {
    Undefined,
    None,
    Bool,
    Number,
    String,
    Bytes,
    Seq,
    Map,
    Iterable,
    Plain,
    Invalid,
}
```

##### Variants

###### `Undefined`

The value is undefined

###### `None`

The value is the none singleton (`()`)

###### `Bool`

The value is a [`bool`]

###### `Number`

The value is a number of a supported type.

###### `String`

The value is a string.

###### `Bytes`

The value is a byte array.

###### `Seq`

The value is an array of other values.

###### `Map`

The value is a key/value mapping.

###### `Iterable`

An iterable

###### `Plain`

A plain object without specific behavior.

###### `Invalid`

This value is invalid (holds an error).

This can happen when a serialization error occurred or the engine
encountered a failure in a place where an error can otherwise not
be produced.  Interacting with such values in the context of the
template evaluation process will attempt to propagate the error.

##### Implementations

###### Trait Implementations

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ValueKind) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ValueKind { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ValueKind) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **StructuralPartialEq**
- **Freeze**
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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ValueKind) -> $crate::cmp::Ordering { /* ... */ }
    ```

#### Struct `Value`

Represents a dynamically typed value in the template engine.

```rust
pub struct Value(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_serialize<T: Serialize>(value: T) -> Value { /* ... */ }
  ```
  Creates a value from something that can be serialized.

- ```rust
  pub fn from_safe_string(value: String) -> Value { /* ... */ }
  ```
  Creates a value from a safe string.

- ```rust
  pub fn from_bytes(value: Vec<u8>) -> Value { /* ... */ }
  ```
  Creates a value from a byte vector.

- ```rust
  pub fn from_object<T: Object + Send + Sync + ''static>(value: T) -> Value { /* ... */ }
  ```
  Creates a value from a dynamic object.

- ```rust
  pub fn from_dyn_object<T: Into<DynObject>>(value: T) -> Value { /* ... */ }
  ```
  Like [`from_object`](Self::from_object) but for type erased dynamic objects.

- ```rust
  pub fn make_iterable<I, T, F>(maker: F) -> Value
where
    I: Iterator<Item = T> + Send + Sync + ''static,
    T: Into<Value> + Send + Sync + ''static,
    F: Fn() -> I + Send + Sync + ''static { /* ... */ }
  ```
  Creates a value that is an iterable.

- ```rust
  pub fn make_object_iterable<T, F>(object: T, maker: F) -> Value
where
    T: Send + Sync + ''static,
    F: for<''a> Fn(&''a T) -> Box<dyn Iterator<Item = Value> + Send + Sync + ''a> + Send + Sync + ''static { /* ... */ }
  ```
  Creates an iterable that iterates over the given value.

- ```rust
  pub fn make_object_map<T, E, A>(object: T, enumerate_fn: E, attr_fn: A) -> Value
where
    T: Send + Sync + ''static,
    E: for<''a> Fn(&''a T) -> Box<dyn Iterator<Item = Value> + Send + Sync + ''a> + Send + Sync + ''static,
    A: Fn(&T, &Value) -> Option<Value> + Send + Sync + ''static { /* ... */ }
  ```
  Creates an object projection onto a map.

- ```rust
  pub fn make_one_shot_iterator<I, T>(iter: I) -> Value
where
    I: Iterator<Item = T> + Send + Sync + ''static,
    T: Into<Value> + Send + Sync + ''static { /* ... */ }
  ```
  Creates a value from a one-shot iterator.

- ```rust
  pub fn from_function<F, Rv, Args>(f: F) -> Value
where
    F: functions::Function<Rv, Args> + for<''a> functions::Function<Rv, <Args as FunctionArgs<''a>>::Output>,
    Rv: FunctionResult,
    Args: for<''a> FunctionArgs<''a> { /* ... */ }
  ```
  Creates a callable value from a function.

- ```rust
  pub fn kind(self: &Self) -> ValueKind { /* ... */ }
  ```
  Returns the kind of the value.

- ```rust
  pub fn is_number(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the value is a number.

- ```rust
  pub fn is_integer(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the number is a real integer.

- ```rust
  pub fn is_kwargs(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the map represents keyword arguments.

- ```rust
  pub fn is_true(self: &Self) -> bool { /* ... */ }
  ```
  Is this value considered true?

- ```rust
  pub fn is_safe(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this value is safe.

- ```rust
  pub fn is_undefined(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this value is undefined.

- ```rust
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this value is none.

- ```rust
  pub fn to_str(self: &Self) -> Option<Arc<str>> { /* ... */ }
  ```
  If the value is a string, return it.

- ```rust
  pub fn as_str(self: &Self) -> Option<&str> { /* ... */ }
  ```
  If the value is a string, return it.

- ```rust
  pub fn as_usize(self: &Self) -> Option<usize> { /* ... */ }
  ```
  If this is an usize return it

- ```rust
  pub fn as_i64(self: &Self) -> Option<i64> { /* ... */ }
  ```
  If this is an i64 return it

- ```rust
  pub fn as_bytes(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
  Returns the bytes of this value if they exist.

- ```rust
  pub fn as_object(self: &Self) -> Option<&DynObject> { /* ... */ }
  ```
  If the value is an object a reference to it is returned.

- ```rust
  pub fn len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length of the contained value.

- ```rust
  pub fn get_attr(self: &Self, key: &str) -> Result<Value, Error> { /* ... */ }
  ```
  Looks up an attribute by attribute name.

- ```rust
  pub fn get_item_by_index(self: &Self, idx: usize) -> Result<Value, Error> { /* ... */ }
  ```
  Looks up an index of the value.

- ```rust
  pub fn get_item(self: &Self, key: &Value) -> Result<Value, Error> { /* ... */ }
  ```
  Looks up an item (or attribute) by key.

- ```rust
  pub fn try_iter(self: &Self) -> Result<ValueIter, Error> { /* ... */ }
  ```
  Iterates over the value.

- ```rust
  pub fn reverse(self: &Self) -> Result<Value, Error> { /* ... */ }
  ```
  Returns a reversed view of this value.

- ```rust
  pub fn downcast_object_ref<T: ''static>(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Returns some reference to the boxed object if it is of type `T`, or None if it isn’t.

- ```rust
  pub fn downcast_object<T: ''static>(self: &Self) -> Option<Arc<T>> { /* ... */ }
  ```
  Like [`downcast_object_ref`](Self::downcast_object_ref) but returns

- ```rust
  pub fn call(self: &Self, state: &State<''_, ''_>, args: &[Value]) -> Result<Value, Error> { /* ... */ }
  ```
  Calls the value directly.

- ```rust
  pub fn call_method(self: &Self, state: &State<''_, ''_>, name: &str, args: &[Value]) -> Result<Value, Error> { /* ... */ }
  ```
  Calls a method on the value.

###### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **FunctionResult**
  - ```rust
    fn into_result(self: Self) -> Result<Value, Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Deserializer**
  - ```rust
    fn deserialize_any<V: Visitor<''de>>(self: Self, visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V: Visitor<''de>>(self: Self, visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V: Visitor<''de>>(self: Self, _name: &''static str, _variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V: Visitor<''de>>(self: Self, _name: &''static str, visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V: Visitor<''de>>(self: Self, _name: &''static str, visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_any<V: Visitor<''de>>(self: Self, visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V: Visitor<''de>>(self: Self, visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V: Visitor<''de>>(self: Self, name: &''static str, variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V: Visitor<''de>>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V: Visitor<''de>>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value, Error> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self { /* ... */ }
    ```

  - ```rust
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Value { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ArgType**
- **Eq**
- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(val: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: &''a str) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: &''a String) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: String) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: Cow<''a, str>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Arc<str>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(_: ()) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: i128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: u128) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: char) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: bool) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: u8) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: i8) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: Arc<Vec<u8>>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: DynObject) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Kwargs) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: Value) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: usize) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: isize) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<I>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: Vec<T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: BTreeMap<Value, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: BTreeMap<String, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: BTreeMap<Arc<str>, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: BTreeMap<&''a str, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: BTreeMap<Cow<''a, str>, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: LinkedList<T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: HashSet<T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: BTreeSet<T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: HashMap<String, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: HashMap<Arc<str>, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: HashMap<&''a str, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: HashMap<Cow<''a, str>, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: HashMap<Value, V>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(val: VecDeque<T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Error) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: Value) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Unpin**
- **Deserialize**
  - ```rust
    fn deserialize<D: Deserializer<''de>>(deserializer: D) -> Result<Self, <D as >::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), std::fmt::Error> { /* ... */ }
    ```

- **IntoDeserializer**
  - ```rust
    fn into_deserializer(self: Self) -> Value { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> &''de Value { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Value { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `ValueIter`

Utility to iterate over values.

```rust
pub struct ValueIter {
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

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `serializing_for_value`

Function that returns true when serialization for [`Value`] is taking place.

MiniJinja internally creates [`Value`] objects from all values passed to the
engine.  It does this by going through the regular serde serialization trait.
In some cases users might want to customize the serialization specifically for
MiniJinja because they want to tune the object for the template engine
independently of what is normally serialized to disk.

This function returns `true` when MiniJinja is serializing to [`Value`] and
`false` otherwise.  You can call this within your own [`Serialize`]
implementation to change the output format.

This is particularly useful as serialization for MiniJinja does not need to
support deserialization.  So it becomes possible to completely change what
gets sent there, even at the cost of serializing something that cannot be
deserialized.

```rust
pub fn serializing_for_value() -> bool { /* ... */ }
```

### Re-exports

#### Re-export `from_args`

```rust
pub use crate::value::argtypes::from_args;
```

#### Re-export `ArgType`

```rust
pub use crate::value::argtypes::ArgType;
```

#### Re-export `FunctionArgs`

```rust
pub use crate::value::argtypes::FunctionArgs;
```

#### Re-export `FunctionResult`

```rust
pub use crate::value::argtypes::FunctionResult;
```

#### Re-export `Kwargs`

```rust
pub use crate::value::argtypes::Kwargs;
```

#### Re-export `Rest`

```rust
pub use crate::value::argtypes::Rest;
```

#### Re-export `merge_maps`

```rust
pub use crate::value::merge_object::merge_maps;
```

#### Re-export `DynObject`

```rust
pub use crate::value::object::DynObject;
```

#### Re-export `Enumerator`

```rust
pub use crate::value::object::Enumerator;
```

#### Re-export `Object`

```rust
pub use crate::value::object::Object;
```

#### Re-export `ObjectExt`

```rust
pub use crate::value::object::ObjectExt;
```

#### Re-export `ObjectRepr`

```rust
pub use crate::value::object::ObjectRepr;
```

#### Re-export `ViaDeserialize`

**Attributes:**

- `#[<cfg>(feature = "deserialization")]`

```rust
pub use self::deserialize::ViaDeserialize;
```

## Macros

### Macro `context`

**Attributes:**

- `#[macro_export]`

Creates a template context from keys and values or merging in another value.

```rust
# use minijinja::context;
let ctx = context!{
    name => "Peter",
    location => "World",
};
```

Alternatively if the variable name matches the key name it can
be omitted:

```rust
# use minijinja::context;
let name = "Peter";
let ctx = context!{ name };
```

The return value is a [`Value`](crate::value::Value).

Note that [`context!`](crate::context!) can also be used recursively if you need to
create nested objects:

```rust
# use minijinja::context;
let ctx = context! {
    nav => vec![
        context!(path => "/", title => "Index"),
        context!(path => "/downloads", title => "Downloads"),
        context!(path => "/faq", title => "FAQ"),
    ]
};
```

Additionally the macro supports a second syntax that can merge other
contexts or values.  In that case one or more values need to be
passed with a leading `..` operator.  This is useful to supply extra
values into render in a common place.  The order of precedence is
left to right:

```rust
# use minijinja::context;
let ctx = context! { a => "A" };
let ctx = context! { ..ctx, ..context! {
    b => "B"
}};

// or

let ctx = context! {
    a => "A",
    ..context! {
        b => "B"
    }
};
```

The merge works with an value, not just values created by the `context!`
macro and is performed lazy.  This means it also works with dynamic
[`Object`](crate::value::Object)s.  The merge uses the underlying
[`merge_maps`](crate::value::merge_maps) function.

# Note on Conversions

This macro uses [`Value::from_serialize`](crate::Value::from_serialize)
for conversions.

This macro currently does not move passed values.  Future versions of
MiniJinja are going to change the move behavior and it's recommended to not
depend on this implicit reference behavior.  You should thus pass values
with `&value` if you intend on still being able to reference them
after the macro invocation.

```rust
pub macro_rules! context {
    /* macro_rules! context {
    () => { ... };
    (
        $($key:ident $(=> $value:expr)?),*
        $(, .. $ctx:expr),* $(,)?
    ) => { ... };
    (
        $(.. $ctx:expr),* $(,)?
    ) => { ... };
} */
}
```

### Macro `args`

**Attributes:**

- `#[macro_export]`

An utility macro to create arguments for function calls.

This creates a slice of values on the stack which can be
passed to [`call`](crate::value::Value::call),
[`call_method`](crate::value::Value::call_method),
[`apply_filter`](crate::State::apply_filter),
[`perform_test`](crate::State::perform_test) or similar
APIs that take slices of values.

It supports both positional and keyword arguments.
To mark a parameter as keyword argument define it as
`name => value`, otherwise just use `value`.

```
# use minijinja::{value::Value, args, Environment};
# let env = Environment::default();
# let state = &env.empty_state();
# let value = Value::from(());
value.call(state, args!(1, 2, foo => "bar"));
```

Note that this like [`context!`](crate::context) goes through
[`Value::from_serialize`](crate::value::Value::from_serialize).

```rust
pub macro_rules! args {
    /* macro_rules! args {
    () => { ... };
    ($($arg:tt)*) => { ... };
} */
}
```

### Macro `render`

**Attributes:**

- `#[macro_export]`

A macro similar to [`format!`] but that uses MiniJinja for rendering.

This can be used to quickly render a MiniJinja template into a string
without having to create an environment first which can be useful in
some situations.  Note however that the template is re-parsed every
time the [`render!`](crate::render) macro is called which is potentially
slow.

There are two forms for this macro.  The default form takes template
source and context variables, the extended form also lets you provide
a custom environment that should be used rather than a default one.
The context variables are passed the same way as with the
[`context!`](crate::context) macro.

# Example

Passing context explicitly:

```
# use minijinja::render;
println!("{}", render!("Hello {{ name }}!", name => "World"));
```

Passing variables with the default name:

```
# use minijinja::render;
let name = "World";
println!("{}", render!("Hello {{ name }}!", name));
```

Passing an explicit environment:

```
# use minijinja::{Environment, render};
let env = Environment::new();
println!("{}", render!(in env, "Hello {{ name }}!", name => "World"));
```

# Panics

This macro panics if the format string is an invalid template or the
template evaluation failed.

```rust
pub macro_rules! render {
    /* macro_rules! render {
    (
        in $env:expr,
        $tmpl:expr
        $(, $key:ident $(=> $value:expr)?)* $(,)?
    ) => { ... };
    (
        $tmpl:expr
        $(, $key:ident $(=> $value:expr)?)* $(,)?
    ) => { ... };
} */
}
```

## Re-exports

### Re-export `default_auto_escape_callback`

```rust
pub use self::defaults::default_auto_escape_callback;
```

### Re-export `escape_formatter`

```rust
pub use self::defaults::escape_formatter;
```

### Re-export `Environment`

```rust
pub use self::environment::Environment;
```

### Re-export `Error`

```rust
pub use self::error::Error;
```

### Re-export `ErrorKind`

```rust
pub use self::error::ErrorKind;
```

### Re-export `Expression`

```rust
pub use self::expression::Expression;
```

### Re-export `Output`

```rust
pub use self::output::Output;
```

### Re-export `Template`

```rust
pub use self::template::Template;
```

### Re-export `AutoEscape`

```rust
pub use self::utils::AutoEscape;
```

### Re-export `HtmlEscape`

```rust
pub use self::utils::HtmlEscape;
```

### Re-export `UndefinedBehavior`

```rust
pub use self::utils::UndefinedBehavior;
```

### Re-export `Value`

Re-export for convenience.

```rust
pub use self::value::Value;
```

### Re-export `State`

```rust
pub use self::vm::State;
```

