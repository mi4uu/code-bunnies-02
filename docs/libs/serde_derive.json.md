# Crate Documentation

**Version:** 1.0.219

**Format Version:** 43

# Module `serde_derive`

This crate provides Serde's two derive macros.

```edition2021
# use serde_derive::{Deserialize, Serialize};
#
#[derive(Serialize, Deserialize)]
# struct S;
#
# fn main() {}
```

Please refer to [https://serde.rs/derive.html] for how to set this up.

[https://serde.rs/derive.html]: https://serde.rs/derive.html

## Macros

### Procedural Macro `Serialize`

**Attributes:**

- `#[proc_macro_derive(Serialize, attributes(serde))]`

```rust
pub #[proc_macro_derive]
// Helpers: #[serde]
pub fn Serialize(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `Deserialize`

**Attributes:**

- `#[proc_macro_derive(Deserialize, attributes(serde))]`

```rust
pub #[proc_macro_derive]
// Helpers: #[serde]
pub fn Deserialize(/* ... */) -> /* ... */ {
    /* ... */
}
```

