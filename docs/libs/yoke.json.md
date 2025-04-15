# Crate Documentation

**Version:** 0.7.5

**Format Version:** 43

# Module `yoke`

This crate provides [`Yoke<Y, C>`][Yoke], which allows one to "yoke" (attach) a zero-copy deserialized
object (say, a [`Cow<'a, str>`](alloc::borrow::Cow)) to the source it was deserialized from, (say, an [`Rc<[u8]>`](alloc::rc::Rc)),
known in this crate as a "cart", producing a type that looks like `Yoke<Cow<'static, str>, Rc<[u8]>>`
and can be moved around with impunity.

Succinctly, this allows one to "erase" static lifetimes and turn them into dynamic ones, similarly
to how `dyn` allows one to "erase" static types and turn them into dynamic ones.

Most of the time the yokeable `Y` type will be some kind of zero-copy deserializable
abstraction, potentially with an owned variant (like [`Cow`](alloc::borrow::Cow),
[`ZeroVec`](https://docs.rs/zerovec), or an aggregate containing such types), and the cart `C` will be some smart pointer like
  [`Box<T>`](alloc::boxed::Box), [`Rc<T>`](alloc::rc::Rc), or [`Arc<T>`](std::sync::Arc), potentially wrapped in an [`Option<T>`](Option).

The key behind this crate is [`Yoke::get()`], where calling [`.get()`][Yoke::get] on a type like
`Yoke<Cow<'static, str>, _>` will get you a short-lived `&'a Cow<'a, str>`, restricted to the
lifetime of the borrow used during [`.get()`](Yoke::get). This is entirely safe since the `Cow` borrows from
the cart type `C`, which cannot be interfered with as long as the `Yoke` is borrowed by [`.get()`](Yoke::get).
[`.get()`](Yoke::get) protects access by essentially reifying the erased lifetime to a safe local one
when necessary.

See the documentation of [`Yoke`] for more details.

## Modules

## Module `cartable_ptr`

Types for optional pointers with niche optimization.

The main type is [`CartableOptionPointer`], which is like `Option<Rc>` but
with a niche so that the resulting `Yoke` has a niche. The following four
types can be stored in the `CartableOptionPointer`:

1. `&T`
2. `Box<T>`
3. `Rc<T>`
4. `Arc<T>`

These four types implement the sealed unsafe trait [`CartablePointerLike`].
In addition, all except `Box<T>` impl [`CloneableCartablePointerLike`],
which allows [`CartableOptionPointer`] to implement `Clone`.

```rust
pub mod cartable_ptr { /* ... */ }
```

### Types

#### Struct `CartableOptionPointer`

A type with similar semantics as `Option<C<T>>` but with a niche.

This type cannot be publicly constructed. To use this in a `Yoke`, see
[`Yoke::convert_cart_into_option_pointer`].

[`Yoke::convert_cart_into_option_pointer`]: crate::Yoke::convert_cart_into_option_pointer

```rust
pub struct CartableOptionPointer<C> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn is_none(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this instance is `None`. From the return value:

###### Trait Implementations

- **Send**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneableCart**
- **Sync**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Traits

#### Trait `CartablePointerLike`

An object fully representable by a non-null pointer.

# Safety

Implementer safety:

1. `into_raw` transfers ownership of the values referenced by StableDeref to the caller,
   if there is ownership to transfer
2. `drop_raw` returns ownership back to the impl, if there is ownership to transfer

Note: if `into_raw` returns the sentinel pointer, memory leaks may occur, but this will not
lead to undefined behaviour.

Note: the pointer `NonNull<Self::Raw>` may or may not be aligned and it should never
be dereferenced. Rust allows unaligned pointers; see [`std::ptr::read_unaligned`].

```rust
pub unsafe trait CartablePointerLike: StableDeref + Sealed {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `&''a T` with <''a, T>
- `alloc::boxed::Box<T>` with <T>
- `alloc::rc::Rc<T>` with <T>
- `alloc::sync::Arc<T>` with <T>

#### Trait `CloneableCartablePointerLike`

An object that implements [`CartablePointerLike`] that also
supports cloning without changing the address of referenced data.

# Safety

Implementer safety:

1. `addref_raw` must create a new owner such that an additional call to
   `drop_raw` does not create a dangling pointer
2. `addref_raw` must not change the address of any referenced data.

```rust
pub unsafe trait CloneableCartablePointerLike: CartablePointerLike {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `&''a T` with <''a, T>
- `alloc::rc::Rc<T>` with <T>
- `alloc::sync::Arc<T>` with <T>

## Module `either`

Types to enable polymorphic carts.

```rust
pub mod either { /* ... */ }
```

### Types

#### Enum `EitherCart`

**Attributes:**

- `#[allow(clippy::exhaustive_enums)]`

A cart that can be one type or the other. Enables ergonomic polymorphic carts.

`EitherCart` enables yokes originating from different data sources and therefore
having different cart types to be merged into the same yoke type, but still being
able to recover the original cart type if necessary.

All relevant Cart traits are implemented for `EitherCart`, and carts can be
safely wrapped in an `EitherCart`.

Also see [`Yoke::erase_box_cart()`](crate::Yoke::erase_box_cart).

# Examples

```
use std::rc::Rc;
use yoke::either::EitherCart;
use yoke::Yoke;

let y1: Yoke<&'static str, Rc<str>> =
    Yoke::attach_to_zero_copy_cart("reference counted hello world".into());

let y2: Yoke<&'static str, &str> = Yoke::attach_to_zero_copy_cart("borrowed hello world");

type CombinedYoke<'a> = Yoke<&'static str, EitherCart<Rc<str>, &'a str>>;

// Both yokes can be combined into a single yoke type despite different carts
let y3: CombinedYoke = y1.wrap_cart_in_either_a();
let y4: CombinedYoke = y2.wrap_cart_in_either_b();

assert_eq!(*y3.get(), "reference counted hello world");
assert_eq!(*y4.get(), "borrowed hello world");

// The resulting yoke is cloneable if both cart types implement CloneableCart
let y5 = y4.clone();
assert_eq!(*y5.get(), "borrowed hello world");
```

```rust
pub enum EitherCart<C0, C1> {
    A(C0),
    B(C1),
}
```

##### Variants

###### `A`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `C0` |  |

###### `B`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `C1` |  |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EitherCart<C0, C1>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneableCart**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Receiver**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **StableDeref**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> EitherCart<C0, C1> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
## Module `erased`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

This module contains helper types for erasing Cart types.

See the docs of [`Yoke::erase_rc_cart()`](crate::Yoke::erase_rc_cart)
and [`Yoke::erase_box_cart()`](crate::Yoke::erase_box_cart) for more info.

✨ *Enabled with the `alloc` Cargo feature.*

```rust
pub mod erased { /* ... */ }
```

### Types

#### Type Alias `ErasedArcCart`

A type-erased Cart that has `Arc` semantics

See the docs of [`Yoke::erase_arc_cart()`](crate::Yoke::erase_rc_cart) for more info.

✨ *Enabled with the `alloc` Cargo feature.*

```rust
pub type ErasedArcCart = alloc::sync::Arc<dyn ErasedDestructor + Send + Sync>;
```

#### Type Alias `ErasedRcCart`

A type-erased Cart that has `Rc` semantics

See the docs of [`Yoke::erase_rc_cart()`](crate::Yoke::erase_rc_cart) for more info.

✨ *Enabled with the `alloc` Cargo feature.*

```rust
pub type ErasedRcCart = alloc::rc::Rc<dyn ErasedDestructor>;
```

#### Type Alias `ErasedBoxCart`

A type-erased Cart that has `Box` semantics

See the docs of [`Yoke::erase_box_cart()`](crate::Yoke::erase_box_cart) for more info.

✨ *Enabled with the `alloc` Cargo feature.*

```rust
pub type ErasedBoxCart = alloc::boxed::Box<dyn ErasedDestructor>;
```

### Traits

#### Trait `ErasedDestructor`

Dummy trait that lets us `dyn Drop`

`dyn Drop` isn't legal (and doesn't make sense since `Drop` is not
implement on all destructible types). However, all trait objects come with
a destructor, so we can just use an empty trait to get a destructor object.

```rust
pub trait ErasedDestructor: ''static {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `T` with <T: ''static>

## Module `trait_hack`

Workarounds for adding trait bounds to `yoke` objects.

# Trait bounds in Yoke

[Compiler bug #89196](https://github.com/rust-lang/rust/issues/89196) makes it tricky to add
trait bounds involving `yoke` types.

For example, you may want to write:

`where for<'a> <Y as Yokeable<'a>>::Output: MyTrait`

The above trait bound will compile, but at call sites, you get errors such as:

> the trait `for<'de> MyTrait` is not implemented for `<Y as Yokeable<'de>>::Output`

There are two known workarounds:

1. If the trait is well-defined on references, like `Debug`, bind the trait to a reference:
    `where for<'a> &'a <Y as Yokeable<'a>>::Output: MyTrait`
2. If the trait involves `Self`, like `Clone`, use [`YokeTraitHack`]:
    `where for<'a> YokeTraitHack<<Y as Yokeable<'a>>::Output>: MyTrait`

# Examples

Code that does not compile ([playground](https://play.rust-lang.org/?version=beta&mode=debug&edition=2018&gist=ebbda5b15a398d648bdff9e439b27dc0)):

```compile_fail
# this compiles in 1.78+, so this text will make it fail
use yoke::*;

trait MiniDataMarker {
    type Yokeable: for<'a> Yokeable<'a>;
}

struct MiniDataPayload<M>
where
    M: MiniDataMarker
{
    pub yoke: Yoke<M::Yokeable, ()>,
}

impl<M> Clone for MiniDataPayload<M>
where
    M: MiniDataMarker,
    for<'a> <M::Yokeable as Yokeable<'a>>::Output: Clone,
{
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

trait MiniDataProvider<M>
where
    M: MiniDataMarker
{
    fn mini_load_data(&self) -> MiniDataPayload<M>;
}

struct MiniStructProvider<M>
where
    M: MiniDataMarker,
{
    pub payload: MiniDataPayload<M>,
}

impl<M> MiniDataProvider<M> for MiniStructProvider<M>
where
    M: MiniDataMarker,
    for<'a> <M::Yokeable as Yokeable<'a>>::Output: Clone,
{
    fn mini_load_data(&self) -> MiniDataPayload<M> {
        self.payload.clone()
    }
}

#[derive(Clone)]
struct SimpleStruct(pub u32);

unsafe impl<'a> Yokeable<'a> for SimpleStruct {
    // (not shown; see `Yokeable` for examples)
#    type Output = SimpleStruct;
#    fn transform(&'a self) -> &'a Self::Output {
#        self
#    }
#    fn transform_owned(self) -> Self::Output {
#        self
#    }
#    unsafe fn make(from: Self::Output) -> Self {
#        std::mem::transmute(from)
#    }
#    fn transform_mut<F>(&'a mut self, f: F)
#    where
#        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
#    {
#        unsafe {
#            f(std::mem::transmute::<&'a mut Self, &'a mut Self::Output>(
#                self,
#            ))
#        }
#    }
}

impl MiniDataMarker for SimpleStruct {
    type DataStruct = SimpleStruct;
}

let provider = MiniStructProvider {
    payload: MiniDataPayload {
        yoke: Yoke::new_always_owned(SimpleStruct(42))
    }
};

// Broken:
// "method cannot be called on `MiniStructProvider<_>` due to unsatisfied trait bounds"
let payload: MiniDataPayload<SimpleStruct> = provider.mini_load_data();

// Working:
let payload = MiniDataProvider::<SimpleStruct>::mini_load_data(&provider);

assert_eq!(payload.yoke.get().0, 42);
```

Example for binding the trait to a reference:

```
use yoke::Yoke;
use yoke::Yokeable;

// Example trait and struct for illustration purposes:
trait MyTrait {
    fn demo(&self) -> u32;
}
struct MyStruct(u32);
impl MyTrait for MyStruct {
    fn demo(&self) -> u32 {
        self.0
    }
}
unsafe impl<'a> Yokeable<'a> for MyStruct {
    // (not shown; see `Yokeable` for examples)
#    type Output = MyStruct;
#    fn transform(&'a self) -> &'a Self::Output {
#        self
#    }
#    fn transform_owned(self) -> Self::Output {
#        self
#    }
#    unsafe fn make(from: Self::Output) -> Self {
#        std::mem::transmute(from)
#    }
#    fn transform_mut<F>(&'a mut self, f: F)
#    where
#        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
#    {
#        unsafe {
#            f(std::mem::transmute::<&'a mut Self, &'a mut Self::Output>(
#                self,
#            ))
#        }
#    }
}

// The trait needs to be defined on references:
impl<'a, T> MyTrait for &'a T
where
    T: MyTrait,
{
    fn demo(&self) -> u32 {
        self.demo()
    }
}

impl<Y, C> MyTrait for Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a>,
    for<'a> &'a <Y as Yokeable<'a>>::Output: MyTrait,
{
    fn demo(&self) -> u32 {
        self.get().demo()
    }
}

fn example() {
    let y = Yoke::<MyStruct, ()>::new_always_owned(MyStruct(42));
    let _: &dyn MyTrait = &y;
}
```

Example for using [`YokeTraitHack`]:

```
use std::rc::Rc;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yoke;
use yoke::Yokeable;

// Example trait and struct for illustration purposes:
trait MyTrait {
    fn demo(data: u32) -> Self;
}
struct MyStruct(u32);
impl MyTrait for MyStruct {
    fn demo(data: u32) -> Self {
        Self(data)
    }
}
unsafe impl<'a> Yokeable<'a> for MyStruct {
    // (not shown; see `Yokeable` for examples)
#    type Output = MyStruct;
#    fn transform(&'a self) -> &'a Self::Output {
#        self
#    }
#    fn transform_owned(self) -> Self::Output {
#        self
#    }
#    unsafe fn make(from: Self::Output) -> Self {
#        std::mem::transmute(from)
#    }
#    fn transform_mut<F>(&'a mut self, f: F)
#    where
#        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
#    {
#        unsafe {
#            f(std::mem::transmute::<&'a mut Self, &'a mut Self::Output>(
#                self,
#            ))
#        }
#    }
}

// The trait needs to be defined on YokeTraitHack:
impl<'a, T> MyTrait for YokeTraitHack<T>
where
    T: MyTrait,
{
    fn demo(data: u32) -> Self {
        YokeTraitHack(T::demo(data))
    }
}

impl<Y> MyTrait for Yoke<Y, Rc<u32>>
where
    Y: for<'a> Yokeable<'a>,
    for<'a> YokeTraitHack<<Y as Yokeable<'a>>::Output>: MyTrait,
{
    fn demo(data: u32) -> Self {
        let rc_u32: Rc<u32> = Rc::new(data);
        Yoke::attach_to_cart(rc_u32, |u| {
            YokeTraitHack::<<Y as Yokeable>::Output>::demo(*u).0
        })
    }
}

fn example() {
    let _ = Yoke::<MyStruct, Rc<u32>>::demo(42);
}
```

```rust
pub mod trait_hack { /* ... */ }
```

### Types

#### Struct `YokeTraitHack`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`
- `#[repr(transparent)]`

A wrapper around a type `T`, forwarding trait calls down to the inner type.

`YokeTraitHack` supports [`Clone`], [`PartialEq`], [`Eq`], and [`serde::Deserialize`] out of
the box. Other traits can be implemented by the caller.

For more information, see the module-level documentation.

# Example

Using `YokeTraitHack` as a type bound in a function comparing two `Yoke`s:

```
use yoke::trait_hack::YokeTraitHack;
use yoke::*;

fn compare_yokes<Y, C1, C2>(y1: Yoke<Y, C1>, y2: Yoke<Y, C2>) -> bool
where
    Y: for<'a> Yokeable<'a>,
    for<'a> YokeTraitHack<<Y as Yokeable<'a>>::Output>: PartialEq,
{
    YokeTraitHack(y1.get()).into_ref() == YokeTraitHack(y2.get()).into_ref()
}
```

```rust
pub struct YokeTraitHack<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn into_ref(self: Self) -> &''a YokeTraitHack<T> { /* ... */ }
  ```
  Converts from `YokeTraitHack<&T>` to `&YokeTraitHack<T>`.

###### Trait Implementations

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ZeroFrom**
  - ```rust
    fn zero_from(cart: &''zf C) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &YokeTraitHack<T>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> YokeTraitHack<T> { /* ... */ }
    ```

## Re-exports

### Re-export `Yokeable`

**Attributes:**

- `#[<cfg>(feature = "derive")]`

```rust
pub use yoke_derive::Yokeable;
```

### Re-export `CloneableCart`

```rust
pub use crate::yoke::CloneableCart;
```

### Re-export `Yoke`

```rust
pub use crate::yoke::Yoke;
```

### Re-export `Yokeable`

```rust
pub use crate::yokeable::Yokeable;
```

