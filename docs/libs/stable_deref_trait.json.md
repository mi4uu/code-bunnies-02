# Crate Documentation

**Version:** 1.2.0

**Format Version:** 43

# Module `stable_deref_trait`

This module defines an unsafe marker trait, StableDeref, for container types that deref to a fixed address which is valid even when the containing type is moved. For example, Box, Vec, Rc, Arc and String implement this trait. Additionally, it defines CloneStableDeref for types like Rc where clones deref to the same address.

It is intended to be used by crates such as [owning_ref](https://crates.io/crates/owning_ref) and [rental](https://crates.io/crates/rental), as well as library authors who wish to make their code interoperable with such crates. For example, if you write a custom Vec type, you can implement StableDeref, and then users will be able to use your custom type together with owning_ref and rental.

no_std support can be enabled by disabling default features (specifically "std"). In this case, the trait will not be implemented for the std types mentioned above, but you can still use it for your own types.

## Traits

### Trait `StableDeref`

An unsafe marker trait for types that deref to a stable address, even when moved. For example, this is implemented by Box, Vec, Rc, Arc and String, among others. Even when a Box is moved, the underlying storage remains at a fixed location.

More specifically, implementors must ensure that the result of calling deref() is valid for the lifetime of the object, not just the lifetime of the borrow, and that the deref is valid even if the object is moved. Also, it must be valid even after invoking arbitrary &self methods or doing anything transitively accessible from &Self. If Self also implements DerefMut, the same restrictions apply to deref_mut() and it must remain valid if anything transitively accessible from the result of deref_mut() is mutated/called. Additionally, multiple calls to deref, (and deref_mut if implemented) must return the same address. No requirements are placed on &mut self methods other than deref_mut() and drop(), if applicable.

Basically, it must be valid to convert the result of deref() to a pointer, and later dereference that pointer, as long as the original object is still live, even if it has been moved or &self methods have been called on it. If DerefMut is also implemented, it must be valid to get pointers from deref() and deref_mut() and dereference them while the object is live, as long as you don't simultaneously dereference both of them.

Additionally, Deref and DerefMut implementations must not panic, but users of the trait are not allowed to rely on this fact (so that this restriction can be removed later without breaking backwards compatibility, should the need arise).

Here are some examples to help illustrate the requirements for implementing this trait:

```
# use std::ops::Deref;
struct Foo(u8);
impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target { &self.0 }
}
```

Foo cannot implement StableDeref because the int will move when Foo is moved, invalidating the result of deref().

```
# use std::ops::Deref;
struct Foo(Box<u8>);
impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target { &*self.0 }
}
```

Foo can safely implement StableDeref, due to the use of Box.


```
# use std::ops::Deref;
# use std::ops::DerefMut;
# use std::rc::Rc;
#[derive(Clone)]
struct Foo(Rc<u8>);
impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target { &*self.0 }
}
impl DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut Self::Target { Rc::make_mut(&mut self.0) }
}
```

This is a simple implementation of copy-on-write: Foo's deref_mut will copy the underlying int if it is not uniquely owned, ensuring unique access at the point where deref_mut() returns. However, Foo cannot implement StableDeref because calling deref_mut(), followed by clone().deref() will result in mutable and immutable references to the same location. Note that if the DerefMut implementation were removed, Foo could safely implement StableDeref. Likewise, if the Clone implementation were removed, it would be safe to implement StableDeref, although Foo would not be very useful in that case, (without clones, the rc will always be uniquely owned).


```
# use std::ops::Deref;
struct Foo;
impl Deref for Foo {
    type Target = str;
    fn deref(&self) -> &Self::Target { &"Hello" }
}
```
Foo can safely implement StableDeref. It doesn't own the data being derefed, but the data is gaurenteed to live long enough, due to it being 'static.

```
# use std::ops::Deref;
# use std::cell::Cell;
struct Foo(Cell<bool>);
impl Deref for Foo {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        let b = self.0.get();
        self.0.set(!b);
        if b { &"Hello" } else { &"World" }
    }
}
```
Foo cannot safely implement StableDeref, even though every possible result of deref lives long enough. In order to safely implement StableAddress, multiple calls to deref must return the same result.

```
# use std::ops::Deref;
# use std::ops::DerefMut;
struct Foo(Box<(u8, u8)>);
impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target { &self.0.deref().0 }
}
impl DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0.deref_mut().1 }
}
```

Foo cannot implement StableDeref because deref and deref_mut return different addresses.


```rust
pub unsafe trait StableDeref: Deref {
    /* Associated items */
}
```

> This trait is unsafe to implement.

#### Implementations

This trait is implemented for the following types:

- `alloc::boxed::Box<T>` with <T: ?Sized>
- `alloc::vec::Vec<T>` with <T>
- `alloc::string::String`
- `alloc::rc::Rc<T>` with <T: ?Sized>
- `alloc::sync::Arc<T>` with <T: ?Sized>
- `core::cell::Ref<''a, T>` with <''a, T: ?Sized>
- `core::cell::RefMut<''a, T>` with <''a, T: ?Sized>
- `&''a T` with <''a, T: ?Sized>
- `&''a mut T` with <''a, T: ?Sized>

### Trait `CloneStableDeref`

An unsafe marker trait for types where clones deref to the same address. This has all the requirements of StableDeref, and additionally requires that after calling clone(), both the old and new value deref to the same address. For example, Rc and Arc implement CloneStableDeref, but Box and Vec do not.

Note that a single type should never implement both DerefMut and CloneStableDeref. If it did, this would let you get two mutable references to the same location, by cloning and then calling deref_mut() on both values.

```rust
pub unsafe trait CloneStableDeref: StableDeref + Clone {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Implementations

This trait is implemented for the following types:

- `alloc::rc::Rc<T>` with <T: ?Sized>
- `alloc::sync::Arc<T>` with <T: ?Sized>
- `&''a T` with <''a, T: ?Sized>

