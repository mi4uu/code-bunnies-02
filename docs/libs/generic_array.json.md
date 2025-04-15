# Crate Documentation

**Version:** 0.14.7

**Format Version:** 43

# Module `generic_array`

This crate implements a structure that can be used as a generic array type.
Core Rust array types `[T; N]` can't be used generically with
respect to `N`, so for example this:

```rust{compile_fail}
struct Foo<T, N> {
    data: [T; N]
}
```

won't work.

**generic-array** exports a `GenericArray<T,N>` type, which lets
the above be implemented as:

```rust
use generic_array::{ArrayLength, GenericArray};

struct Foo<T, N: ArrayLength<T>> {
    data: GenericArray<T,N>
}
```

The `ArrayLength<T>` trait is implemented by default for
[unsigned integer types](../typenum/uint/index.html) from
[typenum](../typenum/index.html):

```rust
# use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::U5;

struct Foo<N: ArrayLength<i32>> {
    data: GenericArray<i32, N>
}

# fn main() {
let foo = Foo::<U5>{data: GenericArray::default()};
# }
```

For example, `GenericArray<T, U5>` would work almost like `[T; 5]`:

```rust
# use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::U5;

struct Foo<T, N: ArrayLength<T>> {
    data: GenericArray<T, N>
}

# fn main() {
let foo = Foo::<i32, U5>{data: GenericArray::default()};
# }
```

For ease of use, an `arr!` macro is provided - example below:

```
# #[macro_use]
# extern crate generic_array;
# extern crate typenum;
# fn main() {
let array = arr![u32; 1, 2, 3];
assert_eq!(array[2], 3);
# }
```

## Modules

## Module `arr`

**Attributes:**

- `#[<cfg_attr>(test, macro_use)]`

Implementation for `arr!` macro.

```rust
pub mod arr { /* ... */ }
```

### Types

#### Type Alias `Inc`

Helper type for `arr!` macro

```rust
pub type Inc<T, U> = <U as AddLength<T, typenum::U1>>::Output;
```

### Traits

#### Trait `AddLength`

Helper trait for `arr!` macro

```rust
pub trait AddLength<T, N: ArrayLength<T>>: ArrayLength<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Output`: Resulting length

##### Implementations

This trait is implemented for the following types:

- `N1` with <T, N1, N2>

## Module `functional`

Functional programming with generic sequences

Please see `tests/generics.rs` for examples of how to best use these in your generic functions.

```rust
pub mod functional { /* ... */ }
```

### Types

#### Type Alias `MappedSequence`

Accessor type for a mapped generic sequence

```rust
pub type MappedSequence<S, T, U> = <<S as MappedGenericSequence<T, U>>::Mapped as GenericSequence<U>>::Sequence;
```

### Traits

#### Trait `MappedGenericSequence`

Defines the relationship between one generic sequence and another,
for operations such as `map` and `zip`.

```rust
pub unsafe trait MappedGenericSequence<T, U>: GenericSequence<T>
where
    <Self as >::Length: ArrayLength<U> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Mapped`: Mapped sequence type

##### Implementations

This trait is implemented for the following types:

- `&''a S` with <''a, T, U, S>
- `&''a mut S` with <''a, T, U, S>
- `GenericArray<T, N>` with <T, U, N>

#### Trait `FunctionalSequence`

Defines functional programming methods for generic sequences

```rust
pub unsafe trait FunctionalSequence<T>: GenericSequence<T> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn map<U, F>(self: Self, f: F) -> MappedSequence<Self, T, U>
where
    Self: MappedGenericSequence<T, U>,
    <Self as >::Length: ArrayLength<U>,
    F: FnMut(<Self as >::Item) -> U { /* ... */ }
  ```
  Maps a `GenericSequence` to another `GenericSequence`.

- ```rust
  fn zip<B, Rhs, U, F>(self: Self, rhs: Rhs, f: F) -> MappedSequence<Self, T, U>
where
    Self: MappedGenericSequence<T, U>,
    Rhs: MappedGenericSequence<B, U, Mapped = MappedSequence<Self, T, U>> + GenericSequence<B, Length = <Self as >::Length>,
    <Self as >::Length: ArrayLength<B> + ArrayLength<U>,
    F: FnMut(<Self as >::Item, <Rhs as >::Item) -> U { /* ... */ }
  ```
  Combines two `GenericSequence` instances and iterates through both of them,

