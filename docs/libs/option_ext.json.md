# Crate Documentation

**Version:** 0.2.0

**Format Version:** 43

# Module `option_ext`

## Traits

### Trait `OptionExt`

Extension trait providing additional methods for `Option`.

```rust
pub trait OptionExt<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `contains`: Returns `true` if the option is a [`Some`] value containing the given value.
- `map_or2`: Returns the result from applying the function to the contained value if the option is [`Some`],
- `map_or_else2`: Returns the result from applying the function to the contained value if the option is [`Some`],

#### Implementations

This trait is implemented for the following types:

- `Option<T>` with <T>

