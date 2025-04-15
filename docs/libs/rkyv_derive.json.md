# Crate Documentation

**Version:** 0.8.10

**Format Version:** 43

# Module `rkyv_derive`

Procedural macros for `rkyv`.

## Macros

### Procedural Macro `Portable`

**Attributes:**

- `#[proc_macro_derive(Portable, attributes(rkyv))]`

Derives `Portable` for the labeled type.

```rust
pub #[proc_macro_derive]
// Helpers: #[rkyv]
pub fn Portable(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `Archive`

**Attributes:**

- `#[proc_macro_derive(Archive, attributes(rkyv))]`

Derives `Archive` for the labeled type.

# Attributes

Additional arguments can be specified using `#[rkyv(..)]`, which accepts
the following arguments:

## Types and fields

- `attr(..)`: Passes along attributes to the generated archived type.

## Types only

- `derive(..)`: Adds the derives passed as arguments to the generated type.
  This is equivalent to `#[rkyv(attr(derive(..)))]`.
- `crate = ..`: Chooses an alternative crate path to import rkyv from.
- `compare(..)`: Implements common comparison operators between the original
  and archived types. Supported comparisons are `PartialEq` and `PartialOrd`
  (i.e. `#[rkyv(compare(PartialEq, PartialOrd))]`).
- `{archive, serialize, deserialize}_bounds(..)`: Adds additional bounds to
  trait implementations. This can be useful for recursive types, where
  bounds may need to be omitted to prevent recursive trait impls.
- `bytecheck(..)`: Passed through to the underlying `CheckBytes` derive for
  the archived type.
- `as = ..`: Uses the given archived type instead of generating a new one.
  This is useful for types which are `Portable` and/or generic over their
  parameters.
- `archived = ..`: Changes the name of the generated archived type. By
  default, archived types are named "Archived" + `the name of the type`.
- `resolver = ..`: Changes the name of the generated resolver type. By
  default, resolver types are named `the name of the type` + "Resolver".
- `remote = ..`: Generate a remote derive for the annotated type instead of
  a regular derive.

## Fields only

- `with = ..`: Applies the given wrapper type to the field.
- `omit_bounds`: Omits trait bounds for the annotated field in the generated
  impl.

# Recursive types

This derive macro automatically adds a type bound `field: Archive` for each
field type. This can cause an overflow while evaluating trait bounds if the
structure eventually references its own type, as the implementation of
`Archive` for a struct depends on each field type implementing it
as well. Adding the attribute `#[rkyv(omit_bounds)]` to a field will
suppress this trait bound and allow recursive structures. This may be too
coarse for some types, in which case additional type bounds may be required
with `{archive, serialize, deserialize}_bounds(..)`.

# Wrappers

Wrappers transparently customize archived types by providing different
implementations of core traits. For example, references cannot be archived,
but the `Inline` wrapper serializes a reference as if it were a field of the
struct. Wrappers can be applied to fields using the `#[rkyv_with = ..]`
attribute.

```rust
pub #[proc_macro_derive]
// Helpers: #[rkyv]
pub fn Archive(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `Serialize`

**Attributes:**

- `#[proc_macro_derive(Serialize, attributes(rkyv))]`

Derives `Serialize` for the labeled type.

This macro also supports the `#[rkyv]` attribute. See [`Archive`] for more
information.

```rust
pub #[proc_macro_derive]
// Helpers: #[rkyv]
pub fn Serialize(/* ... */) -> /* ... */ {
    /* ... */
}
```

### Procedural Macro `Deserialize`

**Attributes:**

- `#[proc_macro_derive(Deserialize, attributes(rkyv))]`

Derives `Deserialize` for the labeled type.

This macro also supports the `#[rkyv]` attribute. See [`Archive`] for more
information.

```rust
pub #[proc_macro_derive]
// Helpers: #[rkyv]
pub fn Deserialize(/* ... */) -> /* ... */ {
    /* ... */
}
```

