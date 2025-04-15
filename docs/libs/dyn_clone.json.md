# Crate Documentation

**Version:** 1.0.19

**Format Version:** 43

# Module `dyn_clone`

[![github]](https://github.com/dtolnay/dyn-clone)&ensp;[![crates-io]](https://crates.io/crates/dyn-clone)&ensp;[![docs-rs]](https://docs.rs/dyn-clone)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

This crate provides a [`DynClone`] trait that can be used in trait objects,
and a [`clone_box`] function that can clone any sized or dynamically sized
implementation of `DynClone`. Types that implement the standard library's
[`std::clone::Clone`] trait are automatically usable by a `DynClone` trait
object.

# Example

```
use dyn_clone::DynClone;

trait MyTrait: DynClone {
    fn recite(&self);
}

impl MyTrait for String {
    fn recite(&self) {
        println!("{} ♫", self);
    }
}

fn main() {
    let line = "The slithy structs did gyre and gimble the namespace";

    // Build a trait object holding a String.
    // This requires String to implement MyTrait and std::clone::Clone.
    let x: Box<dyn MyTrait> = Box::new(String::from(line));

    x.recite();

    // The type of x2 is a Box<dyn MyTrait> cloned from x.
    let x2 = dyn_clone::clone_box(&*x);

    x2.recite();
}
```

This crate includes a macro for concisely implementing `impl
std::clone::Clone for Box<dyn MyTrait>` in terms of `dyn_clone::clone_box`.

```
# use dyn_clone::DynClone;
#
// As before.
trait MyTrait: DynClone {
    /* ... */
}

dyn_clone::clone_trait_object!(MyTrait);

// Now data structures containing Box<dyn MyTrait> can derive Clone:
#[derive(Clone)]
struct Container {
    trait_object: Box<dyn MyTrait>,
}
```

The `clone_trait_object!` macro expands to just the following, which you can
handwrite instead if you prefer:

```
# use dyn_clone::DynClone;
#
# trait MyTrait: DynClone {}
#
impl Clone for Box<dyn MyTrait> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

// and similar for Box<dyn MyTrait + Send>, Box<dyn MyTrait + Sync>, Box<dyn MyTrait + Send + Sync>
```

## Traits

### Trait `DynClone`

This trait is implemented by any type that implements [`std::clone::Clone`].

```rust
pub trait DynClone: Sealed {
    /* Associated items */
}
```

#### Implementations

This trait is implemented for the following types:

- `T` with <T>
- `str`
- `[T]` with <T>

## Functions

### Function `clone`

`&T`&ensp;&mdash;&blacktriangleright;&ensp;`T`

```rust
pub fn clone<T>(t: &T) -> T
where
    T: DynClone { /* ... */ }
```

### Function `clone_box`

`&T`&ensp;&mdash;&blacktriangleright;&ensp;`Box<T>`

```rust
pub fn clone_box<T>(t: &T) -> alloc::boxed::Box<T>
where
    T: ?Sized + DynClone { /* ... */ }
```

### Function `arc_make_mut`

`&mut Arc<T>`&ensp;&mdash;&blacktriangleright;&ensp;`&mut T`

```rust
pub fn arc_make_mut<T>(arc: &mut alloc::sync::Arc<T>) -> &mut T
where
    T: ?Sized + DynClone { /* ... */ }
```

### Function `rc_make_mut`

`&mut Rc<T>`&ensp;&mdash;&blacktriangleright;&ensp;`&mut T`

```rust
pub fn rc_make_mut<T>(rc: &mut alloc::rc::Rc<T>) -> &mut T
where
    T: ?Sized + DynClone { /* ... */ }
```

## Macros

### Macro `clone_trait_object`

**Attributes:**

- `#[macro_export]`

Implement the standard library `Clone` for a trait object that has
`DynClone` as a supertrait.

```
use dyn_clone::DynClone;

trait MyTrait: DynClone {
    /* ... */
}

dyn_clone::clone_trait_object!(MyTrait);

// Now data structures containing Box<dyn MyTrait> can derive Clone.
#[derive(Clone)]
struct Container {
    trait_object: Box<dyn MyTrait>,
}
```

The macro supports traits that have type parameters and/or `where` clauses.

```
use dyn_clone::DynClone;
use std::io::Read;

trait Difficult<R>: DynClone where R: Read {
    /* ... */
}

dyn_clone::clone_trait_object!(<R> Difficult<R> where R: Read);
```

```rust
pub macro_rules! clone_trait_object {
    /* macro_rules! clone_trait_object {
    ($($path:tt)+) => { ... };
} */
}
```

