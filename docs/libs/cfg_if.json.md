# Crate Documentation

**Version:** 1.0.0

**Format Version:** 43

# Module `cfg_if`

A macro for defining `#[cfg]` if-else statements.

The macro provided by this crate, `cfg_if`, is similar to the `if/elif` C
preprocessor macro by allowing definition of a cascade of `#[cfg]` cases,
emitting the implementation which matches first.

This allows you to conveniently provide a long list `#[cfg]`'d blocks of code
without having to rewrite each clause multiple times.

# Example

```
cfg_if::cfg_if! {
    if #[cfg(unix)] {
        fn foo() { /* unix specific functionality */ }
    } else if #[cfg(target_pointer_width = "32")] {
        fn foo() { /* non-unix, 32-bit functionality */ }
    } else {
        fn foo() { /* fallback implementation */ }
    }
}

# fn main() {}
```

## Macros

### Macro `cfg_if`

**Attributes:**

- `#[macro_export]`

The main macro provided by this crate. See crate documentation for more
information.

```rust
pub macro_rules! cfg_if {
    /* macro_rules! cfg_if {
    ($(
        if #[cfg($meta:meta)] { $($tokens:tt)* }
    ) else * else {
        $($tokens2:tt)*
    }) => { ... };
    (
        if #[cfg($i_met:meta)] { $($i_tokens:tt)* }
        $(
            else if #[cfg($e_met:meta)] { $($e_tokens:tt)* }
        )*
    ) => { ... };
    (@__items ($($not:meta,)*) ; ) => { ... };
    (@__items ($($not:meta,)*) ; ( ($($m:meta),*) ($($tokens:tt)*) ), $($rest:tt)*) => { ... };
    (@__identity $($tokens:tt)*) => { ... };
} */
}
```

