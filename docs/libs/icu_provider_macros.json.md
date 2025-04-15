# Crate Documentation

**Version:** 1.5.0

**Format Version:** 43

# Module `icu_provider_macros`

Proc macros for the ICU4X data provider.

These macros are re-exported from `icu_provider`.

## Macros

### Procedural Macro `data_struct`

**Attributes:**

- `#[proc_macro_attribute]`

The `#[data_struct]` attribute should be applied to all types intended
for use in a `DataStruct`.

It does the following things:

- `Apply #[derive(Yokeable, ZeroFrom)]`. The `ZeroFrom` derive can
   be customized with `#[zerofrom(clone)]` on non-ZeroFrom fields.

In addition, the attribute can be used to implement `DataMarker` and/or `KeyedDataMarker`
by adding symbols with optional key strings:

```
# // We DO NOT want to pull in the `icu` crate as a dev-dependency,
# // because that will rebuild the whole tree in proc macro mode
# // when using cargo test --all-features --all-targets.
# pub mod icu {
#   pub mod locid_transform {
#     pub mod fallback {
#       pub use icu_provider::_internal::LocaleFallbackPriority;
#     }
#   }
#   pub use icu_provider::_internal::locid;
# }
use icu::locid::extensions::unicode::key;
use icu::locid_transform::fallback::*;
use icu_provider::yoke;
use icu_provider::zerofrom;
use icu_provider::KeyedDataMarker;
use std::borrow::Cow;

#[icu_provider::data_struct(
    FooV1Marker,
    BarV1Marker = "demo/bar@1",
    marker(
        BazV1Marker,
        "demo/baz@1",
        fallback_by = "region",
        extension_key = "ca"
    )
)]
pub struct FooV1<'data> {
    message: Cow<'data, str>,
};

// Note: FooV1Marker implements `DataMarker` but not `KeyedDataMarker`.
// The other two implement `KeyedDataMarker`.

assert_eq!(&*BarV1Marker::KEY.path(), "demo/bar@1");
assert_eq!(
    BarV1Marker::KEY.metadata().fallback_priority,
    LocaleFallbackPriority::Language
);
assert_eq!(BarV1Marker::KEY.metadata().extension_key, None);

assert_eq!(&*BazV1Marker::KEY.path(), "demo/baz@1");
assert_eq!(
    BazV1Marker::KEY.metadata().fallback_priority,
    LocaleFallbackPriority::Region
);
assert_eq!(BazV1Marker::KEY.metadata().extension_key, Some(key!("ca")));
```

If the `#[databake(path = ...)]` attribute is present on the data struct, this will also
implement it on the markers.

```rust
pub #[proc_macro_attribute]
pub fn data_struct(/* ... */) -> /* ... */ {
    /* ... */
}
```

