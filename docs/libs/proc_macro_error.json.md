# Crate Documentation

**Version:** 1.0.4

**Format Version:** 43

# Module `proc_macro_error`

# proc-macro-error

This crate aims to make error reporting in proc-macros simple and easy to use.
Migrate from `panic!`-based errors for as little effort as possible!

(Also, you can explicitly [append a dummy token stream](dummy/index.html) to your errors).

To achieve his, this crate serves as a tiny shim around `proc_macro::Diagnostic` and
`compile_error!`. It detects the best way of emitting available based on compiler's version.
When the underlying diagnostic type is finally stabilized, this crate will simply be
delegating to it requiring no changes in your code!

So you can just use this crate and have *both* some of `proc_macro::Diagnostic` functionality
available on stable ahead of time *and* your error-reporting code future-proof.

## Cargo features

This crate provides *enabled by default* `syn-error` feature that gates
`impl From<syn::Error> for Diagnostic` conversion. If you don't use `syn` and want
to cut off some of compilation time, you can disable it via

```toml
[dependencies]
proc-macro-error = { version = "1", default-features = false }
```

***Please note that disabling this feature makes sense only if you don't depend on `syn`
directly or indirectly, and you very likely do.**

## Real world examples

* [`structopt-derive`](https://github.com/TeXitoi/structopt/tree/master/structopt-derive)
  (abort-like usage)
* [`auto-impl`](https://github.com/auto-impl-rs/auto_impl/) (emit-like usage)

## Limitations

- Warnings are emitted only on nightly, they are ignored on stable.
- "help" suggestions can't have their own span info on stable,
  (essentially inheriting the parent span).
- If a panic occurs somewhere in your macro no errors will be displayed. This is not a
  technical limitation but rather intentional design. `panic` is not for error reporting.

### `#[proc_macro_error]` attribute

**This attribute MUST be present on the top level of your macro** (the function
annotated with any of `#[proc_macro]`, `#[proc_macro_derive]`, `#[proc_macro_attribute]`).

This attribute performs the setup and cleanup necessary to make things work.

In most cases you'll need the simple `#[proc_macro_error]` form without any
additional settings. Feel free to [skip the "Syntax" section](#macros).

#### Syntax

`#[proc_macro_error]` or `#[proc_macro_error(settings...)]`, where `settings...`
is a comma-separated list of:

- `proc_macro_hack`:

    In order to correctly cooperate with `#[proc_macro_hack]`, `#[proc_macro_error]`
    attribute must be placed *before* (above) it, like this:

    ```no_run
    # use proc_macro2::TokenStream;
    # const IGNORE: &str = "
    #[proc_macro_error]
    #[proc_macro_hack]
    #[proc_macro]
    # ";
    fn my_macro(input: TokenStream) -> TokenStream {
        unimplemented!()
    }
    ```

    If, for some reason, you can't place it like that you can use
    `#[proc_macro_error(proc_macro_hack)]` instead.

    # Note

    If `proc-macro-hack` was detected (by any means) `allow_not_macro`
    and `assert_unwind_safe` will be applied automatically.

- `allow_not_macro`:

    By default, the attribute checks that it's applied to a proc-macro.
    If none of `#[proc_macro]`, `#[proc_macro_derive]` nor `#[proc_macro_attribute]` are
    present it will panic. It's the intention - this crate is supposed to be used only with
    proc-macros.

    This setting is made to bypass the check, useful in certain circumstances.

    Pay attention: the function this attribute is applied to must return
    `proc_macro::TokenStream`.

    This setting is implied if `proc-macro-hack` was detected.

- `assert_unwind_safe`:

    By default, your code must be [unwind safe]. If your code is not unwind safe,
    but you believe it's correct, you can use this setting to bypass the check.
    You would need this for code that uses `lazy_static` or `thread_local` with
    `Cell/RefCell` inside (and the like).

    This setting is implied if `#[proc_macro_error]` is applied to a function
    marked as `#[proc_macro]`, `#[proc_macro_derive]` or `#[proc_macro_attribute]`.

    This setting is also implied if `proc-macro-hack` was detected.

## Macros

Most of the time you want to use the macros. Syntax is described in the next section below.

You'll need to decide how you want to emit errors:

* Emit the error and abort. Very much panic-like usage. Served by [`abort!`] and
  [`abort_call_site!`].
* Emit the error but do not abort right away, looking for other errors to report.
  Served by [`emit_error!`] and [`emit_call_site_error!`].

You **can** mix these usages.

`abort` and `emit_error` take a "source span" as the first argument. This source
will be used to highlight the place the error originates from. It must be one of:

* *Something* that implements [`ToTokens`] (most types in `syn` and `proc-macro2` do).
  This source is the preferable one since it doesn't lose span information on multi-token
  spans, see [this issue](https://gitlab.com/CreepySkeleton/proc-macro-error/-/issues/6)
  for details.
* [`proc_macro::Span`]
* [`proc-macro2::Span`]

The rest is your message in format-like style.

See [the next section](#syntax-1) for detailed syntax.

- [`abort!`]:

    Very much panic-like usage - abort right away and show the error.
    Expands to [`!`] (never type).

- [`abort_call_site!`]:

    Shortcut for `abort!(Span::call_site(), ...)`. Expands to [`!`] (never type).

- [`emit_error!`]:

    [`proc_macro::Diagnostic`]-like usage - emit the error but keep going,
    looking for other errors to report.
    The compilation will fail nonetheless. Expands to [`()`] (unit type).

- [`emit_call_site_error!`]:

    Shortcut for `emit_error!(Span::call_site(), ...)`. Expands to [`()`] (unit type).

- [`emit_warning!`]:

    Like `emit_error!` but emit a warning instead of error. The compilation won't fail
    because of warnings.
    Expands to [`()`] (unit type).

    **Beware**: warnings are nightly only, they are completely ignored on stable.

- [`emit_call_site_warning!`]:

    Shortcut for `emit_warning!(Span::call_site(), ...)`. Expands to [`()`] (unit type).

- [`diagnostic`]:

    Build an instance of `Diagnostic` in format-like style.

#### Syntax

All the macros have pretty much the same syntax:

1.  ```ignore
    abort!(single_expr)
    ```
    Shortcut for `Diagnostic::from(expr).abort()`.

2.  ```ignore
    abort!(span, message)
    ```
    The first argument is an expression the span info should be taken from.

    The second argument is the error message, it must implement [`ToString`].

3.  ```ignore
    abort!(span, format_literal, format_args...)
    ```

    This form is pretty much the same as 2, except `format!(format_literal, format_args...)`
    will be used to for the message instead of [`ToString`].

That's it. `abort!`, `emit_warning`, `emit_error` share this exact syntax.

`abort_call_site!`, `emit_call_site_warning`, `emit_call_site_error` lack 1 form
and do not take span in 2'th and 3'th forms. Those are essentially shortcuts for
`macro!(Span::call_site(), args...)`.

`diagnostic!` requires a [`Level`] instance between `span` and second argument
(1'th form is the same).

> **Important!**
>
> If you have some type from `proc_macro` or `syn` to point to, do not call `.span()`
> on it but rather use it directly:
> ```no_run
> # use proc_macro_error::abort;
> # let input = proc_macro2::TokenStream::new();
> let ty: syn::Type = syn::parse2(input).unwrap();
> abort!(ty, "BOOM");
> //     ^^ <-- avoid .span()
> ```
>
> `.span()` calls work too, but you may experience regressions in message quality.

#### Note attachments

3.  Every macro can have "note" attachments (only 2 and 3 form).
  ```ignore
  let opt_help = if have_some_info { Some("did you mean `this`?") } else { None };

  abort!(
      span, message; // <--- attachments start with `;` (semicolon)

      help = "format {} {}", "arg1", "arg2"; // <--- every attachment ends with `;`,
                                             //      maybe except the last one

      note = "to_string"; // <--- one arg uses `.to_string()` instead of `format!()`

      yay = "I see what {} did here", "you"; // <--- "help =" and "hint =" are mapped
                                             // to Diagnostic::help,
                                             // anything else is Diagnostic::note

      wow = note_span => "custom span"; // <--- attachments can have their own span
                                        //      it takes effect only on nightly though

      hint =? opt_help; // <-- "optional" attachment, get displayed only if `Some`
                        //     must be single `Option` expression

      note =? note_span => opt_help // <-- optional attachments can have custom spans too
  );
  ```

### Diagnostic type

[`Diagnostic`] type is intentionally designed to be API compatible with [`proc_macro::Diagnostic`].
Not all API is implemented, only the part that can be reasonably implemented on stable.


[`abort!`]: macro.abort.html
[`abort_call_site!`]: macro.abort_call_site.html
[`emit_warning!`]: macro.emit_warning.html
[`emit_error!`]: macro.emit_error.html
[`emit_call_site_warning!`]: macro.emit_call_site_error.html
[`emit_call_site_error!`]: macro.emit_call_site_warning.html
[`diagnostic!`]: macro.diagnostic.html
[`Diagnostic`]: struct.Diagnostic.html

[`proc_macro::Span`]: https://doc.rust-lang.org/proc_macro/struct.Span.html
[`proc_macro::Diagnostic`]: https://doc.rust-lang.org/proc_macro/struct.Diagnostic.html

[unwind safe]: https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html#what-is-unwind-safety
[`!`]: https://doc.rust-lang.org/std/primitive.never.html
[`()`]: https://doc.rust-lang.org/std/primitive.unit.html
[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html

[`proc-macro2::Span`]: https://docs.rs/proc-macro2/1.0.10/proc_macro2/struct.Span.html
[`ToTokens`]: https://docs.rs/quote/1.0.3/quote/trait.ToTokens.html


## Modules

## Module `dummy`

Facility to emit dummy implementations (or whatever) in case
an error happen.

`compile_error!` does not abort a compilation right away. This means
`rustc` doesn't just show you the error and abort, it carries on the
compilation process looking for other errors to report.

Let's consider an example:

```rust,ignore
use proc_macro::TokenStream;
use proc_macro_error::*;

trait MyTrait {
    fn do_thing();
}

// this proc macro is supposed to generate MyTrait impl
#[proc_macro_derive(MyTrait)]
#[proc_macro_error]
fn example(input: TokenStream) -> TokenStream {
    // somewhere deep inside
    abort!(span, "something's wrong");

    // this implementation will be generated if no error happened
    quote! {
        impl MyTrait for #name {
            fn do_thing() {/* whatever */}
        }
    }
}

// ================
// in main.rs

// this derive triggers an error
#[derive(MyTrait)] // first BOOM!
struct Foo;

fn main() {
    Foo::do_thing(); // second BOOM!
}
```

The problem is: the generated token stream contains only `compile_error!`
invocation, the impl was not generated. That means user will see two compilation
errors:

```text
error: something's wrong
 --> $DIR/probe.rs:9:10
  |
9 |#[proc_macro_derive(MyTrait)]
  |                    ^^^^^^^

error[E0599]: no function or associated item named `do_thing` found for type `Foo` in the current scope
 --> src\main.rs:3:10
  |
1 | struct Foo;
  | ----------- function or associated item `do_thing` not found for this
2 | fn main() {
3 |     Foo::do_thing(); // second BOOM!
  |          ^^^^^^^^ function or associated item not found in `Foo`
```

But the second error is meaningless! We definitely need to fix this.

Most used approach in cases like this is "dummy implementation" -
omit `impl MyTrait for #name` and fill functions bodies with `unimplemented!()`.

This is how you do it:

```rust,ignore
use proc_macro::TokenStream;
use proc_macro_error::*;

 trait MyTrait {
     fn do_thing();
 }

 // this proc macro is supposed to generate MyTrait impl
 #[proc_macro_derive(MyTrait)]
 #[proc_macro_error]
 fn example(input: TokenStream) -> TokenStream {
     // first of all - we set a dummy impl which will be appended to
     // `compile_error!` invocations in case a trigger does happen
     set_dummy(quote! {
         impl MyTrait for #name {
             fn do_thing() { unimplemented!() }
         }
     });

     // somewhere deep inside
     abort!(span, "something's wrong");

     // this implementation will be generated if no error happened
     quote! {
         impl MyTrait for #name {
             fn do_thing() {/* whatever */}
         }
     }
 }

 // ================
 // in main.rs

 // this derive triggers an error
 #[derive(MyTrait)] // first BOOM!
 struct Foo;

 fn main() {
     Foo::do_thing(); // no more errors!
 }
```

```rust
pub mod dummy { /* ... */ }
```

### Functions

#### Function `set_dummy`

Sets dummy token stream which will be appended to `compile_error!(msg);...`
invocations in case you'll emit any errors.

See [guide](../index.html#guide).

```rust
pub fn set_dummy(dummy: proc_macro2::TokenStream) -> Option<proc_macro2::TokenStream> { /* ... */ }
```

#### Function `append_dummy`

Same as [`set_dummy`] but, instead of resetting, appends tokens to the
existing dummy (if any). Behaves as `set_dummy` if no dummy is present.

```rust
pub fn append_dummy(dummy: proc_macro2::TokenStream) { /* ... */ }
```

## Types

### Struct `SpanRange`

```rust
pub struct SpanRange {
    pub first: proc_macro2::Span,
    pub last: proc_macro2::Span,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `first` | `proc_macro2::Span` |  |
| `last` | `proc_macro2::Span` |  |

#### Implementations

##### Methods

- ```rust
  pub fn single_span(span: Span) -> Self { /* ... */ }
  ```
  Create a range with the `first` and `last` spans being the same.

- ```rust
  pub fn call_site() -> Self { /* ... */ }
  ```
  Create a `SpanRange` resolving at call site.

- ```rust
  pub fn from_tokens(ts: &dyn ToTokens) -> Self { /* ... */ }
  ```
  Construct span range from a `TokenStream`. This method always preserves all the

- ```rust
  pub fn join_range(self: Self, other: SpanRange) -> Self { /* ... */ }
  ```
  Join two span ranges. The resulting range will start at `self.first` and end at

- ```rust
  pub fn collapse(self: Self) -> Span { /* ... */ }
  ```
  Collapse the range into single span, preserving as much information as possible.

##### Trait Implementations

- **Send**
- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SpanRange { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

## Traits

### Trait `ResultExt`

This traits expands `Result<T, Into<Diagnostic>>` with some handy shortcuts.

```rust
pub trait ResultExt {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Ok`

##### Required Methods

- `unwrap_or_abort`: Behaves like `Result::unwrap`: if self is `Ok` yield the contained value,
- `expect_or_abort`: Behaves like `Result::expect`: if self is `Ok` yield the contained value,

#### Implementations

This trait is implemented for the following types:

- `Result<T, E>` with <T, E: Into<Diagnostic>>

### Trait `OptionExt`

This traits expands `Option` with some handy shortcuts.

```rust
pub trait OptionExt {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Some`

##### Required Methods

- `expect_or_abort`: Behaves like `Option::expect`: if self is `Some` yield the contained value,

#### Implementations

This trait is implemented for the following types:

- `Option<T>` with <T>

## Functions

### Function `abort_if_dirty`

Abort macro execution and display all the emitted errors, if any.

Does nothing if no errors were emitted (warnings do not count).

```rust
pub fn abort_if_dirty() { /* ... */ }
```

## Macros

### Macro `diagnostic`

**Attributes:**

- `#[macro_export]`

Build [`Diagnostic`](struct.Diagnostic.html) instance from provided arguments.

# Syntax

See [the guide](index.html#guide).


```rust
pub macro_rules! diagnostic {
    /* macro_rules! diagnostic {
    ($err:expr) => { ... };
    ($span:expr, $level:expr, $fmt:expr, $($args:expr),+ ; $($rest:tt)+) => { ... };
    ($span:expr, $level:expr, $msg:expr ; $($rest:tt)+) => { ... };
    ($span:expr, $level:expr, $fmt:expr, $($args:expr),+) => { ... };
    ($span:expr, $level:expr, $msg:expr) => { ... };
    ($span:expr, $level:expr, $fmt:expr, $($args:expr),+, ; $($rest:tt)+) => { ... };
    ($span:expr, $level:expr, $msg:expr, ; $($rest:tt)+) => { ... };
    ($span:expr, $level:expr, $fmt:expr, $($args:expr),+,) => { ... };
    ($span:expr, $level:expr, $msg:expr,) => { ... };
} */
}
```

### Macro `abort`

**Attributes:**

- `#[macro_export]`

Abort proc-macro execution right now and display the error.

# Syntax

See [the guide](index.html#guide).

```rust
pub macro_rules! abort {
    /* macro_rules! abort {
    ($err:expr) => { ... };
    ($span:expr, $($tts:tt)*) => { ... };
} */
}
```

### Macro `abort_call_site`

**Attributes:**

- `#[macro_export]`

Shortcut for `abort!(Span::call_site(), msg...)`. This macro
is still preferable over plain panic, panics are not for error reporting.

# Syntax

See [the guide](index.html#guide).


```rust
pub macro_rules! abort_call_site {
    /* macro_rules! abort_call_site {
    ($($tts:tt)*) => { ... };
} */
}
```

### Macro `emit_error`

**Attributes:**

- `#[macro_export]`

Emit an error while not aborting the proc-macro right away.

# Syntax

See [the guide](index.html#guide).


```rust
pub macro_rules! emit_error {
    /* macro_rules! emit_error {
    ($err:expr) => { ... };
    ($span:expr, $($tts:tt)*) => { ... };
} */
}
```

### Macro `emit_call_site_error`

**Attributes:**

- `#[macro_export]`

Shortcut for `emit_error!(Span::call_site(), ...)`. This macro
is still preferable over plain panic, panics are not for error reporting..

# Syntax

See [the guide](index.html#guide).


```rust
pub macro_rules! emit_call_site_error {
    /* macro_rules! emit_call_site_error {
    ($($tts:tt)*) => { ... };
} */
}
```

### Macro `emit_warning`

**Attributes:**

- `#[macro_export]`

Emit a warning. Warnings are not errors and compilation won't fail because of them.

**Does nothing on stable**

# Syntax

See [the guide](index.html#guide).


```rust
pub macro_rules! emit_warning {
    /* macro_rules! emit_warning {
    ($span:expr, $($tts:tt)*) => { ... };
} */
}
```

### Macro `emit_call_site_warning`

**Attributes:**

- `#[macro_export]`

Shortcut for `emit_warning!(Span::call_site(), ...)`.

**Does nothing on stable**

# Syntax

See [the guide](index.html#guide).


```rust
pub macro_rules! emit_call_site_warning {
    /* macro_rules! emit_call_site_warning {
    ($($tts:tt)*) => { ... };
} */
}
```

## Re-exports

### Re-export `Diagnostic`

```rust
pub use crate::diagnostic::Diagnostic;
```

### Re-export `DiagnosticExt`

```rust
pub use crate::diagnostic::DiagnosticExt;
```

### Re-export `Level`

```rust
pub use crate::diagnostic::Level;
```

### Re-export `append_dummy`

```rust
pub use crate::dummy::append_dummy;
```

### Re-export `set_dummy`

```rust
pub use crate::dummy::set_dummy;
```

### Re-export `proc_macro_error`

```rust
pub use proc_macro_error_attr::proc_macro_error;
```

