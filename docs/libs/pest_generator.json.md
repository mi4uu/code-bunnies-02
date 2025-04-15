# Crate Documentation

**Version:** 2.8.0

**Format Version:** 43

# Module `pest_generator`

# pest generator

This crate generates code from ASTs (which is used in the `pest_derive` crate).

## Functions

### Function `derive_parser`

Processes the derive/proc macro input and generates the corresponding parser based
on the parsed grammar. If `include_grammar` is set to true, it'll generate an explicit
"include_str" statement (done in pest_derive, but turned off in the local bootstrap).

```rust
pub fn derive_parser(input: proc_macro2::TokenStream, include_grammar: bool) -> proc_macro2::TokenStream { /* ... */ }
```

