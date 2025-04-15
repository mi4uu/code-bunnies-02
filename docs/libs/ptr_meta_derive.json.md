# Crate Documentation

**Version:** 0.3.0

**Format Version:** 43

# Module `ptr_meta_derive`

## Macros

### Procedural Macro `Pointee`

**Attributes:**

- `#[proc_macro_derive(Pointee, attributes(ptr_meta))]`

Derives `Pointee` for the labeled struct which has a trailing DST.

# Attributes

Additional arguments can be specified using attributes.

`#[ptr_meta(...)]` accepts the following arguments:

- `crate = ...`: Chooses an alternative crate path to import ptr_meta from.

```rust
pub #[proc_macro_derive]
// Helpers: #[ptr_meta]
pub fn Pointee(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `pointee`

**Attributes:**

- `#[proc_macro_attribute]`

Generates a `Pointee` implementation for trait object of the labeled trait.

# Arguments

`#[pointee(...)]` takes the following arguments:

- `crate = ...`: Chooses an alternative crate path to import ptr_meta from.

```rust
pub #[proc_macro_attribute]
pub fn pointee(/* ... */) -> /* ... */ {
    /* ... */
}
```