- ```rust
  fn fold<U, F>(self: Self, init: U, f: F) -> U
where
    F: FnMut(U, <Self as >::Item) -> U { /* ... */ }
  ```
  Folds (or reduces) a sequence of data into a single value.

##### Implementations

This trait is implemented for the following types:

- `&''a S` with <''a, T, S: GenericSequence<T>>
- `&''a mut S` with <''a, T, S: GenericSequence<T>>
- `GenericArray<T, N>` with <T, N>

## Module `iter`

`GenericArray` iterator implementation.

```rust
pub mod iter { /* ... */ }
```

### Types

#### Struct `GenericArrayIter`

An iterator that moves out of a `GenericArray`

```rust
pub struct GenericArrayIter<T, N: ArrayLength<T>> {
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
  pub fn as_slice(self: &Self) -> &[T] { /* ... */ }
  ```
  Returns the remaining items of this iterator as a slice

- ```rust
  pub fn as_mut_slice(self: &mut Self) -> &mut [T] { /* ... */ }
  ```
  Returns the remaining items of this iterator as a mutable slice

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **FusedIterator**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn fold<B, F>(self: Self, init: B, f: F) -> B
where
    F: FnMut(B, <Self as >::Item) -> B { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn nth(self: &mut Self, n: usize) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> Option<T> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn rfold<B, F>(self: Self, init: B, f: F) -> B
where
    F: FnMut(B, <Self as >::Item) -> B { /* ... */ }
    ```

## Module `sequence`

Useful traits for manipulating sequences of data stored in `GenericArray`s

```rust
pub mod sequence { /* ... */ }
```

### Types

#### Type Alias `SequenceItem`

Accessor for `GenericSequence` item type, which is really `IntoIterator::Item`

For deeply nested generic mapped sequence types, like shown in `tests/generics.rs`,
this can be useful for keeping things organized.

```rust
pub type SequenceItem<T> = <T as IntoIterator>::Item;
```

### Traits

#### Trait `GenericSequence`

Defines some sequence with an associated length and iteration capabilities.

This is useful for passing N-length generic arrays as generics.

```rust
pub unsafe trait GenericSequence<T>: Sized + IntoIterator {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Length`: `GenericArray` associated length
- `Sequence`: Concrete sequence type used in conjuction with reference implementations of `GenericSequence`

###### Required Methods

- `generate`: Initializes a new sequence instance using the given function.

##### Implementations

This trait is implemented for the following types:

- `&''a S` with <''a, T: ''a, S: GenericSequence<T>>
- `&''a mut S` with <''a, T: ''a, S: GenericSequence<T>>
- `GenericArray<T, N>` with <T, N>

#### Trait `Lengthen`

Defines any `GenericSequence` which can be lengthened or extended by appending
or prepending an element to it.

Any lengthened sequence can be shortened back to the original using `pop_front` or `pop_back`

```rust
pub unsafe trait Lengthen<T>: Sized + GenericSequence<T> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Longer`: `GenericSequence` that has one more element than `Self`

###### Required Methods

- `append`: Returns a new array with the given element appended to the end of it.
- `prepend`: Returns a new array with the given element prepended to the front of it.

##### Implementations

This trait is implemented for the following types:

- `GenericArray<T, N>` with <T, N>

#### Trait `Shorten`

Defines a `GenericSequence` which can be shortened by removing the first or last element from it.

Additionally, any shortened sequence can be lengthened by
appending or prepending an element to it.

```rust
pub unsafe trait Shorten<T>: Sized + GenericSequence<T> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Shorter`: `GenericSequence` that has one less element than `Self`

###### Required Methods

- `pop_back`: Returns a new array without the last element, and the last element.
- `pop_front`: Returns a new array without the first element, and the first element.

##### Implementations

This trait is implemented for the following types:

- `GenericArray<T, N>` with <T, N>

#### Trait `Split`

Defines a `GenericSequence` that can be split into two parts at a given pivot index.

```rust
pub unsafe trait Split<T, K>: GenericSequence<T>
where
    K: ArrayLength<T> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `First`: First part of the resulting split array
- `Second`: Second part of the resulting split array

###### Required Methods

- `split`: Splits an array at the given index, returning the separate parts of the array.

##### Implementations

This trait is implemented for the following types:

- `GenericArray<T, N>` with <T, N, K>
- `&''a GenericArray<T, N>` with <''a, T, N, K>
- `&''a mut GenericArray<T, N>` with <''a, T, N, K>

#### Trait `Concat`

Defines `GenericSequence`s which can be joined together, forming a larger array.

```rust
pub unsafe trait Concat<T, M>: GenericSequence<T>
where
    M: ArrayLength<T> {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Rest`: Sequence to be concatenated with `self`
- `Output`: Resulting sequence formed by the concatenation.

###### Required Methods

- `concat`: Concatenate, or join, two sequences.

##### Implementations

This trait is implemented for the following types:

- `GenericArray<T, N>` with <T, N, M>

## Types

### Struct `GenericArray`

**Attributes:**

- `#[allow(dead_code)]`

Struct representing a generic array - `GenericArray<T, N>` works like [T; N]

```rust
pub struct GenericArray<T, U: ArrayLength<T>> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn as_slice(self: &Self) -> &[T] { /* ... */ }
  ```
  Extracts a slice containing the entire array.

- ```rust
  pub fn as_mut_slice(self: &mut Self) -> &mut [T] { /* ... */ }
  ```
  Extracts a mutable slice containing the entire array.

- ```rust
  pub fn from_slice(slice: &[T]) -> &GenericArray<T, N> { /* ... */ }
  ```
  Converts slice to a generic array reference with inferred length;

- ```rust
  pub fn from_mut_slice(slice: &mut [T]) -> &mut GenericArray<T, N> { /* ... */ }
  ```
  Converts mutable slice to a mutable generic array reference

- ```rust
  pub fn clone_from_slice(list: &[T]) -> GenericArray<T, N> { /* ... */ }
  ```
  Construct a `GenericArray` from a slice by cloning its content

- ```rust
  pub fn from_exact_iter<I>(iter: I) -> Option<Self>
where
    I: IntoIterator<Item = T> { /* ... */ }
  ```
  Creates a new `GenericArray` instance from an iterator with a specific size.

##### Trait Implementations

- **Copy**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[T] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 1] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 2] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 3] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 4] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 5] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 6] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 7] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 8] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 9] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 10] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 11] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 12] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 13] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 14] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 15] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 16] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 17] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 18] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 19] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 20] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 21] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 22] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 23] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 24] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 25] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 26] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 27] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 28] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 29] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 30] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 31] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 32] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 33] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 34] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 35] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 36] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 37] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 38] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 39] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 40] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 41] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 42] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 43] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 44] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 45] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 46] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 47] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 48] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 49] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 50] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 51] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 52] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 53] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 54] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 55] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 56] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 57] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 58] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 59] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 60] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 61] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 62] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 63] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 64] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 70] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 80] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 90] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 100] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 200] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 300] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 400] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 500] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 128] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 256] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 512] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 1000] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[T; 1024] { /* ... */ }
    ```

- **Receiver**
- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(arr: [T; 1]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U1>) -> [T; 1] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 1]) -> &GenericArray<T, ::typenum::U1> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 1]) -> &mut GenericArray<T, ::typenum::U1> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 2]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U2>) -> [T; 2] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 2]) -> &GenericArray<T, ::typenum::U2> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 2]) -> &mut GenericArray<T, ::typenum::U2> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 3]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U3>) -> [T; 3] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 3]) -> &GenericArray<T, ::typenum::U3> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 3]) -> &mut GenericArray<T, ::typenum::U3> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 4]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U4>) -> [T; 4] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 4]) -> &GenericArray<T, ::typenum::U4> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 4]) -> &mut GenericArray<T, ::typenum::U4> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 5]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U5>) -> [T; 5] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 5]) -> &GenericArray<T, ::typenum::U5> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 5]) -> &mut GenericArray<T, ::typenum::U5> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 6]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U6>) -> [T; 6] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 6]) -> &GenericArray<T, ::typenum::U6> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 6]) -> &mut GenericArray<T, ::typenum::U6> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 7]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U7>) -> [T; 7] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 7]) -> &GenericArray<T, ::typenum::U7> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 7]) -> &mut GenericArray<T, ::typenum::U7> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U8>) -> [T; 8] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 8]) -> &GenericArray<T, ::typenum::U8> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 8]) -> &mut GenericArray<T, ::typenum::U8> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 9]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U9>) -> [T; 9] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 9]) -> &GenericArray<T, ::typenum::U9> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 9]) -> &mut GenericArray<T, ::typenum::U9> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 10]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U10>) -> [T; 10] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 10]) -> &GenericArray<T, ::typenum::U10> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 10]) -> &mut GenericArray<T, ::typenum::U10> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 11]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U11>) -> [T; 11] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 11]) -> &GenericArray<T, ::typenum::U11> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 11]) -> &mut GenericArray<T, ::typenum::U11> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 12]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U12>) -> [T; 12] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 12]) -> &GenericArray<T, ::typenum::U12> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 12]) -> &mut GenericArray<T, ::typenum::U12> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 13]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U13>) -> [T; 13] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 13]) -> &GenericArray<T, ::typenum::U13> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 13]) -> &mut GenericArray<T, ::typenum::U13> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 14]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U14>) -> [T; 14] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 14]) -> &GenericArray<T, ::typenum::U14> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 14]) -> &mut GenericArray<T, ::typenum::U14> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 15]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U15>) -> [T; 15] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 15]) -> &GenericArray<T, ::typenum::U15> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 15]) -> &mut GenericArray<T, ::typenum::U15> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 16]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U16>) -> [T; 16] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 16]) -> &GenericArray<T, ::typenum::U16> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 16]) -> &mut GenericArray<T, ::typenum::U16> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 17]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U17>) -> [T; 17] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 17]) -> &GenericArray<T, ::typenum::U17> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 17]) -> &mut GenericArray<T, ::typenum::U17> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 18]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U18>) -> [T; 18] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 18]) -> &GenericArray<T, ::typenum::U18> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 18]) -> &mut GenericArray<T, ::typenum::U18> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 19]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U19>) -> [T; 19] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 19]) -> &GenericArray<T, ::typenum::U19> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 19]) -> &mut GenericArray<T, ::typenum::U19> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 20]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U20>) -> [T; 20] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 20]) -> &GenericArray<T, ::typenum::U20> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 20]) -> &mut GenericArray<T, ::typenum::U20> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 21]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U21>) -> [T; 21] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 21]) -> &GenericArray<T, ::typenum::U21> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 21]) -> &mut GenericArray<T, ::typenum::U21> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 22]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U22>) -> [T; 22] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 22]) -> &GenericArray<T, ::typenum::U22> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 22]) -> &mut GenericArray<T, ::typenum::U22> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 23]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U23>) -> [T; 23] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 23]) -> &GenericArray<T, ::typenum::U23> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 23]) -> &mut GenericArray<T, ::typenum::U23> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 24]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U24>) -> [T; 24] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 24]) -> &GenericArray<T, ::typenum::U24> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 24]) -> &mut GenericArray<T, ::typenum::U24> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 25]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U25>) -> [T; 25] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 25]) -> &GenericArray<T, ::typenum::U25> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 25]) -> &mut GenericArray<T, ::typenum::U25> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 26]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U26>) -> [T; 26] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 26]) -> &GenericArray<T, ::typenum::U26> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 26]) -> &mut GenericArray<T, ::typenum::U26> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 27]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U27>) -> [T; 27] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 27]) -> &GenericArray<T, ::typenum::U27> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 27]) -> &mut GenericArray<T, ::typenum::U27> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 28]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U28>) -> [T; 28] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 28]) -> &GenericArray<T, ::typenum::U28> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 28]) -> &mut GenericArray<T, ::typenum::U28> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 29]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U29>) -> [T; 29] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 29]) -> &GenericArray<T, ::typenum::U29> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 29]) -> &mut GenericArray<T, ::typenum::U29> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 30]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U30>) -> [T; 30] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 30]) -> &GenericArray<T, ::typenum::U30> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 30]) -> &mut GenericArray<T, ::typenum::U30> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 31]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U31>) -> [T; 31] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 31]) -> &GenericArray<T, ::typenum::U31> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 31]) -> &mut GenericArray<T, ::typenum::U31> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 32]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U32>) -> [T; 32] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 32]) -> &GenericArray<T, ::typenum::U32> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 32]) -> &mut GenericArray<T, ::typenum::U32> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 33]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U33>) -> [T; 33] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 33]) -> &GenericArray<T, ::typenum::U33> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 33]) -> &mut GenericArray<T, ::typenum::U33> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 34]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U34>) -> [T; 34] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 34]) -> &GenericArray<T, ::typenum::U34> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 34]) -> &mut GenericArray<T, ::typenum::U34> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 35]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U35>) -> [T; 35] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 35]) -> &GenericArray<T, ::typenum::U35> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 35]) -> &mut GenericArray<T, ::typenum::U35> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 36]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U36>) -> [T; 36] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 36]) -> &GenericArray<T, ::typenum::U36> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 36]) -> &mut GenericArray<T, ::typenum::U36> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 37]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U37>) -> [T; 37] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 37]) -> &GenericArray<T, ::typenum::U37> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 37]) -> &mut GenericArray<T, ::typenum::U37> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 38]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U38>) -> [T; 38] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 38]) -> &GenericArray<T, ::typenum::U38> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 38]) -> &mut GenericArray<T, ::typenum::U38> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 39]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U39>) -> [T; 39] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 39]) -> &GenericArray<T, ::typenum::U39> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 39]) -> &mut GenericArray<T, ::typenum::U39> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 40]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U40>) -> [T; 40] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 40]) -> &GenericArray<T, ::typenum::U40> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 40]) -> &mut GenericArray<T, ::typenum::U40> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 41]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U41>) -> [T; 41] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 41]) -> &GenericArray<T, ::typenum::U41> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 41]) -> &mut GenericArray<T, ::typenum::U41> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 42]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U42>) -> [T; 42] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 42]) -> &GenericArray<T, ::typenum::U42> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 42]) -> &mut GenericArray<T, ::typenum::U42> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 43]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U43>) -> [T; 43] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 43]) -> &GenericArray<T, ::typenum::U43> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 43]) -> &mut GenericArray<T, ::typenum::U43> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 44]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U44>) -> [T; 44] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 44]) -> &GenericArray<T, ::typenum::U44> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 44]) -> &mut GenericArray<T, ::typenum::U44> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 45]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U45>) -> [T; 45] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 45]) -> &GenericArray<T, ::typenum::U45> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 45]) -> &mut GenericArray<T, ::typenum::U45> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 46]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U46>) -> [T; 46] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 46]) -> &GenericArray<T, ::typenum::U46> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 46]) -> &mut GenericArray<T, ::typenum::U46> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 47]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U47>) -> [T; 47] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 47]) -> &GenericArray<T, ::typenum::U47> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 47]) -> &mut GenericArray<T, ::typenum::U47> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 48]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U48>) -> [T; 48] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 48]) -> &GenericArray<T, ::typenum::U48> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 48]) -> &mut GenericArray<T, ::typenum::U48> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 49]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U49>) -> [T; 49] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 49]) -> &GenericArray<T, ::typenum::U49> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 49]) -> &mut GenericArray<T, ::typenum::U49> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 50]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U50>) -> [T; 50] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 50]) -> &GenericArray<T, ::typenum::U50> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 50]) -> &mut GenericArray<T, ::typenum::U50> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 51]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U51>) -> [T; 51] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 51]) -> &GenericArray<T, ::typenum::U51> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 51]) -> &mut GenericArray<T, ::typenum::U51> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 52]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U52>) -> [T; 52] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 52]) -> &GenericArray<T, ::typenum::U52> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 52]) -> &mut GenericArray<T, ::typenum::U52> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 53]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U53>) -> [T; 53] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 53]) -> &GenericArray<T, ::typenum::U53> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 53]) -> &mut GenericArray<T, ::typenum::U53> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 54]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U54>) -> [T; 54] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 54]) -> &GenericArray<T, ::typenum::U54> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 54]) -> &mut GenericArray<T, ::typenum::U54> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 55]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U55>) -> [T; 55] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 55]) -> &GenericArray<T, ::typenum::U55> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 55]) -> &mut GenericArray<T, ::typenum::U55> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 56]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U56>) -> [T; 56] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 56]) -> &GenericArray<T, ::typenum::U56> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 56]) -> &mut GenericArray<T, ::typenum::U56> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 57]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U57>) -> [T; 57] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 57]) -> &GenericArray<T, ::typenum::U57> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 57]) -> &mut GenericArray<T, ::typenum::U57> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 58]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U58>) -> [T; 58] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 58]) -> &GenericArray<T, ::typenum::U58> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 58]) -> &mut GenericArray<T, ::typenum::U58> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 59]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U59>) -> [T; 59] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 59]) -> &GenericArray<T, ::typenum::U59> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 59]) -> &mut GenericArray<T, ::typenum::U59> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 60]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U60>) -> [T; 60] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 60]) -> &GenericArray<T, ::typenum::U60> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 60]) -> &mut GenericArray<T, ::typenum::U60> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 61]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U61>) -> [T; 61] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 61]) -> &GenericArray<T, ::typenum::U61> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 61]) -> &mut GenericArray<T, ::typenum::U61> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 62]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U62>) -> [T; 62] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 62]) -> &GenericArray<T, ::typenum::U62> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 62]) -> &mut GenericArray<T, ::typenum::U62> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 63]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U63>) -> [T; 63] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 63]) -> &GenericArray<T, ::typenum::U63> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 63]) -> &mut GenericArray<T, ::typenum::U63> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 64]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U64>) -> [T; 64] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 64]) -> &GenericArray<T, ::typenum::U64> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 64]) -> &mut GenericArray<T, ::typenum::U64> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 70]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U70>) -> [T; 70] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 70]) -> &GenericArray<T, ::typenum::U70> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 70]) -> &mut GenericArray<T, ::typenum::U70> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 80]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U80>) -> [T; 80] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 80]) -> &GenericArray<T, ::typenum::U80> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 80]) -> &mut GenericArray<T, ::typenum::U80> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 90]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U90>) -> [T; 90] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 90]) -> &GenericArray<T, ::typenum::U90> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 90]) -> &mut GenericArray<T, ::typenum::U90> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 100]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U100>) -> [T; 100] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 100]) -> &GenericArray<T, ::typenum::U100> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 100]) -> &mut GenericArray<T, ::typenum::U100> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 200]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U200>) -> [T; 200] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 200]) -> &GenericArray<T, ::typenum::U200> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 200]) -> &mut GenericArray<T, ::typenum::U200> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 300]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U300>) -> [T; 300] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 300]) -> &GenericArray<T, ::typenum::U300> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 300]) -> &mut GenericArray<T, ::typenum::U300> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 400]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U400>) -> [T; 400] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 400]) -> &GenericArray<T, ::typenum::U400> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 400]) -> &mut GenericArray<T, ::typenum::U400> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 500]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U500>) -> [T; 500] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 500]) -> &GenericArray<T, ::typenum::U500> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 500]) -> &mut GenericArray<T, ::typenum::U500> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 128]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U128>) -> [T; 128] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 128]) -> &GenericArray<T, ::typenum::U128> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 128]) -> &mut GenericArray<T, ::typenum::U128> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 256]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U256>) -> [T; 256] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 256]) -> &GenericArray<T, ::typenum::U256> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 256]) -> &mut GenericArray<T, ::typenum::U256> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 512]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U512>) -> [T; 512] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 512]) -> &GenericArray<T, ::typenum::U512> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 512]) -> &mut GenericArray<T, ::typenum::U512> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 1000]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U1000>) -> [T; 1000] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 1000]) -> &GenericArray<T, ::typenum::U1000> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 1000]) -> &mut GenericArray<T, ::typenum::U1000> { /* ... */ }
    ```

  - ```rust
    fn from(arr: [T; 1024]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(sel: GenericArray<T, ::typenum::U1024>) -> [T; 1024] { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T; 1024]) -> &GenericArray<T, ::typenum::U1024> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &mut [T; 1024]) -> &mut GenericArray<T, ::typenum::U1024> { /* ... */ }
    ```

  - ```rust
    fn from(slice: &[T]) -> &GenericArray<T, N> { /* ... */ }
    ```
    Converts slice to a generic array reference with inferred length;

  - ```rust
    fn from(slice: &mut [T]) -> &mut GenericArray<T, N> { /* ... */ }
    ```
    Converts mutable slice to a mutable generic array reference

