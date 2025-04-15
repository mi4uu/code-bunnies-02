# Crate Documentation

**Version:** 0.8.22

**Format Version:** 43

# Module `schemars_derive`

## Macros

### Procedural Macro `JsonSchema`

**Attributes:**

- `#[proc_macro_derive(JsonSchema, attributes(schemars, serde, validate))]`

```rust
pub #[proc_macro_derive]
// Helpers: #[schemars], #[serde], #[validate]
pub fn JsonSchema(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `JsonSchema_repr`

**Attributes:**

- `#[proc_macro_derive(JsonSchema_repr, attributes(schemars, serde))]`

```rust
pub #[proc_macro_derive]
// Helpers: #[schemars], #[serde]
pub fn JsonSchema_repr(/* ... */) -> /* ... */ {
    /* ... */
}
```

