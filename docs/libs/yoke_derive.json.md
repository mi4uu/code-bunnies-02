# Crate Documentation

**Version:** 0.7.5

**Format Version:** 43

# Module `yoke_derive`

Custom derives for `Yokeable` from the `yoke` crate.

## Macros

### Procedural Macro `Yokeable`

**Attributes:**

- `#[proc_macro_derive(Yokeable, attributes(yoke))]`

Custom derive for `yoke::Yokeable`,

If your struct contains `zerovec::ZeroMap`, then the compiler will not
be able to guarantee the lifetime covariance due to the generic types on
the `ZeroMap` itself. You must add the following attribute in order for
the custom derive to work with `ZeroMap`.

```rust,ignore
#[derive(Yokeable)]
#[yoke(prove_covariance_manually)]
```

Beyond this case, if the derive fails to compile due to lifetime issues, it
means that the lifetime is not covariant and `Yokeable` is not safe to implement.

```rust
pub #[proc_macro_derive]
// Helpers: #[yoke]
pub fn Yokeable(/* ... */) -> /* ... */ {
    /* ... */
}
```

