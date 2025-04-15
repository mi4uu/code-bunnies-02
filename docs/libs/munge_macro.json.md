# Crate Documentation

**Version:** 0.4.3

**Format Version:** 43

# Module `munge_macro`

The proc macro at the core of munge.

## Macros

### Procedural Macro `munge_with_path`

**Attributes:**

- `#[proc_macro]`

Destructures a value by projecting pointers.

```rust
pub #[proc_macro]
pub fn munge_with_path(/* ... */) -> /* ... */ {
    /* ... */
}
```