- **Lengthen**
  - ```rust
    fn append(self: Self, last: T) -> <Self as >::Longer { /* ... */ }
    ```

  - ```rust
    fn prepend(self: Self, first: T) -> <Self as >::Longer { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &[T] { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> GenericArray<T, N>
where
    I: IntoIterator<Item = T> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Split**
  - ```rust
    fn split(self: Self) -> (<Self as >::First, <Self as >::Second) { /* ... */ }
    ```

  - ```rust
    fn split(self: Self) -> (<Self as >::First, <Self as >::Second) { /* ... */ }
    ```

  - ```rust
    fn split(self: Self) -> (<Self as >::First, <Self as >::Second) { /* ... */ }
    ```

- **GenericSequence**
  - ```rust
    fn generate<F>(f: F) -> GenericArray<T, N>
where
    F: FnMut(usize) -> T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenericArray<T, N> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: &''a GenericArray<T, N>) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: &''a mut GenericArray<T, N>) -> <Self as >::IntoIter { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 1] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 2] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 3] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 4] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 5] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 6] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 7] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 8] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 9] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 10] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 11] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 12] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 13] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 14] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 15] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 16] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 17] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 18] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 19] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 20] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 21] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 22] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 23] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 24] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 25] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 26] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 27] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 28] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 29] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 30] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 31] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 32] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 33] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 34] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 35] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 36] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 37] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 38] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 39] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 40] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 41] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 42] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 43] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 44] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 45] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 46] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 47] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 48] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 49] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 50] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 51] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 52] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 53] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 54] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 55] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 56] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 57] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 58] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 59] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 60] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 61] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 62] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 63] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 64] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 70] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 80] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 90] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 100] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 200] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 300] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 400] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 500] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 128] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 256] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 512] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 1000] { /* ... */ }
    ```

  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T; 1024] { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &GenericArray<T, N>) -> Ordering { /* ... */ }
    ```

