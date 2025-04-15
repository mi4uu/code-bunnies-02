# Crate Documentation

**Version:** 0.8.1

**Format Version:** 43

# Module `bytecheck_derive`

Procedural macros for bytecheck.

## Macros

### Procedural Macro `CheckBytes`

**Attributes:**

- `#[proc_macro_derive(CheckBytes, attributes(bytecheck))]`

Derives `CheckBytes` for the labeled type.

This derive macro automatically adds a type bound `field: CheckBytes<__C>`
for each field type. This can cause an overflow while evaluating trait
bounds if the structure eventually references its own type, as the
implementation of `CheckBytes` for a struct depends on each field type
implementing it as well. Adding the attribute `#[check_bytes(omit_bounds)]`
to a field will suppress this trait bound and allow recursive structures.
This may be too coarse for some types, in which case additional type bounds
may be required with `bounds(...)`.

# Attributes

Additional arguments can be specified using attributes.

`#[bytecheck(...)]` accepts the following attributes:

## Types only

- `bounds(...)`: Adds additional bounds to the `CheckBytes` implementation.
  This can be especially useful when dealing with recursive structures,
  where bounds may need to be omitted to prevent recursive type definitions.
  In the context of the added bounds, `__C` is the name of the context
  generic (e.g. `__C: MyContext`).
- `crate = ...`: Chooses an alternative crate path to import bytecheck from.
- `verify`: Adds an additional verification step after the validity of each
  field has been checked. See the `Verify` trait for more information.

## Fields only

- `omit_bounds`: Omits trait bounds for the annotated field in the generated
  impl.

```rust
pub #[proc_macro_derive]
// Helpers: #[bytecheck]
pub fn CheckBytes(/* ... */) -> /* ... */ {
    /* ... */
}
```

