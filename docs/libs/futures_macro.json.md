# Crate Documentation

**Version:** 0.3.31

**Format Version:** 43

# Module `futures_macro`

The futures-rs procedural macro implementations.

## Macros

### Procedural Macro `join_internal`

**Attributes:**

- `#[proc_macro]`

The `join!` macro.

```rust
pub #[proc_macro]
pub fn join_internal(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `try_join_internal`

**Attributes:**

- `#[proc_macro]`

The `try_join!` macro.

```rust
pub #[proc_macro]
pub fn try_join_internal(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `select_internal`

**Attributes:**

- `#[proc_macro]`

The `select!` macro.

```rust
pub #[proc_macro]
pub fn select_internal(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `select_biased_internal`

**Attributes:**

- `#[proc_macro]`

The `select_biased!` macro.

```rust
pub #[proc_macro]
pub fn select_biased_internal(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `test_internal`

**Attributes:**

- `#[proc_macro_attribute]`

```rust
pub #[proc_macro_attribute]
pub fn test_internal(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `stream_select_internal`

**Attributes:**

- `#[proc_macro]`

The `stream_select!` macro.

```rust
pub #[proc_macro]
pub fn stream_select_internal(/* ... */) -> /* ... */ {
    /* ... */
}
```