- **Send**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &GenericArray<T, N>) -> Option<Ordering> { /* ... */ }
    ```

- **FunctionalSequence**
  - ```rust
    fn map<U, F>(self: Self, f: F) -> MappedSequence<Self, T, U>
where
    <Self as >::Length: ArrayLength<U>,
    Self: MappedGenericSequence<T, U>,
    F: FnMut(T) -> U { /* ... */ }
    ```

  - ```rust
    fn zip<B, Rhs, U, F>(self: Self, rhs: Rhs, f: F) -> MappedSequence<Self, T, U>
where
    Self: MappedGenericSequence<T, U>,
    Rhs: MappedGenericSequence<B, U, Mapped = MappedSequence<Self, T, U>> + GenericSequence<B, Length = <Self as >::Length>,
    <Self as >::Length: ArrayLength<B> + ArrayLength<U>,
    F: FnMut(T, <Rhs as >::Item) -> U { /* ... */ }
    ```

  - ```rust
    fn fold<U, F>(self: Self, init: U, f: F) -> U
where
    F: FnMut(U, T) -> U { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut [T] { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: Hasher { /* ... */ }
    ```

- **Shorten**
  - ```rust
    fn pop_back(self: Self) -> (<Self as >::Shorter, T) { /* ... */ }
    ```

  - ```rust
    fn pop_front(self: Self) -> (T, <Self as >::Shorter) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut [T] { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Concat**
  - ```rust
    fn concat(self: Self, rest: <Self as >::Rest) -> <Self as >::Output { /* ... */ }
    ```

- **MappedGenericSequence**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &[T] { /* ... */ }
    ```

## Traits

### Trait `ArrayLength`

Trait making `GenericArray` work, marking types to be used as length of an array

```rust
pub unsafe trait ArrayLength<T>: Unsigned {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `ArrayType`: Associated type representing the array type for the number

#### Implementations

This trait is implemented for the following types:

- `typenum::uint::UTerm` with <T>
- `typenum::uint::UInt<N, typenum::bit::B0>` with <T, N: ArrayLength<T>>
- `typenum::uint::UInt<N, typenum::bit::B1>` with <T, N: ArrayLength<T>>

## Macros

### Macro `arr`

**Attributes:**

- `#[macro_export]`

Macro allowing for easy generation of Generic Arrays.
Example: `let test = arr![u32; 1, 2, 3];`

```rust
pub macro_rules! arr {
    /* macro_rules! arr {
    ($T:ty; $(,)*) => { ... };
    ($T:ty; $($x:expr),* $(,)*) => { ... };
    ($($x:expr,)+) => { ... };
    () => { ... };
} */
}
```

## Re-exports

### Re-export `GenericArrayIter`

```rust
pub use self::iter::GenericArrayIter;
```

## Other Items

### Extern Crate `typenum`

```rust
pub extern crate typenum;
```

