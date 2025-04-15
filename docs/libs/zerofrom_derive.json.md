# Crate Documentation

**Version:** 0.1.6

**Format Version:** 43

# Module `zerofrom_derive`

Custom derives for `ZeroFrom` from the `zerofrom` crate.

## Macros

### Procedural Macro `ZeroFrom`

**Attributes:**

- `#[proc_macro_derive(ZeroFrom, attributes(zerofrom))]`

Custom derive for `zerofrom::ZeroFrom`,

This implements `ZeroFrom<Ty> for Ty` for types
without a lifetime parameter, and `ZeroFrom<Ty<'data>> for Ty<'static>`
for types with a lifetime parameter.

Apply the `#[zerofrom(clone)]` attribute to a field if it doesn't implement
Copy or ZeroFrom; this data will be cloned when the struct is zero_from'ed.

Apply the `#[zerofrom(maybe_borrow(T, U, V))]` attribute to the struct to indicate
that certain type parameters may themselves contain borrows (by default
the derives assume that type parameters perform no borrows and can be copied or cloned).

In rust versions where [this issue](https://github.com/rust-lang/rust/issues/114393) is fixed,
`#[zerofrom(may_borrow)]` can be applied directly to type parameters.

```rust
pub #[proc_macro_derive]
// Helpers: #[zerofrom]
pub fn ZeroFrom(/* ... */) -> /* ... */ {
    /* ... */
}
```

