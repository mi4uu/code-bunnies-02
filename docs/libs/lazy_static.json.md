# Crate Documentation

**Version:** 1.5.0

**Format Version:** 43

# Module `lazy_static`

A macro for declaring lazily evaluated statics.

Using this macro, it is possible to have `static`s that require code to be
executed at runtime in order to be initialized.
This includes anything requiring heap allocations, like vectors or hash maps,
as well as anything that requires function calls to be computed.

# Syntax

```ignore
lazy_static! {
    [pub] static ref NAME_1: TYPE_1 = EXPR_1;
    [pub] static ref NAME_2: TYPE_2 = EXPR_2;
    ...
    [pub] static ref NAME_N: TYPE_N = EXPR_N;
}
```

Attributes (including doc comments) are supported as well:

```rust
use lazy_static::lazy_static;

# fn main() {
lazy_static! {
    /// This is an example for using doc comment attributes
    static ref EXAMPLE: u8 = 42;
}
# }
```

# Semantics

For a given `static ref NAME: TYPE = EXPR;`, the macro generates a unique type that
implements `Deref<TYPE>` and stores it in a static with name `NAME`. (Attributes end up
attaching to this type.)

On first deref, `EXPR` gets evaluated and stored internally, such that all further derefs
can return a reference to the same object. Note that this can lead to deadlocks
if you have multiple lazy statics that depend on each other in their initialization.

Apart from the lazy initialization, the resulting "static ref" variables
have generally the same properties as regular "static" variables:

- Any type in them needs to fulfill the `Sync` trait.
- If the type has a destructor, then it will not run when the process exits.

# Example

Using the macro:

```rust
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn main() {
    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}
```

# Implementation details

The `Deref` implementation uses a hidden static variable that is guarded by an atomic check on each access.

# Cargo features

This crate provides one cargo feature:

- `spin_no_std`: This allows using this crate in a no-std environment, by depending on the standalone `spin` crate.

## Traits

### Trait `LazyStatic`

Support trait for enabling a few common operation on lazy static values.

This is implemented by each defined lazy static, and
used by the free functions in this crate.

```rust
pub trait LazyStatic {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

## Functions

### Function `initialize`

Takes a shared reference to a lazy static and initializes
it if it has not been already.

This can be used to control the initialization point of a lazy static.

Example:

```rust
use lazy_static::lazy_static;

lazy_static! {
    static ref BUFFER: Vec<u8> = (0..255).collect();
}

fn main() {
    lazy_static::initialize(&BUFFER);

    // ...
    work_with_initialized_data(&BUFFER);
}
# fn work_with_initialized_data(_: &[u8]) {}
```

```rust
pub fn initialize<T: LazyStatic>(lazy: &T) { /* ... */ }
```

## Macros

### Macro `lazy_static`

**Attributes:**

- `#[macro_export(local_inner_macros)]`

```rust
pub macro_rules! lazy_static {
    /* macro_rules! lazy_static {
    ($(#[$attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => { ... };
    ($(#[$attr:meta])* pub static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => { ... };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => { ... };
    () => { ... };
} */
}
```

